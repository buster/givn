# Instruction: review

Perform a completeness-based coverage review. The goal is not a percentage
threshold — the goal is that every uncovered region is deliberately classified
and resolved.

**Coverage classification (three buckets only):**
1. **Dead code** (YAGNI/KISS) → delete it now.
2. **Missing test coverage** → add step definitions, or add a scenario and run
   the full RED→GREEN→REFACTOR loop.
3. **Legitimately hard to test** — rare; requires concrete technical justification
   (e.g. OS signal handling, external hardware). Document the justification.

**Coverage is complete when:**
- Every gap is classified into exactly one of the three buckets.
- Every dead code region is deleted.
- Every missing-test gap has a new scenario + passing test, or a fix.
- Every hard-to-test gap has a written justification.

**Sign-off checklist:**
- All scenarios pass (test runner exits 0).
- Every coverage gap classified and resolved (or justified).
- No `@wip` tags remain in the delta spec.
- The delta spec contains no implementation-layer detail (no class names,
  function names, HTTP routes, etc.).

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
