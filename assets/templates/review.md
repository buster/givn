# Review: {{CHANGE_ID}}

## Coverage report

> Coverage is informational. It is recorded here as a hint, not a gate.
> A low number is a signal worth investigating; it is not an automatic failure.

**Coverage result:** <!-- e.g. 84% line, 76% branch — or "not measured" -->

Run your coverage tool and record the summary below:

```
<!-- paste coverage tool output here -->
```

### Coverage observations

<!-- For each uncovered region, note what it is and why. Use the buckets as a
     guide — but this is a record, not a checklist to complete before sign-off. -->

| Location | Lines | Observation | Bucket |
|---|---|---|---|
| | | | <!-- Dead code / Missing test / Hard to test --> |

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
- [ ] No `@wip` tags remain in the delta spec.
- [ ] The delta spec contains no implementation-layer detail.
- [ ] Coverage has been measured and recorded above (or marked "not measured" with reason).

**REVIEW: PASS** <!-- change to FAIL if any sign-off item is unchecked -->
