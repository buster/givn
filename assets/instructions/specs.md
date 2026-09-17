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

This section is the canonical E2E and interaction-inventory policy. Other
artifact instructions, skills, and slash commands reference it and must not
define competing wording.

Enumerate every interaction a consumer performs against the system's black-box
boundary — before writing any scenario text. An interaction is a discrete
action that triggers an observable state change through the real interface.
What counts as "the real interface" depends on the system type:

| System type | Real interface | Inventory must include |
|---|---|---|
| **Web UI** | Rendered browser (via Playwright, Selenium) | Every button click, form submission, navigation link, SSE/WebSocket event that changes visible content. |
| **CLI / terminal** | Real subprocess invocation | Every distinct user action; output-affecting flags are recorded as variants unless they change the action or side effect. |
| **HTTP/REST API** | Real HTTP client against the deployed server | Every endpoint, every side-effect-permitting operation. |
| **Library / SDK** | Public API surface called from real consumer code | Every public function or method a consumer invokes. |
| **Message/event bus** | Real producer/consumer against running services | Every message type consumed, every side-effect-producing message published. |

Record the inventory in the owning use case's `usecase.md`, under the
`## Interactions` capability-aware table. Every row names the capability, the
consumer action, and exactly one matching E2E scenario:

```
## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| <capability-id> | <action description> | <scenario title> |
```

The use-case document is the durable home of the inventory: it is never merged
into a `.feature` file, so it survives the archive merge. When authoring a
delta for an existing capability, read the owning use case's `usecase.md`; when
the change introduces a `NEW in <usecase-id>` capability (per the proposal's
Capability Routing table), create or update that use-case document as part of
the change. A reusable fragment keeps its own document and its interactions
are not copied into every including use case.

### Capability placement

Before recording a `NEW in <usecase-id>` capability, apply the placement test
from `docs/use-case-model.md`: which actor goal does completing this capability
advance? Channel is never an answer — a browser or CLI variant of an action
lives under the action's goal. Capability names are goal-shaped, never
component- or channel-shaped (no server, backend, store, service, panel,
browser, or CLI names).

Each normalized inventory entry MUST have exactly one matching `@e2e` scenario
in the capability's `.feature` file. If an entry has no corresponding scenario, the
spec is incomplete. Variants, error cases, and flag combinations of the same
user action do not create additional inventory entries or additional `@e2e`
scenarios. The inventory is exhaustive after this normalization.

A `Scenario Outline` tagged `@e2e` counts as the e2e evidence for exactly ONE
inventory entry: the tag marks that the outline as a whole proves the
interaction; its Examples rows are input variation, not additional distinct
interactions. Tagging one Outline with many Examples rows never covers many
inventory entries. `givn lint` compares the capability-aware inventory with the
actual `@e2e` titles and reports missing or unlisted mappings per use case.

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
  Selenium, etc.), a real CLI invocation, a real HTTP client, a public library
  API, or a real message producer/consumer — whatever the system exposes.
- It may require local test infrastructure: a running application server,
  a test database, fake containers (Testcontainers), stub services.
- It covers the critical happy path: "can a consumer actually do the thing
  through the real interface?"

**Primary assertion rule:** each `@e2e` scenario's primary assertion is on what
the named real interface returned, displayed, acknowledged, or made visible.
Repository/database checks are secondary evidence and are never the sole
assertion when the system exposes a higher-level consumer interface. Database
state is a valid primary boundary only when the database itself is the public
interface used by the consumer.

**Web UI classification (fail-closed):** a capability whose interface renders HTML, CSS, or JavaScript in a browser is a Web UI — even when it needs only a page request, exposes no controls, and keeps no server-side state.
For a Web UI the browser driver rule below applies, and an HTTP client is not a valid driver.
The HTTP client is the real interface only for a pure REST/HTTP API.

**Real-interface driving rule:** for a browser-UI capability, drive the actual
browser through a real browser driver: click, type, navigate, and read the
rendered page. An HTTP request to a backend route, or `fetch()`/
`XMLHttpRequest` from page JavaScript, exercises an API rather than the UI and
is not a valid browser E2E substitute. For an API-only capability, a real HTTP
client is the correct interface.

