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

Record the inventory as a comment block at the very top of the `.feature`
file, before the Feature header. One entry per interaction:

```
# User Interaction Inventory:
# - <action description>
# - <action description>
```

Each normalized inventory entry MUST have exactly one matching `@e2e` scenario
in the same `.feature` file. If an entry has no corresponding scenario, the
spec is incomplete. Variants, error cases, and flag combinations of the same
user action do not create additional inventory entries or additional `@e2e`
scenarios. The inventory is exhaustive after this normalization.

## Semantic review and retrieval vocabulary

The deterministic gates remain the first line of defence; semantic retrieval
adds an evidence worklist after those structural checks rather than replacing
them.

The E5, BGE, and NLI paths consume one complete deterministic serialization of
the Gherkin context: Feature/Rule context and descriptions, inherited
Backgrounds, scenario descriptions, resolved Given/When/Then roles, tables,
docstrings, Scenario Outline templates, and concrete example rows. Paths, tags,
scores, and classifications remain metadata. Deterministic fingerprints remain
separate evidence. The permanent E5 index reuses an entry only when its stored
content hash matches the hash of this complete serialization; there is no
separate serializer-version escape hatch.

Before model construction or inference, the actual model tokenizer measures the
complete input without truncation or padding. E5 passages and BGE/NLI pairs use
the 512-token limit; pair counts include pair special tokens. Exactly 512 is
 valid and 513 is over. Do not make a scenario longer, repeat setup, or add
 irrelevant qualifiers to evade a semantic finding. Do not make scenarios longer,
 repeat setup, or add irrelevant qualifiers as score workarounds.

The normal Cargo installation includes `spec-retrieval`,
`spec-similarity-gate`, `cross-encoder-reranker`, and `nli-evidence`. A
feature-free build created with `--no-default-features` reports retrieval as
unavailable instead of silently passing a semantic gate.

When `spec-retrieval` is compiled:

- `givn spec index` builds or refreshes the permanent scenario index.
- `givn spec search` and `givn spec explore` provide advisory authoring
  results; they do not block a change.
- `givn check review --change <id>` generates the change-local
  `semantic-review.md` worklist after the structural gates and blocks until
  every admitted candidate is classified and remediated.

When `cross-encoder-reranker` is compiled, E5 remains the raw candidate
generator and the local BGE second stage produces explicit
`BGE_RECOMMENDED` and `BGE_FILTERED` evidence. `BGE_FILTERED` is a
non-blocking evidence status: it is recorded for auditability, does not receive
one of the Givn classifications, and does not force an author to rename or
inflate a scenario. A `BGE_RECOMMENDED` candidate still requires the normal
agent-authored classification and remediation. A pair over the BGE tokenizer
limit receives per-candidate `BGE_TOKEN_CAP` evidence with left, right, pair,
and limit token counts. It remains visible, unresolved, outside the filtered
collection, and blocking. A partial token-cap run remains `BGE_ENABLED`; only a
run in which no BGE pair can be scored is `BGE_UNAVAILABLE`. In the all-over-cap
case, the run reason is `token-cap` and the candidate-level `BGE_TOKEN_CAP`
objects remain visible. Missing artifacts and other no-score failures retain
the existing unavailable fallback rather than being presented as filtered
evidence. A feature-free binary is different and fails with the existing
`spec retrieval is unavailable` diagnostic.

When only part of the E5 raw set enters the bounded BGE pool, the remainder is
recorded as `BGE_UNSCORED`. It remains a blocking E5 candidate and must receive
the normal Givn classification; it is not `BGE_FILTERED`. A candidate whose
pair was inspected and exceeded the tokenizer limit is `BGE_TOKEN_CAP`, not
`BGE_UNSCORED`. Across reports and JSON, these entries use the same E5-rank and
endpoint ordering. A one-query Explore uses the same guard: it reports a
candidate-level cap state when a selected pair is over the limit and reports
`BGE_UNAVAILABLE` only when no selected pair can be scored.

When `nli-evidence` is compiled, NLI independently scores the same E5 candidate
pool rather than only BGE survivors. A scored candidate is
`NLI_RECOMMENDED` or `NLI_FILTERED` using the existing NLI rule. An over-cap pair
is `NLI_TOKEN_CAP`: visible, unresolved, outside `nli_filtered_candidates`, and
blocking. `NLI_UNAVAILABLE` is a run-level state only when no NLI pair can be
scored; an all-over-cap run uses reason `token-cap` and still emits per-
candidate `NLI_TOKEN_CAP` evidence. Combined recommendations intersect the
independent BGE and NLI recommendations. A token-cap, unscored, or unavailable
layer never silently disproves a candidate.

For every token-cap finding, tell the agent to shorten or split the scenarios
and rerun the relevant review. Do not use automatic chunking, automatic
scenario edits, or verbosity inflation as a workaround.

Sharpen the executable invariant and observable boundary facts rather than
inflating scenario prose to avoid a semantic finding.

Classify every admitted candidate as `DUPLIKAT`, `VARIANTE`, or
`VALID-BOUNDARY`, with a written rationale; `UNCERTAIN` blocks the review. A
`BGE_FILTERED` evidence entry is not an admitted candidate and is not a Givn
classification. There is no `UNRELATED` escape classification for admitted
candidates. If two scenarios appear unrelated but remain above the active
 similarity boundary and are admitted, sharpen the domain vocabulary and
 observable invariants, sharpen invariants and boundary facts, then regenerate
 the report. `givn spec review --change
<id>` remains advisory and does not create the blocking worklist. A build with
`spec-similarity-gate` additionally enforces the strict similarity gate.

Without `spec-retrieval`, the blocking semantic gate must report the explicit
`retrieval-unavailable` result rather than passing silently. Feature-free
guidance must not prohibit retrieval-capable builds from using the authoring
commands above.

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

**Real-interface driving rule:** for a browser-UI capability, drive the actual
browser through a real browser driver: click, type, navigate, and read the
rendered page. An HTTP request to a backend route, or `fetch()`/
`XMLHttpRequest` from page JavaScript, exercises an API rather than the UI and
is not a valid browser E2E substitute. For an API-only capability, a real HTTP
client is the correct interface.

**Interface obstacle rule:** an interface obstacle is a design gap, not a
reason to weaken the assertion or switch to a lower-level boundary. Apply the
concrete fix documented in `design.md` before implementing the scenario.

**Canonical scope: one E2E evidence scenario per distinct real user action.**
Each normalized inventory entry has exactly one matching `@e2e` scenario that
exercises that action through the real interface. A capability gets one
`@e2e` scenario for each distinct action a consumer performs end-to-end (for
example, submitting a registration form, clicking Add, or deleting an item).
Input variants, error cases, validation rules, enum values, and output-affecting
flag combinations of the same action are regular scenarios. Before adding a
second `@e2e` scenario, ask whether it is a genuinely different action or only
a variant. Under-covering and over-covering are both defects.

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
