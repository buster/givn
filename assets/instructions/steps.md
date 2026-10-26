# Instruction: steps

Shared step-writing mechanics for Gherkin `.feature` scenarios. A reference
instruction, not a standalone workflow — read by the implementation and
characterization guidance before writing step definitions.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first
question.

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is
the contract; step definitions are shared glue the runner binds to
scenarios. Never maintain a parallel hand-written test file.

When prose — a design note, a task description, or the implementation context
around you — disagrees with the `.feature` file, the `.feature` file is the
executable authority: fix the prose, never silently reinterpret or weaken the
scenario.

## Step definition locations

One file per capability — never a single file for the whole change. A
giant all-in-one step file is a red flag: it hides empty/no-op stubs.

- Rust (`cucumber-rs`): `tests/steps/<capability>.rs`.
- Python (`behave`): `features/steps/<capability>_steps.py`.
- Node (`cucumber-js`): `features/steps/<capability>.steps.js`.
- Java (`cucumber-jvm`): `src/test/java/.../steps/<Capability>Steps.java`.

`@e2e` scenarios use separate step definitions and the same one-file-per-
capability rule. Read the canonical E2E policy with
`givn instructions specs --change <id>`; do not restate its action scope or
interface policy here.

## Step definitions are durable project source

Archive merges feature files and use-case documents only. It never copies,
moves, or deletes a step definition: a delta step copy may sit beside the
change's features, but the permanent per-capability step file is its home, and
the delta copy travels into the archive as history.

Before it mutates anything, archive compares the phrases bound by the change's
delta step files with the phrases bound by the permanent test tree (the project
root minus `givn/changes`, `givn/archive`, `givn/history`, version-control
metadata, build, and cache directories). It refuses when one is missing,
naming the project-relative delta file and the missing phrases (a bounded list
followed by `+N more`). The repair is to merge each definition into its
permanent per-capability step file and rerun the archive.

A step-definition file is a file under a directory named `steps` or ending in
`_steps`, or a file whose name ends in `_steps.<ext>`. Archive reads these
binding tokens: `#[given(`, `#[when(`, `#[then(`; `@given(`, `@when(`,
`@then(`, `@step(`, `@Given(`, `@When(`, `@Then(`, `@And(`, `@But(`,
`@Step(`; `Given(`, `When(`, `Then(`, `And(`, `But(`, `Step(`; and
`.Given(`, `.When(`, `.Then(`, `.Step(`. Each binding is read as one
expression (multi-line decorators, adjacent literals, and raw-string or regex
literals included), and phrases are compared literally with internal
whitespace normalized. A binding whose phrase is computed dynamically is
skipped, so give a step a literal phrase when the archive gate must recognise
it.

## Rule: `@e2e` tags are immutable — never remove them

Once a scenario is tagged `@e2e` in a delta `.feature` file, the tag MUST
NOT be removed. If the verify-e2e gate fails because no e2e runner is
configured, fix the configuration — never delete the `@e2e` tag. Removing
`@e2e` tags to bypass the gate is a procedure violation and is independently
detected at review time (`givn check review`) and lint time (`givn lint
--change <id>`). The procedure: set `verify.e2e_command`, implement the e2e
test infrastructure, and make the scenario pass.

## Canonical E2E policy

Read `givn instructions specs --change <id>` for the real-interface
assertion, driver-fidelity, and normalized action rules. This instruction
supplies step-writing mechanics and applies the resolved design's concrete
driver; it does not define another policy.

## Visual capture mechanics

For a visual interface, each `@e2e` scenario captures a final-state
screenshot per contract viewport after the final assertion. When the contract
declares UI states, each state the scenario traverses is captured as
`<viewport>--<state>.png` (for example `mobile--empty.png`). Before every
capture, animations are frozen (or `prefers-reduced-motion` is emulated) and
fonts and network are settled, so the evidence is deterministic. Committed
evidence lives under `evidence/visual/<scenario-slug>/<viewport>.png` — or
`transcript.txt` for a terminal interface; per-assertion captures live under
`reports/` and are debug artifacts, not review evidence.

