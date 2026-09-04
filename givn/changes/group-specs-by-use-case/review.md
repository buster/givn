# Review: group-specs-by-use-case

## Fabrication Audit

**Step definition scan** — no step definitions added or stubbed; one live
hardcoded path updated (`adr_qualification_gate_steps.rs`), no behaviour
change.

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Migration (38 git-mv renames + 7 group.md + 1 step path fix) | `3c2512b` | yes (test step path) |
| Post-archive verification | (archive commit) | archive move |

**Promised-architecture cross-check** — layout is the deliverable; see
proposal.md's confirmed group table, verified in the migration commit's
rename list (38/38).

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| 7 group directories + group.md | yes | `givn/specs/*/group.md` |

**Strict-mode proof** — full `./run-tests.sh`: 400 scenarios (400 passed)
on the grouped layout.

**Downgraded e2e scenario scan** — N/A: no scenarios authored.

**Local environment check** — N/A.

**E2e isolation check** — N/A.

**Design.md conformance check** — matches: no code components, moves only.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Grouped layout, no delta content | yes | yes | yes | n/a |

## Retirements

N/A

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN

README-IMPACT: none
