---
description: Plan a givn change through design — proposal, spec, and design in the order the manifest defines; hands off to /givn-design-review next
---

Plan a new givn change from proposal through design.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: The argument after `/givn-propose` is the change ID (kebab-case), OR a
plain description of what to build. If nothing is provided, ask.

This command discovers the artifact sequence from `givn` at runtime — it does not
hardcode which steps exist or in what order. If the project has customised its
manifest (added a risk-assessment step, removed review, etc.), this command
automatically follows the configured sequence.

---

## Role: orchestrator

This command is an **orchestrator**. It creates the change scaffold, collects
context, and then delegates production of the planning artifacts through
`design` to a **thinking architect subagent**, in one pass. The orchestrator
does not write artifacts directly. `design-review` and everything after it
(`tasks`, `review`) are separate steps run in their own fresh contexts — see
"When to stop" below.

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

### 4. Collect instructions for every planning artifact up to `design`

For each artifact in the sequence **up to and including `design`** (stop
before `design-review` if it appears in the sequence; otherwise stop before
`tasks`):

```sh
givn instructions <artifact-id> --change <id>
```

Capture the full output for each.

**Do not include `design-review` or anything after it in this pass.**
`design-review` must run as its own step, in a fresh context, after this
architect subagent's work is done — never continued from the same session
that wrote the design (see step 6 below). If the manifest has been
customised to remove `design-review` entirely, the sequence naturally has no
such artifact and this restriction does not apply — in that case, continue
through `tasks` as before.

### 5. Spawn a thinking architect subagent

Use the **Task tool** to spawn a thinking architect subagent (fall back to
describing the pattern in prose if the Task tool is not available to you).

The architect subagent is a **deep-thinking planning agent** — give it room to
reason carefully. It must not rush to produce output; it should think through
requirements, constraints, and decisions before writing each artifact.

Pass a self-contained prompt:

```
You are a thinking architect producing the complete plan for a givn change.

For every user-facing question, follow the effective policy from `givn instructions dialog`: this fresh context does not inherit the parent's loaded policy, so load it in the fresh context before the first question. The presenting agent remains responsible for the question, and a question tool's recommendation or option label does not replace context and answer consequences.

## Change ID
<id>

## What to build
<user's original description or request>

## Artifact sequence (from givn status --json)
<paste the artifacts array>

## Your task
Produce every planning artifact up to and including `design` — stop there.
Do NOT produce `design-review`, `tasks`, or `review`. `design-review` is a
mandatory gate that must run as a separate step, in a fresh context, never
continued from this session — a subagent that just wrote the design cannot
credibly stress-test its own plan. `tasks` and `review` depend on
`design-review` having run first.

For each artifact:
1. Think carefully about the requirements, constraints, and implications.
2. Read all previously written artifacts for this change as inputs.
3. Write the artifact to the path listed in `generates` under givn/changes/<id>/.
4. Follow the instruction for that artifact precisely.

## Instructions per artifact

Follow each artifact's own served instruction — it owns that artifact's rules;
this command only orchestrates the transition:

- `givn instructions proposal --change <id>` — the proposal: WHAT and WHY in
  domain language, the release note, and the capability-routing decision.
- `givn instructions specs --change <id>` — the delta `.feature` files, their
  tags, and the interaction inventory.
- `givn instructions design --change <id>` — the HOW layer.
- Any custom artifact between `design` and `design-review`/`tasks`:
  `givn instructions <artifact-id> --change <id>`.

Pass the full served output for every artifact to the architect subagent as
described above; the served instruction, not this file, carries the rules.

## Stop after `design`

Do not write `tasks.md`. Do not run `givn status --change`. Those happen after
`design-review` has run as a separate, fresh-context step — see the
orchestrator's next steps.

## Report back with
- A summary of each artifact written (path + one-sentence summary)
- Any significant decisions or tradeoffs made
- Any ambiguities that required a judgment call (flag these clearly)
- Lint result for specs
- Final givn status --change <id> output
```

### 6. Wait for the architect subagent to complete

The architect must produce all planning artifacts through `design` (and any
custom artifact preceding `design-review`/`tasks`) before the orchestrator
proceeds. It must NOT produce `design-review`, `tasks`, or `review`.

If the subagent flags ambiguities that require user input, surface them and
wait before spawning again (or re-prompting the subagent with the answers).
Phrase and record every question to the user per the canonical dialog policy
(`givn instructions dialog`): name artifacts in domain language, and give
every option its meaning, gain, cost, and a concrete example.

### 6b. Hand off to design-review

If `design-review` appears in the artifact sequence (the default), tell the
user:

```
Design written. Design-review is mandatory and must run in a fresh context —
run /givn-design-review <id> now before tasks are written.
```

Do not spawn `/givn-design-review`'s subagents yourself from this
orchestrator's context — the user (or their agent harness) invokes
`/givn-design-review` as a new, separate command so the grilling subagent
starts genuinely fresh, not as a continuation of this conversation. Stop
here.

If `design-review` has been removed from the manifest (project override),
tell the user to run `/givn-tasks <id>` next instead. Do not write
`tasks.md` from this orchestrator's own architect subagent — `/givn-tasks`
is its own command with its own fresh invocation, kept consistent regardless
of whether design-review ran.

### 7. Show final status

```sh
givn status --change <id>
```

---

## When to stop

Stop after the architect subagent has written `design` (and any custom
artifact preceding `design-review`/`tasks` in the sequence). Never let this
orchestrator's architect subagent produce `design-review`, `tasks`, or
`review` — each is a separate command run in its own fresh context:
`/givn-design-review`, then `/givn-tasks`, then (after implementation)
`/givn-review`.

---

## Output

```
## Change planned so far: <id>

**Artifacts written** (in manifest order):
- proposal:  givn/changes/<id>/proposal.md
  <one-sentence summary>
- specs:     givn/changes/<id>/specs/<group>/<cap>.feature  (N scenarios, all @wip)
- design:    givn/changes/<id>/design.md
  Key decisions: <brief summary>
[... any additional artifacts preceding design-review/tasks ...]

**Decisions / tradeoffs:** <any flagged by the architect>

**Next:** Run `/givn-design-review <id>` now, in a fresh context, before
tasks are written. (Or `/givn-tasks <id>` if design-review has been removed
from this project's manifest.)
```

---

## Guardrails

- The orchestrator delegates proposal/spec/design writing to the architect
  subagent — never `design-review`, `tasks`, or `review`.
- Always run `givn status --change <id> --json` for current state — never
  hardcode the sequence.
- No implementation detail (HOW) in proposal or specs.
- All scenarios `@wip` — step definitions are not written during planning.
- `givn lint --change <id>` after writing specs; fix parse errors (exit 1).
- design.md must name a concrete strict-mode mechanism and stub pattern —
  this is what prevents the change from being fabricated with empty stubs
  during implementation. Do not accept a plan that omits it.
- design.md must name a local run command, a digital twin for every
  external/third-party dependency, and a fix for any foreseeable
  real-interface obstacle. Do not accept a plan that defers this to
  "figure it out during implementation."
- design.md must not contain a version number written from memory. Every
  versioned technology choice is either "latest"/"LTS" or a looked-up
  version with a note of what was checked. Reject a plan with bare version
  numbers and no lookup note or "latest/LTS" designation.
- **Never run `/givn-design-review`'s subagents from within this
  orchestrator's context.** Design-review must start genuinely fresh — tell
  the user to invoke `/givn-design-review <id>` as its own command.
- Critical ambiguities go to the user — the architect does not guess.
