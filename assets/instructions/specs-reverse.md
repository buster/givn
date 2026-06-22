# Instruction: specs-reverse

Write Gherkin `.feature` files that **characterize** the existing behaviour of
the capability. These are descriptive specs — they assert what the system *does*
today, not what it *should* do.

**Stance: descriptive, not prescriptive.**
- Scenarios assert **observed behaviour** in domain language (Given/When/Then
  about inputs, outputs, state visible to a user/client). The behaviour is
  probed from the real code, not imagined from requirements.
- Never reference internal calls, class names, function names, specific code
  structures, HTTP routes, step-definition mechanics, or framework details.
- The only permitted implementation-flavoured content is an aspect that is
  *itself* an externally observable requirement.

**Tags (same as normal mode):**
- Tag the Feature `@givn.delta @<capability>`.
- Tag each Scenario `@givn.added` (all scenarios are added — the permanent spec
  is empty for this capability).
- Tag `@wip` while step definitions are not yet characterized.
- `@givn.removed` and `@givn.modified` are NOT used in reverse-engineering
  changes (there is no existing permanent spec to modify or remove from).

**Merge semantics:**
- All scenarios are `@givn.added` against an empty permanent spec. On archive,
  the merge creates the initial `givn/specs/<capability>/<capability>.feature`
  baseline.
- `@givn.*` tags are stripped on archive, leaving clean runnable Gherkin.

**Provisional behaviour — the characterization loop:**
- The behaviour described in these scenarios was **inferred** from reading the
  code and documentation. It is provisional.
- During characterization (`/givn-characterize`), each scenario's step
  definitions are written to match the **actual** behaviour. If the test fails
  (the spec does not match reality), the spec is corrected to match AS-IS
  behaviour — never the production code.
- See `givn-characterize` for the full diagnostic loop: infrastructure failure
  vs spec/implementation mismatch.

**Bug handling:**
- If a scenario describes buggy behaviour (the code does something wrong, but
  that is what it actually does), the scenario captures the buggy AS-IS
  behaviour. The bug is flagged in `proposal.md` (Out of Scope) and in
  `review.md` as a follow-up `@givn.modified` candidate.
- Do NOT write a scenario that describes the "correct" behaviour if the code
  does something different. Characterize AS-IS.
