"""Markdown rendering for retrospective analyses."""

from __future__ import annotations

from typing import Any

from givn_harness.accounting import Analysis
from givn_harness.types import Measurement


def _value(value: Any) -> str:
    if value is None:
        return "unavailable"
    if isinstance(value, float):
        if value.is_integer():
            return f"{int(value):,}"
        return f"{value:,.3f}".rstrip("0").rstrip(".")
    if isinstance(value, int):
        return f"{value:,}"
    return str(value)


def _seconds(value: Any) -> str:
    if not isinstance(value, (int, float)):
        return "unavailable"
    seconds = float(value)
    if seconds < 60:
        return f"{seconds:.1f}s"
    if seconds < 3600:
        return f"{seconds / 60:.1f}min"
    return f"{seconds / 3600:.2f}h"


def _table(headers: list[str], rows: list[list[Any]]) -> list[str]:
    lines = [
        "| " + " | ".join(headers) + " |",
        "|" + "|".join("---" for _ in headers) + "|",
    ]
    for row in rows:
        lines.append("| " + " | ".join(str(cell) for cell in row) + " |")
    return lines


def _measurement_row(item: Measurement) -> list[str]:
    return [
        f"`{item.id}`",
        _value(item.value),
        item.unit,
        item.status,
        item.provenance,
        item.note,
        ", ".join(f"`{event}`" for event in item.evidence[:4]) or "",
    ]


