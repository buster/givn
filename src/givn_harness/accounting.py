"""Normalization and accounting for recorded givn sessions.

The rules here are intentionally conservative. Absent values stay absent,
overlapping intervals are never summed into wall time, and every derived
number names its provenance and validity. This module never executes code from
the analyzed repository and never contacts a provider.
"""

from __future__ import annotations

import os
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import Any, Iterable

from givn_harness.events import iter_all
from givn_harness.types import (
    STATUS_INFERRED,
    STATUS_OBSERVED,
    STATUS_PARTIAL,
    STATUS_UNAVAILABLE,
    Diagnostic,
    Event,
    EventFilter,
    Finding,
    Measurement,
    SourceFile,
)

RECORDER_PLUMBING_NAMES = {"history", "attribution", "hook intake", "harness-hook"}
RECORDER_PLUMBING_PREFIXES = (
    "history ingest",
    "history harness-hook",
    "history pending",
    "attribution ",
)

FAILURE_OUTCOMES = {"failed", "failure", "error", "aborted", "cancelled", "canceled"}
SUCCESS_OUTCOMES = {"ok", "success", "completed", "complete"}

USAGE_INPUT_KEYS = ("input", "input_tokens", "prompt_tokens")
USAGE_OUTPUT_KEYS = ("output", "output_tokens", "completion_tokens")
USAGE_CACHE_READ_KEYS = ("cache_read", "cacheRead", "cache_read_tokens")
USAGE_CACHE_WRITE_KEYS = ("cache_write", "cacheWrite", "cache_write_tokens")
USAGE_REASONING_KEYS = ("reasoning", "reasoning_tokens")

COST_COMPONENT_KEYS = (
    "input",
    "output",
    "cacheRead",
    "cacheWrite",
    "cache_read",
    "cache_write",
    "reasoning",
)


def _first_number(data: dict[str, Any], keys: Iterable[str]) -> float | None:
    for key in keys:
        value = data.get(key)
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            continue
        return float(value)
    return None


def _as_float(value: Any) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    return float(value)


def _normalize_label(text: str) -> str:
    """Collapse whitespace and volatile arguments for repeat detection."""
    cleaned = " ".join(text.split())
    # Commit hashes, numeric ids and generated names are not stable identities.
    parts = []
    for token in cleaned.split(" "):
        if len(token) >= 12 and all(char in "0123456789abcdef" for char in token.lower()):
            parts.append("<hash>")
        elif token.isdigit():
            parts.append("<n>")
        else:
            parts.append(token)
    return " ".join(parts)[:240]


def is_recorder_plumbing(event: Event) -> bool:
    """True for recorder intake/attribution, not user-facing workflow work."""
    if event.producer != "givn":
        return False
    if event.name in RECORDER_PLUMBING_NAMES:
        return True
    label = event.label.lower()
    return any(label.startswith(prefix) for prefix in RECORDER_PLUMBING_PREFIXES)


def is_failure(event: Event) -> bool:
    if event.outcome.lower() in FAILURE_OUTCOMES:
        return True
    exit_code = event.data.get("exit_code")
    if isinstance(exit_code, int) and not isinstance(exit_code, bool) and exit_code != 0:
        return True
    error = event.data.get("error")
    return bool(error)


def error_text(event: Event, limit: int = 240) -> str:
    value = event.data.get("error")
    if isinstance(value, str):
        text = value
    elif isinstance(value, dict):
        text = str(value.get("message") or value.get("text") or value)
    else:
        text = event.label or event.name
    text = " ".join(str(text).split())
    return text if len(text) <= limit else text[: limit - 1] + "…"


def _union_seconds(intervals: list[tuple[float, float]]) -> tuple[float, int]:
    if not intervals:
        return 0.0, 0
    ordered = sorted(intervals)
    total = 0.0
    blocks = 0
    current_start, current_end = ordered[0]
    for start, end in ordered[1:]:
        if start <= current_end:
            current_end = max(current_end, end)
        else:
            total += max(0.0, current_end - current_start)
            blocks += 1
            current_start, current_end = start, end
    total += max(0.0, current_end - current_start)
    return total, blocks + 1


def _overlap_pairs(
    intervals: list[tuple[float, float, str]], limit: int = 20
) -> list[tuple[str, str]]:
    """Return bounded examples of overlapping intervals (nested work)."""
    if limit <= 0:
        return []
    ordered = sorted(intervals)
    pairs: list[tuple[str, str]] = []
    for index, (start, end, event_id) in enumerate(ordered):
        for other_start, other_end, other_id in ordered[index + 1 :]:
            if other_start >= end:
                break
            if other_start < end and other_id != event_id:
                pairs.append((event_id, other_id))
                if len(pairs) >= limit:
                    return pairs
    return pairs


@dataclass
class _UsageRequest:
    request_id: str
    source: str  # "turn" or "usage"
    evidence: str
    input: float | None = None
    output: float | None = None
    cache_read: float | None = None
    cache_write: float | None = None
    reasoning: float | None = None
    total: float | None = None
    model: str | None = None


@dataclass
class _CostRecord:
    evidence: str
    value: float | None
    components: dict[str, float] | None
    cumulative: bool
    source: str


