# Instruction: review

Perform a completeness-based coverage review. The goal is not a percentage
threshold — the goal is that every uncovered region is deliberately classified
and resolved.

**Step 0 (mandatory, before anything else): the fabrication audit.**

This is a mechanical check, not a judgment call. It exists because a project
can reach review with every task checked off, the runner exiting 0, and
still have almost nothing actually implemented — if step definitions were
left as empty/no-op stubs with `@wip` removed. Do this BEFORE trusting any
other artifact in this change:

1. **Grep every step definition file for empty or trivial bodies.** Look for:
   - Empty bodies: `{}`, `() {}`, a method whose entire body is whitespace.
   - Trivial no-ops: a bare `pass`, a bare `return`, a body containing only
     a comment.
   - A step body that never references any assertion library, mock, HTTP
     client, or domain object at all.
   Any match is a **fabricated step** — the scenario using it is not actually
   tested, regardless of what tasks.md or the runner claims.
2. **Cross-check tasks.md commits.** For every checked-off `[x]` scenario
   task, confirm a corresponding commit exists (`git log --oneline --grep`
   or by the recorded commit hash) AND that the commit's diff touches
   production source files, not only the `.feature` file or the step
   definition stub. A checked box with no commit, or a commit that touches
   nothing but the spec, is a fabricated task.
3. **Cross-check design.md's promised architecture against what exists.**
   For every class/service/controller/module design.md says will be created,
   confirm it exists in the source tree. List anything design.md promised
   that was never created — this is not a "hard to test" gap, it is
   incomplete implementation and must be resolved before REVIEW: PASS.
4. **Confirm the runner is strict** (re-run the proof-of-strictness check
   from the tasks setup task if it is not already recorded with output in
   tasks.md). If the runner is not strict, every "PASS" in this change is
   unverified — resolve this first.
5. If any fabricated step, missing commit, or missing promised component is
   found: this is NOT a coverage gap to classify — it is unfinished work.
   Reopen the corresponding task(s) in tasks.md (uncheck them), and drive
   them through the full RED→GREEN→REFACTOR→COMMIT loop for real before
   continuing the review. Do not write `REVIEW: PASS` while any exist.

Record the fabrication audit result (files scanned, findings, remediation)
in review.md even when the result is clean — "0 empty step bodies found
across N step definition files" is a required, checkable statement.

**First: record the coverage number.**
Run the coverage tool and note the overall percentage (line and/or branch).
This is a fact recorded in the artifact — not a gate. A low number is a signal
to investigate; investigation may reveal dead code to delete or tests to add.

**Coverage MUST include the Gherkin runner tests.** The coverage tool must be
configured to instrument every test binary that `verify.command` executes —
crucially including the Gherkin/Cucumber runner binary that executes the
`.feature` files under `givn/specs/`. A coverage run that instruments only
unit tests (and omits the `.feature` execution) is invalid: it undercounts
the real coverage and misleads the gap classification below. If the project's
coverage tool does not automatically pick up the Gherkin runner binary,
configure it to do so (most tools instrument all test binaries by default
when invoked over the whole test suite; verify this in the raw output).

**Redundant unit tests.** When a unit test exercises behaviour that is fully
covered by a `.feature` scenario (the scenario drives the same code path
through the black-box boundary), the unit test is redundant. The `.feature`
file is the source of truth. Remove the redundant unit test — do not keep a
parallel test surface. (Keep unit tests only for code paths that no `.feature`
scenario reaches.)

**Coverage classification (three buckets only):**
1. **Dead code** (YAGNI/KISS) → delete it now.
2. **Missing test coverage** → add step definitions, or add a scenario and run
   the full RED→GREEN→REFACTOR loop.
3. **Legitimately hard to test** — rare; requires concrete technical justification
   (e.g. OS signal handling, external hardware). Document the justification.

**Coverage is complete when:**
- The coverage percentage is recorded.
- Every gap is classified into exactly one of the three buckets.
- Every dead code region is deleted.
- Every missing-test gap has a new scenario + passing test, or a fix.
- Every hard-to-test gap has a written justification.

**E2e smoke test coverage:**
- Every capability MUST have at least one `@e2e` scenario that exercises the
  real interaction surface (browser, CLI, API). If a capability has no `@e2e`
  scenario, this is a coverage gap — classify it as "missing test" and add one.
- The e2e runner (`verify.e2e_command`) must pass alongside `verify.command`.
- E2e step definitions are separate from unit/integration step definitions.

**Sign-off checklist:**
- Fabrication audit complete: 0 empty/no-op step bodies found (or all found
  ones remediated with real assertions + production code + commits).
- Every checked-off task in tasks.md has a corresponding commit that touches
  production source, verified by inspection (not by trusting the checkbox).
- Every component design.md promised to create actually exists in the tree.
- The runner's strict-mode proof is present and passing (from tasks.md setup).
- All non-@e2e scenarios pass (`verify.command` exits 0).
- All `@e2e` scenarios pass (`verify.e2e_command` exits 0).
- Coverage was measured across all test binaries, including the Gherkin
  runner that executes the `.feature` files (not unit tests alone).
- Every coverage gap classified and resolved (or justified).
- Every dead code region is deleted.
- Every missing-test gap has a new scenario + passing test, or a fix.
- Every hard-to-test gap has a written justification.
- Redundant unit tests (covered by `.feature` scenarios) have been removed.
- No `@wip` tags remain in the delta spec.
- The delta spec contains no implementation-layer detail (no class names,
  function names, HTTP routes, etc.).
- Every capability has at least one `@e2e` smoke-test scenario.

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
