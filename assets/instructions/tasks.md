# Instruction: tasks

Write the implementation task breakdown as a strict scenario-by-scenario
RED→GREEN→REFACTOR→COMMIT loop.

## Setup task (first task, always)

- Install/configure the Gherkin runner from design.md. It MUST execute
  `.feature` files under `givn/specs/` and the change's `specs/` — a bare
  unit-test runner is not sufficient.
- Configure strict mode: undefined/pending steps FAIL the build, never
  silently pass or skip.
- Set `verify.command` in `givn/config.yaml` to the exact runner invocation.
- Create the runner test binary and step-definition skeleton, **one file per
  capability** (e.g. `CompanyOnboardingSteps`). Never one file for everything.
- Run the proof-of-strictness check (below). Do not proceed until it passes.

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

## TDD loop — non-@e2e scenarios (in .feature order)

```
For each scenario:
  RED:      Remove @wip from THIS scenario only. Unimplemented steps use the
              stub above. Run the runner TARGETING THIS SCENARIO ONLY (name,
              line, or tag — never infer from whole-suite output). PASTE the
              output. MUST exit non-zero. Exit 0 = runner not strict or a
              step is empty — hard stop, fix before continuing.
  GREEN:    Replace stubs with real assertions. Write minimum production
              code. LIST every production file created/modified — an empty
              list on a non-reuse scenario means investigate, not proceed.
              Compiled/typed languages: run the compile step first, confirm
              it succeeds. Run the runner, targeting this scenario only.
              PASTE the output. MUST exit zero.
  REFACTOR: Clean up, no behaviour change. Re-run targeting this scenario.
              PASTE the output. MUST still exit zero.
  COMMIT:   One commit for RED+GREEN+REFACTOR. Message references the
              scenario title verbatim, e.g. "feat(<capability>): <title>".
              Record the hash. No commit, or a commit touching only the
              spec/stub = task not done.
```

## TDD loop — @e2e scenarios (after all non-@e2e are GREEN)

Same loop, e2e runner (`verify.e2e_command`), e2e step location, e2e
infrastructure setup (browser/server/DB) in GREEN. Commit message:
`test(e2e): <title>`.

## E2e setup task

Before the first `@e2e` scenario: install the e2e framework from design.md,
create e2e step skeletons (one file per capability, separate location),
configure and prove strict mode for the e2e runner (same procedure as
above), set `verify.e2e_command`, document local environment start/stop.

## Distinctions

- `givn lint`: static check, no tests.
- `verify.command`: confirms GREEN for non-@e2e — only once strict mode is proven.
- `verify.e2e_command`: confirms GREEN for @e2e — only once strict mode is proven.
- Never conflate the three.

## Tasks file format

- `[ ]`/`[x]` checkboxes. One section per scenario: RED/GREEN/REFACTOR/COMMIT.
- Each RED/GREEN/REFACTOR line has a field for pasted runner output.
- Each scenario has a field for the commit hash.
- Non-@e2e scenarios first, @e2e last.
- Check `[x]` only when evidence and commit hash are filled in. Empty fields
  = incomplete.
- Done when: all boxes checked, every scenario has a commit, both
  `verify.command` and `verify.e2e_command` exit 0 on the full suite.
