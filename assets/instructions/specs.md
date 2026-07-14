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

## User Interaction Inventory (mandatory, before writing any scenario)

Enumerate every interaction a consumer performs against the system's black-box
boundary — before writing any scenario text. An interaction is a discrete
action that triggers an observable state change through the real interface.
What counts as "the real interface" depends on the system type:

| System type | Real interface | Inventory must include |
|---|---|---|
| **Web UI** | Rendered browser (via Playwright, Selenium) | Every button click, form submission, navigation link, SSE/WebSocket event that changes visible content. |
| **CLI / terminal** | Real subprocess invocation | Every subcommand, every flag combination that produces different output. |
| **HTTP/REST API** | Real HTTP client against the deployed server | Every endpoint, every side-effect-permitting operation. |
| **Library / SDK** | Public API surface called from real consumer code | Every public function or method a consumer invokes. |
| **Message/event bus** | Real producer/consumer against running services | Every message type consumed, every side-effect-producing message published. |

Record the inventory as a comment block at the very top of the `.feature`
file, before the Feature header. One entry per interaction:

```
# User Interaction Inventory:
# - <action description>
# - <action description>
```

Each inventory entry MUST have at least one matching `@e2e` scenario in the
same `.feature` file. If an inventory entry has no corresponding `@e2e`
scenario, the spec is incomplete — fill the gap before moving to design.
The inventory is the exhaustive list; there is no second-class interaction
that can skip e2e coverage.

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
- It covers the critical happy path: "can a consumer actually do the thing
  through the real interface?"

**Scope: one `@e2e` scenario per User Interaction Inventory entry — not one
per variant, not one total.** Every entry in the inventory must have exactly
one matching `@e2e` scenario that exercises that interaction through the
real interface. A capability gets one `@e2e` scenario for each distinct
interaction a consumer performs end-to-end (e.g. "submit registration form,"
"click 'Add' button," "delete an item"). It does NOT get an additional `@e2e`
scenario for every input variant, error case, validation rule, or enum/
dropdown value of the same interaction — those belong as regular (non-`@e2e`)
scenarios, which are cheaper to run and already prove the logic without
paying for a real browser/CLI/API round trip each time. Before adding a
second `@e2e` scenario for the same inventory entry, ask: "is this a
genuinely different interaction, or a variant of one already covered?" If
it is a variant, it is not a separate `@e2e` scenario. Under-covering (an
inventory entry with zero `@e2e` scenarios) and over-covering (one per input
variant) are both defects — the target is exactly one per inventory entry.

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

<example caption="Capability with inventory, unit scenarios, and one @e2e per inventory entry">
# User Interaction Inventory:
# - submit registration form on /register page
# - navigate to login page
# - submit login form on /login page

@givn.delta @user-registration

Feature: User Registration

  @givn.added @wip
  Scenario: Valid email and password creates an account
    Given a registration form
    When the user submits email "test@example.com" and a valid password
    Then an account is created for "test@example.com"

  @givn.added @wip
  Scenario: Registered user can log in again
    Given an account for "test@example.com"
    When the user submits email "test@example.com" and a valid password on the login page
    Then the dashboard is shown

  @givn.added @e2e @wip
  Scenario: Consumer submits the registration form
    Given a browser on the registration page
    When the user fills in email "test@example.com" and a valid password
    And clicks the "Register" button
    Then the browser shows the welcome page

  @givn.added @e2e @wip
  Scenario: Consumer navigates to login and signs in
    Given a browser on the welcome page
    When the user clicks the "Log in" link
    And fills in email "test@example.com" and the password used at registration
    And clicks the "Sign in" button
    Then the browser shows the dashboard
</example>

**Merge semantics:**
- `@givn.added`: appended to the permanent spec (error if title already exists).
- `@givn.modified`: replaces scenario by title (error if not found).
- `@givn.removed`: deletes scenario by title (error if not found).
- On archive, all `@givn.*` tags are stripped, leaving clean runnable Gherkin.
  The `@e2e` tag is preserved (it is not a `@givn.*` tag).
