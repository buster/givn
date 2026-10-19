"""The models the harness runs.

Every agent runs direct DeepSeek V4 Flash. opencode gets the model's `max`
reasoning variant (`#max`); omp gets `--thinking max`; maki uses the same
provider/model. The judge is always opencode with `#max`, independent of the
agent under test. The harness refuses any other model so a run cannot switch to
another provider, a stronger variant, or a fallback.
"""

DEEPSEEK_FLASH_MAX = "deepseek/deepseek-flash#max"
DEEPSEEK_FLASH = "deepseek/deepseek-flash"

AGENTS = ("opencode", "omp", "maki")
AGENT_MODELS = {
    "opencode": DEEPSEEK_FLASH_MAX,
    "omp": DEEPSEEK_FLASH,
    "maki": DEEPSEEK_FLASH,
}


def require_model(model: str | None = None) -> str:
    """Return the judge model, or refuse a different selection."""
    selected = model or DEEPSEEK_FLASH_MAX
    if selected != DEEPSEEK_FLASH_MAX:
        raise ValueError(
            f"only the direct DeepSeek model {DEEPSEEK_FLASH_MAX!r} is allowed; "
            f"got {selected!r}"
        )
    return selected


def require_agent_model(agent: str, model: str | None = None) -> str:
    """Return the pinned model for one agent, or refuse a different selection."""
    if agent not in AGENT_MODELS:
        raise ValueError(f"unknown agent {agent!r}; choose one of {', '.join(AGENTS)}")
    selected = model or AGENT_MODELS[agent]
    if selected != AGENT_MODELS[agent]:
        raise ValueError(
            f"agent {agent!r} is pinned to the direct DeepSeek model "
            f"{AGENT_MODELS[agent]!r}; got {selected!r}"
        )
    return selected
