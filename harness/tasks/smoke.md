# Task: Givn Workflow Smoke

Create a small givn change named `smoke` for a greeting capability, then produce
minimal artifacts through the workflow:

1. Create the change with `givn new smoke` if no active change exists.
2. Write `proposal.md`: a concrete goal, scope, and success guarantee.
3. Write one `specs/*.feature` with a valid Given/When/Then scenario.
4. Write `design.md`: one decision with its rationale and alternatives.
5. Write `design-review.md` containing `DESIGN-REVIEW: PASS`.
6. Write `tasks.md` with one actionable, unchecked task.
7. Write `review.md` containing `REVIEW: PASS`.

Use one givn lifecycle step per iteration. Do not implement production code,
do not run the archive gate, and stop once those artifacts exist. This is a
harness smoke: it exercises the artifacts, the automated checks, the LLM judge,
and full-record capture, not a complete product change.
