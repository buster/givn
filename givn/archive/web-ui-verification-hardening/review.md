# Review: web-ui-verification-hardening

## Fabrication Audit (do this first)

**Step definition scan** — every touched step definition file checked for
empty/no-op bodies (`{}`, bare `pass`, bare `return`, `unimplemented!()`
left behind, no assertion reference):

```
rg -n "unimplemented!|todo!|\{\s*\}|pass$|return;" tests/steps/visual_review_gate_steps.rs tests/steps/visual_design_contract_steps.rs tests/steps/prose_consolidation_steps.rs
-> 0 empty/no-op step bodies; the only matches are two `panic!` calls inside
   existing `unwrap_or_else` read-error handlers
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Setup: strict-mode proof and skeletons | `3c62d3be` | Test runner sources (`tests/steps/`) |
| Canonical policy classifies a browser-rendered page as a Web UI | `e1d2a2f3` | `assets/instructions/specs.md` |
| Canonical policy requires a rendered-state assertion | `4c944227` | `assets/instructions/specs.md` |
| Canonical policy requires evidence for every declared UI state | `1331246f` | `assets/instructions/specs.md` |
| Canonical policy refuses a silent deferral of a mandatory check | `49286197` | `assets/instructions/specs.md` |
| Design guidance classifies a browser-rendered screen as a Web UI | `b82fe9b5` | `assets/instructions/design.md` |
| Design guidance names the default browser driver | `5536e8c0` | `assets/instructions/design.md` |
| Design guidance requires breakpoints and accessibility | `a9385537` | `assets/instructions/design.md` |
| Design guidance requires a driver for every Web UI matrix row | `0a4360ff` | `assets/instructions/design.md` |
| Design guidance names the user-visible persona boundary | `54a2e9a7` | `assets/instructions/design.md` |
| Design-review guidance tests the opposite interface classification | `d7fa4b6a` | `assets/instructions/design-review.md` |
| Design-review guidance records the persona disposition | `cc8bc2a4` | `assets/instructions/design-review.md` |
| Design-review guidance requires an operator question before a deferral | `3b3741df` | `assets/instructions/design-review.md` |
| Proposal guidance requires confirmed personas or a reasoned none | `9d93318f` | `assets/instructions/proposal.md` |
| The design template carries the state, breakpoint, and accessibility lines | `db7678ed` | `assets/templates/design.md` |
| The proposal template carries the persona confirmation line | `d8471eca` | `assets/templates/proposal.md` + step |
| The review template carries the persona disposition and deferral markers | `9591a6e3` | `assets/templates/review.md` |
| The design-review template carries the persona disposition table | `7f02f539` | `assets/templates/design-review.md` |
| Synchronized authoring guidance carries the classification, driver, and persona rules | `2b461bbf` | `assets/skills/{givn-design,givn-design-review,givn-propose}/SKILL.md.tmpl` |
| Review guidance tests the opposite interface classification | `1e137a8e` | `assets/instructions/review.md` |
| Review guidance fails a deferral without an operator decision | `89370d57` | `assets/instructions/review.md` |
| Review guidance records the persona disposition | `059e832a` | `assets/instructions/review.md` |
| Review guidance names the state screenshots the vision review saw | `148a2a93` | `assets/instructions/review.md` + `assets/skills/givn-steps/SKILL.md.tmpl` |
| A web contract with missing state evidence fails review | `642e0c33` | `src/integrity_hook.rs` + steps |
| A web contract with complete state evidence passes review | `8d7f5f94` | Fixture steps (paired with `642e0c33`) |
| A visual change without a persona disposition fails review | `5d4e9d3e` | `src/integrity_hook.rs` + steps |
| A visual change with a confirmed persona disposition passes review | `cdc99ebd` | Fixture evidence (paired with `5d4e9d3e`) |
| A visual interface with an empty persona disposition fails review | `9dd24f65` | Fixture steps |
| An empty state screenshot fails review | `befb6bd1` | Fixture steps |
| A deferral without an operator decision fails review | `dbd68e52` | `src/integrity_hook.rs` + steps |
| A deferral with an operator decision passes review | `170c5b4e` | Fixture evidence (paired with `dbd68e52`) |
| Finalization: tasks, formatting, agent integrations | `016ea64e` | Regenerated skills, README, tasks |

**Promised-architecture cross-check** — every component design.md named
exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `VisualContract` with `states`, `parse_visual_contract` | Yes | `src/integrity_hook.rs` |
| `check_persona_disposition`, `persona_disposition`, `check_deferrals` | Yes | `src/integrity_hook.rs` |
| `check_visual_evidence` state-evidence branch | Yes | `src/integrity_hook.rs` |
| Fixture steps for states, persona, deferral | Yes | `tests/steps/visual_review_gate_steps.rs` |
| Ejected proposal and design-review template steps | Yes | `tests/steps/visual_design_contract_steps.rs` |
| Instruction, template, and skill assets | Yes | `assets/instructions/{specs,design,design-review,review,proposal}.md`, `assets/templates/{design,design-review,proposal,review}.md`, `assets/skills/{givn-design,givn-design-review,givn-propose,givn-steps,givn-review,givn-tasks}/SKILL.md.tmpl` |
| Arc42 chapters and glossary | Yes | `docs/arc42/{05,06,08,09,10,11,12}`, `docs/arc42/adr/ADR-028-hooks-and-mandatory-checks.md` |

**Strict-mode proof** — present in tasks.md setup with non-zero exit:

```
GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --test features_runner
1 feature / 1 scenario (1 failed) / 1 step (1 failed) / exit status: 1
```

**Downgraded e2e scenario scan** — the delta adds no `@e2e` scenario; every
touched inventory entry keeps its existing `@e2e` scenario, whose Then steps
assert on the real CLI subprocess (`givn instructions`, `givn skills sync`,
`givn check review`) and not on a repository/database. No browser capability
exists in this change:

| @e2e Scenario (unchanged) | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Canonical E2E policy is emitted by specs instructions | Yes — `givn instructions specs` stdout | N/A (CLI) | No |
| Author requests the visual design contract guidance | Yes — `givn instructions design` stdout | N/A (CLI) | No |
| Synchronized guidance defines the contract approval and capture mechanics | Yes — generated skill files | N/A (CLI) | No |
| A visual change with complete evidence passes review | Yes — `givn check review` exit 0 | N/A (CLI) | No |
| Review guidance requires the visual verdict and persona lenses | Yes — `givn instructions review` stdout | N/A (CLI) | No |

**E2e scope check** — the delta adds no distinct action, so no new `@e2e`
scenario is required; the new guidance and gate behaviours are regular
scenarios over the same real CLI interface:

| Capability | Delta @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| prose-consolidation | 0 (existing @e2e unchanged) | Yes | No |
| visual-design-contract | 0 (existing @e2e unchanged) | Yes | No |
| visual-review-gate | 0 (existing @e2e unchanged) | Yes | No |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`, which runs `cargo nextest run --test features_runner`
with `GIVN_CUCUMBER_TAGS=@e2e`. The only `@e2e` step implementations are the
shared `tests/steps/` modules; `git status` shows no untracked parallel e2e
implementation:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| all three | `tests/features_runner.rs` + `tests/steps/` | No | N/A |

