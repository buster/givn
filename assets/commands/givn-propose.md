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
(e.g. "add user auth" → `add-user-auth`). Confirm with the user only if
multiple reasonable kebab-case IDs could derive from the description.

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
  Tags go on the line **before** `Scenario:`, never inside the scenario body.
  <example caption="Correct tag placement">
  @givn.added @wip
  Scenario: Observable outcome when something happens
    Given some context
    When an action occurs
    Then an observable result is visible
  </example>
- Scenarios assert observable behaviour in Given/When/Then domain language.
- **Every capability MUST include at least one `@e2e` smoke-test scenario** that
  exercises the real interaction surface (browser, CLI, API). Tag these
  `@givn.added @e2e @wip`. They test the critical happy path through the real
  external boundary — not unit-level or in-process behaviour.
  <example caption="E2e scenario">
  @givn.added @e2e @wip
  Scenario: User completes the action through the real interface
    Given a browser on the application page
    When the user performs the action
    Then the expected result is visible in the interface
  </example>
- After writing, validate: run `givn lint --change <id>`
  Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.
- Think about: what are the key observable behaviours? What are the edge cases?
  What existing behaviour might be affected? What does the real user interaction
  look like end-to-end?

**design** (design.md):
- HOW layer: technology decisions, architecture impact, step def locations
  (one file per capability, name the exact files), data model changes.
  Justify non-obvious choices.
- **Test runner**: pick one fitting the tech stack, document the command. If
  `verify.command` is still `"givn missing-testrunner"`, set it now.
- **Strict mode (mandatory)**: most runners treat undefined/pending steps as
  neither pass nor hard failure by default; some (notably cucumber-jvm with
  an empty step body) report an outright PASS for a step that does nothing.
  Document explicitly: the strict-mode flag/config for the chosen runner
  (`cucumber-rs`: `.fail_on_skipped()`; `cucumber-js`/`cucumber-jvm` CLI:
  `--strict`; `cucumber-jvm` JUnit Platform Suite: no native flag — enforce
  via never-empty bodies plus a plugin/listener failing on UNDEFINED/
  PENDING; `behave`: steps must `raise`), the not-implemented stub for the
  language (Java `PendingException`, Python `NotImplementedError`, Rust
  `unimplemented!()`, JS `return "pending"` or `throw new Error(...)`), and
  the single-scenario run command.
- **Local runnability & digital twins (mandatory)**: a local run command
  that starts the entire system + every dependency in one step (e.g.
  `docker-compose up`), in an isolated network; a named digital twin
  (fake/stub/emulator, same isolated network) for every external/
  third-party dependency — no scenario may depend on a live third-party
  service; and any foreseeable real-interface obstacle (browser session/
  cookie persistence, auth redirects) named with its concrete fix.
  "The interface had a technical problem so we tested the database
  instead" is never acceptable — that obstacle must be designed away here.
- **E2E smoke test infrastructure**: e2e runner command (→
  `verify.e2e_command`), e2e step location (separate, one file per
  capability), local test infrastructure (browser driver, server, test
  DB/Testcontainers/fakes), framework choice + justification (Playwright
  default for browser), strict-mode config for the e2e runner specifically.
  **Real-interface assertion rule**: every `@e2e` scenario's primary
  assertion is on what the real interface returned/displayed — repository
  checks are secondary/additional only, never a replacement.
- Think about: simplest architecture that delivers the proposal, tradeoffs,
  affected existing code, whether the runner can actually be forced to fail
  on an unimplemented step or whether step-body discipline must carry that
  weight, and what could go wrong when actually driving the real interface.

**tasks** (tasks.md):
- **First task: test infrastructure setup, PROVEN strict.** Install the
  runner, configure the strict-mode flag from design.md, create step
  skeletons one file per capability, set `verify.command`. Mandatory
  **proof-of-strictness sub-task**: write one step using the not-implemented
  stub, run the runner, confirm NON-ZERO exit, paste command + output. This
  gates all others — an unproven runner makes every later GREEN meaningless.
  Not optional: this exact failure has happened — every step left an empty
  body, `@wip` removed, runner silently reporting 100% GREEN.
- **One checkbox per non-@e2e scenario**, full RED→GREEN→REFACTOR→COMMIT.
  Each RED/GREEN/REFACTOR sub-item has a field for pasted runner output
  (targeting that scenario only, via design.md's single-scenario command) —
  not a description. RED's unimplemented steps use the not-implemented
  stub, never an empty body. GREEN lists every production file
  created/modified — empty list on a non-reuse scenario means investigate.
  COMMIT has a field for the hash; message references the scenario title.
- **After all non-@e2e scenarios**: e2e setup task — bring up the local
  environment (local run command from design.md, all dependencies, all
  digital twins, isolated network) and confirm clean startup; install the
  e2e framework, e2e step skeleton one file per capability, PROVE strict
  mode for the e2e runner, set `verify.e2e_command`. Then one task per @e2e
  scenario with the same cycle and evidence requirements, PLUS: GREEN
  asserts primarily on the real interface's output, never repository-only.
  If a real-interface obstacle appears mid-implementation, the task's fix
  is the one named in design.md's Anticipated Interface Obstacles section —
  never a downgrade to repository-only assertions.
- **Every scenario task has a "Design constraints" block** drawn from
  design.md, specific to that scenario — not optional. The executing agent
  reads the task as its primary architecture reference; without constraints
  it will take the shortest path, which may not match the design.
  ```
  Design constraints:
    - <which module/layer owns this behaviour>
    - <which protocol, data format, or library to use>
    - <what response shape / side-effect is required by the design>
  ```
- Think about: right order, setup needed before features, task dependencies.

**Any custom artifact**:
- Follow the instruction returned by `givn instructions <id>`.
- Write to the path in `generates`.
- Think before writing.

## After writing tasks (or the last planning artifact)

Run: givn apply --change <id>
Confirm task tracking is live.

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

- The orchestrator delegates all artifact writing to the architect subagent.
- Always run `givn status --change <id> --json` for current state — never
  hardcode the sequence.
- No implementation detail (HOW) in proposal or specs.
- All scenarios `@wip` — step definitions are not written during planning.
- `givn lint --change <id>` after writing specs; fix parse errors (exit 1).
- Tasks use `- [ ]` checkboxes for `givn apply` to track.
- design.md must name a concrete strict-mode mechanism and stub pattern —
  this is what prevents the change from being fabricated with empty stubs
  during implementation. Do not accept a plan that omits it.
- tasks.md's first task must include a proof-of-strictness sub-task with a
  field for non-zero-exit evidence. Do not accept a plan that omits it.
- Every scenario task ends in COMMIT; every RED/GREEN/REFACTOR line has a
  field for pasted output. A tasks.md with nowhere to record what happened
  is incomplete.
- design.md must name a local run command, a digital twin for every
  external/third-party dependency, and a fix for any foreseeable
  real-interface obstacle. Do not accept a plan that defers this to
  "figure it out during implementation."
- Every `@e2e` scenario asserts primarily on the real interface's output.
  Do not accept a plan or a task whose e2e GREEN criteria is
  repository-state-only.
- Critical ambiguities go to the user — the architect does not guess.
