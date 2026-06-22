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
  `verify.command` (e.g. `cargo test --test features_runner`, `behave
  givn/specs`, `npx cucumber-js "givn/specs/**/*.feature"`). If
  `verify.command` in `givn/config.yaml` is still the `givn missing-testrunner`
  sentinel, running it prints setup guidance — follow that guidance to
  configure a real Gherkin/Cucumber runner.
- **Step definition locations**: where Cucumber/Gherkin step definitions live.
  "Step definitions" always means the bindings the Gherkin runner uses to map
  `.feature` steps to code — never a parallel hand-maintained test file.

## Executable-spec invariant (enforced)

The chosen `verify.command` MUST execute the `.feature` files produced by
the `specs` artifact. A bare unit-test runner that does not consume Gherkin
`.feature` files (e.g. `cargo test` with only `#[test]` functions, bare
`pytest`) is NOT acceptable on its own — it must be wrapped in or replaced
by a Gherkin/Cucumber runner (e.g. `cucumber-rs` for Rust, `behave` for
Python, `cucumber-js` for Node).

"Step definitions" always means the bindings the Gherkin runner uses to map
`.feature` steps to code — never a parallel hand-maintained test file. The
`.feature` file is the sole source of scenario identity; if a scenario's
steps change, the test surface changes automatically.

The runner must load `.feature` files from `givn/specs/**/*.feature` (the
permanent spec) and, during a change, from the change's `specs/**/*.feature`.
Document the exact runner invocation that does this. For Rust projects,
`cucumber-rs` is the canonical choice — add it as a dev-dependency and
create a test binary that calls `cucumber::run_and_exit(...).await` with the
`givn/specs` path.
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