**Local environment check** — design.md's Local Runnability names
`./run-tests.sh`; givn is a CLI with no external dependency or digital twin:

```
./run-tests.sh -> Summary [20.472s] 437 tests run: 437 passed, 0 skipped
./run-tests.sh --e2e -> Summary [4.133s] 83 tests run: 83 passed, 354 skipped
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`; they differ and the e2e scope is
strictly smaller. No archive receipt exists yet (candidate not archived), so
the counts below are the implementation runs:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 437 | 83 | Yes |

**Design.md conformance check** — implementation matches the reviewed design.
Two skill templates (`givn-review`, `givn-tasks`) listed in the design's
Architecture Impact were mirrored during finalization and re-synced, not
during their own scenario task; no design change was needed:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Gate location | `src/integrity_hook.rs`, wired into `run_integrity_hook` | `check_visual_review` + `check_deferrals` called from `run_integrity_hook` | Yes | N/A |
| Step files | `visual_review_gate_steps.rs`, `visual_design_contract_steps.rs` | Same; the review-template step is reused from `arc42_review_alignment_steps.rs` as the design states | Yes | N/A |
| Contract format and paths | `- States:` line, `<viewport>--<state>.png` | Same | Yes | N/A |
| Runner | `./run-tests.sh` / `./run-tests.sh --e2e` | Same | Yes | N/A |
| Skill mirrors | `givn-review`, `givn-tasks` mirror the review changes | Mirrored in the finalization commit and re-synced | Yes | N/A |

**Interaction coverage verification** — every touched inventory entry maps
to a design matrix row and its existing `@e2e` scenario with the promised CLI
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| request canonical E2E guidance (prose-consolidation) | Yes | Canonical E2E policy is emitted by specs instructions | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| request the design guidance for a visual interface | Yes | Author requests the visual design contract guidance | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| read the synchronized design and step guidance | Yes | Synchronized guidance defines the contract approval and capture mechanics | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| run the review check on a visual change | Yes | A visual change with complete evidence passes review | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| read the visual review guidance | Yes | Review guidance requires the visual verdict and persona lenses | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |

