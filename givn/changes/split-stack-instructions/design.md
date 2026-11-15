# Design: split-stack-instructions

## Overview

Four coordinated surfaces, each owned by one component:

1. **Shipped guidance corpus** (`assets/instructions/`) gains six stack
   instructions (`steps-rust`, `steps-python`, `steps-node`, `steps-java`,
   `steps-go`, `steps-ruby`) holding the per-runner rows that `steps` and
   `design` carry today. Both core instructions keep the universal rule, name
   each stack instruction id with its load condition, and carry no stack body.
   `specs` and `review` carry no stack body and name no stack instruction;
   `tasks` and `tasks-reverse` point at `steps` for the mapping and carry no
   stack body; `ise` names `steps` and one example id.
2. **Instruction id set** (`src/cli/instructions.rs`) registers the six stack
   ids as policy instruction ids, so `givn instructions steps-rust` resolves
   without a change context and the unknown-id refusal lists them.
3. **Instruction reference check** (`tests/common/instruction_references.rs`,
   new) extracts every `givn instructions <id>` direction from served guidance
   and every effective addon instruction, then resolves each id in its
   documented context, so a core instruction that names an id nobody serves
   fails by name. It reads the served text, not the asset, so a project
   override is checked as served.
4. **Corpus budget** (the same change-local step file) records the pre-split
   total (1,630 lines over the four core files), the post-split cap, and the
   4,500-line corpus cap, and asserts both mechanically over the shipped
   assets.

The split moves rows; it adds no runner, changes no `@e2e` policy, and changes
no request contract: text and JSON still carry identical content.

## Visual Design Contract

```
- Interface: terminal
- Viewports: not applicable (terminal text surface)
- States: not applicable
- Design system: none (CLI)
```

`Interface: terminal` is deliberate: the change alters served terminal text and
the known-id list in a refusal, not a rendered screen. The delta adds no `@e2e`
scenario (every new scenario is a variant of the existing single request
action), so no transcript evidence is committed. The terminal baseline is the
served guidance and the refusal text:

- every served stack sentence names the id and the condition, never a bare id;
- a refusal names the requested id and lists the known ids;
- no Givn implementation symbol appears in served guidance;
- the text path and the JSON `instruction` field carry the same content.

**Personas:** `agent--technical-role`, `developer--primary-user`, and
`givn-maintainer--owner` are confirmed. `agent--technical-role` loads the stack
instruction; `developer--primary-user` reads the same text manually;
`givn-maintainer--owner` owns the corpus budget.

## Use-Case Traceability

Owning use-case IDs: `obtain-effective-guidance` (`serve-extended-rules`) and
the `support` fragment (`instruction-integrity`).
Ideation source: `ise-rebuildability`.
Confirmed Persona IDs: `agent--technical-role`, `developer--primary-user`,
`givn-maintainer--owner`.

