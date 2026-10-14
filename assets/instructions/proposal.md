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
- **Capability routing.** Run `givn spec route --change <id>`. When it
  reports signal, record route's recommendation and the decision in the
  Capability Routing table. When it reports no signal, omit the
  recommendation column and record the decision with its rationale directly.
  The recorded decision stays mandatory either way.
- **Persona confirmation.** A change with a user-visible surface (a browser interface, a terminal interface, or CLI text output) names at least one confirmed Persona, or records `none` with the reason and the operator confirmation.

**Structure:**
1. Release note — one complete, user-facing sentence naming what the change does.
2. Problem / Opportunity — what is wrong or missing?
3. Proposed Solution — what should the system do, observably?
4. Out of Scope — what is not changing?
5. Open Questions — unresolved decisions before moving to specs.

## Step orchestration (propose)

**Audience: the agent running the propose step.** Artifact authors follow the
artifact rules above; this section owns only the step's sequencing. The propose
step is an orchestrator — it creates the scaffold and delegates production of
the planning artifacts through `design` to one **thinking architect subagent**;
it does not write the artifacts itself.

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
   `givn instructions <artifact-id> --change <id>`. Capture the full output.
4. Spawn the architect subagent (Task tool; fall back to a prose description of
   the delegation when the tool is unavailable) with a self-contained prompt:
   the change ID, the user's request, the artifact sequence, and the full
   served instruction text for every artifact in the pass. The architect
   thinks through requirements, constraints, and decisions before writing, and
   writes each artifact by following its served instruction.
5. Stop after `design` and after any custom artifact that precedes
   `design-review`/`tasks` in the sequence. The architect must not produce
   `design-review`, `tasks`, or `review`.

**Critical ambiguities go to the user — the architect does not guess.**

### Sequencing and integrity

`design-review` is mandatory and must run as its own step, in a fresh context,
after the architect subagent's work is done — never continued from the same
session that wrote the design, and never spawned from the proposing
orchestrator's context. Tell the user to run `/givn-design-review <change-id>`
now; a
subagent that just wrote the design cannot credibly stress-test its own plan.
Only after `design-review` signs off does the task-list step run.