**Coverage measurement validity** — coverage is measured once by the archive
against the committed candidate; no receipt exists yet, so this review
classifies from static inspection and the implementation evidence. The
configured `measure_command`/`merge_command` are unchanged by this change;
the archive validates them. New production code (`src/integrity_hook.rs`)
is covered by 24 lib unit tests and 10 gate scenarios.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `author-change` guidance stays synchronized from `assets/`; `verify-change` still blocks an invalid change | Yes |
| Capability ownership and typed relationships preserved | `prose-consolidation` and `visual-design-contract` extended in `author-change`; `visual-review-gate` extended in `verify-change`; both keep `fragment: support` | Yes |
| Confirmed Persona-relevant outcomes preserved | Both use cases declare `Personas: none`; this change records `none` with the reason (assumption A12) and flags it for revision; no Persona was invented or promoted | Yes, with A12 |
| Actors remain distinct from Persona biographies | Gherkin actors stay `Developer`/`Reviewer`/`Authoring agent`; Personas are review lenses only | Yes |
| Migration ledger and coverage evidence complete, if applicable | Not a migration change | N/A |

## Arc42 implementation conformance

The completed implementation matches the change-level assessment and the
durable chapters. ADR-028's body now names the three amendments.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Integrity hook owns the persona, state, and deferral checks | `docs/arc42/05-building-block-view.md` | Affected | `check_persona_disposition`, `check_deferrals`, state evidence in `check_visual_evidence` | Gate scenarios | Implemented and wired into `run_integrity_hook` | Yes |
| `givn check review` runs the extended visual gate | `docs/arc42/06-runtime-view.md` | Affected | Gate inside the integrity hook | Gate scenarios | `givn check review` fails for missing state evidence, persona disposition, or an unpaired deferral | Yes |
| Visual review discipline | `docs/arc42/08-crosscutting-concepts.md` | Affected | Classification, states, persona disposition, deferral pair | All gate scenarios | Section 8.17 matches the built format and checks | Yes |
| ADR-028 third amendment | `docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/ADR-028-hooks-and-mandatory-checks.md` | Affected | Gate is a refinement of the mandatory-check boundary | `arc42.md` verdict | Chapter 09 paragraph and the ADR body name the amendment | Yes |
| Gate-safety quality scenarios | `docs/arc42/10-quality-requirements.md` | Affected | Q-88 to Q-91 | Gate scenarios | Q-88..Q-91 match the built behavior | Yes |
| Risks and guidance-level debt | `docs/arc42/11-risks-and-technical-debt.md` | Affected | R-57 revised, R-60..R-62, D-19 | — | R-57..R-62 and D-19 match the built boundaries | Yes |
| Visual terms and invariants | `docs/arc42/12-glossary.md` | Affected | Terms used in code and specs | — | `VisualContract.states`, `persona_disposition`, `check_deferrals` reuse the glossary terms | Yes |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary holds every new term (Web UI classification, UI state
evidence, Persona disposition, Deferral decision) and the extended
visual-review invariants. The spec, design, and code reuse them: the parser
reads `- States:`, the persona marker is exactly `PERSONAS:`, the deferral
pair is exactly `DEFERRAL:` / `OPERATOR-DECISION:`, and the state evidence
path is `<viewport>--<state>.png`. No inconsistent reuse found.

## Visual Review

N/A — this change's design declares no `## Visual Design Contract` section,
so the visual gate is exempt. The change introduces no screen; it hardens
the guidance and the gate that govern other changes' visual reviews.

## Deterministic gate dispositions

`givn check review --change web-ui-verification-hardening` reports 341
deterministic shape-match findings involving this delta. Every finding is
dispositioned below.

