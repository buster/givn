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
instruction ids, the instruction-reference check, and the corpus budget. The
plan contains exactly those, plus one grilling finding the reviewer raised
under one-canonical-owner: `tasks.md` and `tasks-reverse.md` still carried the
per-runner strict and stub rows the split moves. They now point at `steps` and
carry no stack body, and the `tasks` guidance has its own delta scenario. No
other capability, runner, `@e2e` policy, or request field is added. "Done" is
unambiguous: both budget scenarios pass over the measured assets, every stack
id resolves, every served direction resolves in its named context, and the
consolidated review guidance keeps every obligation.

## Decisions and landing lines

| # | Decision | Landing |
|---|---|---|
| 1 | Keep the universal binding-token list in `steps.md`; move only stack-specific forms; narrow the "no binding form" claim | `assets/instructions/steps.md:58-63` (archive token list retained), `specs/obtain-effective-guidance/serve-extended-rules.feature:22-30` |
| 2 | Every stack id gets a delta scenario: six-row load-condition outline and a six-row body outline; an empty file fails | `specs/obtain-effective-guidance/serve-extended-rules.feature:6-20, 32-49, 65-86` |
| 3 | `ise` names `steps` as the mapping and `steps-rust` as one example; fixture is stack-independent | `assets/instructions/ise.md:32-36`, `specs/fragments/instruction-integrity.feature:38-46` |
| 4 | The reference check returns `(line, line_text, id)` and enforces the `--change` direction | `givn/changes/split-stack-instructions/design.md:326-345`; precedent `tests/steps/instruction_integrity_steps.rs:650-738` |
| 5 | The reference check covers effective addon instructions | `design.md:348-352`, `specs/fragments/instruction-integrity.feature:8-13`; `docs/arc42/08-crosscutting-concepts.md:1467-1473` |
| 6 | Caps are measured: `CORE_INSTRUCTIONS_MAX = 1544`, `CORPUS_LINES_MAX = 4500`, corpus 4481 | `design.md:266-307`; measured with `wc -l` below |
| 7 | `review.md` consolidation is proved by one stable marker per obligation | `assets/instructions/review.md:92-95, 158-172, 302, 335-337, 449-464`; `specs/fragments/instruction-integrity.feature:61-72` |
| 8 | Add the `Stack instruction` glossary row with genus, differentia, contrast, and one sentence | `docs/glossary.md:93`; `arc42.md:18, 49-53` |
| 9 | The modified ADR register scenario is existing behavior protected; delete the dead 28 binding in the implementation batch | `design.md:194-199, 378-386`; owner: the batch that adds the 29 binding |
| 10 | `ADR-077` is already accepted; correct the design's `proposed` claim | `design.md:194-199`; `docs/arc42/adr/ADR-077-stack-instructions-and-corpus-budget.md:1-8` |
| 11 | Replace `reqwest` in the review guidance with "promised driving mechanism"; the per-stack mentions are interface-typed | `assets/instructions/review.md:166-170` |
| 12 | `steps-go` draws the godog strict mechanism from `design.md` as well as `steps.md` | `design.md:219`; `assets/instructions/steps-go.md:6` |
| 13 | Remove the Java parenthetical from the strict-mode paragraph | `assets/instructions/design.md:144-155` |
| 14 | Fix the canonical-owner column: item 0 by `steps` plus review/lint, item 12 is the review result, E2e coverage's opposite-classification by the interface-classification policy | `design.md:288-296` |
| 15 | Narrow the terminal baseline to "no Givn implementation symbol" | `design.md:49` |
| 16 | Record the arc42 promises the extractor enforces, and name the review-consolidation scenario | `docs/arc42/08-crosscutting-concepts.md:1460-1473`, `docs/arc42/05-building-block-view.md:65`, `arc42.md:42-45` |

## Required branches

