---
description: (Re-)write the Gherkin .feature spec for a givn change
---

Write (or re-write) the Gherkin spec for a givn change.

**Tip:** If you are starting a new change, use `/givn-propose` instead — it
writes the proposal, spec, design, and tasks in one pass.
Use this command to redo or extend the spec for an existing change.

**Input**: Optionally specify a change ID after `/givn-spec`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Get spec instructions

```sh
givn instructions specs --change <id>
```

Parse the output for:
- `output`: glob pattern for delta spec paths
  (typically `givn/changes/<id>/specs/<capability>/<capability>.feature`)
- `template`: structure to follow
- `instruction`: guidance on what to produce
- `context` / `rules`: constraints for you — **do not copy these into the file**
- `dependencies`: completed artifacts to read (proposal.md)

### 3. Read the proposal

Read `givn/changes/<id>/proposal.md`. The capabilities/features listed there
determine how many `.feature` files to create (one per capability).

Also check permanent specs at `givn/specs/` — existing scenarios for a
capability are the baseline; delta scenarios extend or modify them.

### 4. Write delta .feature files

For each capability, create:
`givn/changes/<id>/specs/<capability>/<capability>.feature`

**Required tags:**
- `@givn.delta` on the Feature line (marks this as a delta document)
- `@<capability>` on the Feature line (capability identifier, kebab-case)
- One of `@givn.added` / `@givn.modified` / `@givn.removed` on each Scenario
  (default is `@givn.added` if omitted)
- `@wip` on every Scenario (steps not yet implemented)

**Delta tag meanings:**
| Tag | Meaning |
|---|---|
| `@givn.added` | Append to permanent spec on archive |
| `@givn.modified` | Replace scenario by title on archive |
| `@givn.removed` | Delete scenario by title on archive |

**Rules:**
- Scenarios assert observable behaviour in domain language (Given/When/Then).
- No class names, function names, routes, DB schemas, or step mechanics.
- One scenario = one observable behaviour.
- `@givn.removed` scenarios: include exactly one placeholder step.
- All `@givn.*` tags are stripped automatically on archive.

### 5. Validate syntax

```sh
givn lint
```

- Exit 0 = clean syntax, `@wip` scenarios present (expected at this stage)
- Exit 1 = parse error (fix before proceeding)
- Exit 2 = `@wip` or PENDING (expected; does NOT indicate test failure)

### 6. Show next step

```sh
givn plan --json
```

Find this artifact (`specs`) in the sequence and report the next artifact id
as the next command to run (e.g. `/givn-design`, or whatever follows `specs`
in the configured manifest).

---

## Output

```
## Spec written: <id>

**Files created:**
- givn/changes/<id>/specs/<capability>/<capability>.feature  (N scenarios)

**Lint:** clean (N @wip scenarios — expected)

**Next:** Run `/<next-artifact-id>` to continue (from `givn plan --json`).
```

---

## Guardrails

- Keep all scenarios `@wip` — step definitions are not written yet.
- Never add implementation detail to the spec (belongs in design.md).
- Run `givn lint` and fix parse errors before finishing.
- One `.feature` file per capability. Do not put all scenarios in one file.
- `givn lint` exit 2 (@wip present) is expected and correct at this stage.