## Prefer the real interface

The black-box-first policy — served by
`givn instructions design --change <id>` — applies to every internal step or
test that remains. Prefer reused real-interface steps. For an internal step or
test that remains, record **which case this test covers that the E2E does not**;
otherwise remove the duplicate and extend the E2E scenario.

## Step reuse

Check for a matching step definition before writing a new one. Reused: keep
as-is — its assertions are already proven. New: write fresh.

## Rule: step body is never empty

`{}`, bare `pass`, bare `return`, whitespace-only — all indistinguishable
from PASS in most Cucumber runners. Banned at every phase, not just RED.
Unimplemented steps use:

| Language / framework | Stub |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO");` |
| Python (`behave`) | `raise NotImplementedError("TODO")` |
| JS/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO")` |

Exempt only once a step has a real assertion (GREEN) or performs a real
reused action. Before trusting any PASS, confirm the runner is strict; the
change's setup task proves it once, with evidence.

## Behavior batch (not per scenario)

Group related scenarios into a coherent behavior batch. The batch gets one test
phase, one implementation phase, one refactor phase, and one batch commit.
Individual examples do not need their own failing test or production diff.

### TESTS

1. Bind every Given/When/Then phrase. Given establishes real preconditions;
   When invokes the intended boundary; Then asserts the observable result.
2. Add or update the tests and step glue. Run them against the current system.
   Do not record a test revision, RED output, or TDD order.
3. A failing test must fail on its assertion, not on discovery, registration,
   step binding, import, fixture, or environment startup. Fix setup failures
   and rerun; do not write production code for them.
4. A passing test is allowed when it protects a distinct requirement. Merge,
   parameterize, or delete genuine duplication. Do not narrow correct
   production code merely to force a failure.
5. For high-risk rules (identity, omitted alternative, boundary, lifetime
   limit, precedence), the implementer may check that at least one test rejects
   a plausible wrong behavior. It is not a required TDD record.

### IMPLEMENTATION

6. Implement the behavior batch or wire the real implementation. List every
   production file created or modified. Run the relevant scenarios; they must
   exit zero. Capture the output.
   - E2e: bring up the local environment from design.md if needed; assert
     primarily on the real external interface. Never substitute an internal
     stub for a browser or CLI E2E.

5a. Validate coverage measurement before evaluating gaps:

    - Instrument every production process used by the batch.
    - Launch instrumented artifacts, not normal builds.
    - Use collision-safe output; flush; merge all profiles.
    - Verify one exercised production path is non-zero.
    - Reject runner-only, test-only, absent, or zero-production coverage as
      `COVERAGE MEASUREMENT: INVALID`; fix instrumentation and rerun.

### REFACTOR

7. Clean up without changing behaviour. Run the relevant scenarios; they must
   still exit zero. Capture the output.

### COMMIT

8. One batch commit for TESTS + IMPLEMENTATION + REFACTOR. The message carries
   the release note as its subject and names the batch in the body. Scenarios
   record it as `COMMIT: <hash> -- Batch: <batch-name>`.

### Done

9. Check the task boxes when the batch tests pass and the commit is recorded.
   Do not use mass text replacement to fake progress.
10. Move to the next behavior batch.

Characterization is a separate reverse-engineering loop. A normal change may
not use `CHARACTERIZATION` as an alternative to writing tests.

## Distinctions

`givn lint`: static check, no tests. The project's `verify.command`: GREEN
for non-@e2e, only once strict mode is proven. `verify.e2e_command`: GREEN
for @e2e. Never conflate the three.

## Guardrails

- One behavior batch at a time; batch GREEN and batch REFACTOR are allowed.
- Step definitions: shared glue, one file per capability, never a
  hand-maintained parallel test file.
- Reuse existing steps; never duplicate.
- Capture the test and implementation results you rely on; do not describe them
  from memory.
- `@e2e` GREEN requires a real-interface assertion. Repository-only
  assertions never satisfy an `@e2e` scenario, no matter how the
  interface's obstacle is explained.
