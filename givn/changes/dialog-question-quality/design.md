# Design: dialog-question-quality

## Technical Approach

This change edits embedded guidance assets and adds one reachability path to
the `givn instructions` CLI. No new dependency, data model, or runtime service
is introduced. The existing asset cascade (`src/config/cascade.rs`), the
embedded asset tree (`src/config/embedded.rs`, `#[folder = "assets/"]`), and
the skill renderer (`src/skills/render.rs`) are reused unchanged.

Two mechanical pieces:

1. **The canonical policy asset** — `assets/instructions/dialog.md` is the
   single normative source for how an interactive step phrases and records a
   question. It is not an artifact and not part of any lifecycle gate.
2. **A policy instruction lookup** — `givn instructions dialog` serves that
   asset. Today `run_artifact` (`src/cli/instructions.rs`) rejects any id that
   is not a manifest artifact before resolving the instruction. The minimal
   wiring: when the id is not a manifest artifact, fall back to
   `resolve_instruction(id)`; serve the policy when it resolves, keep the
   existing `unknown artifact '<id>'` error when it does not.

## The Canonical Policy Asset

`assets/instructions/dialog.md`, one page plus the worked example:

- Scope line: applies to every interactive step — ideation, explore,
  `grill-me`, `grill-with-docs`, proposal ambiguity flagging, design-review
  grilling, review dispositions, archive confirmations.
- Rule 1 — named artifacts: every artifact is named in domain language with
  its identifier in parentheses. Keeps the existing bad/good example.
- Rule 2 — option treatment: every option states what it means, what it
  gains, what it costs, and one concrete example of what the user sees or
  what the system does. Never a bare letter or abstract label.
- Rule 3 — cross-reference gloss: every cross-reference (use case, question,
  ADR, invariant, risk, event number) carries a one-line gloss on first use
  per file or per turn.
- Rule 4 — standalone answerability: a question names the decision it blocks,
  the context that forces it, the options with gains and costs, and a
  concrete example of each.
- Rule 5 — rejected alternatives: the resolution records what was rejected
  and why, not only the chosen outcome.
- Rule 6 — one question at a time; wait for the answer.
- Closing line: "Context required to answer a question is not filler; option
  codes without meanings are."
- The worked example is the handover's full record for the remote-capability
  question (bad one-liner vs. full record with context, options, gains,
  costs, examples, and rejected alternatives). It stays in the policy; the
  policy is only loaded when explicitly requested, so its length does not
  burden every turn.

The policy's exact rule phrases are asserted by the delta scenarios:

| Delta scenario | Asserted policy phrases (implementation detail) |
|---|---|
| The canonical dialog policy is retrievable through the Givn CLI | "domain language" + "identifier in parentheses"; "what it means", "what it gains", "what it costs", "concrete example"; "answerable" + "without opening another file"; "rejected" + "resolution" |
| The canonical dialog policy demonstrates the full question record | "In practice:", "Gain:", "Cost:", "Rejected:", "one-line gloss on first use", and the outcome-only non-example |
| The dialog policy is retrievable without an active change | "domain language" + "identifier in parentheses" |
| An unresolvable instruction id keeps the known-artifact error | stderr "unknown artifact" |
| The design-review template carries the per-question record shape | "Branch", "Outcome", "In practice:", "Recommended:", "Rejected:" |

## CLI Reachability

`run_artifact` in `src/cli/instructions.rs` currently resolves the change
context first, then looks the id up in `manifest.artifacts`, bailing with the
known-artifact list when absent, then resolves the instruction. The change
reorders and extends it:

1. look the id up in `manifest.artifacts`;
2. **manifest artifact** — existing behavior: resolve the change context, then
   resolve and print the instruction;
3. **non-artifact id** — resolve the instruction through the same cascade
   before any change resolution. On success, serve it as a policy instruction
   with no active change required. On failure, print the existing
   `unknown artifact '<id>'. Known artifacts: …` error.

This ordering is deliberate: `givn instructions dialog` must work in a freshly
initialized project with no active change, while `givn instructions specs`
keeps its existing "No active changes" behavior, and a typo keeps the
known-artifact error in every project state.

JSON output for a policy keeps the stable shape: `id`, `instruction`, and
empty `generates`/`requires` arrays, so a machine consumer can parse every
instruction response identically. The `skill_to_load` addition is unchanged.

