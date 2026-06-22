---
name: givn-characterize
description: Execute the characterization loop for a reverse-engineering change — GREEN-first tests, testability refactor, reverse review.
---

# givn-characterize

Execute the characterization loop for change `<change-id>`.

## Context

- Tasks: `givn/changes/<change-id>/tasks.md`
- Spec: `givn/changes/<change-id>/specs/`
- Design: `givn/changes/<change-id>/design.md`
- Test runner: read `verify.command` from `givn/config.yaml` at runtime (MUST
  be a Gherkin/Cucumber runner that executes the `.feature` files — see the
  executable-spec invariant below; never the `givn missing-testrunner`
  sentinel)
- Features path: `GIVN_FEATURES=givn/specs`

## Executable-spec invariant (enforced)

`verify.command` MUST be a Gherkin/Cucumber runner that executes the
`.feature` files (e.g. `cucumber-rs` for Rust, `behave` for Python,
`cucumber-js` for Node). "Step definitions" always means the bindings the
runner uses to map `.feature` steps to code — never a parallel
hand-maintained test file. If `verify.command` is still the `givn
missing-testrunner` sentinel, stop and set it (per the design artifact)
before continuing.

## The characterization technique

GREEN-first, not RED-first:

1. **GREEN**: write Gherkin step definitions asserting **observed** behaviour
   (probe the real code). The runner executes the `.feature` file and binds
   the steps to these definitions — never a parallel `#[test]` fn. Remove
   `@wip` from this scenario only. Run `verify.command` (the Gherkin runner)
   → must pass immediately.
2. If it fails: diagnose (infrastructure problem → fix plumbing; spec/mismatch
   → correct the spec, never the code).
3. **REFACTOR**: if design.md calls for a testability seam, apply it. Run
   `verify.command` → must still pass. If red, revert the refactor.
4. Check off the task. Move to the next scenario.

## What not to do

- Do NOT change production code behaviour to satisfy a test.
- Do NOT fix bugs discovered during characterization. Flag them.
- Do NOT use `givn check <artifact>` during the loop — run `verify.command`
  (the Gherkin runner) directly. `givn check` is only for the review epilogue.
- Do NOT write a parallel `#[test]`/unit-test file alongside the `.feature`
  file — the Gherkin runner executes the `.feature` file via step definitions.
- Do NOT do general cleanup (rename, dedup, logic simplification) during
  REFACTOR — only testability seams (extract interface, DI, make public).
- Do NOT remove `REVERSE-ENGINEERING NOTE` markers — they are intentional debt
  markers.

## Spec/implementation mismatch diagnostic

When verify fails and infrastructure is sound:

1. **Diagnose**: documentation was wrong, or code is ambiguous/misleading.
2. **If docs wrong**: update design.md + tasks.md + the .feature scenario +
   the source documentation. Re-run verify.
3. **If code ambiguous**: update the .feature scenario to AS-IS behaviour. Add
   a code comment: `REVERSE-ENGINEERING NOTE: <what was misleading> — actual
   behaviour: <Z>. Code smell, refactor candidate.` (Use the language's comment
   syntax.) Do NOT change code behaviour. Re-run verify.

## Reverse review epilogue

After all tasks are checked, a review subagent:
- Runs coverage (record the number, classify gaps). Coverage MUST be measured
  across all test binaries, including the Gherkin runner that executes the
  `.feature` files — not unit tests alone. A coverage run that omits the
  `.feature` execution is invalid and misleads the gap classification.
- Dead code → flag as `@givn.removed` candidate (do NOT delete).
- Missing test → flag as `@givn.added` candidate (do NOT add during review).
- Redundant unit tests (fully covered by a `.feature` scenario) → remove and
  list in review.md with the superseding scenario; confirm coverage unchanged.
- Confirms behaviour-preservation invariant (all characterization tests still
  GREEN after refactoring).
- Compiles: spec corrections, documentation corrections, code smell markers,
  suspected bugs.
- Writes `review.md` with `REVIEW: PASS`.
- Runs `givn check review --change <change-id>`.

## Transition out

When characterization + review are complete, suggest:
`/givn-archive` — archive the change and create the baseline spec.

## Verify command

Read `verify.command` from `givn/config.yaml`. It MUST be a Gherkin/Cucumber
runner that executes the `.feature` files (see the executable-spec invariant
above). If it is still the `givn missing-testrunner` sentinel, stop and set
it (per the design artifact) before continuing.
