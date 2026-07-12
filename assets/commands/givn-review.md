---
description: Complete the coverage review for a givn change — classify every gap, sign off, confirm GREEN
---

Complete the coverage review for a givn change.

**Input**: Optionally specify a change ID after `/givn-review`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Role: orchestrator

Delegates all review work to a **review subagent**: runs the test suite,
measures coverage, classifies every gap, resolves them, writes `review.md`.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Announce: "Using change: <id>". If `tasks` is not `"status": "done"`, warn
the user and confirm before proceeding.

### 2. Collect context

Read `tasks.md`, `specs/**/*.feature`, `design.md`, `verify.command` and
`verify.e2e_command` from `givn/config.yaml`.

```sh
givn instructions review --change <id>
```

Capture `output`, `template`, `instruction`, `rules`, `dependencies`.

### 3. Spawn a review subagent

Pass a self-contained prompt:

```
You are a review agent completing the coverage review for givn change '<id>'.

## Your objective
Produce a complete, signed-off review.md at givn/changes/<id>/review.md.
The final line MUST be REVIEW: PASS. Do not write it until all sign-off
conditions are met.

## Review instructions
<paste full output of: givn instructions review --change <id>>

## Verify commands
Unit/integration: <verify.command>
E2E smoke tests:   <verify.e2e_command>

## Steps

### 0. Fabrication audit (mandatory, first)
A change can reach review with every task checked off, the runner exiting
0, and almost nothing implemented — if steps were left as empty stubs.
Mechanical check, before trusting anything else:

1. Grep every step definition file for empty/trivial bodies (`{}`, bare
   `pass`/`return`, whitespace-only, no assertion reference). Record files
   scanned and findings (file:line).
2. For every checked `[x]` task, confirm a commit exists and `git show
   --stat <hash>` touches production source, not just spec/stub. Record
   any checked task with no commit or a spec-only commit.
3. For every component design.md said would be created, confirm it exists
   in the tree. Record anything promised but missing.
4. Confirm the setup task's proof-of-strictness evidence is present with
   non-zero exit. Missing/zero = the whole suite's GREEN is unverified.
5. Read every `@e2e` scenario's step definitions. If its Then steps assert
   ONLY against a repository/database (no assertion on page content, HTTP
   response, redirect, or CLI output), record it as a downgraded e2e
   scenario — an integration test wearing an `@e2e` tag. It does not prove
   the feature works end-to-end, regardless of the runner reporting GREEN.
6. For a browser-UI capability, read every `@e2e` step definition's
   implementation, not just its assertion. If it sends an HTTP request or
   calls `fetch()`/`XMLHttpRequest` from the page's JS context instead of
   driving the browser (click/type/navigate via a real driver), record it
   as a downgraded scenario — the interaction was never real.
7. Check `@e2e` scope: each capability should have one `@e2e` scenario per
   distinct happy-path action, not one per input variant/error case/enum
   value. Record any capability with more than one `@e2e` scenario covering
   the same action as an over-production finding — convert the excess to
   regular scenarios.
8. Confirm the local run command (design.md's Local Runnability section)
   still starts the full stack — including every digital twin — cleanly.
9. Any finding from 1-8 is unfinished implementation, not a coverage gap.
   Uncheck the task, drive it through RED→GREEN→REFACTOR→COMMIT for real —
   fixing the real-interface obstacle per design.md, never by leaving the
   repository-only assertion or HTTP/fetch() shortcut in place. Do not
   write REVIEW: PASS while any finding remains.

Paste the result into review.md's Fabrication Audit section even when clean.

### 1. Confirm all tests pass
Run <verify.command> and <verify.e2e_command>. All scenarios GREEN, no @wip.
Failures: diagnose infrastructure (fix plumbing only) or coverage gap
(classify below). Never change production code to force a pass.

### 2. Measure and record coverage
Run the coverage tool from design.md. Record percentage and raw output in
review.md — a fact, not a threshold gate.

### 3. Classify every gap
| Bucket | Meaning | Action |
|---|---|---|
| Dead code | YAGNI/KISS | Delete; re-run tests. |
| Missing test | Not covered by a scenario | Add scenario (@givn.added), RED/GREEN/REFACTOR/COMMIT. |
| Hard to test | Infra/I-O boundary | Concrete technical justification. |

### 4. Resolve each gap
Per the table above; hard-to-test gaps get a written justification, no code change.

### 5. Confirm e2e coverage
Every capability needs exactly one `@e2e` scenario per distinct happy-path
action, with a real-interface primary assertion driven through the actual
interface (real browser for a browser-UI capability — never HTTP/fetch()
standing in for it). Missing, downgraded, driven via HTTP/fetch() instead
of the real interface, or over-produced (one per variant) = classify as
"missing test"/"needs fixing" and add, fix, or convert accordingly.

### 6. Final GREEN confirmation
Re-run both runners. Must pass after all gap resolutions.

### 7. Write review.md
Coverage percentage + tool output, gap table, test suite output, sign-off
checklist, final line REVIEW: PASS.

### 8. Run the review hook
`givn check review --change <id>`. Fix failures before reporting back.

## Report back with
Fabrication audit result, coverage percentage, gaps classified by bucket,
final test result, path written, confirmation REVIEW: PASS is the final line.
```

