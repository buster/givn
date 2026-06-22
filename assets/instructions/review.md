# Instruction: review

Perform a completeness-based coverage review. The goal is not a percentage
threshold — the goal is that every uncovered region is deliberately classified
and resolved.

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

**Sign-off checklist:**
- All scenarios pass (test runner exits 0).
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

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
