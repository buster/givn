# Instruction: specs

Write Gherkin `.feature` files that serve as the executable specification for
this change. These files are the delta — they will be merged into the permanent
spec on archive.

**Rules (enforced by default):**
- Scenarios assert **observable behaviour** in domain language (Given/When/Then
  about inputs, outputs, state visible to a user/client). Never reference internal
  calls, class names, function names, specific code structures, HTTP routes,
  step-definition mechanics, or framework details.
- The only permitted implementation-flavoured content is an aspect that is
  *itself* an externally observable requirement.
- Tag the Feature `@givn.delta @<capability>`.
- Tag each Scenario `@givn.added` (default), `@givn.modified`, or `@givn.removed`.
- Tag `@wip` while step definitions are not yet implemented.
- `@givn.removed` scenarios include exactly one placeholder step so the parser
  accepts them; givn lint suppresses warnings for them.

## Executable-spec invariant

The `.feature` file is the executable specification. The chosen
`verify.command` (see `design.md`) MUST execute these `.feature` files via a
Gherkin/Cucumber runner — never a parallel hand-maintained test file. Step
text is the contract; step definitions are shared glue that the runner binds
to the scenarios. If a scenario's steps change, the test surface changes
automatically.

**Merge semantics:**
- `@givn.added`: appended to the permanent spec (error if title already exists).
- `@givn.modified`: replaces scenario by title (error if not found).
- `@givn.removed`: deletes scenario by title (error if not found).
- On archive, all `@givn.*` tags are stripped, leaving clean runnable Gherkin.