@dataclass
class Analysis:
    """The complete structured result of one retrospective analysis."""

    sources: list[SourceFile]
    filters: dict[str, Any]
    diagnostics: list[Diagnostic] = field(default_factory=list)
    measurements: list[Measurement] = field(default_factory=list)
    findings: list[Finding] = field(default_factory=list)
    data: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema": "givn-harness/analysis/v1",
            "sources": [source.to_dict() for source in self.sources],
            "filters": self.filters,
            "data_quality": {
                "diagnostic_count": len(self.diagnostics),
                "diagnostics": [item.to_dict() for item in self.diagnostics],
            },
            "measurements": [item.to_dict() for item in self.measurements],
            "findings": [item.to_dict() for item in self.findings],
            "data": self.data,
        }


def _parse_bound(value: str | None, label: str) -> datetime | None:
    if not value:
        return None
    text = value.strip()
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(text)
    except ValueError as error:
        raise ValueError(f"{label} must be ISO-8601, got {value!r}") from error
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def build_filter(
    sessions: Iterable[str] | None = None,
    scopes: Iterable[str] | None = None,
    since: str | None = None,
    until: str | None = None,
    kinds: Iterable[str] | None = None,
) -> EventFilter:
    return EventFilter(
        sessions=set(sessions) if sessions else None,
        scopes=set(scopes) if scopes else None,
        since=_parse_bound(since, "--since"),
        until=_parse_bound(until, "--until"),
        kinds=set(kinds) if kinds else None,
    )


def filters_to_dict(event_filter: EventFilter) -> dict[str, Any]:
    def stamp(value: datetime | None) -> str | None:
        return value.isoformat().replace("+00:00", "Z") if value else None

    return {
        "sessions": sorted(event_filter.sessions) if event_filter.sessions else None,
        "scopes": sorted(event_filter.scopes) if event_filter.scopes else None,
        "since": stamp(event_filter.since),
        "until": stamp(event_filter.until),
        "kinds": sorted(event_filter.kinds) if event_filter.kinds else None,
    }


