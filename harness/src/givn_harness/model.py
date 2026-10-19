"""Pinned model identity and a small guard record.

The harness allows exactly the direct DeepSeek Flash model with the maximum
reasoning variant. It refuses another provider, model or fallback. This module
validates the harness command; it is not a provider-side hard ceiling.
"""

from __future__ import annotations

from typing import Any

DEEPSEEK_FLASH_MAX = "deepseek/deepseek-flash#max"
DEEPSEEK_FLASH = "deepseek/deepseek-flash"
ALLOWED_MODELS = {DEEPSEEK_FLASH_MAX}


def require_model(model: str | None) -> str:
    selected = model or DEEPSEEK_FLASH_MAX
    if selected not in ALLOWED_MODELS:
        raise ValueError(
            f"only {DEEPSEEK_FLASH_MAX!r} is allowed; got {selected!r}. "
            "No fallback, other provider or weaker variant is permitted."
        )
    return selected


def guard_record(model: str, driver: str) -> dict[str, Any]:
    return {
        "allowed": sorted(ALLOWED_MODELS),
        "selected": model,
        "driver": driver,
        "variant": "max" if model.endswith("#max") else "default",
        "enforcement": (
            "CLI validation and pinned execution command. Provider-side inventory "
            "and native outgoing-request validation are not claimed by this record."
        ),
    }
