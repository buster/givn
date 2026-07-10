# Instruction: design

Write the technical design for this change. This is the HOW layer — implementation
details are welcome and required here.

**Scope (what belongs in design):**
- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step definitions live.
  **One file per capability** (e.g. `CompanyOnboardingSteps`,
  `AuthenticationSteps`) — never a single file holding every step for the
  whole change. Name the exact file(s) and package/module path.
- Test runner: choose the runner that fits the tech stack (e.g. `cargo test
  --test features_runner`, `npm test`, `pytest`). Document the chosen
  command. If `verify.command` in `givn/config.yaml` is still the `givn
  missing-testrunner` sentinel, running it prints setup guidance — follow
  that guidance to configure a real Gherkin/Cucumber runner. The tasks
  artifact will include a setup task to install and configure it.
- **Strict-mode flag/config for the chosen runner** — the exact flag or
  configuration key that makes undefined/pending steps FAIL the build
  instead of silently passing or skipping (see "Strict mode" below). Name it
  explicitly; do not leave this implicit.
- **Single-scenario run command** — the exact invocation to run ONE named
  scenario (not the whole suite), e.g. `cucumber-js --name "<title>"`,
  `mvn test -Dcucumber.filter.name="<title>"`, `cargo test <name> --
  --exact`, `behave -n "<title>"`. Tasks will require running each scenario
  individually during RED and GREEN — the implementer must know this command
  without guessing.
- Any other technical decision the implementer needs before writing tasks.

## Strict mode (mandatory)

Most Cucumber/Gherkin runners, by default, treat an **undefined step** or a
**pending step** (one whose body signals "not implemented") as neither a
pass nor a hard failure — some report it as a separate "pending"/"skipped"
count and still exit 0. Some runners go further: an empty step method body
in a compiled language (e.g. Java) is not detectable as "pending" at all —
it simply runs, does nothing, and reports PASS. This is the exact defect
that produces a project where every task is checked off, the runner exits 0,
and nothing was actually implemented.

The design MUST specify how the chosen runner is put into **strict mode**,
where undefined/pending steps cause a non-zero exit:

| Runner | Strict-mode mechanism |
|---|---|
| `cucumber-rs` | `.fail_on_skipped()` on the `Cucumber` builder |
| `cucumber-js` | `--strict` flag (default `true` in v9+, but confirm and pin it explicitly in `cucumber.js`) |
| `cucumber-jvm` (CLI runner) | `--strict` flag |
| `cucumber-jvm` (JUnit Platform Suite) | No first-class "strict" flag — the discipline is enforced by NEVER leaving a step body empty (see tasks.md rules) plus a plugin/listener that fails the build on any `Result.Type.UNDEFINED` or `PENDING`. Document the plugin/listener class used. |
| `behave` (Python) | Pending steps must explicitly `raise`; `behave` does not have an empty-body "pass" pattern by convention — but a step decorated and left with a bare `pass` DOES report as passed. Document that every new step body starts with `raise NotImplementedError(...)` until real assertions are added. |
| `godog` (Go) | Return `godog.ErrPending` from unimplemented steps; godog treats this as a failure by default. |

If the chosen runner cannot fail on undefined/pending steps by
configuration alone (e.g. `cucumber-jvm` with the JUnit Platform Suite
runner), the design MUST state explicitly that step-body discipline (never
an empty body — see tasks.md) is the enforcement mechanism, and MUST name a
mechanical check (e.g. a grep-based lint script, or a review-time audit) that
detects empty/no-op step bodies before archive.

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
- The chosen runner + strict-mode configuration must be proven, not assumed
  — the tasks artifact's setup task will include a proof-of-strictness check
  that exercises this exact configuration.
