"""Conservative campaign spending ledger.

A reservation is written before a paid request is dispatched. A crash leaves
the reservation in place; it is not refunded. Admission refuses a request when
charged plus reserved plus the new conservative estimate could exceed the cap.
Post-response totals alone are not treated as a hard invoice ceiling.
"""

from __future__ import annotations

import fcntl
import json
import math
import os
import tomllib
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any


class BudgetError(RuntimeError):
    """Base class for budget failures."""


class BudgetExceeded(BudgetError):
    """The next request cannot be admitted under the cap."""


class MissingPrice(BudgetError):
    """No dated price source is configured."""


@dataclass(frozen=True)
class PriceTable:
    currency: str
    source: str
    date: str
    input_per_million: float
    output_per_million: float
    cache_read_per_million: float = 0.0
    cache_write_per_million: float = 0.0
    safety_factor: float = 1.25

    def to_dict(self) -> dict[str, Any]:
        return {
            "currency": self.currency,
            "source": self.source,
            "date": self.date,
            "input_per_million": self.input_per_million,
            "output_per_million": self.output_per_million,
            "cache_read_per_million": self.cache_read_per_million,
            "cache_write_per_million": self.cache_write_per_million,
            "safety_factor": self.safety_factor,
        }


def load_prices(path: str | Path) -> PriceTable:
    source_path = Path(path)
    if not source_path.is_file():
        raise MissingPrice(f"price table not found: {source_path}")
    try:
        data = tomllib.loads(source_path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        raise MissingPrice(f"cannot parse price table: {error}") from error
    if data.get("schema") != 1:
        raise MissingPrice("price table schema must be 1")
    if data.get("placeholder") is True:
        raise MissingPrice(
            "price table is a placeholder; fill in verified rates from a dated source"
        )
    rates = data.get("per_million_tokens")
    if not isinstance(rates, dict):
        raise MissingPrice("price table needs [per_million_tokens]")
    try:
        return PriceTable(
            currency=str(data.get("currency") or "USD"),
            source=str(data["source"]),
            date=str(data["date"]),
            input_per_million=float(rates["input"]),
            output_per_million=float(rates["output"]),
            cache_read_per_million=float(rates.get("cache_read") or 0.0),
            cache_write_per_million=float(rates.get("cache_write") or 0.0),
            safety_factor=float(data.get("safety_factor") or 1.25),
        )
    except (KeyError, TypeError, ValueError) as error:
        raise MissingPrice(f"incomplete price table: {error}") from error


def estimate_prompt_tokens(text: str) -> int:
    """Conservative character-based estimate (at most ~3 chars per token)."""
    return max(1, math.ceil(len(text) / 3))


def estimate_request_usd(
    prompt_text: str,
    max_output_tokens: int,
    prices: PriceTable,
    *,
    cached_tokens: int = 0,
) -> float:
    prompt_tokens = estimate_prompt_tokens(prompt_text)
    uncached = max(0, prompt_tokens - cached_tokens)
    # Worst-case output plus uncached input, then an explicit safety margin.
    cost = (
        uncached * prices.input_per_million
        + max_output_tokens * prices.output_per_million
        + cached_tokens * prices.cache_read_per_million
    ) / 1_000_000.0
    return round(cost * prices.safety_factor, 8)


class SpendLedger:
    """Durable, file-locked ledger with reservations and committed charges."""

    def __init__(self, path: str | Path, cap_usd: float) -> None:
        self.path = Path(path)
        self.cap_usd = float(cap_usd)
        if self.cap_usd <= 0:
            raise BudgetError("budget cap must be positive")
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self._lock_path = self.path.with_suffix(self.path.suffix + ".lock")
        self._data = self._load()
        if "entries" not in self._data:
            self._data = {
                "schema": "givn-harness/spend-ledger/v1",
                "cap_usd": self.cap_usd,
                "created_at": _stamp(),
                "entries": {},
            }
            self._save()

    def _load(self) -> dict[str, Any]:
        if not self.path.is_file():
            return {}
        try:
            data = json.loads(self.path.read_text(encoding="utf-8"))
        except ValueError as error:
            raise BudgetError(f"ledger is corrupt: {error}") from error
        if not isinstance(data, dict):
            raise BudgetError("ledger is not an object")
        return data

    def _save(self) -> None:
        temporary = self.path.with_suffix(self.path.suffix + ".tmp")
        temporary.write_text(json.dumps(self._data, indent=2) + "\n", encoding="utf-8")
        os.replace(temporary, self.path)

    def _locked(self):
        return _FileLock(self._lock_path)

    def _refresh(self) -> None:
        self._data = self._load()

    def totals(self) -> dict[str, float]:
        entries = self._data.get("entries", {})
        committed = 0.0
        reserved = 0.0
        uncertain = 0.0
        for entry in entries.values():
            state = entry.get("state")
            amount = float(entry.get("amount") or 0.0)
            if state == "committed":
                committed += amount
            elif state == "reserved":
                reserved += amount
            elif state == "uncertain":
                uncertain += amount
        return {
            "cap_usd": self.cap_usd,
            "committed_usd": round(committed, 8),
            "reserved_usd": round(reserved, 8),
            "uncertain_usd": round(uncertain, 8),
            "exposure_usd": round(committed + reserved + uncertain, 8),
            "remaining_usd": round(self.cap_usd - committed - reserved - uncertain, 8),
        }

    def reserve(self, request_id: str, amount_usd: float, *, note: str = "") -> dict[str, Any]:
        amount = float(amount_usd)
        if amount < 0:
            raise BudgetError("reservation amount cannot be negative")
        with self._locked():
            self._refresh()
            entries = self._data.setdefault("entries", {})
            if request_id in entries:
                raise BudgetError(f"request id already exists: {request_id}")
            totals = self.totals()
            if totals["exposure_usd"] + amount > self.cap_usd + 1e-9:
                raise BudgetExceeded(
                    f"cannot admit {request_id}: exposure {totals['exposure_usd']:.6f} "
                    f"+ {amount:.6f} exceeds cap {self.cap_usd:.6f}"
                )
            entry = {
                "state": "reserved",
                "amount": round(amount, 8),
                "created_at": _stamp(),
                "note": note,
                "dispatched": False,
            }
            entries[request_id] = entry
            self._save()
            return entry

    def mark_dispatched(self, request_id: str) -> None:
        with self._locked():
            self._refresh()
            entry = self._entry(request_id)
            entry["dispatched"] = True
            entry["dispatched_at"] = _stamp()
            self._save()

    def commit(self, request_id: str, actual_usd: float | None, *, note: str = "") -> dict[str, Any]:
        with self._locked():
            self._refresh()
            entry = self._entry(request_id)
            if actual_usd is None:
                entry["state"] = "uncertain"
                entry["note"] = (str(entry.get("note") or "") + " " + note).strip()
                entry["updated_at"] = _stamp()
                self._save()
                return entry
            actual = float(actual_usd)
            if actual < 0:
                raise BudgetError("actual cost cannot be negative")
            reserved_amount = entry.get("amount")
            entry["state"] = "committed"
            entry["reserved_amount"] = reserved_amount
            entry["amount"] = round(actual, 8)
            entry["actual_usd"] = round(actual, 8)
            if isinstance(reserved_amount, (int, float)) and actual > reserved_amount:
                entry["overage_usd"] = round(actual - reserved_amount, 8)
                entry["admission_exceeded"] = True
            entry["note"] = (str(entry.get("note") or "") + " " + note).strip()
            entry["updated_at"] = _stamp()
            self._save()
            return entry

    def release_undispatched(self, request_id: str) -> None:
        with self._locked():
            self._refresh()
            entry = self._entry(request_id)
            if entry.get("dispatched"):
                raise BudgetError(
                    "a dispatched request cannot be refunded; keep it uncertain"
                )
            entries = self._data.get("entries", {})
            entries.pop(request_id, None)
            self._save()

    def _entry(self, request_id: str) -> dict[str, Any]:
        entry = self._data.get("entries", {}).get(request_id)
        if not isinstance(entry, dict):
            raise BudgetError(f"unknown request id: {request_id}")
        return entry

    def assert_reconciled(self) -> None:
        totals = self.totals()
        if totals["uncertain_usd"] > 0:
            raise BudgetError(
                f"{totals['uncertain_usd']:.6f} USD remains uncertain; reconcile before "
                "dispatching another paid request"
            )


class _FileLock:
    def __init__(self, path: Path) -> None:
        self.path = path
        self.handle = None

    def __enter__(self) -> "_FileLock":
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.handle = self.path.open("a+")
        fcntl.flock(self.handle.fileno(), fcntl.LOCK_EX)
        return self

    def __exit__(self, *_: object) -> None:
        if self.handle is not None:
            fcntl.flock(self.handle.fileno(), fcntl.LOCK_UN)
            self.handle.close()


def _stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
