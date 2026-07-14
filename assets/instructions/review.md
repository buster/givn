# Instruction: review

Completeness-based coverage review. Goal: every uncovered region is
deliberately classified and resolved. No percentage threshold.

## Step 0 (mandatory, first): fabrication audit

A change can reach review with every task checked off, the runner exiting
0, and almost nothing actually implemented — if steps were left as
empty/no-op stubs. Mechanical check, before trusting any other artifact:

0. **Verify @e2e tag integrity.** Grep every delta `.feature` file in
   `givn/changes/<change-id>/specs/`. If any scenario's tags were changed
   from including `@e2e` to excluding it, record this as a
   fabrication-audit finding: `@e2e` coverage was planned and then removed
   to bypass the verify-e2e gate. The fix is to restore the `@e2e` tag and
   configure `verify.e2e_command`. Tag removal is independently detected by
   `givn check review` and `givn lint` through tasks.md cross-reference.

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
9. **Confirm `verify.e2e_command` is actually scoped to `@e2e`, not a copy
   of `verify.command`.** Read both values from `givn/config.yaml`
   literally. If they are identical strings, the "e2e run" re-runs the
   entire suite and proves nothing about e2e isolation specifically — this
   is always a fabrication-audit finding, never an acceptable
   configuration, regardless of whether both currently exit 0. Confirm by
   running each and comparing reported scenario counts: `verify.e2e_command`
   MUST report strictly fewer scenarios than `verify.command`, unless
   literally every in-scope scenario is tagged `@e2e` (state this
   explicitly if so). If the counts are equal without that justification,
   the tag filter is missing or broken — reopen as a fabrication finding.
10. **Diff the implementation against design.md's stated commands, file
    layout, and framework choices — do not just check they exist.**
    design.md names specific things: the exact `verify.e2e_command` value
    (including its tag-filter flag), exact step-definition file paths (e.g.
    separate files per capability), exact framework/driver. If the
    as-built project deviates from what design.md explicitly named — same
    step file used for both e2e and non-e2e when design.md named two, a
    different or unfiltered command than design.md specified, a different
    driver — and `design.md` was never updated to reflect a deliberate,
    re-reviewed change, this is not a coverage gap or an acceptable
    "practical" shortcut. It is an implementation that silently overrode an
    already-reviewed design decision. Record it as a fabrication-audit
    finding: the fix is to either (a) conform the implementation to
    design.md as originally reviewed, or (b) update design.md with the
    corrected decision and re-run design-review (and reassess the arc42
    chapter table if the change is structural) before the task can be
    considered done — never to silently keep the deviation.
11. **Interaction coverage verification** — cross-reference the spec's
    User Interaction Inventory against the design's Interaction Coverage
    Matrix and the actual `.feature` file + step definitions:
    a. Read the spec `.feature` file and extract the `# User Interaction
       Inventory:` comment block. List every entry.
    b. Read the design's `Interaction Coverage Matrix` table. Confirm
       every inventory entry appears as a row. Flag missing or extra rows
       as a finding — every interaction must be mapped, no unmapped
       interaction may exist.
    c. For each matrix row, confirm an `@e2e` scenario with a matching
       title exists in the `.feature` file. The scenario title need not be
       identical to the inventory description, but it must clearly cover
       the same interaction.
    d. For each matrix row, grep the e2e step definition file(s) for the
       promised driving mechanism. If the matrix says "Playwright: click
       button X", the step file must import/use Playwright (not `reqwest`,
       not raw HTTP). If the driving mechanism is an HTTP client for an
       API-only capability, confirm the step file uses that HTTP client.
       Mismatch = driving-mechanism finding: the implementation silently
       used a different driver than the reviewed design committed to.
    e. Record the full cross-reference as a table in the fabrication audit
       section of review.md.
12. Any finding from 1-11 is unfinished work, not a coverage gap. Reopen the
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
- `verify.e2e_command` is not identical to `verify.command`; its scenario
  count was proven strictly smaller (or "all scenarios are @e2e" was
  stated explicitly).
- Implementation (commands, file layout, framework/driver) matches what
  design.md explicitly named — any deviation was resolved by updating
  design.md and re-running design-review, never left as a silent shortcut.
- Interaction coverage verified: every spec inventory entry maps to a
  design matrix row, an existing `@e2e` scenario, and step definitions
  using the promised driving mechanism — no mismatches.
- No finding was excused with a classification outside the three buckets
  (no "acceptable for now," "future scope," "sufficient for current scope,"
  or similar invented exception anywhere in this report).

## Result

End the file with `REVIEW: PASS` or `REVIEW: FAIL`. The archive gate reads
this marker.
