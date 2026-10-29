# Instruction: tasks

Write the implementation task breakdown as coherent behavior batches. The
design and tasks guide the test-first order; the implementer follows it. Do not
record TDD order, RED proofs, or test-first evidence in the task list.

## Preconditions (before writing any task)

- **`design-review` must be done.** If `design-review` appears in the artifact
  sequence (`givn status --change <id> --json`), it must have status `done`
  before tasks are written. Never write tasks around a missing or skipped
  design-review.
- **Interaction coverage pre-flight.** Cross-check the interaction inventory in
  each owning use-case document (`usecase.md` §Interactions — never a
  `.feature` comment block) against the design's Interaction Coverage Matrix.
  Every inventory entry needs a matrix row *and* an existing `@e2e` scenario.
  Any gap is a finding: report it before writing tasks. Never write task
  entries for `@e2e` scenarios that do not exist yet.

## Setup task (first task, always)

Before writing scenario tasks, read the owning `usecase.md`, its guarantees and
rules, confirmed Persona references, and any ideation seed. Copy relevant domain
constraints into every scenario task so implementation cannot lose use-case
context between artifacts.

- Install/configure the Gherkin runner from design.md. It MUST execute
  `.feature` files under `givn/specs/` and the change's `specs/` — a bare
  unit-test runner is not sufficient.
- Configure strict mode: undefined/pending steps FAIL the build, never
  silently pass or skip.
- Set `verify.command` in `givn/commands.yaml` to the exact runner invocation.
- Create the runner test binary and step-definition skeleton, **one file per
  capability** (e.g. `CompanyOnboardingSteps`). Never one file for everything.
- Run the proof-of-strictness check (below). Do not proceed until it passes.
- **End state**: after the proof, remove or fix the probe and re-run the runner
  so it exits 0 again — back to a passing baseline with the real scenarios
  still `@wip`.

## Rule: prove strict mode once, with evidence

Empty/undefined steps silently reporting PASS is the failure mode that makes
GREEN meaningless. Prove the runner rejects them before writing any scenario:

1. Write one step using the not-implemented stub (table below) or leave it
   undefined.
2. Run the runner.
3. Confirm non-zero exit.
4. If it exits 0, enable the runner's strict flag:
   - `cucumber-rs`: `.fail_on_skipped()` on the builder.
   - `cucumber-js`: `--strict` (pin explicitly; do not rely on defaults).
   - `cucumber-jvm` CLI: `--strict`.
   - `cucumber-jvm` JUnit Platform Suite: no native strict flag — enforcement
     is 100% step-body discipline (never empty) plus a listener/plugin that
     fails the build on UNDEFINED/PENDING results.
   - `behave`: steps must `raise` explicitly; a bare `pass` is a silent PASS.
   - `godog`: `godog.ErrPending` fails by default.
5. Paste the command and non-zero-exit output into the setup task.

## Rule: step body is never empty

`{}`, bare `pass`, bare `return` — all indistinguishable from PASS. Banned
at every phase, not just RED. Unimplemented steps use:

| Language / framework | Stub |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO");` |
| Python (`behave`) | `raise NotImplementedError("TODO")` |
| JS/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO")` |

A step body is exempt only once it has a real assertion (GREEN) or performs
a real reused setup action. Reuse is never an excuse for a new step to be empty.

## Resident rules (project-start block)

The managed project-start block (`AGENTS.md`) renders the section between the
`givn:resident-rules` markers verbatim; it is part of this served instruction,
so the text has exactly one home.

<!-- givn:resident-rules:begin -->
- Tick off each scenario task when its batch is complete — not when the code
  merely compiles. Never batch-check boxes at the end.
- One atomic `givn commit` per coherent behavior batch, not per scenario. The
  revision message carries the release note as its message subject and names the batch
  in the body. For a single-scenario batch, the scenario title goes in the message body.
  A raw Git commit is stamped by the hook, a raw Fossil check-in is
  not.
- Each checked scenario records the batch `COMMIT:` revision. The commit must
  contain the scenario's implementation or regression test; a comment-only,
  tag-only, task-only, or feature-only change is not evidence.
- Tests may pass immediately when they protect a distinct requirement. Do not
  force a production diff per scenario. Deduplicate or parameterize genuine
  repetition.
<!-- givn:resident-rules:end -->

## Behavior batches — non-@e2e

Group related scenarios into one behavior batch (for example, certificate
issuance and its refusals). Work the batch, not one scenario at a time.