def render_analysis(analysis: Analysis) -> str:
    data = analysis.data
    corpus = data["corpus"]
    quality = data["data_quality"]
    time_data = data["time"]
    usage = data["usage"]
    cost = data["cost"]
    failures = data["failures"]
    commands = data["commands"]
    phases = data["phases"]
    questions = data["questions"]
    context = data["context"]
    subagents = data["subagents"]

    lines: list[str] = []
    lines.append("# Givn record analysis")
    lines.append("")
    lines.append(
        "Retrospective analysis of recorded events. This is not a controlled "
        "evaluation, an invoice, or a product-quality score."
    )
    lines.append("")

    lines.append("## Data quality first")
    lines.append("")
    lines.append(f"- Events decoded: **{_value(corpus['events'])}**")
    lines.append(f"- Events analyzed after filters: **{_value(corpus['analyzed_events'])}**")
    lines.append(
        f"- Input diagnostics (including expected duplicate-usage notes): "
        f"**{_value(len(analysis.diagnostics))}**"
    )
    if quality["diagnostic_codes"]:
        lines.append(
            "- Diagnostic codes: "
            + ", ".join(
                f"`{code}` x{count}"
                for code, count in list(quality["diagnostic_codes"].items())[:12]
            )
        )
    lines.append(f"- Events without a timestamp: {_value(quality['events_without_timestamp'])}")
    lines.append(f"- Events without a duration: {_value(quality['events_without_duration'])}")
    lines.append(f"- Unresolved parent links: {_value(quality['unresolved_parents'])}")
    lines.append(f"- Duplicate linked usage events: {_value(quality['duplicate_usage_events'])}")
    lines.append(f"- Orphan usage events: {_value(quality['orphan_usage_events'])}")
    if quality["model_identities"]:
        lines.append(
            "- Recorded model identities: "
            + ", ".join(f"`{name}` x{count}" for name, count in quality["model_identities"].items())
        )
    if quality["declared_metadata"]:
        lines.append(
            "- Declared sidecar metadata (declared, not observed): "
            + ", ".join(f"`{key}`" for key in sorted(quality["declared_metadata"]))
        )
    lines.append("")

    lines.append("## What this analysis can and cannot establish")
    lines.append("")
    lines.append(
        "- It can establish observed event counts, available durations, usage, "
        "cost values, errors and repeated patterns."
    )
    lines.append(
        "- It cannot establish product correctness, whether a review prevented a "
        "defect, or whether removing a rule would improve results. Those remain "
        "unmeasured."
    )
    lines.append(
        "- The observed envelope is not trial elapsed time, and the recorded "
        "interval union is not uninterrupted active work."
    )
    lines.append("")

    lines.append("## Inputs")
    lines.append("")
    inputs = []
    for source in analysis.sources:
        inputs.append(
            [
                f"`{source.path.name}`",
                f"`{source.sha256[:16]}`",
                _value(source.boundary_bytes),
                _value(source.total_bytes - source.boundary_bytes),
                str(source.path),
            ]
        )
    lines.extend(
        _table(["File", "SHA-256", "Bytes read", "Bytes appended later", "Path"], inputs)
    )
    lines.append("")
    lines.append("Filters: `" + repr(analysis.filters) + "`")
    lines.append("")
    repository = data.get("repository")
    if repository:
        lines.append("## Repository context")
        lines.append("")
        if repository.get("available"):
            lines.append(f"- Path: `{repository['path']}`")
            lines.append(f"- HEAD: `{repository.get('short_head')}` on `{repository.get('branch')}`")
            lines.append(f"- Dirty working tree: {repository.get('dirty')}")
            if repository.get("recent_commits"):
                lines.append("- Recent commits: " + "; ".join(f"`{line}`" for line in repository["recent_commits"]))
        else:
            lines.append(f"- Repository context unavailable: {repository.get('error')}")
        lines.append(
            "- Repository state is context, not time evidence. Commit timestamps "
            "are not proof of when work started or ended."
        )
        lines.append("")

    lines.append("## Corpus")
    lines.append("")
    lines.append(
        f"- Time span: {corpus['first_event_at'] or 'unavailable'} to {corpus['last_event_at'] or 'unavailable'}"
    )
    lines.append(f"- Sessions: {_value(corpus['session_count'])}; scopes: {_value(corpus['scope_count'])}")
    lines.append("- Kinds: " + ", ".join(f"`{k}` {v}" for k, v in corpus["kinds"].items()))
    lines.append("- Producers: " + ", ".join(f"`{k}` {v}" for k, v in corpus["producers"].items()))
    lines.append(
        "- Outcomes: " + ", ".join(f"`{k}` {v}" for k, v in list(corpus["outcomes"].items())[:12])
    )
    lines.append("")

    lines.append("## Time accounting")
    lines.append("")
    lines.append(
        "- Observed envelope (last minus first timestamp): "
        f"**{_seconds(time_data['envelope_seconds'])}**"
    )
    lines.append(
        "- Recorded interval union (`[at - duration, at]`, overlaps counted once): "
        f"**{_seconds(time_data['recorded_interval_union_seconds'])}**"
    )
    lines.append(
        "- Envelope outside any recorded interval: "
        f"**{_seconds(time_data['unattributed_envelope_seconds'])}** (unattributed, not idle)"
    )
    lines.append("")
    lines.append(
        "Per-kind duration sums overlap and are therefore reported separately. "
        "They are never added into wall time."
    )
    lines.append("")
    rows = [
        [f"`{kind}`", _seconds(seconds)]
        for kind, seconds in list(time_data["duration_sum_by_kind_seconds"].items())[:12]
    ]
    lines.extend(_table(["Kind", "Duration sum"], rows))
    lines.append("")

    lines.append("## Usage")
    lines.append("")
    lines.append(f"- Turns: {_value(usage['turns'])}; turns without usage: {_value(usage['turns_without_usage'])}")
    lines.append(
        "- Tokens: "
        f"input {_value(usage['input_tokens'])}, output {_value(usage['output_tokens'])}, "
        f"cache read {_value(usage['cache_read_tokens'])}, cache write {_value(usage['cache_write_tokens'])}, "
        f"reasoning {_value(usage['reasoning_tokens'])}, total {_value(usage['total_tokens'])}"
    )
    lines.append(
        f"- Duplicate linked usage events not counted: {_value(usage['duplicate_usage_not_counted'])}"
    )
    lines.append(
        "- Reasoning tokens are reported separately and are not added to output tokens; "
        "their inclusion semantics are not assumed."
    )
    lines.append("")

    lines.append("## Cost")
    lines.append("")
    lines.append(f"- Observed cost: **{_value(cost['observed_total_usd'])} USD** ({cost['status']})")
    if cost["cumulative_rollup_usd"] is not None:
        lines.append(
            f"- Cumulative rollup observed (maximum, not summed): {_value(cost['cumulative_rollup_usd'])} USD"
        )
    lines.append(
        f"- Cost records with a value: {_value(cost['records_with_value'])}/{_value(cost['records'])}; "
        f"turns missing cost: {_value(cost['missing_cost_requests'])}"
    )
    lines.append(
        "- Billing basis: producer-reported values, not an invoice. Missing cache or "
        "usage information stays missing. No list-price estimate is substituted."
    )
    lines.append("")

    lines.append("## Failures, retries and archive friction")
    lines.append("")
    lines.append(
        f"- Failure events: {_value(failures['total'])}; recovered later: {_value(failures['recovered'])}; "
        f"unrecovered in this record: {_value(failures['unrecovered'])}"
    )
    lines.append(
        f"- Archive-related events: {_value(commands['archive_attempts'])}; failures: "
        f"{_value(commands['archive_failures'])}; explicit dirty-tree mentions: "
        f"{_value(commands['archive_dirty_mentions'])}"
    )
    lines.append(
        "- An error or nonzero command can be expected evidence, such as a RED test. "
        "Recovery is evidence; it is not proof that the failure was avoidable."
    )
    lines.append("")
    if failures["rows"]:
        rows = []
        for row in failures["rows"][:15]:
            rows.append(
                [
                    f"`{row['event']}`",
                    row["kind"],
                    row["name"],
                    row["outcome"],
                    row["duration_ms"] if row["duration_ms"] is not None else "",
                    row["error"][:160],
                ]
            )
        lines.extend(_table(["Event", "Kind", "Name", "Outcome", "ms", "Error"], rows))
        lines.append("")

    if commands["repeated"]:
        lines.append("Repeated command identities (counted after volatile tokens are normalized):")
        lines.append("")
        rows = []
        for row in commands["repeated"][:15]:
            rows.append(
                [
                    f"`{row['normalized'][:120]}`",
                    row["count"],
                    ", ".join(f"{key}={value}" for key, value in row["outcomes"].items()),
                    "yes" if row["recovered"] else "no",
                ]
            )
        lines.extend(_table(["Command", "Count", "Outcomes", "Later success"], rows))
        lines.append("")

    lines.append("## Phase markers")
    lines.append("")
    if phases["markers"]:
        rows = []
        for row in phases["markers"]:
            rows.append(
                [
                    f"`{row['event']}`",
                    row["name"],
                    row["label"],
                    row["outcome"],
                    row["at"] or "",
                ]
            )
        lines.extend(_table(["Event", "Name", "Label", "Outcome", "At"], rows))
        lines.append("")
    lines.append(
        "Phase markers mean the first observation of an artifact being Done. "
        "They are not proof of when work started and are not used as phase intervals."
    )
    lines.append("")

    lines.append("## Questions")
    lines.append("")
    lines.append(
        f"{_value(questions['count'])} question(s), total recorded wait "
        f"{_seconds(questions['total_wait_ms'] / 1000)}."
    )
    if questions["rows"]:
        lines.append("")
        rows = []
        for row in questions["rows"]:
            rows.append(
                [
                    f"`{row['event']}`",
                    _seconds((row["wait_ms"] or 0) / 1000),
                    row["text"][:160],
                    row["answer"] or "(no answer recorded)",
                ]
            )
        lines.extend(_table(["Event", "Recorded wait", "Question text", "Answer"], rows))
    lines.append("")
    lines.append("Silence, counts and final artifacts do not prove approval.")
    lines.append("")

    lines.append("## Context pressure")
    lines.append("")
    if context["max_ratio"] is None:
        lines.append("No turn recorded both context size and context window.")
    else:
        lines.append(
            f"Maximum recorded context/window ratio: **{context['max_ratio']:.0%}**; "
            f"{context['turns_over_80_percent']} turn(s) at or above 80%."
        )
    lines.append("")

    lines.append("## Subagents")
    lines.append("")
    if subagents["rows"]:
        rows = []
        for row in subagents["rows"]:
            rows.append(
                [
                    row["identity"],
                    row["status"],
                    _seconds((row["duration_ms"] or 0) / 1000) if row["duration_ms"] else "unavailable",
                    ", ".join(f"`{event}`" for event in row["events"][:3]),
                    "unavailable" if row["usage"] == "unavailable" else (str(row["usage"])[:60] if row["usage"] else "unrecorded"),
                ]
            )
        lines.extend(_table(["Identity", "Status", "Duration", "Events", "Usage"], rows))
        lines.append("")
    lines.append(
        "Subagent spans overlap their parent and are not added to wall time. "
        "Unavailable subagent usage stays unavailable."
    )
    lines.append("")

    lines.append("## Expensive recorded operations")
    lines.append("")
    if data["expensive_operations"]:
        rows = [
            [f"`{row['event']}`", _seconds(row["duration_seconds"])]
            for row in data["expensive_operations"][:15]
        ]
        lines.extend(_table(["Event", "Recorded duration"], rows))
    else:
        lines.append("No event carried a usable duration.")
    lines.append("")

    lines.append("## Evidence-linked findings")
    lines.append("")
    if not analysis.findings:
        lines.append("No heuristic finding met its evidence threshold. That is not proof of smooth execution.")
    for finding in analysis.findings:
        lines.append(f"### {finding.title}")
        lines.append("")
        lines.append(f"- Severity: `{finding.severity}`; status: `{finding.status}`")
        lines.append(f"- Observation: {finding.observation}")
        lines.append(f"- Hypothesis: {finding.hypothesis}")
        if finding.evidence:
            lines.append(
                "- Evidence: " + ", ".join(f"`{event}`" for event in finding.evidence)
            )
        if finding.limitations:
            lines.append(f"- Limitation: {finding.limitations}")
        lines.append("")

    lines.append("## Measurements")
    lines.append("")
    lines.extend(
        _table(
            ["Measurement", "Value", "Unit", "Status", "Provenance", "Note", "Evidence"],
            [_measurement_row(item) for item in analysis.measurements],
        )
    )
    lines.append("")

    lines.append("## Diagnostics")
    lines.append("")
    if not analysis.diagnostics:
        lines.append("No damaged or ambiguous input was observed.")
    else:
        rows = []
        for item in analysis.diagnostics[:80]:
            rows.append(
                [
                    item.code,
                    item.source,
                    item.line or "",
                    item.event_id or "",
                    item.message[:200],
                ]
            )
        lines.extend(_table(["Code", "Source", "Line", "Event", "Message"], rows))
        if len(analysis.diagnostics) > 80:
            lines.append("")
            lines.append(f"({len(analysis.diagnostics) - 80} more diagnostics omitted.)")
    lines.append("")

    lines.append("## Improvement loop")
    lines.append("")
    lines.append(
        "1. Inspect the cited evidence; do not act on a label alone."
    )
    lines.append(
        "2. Form one narrow hypothesis about givn guidance, a guardrail or a rule."
    )
    lines.append(
        "3. Change one thing and compare current versus candidate in a controlled benchmark."
    )
    lines.append(
        "4. Retain or revise only when independent grading supports the change."
    )
    lines.append("")
    return "\n".join(lines)
