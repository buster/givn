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

**Local environment check** — the local run command (design.md's Local
Runnability section) starts the full stack, including digital twins,
cleanly:

```
<!-- paste local-run command + confirmation of clean startup -->
```

**Result:** CLEAN / FINDINGS (if findings, list remediation taken — reopened
tasks, re-run RED/GREEN/REFACTOR/COMMIT, real-interface obstacle fixed per
design.md — before continuing this review).

---

## Coverage

**Coverage result:** <!-- e.g. 84% line, 76% branch — or "not measured" -->

```
<!-- paste coverage tool output here -->
```

## Coverage gap classification

Every uncovered region must be classified as exactly one of:
1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run RED/GREEN/REFACTOR.
3. **Legitimately hard to test** — rare; requires concrete technical justification.

Coverage is **complete** when every gap is classified and resolved.
There is no minimum percentage; a high number is a side effect, not the target.

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| | | | |

---

## Test suite result

Run `{{VERIFY_COMMAND}}` and paste the summary below.

```
<!-- paste runner output here -->
```

**Result:** PASS / FAIL

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

**REVIEW: PASS** <!-- change to FAIL if any sign-off item is unchecked -->
