# Instruction: proposal

Write a proposal that describes the WHAT and WHY of this change, in domain
language that a non-technical stakeholder can read and validate.

**Rules (enforced by default):**
- Describe observable behaviour, not implementation. No class names, function
  signatures, HTTP routes, DB schemas, step-definition mechanics, or framework
  references.
- The only permitted implementation-flavoured content is an aspect that is
  *itself* an externally observable requirement (e.g. "the response is valid JSON",
  "data persists across restart", "responds within 200 ms"). Heuristic: if it
  would still be true if the system were reimplemented in a different language
  and framework, it belongs in the spec; otherwise it belongs in design/tasks.
- Write from the user's perspective. Use domain language (business terms, not
  technical terms).
- **Executable-spec invariant.** The `.feature` files produced downstream (in
  the `specs` artifact) are the executable tests. The `verify.command` chosen
  in `design.md` MUST execute them via a Gherkin/Cucumber runner — never a
  parallel hand-maintained test file. Keep this in mind: every behaviour you
  describe here must be expressible as a runnable Gherkin scenario.
- **Use-case context.** When a change has a permanent use-case ID or an
  ideation change seed, read its `usecase.md` and record the ID, source topic,
  and confirmed affected Persona IDs. Never invent or auto-promote Personas;
  do not copy Persona biographies into the proposal.
- **Persona confirmation.** A change with a user-visible surface (a browser interface, a terminal interface, or CLI text output) names at least one confirmed Persona, or records `none` with the reason and the operator confirmation.

**Structure:**
1. Problem / Opportunity — what is wrong or missing?
2. Proposed Solution — what should the system do, observably?
3. Out of Scope — what is not changing?
4. Open Questions — unresolved decisions before moving to specs.
