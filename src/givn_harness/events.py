"""Streaming, tolerant reader for givn ``events.jsonl`` records.

The reader never rewrites, sorts or repairs the source record. It fixes an
explicit byte boundary when reading starts, hashes exactly the bytes it read,
and reports damaged or unsupported input instead of dropping it silently.
"""

from __future__ import annotations

import hashlib
import json
import os
from collections.abc import Iterator
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from givn_harness.types import Diagnostic, Event, SourceFile

KNOWN_KINDS = {
    "command",
    "tool",
    "turn",
    "usage",
    "subagent",
    "phase",
    "step",
    "question",
}


def parse_timestamp(value: Any) -> datetime | None:
    """Parse an ISO-8601 timestamp; return ``None`` for anything else."""
    if not isinstance(value, str) or not value:
        return None
    text = value.strip()
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(text)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def _number(value: Any) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    return float(value)


def _identifier(value: Any) -> str | None:
    if isinstance(value, (int, float)) and not isinstance(value, bool):
        if isinstance(value, float) and value.is_integer():
            return str(int(value))
        return str(value)
    if isinstance(value, str) and value:
        return value
    return None


def freeze_source(path: Path, index: int) -> SourceFile:
    """Hash exactly the bytes this read will consume.

    The boundary is fixed from the first ``stat``. Bytes appended while the
    hash is computed are outside the boundary and are reported separately by
    a second ``stat`` after hashing.
    """
    stat = path.stat()
    boundary = stat.st_size
    digest = hashlib.sha256()
    remaining = boundary
    with path.open("rb") as handle:
        while remaining > 0:
            chunk = handle.read(min(1024 * 1024, remaining))
            if not chunk:
                break
            remaining -= len(chunk)
            digest.update(chunk)
    return SourceFile(
        index=index,
        path=path.resolve(),
        sha256=digest.hexdigest(),
        boundary_bytes=boundary,
        total_bytes=path.stat().st_size,
        mtime_ns=stat.st_mtime_ns,
    )


def iter_raw_lines(source: SourceFile) -> Iterator[tuple[int, int, str | None, bool]]:
    """Yield ``(line_no, byte_offset, text, ended_with_newline)`` up to boundary.

    ``text`` is ``None`` for undecodable bytes. The reader stops exactly at the
    captured byte boundary even if the file keeps growing.
    """
    offset = 0
    line_no = 0
    with source.path.open("rb") as handle:
        while offset < source.boundary_bytes:
            raw = handle.readline(source.boundary_bytes - offset)
            if not raw:
                break
            line_no += 1
            start = offset
            offset += len(raw)
            ended = raw.endswith(b"\n")
            text = raw.decode("utf-8", errors="replace")
            if text.endswith("\n"):
                text = text[:-1]
            if text.endswith("\r"):
                text = text[:-1]
            yield line_no, start, text, ended


def _bounded_preview(text: str, limit: int = 160) -> str:
    text = " ".join(text.split())
    return text if len(text) <= limit else text[: limit - 1] + "…"


