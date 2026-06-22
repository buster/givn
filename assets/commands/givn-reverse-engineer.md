---
description: Reverse-engineer an existing capability into givn specs — discovery, plan, and characterization setup
---

Reverse-engineer an existing codebase capability into givn specifications. This
command produces the **initial baseline spec** for a capability by
characterizing its observed black-box behaviour. Production code is refactored
only for testability seams; behaviour is preserved.

**Input**: The argument after `/givn-reverse-engineer` is the change ID
(kebab-case), OR a plain description of the capability to reverse-engineer
(e.g. "the gherkin merge engine"). If nothing is provided, ask.

This command discovers the artifact sequence from `givn` at runtime — it does
not hardcode which steps exist or in what order. It follows the same manifest
sequence as `/givn-propose`, but with reverse-engineering-specific instructions
for each artifact.

---

## Role: orchestrator

This command is an **orchestrator**. It runs a **discovery phase** (unique to
reverse-engineering), creates the change scaffold, collects context, and then
delegates all planning artifact production to a **thinking architect
subagent** using reverse-mode instructions. The orchestrator does not write
artifacts directly.

---

## The reverse-engineering stance

**Descriptive, not prescriptive.** You are capturing what the system *does*
today, not what it *should* do. Every spec assertion is probed from the real
code. Suspected bugs are flagged, never fixed. Production code behaviour is
never changed — it is refactored only for testability seams (extract interface,
DI, make public), with a behaviour-preservation guarantee.

---

## Steps

### 1. Discovery phase — explore and propose a capability

Use the **Task tool** to spawn a discovery subagent (fall back to describing
the pattern in prose if the Task tool is not available to you).

Pass a self-contained prompt:

```
You are a discovery agent for a givn reverse-engineering change.

Your job: explore the codebase and propose up to 3 candidate capabilities to
reverse-engineer, ranked by suitability.

## What to do

1. Explore the codebase. Identify coherent capabilities — modules, command
   groups, or subsystems that have a clear black-box boundary (CLI surface,
   public API, or module boundary).

2. For each candidate capability, note:
   - Capability name (kebab-case)
   - Black-box boundary (how it will be exercised in tests)
   - Brief description of what it does
   - Estimated number of observable behaviours (scenarios)

3. Rank candidates by suitability. Consider: clear boundary, tractable scope,
   high value. Return up to 3.

## Return format

Return a JSON object:

{
  "candidates": [
    {
      "capability": "<kebab-case name>",
      "boundary": "<CLI surface / public API / module boundary description>",
      "description": "<what it does>",
      "estimated_scenarios": <number>,
      "observable_behaviours": [
        "<brief description of each observable behaviour>"
      ],
      "suspected_bugs": [
        "<description of any suspected bugs found during discovery, or empty array>"
      ],
      "codebase_findings": [
        "<any architectural observations relevant to characterization>"
      ]
    }
  ]
}

Return only the JSON — no prose before or after it.
```

### 2. Present discovery results interactively

Wait for the discovery subagent to return.

Present all candidates to the user, ranked:

```
## Discovery complete

**Candidate 1 (recommended):** <capability>
  Boundary: <boundary>
  Description: <description>
  Estimated scenarios: <N>
  Observable behaviours: <list>
  Suspected bugs: <list or "none">

**Candidate 2:** <capability>
  ...

**Candidate 3:** <capability>
  ...
```

**Wait for the user to select one.** This is an interactive checkpoint.
The user may:
- Select a candidate.
- Adjust the boundary of a selected candidate.
- Add or remove observable behaviours.
- Flag additional suspected bugs.
- Reject all and ask for a different search.

Do NOT proceed until the user selects a capability. If the user wants a
different search, re-run the discovery subagent with the user's guidance.

### 3. Resolve the change ID

If the user gave a kebab-case ID, use it. Otherwise derive one from the
capability name (e.g. "gherkin merge engine" → `reverse-engineer-gherkin-merge`).
Confirm with the user if ambiguous.

