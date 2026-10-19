"""Small shared value types.

Every measurement carries a unit, provenance and status. Outcome, workflow
completion and measurement validity stay separate; there is deliberately no
single ``overall_pass`` field for a trial.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from pathlib import Path
from typing import Any

# Measurement status vocabulary. ``observed`` means the record contains the
# value directly; ``inferred`` means a labelled derivation; ``partial`` means
# the value is known to be incomplete; ``unavailable`` means no usable value.
STATUS_OBSERVED = "observed"
STATUS_INFERRED = "inferred"
STATUS_PARTIAL = "partial"
STATUS_UNAVAILABLE = "unavailable"

VALID_STATUSES = (
    STATUS_OBSERVED,
    STATUS_INFERRED,
    STATUS_PARTIAL,
    STATUS_UNAVAILABLE,
)


@dataclass(frozen=True)
class SourceFile:
    """One immutable input record and the byte boundary that was read."""

    index: int
    path: Path
    sha256: str
    boundary_bytes: int
    total_bytes: int
    mtime_ns: int

    @property
    def current_bytes(self) -> int:
        """Size right now; the record may still be growing."""
        try:
            return self.path.stat().st_size
        except OSError:
            return self.total_bytes

    @property
    def appended(self) -> bool:
        return self.boundary_bytes < self.current_bytes

    @property
    def appended_bytes(self) -> int:
        return max(0, self.current_bytes - self.boundary_bytes)

    def to_dict(self) -> dict[str, Any]:
        return {
            "index": self.index,
            "path": str(self.path),
            "sha256": self.sha256,
            "boundary_bytes": self.boundary_bytes,
            "captured_total_bytes": self.total_bytes,
            "current_bytes": self.current_bytes,
            "mtime_ns": self.mtime_ns,
            "appended_after_capture": self.appended,
            "appended_bytes": self.appended_bytes,
        }


@dataclass
class Diagnostic:
    """A damaged, ambiguous or unsupported input observation."""

    code: str
    message: str
    source: str = ""
    line: int | None = None
    byte_offset: int | None = None
    event_id: str | None = None

    def to_dict(self) -> dict[str, Any]:
        data: dict[str, Any] = {"code": self.code, "message": self.message}
        for key in ("source", "line", "byte_offset", "event_id"):
            value = getattr(self, key)
            if value not in (None, ""):
                data[key] = value
        return data


@dataclass
class Measurement:
    """A single number with explicit provenance and validity."""

    id: str
    value: Any
    unit: str
    status: str = STATUS_OBSERVED
    provenance: str = ""
    evidence: list[str] = field(default_factory=list)
    note: str = ""

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "value": self.value,
            "unit": self.unit,
            "status": self.status,
            "provenance": self.provenance,
            "evidence": list(self.evidence),
            "note": self.note,
        }


@dataclass
class Finding:
    """An evidence-linked observation and an explicitly limited hypothesis."""

    id: str
    title: str
    observation: str
    hypothesis: str
    severity: str = "info"
    evidence: list[str] = field(default_factory=list)
    limitations: str = ""
    status: str = STATUS_OBSERVED

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "title": self.title,
            "observation": self.observation,
            "hypothesis": self.hypothesis,
            "severity": self.severity,
            "evidence": list(self.evidence),
            "limitations": self.limitations,
            "status": self.status,
        }


@dataclass
class Event:
    """A normalized event from one source record.

    ``id`` is namespaced by source file, so identical numeric ids in two
    exports never collide. ``parent`` is namespaced the same way when it
    resolves inside the same source record.
    """

    source_index: int
    line: int
    byte_offset: int
    raw_id: Any
    id: str
    at: datetime | None
    kind: str
    name: str
    label: str
    producer: str
    outcome: str
    duration_ms: float | None
    parent: str | None
    session: str | None
    scope_kind: str | None
    scope_id: str | None
    schema: Any
    data: dict[str, Any]
    raw: dict[str, Any]

    @property
    def interval_end(self) -> float | None:
        return self.at.timestamp() if self.at is not None else None

    @property
    def interval_start(self) -> float | None:
        if self.at is None or self.duration_ms is None:
            return None
        return self.at.timestamp() - (self.duration_ms / 1000.0)


@dataclass
class EventFilter:
    sessions: set[str] | None = None
    scopes: set[str] | None = None
    since: datetime | None = None
    until: datetime | None = None
    kinds: set[str] | None = None

    def accepts(self, event: Event) -> bool:
        if self.sessions is not None and (event.session or "") not in self.sessions:
            return False
        if self.scopes is not None:
            scope = f"{event.scope_kind}:{event.scope_id}" if event.scope_id else ""
            short = event.scope_id or ""
            if scope not in self.scopes and short not in self.scopes:
                return False
        if self.since is not None:
            if event.at is None or event.at < self.since:
                return False
        if self.until is not None:
            if event.at is None or event.at > self.until:
                return False
        if self.kinds is not None and event.kind not in self.kinds:
            return False
        return True
