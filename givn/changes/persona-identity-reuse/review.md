# Review: persona-identity-reuse

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed; existing `@e2e` scenarios are untouched. |
| Empty/no-op step bodies | 0 found in both touched step files. |
| Checked tasks have commits touching production | Clean — setup `b3bb6293` plus eight scenario commits (`8572ef2d`, `b0dd1e43`, `f9cc68a2`, `891caa07` + fold `ea0e4eb4`, `46af6176`, `1d5f898f`, `256df102`, `80db1723`). |
| Promised components exist | Clean — the persona template, the criteria and protocol, the advisory lint scan, the review-time resolution, the glossary rows, and the ADR-028 amendment all exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario. |
| `verify.e2e_command` isolation | `./run-tests.sh` vs `./run-tests.sh --e2e`; counts 520 vs 88 — strictly fewer. |
| Implementation vs design.md | Matches, with one recorded refinement: the project- and topic-resolution guards were folded into the dangling scenario during implementation because a pure negative guard cannot fail RED; the fold is recorded in `design.md`'s spec coverage and committed as `ea0e4eb4`. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

Two scenarios are legitimate immediate-GREEN guards (the exempt-path reasoned
none and the preserved-behaviors scenario); both are recorded as guards in
`tasks.md`.

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| visual-review-gate: run the review check on a visual change (existing) | yes | `A visual change with complete evidence passes review` | Real `givn check review` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | yes | `A consumer synchronizes skills and every agent target receives the ideation facilitation skill` | Real `givn skills sync` subprocess |
| lint-command: run the static lint check | yes (sibling coverage) | `A duplicate title across two permanent spec files is a hard error` | Real `givn lint` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end and
the new scenarios are variants. No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Identity criteria and reuse protocol | 08 §8.22 | Yes | Identity Criteria, Reuse-First Protocol | ideation tasks | skill + command assets | Yes |
| Persona template schema | 08 §8.22 | Yes | Persona Template | template task | `assets/templates/persona.md` | Yes |
| Advisory persona scan | 06 §6.4, 05 `cli::lint` | Yes | Lint Advisory | lint tasks | `src/cli/lint.rs::check_persona_references` | Yes |
| Review-time ID resolution | 06 §6.6, 05 `integrity_hook` | Yes | Review-Time Validation | review-gate tasks | `src/integrity_hook.rs::check_persona_disposition` | Yes |
| R-46 mitigation | 11 table | Yes | Risks | (covered by lint/review tasks) | advisory scan + resolution | Yes |
| Glossary rows | 12 glossary | Yes | Glossary | ideation task | updated `Persona segment`, `Persona promotion` | Yes |
| ADR-028 fourth amendment | 09 register, ADR-028 file | Yes | (Architecture Impact) | (covered) | amendment sentences | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary's `Persona`, `Persona segment`, `Persona promotion`, and
`Persona disposition` rows match the guidance and the validation: the delta
scenarios say "persona segment", "authority boundary", "evaluation
capability", "own vocabulary", "topic stance", and "record entries only"; the
code resolves IDs against `givn/personas/`. No glossary term is reused
inconsistently and no undefined term appears in the spec. Clean.

## Visual review

The change declares no visual interface: it adds guidance text, a template,
and validation logic. Opposite-classification test: the generated guidance
and the template are plain Markdown read in a terminal; nothing renders HTML,
CSS, or JavaScript in a browser, so no browser driver applies. No `@e2e`
scenario in this delta requires a screenshot or transcript. The visual block
is N/A.

## Overlap dispositions