- Follow the test-first order the design and tasks describe. The implementer
  writes the tests and production code in that order; the task list records the
  batch commit, not the order.
- All steps must be bound. Given establishes real preconditions, When invokes
  the intended boundary, Then asserts the observable result.
- A test may pass immediately when it protects a distinct requirement. Merge,
  parameterize, or delete genuine duplication. Do not change production code
  merely to force a failure.
- Run the full batch. It must exit zero before the batch is committed.
- One batch commit, e.g. `feat(<capability>): <release note>`. The message
  carries the release note as its message subject and names the batch in the body.
  Scenarios record it as `COMMIT: <hash> -- Batch: <batch-name>`.

## Rule: @e2e tags are immutable — never remove them

Once `@e2e` is placed on a delta scenario, it MUST NOT be removed. The
`verify-e2e` hook at review time and `givn lint --change <id>` both
independently detect tag removal by cross-referencing against tasks.md
content. If e2e is blocked by infrastructure, configure
`verify.e2e_command` — never delete the tag. Removing `@e2e` tags to bypass
a gate is a procedure violation.

## Behavior batches — @e2e

Same batch loop with the e2e runner (`verify.e2e_command`), e2e step location,
and e2e infrastructure setup from design.md. Before writing or reviewing E2E
steps, read the canonical policy with `givn instructions specs --change <id>`.
It owns action normalization, real-interface assertions, and driver fidelity.
The Then step must assert the real external interface; never substitute an
internal stub for a browser or CLI E2E. Scenarios record the batch commit as
`COMMIT: <hash> -- Batch: <batch-name>`.

## E2e setup task

Before the first `@e2e` scenario:
- Bring up the local environment from design.md's Local Runnability
  section: run the local run command (docker-compose or equivalent), every
  dependency and every digital twin for external services, in the isolated
  network. Confirm it starts cleanly.
- Install the e2e framework, create e2e step skeletons in the exact
  location(s) design.md's "Step Definition Locations" section names — one
  file per capability, separate from non-@e2e steps. If design.md names
  distinct files (e.g. `foo_steps.rs` vs `e2e_steps.rs`) and reality ends up
  with one shared file instead, that is a deviation from design.md — see
  "Rule: never silently deviate from design.md" below, not a shortcut to take.
- Configure and prove strict mode for the e2e runner (same procedure as
  above), set `verify.e2e_command` using the exact command design.md names
  in its "E2E Test Infrastructure" section (e.g. the base runner plus its
  tag-filter flag/env var — `-- --tags @e2e`, `CUCUMBER_FILTER_TAGS`,
  `--strict --tags @e2e`, etc., depending on the runner from
  `design.md`/arc42 ch.4).
- **Rule: `verify.e2e_command` must be a strict subset, never equal to
  `verify.command`.** If both are byte-identical, the "e2e run" is silently
  re-running the whole suite, not isolating `@e2e` scenarios — this defeats
  the entire purpose of a separate e2e gate and is a hard-stop, fix-before-
  continuing condition, not a passable state. Prove the filter is real:
  run `verify.command` and record the scenario count in the output; run
  `verify.e2e_command` and record its scenario count; the e2e count MUST be
  strictly less than the full count (unless literally every scenario in
  scope is tagged `@e2e`, which must be stated explicitly if true). Paste
  both counts into this task.
- Confirm the e2e runner can reach the running local environment (health
  check, or a trivial smoke request) before writing any scenario steps.

## Closeout: merged step definitions

Before declaring the change archive-ready, check that every step phrase bound by
the delta step files is also bound in the permanent test tree. `givn archive`
enforces the same condition before mutation and refuses with the
project-relative delta file and the missing phrases (bounded, then `+N more`);
the closeout check catches the gap while the tasks are still open, and it never
replaces the archive gate. A delta step copy is history — the definitions are
merged into the permanent per-capability step file, never moved by archive.

## Rule: never silently deviate from design.md

Tasks execute what design.md decided — they do not re-decide it. If, while
implementing, the exact command, file layout, framework, or approach
design.md names turns out to be wrong, impractical, or an easier
alternative appears (e.g. reusing one step file instead of the two
design.md named, or pointing `verify.e2e_command` at the same command as
`verify.command` because configuring a real tag filter is inconvenient),
that is a design gap or a design.md error — not something tasks may work
around unilaterally:

1. STOP. Do not implement the deviation.
2. Update `design.md` to reflect the corrected decision, with the same
   rigor as the original section (name the exact mechanism/command/file,
   not "TBD" or "similar to below").
3. Because `design.md` changed, `design-review` un-gates and must be
   re-run (and, if the change plausibly affects any arc42 chapter — most
   commonly ch.05/06/08/09 for a structural or infrastructure decision —
   the arc42 addon's chapter table must be reassessed against the new
   design.md, not left stale).
4. Only after design-review re-passes does the task resume, now
   implementing what the corrected design.md says.

A tasks-phase shortcut that never touches design.md is not a faster path —
it is an unreviewed, unrecorded design decision made by the wrong artifact.
Reviewers (see review.md implementation evidence check) diff actual implementation
against design.md's stated commands/files/approach specifically to catch
this.

## Distinctions

- `givn lint`: static check, no tests.
- `verify.command`: confirms GREEN for non-@e2e — only once strict mode is proven.
- `verify.e2e_command`: confirms GREEN for @e2e, scoped down to `@e2e`
  scenarios only via the runner's own tag-filter mechanism — never the same
  string as `verify.command`. See "Rule" above.
- Never conflate the three.

## Tasks file format

- `[ ]`/`[x]` checkboxes. One section per scenario, grouped into behavior
  batches. Do not add TEST-FIRST, RED, or test-order fields.
- Every scenario task carries a populated **Design constraints** block copied
  from `design.md` — the implementer may not read design.md, so the task must
  carry every decision it needs. Never leave the block empty or generic.
{{#if addons.architecture_principles}}
  When the scenario touches a component, the block names the contract it uses;
  the implementer never imports the component's implementation.
{{/if}}
- Each scenario records the batch `COMMIT:` revision with a `Batch: <name>`
  label. A shared batch revision is allowed.
- Before generating many task sections, validate one through the real parser.
  Fix the format on one, then generate the rest.
- Examples:
  - `COMMIT: <hash> -- Batch: certificate-issuance`
- Non-@e2e scenarios first, @e2e last.
- Check `[x]` only when the batch commit is filled in. An empty field is
  incomplete.
- **tasks.md must always be up-to-date.** Check each scenario's box
  immediately after completing it — before starting the next scenario.
  An unchecked box means that scenario is not done. Never leave all boxes
  unchecked then batch-check them at the end. Using `sed` or any
  text-replacement tool to mass-check boxes without a commit hash is
  prohibited.
- Done when: all boxes checked, every scenario records its batch commit, both
  `verify.command` and `verify.e2e_command` exit 0 on the full suite, and the
  e2e setup task's scenario-count proof (e2e count strictly less than full
  count, or an explicit "all scenarios are @e2e" statement) is pasted.

## Step orchestration (implement)

**Audience: the agent running the implement step.** This section is the
served instruction the implement files route to; artifact authors follow the
task-list rules above.

Work the task list in the current context. The user or harness may split it into
behavior batches across sessions; the artifacts remain the handoff, and no step
requires or forbids that topology. Follow the test-first order described by the
design and tasks. Finish one batch before starting the next.

**Verify every batch yourself.** A checked box is not proof: a batch has
previously been checked off while every step was an empty stub and the runner
silently reported GREEN. Verify every batch — never a spot-check:

1. Re-run the relevant scenarios yourself and confirm zero exit.
2. Inspect the batch commit. It may cover several scenarios; it must not be a
   comment-only, tag-only, task-only, or feature-only change presented as
   behavior evidence.
3. Grep the step files for empty bodies (`{}`, bare `pass`/`return`) — any
   match rejects the batch.
4. Confirm each scenario records its batch `COMMIT:` reference, not just a
   checked box.
5. For every `@e2e` scenario, confirm at least one Then step asserts on the
   named real interface's output — never repository-only state, and never an
   internal stub substituted for a browser or CLI E2E.
6. For high-risk rules (identity, omitted alternative, boundary, lifetime
   limit, precedence), the author may add a discrimination proof: revert
   or mutate the production behavior and confirm the test fails. It is a
   risk-based check, not a required TDD record.

**GREEN requires all three**: the scenarios pass, the implementation matches
the design, and independent verification confirms it. A batch that deviates
from the design is rejected even when its tests pass. On any failure: uncheck
the affected scenarios, name the failure, and redo the batch.

**Never claim PASS without re-checking.** No empty or no-op stubs may remain,
the production files touched must be listed, and the zero exit must come from
a run you performed — never an inference from whole-suite output.
