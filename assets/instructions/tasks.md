# Instruction: tasks

Write the implementation task breakdown as a strict scenario-by-scenario
RED→GREEN→REFACTOR→COMMIT loop.

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
  COMMIT:   One commit for RED+GREEN+REFACTOR, e.g.
              "feat(<capability>): <release note>". The message
              carries the release note as its message subject, and
              the scenario title goes in the message body.
              Record the hash. No commit, or a commit touching only the
              spec/stub = task not done.
```

## Rule: @e2e tags are immutable — never remove them

Once `@e2e` is placed on a delta scenario, it MUST NOT be removed. The
`verify-e2e` hook at review time and `givn lint --change <id>` both
independently detect tag removal by cross-referencing against tasks.md
content. If e2e is blocked by infrastructure, configure
`verify.e2e_command` — never delete the tag. Removing `@e2e` tags to bypass
a gate is a procedure violation.

## TDD loop — @e2e scenarios (after all non-@e2e are GREEN)

Same loop, e2e runner (`verify.e2e_command`), e2e step location, and e2e
infrastructure setup from design.md in GREEN. Before writing or reviewing E2E
steps, read the canonical policy with `givn instructions specs --change <id>`.
It owns action normalization, real-interface assertions, and driver fidelity;
tasks owns only the phase mechanics and evidence capture. Commit message:
`test(e2e): <release note>` — the message
carries the release note as its message subject, and
the scenario title goes in the message body.

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
Reviewers (see review.md fabrication audit) diff actual implementation
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

- `[ ]`/`[x]` checkboxes. One section per scenario: RED/GREEN/REFACTOR/COMMIT.
- Each RED/GREEN/REFACTOR line has a field for pasted runner output.
- Each scenario has a field for the commit hash.
- Non-@e2e scenarios first, @e2e last.
- Check `[x]` only when evidence and commit hash are filled in. Empty fields
  = incomplete.
- **tasks.md must always be up-to-date.** Check each scenario's box
  immediately after completing it — before starting the next scenario.
  An unchecked box means that scenario is not done. Never leave all
  boxes unchecked and batch-check them at the end. Using `sed` or any
  text-replacement tool to mass-check boxes without filling in evidence
  and commit hashes is prohibited.
- Done when: all boxes checked, every scenario has a commit, both
  `verify.command` and `verify.e2e_command` exit 0 on the full suite, and
  the e2e setup task's scenario-count proof (e2e count strictly less than
  full count, or an explicit "all scenarios are @e2e" statement) is pasted.
