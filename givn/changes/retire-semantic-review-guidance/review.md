# Review: retire-semantic-review-guidance

## Fabrication Audit

**Step definition scan** — the new step bodies perform real assertions
against the review instruction text and CLI subprocess output. No stub
bodies added.

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Delta + guidance deletion + new steps | `c4714fe` | yes (assets, steps) |
| Post-archive verification | (archive commit) | permanent spec merge |

**Promised-architecture cross-check** — no components promised; none built.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none promised | n/a | n/a |

**Strict-mode proof** — delta verify run: `GIVN_FEATURES=givn/changes/retire-semantic-review-guidance/specs ./run-tests.sh` → 4 scenarios (4 passed).

**Downgraded e2e scenario scan** — the one `@e2e` scenario asserts real CLI
subprocess output (`givn instructions review`), not a proxy.

**Local environment check** — N/A (no run surface).

**E2e isolation check** — N/A for this change (guidance-level e2e).

**Design.md conformance check** — matches design.md exactly.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Replace 2 semantic scenarios | yes | yes | yes | n/a |

## Retirements

N/A — no capability retired; scenarios within a capability were replaced.

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN
- README-IMPACT: none