def analyze_sources(
    sources: list[SourceFile],
    event_filter: EventFilter | None = None,
    *,
    metadata: dict[str, Any] | None = None,
) -> Analysis:
    """Run the shared accounting pipeline over frozen sources."""
    event_filter = event_filter or EventFilter()
    result = Analysis(
        sources=sources,
        filters=filters_to_dict(event_filter),
        diagnostics=[],
    )
    metadata = metadata or {}

    kind_counts: Counter[str] = Counter()
    producer_counts: Counter[str] = Counter()
    outcome_counts: Counter[str] = Counter()
    scope_counts: Counter[str] = Counter()
    session_counts: Counter[str] = Counter()
    schema_counts: Counter[str] = Counter()
    model_counts: Counter[str] = Counter()
    stop_reasons: Counter[str] = Counter()

    first_at: datetime | None = None
    last_at: datetime | None = None
    events_without_at = 0
    filtered_out = 0
    events_seen = 0

    intervals_all: list[tuple[float, float, str]] = []
    durations_by_kind: Counter[str] = Counter()
    duration_count_by_kind: Counter[str] = Counter()
    durations_by_producer: Counter[str] = Counter()
    durations_by_category: Counter[str] = Counter()
    missing_duration_by_kind: Counter[str] = Counter()

    ids: set[str] = set()
    parents: set[str] = set()
    missing_parents: set[str] = set()

    turns: dict[str, Event] = {}
    usage_events_by_parent: dict[str, list[Event]] = defaultdict(list)
    orphan_usage: list[Event] = []

    cost_records: list[_CostRecord] = []
    cost_missing_requests = 0
    cost_ambiguous_requests = 0

    failures: list[Event] = []
    failure_groups: dict[str, list[Event]] = defaultdict(list)
    command_counts: Counter[str] = Counter()
    command_outcomes: dict[str, Counter[str]] = defaultdict(Counter)
    archive_events: list[Event] = []
    phase_events: list[Event] = []
    question_events: list[Event] = []
    subagent_events: dict[str, list[Event]] = defaultdict(list)
    context_ratios: list[tuple[float, int | None, str]] = []

    for event in iter_all(sources, result.diagnostics):
        events_seen += 1
        if not event_filter.accepts(event):
            filtered_out += 1
            continue

        ids.add(event.id)
        if event.parent:
            parents.add(event.parent)
        kind_counts[event.kind] += 1
        producer_counts[event.producer] += 1
        outcome_counts[event.outcome or "missing"] += 1
        if event.scope_id:
            scope_counts[f"{event.scope_kind}:{event.scope_id}"] += 1
        if event.session:
            session_counts[event.session] += 1
        if event.schema is not None:
            schema_counts[str(event.schema)] += 1

        if event.at is None:
            events_without_at += 1
        else:
            first_at = event.at if first_at is None or event.at < first_at else first_at
            last_at = event.at if last_at is None or event.at > last_at else last_at

        if event.duration_ms is not None:
            if event.at is not None:
                end = event.at.timestamp()
                start = end - event.duration_ms / 1000.0
                intervals_all.append((start, end, event.id))
            durations_by_kind[event.kind] += event.duration_ms
            duration_count_by_kind[event.kind] += 1
            durations_by_producer[event.producer or "unknown"] += event.duration_ms
            category = _category(event)
            durations_by_category[category] += event.duration_ms
        else:
            missing_duration_by_kind[event.kind] += 1

        if is_recorder_plumbing(event):
            # Keep the record; exclude it from friction totals below.
            pass
        elif event.kind == "command":
            key = _normalize_label(f"{event.name} {event.label}".strip())
            command_counts[key] += 1
            command_outcomes[key][event.outcome or "missing"] += 1
            if "archive" in event.name.lower() or "archive" in event.label.lower():
                archive_events.append(event)
        elif event.kind in ("step", "phase") and (
            "archive" in event.name.lower() or "archive" in event.label.lower()
        ):
            archive_events.append(event)

        if is_failure(event):
            failures.append(event)
            failure_groups[_normalize_label(f"{event.kind}:{event.name}:{event.label}")].append(event)

        if event.kind == "turn":
            turns[event.id] = event
            data = event.data
            usage = data.get("usage")
            if isinstance(usage, dict):
                request = _usage_from_dict(event.id, "turn", event.id, usage)
                model = usage.get("model") or data.get("model")
                if isinstance(model, str):
                    request.model = model
                    model_counts[model] += 1
                stop = data.get("stop_reason")
                if isinstance(stop, str):
                    stop_reasons[stop] += 1
                context_size = _as_float(data.get("context_size"))
                context_window = _as_float(data.get("context_window"))
                if context_size is not None and context_window:
                    context_ratios.append(
                        (context_size / context_window, event.raw_id if isinstance(event.raw_id, int) else None, event.id)
                    )
            # Cost is counted once per request; a linked usage event is not a
            # second request.
            cost = data.get("cost")
            if cost is not None:
                record = _cost_from_value(event.id, cost, data)
                if record is not None:
                    cost_records.append(record)
                else:
                    cost_ambiguous_requests += 1
            elif isinstance(usage, dict) and usage.get("cost") is not None:
                record = _cost_from_value(event.id, usage.get("cost"), usage)
                if record is not None:
                    cost_records.append(record)
                else:
                    cost_ambiguous_requests += 1
            else:
                cost_missing_requests += 1
        elif event.kind == "usage":
            if event.parent:
                usage_events_by_parent[event.parent].append(event)
            else:
                orphan_usage.append(event)
            model = event.data.get("model")
            if isinstance(model, str):
                model_counts[model] += 1
        elif event.kind == "question":
            question_events.append(event)
        elif event.kind == "subagent":
            identity = str(event.data.get("subagent") or event.name or event.session or event.id)
            subagent_events[identity].append(event)
        elif event.kind == "phase":
            phase_events.append(event)

    # Parent links that resolve to nothing are ambiguous subagent/parent spans,
    # not guessed merges.
    unresolved = sorted(parents - ids)
    for parent in unresolved[:100]:
        result.diagnostics.append(
            Diagnostic(
                "unresolved_parent",
                f"parent {parent!r} is not present in the filtered record",
                event_id=parent,
            )
        )
        missing_parents.add(parent)

    # --- Usage authority -------------------------------------------------
    authoritative: list[_UsageRequest] = []
    duplicate_usage: list[str] = []
    ambiguous_usage: list[str] = []
    linked_without_turn: list[str] = []
    for turn_id, turn in turns.items():
        inline = turn.data.get("usage")
        linked = usage_events_by_parent.get(turn_id, [])
        if isinstance(inline, dict):
            request = _usage_from_dict(turn_id, "turn", turn_id, inline)
            model = inline.get("model") or turn.data.get("model")
            if isinstance(model, str):
                request.model = model
            authoritative.append(request)
            if linked:
                duplicate_usage.extend(event.id for event in linked)
        elif linked:
            if len(linked) > 1:
                ambiguous_usage.append(turn_id)
            event = linked[0]
            request = _usage_from_dict(turn_id, "usage", event.id, event.data)
            model = event.data.get("model") or turn.data.get("model")
            if isinstance(model, str):
                request.model = model
            authoritative.append(request)
        # A turn with neither inline nor linked usage stays missing.
    for parent_id, linked in usage_events_by_parent.items():
        if parent_id not in turns:
            linked_without_turn.extend(event.id for event in linked)
    if linked_without_turn:
        result.diagnostics.append(
            Diagnostic(
                "usage_parent_not_a_turn",
                f"{len(linked_without_turn)} usage event(s) link to a parent that is not a "
                f"turn in the filtered set; examples: {', '.join(linked_without_turn[:5])}",
                event_id=linked_without_turn[0],
            )
        )
    for event in orphan_usage:
        authoritative.append(_usage_from_dict(event.id, "usage", event.id, event.data))
    if orphan_usage:
        result.diagnostics.append(
            Diagnostic(
                "orphan_usage",
                f"{len(orphan_usage)} usage event(s) have no parent and are treated as "
                f"independent observations; examples: {', '.join(event.id for event in orphan_usage[:5])}",
                event_id=orphan_usage[0].id,
            )
        )
    if duplicate_usage:
        result.diagnostics.append(
            Diagnostic(
                "duplicate_usage",
                f"{len(duplicate_usage)} linked usage event(s) repeat usage already present "
                f"inline on the parent turn; one authoritative representation per request is "
                f"used. Examples: {', '.join(duplicate_usage[:5])}",
                event_id=duplicate_usage[0],
            )
        )
    for event_id in ambiguous_usage:
        result.diagnostics.append(
            Diagnostic(
                "ambiguous_usage",
                "more than one linked usage event for one request; only the first is used",
                event_id=event_id,
            )
        )

    usage_totals = {
        "input": _sum_optional(item.input for item in authoritative),
        "output": _sum_optional(item.output for item in authoritative),
        "cache_read": _sum_optional(item.cache_read for item in authoritative),
        "cache_write": _sum_optional(item.cache_write for item in authoritative),
        "reasoning": _sum_optional(item.reasoning for item in authoritative),
        "total": _sum_optional(item.total for item in authoritative),
    }
    usage_observed = [item for item in authoritative if item.input is not None or item.output is not None]
    turns_with_usage = {item.request_id for item in authoritative if item.request_id in turns}
    usage_missing = len(turns) - len(turns_with_usage)

    # --- Cost normalization ---------------------------------------------
    observed_cost = 0.0
    cost_with_value = 0
    non_cumulative_values: list[float] = []
    cumulative_entries = [item for item in cost_records if item.cumulative]
    cost_units: Counter[str] = Counter()
    for record in cost_records:
        if record.value is None:
            continue
        cost_with_value += 1
        cost_units["cumulative" if record.cumulative else "per_request"] += 1
        if not record.cumulative:
            non_cumulative_values.append(record.value)
    observed_cost_sum: float | None = (
        sum(non_cumulative_values) if non_cumulative_values else None
    )
    cumulative_total: float | None = None
    if cumulative_entries:
        values = [item.value for item in cumulative_entries if item.value is not None]
        if values:
            cumulative_total = max(values)
        result.diagnostics.append(
            Diagnostic(
                "cumulative_cost",
                "cost records declare cumulative semantics; reported as a maximum observed rollup, not summed",
            )
        )
    observed_cost_status = (
        STATUS_UNAVAILABLE
        if not cost_records
        else STATUS_PARTIAL
        if (
            len(cost_records) != cost_with_value
            or cumulative_entries
            or cost_missing_requests
            or cost_ambiguous_requests
        )
        else STATUS_OBSERVED
    )

    # --- Timing ----------------------------------------------------------
    interval_seconds, interval_blocks = _union_seconds([(a, b) for a, b, _ in intervals_all])
    envelope_seconds: float | None = None
    if first_at is not None and last_at is not None:
        envelope_seconds = max(0.0, (last_at - first_at).total_seconds())
    overlaps = _overlap_pairs(intervals_all)
    unattributed = None
    if envelope_seconds is not None:
        unattributed = max(0.0, envelope_seconds - interval_seconds)

    # --- Failures and retries -------------------------------------------
    # Scan successful events once and match a later success with the same
    # normalized identity. A failed event with no later success is
    # unrecovered; recovery itself is evidence, not proof of avoidable rework.
    successful_events: list[Event] = []
    for event in iter_all(sources):
        if not event_filter.accepts(event):
            continue
        if not is_failure(event) and event.at is not None:
            successful_events.append(event)
    success_index: dict[str, list[Event]] = defaultdict(list)
    for event in successful_events:
        success_index[_failure_key(event)].append(event)
    recovered = []
    unrecovered = []
    for event in failures:
        key = _failure_key(event)
        later = next(
            (
                candidate
                for candidate in success_index.get(key, [])
                if event.at and candidate.at and candidate.at > event.at
            ),
            None,
        )
        if later is not None:
            recovered.append(event)
        else:
            unrecovered.append(event)

    # --- Archive friction ------------------------------------------------
    archive_attempts = [event for event in archive_events if event.kind in ("command", "phase", "step")]
    archive_failures = [event for event in archive_attempts if is_failure(event)]
    dirty_mentions = [
        event
        for event in archive_attempts
        if "dirty" in (error_text(event) + " " + event.label).lower()
    ]

    # --- Subagent spans ---------------------------------------------------
    subagent_rows: list[dict[str, Any]] = []
    for identity, events in sorted(subagent_events.items()):
        duration = max(
            (event.duration_ms for event in events if event.duration_ms is not None),
            default=None,
        )
        status = "unknown"
        for event in events:
            if event.outcome in SUCCESS_OUTCOMES | FAILURE_OUTCOMES:
                status = event.outcome
        usage = next(
            (event.data.get("usage") for event in events if event.data.get("usage") is not None),
            None,
        )
        subagent_rows.append(
            {
                "identity": identity,
                "events": [event.id for event in events],
                "duration_ms": duration,
                "status": status,
                "usage": usage if isinstance(usage, (dict, str)) else None,
                "session": next((event.session for event in events if event.session), None),
            }
        )

    # --- Question waits ---------------------------------------------------
    question_rows: list[dict[str, Any]] = []
    for event in question_events:
        question = event.data.get("question")
        text = ""
        truncated = False
        if isinstance(question, dict):
            text = str(question.get("text") or "")
            truncated = bool(question.get("truncated"))
        elif isinstance(question, str):
            text = question
        text = text or event.label
        answer = (metadata.get("answers") or {}).get(event.id) or (
            metadata.get("answers") or {}
        ).get(str(event.raw_id))
        question_rows.append(
            {
                "event": event.id,
                "text": _bounded(text, 500),
                "truncated": truncated,
                "wait_ms": event.duration_ms,
                "answer": answer if isinstance(answer, str) else None,
                "answer_status": "declared" if isinstance(answer, str) else "not_recorded",
            }
        )

    # --- Context pressure -------------------------------------------------
    context_ratios.sort(reverse=True)
    context_pressure = [
        {"ratio": round(ratio, 3), "raw_id": raw_id, "event": event_id}
        for ratio, raw_id, event_id in context_ratios[:5]
    ]

    expensive_durations = sorted(
        intervals_all, key=lambda item: item[1] - item[0], reverse=True
    )[:15]

    # --- Data quality summary ---------------------------------------------
    diagnostic_codes = Counter(item.code for item in result.diagnostics)
    coverage = {
        "events_seen": events_seen,
        "events_analyzed": events_seen - filtered_out,
        "events_filtered_out": filtered_out,
        "events_without_timestamp": events_without_at,
        "events_without_duration": sum(missing_duration_by_kind.values()),
    }

    data: dict[str, Any] = {
        "schema_version": 1,
        "corpus": {
            "sources": len(sources),
            "bytes": sum(source.boundary_bytes for source in sources),
            "events": events_seen,
            "analyzed_events": events_seen - filtered_out,
            "filtered_out": filtered_out,
            "kinds": dict(kind_counts.most_common()),
            "producers": dict(producer_counts.most_common()),
            "outcomes": dict(outcome_counts.most_common()),
            "schemas": dict(schema_counts),
            "scopes": dict(scope_counts.most_common(50)),
            "sessions": dict(session_counts.most_common(50)),
            "session_count": len(session_counts),
            "scope_count": len(scope_counts),
            "first_event_at": _stamp(first_at),
            "last_event_at": _stamp(last_at),
        },
        "data_quality": {
            "diagnostics": [item.to_dict() for item in result.diagnostics],
            "diagnostic_codes": dict(diagnostic_codes.most_common()),
            "events_without_timestamp": events_without_at,
            "events_without_duration": sum(missing_duration_by_kind.values()),
            "unresolved_parents": len(missing_parents),
            "duplicate_usage_events": len(duplicate_usage),
            "orphan_usage_events": len(orphan_usage),
            "ambiguous_usage_requests": len(ambiguous_usage),
            "model_identities": dict(model_counts.most_common()),
            "declared_metadata": metadata,
        },
        "time": {
            "envelope_seconds": envelope_seconds,
            "recorded_interval_union_seconds": round(interval_seconds, 3),
            "recorded_interval_blocks": interval_blocks,
            "unattributed_envelope_seconds": round(unattributed, 3) if unattributed is not None else None,
            "duration_sum_by_kind_seconds": {
                kind: round(value / 1000.0, 3) for kind, value in durations_by_kind.most_common()
            },
            "duration_sum_by_producer_seconds": {
                key: round(value / 1000.0, 3) for key, value in durations_by_producer.most_common()
            },
            "duration_sum_by_category_seconds": {
                key: round(value / 1000.0, 3) for key, value in durations_by_category.most_common()
            },
            "duration_event_count_by_kind": dict(duration_count_by_kind.most_common()),
            "missing_duration_by_kind": dict(missing_duration_by_kind.most_common()),
            "overlap_examples": [
                {"a": left, "b": right} for left, right in overlaps
            ],
            "overlap_note": (
                "Nested commands, tools, turns and subagents overlap. The union is "
                "recorded active time, not trial elapsed time; per-kind sums overlap "
                "and must never be added into wall time."
            ),
        },
        "usage": {
            "requests_with_usage": len(usage_observed),
            "turns": len(turns),
            "turns_without_usage": usage_missing,
            "input_tokens": usage_totals["input"],
            "output_tokens": usage_totals["output"],
            "cache_read_tokens": usage_totals["cache_read"],
            "cache_write_tokens": usage_totals["cache_write"],
            "reasoning_tokens": usage_totals["reasoning"],
            "total_tokens": usage_totals["total"],
            "reasoning_included_in_output": False,
            "duplicate_usage_not_counted": len(duplicate_usage),
            "linked_usage_without_turn": len(linked_without_turn),
            "orphan_usage": len(orphan_usage),
            "models": dict(model_counts.most_common()),
            "stop_reasons": dict(stop_reasons.most_common()),
        },
        "cost": {
            "observed_total_usd": round(observed_cost_sum, 8) if observed_cost_sum is not None else None,
            "cumulative_rollup_usd": round(cumulative_total, 8) if cumulative_total is not None else None,
            "records": len(cost_records),
            "records_with_value": cost_with_value,
            "missing_cost_requests": cost_missing_requests,
            "ambiguous_cost_requests": cost_ambiguous_requests,
            "billing_basis": "producer-reported; not an invoice",
            "units": dict(cost_units),
            "status": observed_cost_status,
        },
        "failures": {
            "total": len(failures),
            "recovered": len(recovered),
            "unrecovered": len(unrecovered),
            "rows": [_failure_row(event) for event in failures[:100]],
            "recovered_event_ids": [event.id for event in recovered[:100]],
            "unrecovered_event_ids": [event.id for event in unrecovered[:100]],
            "note": (
                "An error or nonzero command can be expected evidence (for example a "
                "RED test). A repeated command is not automatically waste."
            ),
        },
        "commands": {
            "repeated": [
                {
                    "normalized": key,
                    "count": count,
                    "outcomes": dict(command_outcomes.get(key, {})),
                    "recovered": command_outcomes.get(key, {}).get("ok", 0) > 0,
                }
                for key, count in command_counts.most_common(30)
                if count > 1
            ],
            "archive_attempts": len(archive_attempts),
            "archive_failures": len(archive_failures),
            "archive_dirty_mentions": len(dirty_mentions),
            "archive_events": [_event_row(event) for event in archive_attempts[:50]],
            "recorder_plumbing_excluded": sum(
                1 for event in failures if is_recorder_plumbing(event)
            ),
        },
        "phases": {
            "markers": [_event_row(event) for event in phase_events],
            "note": (
                "Phase markers are the first observation of an artifact being Done, "
                "not proof of when work started. They do not define work intervals."
            ),
        },
        "questions": {
            "count": len(question_rows),
            "total_wait_ms": sum(
                int(event.duration_ms) for event in question_events if event.duration_ms is not None
            ),
            "rows": question_rows,
            "note": (
                "A question record may contain text and wait time but no answer. "
                "Silence, counts and final artifacts do not prove approval."
            ),
        },
        "context": {
            "pressure": context_pressure,
            "max_ratio": round(context_ratios[0][0], 3) if context_ratios else None,
            "turns_over_80_percent": sum(1 for ratio, _, _ in context_ratios if ratio >= 0.8),
        },
        "subagents": {
            "span_count": len(subagent_rows),
            "rows": subagent_rows,
            "note": (
                "Subagent spans overlap their parent. Their durations are not added "
                "to wall time, and unavailable subagent usage stays unavailable."
            ),
        },
        "expensive_operations": [
            {
                "event": event_id,
                "duration_seconds": round(end - start, 3),
            }
            for start, end, event_id in expensive_durations
        ],
        "coverage": coverage,
    }

    result.data = data
    _add_measurements(result, data)
    _add_findings(result, data)
    return result