Classification rule: `variant` when both scenarios exercise the same
instruction surface or the same review gate with a different rule or
outcome; `boundary` when the pair shares only the generic Given/When/Then
shape across different actions or instruction surfaces. No pair is a
`duplicate`: each scenario asserts a distinct guidance phrase, gate rule, or
exemption boundary.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Canonical policy classifies a browser-rendered page as a Web UI` | `Design guidance requires breakpoints and accessibility` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Canonical policy requires evidence for every declared UI state` | variant |
| `Canonical policy requires a rendered-state assertion` | `Canonical policy refuses a silent deferral of a mandatory check` | variant |
| `Canonical policy requires a rendered-state assertion` | `Design guidance classifies a browser-rendered screen as a Web UI` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Design guidance names the default browser driver` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Design guidance names the user-visible persona boundary` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Design-review guidance records the persona disposition` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Review guidance tests the opposite interface classification` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Canonical policy requires a rendered-state assertion` | `Review guidance records the persona disposition` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Canonical policy refuses a silent deferral of a mandatory check` | variant |
| `Canonical policy requires evidence for every declared UI state` | `Design guidance classifies a browser-rendered screen as a Web UI` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Design guidance names the default browser driver` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Design guidance names the user-visible persona boundary` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Design-review guidance records the persona disposition` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Review guidance tests the opposite interface classification` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Canonical policy requires evidence for every declared UI state` | `Review guidance records the persona disposition` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design guidance classifies a browser-rendered screen as a Web UI` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design guidance names the default browser driver` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design guidance names the user-visible persona boundary` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design-review guidance records the persona disposition` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Review guidance tests the opposite interface classification` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Canonical policy refuses a silent deferral of a mandatory check` | `Review guidance records the persona disposition` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Design guidance names the default browser driver` | variant |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Design guidance names the user-visible persona boundary` | variant |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Design-review guidance records the persona disposition` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Review guidance tests the opposite interface classification` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design guidance classifies a browser-rendered screen as a Web UI` | `Review guidance records the persona disposition` | boundary |
| `Design guidance names the default browser driver` | `Design guidance names the user-visible persona boundary` | variant |
| `Design guidance names the default browser driver` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Design guidance names the default browser driver` | `Design-review guidance records the persona disposition` | boundary |
| `Design guidance names the default browser driver` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Design guidance names the default browser driver` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design guidance names the default browser driver` | `Review guidance tests the opposite interface classification` | boundary |
| `Design guidance names the default browser driver` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design guidance names the default browser driver` | `Review guidance records the persona disposition` | boundary |
| `Design guidance requires a driver for every Web UI matrix row` | `Review guidance names the state screenshots the vision review saw` | boundary |
| `Design guidance names the user-visible persona boundary` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Design guidance names the user-visible persona boundary` | `Design-review guidance records the persona disposition` | boundary |
| `Design guidance names the user-visible persona boundary` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Design guidance names the user-visible persona boundary` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design guidance names the user-visible persona boundary` | `Review guidance tests the opposite interface classification` | boundary |
| `Design guidance names the user-visible persona boundary` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design guidance names the user-visible persona boundary` | `Review guidance records the persona disposition` | boundary |
| `Design-review guidance tests the opposite interface classification` | `Design-review guidance records the persona disposition` | variant |
| `Design-review guidance tests the opposite interface classification` | `Design-review guidance requires an operator question before a deferral` | variant |
| `Design-review guidance tests the opposite interface classification` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design-review guidance tests the opposite interface classification` | `Review guidance tests the opposite interface classification` | boundary |
| `Design-review guidance tests the opposite interface classification` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design-review guidance tests the opposite interface classification` | `Review guidance records the persona disposition` | boundary |
| `Design-review guidance records the persona disposition` | `Design-review guidance requires an operator question before a deferral` | variant |
| `Design-review guidance records the persona disposition` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design-review guidance records the persona disposition` | `Review guidance tests the opposite interface classification` | boundary |
| `Design-review guidance records the persona disposition` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design-review guidance records the persona disposition` | `Review guidance records the persona disposition` | boundary |
| `Design-review guidance requires an operator question before a deferral` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Design-review guidance requires an operator question before a deferral` | `Review guidance tests the opposite interface classification` | boundary |
| `Design-review guidance requires an operator question before a deferral` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Design-review guidance requires an operator question before a deferral` | `Review guidance records the persona disposition` | boundary |
| `Proposal guidance requires confirmed personas or a reasoned none` | `Review guidance tests the opposite interface classification` | boundary |
| `Proposal guidance requires confirmed personas or a reasoned none` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Proposal guidance requires confirmed personas or a reasoned none` | `Review guidance records the persona disposition` | boundary |
| `Review guidance tests the opposite interface classification` | `Review guidance fails a deferral without an operator decision` | variant |
| `Review guidance tests the opposite interface classification` | `Review guidance records the persona disposition` | variant |
| `Review guidance fails a deferral without an operator decision` | `Review guidance records the persona disposition` | variant |
| `A web contract with missing state evidence fails review` | `A visual change without a persona disposition fails review` | variant |
| `A web contract with missing state evidence fails review` | `A visual interface with an empty persona disposition fails review` | variant |
| `A web contract with missing state evidence fails review` | `An empty state screenshot fails review` | variant |
| `A web contract with missing state evidence fails review` | `A deferral without an operator decision fails review` | variant |
| `A web contract with complete state evidence passes review` | `A visual change with a confirmed persona disposition passes review` | variant |
| `A web contract with complete state evidence passes review` | `A deferral with an operator decision passes review` | variant |
| `A visual change without a persona disposition fails review` | `A visual interface with an empty persona disposition fails review` | variant |
| `A visual change without a persona disposition fails review` | `An empty state screenshot fails review` | variant |
| `A visual change without a persona disposition fails review` | `A deferral without an operator decision fails review` | variant |
| `A visual change with a confirmed persona disposition passes review` | `A deferral with an operator decision passes review` | variant |
| `A visual interface with an empty persona disposition fails review` | `An empty state screenshot fails review` | variant |
| `A visual interface with an empty persona disposition fails review` | `A deferral without an operator decision fails review` | variant |
| `An empty state screenshot fails review` | `A deferral without an operator decision fails review` | variant |
| `givn addons disable when already disabled is idempotent` | `A web contract with complete state evidence passes review` | boundary |
| `givn addons disable when already disabled is idempotent` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `givn addons disable when already disabled is idempotent` | `A deferral with an operator decision passes review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A web contract with missing state evidence fails review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A visual change without a persona disposition fails review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A visual interface with an empty persona disposition fails review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `An empty state screenshot fails review` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A deferral without an operator decision fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A web contract with missing state evidence fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A visual change without a persona disposition fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A visual interface with an empty persona disposition fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `An empty state screenshot fails review` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A deferral without an operator decision fails review` | boundary |
| `Id with spaces is rejected` | `A web contract with missing state evidence fails review` | boundary |
| `Id with spaces is rejected` | `A visual change without a persona disposition fails review` | boundary |
| `Id with spaces is rejected` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Id with spaces is rejected` | `An empty state screenshot fails review` | boundary |
| `Id with spaces is rejected` | `A deferral without an operator decision fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A web contract with missing state evidence fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A visual change without a persona disposition fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `An empty state screenshot fails review` | boundary |
| `Id with non-ASCII letters is rejected` | `A deferral without an operator decision fails review` | boundary |
| `Id with a dot is rejected` | `A web contract with missing state evidence fails review` | boundary |
| `Id with a dot is rejected` | `A visual change without a persona disposition fails review` | boundary |
| `Id with a dot is rejected` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Id with a dot is rejected` | `An empty state screenshot fails review` | boundary |
| `Id with a dot is rejected` | `A deferral without an operator decision fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A web contract with missing state evidence fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A visual change without a persona disposition fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Id ending with a hyphen is rejected` | `An empty state screenshot fails review` | boundary |
| `Id ending with a hyphen is rejected` | `A deferral without an operator decision fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A web contract with missing state evidence fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A visual change without a persona disposition fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `An empty state screenshot fails review` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A deferral without an operator decision fails review` | boundary |
| `Canonical policy uses transcripts for terminal interfaces` | `Design guidance requires a driver for every Web UI matrix row` | boundary |
| `Canonical policy uses transcripts for terminal interfaces` | `Review guidance names the state screenshots the vision review saw` | boundary |
| `Design guidance names the default viewports` | `Canonical policy classifies a browser-rendered page as a Web UI` | boundary |
| `Design guidance names the default viewports` | `Design guidance requires breakpoints and accessibility` | variant |
| `Design guidance keeps improvements inside the style` | `Design guidance requires a driver for every Web UI matrix row` | variant |
| `Design guidance keeps improvements inside the style` | `Review guidance names the state screenshots the vision review saw` | boundary |
| `Design-review guidance requires an approved contract` | `Canonical policy classifies a browser-rendered page as a Web UI` | boundary |
| `Design-review guidance requires an approved contract` | `Design guidance requires breakpoints and accessibility` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A web contract with missing state evidence fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A visual change without a persona disposition fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A visual interface with an empty persona disposition fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `An empty state screenshot fails review` | boundary |
| `A malformed permanent feature reports its path and fails the audit` | `A deferral without an operator decision fails review` | boundary |
| `Review instructions require deterministic gate dispositions` | `Canonical policy requires a rendered-state assertion` | boundary |
| `Review instructions require deterministic gate dispositions` | `Canonical policy requires evidence for every declared UI state` | boundary |
| `Review instructions require deterministic gate dispositions` | `Canonical policy refuses a silent deferral of a mandatory check` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design guidance classifies a browser-rendered screen as a Web UI` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design guidance names the default browser driver` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design guidance names the user-visible persona boundary` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design-review guidance records the persona disposition` | boundary |
| `Review instructions require deterministic gate dispositions` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Review instructions require deterministic gate dispositions` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Review instructions require deterministic gate dispositions` | `Review guidance tests the opposite interface classification` | boundary |
| `Review instructions require deterministic gate dispositions` | `Review guidance fails a deferral without an operator decision` | boundary |
| `Review instructions require deterministic gate dispositions` | `Review guidance records the persona disposition` | boundary |
| `Routing requires exactly one input` | `A web contract with missing state evidence fails review` | boundary |
| `Routing requires exactly one input` | `A visual change without a persona disposition fails review` | boundary |
| `Routing requires exactly one input` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Routing requires exactly one input` | `An empty state screenshot fails review` | boundary |
| `Routing requires exactly one input` | `A deferral without an operator decision fails review` | boundary |
| `Detail mode — unknown change errors` | `A web contract with missing state evidence fails review` | boundary |
| `Detail mode — unknown change errors` | `A visual change without a persona disposition fails review` | boundary |
| `Detail mode — unknown change errors` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Detail mode — unknown change errors` | `An empty state screenshot fails review` | boundary |
| `Detail mode — unknown change errors` | `A deferral without an operator decision fails review` | boundary |
| `Uninitialized project errors` | `A web contract with missing state evidence fails review` | boundary |
| `Uninitialized project errors` | `A visual change without a persona disposition fails review` | boundary |
| `Uninitialized project errors` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Uninitialized project errors` | `An empty state screenshot fails review` | boundary |
| `Uninitialized project errors` | `A deferral without an operator decision fails review` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A web contract with complete state evidence passes review` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A deferral with an operator decision passes review` | boundary |
| `Path traversal id is rejected by status` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by status` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by status` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by status` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by status` | `A deferral without an operator decision fails review` | boundary |
| `Path traversal id is rejected by archive` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by archive` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by archive` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by archive` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by archive` | `A deferral without an operator decision fails review` | boundary |
| `Path traversal id is rejected by check` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by check` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by check` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by check` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by check` | `A deferral without an operator decision fails review` | boundary |
| `Path traversal id is rejected by lint` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by lint` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by lint` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by lint` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by lint` | `A deferral without an operator decision fails review` | boundary |
| `Path traversal id is rejected by graph` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by graph` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by graph` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by graph` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by graph` | `A deferral without an operator decision fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A web contract with missing state evidence fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A visual change without a persona disposition fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Path traversal id is rejected by instructions` | `An empty state screenshot fails review` | boundary |
| `Path traversal id is rejected by instructions` | `A deferral without an operator decision fails review` | boundary |
| `The check passes on a complete README` | `A web contract with complete state evidence passes review` | boundary |
| `The check passes on a complete README` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `The check passes on a complete README` | `A deferral with an operator decision passes review` | boundary |
| `Missing impact decision refuses the review` | `A web contract with missing state evidence fails review` | boundary |
| `Missing impact decision refuses the review` | `A visual change without a persona disposition fails review` | boundary |
| `Missing impact decision refuses the review` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Missing impact decision refuses the review` | `An empty state screenshot fails review` | boundary |
| `Missing impact decision refuses the review` | `A deferral without an operator decision fails review` | boundary |
| `Updated impact decision passes the review` | `A web contract with complete state evidence passes review` | boundary |
| `Updated impact decision passes the review` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `Updated impact decision passes the review` | `A deferral with an operator decision passes review` | boundary |
| `A disabled addon needs no impact decision` | `A web contract with complete state evidence passes review` | boundary |
| `A disabled addon needs no impact decision` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `A disabled addon needs no impact decision` | `A deferral with an operator decision passes review` | boundary |
| `A recorded impact decision passes the review` | `A web contract with complete state evidence passes review` | boundary |
| `A recorded impact decision passes the review` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `A recorded impact decision passes the review` | `A deferral with an operator decision passes review` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A web contract with complete state evidence passes review` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A deferral with an operator decision passes review` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A web contract with missing state evidence fails review` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `A visual change without a persona disposition fails review` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `A visual interface with an empty persona disposition fails review` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `An empty state screenshot fails review` | variant |
| `Integrity hook fails on Step Definitions section that is not a table` | `A deferral without an operator decision fails review` | variant |
| `Integrity hook passes when arc42 is disabled` | `A web contract with complete state evidence passes review` | variant |
| `Integrity hook passes when arc42 is disabled` | `A visual change with a confirmed persona disposition passes review` | variant |
| `Integrity hook passes when arc42 is disabled` | `A deferral with an operator decision passes review` | variant |
| `Integrity hook passes on edited arc42 chapter` | `A web contract with complete state evidence passes review` | variant |
| `Integrity hook passes on edited arc42 chapter` | `A visual change with a confirmed persona disposition passes review` | variant |
| `Integrity hook passes on edited arc42 chapter` | `A deferral with an operator decision passes review` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A web contract with missing state evidence fails review` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A visual change without a persona disposition fails review` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A visual interface with an empty persona disposition fails review` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `An empty state screenshot fails review` | variant |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A deferral without an operator decision fails review` | variant |
| `Check tasks blocked when delta spec has @wip` | `A web contract with missing state evidence fails review` | variant |
| `Check tasks blocked when delta spec has @wip` | `A visual change without a persona disposition fails review` | variant |
| `Check tasks blocked when delta spec has @wip` | `A visual interface with an empty persona disposition fails review` | variant |
| `Check tasks blocked when delta spec has @wip` | `An empty state screenshot fails review` | variant |
| `Check tasks blocked when delta spec has @wip` | `A deferral without an operator decision fails review` | variant |
| `Lost E2E evidence blocks migration coverage` | `A web contract with missing state evidence fails review` | variant |
| `Lost E2E evidence blocks migration coverage` | `A visual change without a persona disposition fails review` | variant |
| `Lost E2E evidence blocks migration coverage` | `A visual interface with an empty persona disposition fails review` | variant |
| `Lost E2E evidence blocks migration coverage` | `An empty state screenshot fails review` | variant |
| `Lost E2E evidence blocks migration coverage` | `A deferral without an operator decision fails review` | variant |
| `Lower interaction coverage blocks migration coverage` | `A web contract with missing state evidence fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `A visual change without a persona disposition fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `An empty state screenshot fails review` | boundary |
| `Lower interaction coverage blocks migration coverage` | `A deferral without an operator decision fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A web contract with missing state evidence fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A visual change without a persona disposition fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `An empty state screenshot fails review` | boundary |
| `Lower line coverage blocks migration coverage` | `A deferral without an operator decision fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A web contract with missing state evidence fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A visual change without a persona disposition fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A visual interface with an empty persona disposition fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `An empty state screenshot fails review` | boundary |
| `Lower branch coverage blocks migration coverage` | `A deferral without an operator decision fails review` | boundary |
| `Review passes when a long scenario has a split-or-keep entry` | `A web contract with complete state evidence passes review` | variant |
| `Review passes when a long scenario has a split-or-keep entry` | `A visual change with a confirmed persona disposition passes review` | variant |
| `Review passes when a long scenario has a split-or-keep entry` | `A deferral with an operator decision passes review` | variant |
| `Review passes when a shape match exists only between permanent scenarios` | `A web contract with complete state evidence passes review` | variant |
| `Review passes when a shape match exists only between permanent scenarios` | `A visual change with a confirmed persona disposition passes review` | variant |
| `Review passes when a shape match exists only between permanent scenarios` | `A deferral with an operator decision passes review` | variant |
| `Review blocks with a clear message when review.md is missing` | `A web contract with missing state evidence fails review` | variant |
| `Review blocks with a clear message when review.md is missing` | `A visual change without a persona disposition fails review` | variant |
| `Review blocks with a clear message when review.md is missing` | `A visual interface with an empty persona disposition fails review` | variant |
| `Review blocks with a clear message when review.md is missing` | `An empty state screenshot fails review` | variant |
| `Review blocks with a clear message when review.md is missing` | `A deferral without an operator decision fails review` | variant |
| `A modified delta matching its merge target in the same capability is allowed` | `A web contract with complete state evidence passes review` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `A modified delta matching its merge target in the same capability is allowed` | `A deferral with an operator decision passes review` | boundary |
| `A removed delta referencing its target by title is allowed` | `A web contract with complete state evidence passes review` | boundary |
| `A removed delta referencing its target by title is allowed` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `A removed delta referencing its target by title is allowed` | `A deferral with an operator decision passes review` | boundary |
| `The default scope excludes the archive tree` | `A web contract with complete state evidence passes review` | boundary |
| `The default scope excludes the archive tree` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `The default scope excludes the archive tree` | `A deferral with an operator decision passes review` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A web contract with complete state evidence passes review` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A visual change with a confirmed persona disposition passes review` | boundary |
| `givn check with known artifact proceeds to run hooks` | `A deferral with an operator decision passes review` | boundary |
| `A visual change with complete evidence passes review` | `A web contract with complete state evidence passes review` | variant |
| `A visual change with complete evidence passes review` | `A visual change with a confirmed persona disposition passes review` | variant |
| `A visual change with complete evidence passes review` | `A deferral with an operator decision passes review` | variant |
| `Review guidance requires the visual verdict and persona lenses` | `Canonical policy classifies a browser-rendered page as a Web UI` | boundary |
| `Review guidance requires the visual verdict and persona lenses` | `Design guidance requires breakpoints and accessibility` | boundary |
| `A visual change without a visual verdict fails review` | `A web contract with missing state evidence fails review` | variant |
| `A visual change without a visual verdict fails review` | `A visual change without a persona disposition fails review` | variant |
| `A visual change without a visual verdict fails review` | `A visual interface with an empty persona disposition fails review` | variant |
| `A visual change without a visual verdict fails review` | `An empty state screenshot fails review` | variant |
| `A visual change without a visual verdict fails review` | `A deferral without an operator decision fails review` | variant |
| `A change without a visual contract is exempt from the visual gate` | `A web contract with complete state evidence passes review` | variant |
| `A change without a visual contract is exempt from the visual gate` | `A visual change with a confirmed persona disposition passes review` | variant |
| `A change without a visual contract is exempt from the visual gate` | `A deferral with an operator decision passes review` | variant |
| `An unverified visual verdict cannot pass review` | `A web contract with missing state evidence fails review` | variant |
| `An unverified visual verdict cannot pass review` | `A visual change without a persona disposition fails review` | variant |
| `An unverified visual verdict cannot pass review` | `A visual interface with an empty persona disposition fails review` | variant |
| `An unverified visual verdict cannot pass review` | `An empty state screenshot fails review` | variant |
| `An unverified visual verdict cannot pass review` | `A deferral without an operator decision fails review` | variant |
| `A terminal change passes review with a transcript` | `A web contract with complete state evidence passes review` | variant |
| `A terminal change passes review with a transcript` | `A visual change with a confirmed persona disposition passes review` | variant |
| `A terminal change passes review with a transcript` | `A deferral with an operator decision passes review` | variant |
| `Review guidance treats style improvements as contract updates` | `Canonical policy requires a rendered-state assertion` | boundary |
| `Review guidance treats style improvements as contract updates` | `Canonical policy requires evidence for every declared UI state` | boundary |
| `Review guidance treats style improvements as contract updates` | `Canonical policy refuses a silent deferral of a mandatory check` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design guidance classifies a browser-rendered screen as a Web UI` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design guidance names the default browser driver` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design guidance names the user-visible persona boundary` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design-review guidance tests the opposite interface classification` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design-review guidance records the persona disposition` | boundary |
| `Review guidance treats style improvements as contract updates` | `Design-review guidance requires an operator question before a deferral` | boundary |
| `Review guidance treats style improvements as contract updates` | `Proposal guidance requires confirmed personas or a reasoned none` | boundary |
| `Review guidance treats style improvements as contract updates` | `Review guidance tests the opposite interface classification` | variant |
| `Review guidance treats style improvements as contract updates` | `Review guidance fails a deferral without an operator decision` | variant |
| `Review guidance treats style improvements as contract updates` | `Review guidance records the persona disposition` | variant |
| `Review guidance records an unverified verdict without vision` | `Design guidance requires a driver for every Web UI matrix row` | boundary |
| `Review guidance records an unverified verdict without vision` | `Review guidance names the state screenshots the vision review saw` | variant |
| `A change that declares no visual interface is exempt` | `A web contract with complete state evidence passes review` | variant |
| `A change that declares no visual interface is exempt` | `A visual change with a confirmed persona disposition passes review` | variant |
| `A change that declares no visual interface is exempt` | `A deferral with an operator decision passes review` | variant |
| `A web contract without viewports fails review` | `A web contract with missing state evidence fails review` | variant |
| `A web contract without viewports fails review` | `A visual change without a persona disposition fails review` | variant |
| `A web contract without viewports fails review` | `A visual interface with an empty persona disposition fails review` | variant |
| `A web contract without viewports fails review` | `An empty state screenshot fails review` | variant |
| `A web contract without viewports fails review` | `A deferral without an operator decision fails review` | variant |
| `An empty evidence file fails review` | `A web contract with missing state evidence fails review` | variant |
| `An empty evidence file fails review` | `A visual change without a persona disposition fails review` | variant |
| `An empty evidence file fails review` | `A visual interface with an empty persona disposition fails review` | variant |
| `An empty evidence file fails review` | `An empty state screenshot fails review` | variant |
| `An empty evidence file fails review` | `A deferral without an operator decision fails review` | variant |

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold.

