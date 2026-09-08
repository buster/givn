# Review: finalize-usecase-migration-release

## Fabrication Audit

- The migration scenarios invoke the real `givn check migration` subprocess in
  temporary initialized projects.
- Strict Cucumber execution remains enforced by `.fail_on_skipped()`.
- Full Givn verification passed: 41 features, 375 scenarios, 2056 steps.
- Givn E2E verification passed: 33 features, 70 scenarios, 406 steps, plus
  five installed-build release-proof scenarios.
- Watn full and E2E suites passed.
- Kiekste full and E2E suites passed with `KIEKSTE_TEST_JOBS=1`.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Capability ownership | `verify-change` and `inspect-change-state` deltas include every active capability. | yes |
| Migration behavior evidence | 378 entries before, 379 after; four intentional vocabulary replacements are explicit `RETIRE` ledger rows. | yes |
| E2E evidence | 72 before, 73 after; no lost E2E evidence after explicit retirements. | yes |
| Interaction coverage | 34 before, 46 after; no lower interaction coverage. | yes |
| Persona/Handoff | No Persona context applies; Kiekste Handoff remains pending and untouched. | yes |
| Historical archive | Watn and Kiekste archives remain unchanged by this Givn change. | yes |

## Source Coverage

The comparable non-E2E source interval is:

```text
before: 6773/13704 lines, 0/0 branches
after:  7159/14478 lines, 0/0 branches
```

The branch denominator is zero on both sides and is not materially measured.
The line-coverage ratio is higher after the canonical migration. The
historical release-proof subprocess was unavailable in the temporary worktree,
but the current 0.6.0 E2E suite passes all five release-proof scenarios.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement | EXTEND givn-workflow-enforcement | The migration coverage scenarios extend the existing verification capability. |

## Overlap Dispositions

All reported shape matches are distinct migration dimensions or existing
command contracts. They are disposed as boundaries.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A malformed permanent feature reports its path and fails the audit` | `Lost E2E evidence blocks migration coverage` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Lower branch coverage blocks migration coverage` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Lower interaction coverage blocks migration coverage` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `Lower line coverage blocks migration coverage` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `A missing permanent specifications directory is an empty corpus` | `Explicit retirement documents a changed behavior` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `A proposal matching an existing capability surface recommends EXTEND` | `Explicit retirement documents a changed behavior` | boundary |
| `A proposal with no matching surface recommends NEW` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `A proposal with no matching surface recommends NEW` | `Explicit retirement documents a changed behavior` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Lower branch coverage blocks migration coverage` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Lower line coverage blocks migration coverage` | boundary |
| `Archived scenarios are excluded from the audit` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Archived scenarios are excluded from the audit` | `Explicit retirement documents a changed behavior` | boundary |
| `A structural subset is reported as an overlap` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `A structural subset is reported as an overlap` | `Explicit retirement documents a changed behavior` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Lower branch coverage blocks migration coverage` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Lower line coverage blocks migration coverage` | boundary |
| `Detail mode — unknown change errors` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Detail mode — unknown change errors` | `Lower branch coverage blocks migration coverage` | boundary |
| `Detail mode — unknown change errors` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Detail mode — unknown change errors` | `Lower line coverage blocks migration coverage` | boundary |
| `Exact duplicates across features are reported together` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Exact duplicates across features are reported together` | `Explicit retirement documents a changed behavior` | boundary |
| `Explicit retirement documents a changed behavior` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Lost E2E evidence blocks migration coverage` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Lower branch coverage blocks migration coverage` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Lower interaction coverage blocks migration coverage` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Lower line coverage blocks migration coverage` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Lost E2E evidence blocks migration coverage` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Lower branch coverage blocks migration coverage` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Lower interaction coverage blocks migration coverage` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Lower line coverage blocks migration coverage` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Lower branch coverage blocks migration coverage` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Lower line coverage blocks migration coverage` | boundary |
| `Id ending with a hyphen is rejected` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Id ending with a hyphen is rejected` | `Lower branch coverage blocks migration coverage` | boundary |
| `Id ending with a hyphen is rejected` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Id ending with a hyphen is rejected` | `Lower line coverage blocks migration coverage` | boundary |
| `Id with a dot is rejected` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Id with a dot is rejected` | `Lower branch coverage blocks migration coverage` | boundary |
| `Id with a dot is rejected` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Id with a dot is rejected` | `Lower line coverage blocks migration coverage` | boundary |
| `Id with non-ASCII letters is rejected` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Id with non-ASCII letters is rejected` | `Lower branch coverage blocks migration coverage` | boundary |
| `Id with non-ASCII letters is rejected` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Id with non-ASCII letters is rejected` | `Lower line coverage blocks migration coverage` | boundary |
| `Id with spaces is rejected` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Id with spaces is rejected` | `Lower branch coverage blocks migration coverage` | boundary |
| `Id with spaces is rejected` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Id with spaces is rejected` | `Lower line coverage blocks migration coverage` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Lower branch coverage blocks migration coverage` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Lower line coverage blocks migration coverage` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Integrity hook passes when tasks.md commit hashes are present` | `Explicit retirement documents a changed behavior` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Integrity hook skips arc42 check when slug cannot be resolved` | `Explicit retirement documents a changed behavior` | boundary |
| `Lexical search reports an empty result for unknown terms` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Lexical search reports an empty result for unknown terms` | `Explicit retirement documents a changed behavior` | boundary |
| `Lost E2E evidence blocks migration coverage` | `Lower branch coverage blocks migration coverage` | boundary |
| `Lost E2E evidence blocks migration coverage` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Lost E2E evidence blocks migration coverage` | `Lower line coverage blocks migration coverage` | boundary |
| `Lower interaction coverage blocks migration coverage` | `Lower branch coverage blocks migration coverage` | boundary |
| `Lower interaction coverage blocks migration coverage` | `Lower line coverage blocks migration coverage` | boundary |
| `Lower line coverage blocks migration coverage` | `Lower branch coverage blocks migration coverage` | boundary |
| `Missing impact decision refuses the review` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Missing impact decision refuses the review` | `Lower branch coverage blocks migration coverage` | boundary |
| `Missing impact decision refuses the review` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Missing impact decision refuses the review` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by archive` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by archive` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by archive` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by archive` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by check` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by check` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by check` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by check` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by graph` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by graph` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by graph` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by graph` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by instructions` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by instructions` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by instructions` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by instructions` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by lint` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by lint` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by lint` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by lint` | `Lower line coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by status` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Path traversal id is rejected by status` | `Lower branch coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by status` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Path traversal id is rejected by status` | `Lower line coverage blocks migration coverage` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Lower branch coverage blocks migration coverage` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Lower line coverage blocks migration coverage` | boundary |
| `Routing requires exactly one input` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Routing requires exactly one input` | `Lower branch coverage blocks migration coverage` | boundary |
| `Routing requires exactly one input` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Routing requires exactly one input` | `Lower line coverage blocks migration coverage` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `Consumer runs migration coverage check and sees passed evidence` | boundary |
| `Same assertion shape without exact behavior is reported as a structural match` | `Explicit retirement documents a changed behavior` | boundary |
| `Uninitialized project errors` | `Lost E2E evidence blocks migration coverage` | boundary |
| `Uninitialized project errors` | `Lower branch coverage blocks migration coverage` | boundary |
| `Uninitialized project errors` | `Lower interaction coverage blocks migration coverage` | boundary |
| `Uninitialized project errors` | `Lower line coverage blocks migration coverage` | boundary |

README-IMPACT: none

REVIEW: PASS
