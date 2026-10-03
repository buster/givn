# Review: rewrite-history

## Fabrication Audit (do this first)

**Step definition scan** — reuse of the shared `the user runs` step and two
new history-containment assertions with real `git log` checks; no empty
bodies.

**Commit cross-check**:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Plan | `7946838b` | Planning artifacts |
| history rewrite | `60814310` | `src/cli/rewrite_history.rs`, `src/main.rs`, arc42 5/8, steps |

**Promised-architecture cross-check** — `cli::rewrite_history`, the
`rewrite-history` subcommand, the use-case document, and the delta spec all
exist.

**Strict-mode proof** — `.fail_on_skipped()` (existing); the delta failed
before the command existed and is 5/5 after.

**Downgraded e2e scenario scan** — no `@e2e` tag added or removed; the
rewrite is driven as a real subprocess against a real `git-filter-repo` in
the delta.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| (delta drive is a real subprocess) | Yes | N/A (CLI) | No |

**E2e scope check** — the delta adds no `@e2e`; existing scopes unchanged.

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`; a single implementation exists.

**Local environment check** — no stack, no digital twins.

**E2e isolation check** — `./run-tests.sh` vs `./run-tests.sh --e2e`; the
archive receipt records both scopes. Pre-merge: regular 558 pass, e2e 88
pass.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | New use case `rewrite-history` carries the full contract | Yes |
| Capability ownership and typed relationships preserved | New capability `history-rewrite`; includes `rules-policy` | Yes |
| Confirmed Persona-relevant outcomes preserved | Explicit, reviewable cleanup; no push | Yes |
| Actors remain distinct from Persona biographies | Gherkin names the developer action | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A | Yes |

PERSONAS: developer--primary-user, givn-maintainer--owner

## Deferred mandatory checks

None.

**Design.md conformance check**: all decisions (command name, confirmation,
tool refusal, Fossil/versioned refusals, file preservation) match the
implementation.

**Interaction coverage verification** — three matrix entries map to three
delta scenarios driving the real subprocess; no inventory gap.

## Visual Review

N/A — no visual interface.

VISUAL-REVIEW: PASS

## Retirements

N/A.

## Arc42 implementation conformance

| Chapter | `arc42.md` | Implementation | Match? |
|---|---|---|---|
| 5 building blocks | Yes | New `cli::rewrite_history` row | Yes |
| 8 crosscutting | Yes | 8.7 names the remediation | Yes |
| 12 glossary | No new term | "History rewrite" present from the handoff | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt yet; the delta is 5/5 including the real rewrite.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| Command paths | 0 uncovered | Delta covers every branch: policy, backend, dirty, confirmation, purge, verify | None required |

## Scenario execution evidence

Unproven candidate — no receipt yet; the archive verifies the merged
candidate.

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| Piped input without the confirmation flag refuses | The versioned policy is refused | `variant` |
| Piped input without the confirmation flag refuses | The Fossil backend is refused by name | `variant` |
| The versioned policy is refused | The Fossil backend is refused by name | `variant` |
| Id with spaces is rejected | Piped input without the confirmation flag refuses | `variant` |
| Id with spaces is rejected | The versioned policy is refused | `variant` |
| Id with spaces is rejected | The Fossil backend is refused by name | `variant` |
| Id with non-ASCII letters is rejected | Piped input without the confirmation flag refuses | `variant` |
| Id with non-ASCII letters is rejected | The versioned policy is refused | `variant` |
| Id with non-ASCII letters is rejected | The Fossil backend is refused by name | `variant` |
| Id with a dot is rejected | Piped input without the confirmation flag refuses | `variant` |
| Id with a dot is rejected | The versioned policy is refused | `variant` |
| Id with a dot is rejected | The Fossil backend is refused by name | `variant` |
| Id ending with a hyphen is rejected | Piped input without the confirmation flag refuses | `variant` |
| Id ending with a hyphen is rejected | The versioned policy is refused | `variant` |
| Id ending with a hyphen is rejected | The Fossil backend is refused by name | `variant` |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | Piped input without the confirmation flag refuses | `variant` |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | The versioned policy is refused | `variant` |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | The Fossil backend is refused by name | `variant` |
| A malformed permanent feature reports its path and fails the audit | Piped input without the confirmation flag refuses | `variant` |
| A malformed permanent feature reports its path and fails the audit | The versioned policy is refused | `variant` |
| A malformed permanent feature reports its path and fails the audit | The Fossil backend is refused by name | `variant` |
| Routing requires exactly one input | Piped input without the confirmation flag refuses | `variant` |
| Routing requires exactly one input | The versioned policy is refused | `variant` |
| Routing requires exactly one input | The Fossil backend is refused by name | `variant` |
| givn addons enable errors when givn/ is not initialised | Piped input without the confirmation flag refuses | `variant` |
| givn addons enable errors when givn/ is not initialised | The versioned policy is refused | `variant` |
| givn addons enable errors when givn/ is not initialised | The Fossil backend is refused by name | `variant` |
| givn addons disable errors when givn/ is not initialised | Piped input without the confirmation flag refuses | `variant` |
| givn addons disable errors when givn/ is not initialised | The versioned policy is refused | `variant` |
| givn addons disable errors when givn/ is not initialised | The Fossil backend is refused by name | `variant` |
| Detail mode — unknown change errors | Piped input without the confirmation flag refuses | `variant` |
| Detail mode — unknown change errors | The versioned policy is refused | `variant` |
| Detail mode — unknown change errors | The Fossil backend is refused by name | `variant` |
| Uninitialized project errors | Piped input without the confirmation flag refuses | `variant` |
| Uninitialized project errors | The versioned policy is refused | `variant` |
| Uninitialized project errors | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by status | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by status | The versioned policy is refused | `variant` |
| Path traversal id is rejected by status | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by archive | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by archive | The versioned policy is refused | `variant` |
| Path traversal id is rejected by archive | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by check | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by check | The versioned policy is refused | `variant` |
| Path traversal id is rejected by check | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by lint | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by lint | The versioned policy is refused | `variant` |
| Path traversal id is rejected by lint | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by graph | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by graph | The versioned policy is refused | `variant` |
| Path traversal id is rejected by graph | The Fossil backend is refused by name | `variant` |
| Path traversal id is rejected by instructions | Piped input without the confirmation flag refuses | `variant` |
| Path traversal id is rejected by instructions | The versioned policy is refused | `variant` |
| Path traversal id is rejected by instructions | The Fossil backend is refused by name | `variant` |
| Missing impact decision refuses the review | Piped input without the confirmation flag refuses | `variant` |
| Missing impact decision refuses the review | The versioned policy is refused | `variant` |
| Missing impact decision refuses the review | The Fossil backend is refused by name | `variant` |
| Integrity hook fails on Step Definitions section that is not a table | Piped input without the confirmation flag refuses | `variant` |
| Integrity hook fails on Step Definitions section that is not a table | The versioned policy is refused | `variant` |
| Integrity hook fails on Step Definitions section that is not a table | The Fossil backend is refused by name | `variant` |
| Archive blocked when delta spec has non-removed @wip scenario | Piped input without the confirmation flag refuses | `variant` |
| Archive blocked when delta spec has non-removed @wip scenario | The versioned policy is refused | `variant` |
| Archive blocked when delta spec has non-removed @wip scenario | The Fossil backend is refused by name | `variant` |
| Check tasks blocked when delta spec has @wip | Piped input without the confirmation flag refuses | `variant` |
| Check tasks blocked when delta spec has @wip | The versioned policy is refused | `variant` |
| Check tasks blocked when delta spec has @wip | The Fossil backend is refused by name | `variant` |
| Lost E2E evidence blocks migration coverage | Piped input without the confirmation flag refuses | `variant` |
| Lost E2E evidence blocks migration coverage | The versioned policy is refused | `variant` |
| Lost E2E evidence blocks migration coverage | The Fossil backend is refused by name | `variant` |
| Lower interaction coverage blocks migration coverage | Piped input without the confirmation flag refuses | `variant` |
| Lower interaction coverage blocks migration coverage | The versioned policy is refused | `variant` |
| Lower interaction coverage blocks migration coverage | The Fossil backend is refused by name | `variant` |
| Lower line coverage blocks migration coverage | Piped input without the confirmation flag refuses | `variant` |
| Lower line coverage blocks migration coverage | The versioned policy is refused | `variant` |
| Lower line coverage blocks migration coverage | The Fossil backend is refused by name | `variant` |
| Lower branch coverage blocks migration coverage | Piped input without the confirmation flag refuses | `variant` |
| Lower branch coverage blocks migration coverage | The versioned policy is refused | `variant` |
| Lower branch coverage blocks migration coverage | The Fossil backend is refused by name | `variant` |
| Review blocks with a clear message when review.md is missing | Piped input without the confirmation flag refuses | `variant` |
| Review blocks with a clear message when review.md is missing | The versioned policy is refused | `variant` |
| Review blocks with a clear message when review.md is missing | The Fossil backend is refused by name | `variant` |
| A visual change without a visual verdict fails review | Piped input without the confirmation flag refuses | `variant` |
| A visual change without a visual verdict fails review | The versioned policy is refused | `variant` |
| A visual change without a visual verdict fails review | The Fossil backend is refused by name | `variant` |
| An unverified visual verdict cannot pass review | Piped input without the confirmation flag refuses | `variant` |
| An unverified visual verdict cannot pass review | The versioned policy is refused | `variant` |
| An unverified visual verdict cannot pass review | The Fossil backend is refused by name | `variant` |
| A web contract without viewports fails review | Piped input without the confirmation flag refuses | `variant` |
| A web contract without viewports fails review | The versioned policy is refused | `variant` |
| A web contract without viewports fails review | The Fossil backend is refused by name | `variant` |
| An empty evidence file fails review | Piped input without the confirmation flag refuses | `variant` |
| An empty evidence file fails review | The versioned policy is refused | `variant` |
| An empty evidence file fails review | The Fossil backend is refused by name | `variant` |
| A web contract with missing state evidence fails review | Piped input without the confirmation flag refuses | `variant` |
| A web contract with missing state evidence fails review | The versioned policy is refused | `variant` |
| A web contract with missing state evidence fails review | The Fossil backend is refused by name | `variant` |
| A visual change without a persona disposition fails review | Piped input without the confirmation flag refuses | `variant` |
| A visual change without a persona disposition fails review | The versioned policy is refused | `variant` |
| A visual change without a persona disposition fails review | The Fossil backend is refused by name | `variant` |
| A visual interface with an empty persona disposition fails review | Piped input without the confirmation flag refuses | `variant` |
| A visual interface with an empty persona disposition fails review | The versioned policy is refused | `variant` |
| A visual interface with an empty persona disposition fails review | The Fossil backend is refused by name | `variant` |
| An empty state screenshot fails review | Piped input without the confirmation flag refuses | `variant` |
| An empty state screenshot fails review | The versioned policy is refused | `variant` |
| An empty state screenshot fails review | The Fossil backend is refused by name | `variant` |
| A deferral without an operator decision fails review | Piped input without the confirmation flag refuses | `variant` |
| A deferral without an operator decision fails review | The versioned policy is refused | `variant` |
| A deferral without an operator decision fails review | The Fossil backend is refused by name | `variant` |
| A visual change with an unknown persona disposition fails review | Piped input without the confirmation flag refuses | `variant` |
| A visual change with an unknown persona disposition fails review | The versioned policy is refused | `variant` |
| A visual change with an unknown persona disposition fails review | The Fossil backend is refused by name | `variant` |
| A visual change with a reasoned none fails review | Piped input without the confirmation flag refuses | `variant` |
| A visual change with a reasoned none fails review | The versioned policy is refused | `variant` |
| A visual change with a reasoned none fails review | The Fossil backend is refused by name | `variant` |

## Split-or-keep

None — no long-scenario finding.

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN.
- [x] Scenario execution evidence recorded.
- [x] Coverage gaps classified.
- [x] No `@wip` tags; no implementation detail in the delta.
- [x] Local run cleanly; no digital twins.
- [x] e2e command identified; no parallel implementation.
- [x] e2e isolation holds.
- [x] Implementation matches design.
- [x] Interaction coverage verified.
- [x] Internal Primitives states none.
- [x] arc42 slugs map.
- [x] No invented exceptions.

README-IMPACT: none

**REVIEW: PASS**