| Branch | Finding and resolution |
|---|---|
| **Scope** | No scope mismatch. The proposal's four surfaces are present. The tasks-guidance consolidation is the one added surface; it closes the one-canonical-owner gap and both files now name `steps` and carry no stack body. No out-of-scope runner, `@e2e`, or request change. |
| **Tech choices** | Rust, `cucumber-rs` 0.23.0 from `Cargo.lock`, `rust_embed` folder glob, and a test-support module. No production module is added; the six ids join `POLICY_INSTRUCTION_IDS`, not the artifact manifest. The check runs the built binary per id but is bounded to two scenarios; the alternative (a new CLI subcommand) adds public surface the change does not need. |
| **Missing scenarios** | Closed: six load-condition rows, six stack bodies (path, binding, stub, strict, command), the tasks guidance, addon-instruction coverage, both wrong-direction cases, and the review-obligation markers. An empty stack file fails the body outline because every required marker is missing. The `ise` scenario no longer assumes a project runner. |
| **Testability** | Every new step drives the built binary against a fixture or measures the checkout's assets and asserts a named sentence, absence, refusal, or count. RED is reachable: the new step phrases are unbound before implementation, and `fail_on_skipped()` reports that as failed. Strict mode is `.fail_on_skipped()` at `tests/features_runner.rs:55,155`; the not-implemented pattern is the stack instruction's stub; the single-scenario command is `GIVN_FEATURES=... cargo test --test features_runner -- --exact "<title>"`. The ADR scenario is existing behavior protected, not a RED claim. |
| **E2E fidelity** | No `@e2e` scenario is added or removed. The CLI is the real interface; the three permanent inventory rows keep their existing drivers. The delta's regular scenarios assert invariants the `@e2e` rows do not. The terminal interface records no transcript because no delta `@e2e` scenario exists; the regular scenarios are the served-text evidence. |
| **Interaction Coverage** | The use-case inventory in `specs/obtain-effective-guidance/usecase.md` has one `serve-extended-rules` row; the design matrix maps it and the delta adds no second action. The two `instruction-integrity` rows are unchanged. No inventory entry is unmapped. |
| **Risk** | The single most likely failure is the corpus cap: 4,481 measured leaves 19 lines. The mitigation is the design's own `G2`: re-cut to fewer stack files rather than raise a cap; the constants are measured, and `CORE_INSTRUCTIONS_MAX` is asserted not to exceed the pre-split total. The second risk is losing a review obligation; the nine marker strings prove each one survives. |
| **Use-case and Persona context** | `obtain-effective-guidance` and the `support` fragment keep their identity, contract, capability ownership, and typed relationships. The `ise` delta wording matches the addon instruction. The three confirmed Personas are dispositioned in the design; no invented Persona and no biography used as a Gherkin actor. |
| **Decision ledger** | Every proposal question resolves in the design: D1, D2, and D3 decide the proposal's questions; D4 to D6 are local and realized. No decision is carried; no scenario depends on a carried answer. The seed's `Q15` reference is preserved, and the new glossary row records the one term the grilling added. |
| **Contracts and components** | The served-guidance contract is unchanged; the id set grows and no field or refusal changes. The new reference check is internal with one consumer (the acceptance suite), so one module without a port is proportionate. The check and the budget are separate modules, because one reads served text and the other reads shipped assets. |
| **Decision timing** | No foundational question is answered by usage. No contract is treated as an open question. Reversing D4 to D6 is one array or file edit; reversing D1 to D3 re-merges files, which the ADR records. |
| **Visual design contract** | `Interface: terminal`, viewports and states not applicable, design system none. The interface is a served text surface, not a rendered screen. The terminal baseline is the served guidance and the refusal text. No delta `@e2e` scenario means no transcript is committed; the review visual block is satisfied vacuously and the regular scenarios carry the text assertions. |
| **Deferrals** | None. No mandatory check is deferred, so no operator decision is required. |
| **Persona disposition** | `agent--technical-role` loads one stack instruction; `developer--primary-user` reads the shorter core text; `givn-maintainer--owner` owns the corpus budget. Each has a design impact row. A declared terminal contract requires at least one confirmed Persona; three are confirmed. |
| **Interface classification** | The opposite classification (Web UI) does not hold: the change alters served terminal text and a refusal list, and renders no HTML, CSS, or JavaScript. Playwright and browser drivers are therefore not applicable. |
| **ADR qualification** | The qualification is recorded once in `arc42.md`, `ADR-077` is accepted, and the register reads 29 accepted with no proposed. The cheaper home (chapter 08 plus the design) is named for the current facts; the ADR keeps the rejected compositions and the durable ceiling. The modified count scenario is existing behavior protected; the change-scoped run executes only the delta, so the dead 28 binding is deleted in the same batch (design.md:378-386). |
| **Architecture documentation (arc42)** | The 12-row table was re-derived from proposal, specs, and design before reading `arc42.md`; the affected rows are 02, 04, 05, 08, 09, 10, 11, and 12. Every chapter file exists with real content; no box-drawing or pseudo-diagram remains; the one diagram is Mermaid. Chapter 09's register links `ADR-077`; chapter 11 carries `R-148` and `D-29`; chapter 12 now records the glossary row. `docs/arc42/05:65` and `08:1460-1473` state the check's context, line text, and addon sources, so the extractor's promises are enforced by the design rather than aspirational. |
| **Ubiquitous language** | `Stack instruction` is recorded in `docs/glossary.md:93` with genus (`served instruction`), differentia (`<id>-<stack>`, one runner's mechanics, policy id without change context), the contrast `Stack instruction, not Stack recipe`, and one domain sentence. The `ise` instruction and the use case use the same term. |
| **Structural model** | `docs/domain-model.md` needs no change: the model covers domain units and excludes internal modules, command parsing, and file formats. Instruction composition is architecture. The absence is assessed, not omitted. |

## Contract and component checks

- **Served guidance** has one implementing component (`src/cli/instructions.rs`
  with `src/config/cascade.rs`), no new parameter or field, and one added id
  set. Consumers (authoring agent, developer, generated skills) use the same
  request.
- **Guidance reference check** has one consumer (the acceptance suite) and one
  responsibility (extract and resolve named ids). It reads through the built
  binary; the budget reads assets. Splitting them keeps one responsibility per
  module and avoids a second resolution rule.
- No consumer imports a component implementation; the test-support module is
  the composition point for the acceptance suite.

## Review-context checks

- The five step-bearing guidance files (`steps`, `design`, `review`, `tasks`,
  `tasks-reverse`), `ise.md`, and the six stack files are the only corpus
  changes; no generated skill, command file, or production source was edited.
- `review.md` consolidation keeps every Step 0 obligation, the E2E coverage
  opposite-classification sentence, every sign-off item, and the spot-check
  sequence; the delta marker scenario asserts the nine stable strings.
- The shape advisory pairing the `ise` JSON scenario with the stack JSON
  scenario is a `variant`: one subject is an addon-bound id, the other a policy
  stack id, and their resolution paths differ. The review gate records the
  disposition in `review.md` at review time.
- `givn lint --change split-stack-instructions` exits 2 with 17 `@wip`
  findings (expected before implementation) and no parse, shape-blocking, or
  unbound-use-case finding. The only shape advisory is the variant above; the
  remaining `SUBST` advisories are generic-step noise from permanent scenarios.

## Measured budget

```text
before: assets/instructions/*.md 4528 (22 files)
        specs 373  steps 261  design 386  review 610  -> 1630
after:  assets/instructions/*.md 4481 (28 files)
        specs 373  steps 257  design 383  review 531  -> 1544
        stack files 53 (rust 9, python 9, node 9, java 10, go 8, ruby 8)
        ise +4, steps -4, design -3, review -79, tasks -11, tasks-reverse -7
```

`CORE_INSTRUCTIONS_MAX = 1544` and `CORPUS_LINES_MAX = 4500`; the core total is
the measured value and stays below `CORE_INSTRUCTIONS_BEFORE = 1630`. The
corpus has 19 lines of headroom. Both constants are asserted by the
`instruction-integrity` scenarios, and the core scenario asserts the relation
`CORE_INSTRUCTIONS_MAX <= CORE_INSTRUCTIONS_BEFORE`.

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
- `An artifact instruction named without its change context is a finding`.
- `A policy instruction named with a change context is a finding`.
- `The ise instruction names only instruction ids that resolve` (stack
  independent).
- `The four core step instructions stay within their recorded total`.
- `The shipped instruction corpus stays within its line budget`.
- `The consolidated review guidance keeps every surviving obligation`
  (nine stable markers).

`specs/fragments/adr-qualification-gate.feature` keeps the modified
register-count scenario, now recorded as existing behavior protected.

## Blockers and gaps

No unresolved blocker. Closed gaps, each with its landing:

| Gap | Landing |
|---|---|
| Stack bodies could survive in `steps` or `design` | six-row body outline; `assets/instructions/steps.md:27-35`, `assets/instructions/design.md:144-155` |
| The archive binding-token list could be deleted with the stub table | kept at `assets/instructions/steps.md:58-63`; `serve-extended-rules.feature:29` |
| `ise` claimed a project runner it cannot know | `assets/instructions/ise.md:32-36`; `instruction-integrity.feature:38-46` |
| The reference check could not enforce `--change` | `design.md:326-345`; line text retained |
| Effective addon instructions were outside the source set | `design.md:348-352`; `docs/arc42/05:65` |
| Caps were estimates | measured 1544/4481; `design.md:266-307` |
| `review.md` consolidation had no mechanical proof | `instruction-integrity.feature:61-72` |
| The glossary had no `Stack instruction` row | `docs/glossary.md:93`; `arc42.md:18,49-53` |
| The design said `ADR-077` was proposed | `design.md:194-199`; the ADR is accepted |
| The dead 28 binding had no owner | `design.md:378-386`; deleted with the 29 binding in the implementation batch |
| `review.md` named `reqwest` as a driver counterexample | `assets/instructions/review.md:166-170` |
| `steps-go` omitted the godog strict source | `design.md:219`; `assets/instructions/steps-go.md:6` |
| The Java parenthetical contradicted the absence assertion | `assets/instructions/design.md:144-155` |
| The canonical-owner column misattributed item 0, item 12, and the E2E-coverage check | `design.md:288-296` |
| The terminal baseline banned legitimate Rust symbols | `design.md:49` |
| Arc42 promises were unenforced and the consolidation target was unnamed | `docs/arc42/08:1460-1473`, `docs/arc42/05:65`, `arc42.md:42-45` |
| `tasks`/`tasks-reverse` kept duplicate stack bodies | `assets/instructions/tasks.md:42-60`, `assets/instructions/tasks-reverse.md:19-30`; `serve-extended-rules.feature:57-62` |

## Verification

```sh
givn lint --change split-stack-instructions   # exit 2, 17 @wip findings, no errors
givn status --change split-stack-instructions # design artifacts present, design-review next
wc -l assets/instructions/{specs,steps,design,review}.md  # 373 257 383 531 = 1544
wc -l assets/instructions/*.md                            # 4481
```

DESIGN-REVIEW: PASS
