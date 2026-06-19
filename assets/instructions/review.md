# Instruction: review

Perform a review of the completed change. The goal is a clean test suite and
an honest coverage record — not a coverage percentage threshold.

**Coverage is informational, not a gate.**
Run the coverage tool, record the number and the uncovered regions. Use the
three buckets to label each observation — but gap resolution is encouraged,
not required for sign-off. A low percentage is a hint worth noting; it does
not block the review from passing.

**Coverage buckets (for labelling observations):**
1. **Dead code** (YAGNI/KISS) — consider deleting it.
2. **Missing test coverage** — consider adding a scenario or step.
3. **Legitimately hard to test** — note the reason (e.g. OS signal handling,
   external hardware, I/O boundary).

**The real sign-off gate:**
- All scenarios pass (test runner exits 0).
- No `@wip` tags remain in the delta spec.
- The delta spec contains no implementation-layer detail.
- Coverage has been measured and recorded (or explicitly marked "not measured"
  with a reason).

**Review result:** End the file with `REVIEW: PASS` or `REVIEW: FAIL`.
The archive gate reads this marker.
