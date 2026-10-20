"""Shared helpers for deterministic harness tests."""

from __future__ import annotations

import json
from pathlib import Path


def write_events(path: Path, events: list[dict], *, trailing_newline: bool = True) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    body = "\n".join(json.dumps(event, ensure_ascii=False) for event in events)
    if trailing_newline:
        body += "\n"
    path.write_text(body, encoding="utf-8")
    return path


def event(
    raw_id: int,
    *,
    at: str,
    kind: str = "command",
    name: str = "command",
    label: str = "",
    duration_ms: float | int | None = None,
    outcome: str = "ok",
    source: str = "givn",
    session: str | None = None,
    parent: int | None = None,
    scope: dict | None = None,
    data: dict | None = None,
    schema: int = 1,
) -> dict:
    record = {
        "schema": schema,
        "id": raw_id,
        "at": at,
        "scope": scope or {"kind": "change", "id": "demo"},
        "source": source,
        "kind": kind,
        "name": name,
        "label": label or name,
        "outcome": outcome,
        "data": data or {},
    }
    if duration_ms is not None:
        record["duration_ms"] = duration_ms
    if session is not None:
        record["session"] = session
    if parent is not None:
        record["parent"] = parent
    return record
