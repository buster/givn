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
test that remains.

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

## RED → GREEN → REFACTOR → COMMIT (one scenario at a time)

Every phase targets the runner **at this scenario only** (name/line/tag —
never infer from whole-suite output) and captures output verbatim as evidence.

### RED

1. Remove `@wip` from this scenario only.
2. New steps use the stub above; reused steps kept as-is.
3. Run targeting this scenario. Non-@e2e: the project's `verify.command`.
   @e2e: `verify.e2e_command`. Capture output.
   - Non-zero exit → RED confirmed, proceed to GREEN.
   - Zero exit, all steps reused → legitimate immediate GREEN (record which
     steps were reused).
   - Zero exit with a new step → step is empty or runner isn't strict. STOP,
     fix before proceeding.

### GREEN

4. Replace stubs with real assertions (concrete values, not "it ran").
   - Normal TDD: write minimum production code. List every file
     created/modified — empty list on a non-reuse scenario is a red flag.
   - Characterization: probe observed behaviour, no production code
     changes. On failure, run the mismatch diagnostic.
   - E2e: bring up the local environment (design.md's Local Runnability
     section) if not already running, write glue driving the real
     interface, and assert PRIMARILY on its output. If a real-interface
     obstacle appears, apply the fix design.md names for it; if none is
     named, STOP and report the blocker — never downgrade to a
     repository-only assertion.
5. Run targeting this scenario → zero exit. Capture output.

5a. Validate coverage measurement before evaluating gaps:

    - Instrument every production process used by the scenario.
    - Launch instrumented artifacts, not normal builds.
    - Use collision-safe output; flush; merge all profiles.
    - Verify one exercised production path is non-zero.
    - Reject runner-only, test-only, absent, or zero-production coverage as
      `COVERAGE MEASUREMENT: INVALID`; fix instrumentation and rerun.
    - Only then treat uncovered production code as RED. Capture output.

### REFACTOR

6. Normal TDD: general cleanup. Characterization: testability seams only —
   revert if red. E2e: clean up infrastructure code.
7. Re-run targeting this scenario → still zero exit. Capture output.

### COMMIT

8. One commit for RED+GREEN+REFACTOR, following the commit contract
   served by `givn instructions tasks --change <id>`. Record the hash.
   No commit, or a commit touching only the spec, means the work is not
   done.

### Done

9. Paste RED/GREEN/REFACTOR evidence, files touched, and commit hash into
   the task in `tasks.md`. Only then check it: `- [ ]` → `- [x]`. Do this
   **immediately** — before moving to the next scenario. tasks.md must
   always reflect true state: an unchecked box means the scenario is not
   complete. Never leave boxes unchecked and batch-check them later.
   Using `sed` or any text-replacement tool to mass-check boxes is
   prohibited — each box is checked individually, with evidence and
   commit hash filled in.
10. Move to the next scenario.

## Distinctions

`givn lint`: static check, no tests. The project's `verify.command`: GREEN
for non-@e2e, only once strict mode is proven. `verify.e2e_command`: GREEN
for @e2e. Never conflate the three.

## Guardrails

- One scenario at a time.
- Step definitions: shared glue, one file per capability, never a
  hand-maintained parallel test file.
- Reuse existing steps; never duplicate.
- Every run targets the single scenario; output is captured, not described.
- `@e2e` GREEN requires a real-interface assertion. Repository-only
  assertions never satisfy an `@e2e` scenario, no matter how the
  interface's obstacle is explained.
