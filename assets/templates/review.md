# Review: {{CHANGE_ID}}

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

- [ ] All scenarios pass (`{{VERIFY_COMMAND}}` exits 0).
- [ ] Every coverage gap is classified and resolved (or justified).
- [ ] No `@wip` tags remain in the delta spec.
- [ ] The delta spec contains no implementation-layer detail.

**REVIEW: PASS** <!-- change to FAIL if any sign-off item is unchecked -->
