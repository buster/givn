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

## Use-case traceability

Read the selected `usecase.md` and relevant ideation change seed. Add a table
mapping use-case guarantees, rules, events, capability ownership, and confirmed
Persona impact to technical decisions and E2E evidence. Keep Actors in Gherkin
distinct from Personas, which are stakeholder review lenses.

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

## Canonical E2E policy

Read `givn instructions specs --change <id>` before designing interaction
coverage. That instruction is the only normative source for the user
interaction inventory, one-E2E-per-action scope, real-interface assertions,
and flag-variant classification. Design-specific infrastructure and matrix
details belong here; the policy itself is not restated here.

## Default viewports (mandatory for a Web UI)

A Web UI must be usable at two default viewports: mobile in the Samsung S24
class (360x780 CSS pixels) and desktop in Full HD (1920x1080 CSS pixels). The
`## Visual Design Contract` may add viewports; it may not drop these two.

A style improvement that stays within the contract is not an error: it
updates the contract (tokens, primitives, or reference views) in the same
change. Record the improvement and the contract update together; never
silently keep a deviation and never reject a conforming improvement.

## E2E smoke test infrastructure

Every capability with `@e2e` scenarios MUST document:

- **E2E runner command**: the exact `verify.e2e_command` value, filtered to
  `@e2e` and distinct from `verify.command`.
- **E2E step definition location**: separate from unit/integration steps, one
  file per capability.
- **Local test infrastructure**: browser driver, CLI invocation, server
  startup/stop, port, test database/seed data, containers, and fake services
  required for local execution.
- **E2E framework choice and justification**: name the framework and why it
  matches the named real interface.
- **E2E strict-mode proof**: the exact configuration and independent proof
  that undefined or pending E2E steps fail.

These are design decisions, not a second interaction policy. Apply the
resolved specs instruction for the user-facing inventory and real-interface
fidelity.

## Black-Box-First policy

Prefer the real interface for every behavior. If an internal test is retained,
the design must answer: **which case this test covers that the E2E does not**?
If there is no concrete answer, use the E2E scenario instead. This policy
prevents weaker duplicate scenarios from surviving under a new title.

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
  (e.g. "reuse one browser context across all steps in a scenario," "run the
  browser headed with a persistent user-data-dir in CI," or, for an API-only
  capability, "use a cookie-jar-aware HTTP client against the public API"). Do not defer this
  decision to tasks.md or to the implementer discovering it mid-scenario —
  a scenario blocked on an undesigned interface problem is a design gap.

## Interaction Coverage Matrix (mandatory)

After reading the canonical specs policy, the design MUST contain an
Interaction Coverage Matrix that maps every normalized inventory entry to its
matching `@e2e` scenario and driving mechanism. One row per entry:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| (from the spec's inventory comment) | (the @e2e scenario title) | (Web UI / CLI / HTTP API / Library / Message bus) | (specific driver + action) |

**Every row must have a non-empty driving mechanism.** For a Web UI, the
driving mechanism MUST name a specific browser driver (Playwright, Selenium,
WebDriverIO) and describe what action it performs (click button X, fill
input Y, navigate to Z). An HTTP client is NOT a valid driving mechanism for
a Web UI — it cannot click buttons or read the DOM. If the interface type is
HTTP/REST API, an HTTP client IS the real interface and that is the correct
driving mechanism.

The matrix is proven at design time and verified at review time. If an
inventory entry has no matching row, the design is incomplete.

## Real-interface design rule

Apply the canonical specs policy for primary assertions, real-interface
fidelity, and normalized action scope. This design must still name the
concrete interface and driver for the capability and document any interface
obstacle; it must not introduce a second wording of the policy.

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
- State the interface type and driving mechanism for every capability with
  `@e2e` scenarios, applying the canonical policy from the specs instruction.