**No silent deferral:** a mandatory check is never deferred because of a project constraint.
When a constraint conflicts with a mandatory check (for example "no dependencies" versus a browser driver), the conflict is put to the operator as an operator question; only the operator's recorded decision may allow the deferral, and a review without that record fails the review.

**Interface obstacle rule:** an interface obstacle is a design gap, not a
reason to weaken the assertion or switch to a lower-level boundary. Apply the
concrete fix documented in `design.md` before implementing the scenario.

**Rendered state:** at least one `@e2e` scenario of a Web UI asserts a value the browser produced by rendering and observes that the rendered state changes between two observations — a clock that advances, a counter that increments, or content that appears after a request.
A successful page request and a static heading are not rendered-state evidence.

**Canonical scope: one E2E evidence scenario per distinct real user action.**
Each normalized inventory entry has exactly one matching `@e2e` scenario that
exercises that action through the real interface. A capability gets one
`@e2e` scenario for each distinct action a consumer performs end-to-end (for
example, submitting a registration form, clicking Add, or deleting an item).
Input variants, error cases, validation rules, enum values, and output-affecting
flag combinations of the same action are regular scenarios. Before adding a
second `@e2e` scenario, ask whether it is a genuinely different action or only
a variant. Under-covering and over-covering are both defects.

**Non-e2e scenarios must differ from their action's `@e2e` scenario.** Every
normalized inventory action has exactly one `@e2e` scenario. A regular
(non-e2e) scenario must assert an invariant distinct from that `@e2e`
scenario — a variant rule, edge precondition, or genuinely different end
state. A regular scenario that only re-asserts the `@e2e` outcome (the same
observable result asserted against the module instead of through the browser)
is a duplicate: the layer is a driver, not an invariant. Semantic review flags
it — distinguish it or delete it.

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

## Visual evidence and baseline (Web UI and terminal)

A visual interface is one whose evidence is what the consumer sees or reads.
For a Web UI, every `@e2e` scenario commits one final-state screenshot per
viewport named by the design's `## Visual Design Contract`. The default
viewports are mobile in the Samsung S24 class (360x780 CSS pixels) and
desktop in Full HD (1920x1080 CSS pixels). Screenshots are captured after the
final assertion with animations frozen and fonts and network settled;
intermediate captures are debug artifacts, not review evidence.

When the design's `## Visual Design Contract` declares UI states, every `@e2e` scenario commits one screenshot per viewport for the state it traverses: the final state as `<viewport>.png`, and each additional declared state as `<viewport>--<state>.png` (for example the empty, loading, or error state).

For a Web UI, every `@e2e` scenario asserts the visual baseline at every
interaction, in addition to its primary assertion:

- every control has a visible label;
- no internal identifier or state name appears in visible text;
- the page has a real title and heading;
- no visible content is duplicated;
- the error state has a heading and a way back;
- the interface uses the configured language;
- `prefers-reduced-motion` is honored.

These are ordinary scenario assertions, not a separate tool. They run at every
assertion of a visual capability, so a mid-flow defect cannot hide behind a
clean final state.

For a terminal interface, each `@e2e` scenario commits a captured transcript
instead of screenshots, and the same baseline applies to the text the consumer
reads. API-only capabilities are exempt from visual evidence: the HTTP
response is their interface, and its assertions already cover it.

The design's `## Visual Design Contract` names the viewports and the evidence
kind. This policy owns the evidence rule, not the design's format: see
`givn instructions design` for the contract and capture mechanics.

<example caption="Use-case inventory (in usecase.md), unit scenarios, and one @e2e per inventory entry">
<!-- givn/specs/<usecase-id>/usecase.md — ## Interactions section -->
| Capability | Consumer action | E2E scenario |
|---|---|---|
| user-registration | submit registration form | Consumer submits the registration form |
| user-registration | navigate to login | Consumer navigates to login and signs in |

<!-- givn/changes/<id>/specs/<usecase-id>/user-registration.feature -->
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