### 4. Wait for the review subagent

Must finish the fabrication audit, all gap resolution, write `review.md`,
and run `givn check review` first. A reported blocker goes to the user.

### 4a. Orchestrator spot-check (do not skip)

Before accepting `REVIEW: PASS`, independently re-check a sample rather than
trusting the subagent's report:

1. Grep 2-3 step definition files yourself for empty bodies (or all if
   fewer than 5). Confirm any "0 findings" claim.
2. Confirm 2-3 checked-off tasks have commits touching production code, via
   `git show --stat`.
3. Read every `@e2e` scenario's step definitions yourself. Confirm each has
   a real-interface assertion, not repository-only, and — for a browser-UI
   capability — is actually driven through a real browser, not HTTP/fetch().
4. Confirm no capability has more than one `@e2e` scenario per happy-path
   action.
5. Any contradiction: reject `REVIEW: PASS`, re-spawn the subagent with the
   specific discrepancy named.

### 5. Show final status

```sh
givn status --change <id>
```

Display: "Run `/givn-archive` to archive this change."

---

## Output

```
## Review complete: <id>

**Fabrication audit:** CLEAN (or: N findings remediated — see review.md)

**Coverage:** <percentage>
**Coverage gaps classified:** N
  - Dead code removed: N
  - Missing tests added: N
  - Hard to test (justified): N

**Final test run:** GREEN

**Review written:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change.
```

---

## Guardrails

- The orchestrator delegates coverage tools and review.md writing.
- Fabrication audit (step 0) is mandatory, always first, never optional.
- Orchestrator runs its own spot-check (4a) — never trusts the subagent's
  audit at face value.
- Any fabrication finding is unfinished implementation, not a coverage gap
  — remediate with a real RED/GREEN/REFACTOR/COMMIT cycle, never waved through.
- Coverage percentage is a fact, not a threshold.
- `REVIEW: PASS` requires all gaps resolved AND a clean fabrication audit.
- New scenarios added during review go through the full loop before sign-off.
- `givn lint` is static only; GREEN comes from the runners.
- Every capability needs exactly one `@e2e` scenario per distinct
  happy-path action, with a real-interface primary assertion —
  repository-only assertions, and HTTP/fetch() standing in for real browser
  interaction on a browser-UI capability, never satisfy this. More than one
  `@e2e` scenario per action is over-production, not thoroughness.
- The local run command must start the full stack, digital twins included,
  cleanly — confirmed during this review, not assumed.
- Archive hard-blocks if `REVIEW: PASS` is absent or hooks fail.