| Use-case contract element | Technical decision | Capability / scenario evidence |
|---|---|---|
| `obtain-effective-guidance` rule: each core step instruction names each stack instruction id with the condition for loading it and carries no stack body | `steps` and `design` carry one paragraph naming six ids and their runner condition; the rows move to the stack files | `serve-extended-rules`: `Core step guidance names each stack instruction with its load condition`, `Core step guidance carries no stack body`, `Design guidance names each stack instruction with its load condition and carries no runner table` |
| `obtain-effective-guidance` rule: the core instruction's served text is complete for a change that touches no named stack, and the split removes no universal rule | the one-file-per-capability rule, the empty-body prohibition, the strict-mode proof requirement, and the single-scenario requirement stay in the core instruction without their per-runner tables | same three scenarios; each asserts a retained universal rule beside its absence assertions |
| `obtain-effective-guidance` extension: a stack instruction's body is served only by a request for its own id | `POLICY_INSTRUCTION_IDS` gains the six ids; `resolve_instruction` serves `instructions/<id>.md` | `serve-extended-rules`: `Every stack instruction is served only on its own request with its runner body` |
| `obtain-effective-guidance` extension: the machine-readable path carries the same content | unchanged JSON path; the stack id is a policy id with no `generates`/`requires` | `serve-extended-rules`: `The machine-readable stack path carries the same content as the text path` |
| `obtain-effective-guidance` extension: an unknown id keeps the refusal and lists the known ids, now including the stack ids | the same refusal string builds its list from the manifest ids plus `POLICY_INSTRUCTION_IDS` | `serve-extended-rules`: `An unknown stack instruction id is refused by name with the known list` |
| `obtain-effective-guidance` rule: `specs`, `review`, `tasks`, and `tasks-reverse` carry no stack body | their per-stack mentions are interface-typed (browser driver) or generic; `tasks` and `tasks-reverse` point at `steps`; the split moves nothing in and deletes no rule | `serve-extended-rules`: `Specs and review guidance carry no stack body`, `The tasks guidance points at the stack mapping and carries no stack body` |
| `instruction-integrity` extension: a named instruction id that does not resolve fails the check by name | new `instruction_references` check over served text and every effective addon instruction; each id resolved in its documented context, with a missing or extra context a finding | `instruction-integrity`: `Every instruction id a served instruction names resolves`, `A dangling stack instruction id fails the reference check by name`, `An artifact instruction named without its change context is a finding`, `A policy instruction named with a change context is a finding` |
| `instruction-integrity` rule: the `ise` instruction's directions resolve | `ise.md` names `steps` as the stack mapping and `steps-rust` as one example, plus the two record owners; the check resolves them while `ise` is effective | `instruction-integrity`: `The ise instruction names only instruction ids that resolve` |
| `instruction-integrity` rule: one canonical owner per rule | the moved rows leave the core instruction rather than being copied; the core points at the stack instruction | the four `serve-extended-rules` absence scenarios |
| `instruction-integrity` rule: the consolidated review guidance keeps every obligation | the review consolidation keeps one stable marker per surviving check | `instruction-integrity`: `The consolidated review guidance keeps every surviving obligation` |
| `instruction-integrity` rule: the corpus stays within its recorded budget | the four core cap and the 4,500-line corpus cap, measured over `assets/instructions/*.md` | `instruction-integrity`: `The four core step instructions stay within their recorded total`, `The shipped instruction corpus stays within its line budget` |
| `support` fragment rule: the register count matches the accepted records | `ADR-077` is already accepted and the register already counts 29, so the modified scenario is existing behavior protected; no capability behavior changes | `adr-qualification-gate`: `The migration preserves independent architecture history across three proposed decisions` (`@givn.modified`) |

## Architecture Impact

No module is added to `src/`. Three code surfaces change:

- `src/cli/instructions.rs`: `POLICY_INSTRUCTION_IDS` gains `steps-rust`,
  `steps-python`, `steps-node`, `steps-java`, `steps-go`, `steps-ruby`. Nothing
  else in the resolver changes; the cascade already serves
  `instructions/<id>.md` and the unknown-id message already builds its known
  list from the manifest plus this array.
- `assets/instructions/`: six new files, and edits to `steps.md`, `design.md`,
  `review.md`, and `ise.md`. `#[folder = "assets/"]` on
  `src/config/embedded.rs` picks up new files at build time, so no registration
  file lists them.
- `tests/common/instruction_references.rs` (new) plus `tests/steps/`: the
  instruction-direction extraction and resolution, the budget assertions, and
  one count binding for the modified register scenario.
  `tests/common/reference_check.rs` keeps its file-reference job unchanged.

No Mermaid diagram is warranted: the serving path is unchanged and the change
adds no component. The one new flow is inside the test harness:

