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
- **Seed stamping.** When the change originates from an ideation change seed,
  record the seed path in the proposal's Use-Case Context and set the seed's
  status line to `Status: proposed by <change-id>`. The seed's original text
  stays unchanged; only the status line is edited.
- **Question disposition.** When the change originates from an ideation seed,
  every intersecting seed question is dispositioned in the proposal before
  specs: **answered** with its source, **carried** with an owner and the
  artifact that will decide it, or **out of scope** with a reason. An
  undispositioned intersecting question blocks the specs step; the same
  structure is advisory without an ideation source. "Resolved in `design.md`"
  alone is not a disposition. A statement qualified `(open: Q<N>)` names a
  question that must be dispositioned here.
- **Question IDs.** Open questions carry stable IDs `D1`, `D2`, … (`D<n>` in
  the templates) with the full question and its option treatment; the
  design's ledger resolves each ID.
- **Persona confirmation.** A change with a user-visible surface (a browser interface, a terminal interface, or CLI text output) names at least one confirmed Persona, or records `none` with the reason and the operator confirmation.

**Structure:**
1. Problem / Opportunity — what is wrong or missing?
2. Proposed Solution — what should the system do, observably?
3. Out of Scope — what is not changing?
4. Open Questions — unresolved decisions before moving to specs.
