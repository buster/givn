# Design: {{CHANGE_ID}}

## Overview

> How will we implement this change? Summarise the approach.

## Use-Case Traceability

> Owning use-case ID: <usecase-id or none>
> Ideation source: <topic-id or none>
> Confirmed Persona IDs: <ids or none>
>
> Map the use-case guarantees, rules, events, capability ownership, and Persona
> impact to technical decisions and E2E evidence. Actors remain distinct from
> Personas.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| | | |

## Technology Decisions

> What language, framework, libraries, or patterns will be used?
> Justify decisions made here (why this approach over alternatives?).
>
> **Version freshness (mandatory):** never write a version number from
> memory. For each versioned choice below, state "latest" / the ecosystem's
> LTS as the resolution strategy, OR record a looked-up version with how/
> when it was checked:
> | Technology | Version | Resolution |
> |---|---|---|
> | (e.g. Node.js) | (e.g. latest LTS) | (e.g. resolve via `nvm install --lts` at setup time) |
> | (e.g. Playwright) | (e.g. 1.49.x) | (e.g. checked npmjs.com/package/playwright at design time) |

## Step Definitions

> Location of step definition files — **one file per capability**, never one
> file for the whole change:
> | Capability | Step definition file |
> |---|---|
> | | |
>
> Test runner command (from `verify.command` in givn/config.yaml):
> ```
> {{VERIFY_COMMAND}}
> ```
>
> Single-scenario run command (exact invocation to run ONE named scenario,
> not the whole suite — used by every RED/GREEN check in tasks.md):
> ```
 > (e.g. cargo test <name> -- --exact, mvn test -Dcucumber.filter.name="<title>",
 >  cucumber-js --name "<title>", behave -n "<title>")
 > ```

## Internal Primitives

> Internal data structures and abstractions this change introduces or modifies.
> These are the "what lives inside" — not public API surface, but the internal
> building blocks the step definitions and production code share:
>
> | Primitive | Type | Purpose |
> |---|---|---|
> | (e.g. `ChangeState`) | `struct` | Holds per-scenario artifact statuses |
> | (e.g. `parse_tasks_progress`) | `fn` | Extracts completed/total counts from tasks.md |

## Strict Mode (mandatory)

> Undefined/pending steps must FAIL the runner, never silently pass or skip.
> Exact mechanism for the chosen runner:
> ```
> (e.g. cucumber-rs: .fail_on_skipped(); cucumber-js: --strict, pinned in
>  cucumber.js; cucumber-jvm CLI: --strict; cucumber-jvm JUnit Platform
>  Suite: no native flag — never leave a step body empty, plus
>  <plugin/listener> failing on UNDEFINED/PENDING; behave: every new step
>  starts with raise NotImplementedError(...))
> ```
>
> **Not-implemented stub for this language** (every step without a real
> assertion yet — never an empty body):
> ```
> (e.g. Java: throw new io.cucumber.java.PendingException("TODO");
>  Python: raise NotImplementedError("TODO");
>  JS/TS: return "pending"; or throw new Error("TODO");
>  Rust: unimplemented!() or todo!(); Go: return godog.ErrPending;
>  Ruby: pending("TODO"))
> ```

## Local Runnability & Digital Twins (mandatory)

> **Local run command** — the single command that starts the entire system
> (app + every dependency) for local testing:
> ```
> (e.g. docker-compose up, make dev-up, ./scripts/local-env.sh up)
> ```
>
> **Isolated network:** confirm every dependency runs in a docker-compose
> (or equivalent) network isolated from the host — no shared dev database,
> no service assumed to already be running.
>
> **Digital twin per external/third-party dependency** — one row per
> service this system talks to that is NOT part of this codebase:
> | External dependency | Digital twin (fake/stub/emulator) | Runs where |
> |---|---|---|
> | (e.g. email provider) | (e.g. Mailpit container) | (e.g. docker-compose service `mailpit`) |
> | (e.g. payment processor) | (e.g. local Stripe mock) | |
> | (e.g. third-party REST API) | (e.g. WireMock/Prism container) | |
>
> No scenario may depend on a live third-party service. If there are no
> external dependencies, state that explicitly — do not leave this blank.
>
> **Anticipated interface obstacles and their fix** — name any foreseeable
> technical obstacle to driving the real interface (browser session/cookie
> persistence across steps, auth redirects, websocket handshakes) and the
> concrete fix. A scenario blocked on an undesigned interface problem during
> implementation is a design gap, not an excuse to test something weaker:
> ```
> (e.g. "Browser context is created once per scenario and reused across all
>  steps so session cookies persist; the e2e harness never opens a second
>  browser context mid-scenario.")
> ```

## E2E Smoke Test Infrastructure

> **Interface type** (Web UI / CLI / HTTP API / Library / Message bus):
> ```
> (e.g. "Web UI — Playwright drives the rendered page." or
>  "HTTP API — an HTTP client is the public consumer interface.")
> ```
> **E2E runner command** (→ `verify.e2e_command`):
> ```
> (e.g. npm run test:e2e, behave --tags=@e2e, cargo test --test e2e_runner)
> ```
> **E2E step location** (separate from unit/integration, one file per capability):
>
> **Local test infrastructure:** browser driver (Playwright default,
> headless), CLI invocation, server start/stop/port, test DB/seed
> data/Testcontainers/fakes.
>
> **E2E framework choice and justification:**
>
> **Interaction Coverage Matrix** — one row per User Interaction Inventory
> entry, mapping to its matching `@e2e` scenario and driving mechanism:
> | Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
> |---|---|---|---|
> | (from spec's inventory) | | | |
>
> **Real-interface assertion rule:** every `@e2e` scenario's primary
> assertions are on what the real interface returned/displayed (page
> content, HTTP response, redirect, CLI output) — repository/database
> assertions are secondary and additional only, never a replacement.
>
> **Real-interface driving rule:** for a browser-UI capability, step
> definitions drive the actual browser (click/type/navigate via the driver
> named above) — never an HTTP request to the backend route, and never
> `fetch()`/`XMLHttpRequest` called from the page's own JS context. Either
> shortcut only exercises the API, not the UI. For an API-only capability, a
> real HTTP client genuinely is the real interface.
>
> **E2E scope:** read `givn instructions specs --change <id>` for the
> canonical one-E2E-per-distinct-action policy; do not redefine it here.

## Architecture Impact

> What existing modules/components are affected?
> Are any new modules introduced?

## Data Model Changes

> Any schema, struct, or database changes required?

## Open Technical Questions

> List unresolved HOW decisions before moving to tasks.
