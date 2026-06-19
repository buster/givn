---
description: Plan a givn change end-to-end — runs all planning artifacts in the order the manifest defines
---

Plan a new givn change from start to finish.

**Input**: The argument after `/givn-propose` is the change ID (kebab-case), OR a
plain description of what to build. If nothing is provided, ask.

This command discovers the artifact sequence from `givn` at runtime — it does not
hardcode which steps exist or in what order. If the project has customised its
manifest (added a risk-assessment step, removed review, etc.), this command
automatically follows the configured sequence.

---

## Role: orchestrator

This command is an **orchestrator**. It creates the change scaffold, collects
context, and then delegates all planning artifact production to a **thinking
architect subagent**. The architect subagent produces the complete plan
(proposal through tasks) in one pass. The orchestrator does not write artifacts
directly.

---

## Steps

### 1. Resolve the change ID

If the user gave a description rather than a kebab-case ID, derive one
(e.g. "add user auth" → `add-user-auth`). Confirm with the user before
proceeding only if the name is genuinely ambiguous.

### 2. Create the change

```sh
givn new <id>
```

### 3. Discover the artifact sequence

```sh
givn status --change <id> --json
```

Parse the JSON:
- `artifacts`: array in execution order, each with `id`, `status`, `optional`, `gates_archive`
- `next_required`: the first artifact to write
- `verify_command`: the configured test runner

### 4. Collect instructions for every planning artifact

For each artifact in the sequence (up to and including `tasks`, excluding `review`):

```sh
givn instructions <artifact-id> --change <id>
```

Capture the full output for each.

### 5. Spawn a thinking architect subagent

Use the **Task tool** to spawn a thinking architect subagent (fall back to
describing the pattern in prose if the Task tool is not available to you).

The architect subagent is a **deep-thinking planning agent** — give it room to
reason carefully. It must not rush to produce output; it should think through
requirements, constraints, and decisions before writing each artifact.

Pass a self-contained prompt:

```
You are a thinking architect producing the complete plan for a givn change.

## Change ID
<id>

## What to build
<user's original description or request>

## Artifact sequence (from givn status --json)
<paste the artifacts array>

## Your task
Produce every planning artifact in order — stopping before `review`.
Do not produce the review artifact; that belongs to a separate step after
implementation is complete.

For each artifact:
1. Think carefully about the requirements, constraints, and implications.
2. Read all previously written artifacts for this change as inputs.
3. Write the artifact to the path listed in `generates` under givn/changes/<id>/.
4. Follow the instruction for that artifact precisely.

## Instructions per artifact

<for each artifact, paste the full output of: givn instructions <id> --change <id>>

## Artifact-specific rules

**proposal** (proposal.md):
- WHAT and WHY in domain language — no implementation detail.
- Observable behaviour only. No class names, routes, DB schemas, step mechanics.
- Think about: who benefits, what changes from the user's perspective, what
  constraints or risks exist.

**specs** (specs/**/*.feature):
- One .feature file per capability, derived from the proposal.
- Tags: `@givn.delta @<capability>` on Feature; `@givn.added` + `@wip` on
  each Scenario (use `@givn.modified` or `@givn.removed` where appropriate).
- Scenarios assert observable behaviour in Given/When/Then domain language.
- After writing, validate: run `givn lint --change <id>`
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Think about: what are the key observable behaviours? What are the edge cases?
  What existing behaviour might be affected?

**design** (design.md):
- HOW layer: technology decisions, architecture impact, step def locations,
  runner command, data model changes. Justify non-obvious choices.
- Think about: what is the simplest architecture that delivers the proposal?
  What are the tradeoffs? What existing code is affected?

**tasks** (tasks.md):
- One `- [ ]` checkbox per scenario, covering a full RED/GREEN/REFACTOR cycle.
  Infrastructure/setup tasks first, feature scenarios after.
- Confirm verify command from givn/config.yaml.
- Think about: what is the right order? What setup is needed before features?
  Are any tasks dependent on each other?

**Any custom artifact**:
- Follow the instruction returned by `givn instructions <id>`.
- Write to the path in `generates`.
- Think before writing.

## After writing tasks (or the last planning artifact)

Run: givn apply --change <id>
Confirm task tracking is live.

## Verify command
<verify_command from givn status --json>

## Report back with
- A summary of each artifact written (path + one-sentence summary)
- Any significant decisions or tradeoffs made
- Any ambiguities that required a judgment call (flag these clearly)
- Lint result for specs
- Final givn status --change <id> output
```

### 6. Wait for the architect subagent to complete

The architect must produce all planning artifacts (through `tasks`) and run
`givn apply` before the orchestrator proceeds.

If the subagent flags ambiguities that require user input, surface them and
wait before spawning again (or re-prompting the subagent with the answers).

### 7. Show final status

```sh
givn status --change <id>
```

---

## When to stop

Stop after the architect subagent has written the last artifact whose `id`
appears in the sequence **before** `review`. In the default manifest that is
`tasks`; in a customised manifest it may be different — the architect follows
the actual sequence from `givn status --json`.

Do **not** produce the `review` artifact — that belongs to `/givn-review` after
implementation is done.

---

## Output

```
## Change planned: <id>

**Artifacts written** (in manifest order):
- proposal:  givn/changes/<id>/proposal.md
  <one-sentence summary>
- specs:     givn/changes/<id>/specs/<cap>/<cap>.feature  (N scenarios, all @wip)
- design:    givn/changes/<id>/design.md
  Key decisions: <brief summary>
- tasks:     givn/changes/<id>/tasks.md
  N tasks (one per scenario)
[... any additional artifacts from the manifest ...]

**Decisions / tradeoffs:** <any flagged by the architect>

**Ready to implement.** Run `/givn-implement` to start the TDD loop.
```

---

## Guardrails

- The orchestrator does not write any artifact — it delegates to the architect subagent.
- Always run `givn status --change <id> --json` to get current state and
  next step — never hardcode the sequence.
- The architect subagent must not put implementation detail (HOW) in the
  proposal or specs.
- All scenarios must be `@wip` — step definitions are not written during planning.
- The architect must run `givn lint --change <id>` after writing specs and fix
  parse errors (exit 1) before continuing.
- Tasks must use `- [ ]` checkboxes so `givn apply` can track progress.
- If the architect flags critical ambiguities before it can proceed, surface
  them to the user — do not guess on their behalf.
