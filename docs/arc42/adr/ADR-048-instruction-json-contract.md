---
status: accepted
date: 2026-08-17
decision-makers:
  - givn maintainers
consulted:
  - instruction-integrity scenarios and command assets
informed:
  - agents and machine consumers of instructions
---

# Keep the effective authoring policy and instruction metadata explicit

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - generated guidance and machine consumers depend on the instruction contract.
- [x] **Shapes quality attributes** - one effective policy reduces authoring drift and interface-fidelity failures.
- [x] **Structural** - the resolved `specs` instruction and JSON object form an agent integration boundary.
- [x] **Must be shared** - every command and skill must delegate to the same effective policy.

## Context and Problem Statement

Interaction rules were distributed across assets and the documented JSON metadata did not match the CLI output. Agents need complete resolved policy and machine-readable artifact metadata without parsing prose.

## Decision Drivers

- one policy source
- project override support
- stable JSON fields
- explicit retrieval-mode guidance

## Considered Options

- Keep distributed policy and parse human output.
- Make `specs` authoritative, keep derived guidance thin, and emit structured metadata.
- Add a runtime include system for every guidance fragment.

## Decision Outcome

Chosen option: **Use the resolved `specs` instruction as the normative policy and emit one JSON object from `givn instructions <artifact> --json`** with `id`, `generates`, `requires`, and `instruction`. The object uses the effective manifest and cascade; human output remains instruction text.

### Consequences

- Project instruction overrides are effective for both human and JSON output.
- Skills and commands are generated snapshots and may be stale until sync.
- Derived guidance must not restate a competing E2E policy.

### Confirmation

Instruction-integrity scenarios assert canonical policy clauses, override precedence, generated guidance parity, and one valid JSON document.

## Pros and Cons of the Options

### Effective policy plus JSON metadata

- Good, because agents have one authority and machines have explicit fields.
- Bad, because consumers must honor the effective override and field contract.

### Distributed policy and prose parsing

- Good, because each asset can be self-contained.
- Bad, because wording and metadata drift.

### Runtime include system

- Good, because shared content could be inserted mechanically.
- Bad, because it adds a renderer protocol and complicates project overrides.

## More Information

The canonical authoring policy is supplied by the resolved specs instruction;
the machine contract is described in [chapter 8.3b](../08-crosscutting-concepts.md#83b-machine-readable-instruction-contract).
