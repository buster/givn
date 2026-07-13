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
5. **Read every `@e2e` scenario's Then steps.** If a scenario's Then steps
   assert ONLY against a repository/database (no assertion on page content,
   HTTP response, redirect, or CLI output), it is a downgraded e2e scenario
   — an integration test wearing an `@e2e` tag. This is unfinished work,
   not a coverage gap, regardless of the runner reporting GREEN. Record
   each downgraded scenario found.
6. **For a browser-UI capability, read every `@e2e` step definition's
   implementation, not just its assertions.** If the step body sends an
   HTTP request directly, or calls `fetch()`/`XMLHttpRequest` from inside
   the page's JS context, instead of driving the browser (click/type/
   navigate) through a real driver, it is a downgraded e2e scenario even if
   its Then assertion checks page content — the interaction was never real.
   Record each occurrence found.
7. **Identify the exact file(s) `verify.e2e_command` actually invokes.**
   Read the command itself from `givn/config.yaml`, not just its name — a
   file merely containing "e2e" in its path is not evidence it is bound to
   that command. Then run `git status` and search the tree for any other
   `@e2e` step implementation for the same capability, including untracked
   or uncommitted files. If more than one implementation exists, read all
   of them. A weaker/downgraded implementation being reviewed while a
   stronger, real-interface implementation already exists elsewhere in the
   tree (committed or not) is itself a fabrication-audit finding — the
   review must use and credit the strongest one, not whichever one
   `verify.e2e_command` happens to point at if that is the wrong one.
8. **Check `@e2e` scope: one per happy-path action, not one per variant.**
   If a capability has more than one `@e2e` scenario, confirm each covers a
   genuinely distinct user-facing action (e.g. registration vs. login), not
   input variants/error cases/enum values of the same action. Excess `@e2e`
   scenarios covering variants should be converted to regular (non-`@e2e`)
   scenarios — flag this as a fabrication-audit finding (wasted e2e budget
   on the same underlying action, e2e coverage should be reserved for one
   distinct action).
9. Any finding from 1-8 is unfinished work, not a coverage gap. Reopen the
   task, run RED→GREEN→REFACTOR→COMMIT for real — driving the real
   interface per design.md's "Anticipated interface obstacles" fix, not by
   asserting on the repository instead. Do not write `REVIEW: PASS` while
   any finding remains.

Record the result even when clean: "0 empty step bodies found across N files."

## Coverage

Record the percentage (line/branch) and raw tool output as a fact, not a
gate. Coverage MUST include the Gherkin runner binary — a run that
instruments only unit tests undercounts and misleads classification below.

Redundant unit tests (behaviour already covered by a `.feature` scenario
through the black-box boundary) are removed — the `.feature` file is the
source of truth.

## Classification (three buckets — no fourth bucket)

1. Dead code (YAGNI/KISS) → delete.
2. Missing test coverage → add scenario, run RED→GREEN→REFACTOR→COMMIT.
3. Legitimately hard to test (rare) → concrete technical justification.

These three buckets are exhaustive. **Any classification outside them is
itself a fabrication-audit finding, not a valid outcome** — e.g. "acceptable
for this iteration scope," "sufficient for current scope," "future scope,"
"good enough for now," or any similar invented exception that waves off a
required check instead of fixing it or explicitly justifying it under
bucket 3. A missing or downgraded `@e2e` scenario always classifies as
bucket 2 (missing test coverage) — it can never be excused into a fourth,
undefined bucket. If you find yourself writing a sentence that excuses a
gap without naming one of the three buckets above, stop and reclassify it.

## E2e coverage

Every capability needs exactly one `@e2e` scenario per distinct user-facing
happy-path action, exercising the real interaction surface, with a primary
assertion on what that interface produced. For a browser-UI capability, the
step definitions must actually drive the browser (click/type/navigate) —
not an HTTP request or in-page `fetch()` standing in for browser
interaction. Missing, downgraded to repository-only assertions, or driven
via HTTP/fetch() instead of the real interface = classify as "missing
test," add or fix one — never as "acceptable for now" or any other
exception outside the three buckets above, even if a real-browser
implementation is reportedly not yet installed or not yet finished
elsewhere in the tree. Before concluding a real driver is unavailable,
search the tree yourself (including untracked files, see fabrication-audit
step 7) — do not take an absence at face value. More than one `@e2e`
scenario per action (one per variant/error case) = convert the excess to
regular scenarios. `verify.e2e_command` must pass alongside `verify.command`.
Confirm the local run command (design.md's Local Runnability section) still
starts the full stack, including digital twins, cleanly.

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
- Every capability has exactly one `@e2e` scenario per distinct happy-path
  action, with a real-interface primary assertion — none downgraded to
  repository-only checks, and none driven via HTTP/fetch() in place of real
  browser interaction on a browser-UI capability.
- No capability has more than one `@e2e` scenario per happy-path action
  (variants/error cases are regular scenarios, not additional `@e2e`).
- The local run command starts the full stack (including digital twins)
  cleanly, verified during this review.
- The exact file(s) `verify.e2e_command` invokes were identified and read;
  the tree (including untracked files) was searched for a second/parallel
  e2e implementation before accepting a weaker one.
- No finding was excused with a classification outside the three buckets
  (no "acceptable for now," "future scope," "sufficient for current scope,"
  or similar invented exception anywhere in this report).

## Result

End the file with `REVIEW: PASS` or `REVIEW: FAIL`. The archive gate reads
this marker.
