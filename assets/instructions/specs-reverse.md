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

## Feature: description block

The free-text lines after the `Feature:` line must contain:

1. **The black-box boundary** — named explicitly:
   "The black-box boundary for this capability is: <CLI surface / public API
   / module boundary>".
2. **Suspected bugs** discovered during discovery, flagged as follow-up
   `@givn.modified` candidates. These are NOT addressed in this change — they
   are recorded here for visibility and addressed in a future prescriptive
   change.

No `proposal.md` is produced in the reverse flow — the Feature: description
block is where the boundary and suspected bugs live.

## Executable-spec invariant

The `.feature` file is the executable specification. The chosen
`verify.command` (configured in the first task of `tasks.md`) MUST execute
these `.feature` files via a Gherkin/Cucumber runner — never a parallel
hand-maintained test file. Step text is the contract; step definitions are
shared glue that the runner binds to the scenarios. If a scenario's steps
change, the test surface changes automatically.

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

**Bug handling:**
- If a scenario describes buggy behaviour (the code does something wrong, but
  that is what it actually does), the scenario captures the buggy AS-IS
  behaviour. The bug is flagged in the Feature: description block and in
  `review.md` as a follow-up `@givn.modified` candidate.
- Do NOT write a scenario that describes the "correct" behaviour if the code
  does something different. Characterize AS-IS.
