---
description: (Re-)write the technical design for a givn change — HOW layer, technology decisions, architecture impact
---

Write (or re-write) the technical design for a givn change.

For starting a new change, use `/givn-propose` instead — it writes the
proposal, spec, design, and tasks in one pass.
Use this command to redo or extend the design for an existing change.

**Input**: Optionally specify a change ID after `/givn-design`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Get design instructions

```sh
givn instructions design --change <id>
```

Parse the output for:
- `output`: path to write the design (typically `givn/changes/<id>/design.md`)
- `template`: structure to follow
- `instruction`: guidance on what to produce
- `context` / `rules`: constraints for you — **do not copy these into the file**
- `dependencies`: completed artifacts to read (proposal.md, specs/)

### 3. Read the inputs

Read:
- `givn/changes/<id>/proposal.md` — WHAT and WHY
- `givn/changes/<id>/specs/<capability>/<capability>.feature` — observable behaviours
- Relevant existing code (architecture, patterns, integration points)

### 4. Write the design

Write `givn/changes/<id>/design.md` covering:

- **Technology decisions** — language, framework, libraries, patterns chosen and why.
  **Version freshness (mandatory):** never write a version number from
  memory — training data has an unknown cutoff and a remembered "current"
  version can now be outdated or end-of-life. For every versioned choice,
  either state "latest" or the ecosystem's LTS explicitly as the resolution
  strategy (e.g. "Node.js: latest LTS"), or look it up now (web search, the
  package registry, or an existing lockfile) and record the version plus
  how/when it was checked (e.g. "Playwright 1.49.x — checked
  npmjs.com/package/playwright"). A bare specific version number with
  neither a lookup note nor a "latest/LTS" designation is an unverified
  guess — do not write one.
- **Architecture impact** — which modules/components are affected; what is new.
- **Data model changes** — schema, structs, database changes if any.
- **Step definition locations** — where Cucumber/Gherkin step defs will live.
- **Runner command** — choose the test runner that fits the tech stack
  (e.g. `cargo test`, `npm test`, `pytest`). Document it in design.md.
  If `verify.command` in `givn/config.yaml` is still `"givn missing-testrunner"`,
  update it now to the chosen runner. The tasks artifact will include a
  setup task to install and configure the runner.
- **Local runnability & digital twins (mandatory)** — the system must be
  runnable and testable locally with one command:
  - Local run command that starts the entire system + every dependency
    (e.g. `docker-compose up`), in an isolated network.
  - A digital twin (fake/stub/emulator, running in the same isolated
    network) for every external/third-party dependency — email provider,
    payment processor, third-party API, cloud service. No scenario may
    depend on a live third-party service. If there are none, say so explicitly.
  - Any foreseeable obstacle to driving the real interface (browser
    session/cookie persistence, auth redirects, websocket handshakes) named
    with its concrete fix. "The browser had a cookie problem so we checked
    the database instead" is not an acceptable outcome — that obstacle
    belongs here, fixed, before tasks.md is written.
- **E2E smoke test infrastructure** — document the e2e setup:
  - E2E runner command → becomes `verify.e2e_command` in `givn/config.yaml`.
  - E2E step definition location (separate from unit/integration steps).
  - Test infrastructure needed locally (browser driver, test server, test DB,
    Testcontainers, fake services). How it starts/stops.
  - E2E framework choice and justification (Playwright is the default
    recommendation for browser-based systems).
  - **Real-interface assertion rule**: every `@e2e` scenario's primary
    assertion is on what the real interface returned/displayed. Repository/
    database checks are secondary and additional only, never a replacement.
  - **The real interface is whichever one a real user/client uses — not
    whichever is easiest to automate.** State explicitly which interface
    type this capability has (browser UI / CLI / API-only). For a
    browser-UI capability, name a real browser driver (Playwright default)
    that clicks/types/navigates through the rendered page — an HTTP request
    to the backend route, or an in-page `fetch()`/`XMLHttpRequest` call, is
    NOT the real interface for a browser UI; both only prove the API works,
    not that a user can use the UI.
- **Non-obvious choices** — justify them; obvious choices need no justification.

This is the HOW layer. Everything that was kept out of the proposal and spec
belongs here: class names, routes, DB schemas, function signatures,
framework details, step-definition mechanics.

### 5. Show next step

```sh
givn status --change <id> --json
```

Read `next_required.id` from the output and report it as the next step
(e.g. `/givn-tasks`, or whatever the manifest defines after `design`).

---

## Output

```
## Design written: <id>

**Design written:** givn/changes/<id>/design.md

### Key decisions
- <decision 1>
- <decision 2>

**Next:** Run `/<next-artifact-id>` to continue (from `givn status --change <id> --json` → `next_required.id`).
```

---

## Guardrails

- Keep implementation detail OUT of proposal.md and the .feature files — it
  belongs here in design.md.
- Read the proposal and spec before writing.
- Justify non-obvious technical choices; skip self-evident ones.
- Local runnability + digital twins are mandatory, not optional — a design
  with no answer for "how does someone run and test this locally, fully
  isolated" is incomplete. Do not accept a design that defers this to tasks.md.
- Every external/third-party dependency needs a named digital twin. No
  design may leave a scenario dependent on a live third-party service.
- Never write a version number from memory. Use "latest"/"LTS", or actually
  look one up (search, registry, existing lockfile) and note how it was
  checked. Reject a design with bare version numbers and no lookup note.
- Verify the file exists after writing.
