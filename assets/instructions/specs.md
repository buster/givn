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
- Tags go on the line **before** `Scenario:`, never inside the scenario body.
- `@givn.removed` scenarios include exactly one placeholder step so the parser
  accepts them; givn lint suppresses warnings for them.

<example caption="Correct tag placement">
@givn.delta @my-capability

Feature: My Capability

  @givn.added @wip
  Scenario: Observable outcome when something happens
    Given some context
    When an action occurs
    Then an observable result is visible
</example>

## Executable-spec invariant

The `.feature` file is the executable specification. The chosen
`verify.command` (see `design.md`) MUST execute these `.feature` files via a
Gherkin/Cucumber runner — never a parallel hand-maintained test file. Step
text is the contract; step definitions are shared glue that the runner binds
to the scenarios. If a scenario's steps change, the test surface changes
automatically.

## End-to-end smoke tests (`@e2e`)

Every capability MUST include at least one `@e2e` smoke-test scenario that
exercises the real interaction surface — browser, CLI, API, message queue,
or whatever the user actually touches. These are not unit-level or
in-process tests; they drive the system the way a real user or external
client would.

**Tag `@e2e`** on any scenario that is an end-to-end smoke test. The `@e2e`
tag goes alongside the delta tag (`@givn.added @e2e @wip`). These scenarios
run via `verify.e2e_command` (configured in `givn/config.yaml`), not
`verify.command`.

**What makes a scenario `@e2e`:**
- It interacts through the real external boundary: a browser (via Playwright,
  Selenium, etc.), a real CLI invocation, a real HTTP client, a real database
  connection — whatever the system exposes.
- It may require local test infrastructure: a running application server,
  a test database, fake containers (Testcontainers), stub services.
- It covers the critical happy path ("can a user actually do the thing?")
  and the most important failure mode.

**What an `@e2e` scenario is NOT:**
- Not a unit test wrapped in Gherkin.
- Not an in-process integration test.
- Not a duplicate of an existing non-e2e scenario with the same assertions —
  it must test something that only a real end-to-end interaction can verify
  (e.g. "the page renders", "the CLI prints output to a terminal",
  "the record appears in the database after the form submission").

**`@e2e` scenarios use separate step definitions.** The e2e step definitions
live in a project-specific location documented in `design.md`. They are
separate from the unit/integration step definitions because they use
different infrastructure (browser driver, HTTP client, test harness).

<example caption="Capability with both unit and e2e scenarios">
@givn.delta @user-registration

Feature: User Registration

  @givn.added @wip
  Scenario: Valid email and password creates an account
    Given a registration form
    When the user submits email "test@example.com" and a valid password
    Then an account is created for "test@example.com"

  @givn.added @e2e @wip
  Scenario: User registers through the web interface
    Given a browser on the registration page
    When the user fills in email "test@example.com" and a valid password
    And clicks the "Register" button
    Then the browser shows the welcome page
    And the user can log in with "test@example.com"
</example>

**Merge semantics:**
- `@givn.added`: appended to the permanent spec (error if title already exists).
- `@givn.modified`: replaces scenario by title (error if not found).
- `@givn.removed`: deletes scenario by title (error if not found).
- On archive, all `@givn.*` tags are stripped, leaving clean runnable Gherkin.
  The `@e2e` tag is preserved (it is not a `@givn.*` tag).