```mermaid
flowchart LR
  Texts[[Served instruction texts]] --> Extract[[Extract givn instructions id directions]]
  Extract --> Resolve[[Resolve each id: artifact with --change, policy without]]
  Resolve -->|refused| Finding[[Finding names the id and its source line]]
  Resolve -->|served| Clean[[No finding]]
```

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Served guidance: `givn instructions <id> [--change <id>] [--json]` | obtain effective guidance | authoring agent, developer, every generated skill | `src/cli/instructions.rs` with `src/config/cascade.rs` | a second serving channel (library or HTTP), or per-id serving behaviour diverges |
| Guidance reference check: extract and resolve every named instruction id in served text | instruction integrity | the acceptance suite, the Givn maintainer | `tests/common/instruction_references.rs` (new) | a shipped `givn` command performs the check for consumer projects, or the check gains a second consumer |

The served-guidance contract is unchanged: no parameter, field, or refusal is
added; the id set grows. The check is a new internal contract with one
consumer, so one module with no port is proportionate.

## Technology Decisions

- **Language and runtime:** Rust, as the repository already builds; no version
  decision.
- **Test runner:** `cucumber-rs` 0.23.0 as `Cargo.lock` pins it, already wired
  through `tests/features_runner.rs`; no new dependency, so no version lookup
  applies. `verify.command` is `./run-tests.sh --non-e2e`; `verify.e2e_command`
  is `./run-tests.sh --e2e`.
- **Module layout:** one new test-support module named above; no new production
  module, so the dependency direction is unchanged.
- **Asset delivery:** embedded through `rust_embed`'s folder glob; no manifest
  edit, no build-script entry.

## Decisions

- Decides D1 (seed open question: which stacks receive an instruction in the
  first cut): **every runner the four core instructions name** — `steps-rust`,
  `steps-python`, `steps-node`, `steps-java`, `steps-go`, `steps-ruby`. The
  core instructions name six runners today; moving five of them and keeping one
  would leave stack bodies in the core instruction, which the capability's rule
  forbids, and would keep every author reading other stacks' rows. Rejected:
  Rust only (the core keeps five stacks' bodies and the rule stays false for
  them); the outline's five languages (the Go and Ruby stub rows lose their
  home, and the outline's separate TypeScript/JavaScript entries would split one
  `cucumber-js` runner across two instructions). Realized in the six asset files
  and the two core instructions.
- Decides D2 (proposal question: where the durable budget check lives and what
  it counts): **a permanent check over the shipped assets, counted with the
  stack instructions**. The scenario asserts `assets/instructions/*.md` totals
  at or below 4,500 lines with every stack instruction counted, and the four
  core files at or below their recorded post-split cap. The plan's `G2` is the
  gate; excluding the stack instructions would let total served text grow
  without bound by extraction, which is the failure the gate exists to prevent.
  Rejected: excluding the stack instructions (the gate stops gating); a recorded
  measurement with no check (no future detection). The cap's cost is real: this
  change must also delete about 90 lines of duplicated guidance, and the plan's
  own action on failure is to re-cut the split. Realized in the change-local
  step file and recorded in `docs/arc42/08-crosscutting-concepts.md`.
