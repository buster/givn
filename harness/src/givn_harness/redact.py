"""Best-effort redaction for reports that may be shared.

Event identifiers and structural codes are retained for correlation. Free-text
fields that commonly carry prompts, commands, errors, questions or agent names
are replaced with a length marker. This is not a substitute for reviewing a
report before publishing it.
"""

from __future__ import annotations

import copy
from typing import Any

from givn_harness.accounting import Analysis

REDACT_KEYS = {
    "text",
    "label",
    "error",
    "stdout",
    "stderr",
    "normalized",
    "answer",
    "identity",
    "question",
    "selected",
    "options",
}


def _redact_string(value: str) -> str:
    return f"<redacted:{len(value)} chars>"


def _walk(value: Any, key: str | None = None) -> Any:
    if isinstance(value, dict):
        if key == "declared_metadata":
            return {item_key: "<redacted>" for item_key in value}
        return {
            item_key: _walk(item_value, item_key)
            for item_key, item_value in value.items()
        }
    if isinstance(value, list):
        return [_walk(item, key) for item in value]
    if isinstance(value, str) and key in REDACT_KEYS:
        return _redact_string(value)
    return value


def redact_analysis(analysis: Analysis) -> Analysis:
    clone = copy.deepcopy(analysis)
    clone.data = _walk(clone.data)
    clone.data["redacted"] = True
    for finding in clone.findings:
        # Derived prose can repeat raw labels; keep the hypothesis but mark the
        # observation as redacted rather than pretending it is raw evidence.
        finding.observation = _redact_string(finding.observation)
    return clone
