# Design: guarantee-question-traceability

## Technical Approach

Guidance and template text only. No runtime code, gate, or dependency change.
The qualifier and the ledger are authoring contracts checked by the proposal,
design, and design-review steps.

## Open-Question Qualifier

`assets/skills/givn-ideation/SKILL.md.tmpl`'s Use Case Elaboration and
guarantee guidance gains the rule: a rule or guarantee that depends on an
unresolved question carries `(open: Q<N>)` after the statement, naming the
question in `questions.md`. A qualified statement must not be presented as
unconditional; the qualifier stays until the question closes, and closing the
question removes the qualifier in the same edit. The rule lives only in the
ideation skill.

`assets/instructions/review.md`'s use-case scope rejection criteria gain the
one-line reference: an unqualified guarantee whose prerequisite is open
(`(open: Q<n>)`) is an inconsistency finding. The canonical wording stays in
the ideation skill.

## Fail-Closed Question Disposition

`assets/instructions/proposal.md` and `assets/commands/givn-propose.md` state
the rule: when a change comes from an ideation seed, every seed question that
intersects the proposal scope is dispositioned in the proposal before specs:

- **answered** — the answer is recorded, with its source;
- **carried** — kept open with an owner and the artifact that will decide it;
- **out of scope** — explicitly excluded with a reason.

An undispositioned intersecting question blocks the specs step. For changes
without an ideation source the same structure is advisory. A bare "resolved
in `design.md`" is not a disposition. A statement qualified `(open: Q<n>)`
names a question that must be dispositioned here.

## Decision Ledger

- `assets/templates/proposal.md`'s Open Questions section shows the shape: a
  `D<n>` ID, the full question with the dialog policy's option treatment, and
  the disposition.
- `assets/instructions/proposal.md` describes the IDs and the disposition
  rule.
- `assets/templates/design.md` gains a `## Decisions` section with the
  `Decides D<n>: <decision> — <where it is realized>` shape, and
  `assets/instructions/design.md` describes the contract: every `D<n>`
  appears in the ledger or is explicitly carried with a named owner. A
  question that originates in a seed records the seed reference beside the
  ledger ID (`Decides D2 (seed Q5): …`), so the handover's "Q5 must be
  dispositioned" criterion is checkable from the proposal alone.
- `assets/instructions/design-review.md` and
  `assets/commands/givn-design-review.md` require the grilling to verify every
  `D<n>`; an unresolved, unrecorded item is a finding.
- `assets/templates/design-review.md` adds the ledger checklist item.

The qualifier literal is pinned as `(open: Q<N>)` in every surface that
writes it, matching the `## Q<N>` heading in the question schema.

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| The ideation guidance carries the open-question qualifier rule | "(open: Q<N>)", "must not be presented as unconditional", "removes the qualifier" |
| The proposal instruction carries the question disposition rule | "answered", "carried", "out of scope", "is not a disposition", "(open: Q", "D1", "advisory" |
| The design and design-review guidance carry the decision ledger | "Decides D", "seed Q", "ledger", "finding" |
| The review instruction rejects an unqualified guarantee with an open prerequisite | "(open: Q<N>)" in the review instruction |
| The proposal design and design-review templates carry the ledger shapes | ejected files contain "Disposition", "## Decisions", "Decides D", "ledger" |

Each instruction Given reads the artifact's instruction plus its command file
into one buffer, so one scenario covers every surface the design edits for
that artifact.

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| One goal and one primary actor; guarantees established by the flow | `docs/use-case-model.md` | The qualifier marks an unestablished prerequisite instead of hiding it | The qualifier scenario |
| Generated guidance is synchronized from canonical assets | `givn/specs/author-change/usecase.md` | The qualifier rule lives in the ideation skill; the ledger contract in the instructions and templates | The three scenarios |
| Review records dispositions | `givn/specs/verify-change/review-disposition-gates.feature` | Design-review verifies every ledger item | The ledger scenario |

Persona impact: `agent--technical-role` (generated guidance and templates) and
`givn-maintainer--owner` (rule corpus). No Persona is invented or promoted;
Gherkin actors stay the author, maintainer, and reviewer.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` | Direct asset reads (instruction plus command), matching the existing steps in that file; adds the proposal and design/design-review Givens and the assertion steps. The template scenario reuses `givn eject` and the generic `the file "…" contains "…"` step |

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/guarantee-question-traceability/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI for the generated-guidance scenario; direct asset
  reads for the instruction scenarios.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| ideation-skill: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing) | CLI | Real `givn skills sync` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | An agent requests machine-readable artifact guidance (existing) | CLI | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end,
and the new scenarios are guidance-content variants. No browser UI exists.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | the skill renderer path that emits the ideation guidance |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

Guidance content and templates only. No parser, gate, manifest, or runtime
behavior changes; the qualifier and the ledger are prose contracts.

## Risks

- Bureaucratic overhead on small changes: the hard requirement is scoped to
  ideation-sourced changes; everything else is advisory.
- The qualifier and the `D<n>` IDs stay grep-able text; no parser or second
  numbering scheme is introduced.
- The qualifier wording lives in one canonical place (the ideation skill) and
  is referenced from the review instruction, so the two cannot drift.