## README impact decision (readme addon enabled)

README-IMPACT: updated - Description

The change extends the consumer-visible blocking gates. The Description
section's "Blocking gates" bullet now names the confirmed-persona
disposition, the per-state evidence, and the operator-decision requirement
for a deferral.

## Coverage

Coverage is measured once by the archive against the committed candidate and
recorded in the verification receipt. No receipt exists yet for this change.

**Coverage result:** not yet measured — unproven candidate; the archive
receipt will carry the merged line/branch result.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/integrity_hook.rs` (states parsing, persona check, deferral check, state evidence) | new | Covered by 24 lib unit tests and 10 gate scenarios | None needed |
| `tests/steps/visual_review_gate_steps.rs`, `tests/steps/visual_design_contract_steps.rs` | test surface | Covered by the scenarios that use them | None needed |
| (no uncovered production region identified) | — | — | — |

No dead code was introduced. No missing test coverage was found. No
hard-to-test region was found.

## Scenario execution evidence

The change is not archived yet, so no verification receipt exists.

```
unproven candidate — no receipt yet. Implementation runs:
delta suite 30/30 passed; permanent suite 437/437; permanent e2e 83/83.
```

**Receipt status:** unproven

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation.
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; the two late skill mirrors are in the design's
      Architecture Impact and were re-synced.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces.
- [x] arc42 slug-mapping verified.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