Why not register `dialog` as a manifest artifact: it has no lifecycle stage,
no generated path, and no gate. Registering it would make it appear in
`givn status`, `givn check`, and archive gating. The fallback is the smaller
change and keeps the artifact graph truthful.

Why not a separate command: the policy is instruction-shaped and belongs to
the same retrieval surface that every other canonical policy uses.

## Asset Responsibilities

The list is exhaustive; no other asset gains a dialog-policy reference.

| Asset | Change |
|---|---|
| `assets/instructions/dialog.md` | New canonical policy (above). |
| `assets/commands/givn-design-review.md` | Replace the inline dialog-discipline paragraph with a reference to the policy; keep the bad/good example. The hardening prompt states that the presentation carries the full option treatment and the record carries the rejected alternatives. |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | Same replacement; the worked bad/good example stays. |
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Reference the policy in "One question at a time"; add the `questions.md` entry schema next to the topic-state list, following the existing fenced use-case schema precedent. |
| `assets/commands/givn-ideation.md` | Reference the policy in the Stance list; keep "one question at a time". |
| `assets/skills/givn-explore/SKILL.md.tmpl` | Reference the policy in the stance. |
| `assets/commands/givn-explore.md` | Reference the policy in the stance. |
| `assets/commands/grill-me.md` | Reference the policy for how each question is phrased and recorded. |
| `assets/commands/grill-with-docs.md` | Same reference. |
| `assets/commands/givn-propose.md` | Reference the policy at the ambiguity/architecture step where questions go to the user. |
| `assets/addons/absolute-mode/policy.md` | Add the "Context required to answer a question is not filler; option codes without meanings are" line; the compression rules stay unchanged. |
| `assets/templates/design-review.md` | Grilling log becomes the scannable index `# | Branch | Question | Outcome`; every question's full record (context, options with meaning/gain/cost/example, rejected alternatives) is written below it as a per-question block in the `questions.md` shape, extended with a `Recommended:` line. |

Boundary: the policy's scope line names review dispositions and archive
confirmations as governed surfaces, but the handover's decided target list
carries no pointer edit in the review or archive guidance. Those two surfaces
stay governed by the policy without a pointer; extending pointers there is
outside this change's decided scope.

The `questions.md` schema (added to the ideation skill):

```markdown
## Q<N> — <short title in domain language>

- Status: open | closed <date> (<decider>)
- Blocks: <the decision or work item this question blocks, one line>
- Context: <why the question exists — the problem, the constraints, and
  what changes depending on the answer, 2–5 sentences>
- Question: <full standalone question>
- Options:
  1. **<option name>** — <what it means>
     In practice: <concrete example of what the user sees or the system does>
     Gain: <what improves>
     Cost: <what gets harder or is given up>
- Resolution: **<chosen option>** because <reason>. Rejected: <option>
  because <reason>; <option> because <reason>.
```

A closed question keeps its full text, options, gains, costs, examples, and
rejected alternatives; an outcome-only resolution is not a valid record.

## Preserved Phrases

Existing specs assert phrases in the surfaces being edited. They must survive
verbatim; the delta scenarios are the enforcement.

| Phrase | Asserted by |
|---|---|
| "one question at a time" | `givn/specs/fragments/ideation-skill.feature` |
| "givn-ideation" in explore guidance and command | `givn/specs/fragments/ideation-skill.feature` |
| "Overlap dispositions", "split-or-keep", "removed+added" | `givn/specs/fragments/skill-rewrites.feature` |
| "Visual Design Contract", "persona", "Relevance", "Evidence" | `givn/specs/author-change/visual-design-contract.feature` |
| "givn instructions specs" in generated givn-design-review guidance | `givn/specs/author-change/prose-consolidation.feature` |

The design-review template keeps the persona disposition table and every
existing section; only the Grilling log area gains the per-question blocks.

## Use-Case Traceability

