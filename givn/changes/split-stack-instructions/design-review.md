# Design Review: split-stack-instructions

Review kind: independent
Author: omp hardening author
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f

The reviewer grilled the plan in a separate context and produced sixteen
decisions; the author applied them to the change artifacts and the durable
docs, then verified the result against the served assets. The reviewer is not
the author and received no author conversation history.

## Scope

The proposal asked for four surfaces: the shipped corpus split, the six policy
instruction ids, the instruction-reference check, and the composition rule. The
plan contains exactly those, plus one grilling finding the reviewer raised
under one-canonical-owner: `tasks.md` and `tasks-reverse.md` still carried the
per-runner strict and stub rows the split moves. They now point at `steps` and
carry no stack body, and the `tasks` guidance has its own delta scenario. No
other capability, runner, `@e2e` policy, or request field is added. "Done" is
unambiguous: a core instruction names every stack id with its condition and
carries no stack body, every stack id resolves, every served direction resolves
in its named context, and the review guidance keeps every obligation.

## Decisions and landing lines

| # | Decision | Landing |
|---|---|---|
| 1 | Keep the universal binding-token list in `steps.md`; move only stack-specific forms; narrow the "no binding form" claim | `assets/instructions/steps.md:58-63` (archive token list retained), `specs/obtain-effective-guidance/serve-extended-rules.feature:22-30` |
| 2 | Every stack id gets a delta scenario: six-row load-condition outline and a six-row body outline; an empty file fails | `specs/obtain-effective-guidance/serve-extended-rules.feature:6-20, 32-49, 65-86` |
| 3 | `ise` names `steps` as the mapping and `steps-rust` as one example; fixture is stack-independent | `assets/instructions/ise.md:32-36`, `specs/fragments/instruction-integrity.feature:38-46` |
| 4 | The reference check returns `(line, line_text, id)` and treats a wrong context as a finding in one direction: a policy id carrying `--change` (narrowed by the operator correction below; an artifact id without the flag is not a finding) | `givn/changes/split-stack-instructions/design.md:334-357`; precedent `tests/steps/instruction_integrity_steps.rs:650-738` |
| 5 | The reference check covers effective addon instructions | `design.md:357-359`, `specs/fragments/instruction-integrity.feature:8-13`; `docs/arc42/08-crosscutting-concepts.md:1469-1478` |
| 6 | Caps were measured; the operator withdrew both on 2026-11-15, leaving one text composition rule | `design.md` Decisions D2 and `## Operator decision: no corpus line cap`; the two budget scenarios are deleted |
| 7 | The review guidance keeps every obligation, proved by one stable marker per obligation; the earlier line-reduction consolidation is restored | `assets/instructions/review.md:92, 166, 486-522`; `specs/fragments/instruction-integrity.feature:46-58` |
| 8 | Add the `Stack instruction` glossary row with genus, differentia, contrast, and one sentence | `docs/glossary.md:93`; `arc42.md:18, 49-53` |
| 9 | The modified ADR register scenario is existing behavior protected; the dead 28 binding is replaced by the 29 binding (landed in the operator correction's `chore(adr):` commit) | `design.md:203-211, 383-392`; owner: the batch that records the scenario |
| 10 | `ADR-077` is already accepted; correct the design's `proposed` claim | `design.md:203-211`; `docs/arc42/adr/ADR-077-stack-instructions.md:1-8` |
| 11 | Replace `reqwest` in the review guidance with "promised driving mechanism"; the per-stack mentions are interface-typed | `assets/instructions/review.md:166-170` |
| 12 | `steps-go` draws the godog strict mechanism from `design.md` as well as `steps.md` | `design.md:219`; `assets/instructions/steps-go.md:6` |
| 13 | Remove the Java parenthetical from the strict-mode paragraph | `assets/instructions/design.md:144-155` |
| 14 | Fix the canonical-owner column: item 0 by `steps` plus review/lint, item 12 is the review result, E2e coverage's opposite-classification by the interface-classification policy | `design.md:288-296` |
| 15 | Narrow the terminal baseline to "no Givn implementation symbol" | `design.md:49` |
| 16 | Record the arc42 promises the extractor enforces, and name the review-obligation scenario | `docs/arc42/08-crosscutting-concepts.md:1469-1478`, `docs/arc42/05-building-block-view.md:65`, `arc42.md:42-45` |

## Required branches

| Branch | Finding and resolution |
|---|---|
| **Scope** | No scope mismatch. The proposal's four surfaces are present. The tasks-guidance consolidation is the one added surface; it closes the one-canonical-owner gap and both files now name `steps` and carry no stack body. No out-of-scope runner, `@e2e`, or request change. |
| **Tech choices** | Rust, `cucumber-rs` 0.23.0 from `Cargo.lock`, `rust_embed` folder glob, and a test-support module. No production module is added; the six ids join `POLICY_INSTRUCTION_IDS`, not the artifact manifest. The check runs the built binary per id but is bounded to three scenarios; the alternative (a new CLI subcommand) adds public surface the change does not need. |
| **Missing scenarios** | Closed: six load-condition rows, six stack bodies (path, binding, stub, strict, command), the tasks guidance, addon-instruction coverage, the one wrong-direction case (a policy id carrying `--change`, with its artifact counterpart asserted as not a finding), and the review-obligation markers. An empty stack file fails the body outline because every required marker is missing. The `ise` scenario no longer assumes a project runner. |
| **Testability** | Every new step drives the built binary against a fixture and asserts a named sentence, absence, refusal, or finding. RED is reachable: the new step phrases are unbound before implementation, and `fail_on_skipped()` reports that as failed. Strict mode is `.fail_on_skipped()` at `tests/features_runner.rs:55,155`; the not-implemented pattern is the stack instruction's stub; the single-scenario command is `GIVN_FEATURES=... cargo test --test features_runner -- --exact "<title>"`. The ADR scenario is existing behavior protected, not a RED claim. |
| **E2E fidelity** | No `@e2e` scenario is added or removed. The CLI is the real interface; the three permanent inventory rows keep their existing drivers. The delta's regular scenarios assert invariants the `@e2e` rows do not. The terminal interface records no transcript because no delta `@e2e` scenario exists; the regular scenarios are the served-text evidence. |
| **Interaction Coverage** | The use-case inventory in `specs/obtain-effective-guidance/usecase.md` has one `serve-extended-rules` row; the design matrix maps it and the delta adds no second action. The two `instruction-integrity` rows are unchanged. No inventory entry is unmapped. |
| **Risk** | The single most likely failure is losing a review obligation while text moves; the nine marker strings prove each one survives. The withdrawn cap risk is recorded in the operator decision below: no line count is a gate, and the absence scenarios detect a core instruction that regrows a stack body. |
| **Use-case and Persona context** | `obtain-effective-guidance` and the `support` fragment keep their identity, contract, capability ownership, and typed relationships. The `ise` delta wording matches the addon instruction. The three confirmed Personas are dispositioned in the design; no invented Persona and no biography used as a Gherkin actor. |
| **Decision ledger** | Every proposal question resolves in the design: D1, D2, and D3 decide the proposal's questions; D4 to D6 are local and realized. No decision is carried; no scenario depends on a carried answer. The seed's `Q15` reference is preserved, and the new glossary row records the one term the grilling added. |
| **Contracts and components** | The served-guidance contract is unchanged; the id set grows and no field or refusal changes. The new reference check is internal with one consumer (the acceptance suite), so one module without a port is proportionate. The composition rule adds no module: it is asserted through the same served-text request. |
| **Decision timing** | No foundational question is answered by usage. No contract is treated as an open question. Reversing D4 to D6 is one array or file edit; reversing D1 to D3 re-merges files, which the ADR records. |
| **Visual design contract** | `Interface: terminal`, viewports and states not applicable, design system none. The interface is a served text surface, not a rendered screen. The terminal baseline is the served guidance and the refusal text. No delta `@e2e` scenario means no transcript is committed; the review visual block is satisfied vacuously and the regular scenarios carry the text assertions. |
| **Deferrals** | None. No mandatory check is deferred; the operator decision withdrew the budget rule and its check, and replaced them with a text rule. |
| **Persona disposition** | `agent--technical-role` loads one stack instruction; `developer--primary-user` reads the shorter core text; `givn-maintainer--owner` owns the composition rule. Each has a design impact row. A declared terminal contract requires at least one confirmed Persona; three are confirmed. |
| **Interface classification** | The opposite classification (Web UI) does not hold: the change alters served terminal text and a refusal list, and renders no HTML, CSS, or JavaScript. Playwright and browser drivers are therefore not applicable. |
| **ADR qualification** | The qualification is recorded once in `arc42.md`, `ADR-077` is accepted, and the register reads 29 accepted with no proposed. The cheaper home (chapter 08 plus the design) is named for the current facts; the ADR keeps the rejected compositions and the composition contract, and records that its budget decision was withdrawn. The modified count scenario is existing behavior protected: the permanent feature and the 29 binding were reconciled in the operator correction, so both scopes pass before archive (design.md:394-402). |
| **Architecture documentation (arc42)** | The 12-row table was re-derived from proposal, specs, and design before reading `arc42.md`; the affected rows are 02, 04, 05, 08, 09, 10, 11, and 12. Every chapter file exists with real content; no box-drawing or pseudo-diagram remains; the one diagram is Mermaid. Chapter 09's register links `ADR-077`; chapter 11 names the withdrawn `R-148` and `D-29` as retired; chapter 12 records the glossary row. `docs/arc42/05:65` and `08:1469-1530` state the check's context, line text, and addon sources, so the extractor's promises are enforced by the design rather than aspirational. |
| **Ubiquitous language** | `Stack instruction` is recorded in `docs/glossary.md:93` with genus (`served instruction`), differentia (`<id>-<stack>`, one runner's mechanics, policy id without change context), the contrast `Stack instruction, not Stack recipe`, and one domain sentence. The `ise` instruction and the use case use the same term. |
| **Structural model** | `docs/domain-model.md` needs no change: the model covers domain units and excludes internal modules, command parsing, and file formats. Instruction composition is architecture. The absence is assessed, not omitted. |

## Contract and component checks

- **Served guidance** has one implementing component (`src/cli/instructions.rs`
  with `src/config/cascade.rs`), no new parameter or field, and one added id
  set. Consumers (authoring agent, developer, generated skills) use the same
  request.
- **Guidance reference check** has one consumer (the acceptance suite) and one
  responsibility (extract and resolve named ids). It reads through the built
  binary, so an override and an addon instruction are graded as served. The
  composition rule adds no second module: the same request asserts it.
- No consumer imports a component implementation; the test-support module is
  the composition point for the acceptance suite.

## Review-context checks

- The five step-bearing guidance files (`steps`, `design`, `review`, `tasks`,
  `tasks-reverse`), `ise.md`, the six stack files, and one direction in
  `design-review.md` (the arc42 addon id drops its ignored `--change`) are the
  only corpus changes; no generated skill, command file, or production source
  was edited.
- `review.md` keeps every Step 0 obligation, the E2E coverage
  opposite-classification sentence, every sign-off item, and the spot-check
  sequence; the earlier line-reduction consolidation is restored, and the delta
  marker scenario asserts the nine stable strings.
- The shape advisory pairing the `ise` JSON scenario with the stack JSON
  scenario is a `variant`: one subject is an addon-bound id, the other a policy
  stack id, and their resolution paths differ. The review gate records the
  disposition in `review.md` at review time.
- `givn lint --change split-stack-instructions` exits 2 with 14 `@wip`
  findings (expected before implementation: two fewer after the budget
  scenarios were deleted, one fewer after the two wrong-context scenarios
  collapsed into one) and no parse, shape-blocking, or unbound-use-case
  finding. The only shape advisory is the variant above; the
  remaining `SUBST` advisories are generic-step noise from permanent scenarios.

## Instruction composition rule

The withdrawn budget is replaced by one rule, asserted as text:

- presence: `steps` and `design` name each stack id with the runner condition;
  each stack request serves that runner's binding, stub, strict mechanism, and
  single-scenario command;
- absence: the core served text carries no per-runner path, stub, binding
  instruction, strict mechanism, or run command;
- one owner: a moved row leaves the core text instead of being copied.

No line count is asserted, and `review.md` keeps its full obligation set.

## New and changed scenarios

`specs/obtain-effective-guidance/serve-extended-rules.feature`:

- `Core step guidance names each stack instruction with its load condition`
  (outline, 6 rows).
- `Core step guidance carries no stack body` (universal token list retained).
- `Design guidance names each stack instruction with its load condition and
  carries no runner table` (outline, 6 rows).
- `Specs and review guidance carry no stack body`.
- `The tasks guidance points at the stack mapping and carries no stack body`.
- `Every stack instruction is served only on its own request with its runner
  body` (outline, 6 rows: path-bearing stacks included, Go and Ruby bodies
  included).
- `The machine-readable stack path carries the same content as the text path`.
- `An unknown stack instruction id is refused by name with the known list`.

`specs/fragments/instruction-integrity.feature`:

- `Every instruction id a served instruction names resolves` (addon sources
  included).
- `A dangling stack instruction id fails the reference check by name`.
- `A policy instruction named with a change context is a finding` (also
  asserts the artifact id without the flag is not a finding).
- `The ise instruction names only instruction ids that resolve` (stack
  independent).
- `The review guidance keeps every surviving obligation` (nine stable markers).

`specs/fragments/adr-qualification-gate.feature` keeps the modified
register-count scenario, now recorded as existing behavior protected.

## Blockers and gaps

No unresolved blocker. Closed gaps, each with its landing:

| Gap | Landing |
|---|---|
| Stack bodies could survive in `steps` or `design` | six-row body outline; `assets/instructions/steps.md:27-35`, `assets/instructions/design.md:144-155` |
| The archive binding-token list could be deleted with the stub table | kept at `assets/instructions/steps.md:58-63`; `serve-extended-rules.feature:29` |
| `ise` claimed a project runner it cannot know | `assets/instructions/ise.md:32-36`; `instruction-integrity.feature:38-46` |
| The reference check could not enforce the context rule | `design.md:334-357`; line text retained, and a policy id carrying `--change` is a finding |
| Effective addon instructions were outside the source set | `design.md:357-359`; `docs/arc42/05:65` |
| Caps were measured, then withdrawn by the operator | `design.md` `## Operator decision: no corpus line cap`; no budget scenario remains |
| `review.md` obligation retention needed a mechanical proof | `instruction-integrity.feature:46-58`; the nine stable markers |
| The glossary had no `Stack instruction` row | `docs/glossary.md:93`; `arc42.md:18,49-53` |
| The design said `ADR-077` was proposed | `design.md:203-211`; the ADR is accepted |
| The dead 28 binding had no owner | `design.md:394-402`; replaced by the 29 binding in the operator correction's `chore(adr):` commit |
| `review.md` named `reqwest` as a driver counterexample | `assets/instructions/review.md:166-170` |
| `steps-go` omitted the godog strict source | `design.md:219`; `assets/instructions/steps-go.md:6` |
| The Java parenthetical contradicted the absence assertion | `assets/instructions/design.md:144-155` |
| The canonical-owner column misattributed item 0, item 12, and the E2E-coverage check | `design.md:288-296` |
| The terminal baseline banned legitimate Rust symbols | `design.md:49` |
| Arc42 promises were unenforced and the review-obligation target was unnamed | `docs/arc42/08:1469-1478`, `docs/arc42/05:65`, `arc42.md:42-45` |
| `tasks`/`tasks-reverse` kept duplicate stack bodies | `assets/instructions/tasks.md:42-60`, `assets/instructions/tasks-reverse.md:19-30`; `serve-extended-rules.feature:57-62` |

## Operator decision

Date: 2026-11-15. After this PASS, the operator withdrew the corpus line
budget. There is no hard limit on instruction length; prose that adds nothing is
still not allowed, and a rule that states valid behavior is not removed or split
to satisfy a line count. The stack split is approved on its own merits.

What changed: `assets/instructions/review.md` was restored from `aa50b100`
(the consolidation had deleted Step 0 items 0, 8, 9, 11, and 12, the
`## E2e coverage` section, the sign-off E2E block, and the spot-check section),
with the HTTP-client wording kept in place of `reqwest`; the two budget
scenarios and every cap constant were deleted; ADR-077 was rewritten to the
composition contract and records the withdrawal; chapters 02, 04, 05, 08, 10,
and 11, `plan.md`, and the ideation page were re-derived without a cap; the
design gained `## Operator decision: no corpus line cap`.

Re-derived sections: Scope, decision rows 6 and 7, Testability, Risk, Contracts
and components, Persona disposition, Deferrals, ADR qualification, Architecture
documentation, the contract checks, the review-context checks, the composition
rule section, the scenario list, the gaps table, and Verification. The affected
sections above were re-checked against the restored `review.md` and the
remaining delta scenarios. The verdict stays `PASS`; the author does not flip
it, and the independent reviewer re-validates the re-derived plan.

## Operator correction

Date: 2026-11-15. The operator narrowed the reference check's context rule and
ordered one artifact repaired. The plan was re-derived before `tasks.md` was
executed, so no task ran against the rejected rule.

**The narrowing.** The plan made a wrong context a finding in both directions:
an artifact id without `--change`, and a policy id with `--change`. The first
half was wrong. Six shipped directions name an artifact id without the flag and
are correct: `assets/instructions/characterize.md:69`,
`design-review.md:18,65`, `proposal.md:111`, `review-reverse.md:24`, and
`specs.md:326`. `run_artifact` auto-selects the single active change when no
change is named and refuses by name only when there is none or more than one
(`src/cli/instructions.rs:119-130`), so those directions resolve today; making
them findings would have forced five asset edits and would force an agent to
know the change id before reading guidance. The surviving half is the one that
is wrong: a policy id carrying `--change` is a finding, because `run_artifact`
resolves and prints a non-artifact id before it reads `change`
(`src/cli/instructions.rs:82-117`), so the flag is ignored and the direction
misleads its reader.

**The one direction the surviving half catches.** `assets/instructions/design-review.md:116`
named ``givn instructions arc42-docs --change <id>``. `arc42-docs` is not a
manifest artifact; it resolves through `resolve_instruction`'s addon fallback
(`src/config/cascade.rs:55-79`), so `run_artifact` takes the policy branch and
ignores the flag — proved by running both forms: the output is identical. That
direction is now the bare id, and `docs/arc42/adr/README.md:19`, which stated
the same direction outside served text, matches. No other shipped direction
carries a context on a policy id; every policy direction (14 `steps`,
12 `dialog`, 5 `review`, 4 `architecture-principles`, `dev-principles`,
`dialog-examples`, `tasks-reverse`, `review-reverse`, `ise`) is bare.

**Landing.** `design.md` (the reference-check contract table row, the
behaviour bullets, the external-boundary rows, the flowchart's resolution node,
the served-text sources, the Architecture Impact list, traceability row, and
D5), `specs/fragments/instruction-integrity.feature`,
`assets/instructions/design-review.md:116`,
`docs/arc42/adr/README.md:19`,
`docs/arc42/08-crosscutting-concepts.md` (the check's promise and the
generated-guidance resolution promise), and
`docs/arc42/05-building-block-view.md:65`. The two wrong-context scenarios
collapse to one: `A policy instruction named with a change context is a
finding` carries both directions and asserts the check fails naming `steps`
with the ignored context while reporting no finding for `proposal`. That
scenario keeps the classification contract: a stem-only or context-blind check
passes the `steps --change` direction wrongly, and a check that still flags the
artifact half fails the `proposal` assertion. The delta drops from 15 scenario
declarations to 14 and from 6 `instruction-integrity` declarations to 5; the
executed-row count drops from 30 to 29.

**The ADR reconciliation.** `e52b4667` accepted `ADR-077`, so
`docs/arc42/adr/README.md` counts 29 accepted rows while
`givn/specs/fragments/adr-qualification-gate.feature:123` asserted 28 and the
permanent suite was red. The same reconciliation `d5fa08ec` made at the 28
boundary is applied now rather than at archive: the permanent scenario asserts
29, the delta `@givn.modified` scenario already asserts 29 and merges to the
same body, and the dead 28 binding at
`tests/steps/adr_qualification_gate_steps.rs:881` is replaced by the 29
binding. This is existing behavior protected; no RED is claimed.

**Re-derived sections.** Scope, decisions row 4, Missing scenarios,
Testability, Risk, Contracts and components, ADR qualification, the contract
checks, the review-context checks, the scenario list, the gaps table, and
Verification. The verdict stays `PASS`; the author does not flip it, and the
independent reviewer re-validates the re-derived plan.

## Verification

```sh
givn lint --change split-stack-instructions      # exit 2, 14 @wip findings, no errors
givn status --change split-stack-instructions    # design artifacts present, tasks next
givn instructions steps | grep -c 'steps-rust'   # 1 (id and load condition present)
givn instructions steps | grep -c 'unimplemented' # 0 (no per-runner body)
givn instructions review | grep -c 'no fourth bucket' # 2 (restored obligation present)
wc -l assets/instructions/review.md              # 611, restored obligation set
cargo test --locked --test features_runner -- --exact "fragments/adr-qualification-gate.feature::The migration preserves independent architecture history across three proposed decisions" # 1 passed, 29 accepted
GIVN_FEATURES=givn/changes/split-stack-instructions/specs cargo test --locked --test features_runner -- --exact "fragments/adr-qualification-gate.feature::The migration preserves independent architecture history across three proposed decisions" # 1 passed, 29 accepted
```

DESIGN-REVIEW: PASS
