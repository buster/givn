# Instruction: design

Write the technical design (the HOW layer). Implementation detail belongs here.

## Scope

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: modules/components affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations, **one file per capability** (e.g.
  `CompanyOnboardingSteps`) — never one file for the whole change. Name the
  exact file(s) and package/module path.
- Test runner: exact command (e.g. `cargo test --test features_runner`). It
  MUST execute the `.feature` files under `givn/specs/` and the change's
  `specs/` — a bare unit-test runner is not sufficient. If `verify.command`
  is still the `givn missing-testrunner` sentinel, follow its guidance.
- **Strict-mode config** (mandatory — see below).
- **Single-scenario run command**: the exact invocation to run ONE named
  scenario, e.g. `cucumber-js --name "<title>"`,
  `mvn test -Dcucumber.filter.name="<title>"`, `cargo test <name> -- --exact`,
  `behave -n "<title>"`.
- Any other technical decision the implementer needs before writing tasks.

## Strict mode (mandatory)

Most runners treat an undefined or pending step as neither pass nor hard
failure by default; some (notably Java/cucumber-jvm with an empty step
method body) report an outright PASS for a step that does nothing. This
produces a project where every task is checked off, the runner exits 0, and
nothing is implemented. Name the exact mechanism:

| Runner | Strict-mode mechanism |
|---|---|
| `cucumber-rs` | `.fail_on_skipped()` on the `Cucumber` builder |
| `cucumber-js` | `--strict` (pin explicitly in `cucumber.js`, do not rely on version defaults) |
| `cucumber-jvm` (CLI) | `--strict` |
| `cucumber-jvm` (JUnit Platform Suite) | No native flag — enforced by never leaving a step body empty, plus a plugin/listener that fails the build on UNDEFINED/PENDING. Name the plugin/listener class. |
| `behave` | Steps must explicitly `raise`; a bare `pass` reports as passed. Every new step body starts with `raise NotImplementedError(...)`. |
| `godog` | Return `godog.ErrPending` — fails by default. |

If the runner cannot fail on undefined/pending steps via config alone, state
that step-body discipline (never empty) is the enforcement mechanism, and
name a mechanical check (grep script or review-time audit) that catches
empty/no-op bodies before archive.

## E2E smoke test infrastructure

Every capability includes `@e2e` scenarios exercising the real interaction
surface. Document:

- **E2E runner command** (becomes `verify.e2e_command`), filtered to `@e2e`.
- **E2E step location**, one file per capability, separate from
  unit/integration steps.
- **Local test infrastructure**: browser driver (Playwright default,
  headless), CLI invocation, server start/stop/port, test database/seed
  data/Testcontainers/fakes.
- **Framework choice and justification.**
- **Strict-mode config for the e2e runner** — proven separately from the
  main runner.

## Executable-spec invariant

`verify.command` MUST execute the `.feature` files from `givn/specs/**` and
the change's `specs/**`. A bare unit-test runner alone is not acceptable —
wrap it in or replace it with a Gherkin runner (`cucumber-rs`, `behave`,
`cucumber-js`). Step definitions are the runner's bindings from `.feature`
steps to code — never a parallel hand-maintained test file.

## Rules

- Do not duplicate scenario text — reference by title.
- Do not change the WHAT here — if the proposal/spec is wrong, fix those artifacts.
- Justify non-obvious decisions only.
- The runner + strict-mode config must be proven, not assumed — the tasks
  setup task includes a proof-of-strictness check against this exact config.