def _category(event: Event) -> str:
    if is_recorder_plumbing(event):
        return "recorder_plumbing"
    if event.kind == "command":
        return "givn_command"
    if event.kind == "step":
        return "givn_step"
    if event.kind == "phase":
        return "givn_phase"
    return event.kind or "unknown"


def _usage_from_dict(
    request_id: str, source: str, evidence: str, data: dict[str, Any]
) -> _UsageRequest:
    return _UsageRequest(
        request_id=request_id,
        source=source,
        evidence=evidence,
        input=_first_number(data, USAGE_INPUT_KEYS),
        output=_first_number(data, USAGE_OUTPUT_KEYS),
        cache_read=_first_number(data, USAGE_CACHE_READ_KEYS),
        cache_write=_first_number(data, USAGE_CACHE_WRITE_KEYS),
        reasoning=_first_number(data, USAGE_REASONING_KEYS),
        total=_first_number(data, ("total_tokens", "total")),
        model=data.get("model") if isinstance(data.get("model"), str) else None,
    )


def _cost_from_value(
    evidence: str, value: Any, data: dict[str, Any]
) -> _CostRecord | None:
    cumulative = bool(
        data.get("cost_cumulative")
        or (isinstance(value, dict) and (value.get("cumulative") or value.get("kind") == "cumulative"))
    )
    if isinstance(value, bool):
        return None
    if isinstance(value, (int, float)):
        return _CostRecord(evidence, float(value), None, cumulative, "numeric")
    if isinstance(value, dict):
        total = _as_float(value.get("total"))
        components: dict[str, float] = {}
        for key in COST_COMPONENT_KEYS:
            number = _as_float(value.get(key))
            if number is not None:
                components[key] = number
        if total is None and components:
            total = sum(components.values())
        if total is None:
            return None
        return _CostRecord(evidence, total, components or None, cumulative, "object")
    return None


