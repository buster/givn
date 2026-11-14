# Instruction: proposal

Write a proposal that describes the WHAT and WHY of this change, in domain
language that a non-technical stakeholder can read and validate.

**Rules (enforced by default):**
- **Release note (required).** The proposal carries one complete, user-facing
  sentence naming what the change does; every revision the change records
  carries the release note as its message subject, with the scenario title in
  the message body. A one-scenario change may reuse its scenario sentence as
  the release note.
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
- **Review kind (required).** Record `Review kind: self-review` (the default)
  or `Review kind: independent` in the proposal. Independence is requested at
  proposal time and the recorded kind is authoritative after a restart. A
  major change records its reason. If the major-change classification is
  ambiguous, ask the user one question; never infer it silently.
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
{{#if addons.architecture_principles}}
  Classify each open question with `givn instructions architecture-principles`;
  a contract is not an open question — it follows the consumer use case.
{{/if}}
{{#if addons.ise}}
- **Corrective change.** When the change responds to an observed difference
  between the frozen end-to-end test result and the declared `Specification`,
  the proposal names the `Spec gap` record and its single outcome before any
  fix, and names the artifact that is wrong, not who found it. The outcome
  selects the route: `specification defect` or `E2E test defect` carries the
  corpus or E2E suite edit in this change, `implementation defect` returns to
  the builder inside the Ise Test with no corpus edit, and
  `acceptable undocumented change` is recorded in the corpus.
{{/if}}
- **Question IDs.** Open questions carry stable IDs `D1`, `D2`, … (`D<n>` in
  the templates) with the full question and its option treatment; the
  design's ledger resolves each ID.
- **Capability routing.** Run `givn spec route --change <id>`. When it
  reports signal, record route's recommendation and the decision in the
  Capability Routing table. When it reports no signal, omit the
  recommendation column and record the decision with its rationale directly.
  The recorded decision stays mandatory either way.
- **Persona confirmation.** A change that declares a visual design contract, including `Interface: none`, names at least one confirmed Persona; only a change that declares no visual design contract may record `none` with the reason and the operator confirmation. A user-visible surface (a browser interface, a terminal interface, or CLI text output) names at least one confirmed Persona.

**Structure:**
1. Release note — one complete, user-facing sentence naming what the change does.
2. Problem / Opportunity — what is wrong or missing?
3. Proposed Solution — what should the system do, observably?
4. Out of Scope — what is not changing?
5. Open Questions — unresolved decisions before moving to specs.

## Step orchestration (propose)

**Audience: the agent running the propose step.** Artifact authors follow the
artifact rules above; this section owns only the step's sequencing. Work in the
current context; the user or harness may split the steps across sessions, and
the artifacts remain the handoff mechanism.

1. Resolve the change ID (derive a kebab-case ID from a plain description) and
   create the change:
   ```sh
   givn new <id>
   ```
2. Read the artifact sequence from `givn status --change <id> --json` — never
   hardcode which steps exist or in what order; a customised manifest is
   followed as configured.
3. Load the served instruction for every artifact in the sequence **up to and
   including `design`** (stop before `design-review` if it appears in the
   sequence; otherwise stop before `tasks`) via
   `givn instructions <artifact-id> --change <id>`.
4. Write each artifact in the current context by following its served
   instruction. Think through requirements, constraints, and decisions before
   writing. Stop after `design` and after any custom artifact that precedes
   `design-review`/`tasks` in the sequence. Do not produce `design-review`,
   `tasks`, or `review` in this pass.

**Critical ambiguities go to the user — do not guess.**

### Sequencing and integrity

`design-review` is mandatory and runs as its own step after the design is
written. Use the review context recorded in the proposal: a `self-review` is
performed in the current context; an `independent` review uses the one review
context described by `givn instructions review`. Only after `design-review`
signs off does the task-list step run.
