---
description: (Re-)write the technical design for a givn change — HOW layer, technology decisions, architecture impact
---

Write (or re-write) the technical design for a givn change.

**Tip:** If you are starting a new change, use `/givn-propose` instead — it
writes the proposal, spec, design, and tasks in one pass.
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
- **Architecture impact** — which modules/components are affected; what is new.
- **Data model changes** — schema, structs, database changes if any.
- **Step definition locations** — where Cucumber/Gherkin step defs will live.
- **Runner command** — choose the test runner that fits the tech stack
  (e.g. `cargo test`, `npm test`, `pytest`). Document it in design.md.
  If `verify.command` in `givn/config.yaml` is still `"givn missing-testrunner"`,
  update it now to the chosen runner. The tasks artifact will include a
  setup task to install and configure the runner.
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
- Verify the file exists after writing.