def _sum_optional(values: Iterable[float | None]) -> float | None:
    present = [value for value in values if value is not None]
    if not present:
        return None
    return float(sum(present))


def _bounded(text: str, limit: int) -> str:
    text = " ".join(str(text).split())
    return text if len(text) <= limit else text[: limit - 1] + "…"


def _failure_key(event: Event) -> str:
    return _normalize_label(f"{event.kind}:{event.name}:{event.label}")


def _failure_row(event: Event) -> dict[str, Any]:
    return {
        "event": event.id,
        "kind": event.kind,
        "name": event.name,
        "label": _bounded(event.label, 240),
        "outcome": event.outcome,
        "error": error_text(event) if is_failure(event) else "",
        "duration_ms": event.duration_ms,
        "at": _stamp(event.at),
        "recorder_plumbing": is_recorder_plumbing(event),
    }


def _event_row(event: Event) -> dict[str, Any]:
    return {
        "event": event.id,
        "kind": event.kind,
        "name": event.name,
        "label": _bounded(event.label, 240),
        "outcome": event.outcome,
        "duration_ms": event.duration_ms,
        "at": _stamp(event.at),
    }


def _stamp(value: datetime | None) -> str | None:
    return value.isoformat().replace("+00:00", "Z") if value else None


def _add_measurements(result: Analysis, data: dict[str, Any]) -> None:
    corpus = data["corpus"]
    time_data = data["time"]
    usage = data["usage"]
    cost = data["cost"]

    def add(
        identifier: str,
        value: Any,
        unit: str,
        status: str,
        provenance: str,
        note: str = "",
        evidence: list[str] | None = None,
    ) -> None:
        result.measurements.append(
            Measurement(
                id=identifier,
                value=value,
                unit=unit,
                status=status,
                provenance=provenance,
                note=note,
                evidence=evidence or [],
            )
        )

    add(
        "events.total",
        corpus["events"],
        "events",
        STATUS_OBSERVED,
        "count of decoded JSONL records across the frozen sources",
    )
    add(
        "events.analyzed",
        corpus["analyzed_events"],
        "events",
        STATUS_OBSERVED,
        "events passing the explicit filters",
    )
    add(
        "time.envelope",
        time_data["envelope_seconds"],
        "seconds",
        STATUS_OBSERVED,
        "last minus first event timestamp; includes idle and unrecorded time",
        "Not trial elapsed time.",
    )
    add(
        "time.recorded_union",
        time_data["recorded_interval_union_seconds"],
        "seconds",
        STATUS_OBSERVED,
        "union of [at - duration_ms, at] intervals",
        "Overlapping nested work is counted once.",
        [item["event"] for item in data["expensive_operations"][:5]],
    )
    add(
        "time.unattributed_envelope",
        time_data["unattributed_envelope_seconds"],
        "seconds",
        STATUS_PARTIAL,
        "envelope minus recorded interval union",
        "Unrecorded time is not necessarily idle and may include work outside the recorder.",
    )
    add(
        "usage.input_tokens",
        usage["input_tokens"],
        "tokens",
        STATUS_OBSERVED if usage["turns_without_usage"] == 0 else STATUS_PARTIAL,
        "one authoritative usage representation per request (inline turn usage preferred)",
        "Reasoning tokens are reported separately and are not added to output.",
    )
    add(
        "usage.output_tokens",
        usage["output_tokens"],
        "tokens",
        STATUS_OBSERVED if usage["turns_without_usage"] == 0 else STATUS_PARTIAL,
        "one authoritative usage representation per request",
    )
    add(
        "usage.total_tokens",
        usage["total_tokens"],
        "tokens",
        STATUS_OBSERVED if usage["turns_without_usage"] == 0 else STATUS_PARTIAL,
        "producer-reported total tokens",
    )
    add(
        "cost.observed",
        cost["observed_total_usd"],
        "USD",
        cost["status"],
        "sum of per-request producer-reported cost values; not an invoice",
        "Cumulative rollups are not summed; missing cost stays missing.",
    )
    add(
        "failures.total",
        data["failures"]["total"],
        "events",
        STATUS_OBSERVED,
        "events whose outcome is failed/error or whose exit code is nonzero",
        "Expected failures, such as RED tests, are included; see recovered/unrecovered.",
    )
    add(
        "archive.attempts",
        data["commands"]["archive_attempts"],
        "events",
        STATUS_OBSERVED,
        "commands/phases/steps whose name or label mentions archive",
    )


