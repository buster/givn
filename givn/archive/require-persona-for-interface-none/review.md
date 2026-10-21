# Review: require-persona-for-interface-none

## Fabrication Audit (do this first)

**Step definition scan** — all step definition files checked for empty/no-op
bodies; no empty, bare `pass`/`return`, or pending step bodies were added.
`cargo test --locked --test features_runner` enforces undefined steps with
cucumber-rs `.fail_on_skipped()` and exits non-zero on skipped scenarios.

```
0 empty step bodies found across the visual-review-gate step files
```

**Commit cross-check**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| A change that declares no visual interface is exempt | `14a686fdf9dba61b57d93bd15e26c218a78dc1cd` | yes: `src/integrity_hook.rs`, fixture step, delta |
| A change that declares the interface none without a persona disposition fails review | `4f55484d6bdfeb19a1f02c3522af35a078548591` | regression test: delta scenario |
| A change that declares the interface none with a reasoned none fails review | `c5fc4e7983b0c8c1c4fede2c126ed43f60fd32d7` | regression test: delta scenario |
| Review guidance requires a persona for a declared interface none contract | `6c0e7bef6b00bf470ce5aa7ea8110a33c1300952` | guidance/templates/arc42 and fixture compatibility |

**Promised-architecture cross-check**

| Component | Exists? | Path |
|---|---|---|
| Contract-based Persona branch | yes | `src/integrity_hook.rs::check_visual_review` |
| Persona disposition resolution | yes | `src/integrity_hook.rs::check_persona_disposition` |
| Fixture step | yes | `tests/steps/visual_review_gate_steps.rs` |
| Review/design guidance and templates | yes | `assets/instructions/*`, `assets/templates/*` |
| Durable architecture documentation | yes | `docs/arc42/05`, `06`, `08`, `09`, `10`, `11`, `12`, `README` |

**Strict-mode proof** — the cucumber-rs runner calls `.fail_on_skipped()` and
exits 1 when `stats.failed > 0 || stats.skipped > 0`; the change-scoped run
reported four passed, none skipped.

**Downgraded e2e scenario scan**

| @e2e Scenario | Real-interface assertion present? | Driven via real interface? | Downgraded? |
|---|---|---|---|
| A visual change with complete evidence passes review | existing permanent CLI/check assertion | real `givn` binary | no |
| Review guidance requires the visual verdict and persona lenses | existing permanent instruction output assertion | real `givn instructions` | no |

No delta `@e2e` scenario was added; the behavior is a variant of the existing
review-check action.

