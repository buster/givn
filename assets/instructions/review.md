# Instruction: review

Completeness-based coverage review. Goal: every uncovered region is
deliberately classified and resolved. No percentage threshold.

## Step 0 (mandatory, first): fabrication audit

A change can reach review with every task checked off, the runner exiting
0, and almost nothing actually implemented — if steps were left as
empty/no-op stubs. Mechanical check, before trusting any other artifact:

1. Grep every step definition file for empty/trivial bodies: `{}`, `() {}`,
   whitespace-only, bare `pass`/`return`, or a body with no reference to any
   assertion library, mock, HTTP client, or domain object. Any match =
   fabricated step.
2. Cross-check every checked `[x]` task has a commit whose diff touches
   production source, not only the spec/stub. No commit or spec-only diff =
   fabricated task.
3. Cross-check every component design.md promised (class/service/
   controller/module) exists in the tree. List anything missing —
   incomplete implementation, not a "hard to test" gap.
4. Confirm the runner's strict-mode proof is present with output in
   tasks.md. Missing = every PASS in this change is unverified.
5. Any finding from 1-4 is unfinished work, not a coverage gap. Reopen the
   task, run RED→GREEN→REFACTOR→COMMIT for real. Do not write
   `REVIEW: PASS` while any finding remains.

Record the result even when clean: "0 empty step bodies found across N files."

## Coverage

Record the percentage (line/branch) and raw tool output as a fact, not a
gate. Coverage MUST include the Gherkin runner binary — a run that
instruments only unit tests undercounts and misleads classification below.

Redundant unit tests (behaviour already covered by a `.feature` scenario
through the black-box boundary) are removed — the `.feature` file is the
source of truth.

## Classification (three buckets)

1. Dead code (YAGNI/KISS) → delete.
2. Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.
3. Legitimately hard to test (rare) → concrete technical justification.

## E2e coverage

Every capability needs at least one `@e2e` scenario exercising the real
interaction surface. Missing = classify as "missing test," add one.
`verify.e2e_command` must pass alongside `verify.command`.

## Sign-off checklist

- Fabrication audit: clean (or all findings remediated).
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Strict-mode proof present and passing.
- `verify.command` and `verify.e2e_command` both exit 0.
- Coverage measured across all test binaries, including the Gherkin runner.
- Every coverage gap classified and resolved.
- Dead code deleted. Missing tests added. Hard-to-test gaps justified.
- Redundant unit tests removed.
- No `@wip` tags remain. No implementation-layer detail in the spec.
- Every capability has at least one `@e2e` scenario.

## Result

End the file with `REVIEW: PASS` or `REVIEW: FAIL`. The archive gate reads
this marker.
