# Review: {{CHANGE_ID}}

## Fabrication Audit (do this first)

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
<!-- paste grep/scan command and result, e.g.
"0 empty step bodies found across N step definition files"
or list each finding with file:line -->
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (not just spec/stub files):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| | | |

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| | | |

**Strict-mode proof** — confirmed present and passing in tasks.md setup task:

```
<!-- paste reference to the proof-of-strictness evidence in tasks.md, or
re-run it now and paste the output -->
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps
checked for a real-interface primary assertion (page content, HTTP
response, redirect, CLI output), not repository-only; for a browser-UI
capability, the step definitions checked for actually driving the browser
(click/type/navigate), not an HTTP request or in-page `fetch()`:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| | | | |

**E2e scope check** — each capability has one `@e2e` scenario per distinct
happy-path action, not one per variant/error case:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| | | | |

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes, and confirmation the tree (including untracked files, via `git
status`) was searched for any other/parallel e2e implementation of the same
capability that might be stronger than the one being reviewed:

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| | | | |

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
<!-- paste local-run command + confirmation of clean startup -->
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/config.yaml`; if identical, this is always a finding
(the "e2e run" is not isolating `@e2e` scenarios at all). Prove isolation
with actual scenario counts from running both:

| `verify.command` value | `verify.e2e_command` value | Identical? | Full-suite scenario count | E2e-only scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| | | | | | |

**Design.md conformance check** — implementation's actual commands, file
layout, and framework/driver choices diffed against what design.md
explicitly named. Any deviation implemented without first updating
design.md and re-running design-review is a finding, not an acceptable
shortcut:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| | | | | |

**Interaction coverage verification** — spec's User Interaction Inventory
cross-referenced against design's Interaction Coverage Matrix, `.feature`
file, and step definitions. Every inventory entry must have a matrix row,
a matching `@e2e` scenario, and step definitions that use the promised
driving mechanism:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| | | | | | |

**Result:** CLEAN / FINDINGS (if findings, list remediation taken — reopened
tasks, re-run RED/GREEN/REFACTOR/COMMIT, real-interface obstacle fixed per
design.md, or design.md corrected + design-review re-run — before
continuing this review).

---

## Coverage

**Coverage result:** <!-- paste line/branch percentages from coverage.command output. If coverage.command is still the sentinel (givn missing-coverage), leave this section empty — the review gate will block until measurement is configured. -->

```
<!-- paste coverage summary from coverage.command output here.

     The coverage.command is configured in givn/config.yaml. This tool
     captures stdout, so run it and paste the coverage summary below.
     Coverage must be measured — do not write "not measured" or any
     placeholder text. Classification without instrumented data is
     guessing, not review. -->

## Coverage gap classification

Every uncovered region must be classified as exactly one of:
1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run RED/GREEN/REFACTOR.
3. **Legitimately hard to test** — rare; requires concrete technical justification.

These three buckets are exhaustive — **no fourth bucket**. Do not excuse a
gap with invented language such as "acceptable for this iteration scope,"
"sufficient for current scope," or "future scope" — a missing or downgraded
`@e2e` scenario is always bucket 2 (missing test coverage).

Coverage is **complete** when every gap is classified and resolved.
There is no minimum percentage; a high number is a side effect, not the target.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| | | | |

---

## Test suite result

Run `{{VERIFY_COMMAND}}` and paste the summary below. Extract coverage data
from this output (if the runner is coverage-instrumented).

```
<!-- paste runner output + coverage summary here -->
```

**Result:** PASS / FAIL

**Coverage measured:** YES / NO (YES = verify.command is coverage-instrumented and output contained coverage data)

---

## Sign-off

- [ ] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [ ] All scenarios pass (`{{VERIFY_COMMAND}}` exits 0).
- [ ] Every coverage gap is classified and resolved (or justified).
- [ ] No `@wip` tags remain in the delta spec.
- [ ] The delta spec contains no implementation-layer detail.
- [ ] Local run command starts the full stack, including digital twins,
      cleanly.
- [ ] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [ ] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (or "all scenarios are @e2e" is stated).
- [ ] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; any deviation went through a design.md update
      + design-review re-run, not a silent shortcut.
- [ ] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [ ] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces; no
      production-code struct/fn/enum that the step definitions rely on is
      omitted (or the section states "none" if the change has no new
      primitives).
- [ ] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md
      was mapped to a real `docs/arc42/<slug>.md` file per
      `src/integrity_hook.rs:check_arc42_conformance`.
- [ ] No finding anywhere in this report is excused with a classification
      outside the three buckets (no "acceptable for now," "future scope,"
      "sufficient for current scope," or similar invented exception).

**REVIEW: PASS** <!-- change to FAIL if any sign-off item is unchecked -->