**E2e scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| visual-review-gate | 2 permanent | run review; read guidance | no |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`; the only e2e implementation is the permanent
`tests/steps/visual_review_gate_e2e_steps.rs` file (plus shared steps). No
parallel implementation exists.

**Local environment check** — `cargo build --locked` then
`cargo run -- check review --change require-persona-for-interface-none`; the
CLI uses local files, no service, database, browser, container, or network.

**E2e isolation check**

| `verify.command` | `verify.e2e_command` | Identical? | Regular count | E2e count | Strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | no | 724 | e2e scope | yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `givn/specs/verify-change/usecase.md`; no new capability or interaction | yes |
| Capability ownership and typed relationships preserved | `EXTEND visual-review-gate`; `verify-change` remains owner | yes |
| Confirmed Persona-relevant outcomes preserved | developer, agent, givn maintainer; no invented Persona | yes |
| Actors remain distinct from Persona biographies | Gherkin uses developer/system actors; Persona remains review lens | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration; coverage is measured by archive | yes |

PERSONAS: developer--primary-user

## Deferred mandatory checks

None.

## Design.md conformance check

| Decision | design.md says | Actually built | Matches? |
|---|---|---|---|
| Contract presence is the Persona boundary | any declared contract, including `Interface: none`, calls `check_persona_disposition` | branch added before verdict/evidence | yes |
| No contract remains exempt | absent section returns early | unchanged | yes |
| `none` value fails | `none`, empty, and reasoned none fail | error paths and unit tests | yes |
| Guidance/template rule | review/design/proposal/design-review guidance and templates state contract-based rule | text updated; guidance scenario passes | yes |

## Interaction coverage verification

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| visual-review-gate: run review | yes | A visual change with complete evidence passes review | real CLI subprocess | real `givn` binary via `common::run_givn` | yes |
| visual-review-gate: read guidance | yes | Review guidance requires the visual verdict and persona lenses | real CLI subprocess | real `givn instructions` | yes |

## Visual Review

N/A. The design declares `Interface: none`; the change's Persona disposition is
recorded above (`developer--primary-user`). No rendered screen, terminal
transcript, or screenshot is introduced. The opposite classification does not
hold: this change emits no UI and adds no CLI output surface beyond an error
message already covered by the review-check tests.

VISUAL-REVIEW: NOT APPLICABLE — Interface: none; Persona recorded.

## Retirements

N/A.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Persona boundary follows declared contract | `docs/arc42/08-crosscutting-concepts.md`, `06-runtime-view.md` | recorded | boundary rule | scenario 1 | `check_visual_review` branch | yes |
| `Interface: none` skips only verdict/evidence | `docs/arc42/06-runtime-view.md`, `10-quality-requirements.md` | recorded | behavior table | scenarios 1–3 | unit and feature tests | yes |
| No-contract exemption remains | `docs/arc42/11-risks-and-technical-debt.md` R-59 | recorded | boundary table | scenario 1 | `no_visual_contract_remains_exempt` | yes |
| ADR-028 trigger widened, no new ADR | `docs/arc42/09-architecture-decisions.md` | `NOT_QUALIFIED` | Decisions D1 | scenario 4 | guidance/docs | yes |

- [x] Every affected chapter was opened and checked.
- [x] Every relevant fact is represented in design and tasks.
- [x] The implementation follows those facts.
- [x] Architecture changes discovered in the completed implementation are documented.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured once by archive against the committed candidate. The
change scope is four scenarios, all passing; the permanent run includes the
pre-existing corpus. The archive verification receipt will carry the measured
line and branch result.

**Coverage gap classification:** no uncovered production region was found in
static inspection; the new branch and error paths are covered by unit and
feature scenarios. No dead code was identified.

| Location | Classification | Action |
|---|---|---|
| none | — | — |

## Scenario execution evidence

`givn check tasks --change require-persona-for-interface-none` produced a
green change-scoped receipt at `063aef2aa4acca08bcbe385b0e08a0fc0208edee`:
4 scenarios, 0 failed, 0 skipped. The archive merged suite is the final
full-corpus authority.

## Overlap dispositions

The deterministic shape-match gate found the following pairs. Each is a
variant of the same review-gate action or a boundary against permanent
exemption behavior; all are classified explicitly.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A change that declares the interface none without a persona disposition fails review | A change that declares the interface none with a reasoned none fails review | variant |
| The readability report is unavailable when the experiment is disabled | A change that declares the interface none without a persona disposition fails review | variant |
| The readability report is unavailable when the experiment is disabled | A change that declares the interface none with a reasoned none fails review | variant |
| Id with spaces is rejected | A change that declares the interface none without a persona disposition fails review | variant |
| Id with spaces is rejected | A change that declares the interface none with a reasoned none fails review | variant |
| Id with non-ASCII letters is rejected | A change that declares the interface none without a persona disposition fails review | variant |
| Id with non-ASCII letters is rejected | A change that declares the interface none with a reasoned none fails review | variant |
| Id with a dot is rejected | A change that declares the interface none without a persona disposition fails review | variant |
| Id with a dot is rejected | A change that declares the interface none with a reasoned none fails review | variant |
| Id ending with a hyphen is rejected | A change that declares the interface none without a persona disposition fails review | variant |
| Id ending with a hyphen is rejected | A change that declares the interface none with a reasoned none fails review | variant |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | A change that declares the interface none without a persona disposition fails review | variant |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | A change that declares the interface none with a reasoned none fails review | variant |
| Canonical policy uses transcripts for terminal interfaces | Review guidance requires a persona for a declared interface none contract | variant |
| Design guidance keeps improvements inside the style | Review guidance requires a persona for a declared interface none contract | variant |
| Design guidance requires a driver for every Web UI matrix row | Review guidance requires a persona for a declared interface none contract | variant |
| A malformed permanent feature reports its path and fails the audit | A change that declares the interface none without a persona disposition fails review | variant |
| A malformed permanent feature reports its path and fails the audit | A change that declares the interface none with a reasoned none fails review | variant |
| Routing requires exactly one input | A change that declares the interface none without a persona disposition fails review | variant |
| Routing requires exactly one input | A change that declares the interface none with a reasoned none fails review | variant |
| givn addons disable when already disabled is idempotent | A change that declares no visual interface is exempt | variant |
| givn addons enable errors when givn/ is not initialised | A change that declares the interface none without a persona disposition fails review | variant |
| givn addons enable errors when givn/ is not initialised | A change that declares the interface none with a reasoned none fails review | variant |
| givn addons disable errors when givn/ is not initialised | A change that declares the interface none without a persona disposition fails review | variant |
| givn addons disable errors when givn/ is not initialised | A change that declares the interface none with a reasoned none fails review | variant |
| givn init lists default-off addons and how to enable them | Review guidance requires a persona for a declared interface none contract | variant |
| Detail mode — unknown change errors | A change that declares the interface none without a persona disposition fails review | variant |
| Detail mode — unknown change errors | A change that declares the interface none with a reasoned none fails review | variant |
| Uninitialized project errors | A change that declares the interface none without a persona disposition fails review | variant |
| Uninitialized project errors | A change that declares the interface none with a reasoned none fails review | variant |
| Valid kebab-case change id is accepted by every command that accepts --change | A change that declares no visual interface is exempt | variant |
| Path traversal id is rejected by status | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by status | A change that declares the interface none with a reasoned none fails review | variant |
| Path traversal id is rejected by archive | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by archive | A change that declares the interface none with a reasoned none fails review | variant |
| Path traversal id is rejected by check | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by check | A change that declares the interface none with a reasoned none fails review | variant |
| Path traversal id is rejected by lint | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by lint | A change that declares the interface none with a reasoned none fails review | variant |
| Path traversal id is rejected by graph | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by graph | A change that declares the interface none with a reasoned none fails review | variant |
| Path traversal id is rejected by instructions | A change that declares the interface none without a persona disposition fails review | variant |
| Path traversal id is rejected by instructions | A change that declares the interface none with a reasoned none fails review | variant |
| The check passes on a complete README | A change that declares no visual interface is exempt | variant |
| Missing impact decision refuses the review | A change that declares the interface none without a persona disposition fails review | variant |
| Missing impact decision refuses the review | A change that declares the interface none with a reasoned none fails review | variant |
| Updated impact decision passes the review | A change that declares no visual interface is exempt | variant |
| A disabled addon needs no impact decision | A change that declares no visual interface is exempt | variant |
| A recorded impact decision passes the review | A change that declares no visual interface is exempt | variant |
| Integrity hook fails on Step Definitions section that is not a table | A change that declares the interface none without a persona disposition fails review | variant |
| Integrity hook fails on Step Definitions section that is not a table | A change that declares the interface none with a reasoned none fails review | variant |
| Integrity hook passes when arc42 is disabled | A change that declares no visual interface is exempt | variant |
| Integrity hook passes on edited arc42 chapter | A change that declares no visual interface is exempt | variant |
| Archive blocked when delta spec has non-removed @wip scenario | A change that declares the interface none without a persona disposition fails review | variant |
| Archive blocked when delta spec has non-removed @wip scenario | A change that declares the interface none with a reasoned none fails review | variant |
| Check tasks blocked when delta spec has @wip | A change that declares the interface none without a persona disposition fails review | variant |
| Check tasks blocked when delta spec has @wip | A change that declares the interface none with a reasoned none fails review | variant |
| Lost E2E evidence blocks migration coverage | A change that declares the interface none without a persona disposition fails review | variant |
| Lost E2E evidence blocks migration coverage | A change that declares the interface none with a reasoned none fails review | variant |
| Lower interaction coverage blocks migration coverage | A change that declares the interface none without a persona disposition fails review | variant |
| Lower interaction coverage blocks migration coverage | A change that declares the interface none with a reasoned none fails review | variant |
| Lower line coverage blocks migration coverage | A change that declares the interface none without a persona disposition fails review | variant |
| Lower line coverage blocks migration coverage | A change that declares the interface none with a reasoned none fails review | variant |
| Lower branch coverage blocks migration coverage | A change that declares the interface none without a persona disposition fails review | variant |
| Lower branch coverage blocks migration coverage | A change that declares the interface none with a reasoned none fails review | variant |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A change that declares no visual interface is exempt | variant |
| Review passes when a long scenario has a split-or-keep entry | A change that declares no visual interface is exempt | variant |
| Review passes when a shape match exists only between permanent scenarios | A change that declares no visual interface is exempt | variant |
| Review blocks with a clear message when review.md is missing | A change that declares the interface none without a persona disposition fails review | variant |
| Review blocks with a clear message when review.md is missing | A change that declares the interface none with a reasoned none fails review | variant |
| A modified delta matching its merge target in the same capability is allowed | A change that declares no visual interface is exempt | variant |
| A removed delta referencing its target by title is allowed | A change that declares no visual interface is exempt | variant |
| The default scope excludes the archive tree | A change that declares no visual interface is exempt | variant |
| givn check with known artifact proceeds to run hooks | A change that declares no visual interface is exempt | variant |
| A visual change with complete evidence passes review | A change that declares no visual interface is exempt | variant |
| A visual change without a visual verdict fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A visual change without a visual verdict fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A change without a visual contract is exempt from the visual gate | A change that declares no visual interface is exempt | variant |
| An unverified visual verdict cannot pass review | A change that declares the interface none without a persona disposition fails review | variant |
| An unverified visual verdict cannot pass review | A change that declares the interface none with a reasoned none fails review | variant |
| A terminal change passes review with a transcript | A change that declares no visual interface is exempt | variant |
| Review guidance records an unverified verdict without vision | Review guidance requires a persona for a declared interface none contract | variant |
| A web contract without viewports fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A web contract without viewports fails review | A change that declares the interface none with a reasoned none fails review | variant |
| An empty evidence file fails review | A change that declares the interface none without a persona disposition fails review | variant |
| An empty evidence file fails review | A change that declares the interface none with a reasoned none fails review | variant |
| Review guidance names the state screenshots the vision review saw | Review guidance requires a persona for a declared interface none contract | variant |
| A web contract with missing state evidence fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A web contract with missing state evidence fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A web contract with complete state evidence passes review | A change that declares no visual interface is exempt | variant |
| A visual change without a persona disposition fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A visual change without a persona disposition fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A visual change with a confirmed persona disposition passes review | A change that declares no visual interface is exempt | variant |
| A visual interface with an empty persona disposition fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A visual interface with an empty persona disposition fails review | A change that declares the interface none with a reasoned none fails review | variant |
| An empty state screenshot fails review | A change that declares the interface none without a persona disposition fails review | variant |
| An empty state screenshot fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A deferral without an operator decision fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A deferral without an operator decision fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A deferral with an operator decision passes review | A change that declares no visual interface is exempt | variant |
| A visual change with an unknown persona disposition fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A visual change with an unknown persona disposition fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A visual change with a reasoned none fails review | A change that declares the interface none without a persona disposition fails review | variant |
| A visual change with a reasoned none fails review | A change that declares the interface none with a reasoned none fails review | variant |
| A change without a visual interface passes review with a reasoned none | A change that declares no visual interface is exempt | variant |
| A machine-only CLI records not applicable and passes review | A change that declares no visual interface is exempt | variant |

## Split-or-keep

No scenario exceeds the lint threshold; no split decision is required.

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN.
- [x] Scenario execution evidence recorded per scope in the change verification receipt.
- [x] Every coverage gap is classified and resolved.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins, cleanly.
- [x] The exact E2E files were identified and no parallel implementation exists.
- [x] `verify.e2e_command` is distinct and the scope counts are isolated.
- [x] Implementation matches design.md; no silent deviation.
- [x] Interaction coverage is complete.
- [x] No internal primitive is omitted from design.
- [x] Arc42 slugs resolve to durable chapter files.
- [x] No finding is excused outside the three coverage buckets.

**REVIEW: PASS**

OUTLINE: AUTHOR developer--primary-user
OUTLINE: REVIEWER agent--technical-role
OUTLINE: PASS
OUTLINE: PASS module:src/integrity_hook.rs | Integrity review gate: check revision identifiers, promised files, visual contract and Persona, deferrals, arc42, coverage, and README impact in order.

README-IMPACT: none
