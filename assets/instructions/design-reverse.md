# Instruction: design-reverse

Write the technical design for characterizing the capability. This is the HOW
layer — but the "how" is about **test strategy and seams**, not about building
new features.

**Scope (what belongs in design-reverse):**
- **Black-box boundary**: the surface through which the capability will be
  exercised (CLI invocations, public API calls, module-level function calls).
  Named explicitly, with examples.
- **Seam strategy**: how the black box will be exercised in tests. If the
  current code is hard to test (tight coupling, hidden I/O, no public
  interface), document the **minimum testability refactor** needed to create a
  seam. This refactor MUST preserve behaviour exactly.
- **Testability refactor constraints** (strictly enforced):
  - Allowed: extract interface, dependency injection, make a function/method
    public, add a test-only constructor or builder, extract a pure function
    from a side-effectful one.
  - NOT allowed: rename, deduplicate, simplify logic, restructure for clarity,
    fix bugs, change error messages, change return types, change control flow.
  - Every refactor must be individually justifiable: "This seam enables
    scenario <title> by allowing <test input>".
- **Test harness**: choose the test runner and framework. Document the chosen
  `verify.command` (e.g. `cargo test`, `npm test`, `pytest`). If
  `verify.command` in `givn/config.yaml` is still `"givn missing-testrunner"`,
  update it.
- **Step definition locations**: where Cucumber/Gherkin step definitions (or
  test functions) will live.
- **Provisional behaviour note**: the inferred behaviour in the specs is
  provisional. If characterization proves a spec wrong, design.md is updated
  alongside the spec correction (the design's seam strategy may need to adjust
  to the real behaviour).

**Rules:**
- Do not duplicate the scenario text from the spec. Reference scenarios by title.
- Do not change the WHAT — if you discover the proposal/spec is wrong, update
  those artifacts instead.
- Justify non-obvious decisions (why this seam approach over alternatives?).
- Every testability refactor must have a behaviour-preservation guarantee:
  characterization tests written before the refactor must still pass after.
