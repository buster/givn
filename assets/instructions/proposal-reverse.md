# Instruction: proposal-reverse

Write a proposal that describes the characterization of an **existing** capability.
This is a reverse-engineering change — the goal is to capture AS-IS behaviour, not
to design new behaviour.

**Stance: descriptive, not prescriptive.**
- You are documenting what the system *does* today, not what it *should* do.
- Write from the perspective of an observer probing a black box.
- Suspected bugs are NOT fixed — they are recorded as out-of-scope follow-up
  candidates for a future `@givn.modified` delta.

**Rules (enforced by default):**
- Describe observable behaviour, not implementation. No class names, function
  signatures, HTTP routes, DB schemas, step-definition mechanics, or framework
  references. The only permitted implementation-flavoured content is an aspect
  that is *itself* an externally observable requirement.
- The black-box boundary (discovered during the discovery phase) is named
  explicitly: "The black-box boundary for this capability is: <CLI surface /
  public API / module boundary>".
- All inferred behaviour is **provisional** — it will be validated during
  characterization. If a spec assertion is proven wrong during characterization,
  it is corrected to match AS-IS behaviour (see `specs-reverse.md`).
- **Executable-spec invariant.** The `.feature` files produced downstream (in
  the `specs` artifact) are the executable tests. The `verify.command` chosen
  in `design-reverse.md` MUST execute them via a Gherkin/Cucumber runner —
  never a parallel hand-maintained test file. Keep this in mind: every
  behaviour you describe here must be expressible as a runnable Gherkin
  scenario.

**Structure:**
1. Capability — which capability is being characterized, and its black-box
   boundary.
2. Current Behaviour — what the system does today, observably, in domain
   language.
3. Out of Scope — suspected bugs, code smells, and architectural issues
   discovered during discovery. These are flagged as follow-up `@givn.modified`
   or `@givn.removed` candidates. They are NOT addressed in this change.
4. Open Questions — any ambiguities discovered that cannot be resolved by
   reading the code; these become characterization probes.
