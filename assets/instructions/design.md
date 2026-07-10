# Instruction: design

Write the technical design for this change. This is the HOW layer — implementation
details are welcome and required here.

**Scope (what belongs in design):**
- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step definitions live.
- Test runner: choose the runner that fits the tech stack (e.g. `cargo test
  --test features_runner`, `npm test`, `pytest`). Document the chosen
  command. If `verify.command` in `givn/config.yaml` is still the `givn
  missing-testrunner` sentinel, running it prints setup guidance — follow
  that guidance to configure a real Gherkin/Cucumber runner. The tasks
  artifact will include a setup task to install and configure it.
- Any other technical decision the implementer needs before writing tasks.

## End-to-end smoke test infrastructure

Every capability includes `@e2e` smoke-test scenarios that exercise the real
interaction surface. The design MUST document:

- **E2E runner command**: the command that executes `@e2e`-tagged scenarios
  (e.g. `npm run test:e2e`, `behave --tags=@e2e`, a dedicated Cucumber
  runner binary that filters on `@e2e`). This becomes `verify.e2e_command`
  in `givn/config.yaml`.
- **E2E step definition location**: where the e2e-specific step definitions
  live (separate from unit/integration steps, because they use different
  infrastructure).
- **Test infrastructure**: what is needed to run e2e tests locally:
  - For browser tests: Playwright, Selenium, or equivalent. How the browser
    is launched (headless by default).
  - For CLI tests: how the built binary is invoked in a real terminal context.
  - For API tests: how the server is started, what port, how it is torn down.
  - For data: test database, seed data, Testcontainers, fake services.
- **Technology choices**: which e2e framework and why (Playwright for browser
  is the default recommendation unless there is a reason to choose otherwise).

The e2e runner must load `.feature` files from `givn/specs/**/*.feature` and
filter to `@e2e`-tagged scenarios only.

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
Document the exact runner invocation that does this (e.g.
`cargo test --test features_runner`, `behave givn/specs`,
`npx cucumber-js "givn/specs/**/*.feature"`). For Rust projects, `cucumber-rs`
is the canonical choice — add it as a dev-dependency and create a test
binary that calls `cucumber::run_and_exit(...).await` with the `givn/specs`
path.

**Rules:**
- Do not duplicate the scenario text from the spec. Reference scenarios by title.
- Do not change the WHAT — if you discover the proposal/spec is wrong, update
  those artifacts instead.
- Justify non-obvious decisions (why this approach over alternatives?).