| Contract element | Source | Technical decision | E2E evidence |
|---|---|---|---|
| Authoring guidance is canonical and synchronized from source assets (author-change rule) | `givn/specs/author-change/usecase.md` | One policy asset; every surface references it instead of restating it | Generated interactive guidance points to the canonical dialog policy |
| `givn instructions <artifact>` serves the resolved instruction (existing capability) | `instruction-integrity` | Fallback resolves non-artifact policy instructions through the same cascade | An agent requests machine-readable artifact guidance (existing @e2e) |
| Generated guidance points to the canonical policy (prose-consolidation) | `givn/specs/author-change/prose-consolidation.feature` | Dialog pointer added to the listed surfaces | Generated interactive guidance points to the canonical dialog policy (regular variant) |
| Ideation parks unknowns and asks one question at a time (ideation-skill) | `givn/specs/fragments/ideation-skill.feature` | `questions.md` schema + policy reference in the ideation skill | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing @e2e) |

Persona impact: `agent--technical-role` consumes the generated guidance and
the CLI output; `givn-maintainer--owner` owns the rule corpus and the
instruction cascade. No Persona is invented or promoted; Actors in Gherkin
remain the Developer and the Authoring agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` | Real `givn` CLI subprocess for the retrieval, no-change, and error scenarios; direct asset read for the record-shape scenario; the eject scenario reuses the existing `ejected design-review template contains` step |
| prose-consolidation | `tests/steps/prose_consolidation_steps.rs` | Real `givn` CLI subprocess; adds the `grill-<name>` generated-command assertion |
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| install-extras | `tests/steps/mod.rs` | Reuse: the existing `givn addons enable` step and the `the file "AGENTS.md" contains …` step from `tests/steps/run_execution_e2e_steps.rs`; no new file |

The instruction-integrity retrieval scenario creates its own initialized
project and change (`common::make_init_root`, `givn new`), invokes
`givn instructions dialog --change dialog-question-quality`, and asserts the
captured stdout. The record-shape scenario reads `assets/instructions/dialog.md`
directly, matching the existing "canonical specs guidance is requested"
pattern. Neither scenario calls internal Rust functions.

## Runner and Strict Mode

The runner contract is unchanged:

```text
./run-tests.sh
./run-tests.sh --e2e
```

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()` on the `Cucumber` builder, plus the
  runner's `stats.skipped > 0` exit. Proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/dialog-question-quality/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI. Scenarios drive the compiled `givn` binary as a
  real subprocess (`common::run_givn`).
- Local run command: `./run-tests.sh` (regular) and `./run-tests.sh --e2e`
  (e2e scope). No server, browser, database, queue, or third-party service is
  involved.
- Digital twins: none required — there is no external dependency.
- Anticipated interface obstacle: none. The policy is a static asset and the
  retrieval path is a local subprocess invocation; no session, auth, or
  network state is involved.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Request artifact instructions through the CLI (instruction-integrity) | An agent requests machine-readable artifact guidance | CLI | Cucumber launches the real `givn instructions specs --change … --json` subprocess and parses stdout |
| Synchronize skills and read generated ideation guidance (ideation-skill) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | CLI | Cucumber launches the real `givn skills sync` subprocess and reads the generated guidance it produced |
| Inspect generated guidance that points to the canonical policy (prose-consolidation) | Generated guidance points to the canonical E2E policy | CLI | Cucumber launches the real `givn skills sync` subprocess and reads the generated skill and command files |

The new scenarios are variants of these actions, not new actions: requesting
the dialog policy is a variant of requesting artifact instructions, and
inspecting the dialog pointers is a variant of inspecting generated guidance.
They are therefore regular scenarios; each normalized action keeps exactly one
`@e2e` scenario. No browser UI exists, so a browser driver would be the wrong
mechanism.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::cli::instructions::run_artifact` fallback path and `givn::config::cascade::resolve_instruction` |

The existing scripts provide collision-safe profiles and merge both reports;
this change does not alter that process.

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing test scripts are
unchanged.

## Architecture Impact

The `givn instructions` CLI gains a documented fallback: a non-artifact id
that resolves as an embedded instruction is served as a policy instruction.
The artifact graph, archive gates, and `givn status` are untouched. The
embedded asset tree gains one file. Guidance content changes only in the
assets listed above; the renderer, cascade, and skill-sync paths are reused
as-is.

## Risks

- The policy is referenced from many surfaces; a surface that restates the
  rules instead of referencing them recreates the drift this change removes.
  The prose-consolidation delta scenario and the review's asset scan are the
  checks.
- Removing the `Recommended answer` column from the Grilling-log index moves
  that content into the per-question blocks. The hardening prompt and the
  template must agree on the new shape or the record becomes ambiguous; both
  are edited in this change.