- Decides D3 (proposal question: which instruction owns the runner mechanics
  the design instruction serves): **one instruction per stack, named by both `steps` and
  `design`**. `steps-rust` holds the Rust step file, binding form, empty-body
  stub, strict-mode mechanism, and single-scenario command; `design` names the
  same id with the condition "the change's runner is Rust". The runner is one
  subject and the rows are three to five lines each, so twelve files would cost
  about 60 lines of headers the corpus cap cannot absorb. The id convention
  `<id>-<stack>` still identifies the file: the stack instruction is owned by
  `steps`, the instruction whose rows are the majority, and `design` points at
  it. Rejected: `steps-rust` plus `design-rust` and ten more (corpus cost);
  runner mechanics staying in `design` (D1's rule stays half false). Realized in
  `assets/instructions/steps-rust.md` and the `design` paragraph.
- Decides D4 (which instruction owns the id list): the six ids join
  `POLICY_INSTRUCTION_IDS`, not the artifact manifest. An artifact entry carries
  `generates`, `requires`, and a lifecycle stage; a stack instruction has none,
  and the manifest drives `givn graph` and the known-artifact refusal. Rejected:
  manifest entries (they would invent a lifecycle stage and change `givn
  graph`). Realized in `src/cli/instructions.rs`.
- Decides D5 (where the reference check lives): a new
  `tests/common/instruction_references.rs`, and the existing
  `named_instruction_ids` helper moves there from
  `tests/steps/instruction_integrity_steps.rs` so served-text and
  installed-skill checks share one extractor. The check runs the built `givn`
  binary per id, because resolution — not the asset text — is the thing that can
  fail: an id can resolve from a project override, an addon, or the embedded
  asset. Rejected: a new `givn` subcommand (a new public surface this change
  does not need); reusing `reference_check` (file references and instruction
  ids have different resolution rules). Realized in the new module.
- Decides D6 (`ise` and the stack instruction): `ise.md` names `steps` as the
  mapping for the change's stack mechanics and names `steps-rust` as one
  worked example, while keeping `design` and `proposal` as the record owners.
  `ise` is served without a change context, so it cannot name "the project's
  runner's" instruction; the mapping lives in `steps`, which the same request
  also serves. Rejected: naming the stack instruction for the project's
  runner (a static addon instruction cannot know the runner); enumerating six
  ids in `ise` (a second copy of a list the core instruction owns). Realized
  in `assets/instructions/ise.md`.

No decision is carried. D1 to D3 resolve the proposal's questions here; D4 to
D6 are local to this change and reversing any is a one-batch edit.
`ADR-077` is accepted, and the active register already reads 29 accepted
decisions. The modified register-count scenario therefore asserts existing
behaviour protected: the new `29` binding replaces the `28` phrase, and the
change-scoped run executes only the delta, so the dead `28` binding in
`tests/steps/adr_qualification_gate_steps.rs` is deleted in the same batch.
No RED is claimed for it.

## The Split: what moves where

Each stack file holds the mechanics the core instructions carried for that
runner, plus the binding form, strict instruction, and single-scenario command
the universal requirements name. Nothing is paraphrased into a new rule, and
the archive binding-token list stays in `steps` as the one statement of the
tokens the archive gate recognises.

| Stack file | Load condition (the runner) | Rows moved in | Source today | Lines |
|---|---|---|---|---|
| `steps-rust.md` | `cucumber-rs` | step definition file, binding form, empty-body stub, strict mode, single scenario | `steps.md` 22-36, 106-124; `design.md` 143-164, 25-28 | 9 |
| `steps-python.md` | `behave` | same five rows | same blocks | 9 |
| `steps-node.md` | `cucumber-js` | same five rows | same blocks | 9 |
| `steps-java.md` | `cucumber-jvm` | step definition file, binding form, empty-body stub, both strict-mode entries, single scenario | same blocks | 10 |
| `steps-go.md` | `godog` | binding registration, empty-body stub, strict behaviour, single scenario | `steps.md` 106-124; `design.md` 158 | 8 |
| `steps-ruby.md` | `cucumber-ruby` | binding form, empty-body stub, strict instruction, single scenario | `steps.md` 106-124 | 8 |

Go and Ruby had no step path, strict row, or command in the core files; the
universal requirement names a binding, a stub, a strict proof, and a single
scenario for every runner, so their stack files carry the runner's own forms.
The source column names the pre-split line ranges in the unmodified assets.

Core instructions, after the split:

| File | Keeps | Names | Removals |
|---|---|---|---|
| `steps.md` | the executable-spec rule, one file per capability, the archive merge rule, `@e2e` tag immutability, the canonical-policy pointer, visual capture, step reuse, the empty-body prohibition, the behavior batch, distinctions, guardrails | six ids, one paragraph with each runner condition | the per-runner step path list and the stub table; the archive binding-token list stays |
| `design.md` | the strict-mode requirement, the single-scenario requirement, the rest of the design contract | the same six ids with the runner condition | the strict-mode table, the per-stack command examples, and the runner-product examples in the executable-spec invariant |
| `specs.md` | unchanged | none | nothing |
| `review.md` | every check and every marker | none | duplicated restatements of the canonical E2E and step policy (see the budget) |
| `tasks.md` | the strict-proof procedure and the empty-body prohibition | `steps` as the stack mapping | the per-runner strict flags and the stub table |
| `tasks-reverse.md` | the strict-proof procedure and the empty-body prohibition | `steps` as the stack mapping | the per-runner strict flags and the stub table |
| `ise.md` | the ise obligations and the two records | `steps` as the mapping, `steps-rust` as one worked id | nothing |

The consolidated `review.md` keeps one imperative sentence per obligation and
one stable marker per obligation, asserted by the `instruction-integrity`
delta scenario.

`steps-rust.md` (the shape the other five follow):

```text
# Instruction: steps-rust

`cucumber-rs` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition file: `tests/steps/<capability>.rs`, one per capability.
- Binding form: `#[given(`, `#[when(`, `#[then(`; one expression per binding.
- Unimplemented body: `unimplemented!()` or `todo!()`.
- Strict mode: `.fail_on_skipped()` on the `Cucumber` builder.
- Single scenario: `cargo test <name> -- --exact`.
```

## The Budget

Measured before this change (`wc -l`), recorded in the change evidence:

```text
assets/instructions/*.md                    4528   (22 files)
specs.md 373  steps.md 261  design.md 386  review.md 610   → 1630
```

The 4,500-line cap is 28 lines *below* the pre-split corpus, because the three
`ise` track predecessors added 131 lines to a 4,397-line baseline. The measured
result after the split:

| Movement | Lines |
|---|---:|
| six stack files added | +53 |
| `ise.md` stack section | +4 |
| `steps.md` net (rows out, id paragraph in) | −4 |
| `design.md` net (table out, rule in) | −3 |
| `review.md` net (duplication removed) | −79 |
| `tasks.md` net (strict and stub lists removed) | −11 |
| `tasks-reverse.md` net (strict and stub lists removed) | −7 |
| **measured corpus** | **4481** |

Measured after the split: `assets/instructions/*.md` 4481 over 28 files;
`specs.md 373  steps.md 257  design.md 383  review.md 531  → 1544`. The
corpus has 19 lines of headroom.

`review.md` consolidation, block by block. Each row keeps the check and deletes
the restatement; the canonical owner keeps the rule:

| Block (today) | Lines today | After | Rule owner kept |
|---|---:|---:|---|
| Step 0 item 0 (`@e2e` tag integrity) | 8 | 5 | `givn instructions steps` owns the tag rule; the review and lint gates detect removal |
| Step 0 items 8 and 9 (E2E scope and explicit scope selection) | 13 | 5 | `givn instructions specs` owns the canonical E2E policy, including explicit scope selection |
| Step 0 item 10 (implementation matches the named design) | 17 | 3 | `design.md` owns the named commands, layout, and driver; the review checks the match |
| Step 0 items 11 and 12 (interaction inventory and finding handling) | 30 | 8 | `specs.md` and the use case own the inventory; `design.md` owns the matrix; the review records the cross-reference and the finding result |
| `## E2e coverage` | 10 | 11 | `givn instructions specs` states the E2E policy; the interface-classification policy in `specs` and `design` owns the opposite-classification check |
| Sign-off checklist E2E block | 31 | 17 | each item becomes one line naming the check |
| `## Step orchestration (review spot-check)` restating Step 0 | 34 | 15 | one sample re-check per obligation, pointing at Step 0 for the mechanics |

The `## E2e coverage` block keeps its full opposite-classification sentence,
so it measures one line longer; the total is still below the plan.

Recorded budget constants, asserted by the scenarios:

| Constant | Value | Meaning |
|---|---:|---|
| `CORE_INSTRUCTIONS_BEFORE` | 1630 | the four core files before the split (373 + 261 + 386 + 610) |
| `CORE_INSTRUCTIONS_MAX` | 1544 | the measured post-split total of the four core files |
| `CORPUS_LINES_MAX` | 4500 | every file under `assets/instructions/`, stack instructions included; the corpus measures 4481 |

`CORE_INSTRUCTIONS_MAX` is a ceiling, not a target: a measured total above it is
a `G2` failure, never a reason to raise the constant. A measured total below it
is recorded in the change evidence, and the constant stays where the design put
it so a later edit cannot spend the difference.

`CORE_INSTRUCTIONS_MAX` must stay at or below `CORE_INSTRUCTIONS_BEFORE`; the
scenario asserts that relation, so a later edit cannot raise the ceiling above
the pre-split total to make room.

If the measured result exceeds either cap, the plan's `G2` action applies: stop
and re-cut the split (drop to fewer stack files) rather than raise a cap or
delete a pinned rule. The change does not proceed to `S7` on a failed budget.

## Instruction Reference Check

`tests/common/instruction_references.rs`:

| Item | Shape | Purpose |
|---|---|---|
| `named_instruction_ids(text)` | `-> Vec<(usize, String, String)>` | every direction as its 1-based line, that line's text, and the id; a direction is a `givn instructions <id>` occurrence inside a code span (or followed by `--change`/`--json`), so prose such as "givn instructions are resolved" is not a direction; moved from `instruction_integrity_steps.rs`, one implementation |
| `IdFinding` | `struct { id, source, line, line_text, expected }` | one unresolved id or one id named in the wrong context |
| `unresolved_instruction_ids(project_root, sources, artifact_ids, change)` | `-> Vec<IdFinding>` | resolve each id with the built `givn` binary: an artifact id with `--change <change>`, a policy id with no context; a missing or extra context is a finding |
| `describe(findings)` | `-> String` | assertion message naming source line, line text, id, and expected context |

Behaviour:

- An id is resolved, never inferred from the asset set: an override, an addon
  instruction, and an embedded asset all count as resolving, and a retired or
  unknown id fails.
- The direction's context is part of the contract: an artifact id without
  `--change` and a policy id with `--change` are findings, matching the
  `all_directions`/`named_change` shape at
  `tests/steps/instruction_integrity_steps.rs:650-738`. The finding keeps the
  direction's line text, so a context mismatch names the sentence it read.
- An addon-bound id (`ise`) is checked only while the addon is effective in the
  fixture; with `ise` off the request refuses by addon name and the check skips
  that id rather than reporting a false finding.
- Read-only: no file is written, staged, or rewritten.

Served-text sources: every manifest artifact id (with the active change), every
policy id the checkout serves, and every effective addon instruction
(`assets/addons/<id>/instructions.md`). The policy set is read from the
`assets/instructions/` stems of this checkout and the addon set from
`assets/addons/`, both filtered by the fixture's effective addons; the check
never asks the binary to enumerate ids, because that would add a command
surface (D4).

## External Boundary Cases

| Boundary | Adversarial case | Expected |
|---|---|---|
| Served text → instruction id | a core instruction names `steps-rust` with no such asset | the check fails and names `steps-rust` with its source line |
| Served text → instruction id | an override names `steps-rustt` | the check fails and names `steps-rustt`; the CLI refusal keeps the unknown-id text |
| Served text → id context | an artifact direction without `--change` | a finding naming the id and the missing context |
| Served text → id context | a policy direction with `--change` | a finding naming the id and the extra context |
| Served text → addon | `ise` named while the fixture has it off | the request refuses naming `ise`; the check skips the addon-bound id |
| Served text → addon instruction | an effective addon instruction names a retired id | the check fails and names the id with its source line |
| Budget → assets | one file over the corpus cap | the corpus scenario fails naming the total and the largest file |
| Budget → core | the four core files over their cap | the core scenario fails naming the total |
| Budget → constant | a cap raised above the pre-split total | the scenario fails on the relation, not only on the measurement |

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `serve-extended-rules` | `tests/steps/serve_extended_rules_steps.rs` | same file |
| `instruction-integrity` | `tests/steps/instruction_integrity_steps.rs` | same file |
| `adr-qualification-gate` (modified register-count scenario) | `tests/steps/adr_qualification_gate_steps.rs` | same file |

Every delta phrase is bound in these permanent files; no delta step copy is
introduced. The modified register-count scenario is existing behavior
protected: it adds the binding for `the final register has 29 accepted
decisions and no proposed decisions` and deletes the now-dead `28` phrase from
`tests/steps/adr_qualification_gate_steps.rs`. The change-scoped run executes
only the delta (`GIVN_FEATURES` points at the change specs), so the deletion is
safe before the archive merge rewrites the permanent scenario; the rest of its
steps are already bound. No RED is claimed for that scenario. The new shared
module `tests/common/instruction_references.rs` is registered in
`tests/common/mod.rs` and carries no `#[given]`/`#[when]`/`#[then]`.

Not-implemented pattern: every new step drives the built `givn` binary against
its fixture or measures the checkout's shipped assets, then asserts a named
sentence, an absence, a refusal, or a count. No step substitutes a stub for a
missing production path, so a RED run fails on the assertion.

Test runner command (from `givn/commands.yaml`):

```text
./run-tests.sh --non-e2e
```

E2E runner command:

```text
./run-tests.sh --e2e
```

Single-scenario run command:

```text
GIVN_FEATURES=givn/changes/split-stack-instructions/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::Core step guidance carries no stack body"
```

## Strict Mode

`cucumber-rs` 0.23.0 with `.fail_on_skipped()` at both registration points in
`tests/features_runner.rs` (lines 55 and 155 today). A skipped step becomes a
failed step carrying `StepError::NotFound`, so an unbound delta phrase reaches
the result writer as `failed = 1` and the run exits 1. The result writer
records the four counts, and `RunnerResult::read` refuses `failed != 0`,
`skipped != 0`, `total == 0`, malformed, unreconciled, and wrong-scope results.

Proof of strictness for this change: the delta's step phrases are bound in the
two named files; the first run before those bindings exist fails as unbound,
which is the proof the runner refuses to count them as skipped.

## Interaction Coverage Matrix

The delta adds no inventory entry: requesting a stack instruction is the same
consumer action as requesting any other instruction id, with another id. The
permanent inventory rows and their `@e2e` scenarios are unchanged, and rows
for support capabilities this change does not touch stay in their permanent
use-case documents and existing designs.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `serve-extended-rules`: request effective instructions by id (permanent row; stack id is a variant) | An agent receives the extended rules through the Givn CLI | CLI | the built `givn` binary invoked in the fixture; stdout asserted |
| `instruction-integrity`: resolve an instruction and expose the machine contract (permanent row) | An agent requests machine-readable artifact guidance | CLI | the built `givn` binary with `--json`; stdout parsed |
| `instruction-integrity`: request normal session-start instructions (permanent row) | Author starts a session with the universal dialog obligation | CLI | the built `givn` binary with no id; stdout asserted |

The delta's own scenarios are regular scenarios: they assert invariants the
`@e2e` rows do not (absence of stack bodies, id resolution, budget counts), so
they do not duplicate an `@e2e` outcome.

## Persona impact

| Capability | Persona | Impact |
|---|---|---|
| `serve-extended-rules` | `agent--technical-role` | Loads one stack instruction instead of reading six runners' rows; the core instruction names the id and the condition, so the choice is mechanical. |
| `serve-extended-rules` | `developer--primary-user` | Reads the same shorter core text; the Rust mechanics stay reachable by id. |
| `serve-extended-rules` | `givn-maintainer--owner` | A moved row keeps one home; the core instruction shrinks and cannot regrow. |
| `instruction-integrity` | `agent--technical-role` | A dangling id fails by name, so a served direction never points at nothing. |
| `instruction-integrity` | `givn-maintainer--owner` | The corpus budget is measured, not remembered; the pre-split total is the ceiling. |

## E2E smoke-test infrastructure

- **Runner:** `./run-tests.sh --e2e` (`verify.e2e_command`) with
  `GIVN_RESULT_SCOPE=e2e`.
- **Step definitions:** `tests/steps/serve_extended_rules_e2e_steps.rs` and
  `tests/steps/instruction_integrity_steps.rs` (the existing `@e2e` bindings);
  the delta adds no `@e2e` scenario and no new e2e step.
- **Local infrastructure:** no server, database, container, or browser. Each
  scenario creates a `tempfile::TempDir`, writes a minimal Givn project, and
  runs the built `givn` binary (`env!("CARGO_BIN_EXE_givn")`). The budget
  scenarios read this checkout's `assets/instructions/` through
  `common::repo_path`.
- **Framework:** cucumber-rs, because the product's real interface is the CLI.
- **Strict-mode proof:** `.fail_on_skipped()` above; the delta's first RED run
  is an unbound phrase, which the runner reports as failed.

The delta adds no `@e2e` scenario, so no transcript is committed; the terminal
evidence is the served text and the refusal the regular scenarios assert.

## Local Runnability

The product needs no services: `cargo run -- instructions steps` (the declared
`run.command` family) serves the text, and `cargo run -- instructions
steps-rust` proves the new id. Tests need only the repository toolchain
(`cargo`, `nextest`, `git`, `fossil`, `git-cliff`, `script`). There is no
external third-party dependency, so no digital twin is required.

Two interface obstacles have designed fixes:

- **Served text vs asset text**: the check reads served output, so a project
  override is graded as served; the budget reads asset files, because the cap
  describes the shipped corpus. The two never share a path, which is why D5
  keeps them in separate modules.
- **Addon-bound ids in a fixture**: a fixture with `ise` off refuses `ise`; the
  check reads the fixture's effective addons before deciding to skip that id,
  so a refusal is never reported as a dangling id.

## Coverage

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and `givn` subprocesses | `./measure-coverage.sh` | cargo coverage builds for the non-E2E and E2E scopes | `coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml` | `./merge-coverages.sh` | `cli::instructions::run_artifact` (executed by every serving scenario) |

Measured population: `src/`. Excluded: tests, generated assets, fixtures, and
the instruction corpus itself. The population is declared before the first run
and is not changed after seeing the result.

The budget and reference scenarios execute no new production branch; the
serving scenarios keep `run_artifact` non-zero, which is the probe. A coverage
drop relative to the previous change is expected only in the ratio's
denominator (the delta adds test code, not production code) and is not a
finding by itself.

## Risks and Trade-offs

- **The 4,500 cap is tight.** The measured result is 4,481 lines against a
  per-file cap of 4,500; a future stack file or guidance addition spends the
  19-line headroom quickly. The mitigation is the plan's own `G2`: re-cut to
  fewer stacks rather than raise the cap. Recorded as a risk in chapter 11
  (R-148).
- **Six tiny files.** A stack file of eight to ten lines is a lookup, not a
  document; the value is that it is the only place that stack's mechanics
  live. If a seventh runner appears, its rows go into a seventh file, and the
  cap makes that cost visible.
- **`design` names `steps-<stack>`.** The pairing convention reads
  `<id>-<stack>` with `steps` as the owner while `design` points at the same
  file. The alternative (twelve files) costs more than the convention's
  awkwardness.
- **The reference check runs the binary per id.** A served-text scan of every
  instruction costs one process per id; the suite already runs the binary
  hundreds of times, so the added cost is bounded and local to two scenarios.
- **Cap constants can be edited.** A future change can raise
  `CORE_INSTRUCTIONS_MAX`, subject to the asserted relation that it never
  exceeds `CORE_INSTRUCTIONS_BEFORE`; raising `CORPUS_LINES_MAX` is a visible
  spec change, which review catches.
- **Consolidating `review.md` risks losing a check.** The plan keeps one
  imperative sentence per obligation and names the owner; review-time
  verification is that every Step 0 obligation and every sign-off item is still
  present as a check.
