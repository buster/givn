---
description: Start a new givn change — create it and write the proposal artifact
---

Start a new givn change.

**Input**: The argument after `/givn-propose` is the change ID (kebab-case), OR a
plain description of what to build. If nothing is provided, ask.

---

## Steps

### 1. Resolve the change ID

If the user gave a description rather than a kebab-case ID, derive one
(e.g. "add user auth" → `add-user-auth`). Confirm with the user before
proceeding if the name is ambiguous.

### 2. Create the change

```sh
givn new <id>
```

This scaffolds `givn/changes/<id>/` with the manifest-defined artifact stubs.

### 3. Get proposal instructions

```sh
givn instructions proposal --change <id>
```

Parse the output for:
- `output`: path to write the proposal (typically `givn/changes/<id>/proposal.md`)
- `template`: structure to follow
- `instruction`: guidance on what to produce
- `context` / `rules`: constraints for you — **do not copy these into the file**

### 4. Write the proposal

Read any existing context (permanent specs at `givn/specs/` if they exist).

Write `givn/changes/<id>/proposal.md` following the template:

1. **Problem / Opportunity** — what is wrong or missing?
2. **Proposed Solution** — what should the system do?
3. **Out of Scope** — what is not changing?
4. **Open Questions** — unresolved decisions before writing specs.

**Rules:**
- Write from the user's perspective in domain language.
- Describe observable behaviour only — no class names, routes, DB schemas,
  function names, framework details, or step-definition mechanics.
- The only technical details allowed are those that are externally observable
  (e.g. "response is valid JSON", "persists across restart").
- If implementation detail surfaces, park it in design.md (not yet written).

### 5. Show progress

```sh
givn status
```

Display next step: "Run `/givn-spec` to write the Gherkin spec."

---

## Output

```
## Change created: <id>

**Proposal written:** givn/changes/<id>/proposal.md

### Summary
<one-paragraph summary of what was captured>

**Next:** Run `/givn-spec` to write the Gherkin .feature spec.
```

---

## Guardrails

- Do not proceed without knowing what the user wants to build.
- Never put implementation detail (HOW) in the proposal.
- If context is critically unclear, ask before writing — but prefer making
  reasonable decisions over stalling.
- Verify the file exists after writing.
