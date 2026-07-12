# Instruction: design

Write the technical design (the HOW layer). Implementation detail belongs here.

## Scope

- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: modules/components affected, what is new. Any diagram
  (component, sequence, flow, tree) MUST be Mermaid (` ```mermaid ` fenced
  code block), never ASCII art. A plain directory/file-tree listing is not a
  diagram and may stay as plain text.
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

## Version freshness (mandatory)

Never write a specific version number for a language, runtime, framework,
library, database, browser/driver, or container image from memory. Training
data is a snapshot with an unknown cutoff — a remembered "current" version
can now be old, superseded, or end-of-life. Confidently wrong is worse than
explicitly unresolved. For every technology decision that names a version,
do ONE of:

- **State "latest" or the ecosystem's LTS explicitly**, as the resolution
  strategy rather than a guess at the number, e.g. "Node.js: latest LTS
  (resolve via `nvm install --lts` or nodejs.org at setup time)",
  "PostgreSQL: latest stable major (resolve via the official
  `postgres:<major>` tag on Docker Hub at setup time)". The tasks setup
  task then pins whatever version that resolves to at implementation time.
- **Look up a current version** (web search, the package registry, or — for
  an existing codebase — its own lockfile) and record the version AND how/
  when it was verified, e.g. "Playwright 1.49.x (checked
  npmjs.com/package/playwright — latest stable as of this design)". A
  version number with no lookup note is treated as an unverified guess.

Applies to every version-bearing decision in this document: language/
runtime, framework, test runner and Cucumber/Gherkin library, browser/
driver, digital twin container image tags, database. "Latest"/"LTS" is
always an acceptable answer; a bare specific number with no lookup and no
"latest/LTS" designation is not.

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

## Local runnability & digital twins (mandatory)

The system MUST be runnable and testable locally, in full, with one
command. "The browser had a cookie problem so I only checked the database"
is not an acceptable outcome — it is a design gap. If a scenario cannot be
driven through the real interface, that is a defect in this section to fix,
not a reason to test something weaker instead.

- **Local run command**: name the exact command that starts the entire
  system — application, database, queues, and every dependency — in one
  step (e.g. `docker-compose up`, a `Makefile` target, a devcontainer
  script). Document it here even though no `givn` command invokes it; it is
  read by a human (or the e2e setup task) to bring the stack up for manual
  or automated testing.
- **Isolated network**: every dependency runs in a docker-compose (or
  equivalent) network isolated from the host and from other projects — no
  shared dev database, no assumption that a service is "already running."
- **Digital twin per external/third-party dependency**: list every service
  the system talks to that is not part of this codebase (payment processor,
  email/SMS provider, third-party API, external auth provider). For each
  one, name its digital twin — a fake/stub/emulator that runs in the same
  docker-compose network, behaves deterministically, and requires no
  network access outside the isolated network. No scenario, `@e2e` or
  otherwise, may depend on a live third-party service. Examples: a
  Mailhog/Mailpit container for email, a local Stripe mock for payments, a
  WireMock/Prism container for a third-party REST API, an emulator image
  for a cloud service (DynamoDB Local, LocalStack, Firebase emulator).
- **Anticipated interface obstacles get a named fix, not a workaround.** If
  you can foresee a technical obstacle to driving the real interface (e.g.
  session cookies not persisting across Playwright navigation steps,
  redirects losing auth state, a websocket handshake that is hard to
  automate), the fix belongs here: name the concrete technical approach
  (e.g. "reuse one browser context across all steps in a scenario," "the
  e2e harness drives the same routes as the browser via a cookie-jar-aware
  HTTP client configured identically to the browser's session," "run the
  browser headed with a persistent user-data-dir in CI"). Do not defer this
  decision to tasks.md or to the implementer discovering it mid-scenario —
  a scenario blocked on an undesigned interface problem is a design gap.

## Rule: an `@e2e` scenario asserts through the real interface — not instead

An `@e2e` scenario exists to prove the system works from the outside: a
browser click, an HTTP request, a CLI invocation, a real login flow. Its
`Then` steps assert on what that real interface produces (page content, an
HTTP response, a redirect, a piece of UI state) as the PRIMARY assertion.
Checking repository/database state is allowed ONLY as an additional,
secondary assertion alongside a real-interface assertion — never as its
replacement.

**The real interface is whichever one an actual user or client uses — not
whichever one is easiest to automate.** If the system has a browser-based
UI, "the real interface" means the rendered browser: a real driver (e.g.
Playwright) clicking buttons, typing into fields, navigating, and reading
the rendered DOM. It does NOT mean an HTTP request to the page's backend
route, and it does NOT mean calling `fetch()`/`XMLHttpRequest` from inside
the page's own JavaScript context to hit an API endpoint directly — both are
integration tests of the API layer wearing an `@e2e` tag; neither proves a
user can actually use the UI. Conversely, if the system's real interface IS
an API (no UI consumes it, or the UI is out of scope for this capability),
then an HTTP request against that API genuinely is the real interface, and
using a real browser would be theater, not more rigorous. State explicitly
in this document which interface type this capability has (browser UI / CLI
/ API-only) and name the concrete driving mechanism that matches it — never
default to HTTP/fetch() for a browser-UI capability because it was easier to
get green.

A scenario whose `Then` steps assert only against the repository/database,
with no assertion on what the browser/API/CLI actually returned or
displayed, is not an e2e scenario. It is an integration test wearing an
`@e2e` tag, and it does not prove the feature works end-to-end. This is a
downgrade, and it happens most often when the real interface hits a
technical obstacle mid-implementation (a browser cookie/session problem, a
flaky selector, an auth redirect) and the path of least resistance is to
assert on the database instead and move on. That obstacle is a defect in
the Local Runnability & Digital Twins section above, or in this design's
assumptions about the interface — fix it there before writing the
scenario's steps. Never fix it by quietly weakening what the scenario proves.

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
- No version number from memory — "latest"/"LTS" or a looked-up, noted
  version only (see Version Freshness above).
- State the interface type (browser UI / CLI / API-only) for every
  capability with `@e2e` scenarios, and name the driving mechanism that
  actually matches it — never HTTP/fetch() for a browser-UI capability.
- Exactly one `@e2e` scenario per distinct happy-path action per
  capability — not one per variant, error case, or enum value.
