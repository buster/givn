# Review: retire-semantic-retrieval-vocabulary

## Fabrication Audit

**Step definition scan** — the new step body asserts real output content
(negative assertions on retrieval advertising). No stubs.

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Deltas + new step | (artifacts commit) | yes (assets already committed in 1.4's build; steps file) |
| Post-archive verification | (archive commit) | permanent spec merge |

**Promised-architecture cross-check** — none promised.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none promised | n/a | n/a |

**Strict-mode proof** — delta verify: 2 scenarios (2 passed).

**Downgraded e2e scenario scan** — the modified e2e scenario drives the real
CLI subprocess and asserts its stdout.

**Design.md conformance check** — matches.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Modify 2 scenarios, 1 new step | yes | yes | yes | n/a |

## Retirements

N/A

## Overlap dispositions

The three shape matches pair the modified `skill-rewrites` scenario with
fixture-driven black-box scenarios that share the same masked
assertion skeleton but assert different content. All pairs are
**boundary**: different preconditions (different instruction sources),
different actions, different asserted content.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Canonical E2E policy is emitted by specs instructions` | `Spec instructions use deterministic gates before retrieval exists` | boundary |
| `Project override replaces the canonical specs instruction` | `Spec instructions use deterministic gates before retrieval exists` | boundary |
| `Canonical policy classifies output-affecting flags as variants` | `Spec instructions use deterministic gates before retrieval exists` | boundary |

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN

README-IMPACT: none
