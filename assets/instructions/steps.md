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

The stack instruction for the change's runner carries that runner's step
definition path, binding form, empty-body stub, strict mechanism, and
single-scenario command; this instruction carries no stack body:
`steps-rust` when the change's runner is `cucumber-rs`;
`steps-python` when the change's runner is `behave`;
`steps-node` when the change's runner is `cucumber-js`;
`steps-java` when the change's runner is `cucumber-jvm`;
`steps-go` when the change's runner is `godog`;
`steps-ruby` when the change's runner is `cucumber-ruby`.

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

Unimplemented steps use the empty-body stub the stack instruction names for
the change's runner; never `{}`, bare `pass`, bare `return`, or a
whitespace-only body.

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
   Tests and bindings must run before production edits for new behavior. The
   observed RED is a meaningful behavioral failure rather than missing
   bindings, syntax, registration, fixtures, or environment failure.
3. Record the actual command and a short observed failure compactly in the
   batch task as they occur. Do not reconstruct the RED from memory or history.
   A failing test must fail on its assertion, not on discovery, registration,
   step binding, import, fixture, or environment startup. Fix setup failures
   and rerun; do not write production code for them.
4. A passing test is allowed when it protects a distinct requirement: record
   the requirement explicitly as existing behavior protected, make no
   production edit for that requirement, and continue. Another requirement in
   the same coherent batch may still require behavioral RED and production
   edits. Merge, parameterize, or delete genuine duplication. Do not narrow
   correct production code merely to force a failure.
5. If production behavior was edited before the requirement's behavioral test
   was run, admit the skipped RED. Never backdate evidence, rewrite history, or
   deliberately break finished behavior. Mutation or discrimination tests are
   not claimed as historical RED. Separate RED, GREEN, and REFACTOR commits and
   retrospective phase-shaped history gates are not required.
6. For high-risk rules (identity, omitted alternative, boundary, lifetime
   limit, precedence), the implementer may check that at least one test rejects
   a plausible wrong behavior. It is not a required TDD record.

### IMPLEMENTATION

7. Implement the behavior batch or wire the real implementation. List every
   production file created or modified. Run the relevant scenarios; they must
   exit zero. Capture the output. The minimum implementation is followed by
   actual GREEN and REFACTOR checks recorded briefly in the batch task.
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

8. Clean up without changing behaviour. Run the relevant scenarios; they must
   still exit zero. Capture the output.

{{#if addons.outline}}
   When the `outline` addon is enabled, inspect its advisory report once per
   batch for supported changed components and relevant neighbors. Disclose
   which languages and components the outline covers and which it omits.
   Unsupported components receive direct source assessment using the same
   responsibility, interface, dependency-direction, and decomposition
   questions, without forced outline analysis. Unavailable outline support is
   not a failure and is not a reason for a waiver or an outline verdict.

   Read the outline before the metrics: declarations, signatures, and call
   order, without bodies or comments. State the unit's responsibility in one
   sentence and its ordered call flow. If the outline cannot carry that story
   without the body or a comment, the unit is unreadable: fix it in REFACTOR;
   if it cannot be fixed within the batch, record a substantive review
   finding. Do not waive it.

   Names are the contract. The unit and type names, method names, parameter
   names and types, return types, and called type names must state the domain
   operation in the project's vocabulary. A generic or machinery name where a
   domain noun exists (`handle`, `process`, `data`, `item`, `manager`, `util`,
   `helper`) is a readability finding.

   Ask whether responsibilities are cohesive, public interfaces narrow,
   dependencies correctly directed, and decomposition understandable. Confirm
   suspected problems against source before conclusions or edits. Source
   confirms a suspected problem; it never substitutes for the outline story.
   Structural call order is not a complete runtime trace; uncertain or omitted
   edges limit the conclusions.
   Metrics and classifications are hints: a clean metric never excuses an
   unreadable outline, and no metric waives it. Reuse the report until
   structural changes make its relevant evidence stale: a later structural
   edit is the reason to regenerate it, not every small edit, and
   nonstructural edits alone do not require rerunning the outline. The outline
   report is reused by final review as advisory evidence; it supplies no
   blocking verdict, metric waiver, or forced reviewer context.
{{/if}}

   Concrete cleanup encountered while implementing agreed behavior is made in a
   separate behavior-preserving refactoring commit with explicit paths and
   actual GREEN checks. It does not authorize speculative features or a broad
   abstraction rewrite.

### COMMIT

9. Inspect the intended explicit-path diff before a coherent completed batch
   commit. Commit selections name the batch files explicitly rather than
   implicitly staging all work. One batch commit covers TESTS + IMPLEMENTATION
   + REFACTOR. The message carries the release note as its subject and names
   the batch in the body. Record the actual reachable revision before scenario
   tasks are completed; scenarios record it as
   `COMMIT: <hash> -- Batch: <batch-name>`. No production diff is required for
   each scenario that protects existing behavior.

### Done

10. Check the task boxes when the batch tests pass and the commit is recorded.
    Do not use mass text replacement to fake progress.
11. Move to the next behavior batch.

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