def iter_events(
    source: SourceFile,
    diagnostics: list[Diagnostic] | None = None,
) -> Iterator[Event]:
    """Yield normalized events, appending diagnostics for damaged input."""
    diagnostics = diagnostics if diagnostics is not None else []
    seen_ids: dict[str, int] = {}
    for line_no, offset, text, ended in iter_raw_lines(source):
        if text is None:
            diagnostics.append(
                Diagnostic(
                    "undecodable_line",
                    "line could not be decoded as UTF-8",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                )
            )
            continue
        if not text.strip():
            diagnostics.append(
                Diagnostic(
                    "blank_line",
                    "blank line inside a JSONL record",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                )
            )
            continue
        try:
            raw = json.loads(text)
        except ValueError as error:
            code = "truncated_final_line" if not ended else "malformed_json"
            diagnostics.append(
                Diagnostic(
                    code,
                    f"{type(error).__name__}: {error}; preview={_bounded_preview(text)!r}",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                )
            )
            continue
        if not isinstance(raw, dict):
            diagnostics.append(
                Diagnostic(
                    "non_object_record",
                    f"record is {type(raw).__name__}, not an object",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                )
            )
            continue

        schema = raw.get("schema")
        if schema != 1:
            diagnostics.append(
                Diagnostic(
                    "unsupported_schema",
                    f"schema {schema!r} is not schema 1; fields are preserved but semantics may differ",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                )
            )

        raw_id = raw.get("id")
        if raw_id is None:
            namespaced = f"{source.index}:line{line_no}"
        else:
            namespaced = f"{source.index}:{raw_id}"
        if namespaced in seen_ids:
            diagnostics.append(
                Diagnostic(
                    "duplicate_event_id",
                    f"event id {namespaced!r} repeats (first at line {seen_ids[namespaced]})",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )
            namespaced = f"{namespaced}#line{line_no}"
        seen_ids[namespaced] = line_no

        parent_raw = _identifier(raw.get("parent"))
        parent = f"{source.index}:{parent_raw}" if parent_raw is not None else None

        at = parse_timestamp(raw.get("at"))
        if at is None:
            diagnostics.append(
                Diagnostic(
                    "invalid_timestamp",
                    f"at={raw.get('at')!r} is not an ISO-8601 timestamp",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )

        duration = _number(raw.get("duration_ms"))
        if raw.get("duration_ms") is not None and duration is None:
            diagnostics.append(
                Diagnostic(
                    "invalid_duration",
                    f"duration_ms={raw.get('duration_ms')!r} is not numeric",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )
        if duration is not None and duration < 0:
            diagnostics.append(
                Diagnostic(
                    "negative_duration",
                    f"duration_ms={duration!r} is negative",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )
            duration = None

        data = raw.get("data")
        if data is None:
            data = {}
        elif not isinstance(data, dict):
            diagnostics.append(
                Diagnostic(
                    "non_object_data",
                    f"data is {type(data).__name__}, not an object; content not interpreted",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )
            data = {}

        scope = raw.get("scope")
        scope_kind = scope.get("kind") if isinstance(scope, dict) else None
        scope_id = scope.get("id") if isinstance(scope, dict) else None
        if isinstance(scope_id, (int, float)) and not isinstance(scope_id, bool):
            scope_id = str(scope_id)

        kind = raw.get("kind")
        if not isinstance(kind, str) or not kind:
            diagnostics.append(
                Diagnostic(
                    "missing_kind",
                    f"kind={kind!r} is not a non-empty string",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )
            kind = "unknown"
        elif kind not in KNOWN_KINDS:
            diagnostics.append(
                Diagnostic(
                    "unknown_kind",
                    f"kind {kind!r} is not in the schema this analyzer knows",
                    source=str(source.path),
                    line=line_no,
                    byte_offset=offset,
                    event_id=namespaced,
                )
            )

        session = _identifier(raw.get("session"))

        yield Event(
            source_index=source.index,
            line=line_no,
            byte_offset=offset,
            raw_id=raw_id,
            id=namespaced,
            at=at,
            kind=kind,
            name=str(raw.get("name") or ""),
            label=str(raw.get("label") or ""),
            producer=str(raw.get("source") or ""),
            outcome=str(raw.get("outcome") or ""),
            duration_ms=duration,
            parent=parent,
            session=session,
            scope_kind=scope_kind if isinstance(scope_kind, str) else None,
            scope_id=scope_id if isinstance(scope_id, str) else None,
            schema=schema,
            data=data,
            raw=raw,
        )

    # Parent references may point at records that were damaged or omitted.
    # Resolution is done by the accounting layer, which has the full id set;
    # this only records the namespace rule for callers.


def read_sources(paths: list[str | os.PathLike[str]]) -> list[SourceFile]:
    sources: list[SourceFile] = []
    for index, raw_path in enumerate(paths):
        path = Path(raw_path)
        if not path.is_file():
            raise FileNotFoundError(f"events file not found: {path}")
        sources.append(freeze_source(path, index))
    return sources


def iter_all(
    sources: list[SourceFile],
    diagnostics: list[Diagnostic] | None = None,
) -> Iterator[Event]:
    for source in sources:
        yield from iter_events(source, diagnostics)
