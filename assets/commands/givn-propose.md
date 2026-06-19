---
description: Plan a givn change end-to-end — proposal, Gherkin spec, design, and task list in one pass
---

Plan a new givn change from start to finish.

**Input**: The argument after `/givn-propose` is the change ID (kebab-case), OR a
plain description of what to build. If nothing is provided, ask.

This command runs the full planning chain: **proposal → spec → design → tasks**.
It stops when the change is fully planned and ready to implement. You invoke
`/givn-implement` when you are ready to start coding.

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

### 3. Get proposal instructions

```sh
givn instructions proposal --change <id>
```

### 4. Write the proposal

Read any existing permanent specs at `givn/specs/` for context.

Write `givn/changes/<id>/proposal.md`:

1. **Problem / Opportunity** — what is wrong or missing?
2. **Proposed Solution** — what should the system do?
3. **Out of Scope** — what is not changing?
4. **Open Questions** — unresolved decisions (if any).

**Rules:**
- Domain language only. No class names, routes, DB schemas, function names,
  framework details, or step-definition mechanics.
- Only externally observable technical constraints are allowed (e.g. "response
  is valid JSON", "persists across restart").
- Park any HOW details — they go in design.md, written below.

---

### 5. Get spec instructions

```sh
givn instructions specs --change <id>
```

### 6. Write delta .feature files

Read the proposal. For each capability, create:
`givn/changes/<id>/specs/<capability>/<capability>.feature`

**Required tags:**
- `@givn.delta` and `@<capability>` on the Feature line
- `@givn.added` / `@givn.modified` / `@givn.removed` on each Scenario (default: `@givn.added`)
- `@wip` on every Scenario

**Rules:**
- Scenarios assert observable behaviour in Given/When/Then domain language.
- No implementation detail in the spec.
- One `.feature` file per capability.

### 7. Validate syntax

```sh
givn lint
```

- Exit 0 or 2 = fine (`@wip` present is expected at this stage).
- Exit 1 = parse error — fix before continuing.

---

### 8. Get design instructions

```sh
givn instructions design --change <id>
```

### 9. Write the design

Read the proposal and the `.feature` files just written.
Read relevant existing code (architecture, patterns, integration points).

Write `givn/changes/<id>/design.md` covering:

- **Technology decisions** — language, framework, libraries chosen and why.
- **Architecture impact** — which modules are affected; what is new.
- **Data model changes** — schema, structs, DB changes if any.
- **Step definition locations** — where Cucumber/Gherkin step defs will live.
- **Runner command** — confirm `verify.command` in `givn/config.yaml`.

This is the HOW layer. Everything kept out of proposal and spec belongs here.

---

### 10. Get tasks instructions

```sh
givn instructions tasks --change <id>
```

### 11. Write the task list

Read the design and the `.feature` files.

Write `givn/changes/<id>/tasks.md` as a checkboxed scenario-by-scenario breakdown.
One task per scenario, each covering a full RED/GREEN/REFACTOR cycle:

```markdown
- [ ] <capability>: <Scenario title>
      RED:      Remove @wip. Write step defs with real assertions. Run: <verify.command> → MUST FAIL.
      GREEN:    Write minimum production code. Run: <verify.command> → PASSES.
      REFACTOR: Clean up. Run: <verify.command> → still PASSES.
```

Order: infrastructure/setup tasks first, feature scenarios after.

### 12. Confirm task tracking

```sh
givn apply --change <id>
```

### 13. Show final status

```sh
givn status
```

---

## Output

```
## Change planned: <id>

**Proposal:** givn/changes/<id>/proposal.md
  <one-paragraph summary of WHAT and WHY>

**Spec:** givn/changes/<id>/specs/<capability>/<capability>.feature
  N scenarios — all @wip, ready for step definitions

**Design:** givn/changes/<id>/design.md
  Key decisions: <brief summary>

**Tasks:** givn/changes/<id>/tasks.md
  N tasks (one per scenario), ready for RED/GREEN/REFACTOR

**Ready to implement.** Run `/givn-implement` to start the TDD loop.
```

---

## Guardrails

- Write all four artifacts before stopping — do not pause between them to ask.
- Never put implementation detail (HOW) in the proposal or spec.
- Keep all scenarios `@wip` — step definitions are not written yet.
- Run `givn lint` after writing specs and fix any parse errors (exit 1) before continuing.
- Tasks must use `- [ ]` checkboxes so `givn apply` can track progress.
- If context is critically unclear before starting, ask — but prefer reasonable
  decisions over stalling mid-chain.
