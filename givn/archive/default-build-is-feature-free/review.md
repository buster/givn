# Review: default-build-is-feature-free

## Fabrication Audit

**Step definition scan** — the modified steps assert real `cargo
metadata` content (empty default list; four declared opt-in features).

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Steps + delta | (implementation commit) | yes (steps) |

**Promised-architecture cross-check** — none promised.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none promised | n/a | n/a |

**Strict-mode proof** — `cargo test --no-default-features` green
(includes this scenario).

**Downgraded e2e scenario scan** — N/A (metadata inspection, no e2e).

**Design.md conformance check** — matches.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Pin empty default set | yes | yes | yes | n/a |

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| release-0-3-0 | EXTEND release-0-3-0 | EXTEND release-0-3-0 | the scenario lives there |

## Retirements

N/A

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN

README-IMPACT: none
