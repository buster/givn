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
givn plan --json
```

Parse the JSON. It contains:
- `artifacts`: array of artifacts in execution order, each with:
  - `id`: artifact identifier
  - `generates`: file path(s) this artifact produces
  - `optional`: whether it can be skipped
  - `gates_archive`: whether it must be complete before archiving
  - `has_instruction`: whether `givn instructions <id>` will return content
- `verify.command`: the configured test runner
- `verify.features_path`: where features live

**The `artifacts` array IS the planning sequence.** Work through every
non-optional artifact in order (stop before `review` — that belongs to
`/givn-review` after implementation is done).

### 4. Loop: write each planning artifact

For each artifact in the sequence (excluding `review` and any artifact whose
`id` is the implementation gate — i.e. where you stop for the user to code):

#### 4a. Get the instruction

If `has_instruction` is true:
```sh
givn instructions <artifact-id> --change <id>
```
Use the returned instruction to understand what to produce and how.

#### 4b. Read dependencies

Before writing each artifact, read all previously written artifacts for this
change (they are the inputs). Also read relevant existing project files
(permanent specs at `givn/specs/`, existing code, etc.) as needed.

#### 4c. Write the artifact

Write to the path(s) listed in `generates` under `givn/changes/<id>/`.

Follow the instruction for this artifact. Key rules per artifact type:

**proposal** (`proposal.md`):
- WHAT and WHY in domain language — no implementation detail.
- Observable behaviour only. No class names, routes, DB schemas, step mechanics.

**specs** (`specs/**/*.feature`):
- One `.feature` file per capability, derived from the proposal.
- Tags: `@givn.delta @<capability>` on Feature; `@givn.added` (default) /
  `@givn.modified` / `@givn.removed` + `@wip` on each Scenario.
- Scenarios assert observable behaviour in Given/When/Then domain language.
- After writing specs, validate: `givn lint` — fix exit 1 (parse error);
  exit 2 (@wip) is expected and correct.

**design** (`design.md`):
- HOW layer: technology decisions, architecture impact, step def locations,
  runner command, data model changes. Justify non-obvious choices.

**tasks** (`tasks.md`):
- One `- [ ]` checkbox per scenario, each covering a full RED/GREEN/REFACTOR
  cycle. Infrastructure/setup first, feature scenarios after.
- Confirm verify command from `givn/config.yaml`.

**Any custom artifact** (e.g. `risk-assessment.md`):
- Follow the instruction returned by `givn instructions <id>`.
- Write to the path in `generates`.

#### 4d. After writing tasks (or the last planning artifact)

```sh
givn apply --change <id>
```

Confirm task tracking is live.

### 5. Show final status

```sh
givn status
```

---

## When to stop

Stop after writing the last artifact whose `id` appears in the sequence
**before** the implementation gate. In the default manifest that is `tasks`;
in a customised manifest it may be different — follow the actual sequence.

Do **not** write the `review` artifact — that belongs to `/givn-review` after
the implementation is done.

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

**Ready to implement.** Run `/givn-implement` to start the TDD loop.
```

---

## Guardrails

- Always run `givn plan --json` to get the sequence — never hardcode it.
- Write all planning artifacts before stopping. Do not pause between them to ask.
- Never put implementation detail (HOW) in the proposal or specs.
- Keep all scenarios `@wip` — step definitions are not written yet.
- Run `givn lint` after writing specs; fix parse errors (exit 1) before continuing.
- Tasks must use `- [ ]` checkboxes so `givn apply` can track progress.
- Follow the instruction from `givn instructions <id>` for every artifact.
- If context is critically unclear before starting, ask — but prefer reasonable
  decisions over stalling mid-chain.