def _add_findings(result: Analysis, data: dict[str, Any]) -> None:
    commands = data["commands"]
    time_data = data["time"]
    cost = data["cost"]
    quality = data["data_quality"]
    context = data["context"]
    questions = data["questions"]
    subagents = data["subagents"]

    def add(
        identifier: str,
        title: str,
        observation: str,
        hypothesis: str,
        severity: str,
        evidence: list[str],
        limitations: str,
        status: str = STATUS_OBSERVED,
    ) -> None:
        result.findings.append(
            Finding(
                id=identifier,
                title=title,
                observation=observation,
                hypothesis=hypothesis,
                severity=severity,
                evidence=evidence[:12],
                limitations=limitations,
                status=status,
            )
        )

    if commands["archive_failures"] >= 2 or (
        commands["archive_attempts"] >= 3 and commands["archive_failures"] >= 1
    ):
        archive_evidence = [row["event"] for row in commands["archive_events"][:6]]
        add(
            "archive-friction",
            "Archive was attempted repeatedly and failed at least once",
            (
                f"{commands['archive_attempts']} archive-related event(s), "
                f"{commands['archive_failures']} failure(s), "
                f"{commands['archive_dirty_mentions']} explicit dirty-tree mention(s)."
            ),
            (
                "Inspect the cited gate output. If dirty-tree or generated-file "
                "rejection is the recurring cause, propose a focused guardrail fix and "
                "test it in a controlled comparison."
            ),
            "warning",
            archive_evidence,
            "The record shows attempts and outcomes, not their cause or the counterfactual without them.",
        )

    repeated_failures = [
        row for row in commands["repeated"] if row["outcomes"].get("failed", 0) > 1
    ]
    if repeated_failures:
        add(
            "repeated-failures",
            "The same command or step failed more than once",
            "; ".join(
                f"{row['normalized']!r} x{row['count']} (failed {row['outcomes'].get('failed', 0)})"
                for row in repeated_failures[:3]
            ),
            (
                "A repeated failing command can be friction or an expected RED cycle. "
                "Inspect the cited outputs to separate the two before changing guidance."
            ),
            "info",
            [],
            "Repetition alone does not prove avoidable rework.",
        )

    if cost["missing_cost_requests"]:
        add(
            "cost-missing",
            "Some requests have no recorded cost",
            f"{cost['missing_cost_requests']} turn(s) carry neither cost nor usage-derived cost.",
            "Do not compute a campaign cost from the observed subset. Reconcile missing usage before using cost as a comparison signal.",
            "warning",
            [],
            "Missing cost is not zero cost.",
            STATUS_PARTIAL,
        )

    if quality["duplicate_usage_events"] or quality["orphan_usage_events"]:
        add(
            "usage-recording-ambiguity",
            "Usage records are duplicated or unlinked",
            (
                f"{quality['duplicate_usage_events']} linked usage event(s) duplicate inline usage; "
                f"{quality['orphan_usage_events']} usage event(s) have no parent."
            ),
            "Keep the one-authoritative-representation rule; this is a recorder/provenance observation, not product friction.",
            "info",
            [],
            "No request totals were double-counted by this analyzer.",
        )

    if context["max_ratio"] is not None and context["max_ratio"] >= 0.8:
        add(
            "context-pressure",
            "Context use approached the recorded window",
            f"Maximum context/window ratio observed: {context['max_ratio']:.0%}; {context['turns_over_80_percent']} turn(s) at or above 80%.",
            "Check compaction events and whether the expensive turns cluster after context growth. Do not infer lost correctness from pressure alone.",
            "info",
            [row["event"] for row in context["pressure"]],
            "Only turns that recorded both context size and window are included.",
        )

    if questions["count"]:
        add(
            "question-waits",
            "Recorded questions carry wait time but often no answer",
            f"{questions['count']} question(s), {questions['total_wait_ms'] / 1000:.1f}s total recorded wait.",
            "Review the cited question text and answer records (if a sidecar supplies them) before treating wait time as avoidable.",
            "info",
            [row["event"] for row in questions["rows"][:6]],
            "Silence, counts and final artifacts do not prove approval.",
        )

    long_subagents = [
        row
        for row in subagents["rows"]
        if row.get("duration_ms") and row["duration_ms"] >= 300_000
    ]
    if long_subagents:
        evidence: list[str] = []
        for row in long_subagents[:5]:
            evidence.extend(row["events"][:2])
        add(
            "long-subagents",
            "One or more subagent spans consumed five minutes or more",
            "; ".join(
                f"{row['identity']} {row['duration_ms'] / 1000:.0f}s ({row['status']})"
                for row in long_subagents[:5]
            ),
            (
                "Inspect the subagents' purposes and outputs. A long span is a candidate "
                "for review, not proof of waste."
            ),
            "info",
            evidence,
            "Subagent duration overlaps parent work and is not added to wall time.",
        )

    if (
        time_data["unattributed_envelope_seconds"] is not None
        and time_data["envelope_seconds"]
        and time_data["unattributed_envelope_seconds"] / time_data["envelope_seconds"] >= 0.5
    ):
        add(
            "unattributed-time",
            "Most of the observed envelope has no recorded work interval",
            (
                f"{time_data['unattributed_envelope_seconds']:.1f}s of "
                f"{time_data['envelope_seconds']:.1f}s is outside any recorded interval."
            ),
            "Prefer externally captured stage boundaries before attributing this time to a workflow step.",
            "info",
            [],
            "Unrecorded time is not necessarily idle or wasted.",
            STATUS_PARTIAL,
        )

    if quality["unresolved_parents"]:
        add(
            "unresolved-parents",
            "Some parent links do not resolve",
            f"{quality['unresolved_parents']} parent reference(s) point outside the filtered set or are missing.",
            "Treat those spans as ambiguous; do not merge them by guess. Re-export with the full session if attribution matters.",
            "info",
            [],
            "No guessed merges were made.",
        )


