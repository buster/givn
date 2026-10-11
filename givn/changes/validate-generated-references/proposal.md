# Proposal: validate-generated-references

## Release note

Generated guidance only sends its reader to files the consumer project has or
that a named creation step creates — the persona schema travels inline,
reverse review is requested from Givn itself, and Givn rejects any guidance
reference that resolves nowhere.

## Use-Case Context

- Use-case ID: fragment `support` — capabilities `instruction-integrity` and
  `generate-ideation-guidance`.
- Ideation topic: none — originates from the operator's investigation of a
  new Fossil project whose generated guidance pointed at files the project
  did not have (2026-10-11).
- Seed path: none.
- Confirmed Personas: `agent--technical-role` (follows the guidance's file
  references), `developer--primary-user` (owns the consumer project whose
  guidance is wrong), `givn-maintainer--owner` (owns the guidance corpus and
  the acceptance suite). Existing confirmed project records are reused; no
  persona is created or promoted.

## Problem / Opportunity

A newly initialized project's generated guidance points at files that exist
only in the Givn repository or inside the Givn binary:

- the ideation skill sends the reader to `assets/templates/persona.md` for
  the persona schema;
- the characterization command sends the reader to
  `assets/instructions/review-reverse.md`;
- the ideation skill, the spec skill, and the specs and review instructions
  name `docs/use-case-model.md` as if it were a project document.

The failure is concrete and was observed. In a new Fossil project, the agent
following the ideation skill could not find the persona schema, searched the
machine, and only succeeded because the Givn checkout happened to be present
on that machine; `givn skills sync` installs only `SKILL.md`, so the
reference is broken for every installed project. On a machine without the
checkout the persona phase cannot be performed at all.

The failure class is invisible at Givn's own gate: the acceptance suite reads
those files from the Givn checkout, where they exist, so it proves the source
while the consumer receives a dead pointer. A survey of one freshly
initialized project found 37 rooted file references across the generated
skills and commands; 32 do not exist after initialization, and all but three
are legitimate outputs of a workflow step (`givn new`, `givn archive`, the
spec merge, the ideation workflow, persona promotion). A check therefore
cannot simply demand existence; it must classify every reference by its
creation owner and fail only the undeclared ones. No such check exists, which
is why the three dead references shipped and stayed.

## Proposed Solution

In a freshly initialized project, every file reference in generated guidance
— the installed skills and commands, and the instruction text Givn prints —
resolves to something the project has or is owned by a named creation step.
Givn's acceptance suite fails on a reference that resolves nowhere.

The check distinguishes the resolution classes and treats them differently:

- **Project file** — exists after initialization (configuration, durable
  documents, scaffolds).
- **Workflow-created** — a location a named creation step creates during the
  lifecycle (change artifacts, archives, permanent specs, ideation topic
  state, persona collection).
- **Project-owned destination** — a location the guidance itself defines and
  the project creates while following it (for example the design system a
  visual contract records, or an architecture decision record the arc42-docs
  workflow writes).
- **Agent-target skill file** — a skill named in guidance, resolved by skill
  name across the configured agent targets.

The current defects are fixed so the rule holds:

- the ideation guidance carries the persona schema in its own text; creating
  a persona no longer requires reading a repository file;
- the characterization guidance requests the reverse review from Givn itself
  — in the command and in the skill — instead of naming a distribution
  artifact;
- the use-case model pointers are dropped where the surrounding text already
  carries the rules;
- the one example location that is not a project claim (`docs/adr/` in the
  grill-with-docs command) is reworded; an illustration is not a claim about
  the project.

## Capability Routing

Route reported no signal (the top candidates tied). The decisions:

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| Guidance references resolve in the consumer project | none — route reported no signal | `EXTEND instruction-integrity` | The property extends the same actor goal as the capability's existing scenarios: generated guidance remains complete and trustworthy for its reader. A dead reference is an incomplete instruction; a separate capability would give one goal two contracts. |
| The ideation guidance carries the persona schema inline | none — route reported no signal | `EXTEND generate-ideation-guidance` | The capability already owns the ideation guidance's embedded schemas — the phase ladder, the term schema, the seed schema, and the scoping rules. The persona schema is the same kind of content and fixes the pointer that broke the observed session. |

## Out of Scope

- No new files are shipped into consumer projects. The change does not
  materialize embedded assets next to skills, and does not change what
  `givn skills sync` installs.
- Rootless relative references — paths written relative to a parent stated in
  the same passage (for example a seed file inside the topic state) — are not
  graded.
- References into the consumer's own implementation layout (source, test, or
  feature files, viewport evidence) are not graded; they are project-owned
  conventions, not Givn-owned guidance paths.
- The content of `docs/use-case-model.md` itself does not change.
- The permanent use-case contracts and the lifecycle order do not change.

## Open Questions

### D1 — Keep or drop the use-case model pointers?

- Question: The generated guidance currently names `docs/use-case-model.md`
  as the durable reference for the use-case rules. The consumer project does
  not have that file. Should the guidance (a) keep the pointer, explicitly
  identified as a Givn-repository source, or (b) drop the pointer where the
  rule text already stands on its own?
- Options:
  1. **Keep, marked** — the pointer stays with an explicit "lives in the
     Givn repository" identification; gain: the maintainer and the curious
     reader can still reach the worked example; cost: one more wording
     pattern and marker for the check to recognise; in practice: the review
     instruction reads "the durable reference lives in the Givn repository,
     `docs/use-case-model.md`".
  2. **Drop** — the pointer is removed where the rules are already inlined;
     gain: no external reference at all, nothing to mark; cost: the
     guidance no longer records where the canonical worked example lives; in
     practice: the review instruction lists the ten criteria without naming
     a file.
- Disposition: carried — the design decides per reference (ideation, spec,
  specs instruction, review instruction). The fixed constraint: no
  use-case-model reference may read as a project-local file, and if kept it
  carries the explicit Givn-repository marker the check recognises.

### D2 — Graded surface: installed guidance only, or printed instructions too?

- Question: Does the check grade only the installed skills and commands, or
  also the instruction text `givn instructions <artifact>` prints?
- Options:
  1. **Both** — the check covers every guidance surface; gain: the same
     dead pointer appears in the specs and review instructions, so
     grading the printed text is the only way to catch it; cost: the check
     fixture needs an active change to request artifact instructions; in
     practice: asking for the specs instruction in a fresh project exposes
     its `docs/use-case-model.md` reference.
  2. **Installed only** — only files on disk are graded; gain: a smaller
     fixture; cost: the printed instructions can keep shipping dead
     references while the check stays green.
- Disposition: answered — the operator approved the check "for all
  referenced files" (2026-10-11), after the dead pointers were shown in the
  instruction output. The acceptance suite grades both surfaces.

### D3 — Rootless references

- Question: Should the check also grade references written relative to a
  stated parent, such as `changes/<change-id>.md` inside the ideation topic
  section?
- Disposition: out of scope — resolving such a reference needs the
  surrounding passage as its base, and every known dead reference is rooted.
  If evidence shows a rootless dead reference, a later change roots the
  wording and widens the check.
