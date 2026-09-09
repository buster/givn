# Design Review: fix-opencode-llm-attribution

## Scope

PASS. The change corrects model lookup at both agent boundaries: OpenCode reads
the required model from chat parameters, and Claude reads the latest assistant model from its
PreToolUse transcript. The shared Git hook retains its best-effort behavior and
legacy Givn trailers, while attributed messages expose only the managed
co-author line. Human commit behavior remains unchanged.

## Grilling Decisions

### Scope and observable contract

PASS. The feature asserts the shell-visible LLM identity, reasoning mode,
harness, and committer name. The scenario title describes shell attribution,
which matches the actual interaction rather than claiming that the scenario
creates a Git commit.

### Technology choice

PASS. Executing the generated JavaScript module with Node.js latest LTS and the
generated Claude hook with a real PreToolUse payload are the smallest real
boundary tests. Live provider sessions would add network dependencies without
improving coverage of the payload translation defects.

### Missing scenarios

PASS. Provider/model composition, transcript model lookup, reasoning mode, and
committer identity are covered. Missing-model and provider-absent fallbacks
remain covered by the archived attribution contract.

### Testability and RED evidence

PASS. Before the asset correction, the targeted scenario executed the generated
plugin and failed on the concrete assertion:

```text
1 scenario (1 failed)
4 steps (3 passed, 1 failed)
assertion `left == right` failed
left: None
right: Some("openai/gpt-5.6-luna")
exit 101
```

After the corrections, both targeted scenarios pass. The checked-in OpenCode
plugin is asserted byte-for-byte equal to the embedded asset, and the Claude
scenario executes the returned RTK-wrapped command through real Git.

### Risk

PASS. The primary risks are OpenCode payload drift and Claude transcript
format/model-switch drift. Both current payload boundaries are explicit in the
design, and missing metadata remains non-blocking.

### Use case, actors, and Personas

PASS. The capability is owned by the reusable `support` fragment; the actor
remains the Givn actor from the historical contract. No Persona is confirmed,
invented, or used as a Gherkin actor. The active support fragment registers
`llm-attribution`; fragment interaction inventories remain empty by contract.

### ADR qualification

PASS. No ADR is created. The structured verdict in `arc42.md` classifies the
choice as `NOT_QUALIFIED` and routes it to `design.md`: there is no new durable
architectural boundary, the decision is local implementation detail, and no
existing ADR governs it.

### Arc42 impact

PASS. All twelve chapters were assessed explicitly in `arc42.md`. The affected
chapters are 02, 03, 06, 10, 11, and 12; those chapters contain the payload
constraint, integration boundary, runtime sequence, quality scenario, drift
risk, and model-identity term. No ADR register update is required.

## E2E Fidelity

The two new interactions each have one matching `@e2e` scenario:

| Inventory entry | Scenario | Interface | Driver |
|---|---|---|---|
| Prepare a shell command from an OpenCode session with a selected model | OpenCode prepares attribution using its selected model | Generated OpenCode plugin module | Node.js latest LTS imports the generated module, dispatches `chat.message` with the selected model object, dispatches `shell.env` for the same session, and inspects the returned shell environment |
| Prepare a Bash commit command from a Claude session with a current model | Claude attributes a commit using its current model | Claude Code PreToolUse hook and Git CLI | The generated hook receives a JSON payload with `transcript_path`, rewrites the Bash command, and the test executes that returned command before inspecting the Git commit |

No external service, provider credential, database, browser, or digital twin is
required.

## Sign-off

- Scope covers model lookup at the OpenCode and Claude integration boundaries.
- The active capability owner and interaction inventory are canonical.
- The OpenCode RED failure was captured before implementation. The Claude path
  was verified against its pre-fix fallback behavior and then exercised after
  transcript extraction was implemented.
- GREEN scenarios exercise both generated integration boundaries.
- Source and checked-in generated assets are kept in parity.
- No ADR is required; `design.md` is the canonical technical owner.

DESIGN-REVIEW: PASS