Every non-contradicting shape match involving this change. The pairs share
generic fixture steps (`a givn project with a change that has passed
design-review`, `givn check review`) but assert different behaviors, so each
sits on a behavior boundary and none is a duplicate.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A visual change with an unknown persona disposition fails review` | `A visual change with a reasoned none fails review` | boundary |
| `givn addons disable when already disabled is idempotent` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A visual change with an unknown persona disposition fails review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A visual change with a reasoned none fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A visual change with an unknown persona disposition fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A visual change with a reasoned none fails review` | boundary |
| `Id with spaces is rejected` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Id with spaces is rejected` | `A visual change with a reasoned none fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A visual change with a reasoned none fails review` | boundary |
| `Id with a dot is rejected` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Id with a dot is rejected` | `A visual change with a reasoned none fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A visual change with a reasoned none fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A visual change with a reasoned none fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A visual change with a reasoned none fails review` | boundary |
| `Routing requires exactly one input` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Routing requires exactly one input` | `A visual change with a reasoned none fails review` | boundary |
| `Detail mode — unknown change errors` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Detail mode — unknown change errors` | `A visual change with a reasoned none fails review` | boundary |
| `Uninitialized project errors` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Uninitialized project errors` | `A visual change with a reasoned none fails review` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Path traversal id is rejected by status` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by status` | `A visual change with a reasoned none fails review` | boundary |
| `Path traversal id is rejected by archive` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by archive` | `A visual change with a reasoned none fails review` | boundary |
| `Path traversal id is rejected by check` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by check` | `A visual change with a reasoned none fails review` | boundary |
| `Path traversal id is rejected by lint` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by lint` | `A visual change with a reasoned none fails review` | boundary |
| `Path traversal id is rejected by graph` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by graph` | `A visual change with a reasoned none fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A visual change with a reasoned none fails review` | boundary |
| `The check passes on a complete README` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Missing impact decision refuses the review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Missing impact decision refuses the review` | `A visual change with a reasoned none fails review` | boundary |
| `Updated impact decision passes the review` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A disabled addon needs no impact decision` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A recorded impact decision passes the review` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A visual change with a reasoned none fails review` | boundary |
| `Integrity hook passes when arc42 is disabled` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Integrity hook passes on edited arc42 chapter` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A visual change with a reasoned none fails review` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A visual change with a reasoned none fails review` | boundary |
| `Lost E2E evidence blocks migration coverage` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Lost E2E evidence blocks migration coverage` | `A visual change with a reasoned none fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `A visual change with a reasoned none fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A visual change with a reasoned none fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A visual change with a reasoned none fails review` | boundary |
| `A change seed proposed by an archived change is reported as stale` | `The change scope reports a dangling persona reference` | boundary |
| `An advisory seed finding is counted in the summary and still exits 0` | `The change scope reports a dangling persona reference` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Review passes when a shape match exists only between permanent scenarios` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A visual change with an unknown persona disposition fails review` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A visual change with a reasoned none fails review` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A removed delta referencing its target by title is allowed` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `The change scope reports a dangling persona reference` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `The change scope reports a dangling persona reference` | boundary |
| `The default scope excludes the archive tree` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `The change scope reports a dangling persona reference` | boundary |
| `Quoted strings are masked in the shape comparison` | `The change scope reports a dangling persona reference` | boundary |
| `Scenario outlines sharing a template produce a shape-match warning` | `The change scope reports a dangling persona reference` | boundary |
| `Docstrings are ignored in the shape comparison` | `The change scope reports a dangling persona reference` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A visual change with complete evidence passes review` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A visual change without a visual verdict fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A visual change without a visual verdict fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A change without a visual contract is exempt from the visual gate` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `An unverified visual verdict cannot pass review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `An unverified visual verdict cannot pass review` | `A visual change with a reasoned none fails review` | boundary |
| `A terminal change passes review with a transcript` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A change that declares no visual interface is exempt` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A web contract without viewports fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A web contract without viewports fails review` | `A visual change with a reasoned none fails review` | boundary |
| `An empty evidence file fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `An empty evidence file fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A web contract with missing state evidence fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A web contract with missing state evidence fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A web contract with complete state evidence passes review` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A visual change without a persona disposition fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A visual change without a persona disposition fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A visual change with a confirmed persona disposition passes review` | `A change without a visual interface passes review with a reasoned none` | boundary |
| `A visual interface with an empty persona disposition fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A visual interface with an empty persona disposition fails review` | `A visual change with a reasoned none fails review` | boundary |
| `An empty state screenshot fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `An empty state screenshot fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A deferral without an operator decision fails review` | `A visual change with an unknown persona disposition fails review` | boundary |
| `A deferral without an operator decision fails review` | `A visual change with a reasoned none fails review` | boundary |
| `A deferral with an operator decision passes review` | `A change without a visual interface passes review with a reasoned none` | boundary |

The deterministic scan additionally reports 324 assertion-subset warnings
between delta and permanent scenarios; all are structural overlaps on generic
shared assertions.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`A dangling persona reference is reported as an advisory` at 12 steps). No
split-or-keep decisions are required.

## Removed+added pairs

None. The delta adds 8 scenarios and modifies none; no scenario is removed.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- `check_persona_references` and its helpers: covered by the dangling and
  `--change` scenarios, including both resolution homes.
- `check_persona_disposition`'s resolution branch: covered by the
  unknown-persona and reasoned-none scenarios plus the unit test.
- The persona template: covered by the template scenario.
- The guidance edits: covered by the three ideation scenarios.

No dead code introduced; no missing coverage; no hard-to-test gap. The
advisory noise on givn's own title-style persona entries is an accepted,
recorded consequence (design.md; chapter 8.22); those entries are out of
scope to migrate.

## Use-case and Persona conformance

- Preserved behaviors verified: user confirmation, the review duty, the cap,
  advisory findings with arbitration, no auto-promotion, and the actor/
  persona distinction.
- Support-fragment `ideation-skill` keeps its fragment ownership.
- Confirmed Personas: `agent--technical-role` (guidance) and
  `givn-maintainer--owner` (rule corpus, glossary). No Persona was invented
  or promoted; Gherkin actors remain the maintainer, reviewer, and agent.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 520 passed, e2e
      88 passed; delta 8 passed).
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read; the new scenarios are regular variants.
- [x] Every E2E scenario keeps its real-interface primary assertion.
- [x] Local run command starts the full stack: `./run-tests.sh`; no external
      dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 520).
- [x] Implementation matches design.md; the guard fold is recorded.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