def analyze_paths(
    paths: list[str],
    *,
    sessions: Iterable[str] | None = None,
    scopes: Iterable[str] | None = None,
    since: str | None = None,
    until: str | None = None,
    kinds: Iterable[str] | None = None,
    metadata: dict[str, Any] | None = None,
    repository: str | None = None,
) -> Analysis:
    from givn_harness.events import read_sources

    sources = read_sources(paths)
    event_filter = build_filter(sessions, scopes, since, until, kinds)
    analysis = analyze_sources(sources, event_filter, metadata=metadata)
    if repository:
        analysis.data["repository"] = repository_metadata(repository)
    return analysis


def repository_metadata(path: str) -> dict[str, Any]:
    """Read-only repository context. Never executes repository code."""
    import subprocess

    root = Path(path).resolve()
    if not root.is_dir():
        return {"path": str(root), "available": False, "error": "not a directory"}

    def git(*args: str) -> str:
        env = dict(os.environ)
        env["GIT_OPTIONAL_LOCKS"] = "0"
        try:
            completed = subprocess.run(
                ["git", "-C", str(root), *args],
                capture_output=True,
                text=True,
                timeout=20,
                env=env,
            )
        except (OSError, subprocess.TimeoutExpired):
            return ""
        return completed.stdout if completed.returncode == 0 else ""

    head = git("rev-parse", "HEAD").strip()
    if not head:
        return {
            "path": str(root),
            "available": False,
            "error": "not a git repository",
            "note": "Missing repository context limits attribution; it does not block analysis.",
        }
    return {
        "path": str(root),
        "available": True,
        "head": head,
        "short_head": head[:12],
        "branch": git("rev-parse", "--abbrev-ref", "HEAD").strip(),
        "dirty": bool(git("status", "--porcelain=v1", "-uall").strip()),
        "status": git("status", "--porcelain=v1", "-uall").splitlines()[:100],
        "recent_commits": git("log", "--oneline", "--decorate", "-10").splitlines()[:10],
        "note": (
            "Repository state is read-only context. Commit timestamps are not "
            "ground truth for time spent or workflow stage boundaries."
        ),
    }