### 4. Create the change

```sh
givn new <id>
```

If the change already exists (user pre-created it), `givn new` will fail —
that's fine. Check if `givn/changes/<id>/` exists and has artifacts. If so,
proceed with the existing change rather than creating a new one.

### 5. Discover the artifact sequence

```sh
givn status --change <id> --json
```

Parse the JSON:
- `artifacts`: array in execution order, each with `id`, `status`, `optional`,
  `gates_archive`
- `next_required`: the first artifact to write
- `verify_command`: the configured test runner

### 6. Collect reverse-mode instructions for every planning artifact

For each artifact in the sequence (up to and including `tasks`, excluding
`review`), the reverse-mode instructions are inlined below. These are the
canonical reverse-engineering instructions (source files in
`assets/instructions/*-reverse.md`).

**Map artifacts to reverse-mode instructions:**

| Artifact id | Reverse-mode instruction file |
|---|---|
| proposal | `assets/instructions/proposal-reverse.md` |
| specs | `assets/instructions/specs-reverse.md` |
| design | `assets/instructions/design-reverse.md` |
| design-review | (reuses `/givn-design-review` — no reverse variant) |
| tasks | `assets/instructions/tasks-reverse.md` |
| arc42-docs (if addon enabled) | (reuses the standard arc42-docs instruction) |
| Any custom artifact | Follow the standard instruction for that artifact |

Read each reverse-mode instruction file and capture its full content. These
will be passed to the architect subagent.

### 7. Spawn a thinking architect subagent

Use the **Task tool** to spawn a thinking architect subagent.

Pass a self-contained prompt:

```
You are a thinking architect producing the complete plan for a givn
reverse-engineering change.

## Change ID
<id>

## Capability to reverse-engineer
<selected capability name, boundary, description from user's choice>

## Discovery results
<paste the full JSON for the selected candidate>

## Artifact sequence (from givn status --json)
<paste the artifacts array>

## Your task
Produce every planning artifact in order — EXCEPT `design-review` and `review`.
- Skip `design-review` — it requires an interactive grilling loop that a
  subagent cannot perform. The user will run `/givn-design-review` separately
  after planning.
- Do not produce `review` — that belongs to `/givn-characterize`.

Produce: proposal → specs → design → tasks (skipping design-review between
design and tasks). If the manifest includes other artifacts (e.g. `arc42-docs`),
produce those too — unless they require interactive review.

For each artifact:
1. Think carefully about the requirements, constraints, and implications.
2. Read all previously written artifacts for this change as inputs.
3. Read the ACTUAL CODEBASE to probe observed behaviour — specs must
   describe what the code really does, not what you guess it does.
4. Write the artifact to the path listed in `generates` under
   givn/changes/<id>/.
5. Follow the reverse-mode instruction for that artifact precisely.

## Reverse-engineering stance
- DESCRIPTIVE, not prescriptive. Capture AS-IS behaviour.
- Never change production code behaviour. Refactor only for testability seams.
- Suspected bugs are FLAGGED, never fixed. Record as follow-up
  @givn.modified candidates.
- All inferred behaviour is PROVISIONAL — it will be validated during
  characterization.

## Instructions per artifact

### proposal (proposal.md)
<paste full content of assets/instructions/proposal-reverse.md>

### specs (specs/**/*.feature)
<paste full content of assets/instructions/specs-reverse.md>

Tags: @givn.delta @<capability> on Feature; @givn.added @wip on each Scenario.
After writing, validate: run `givn lint --change <id>`
Exit 0 or 2 = correct. Exit 1 = parse error — fix before continuing.

### design (design.md)
<paste full content of assets/instructions/design-reverse.md>

**Test runner**: decide which runner fits the tech stack. Document it in
design.md. If `verify.command` in `givn/config.yaml` is still
`"givn missing-testrunner"`, update it now:
```yaml
verify:
  command: "<chosen runner>"
