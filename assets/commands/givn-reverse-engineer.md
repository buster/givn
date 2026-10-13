---
description: Reverse-engineer an existing capability into descriptive Gherkin specs — discovery, feature writing, characterization tasks
---

Reverse-engineer an existing codebase capability into **descriptive Gherkin
specs** that document what the system *does* today. No proposals, no designs —
just `.feature` files + characterization tasks that make them executable.

**Dialog policy:** for every user-facing question, follow the effective policy from `givn instructions dialog`; load it before this context's first question.

**Input**: The argument after `/givn-reverse-engineer` is the change ID
(kebab-case), OR a plain description of the capability to reverse-engineer
(e.g. "the gherkin merge engine"). If nothing is provided, ask.

**Rules**: the reverse-engineering discipline — the descriptive stance, the
interactive discovery phase, the spec-writing rules, and the black-box and
suspected-bug rules — is served by `givn instructions reverse-engineer`. Load
it and follow it; this file owns the invocation orchestration only.

---

## Steps

### 1. Discovery phase — explore and propose a capability

Spawn a discovery subagent (fall back to describing the pattern in prose if
the Task tool is not available to you) with a self-contained prompt that:

- Passes the dialog obligation explicitly (a fresh context does not inherit
  the parent's loaded policy).
- Tells the subagent to run `givn instructions reverse-engineer` and follow
  its discovery rules.
- Asks for up to 3 candidate capabilities, ranked by suitability, each with
  its name, black-box boundary, brief description, estimated scenario count,
  suspected bugs, and codebase findings — returned as JSON only.

### 2. Present discovery results interactively

Wait for the discovery subagent to return, then present all candidates to the
user, ranked. **Wait for the user to select one.** This is an interactive
checkpoint: the user may select a candidate, adjust its boundary, add or remove
observable behaviours, flag additional suspected bugs, or reject all and ask
for a different search.

Do NOT proceed until the user selects a capability.

### 3. Resolve the change ID

If the user gave a kebab-case ID, use it. Otherwise derive one from the
capability name (e.g. "gherkin merge engine" → `reverse-engineer-gherkin-merge`).
Confirm with the user if ambiguous.

### 4. Create the change (skipping proposal, design, design-review)

```sh
givn new <id> --skip proposal,design,design-review
```

If the change already exists (user pre-created it), `givn new` will fail —
that's fine; proceed with the existing change.

### 5. Write the descriptive spec, following the served instruction

Write `givn/changes/<id>/specs/<group>/<capability>.feature` per the
spec-writing rules served by `givn instructions reverse-engineer`. Read the
ACTUAL CODEBASE to determine observed behaviour. After writing, validate:

```sh
givn lint --change <id>
```

Exit 0 or 2 = correct (exit 2 means @wip scenarios present — expected).
Exit 1 = parse error — fix before continuing.

### 6. Write tasks.md (characterization cycles)

Write `givn/changes/<id>/tasks.md` with a test-infrastructure setup task
(proving strict mode once, with evidence) followed by one RED/GREEN/REFACTOR
characterization cycle per scenario — the task-file format is served by
`givn instructions tasks-reverse`.

Then confirm task tracking is live:

```sh
givn status --change <id>
```

---

## Guardrails

- Follow the rules served by `givn instructions reverse-engineer`; where this
  file and the served instruction disagree, the served instruction wins.
- Discovery is always interactive — never skip the user confirmation
  checkpoint.
- One capability per change. If the user wants multiple capabilities, suggest
  separate changes.
- Never change production code behaviour during planning; testability seams
  are applied during `/givn-characterize`.