```

### tasks (tasks.md)
<paste full content of assets/instructions/tasks-reverse.md>

**First task must always be test infrastructure setup**: install the runner,
configure the project for characterization (step definitions skeleton or test
module structure), and verify `verify.command` is set.
Subsequent tasks: one `- [ ]` checkbox per scenario, covering a full
characterization cycle (GREEN-first, then REFACTOR if a seam is needed).
Every scenario task must include a "Design constraints" block drawn from
design.md, specific to that scenario.

### Any custom artifact
Follow the instruction returned by `givn instructions <id>`.
Write to the path in `generates`.
Think before writing.

## After writing tasks (or the last planning artifact)

Run: givn apply --change <id>
Confirm task tracking is live.

## Report back with
- A summary of each artifact written (path + one-sentence summary)
- The black-box boundary chosen
- Number of scenarios written
- Any suspected bugs flagged
- Any significant code smells or ambiguities discovered
- Lint result for specs
- Final givn status --change <id> output
```

### 8. Wait for the architect subagent to complete

The architect must produce all planning artifacts (through `tasks`) and run
`givn apply` before the orchestrator proceeds.

If the subagent flags ambiguities that require user input, surface them and
wait before spawning again.

### 9. Show final status

```sh
givn status --change <id>
```

---

## When to stop

Stop after the architect subagent has written the last artifact whose `id`
appears in the sequence, **excluding `design-review` and `review`**:
- `design-review` is skipped — the user runs `/givn-design-review` separately
  (it requires an interactive grilling loop).
- `review` is produced by `/givn-characterize` after characterization.

In the default manifest, the architect produces: proposal, specs, design,
tasks. It does NOT produce design-review or review.

If the `arc42-docs` addon is enabled, the architect produces the arc42-docs
artifact as part of planning (it creates/updates `docs/arc42/` chapters based
on what reverse-engineering revealed — chapters 5, 6, and 8 are primary
outputs).

---

## Output

```
## Reverse-engineering change planned: <id>

**Capability:** <capability name>
**Black-box boundary:** <boundary>

**Artifacts written** (in manifest order):
- proposal:  givn/changes/<id>/proposal.md
  <one-sentence summary>
- specs:     givn/changes/<id>/specs/<cap>/<cap>.feature  (N scenarios, all @wip)
- design:    givn/changes/<id>/design.md
  Test runner: <chosen runner>
  Seams: <brief summary of testability refactors planned>
- tasks:     givn/changes/<id>/tasks.md
  N tasks (one per scenario + 1 infrastructure setup)
[... any additional artifacts from the manifest ...]

**Suspected bugs flagged (NOT fixed):**
- <bug 1 or "none">

**Ready to characterize.** Run `/givn-design-review` to stress-test the plan,
then `/givn-characterize` to start the characterization loop.
```

---

## Guardrails

- The orchestrator does not write any artifact — it delegates to the architect
  subagent (and the discovery subagent before it).
- Discovery is always interactive — never skip the user confirmation checkpoint.
- One capability per change. If the user wants multiple capabilities, suggest
  separate changes.
- The architect subagent must not put implementation detail (HOW) in the
  proposal or specs.
- All scenarios must be `@givn.added @wip` — they are descriptive, against an
  empty permanent spec.
- The architect must run `givn lint --change <id>` after writing specs and fix
  parse errors (exit 1) before continuing.
- Tasks must use `- [ ]` checkboxes so `givn apply` can track progress.
- The architect must read the ACTUAL CODEBASE to determine observed behaviour —
  specs must not be guessed from documentation alone. Documentation can be wrong.
- Suspected bugs are flagged in proposal.md (Out of Scope) — never fixed.
- Production code is never changed during planning. Testability refactors are
  documented in design.md but applied during `/givn-characterize`.
- The architect MUST skip `design-review` — it is an interactive command
  (`/givn-design-review`) that cannot be run by a subagent. The architect
  produces tasks directly after design.
