# Design: validate-generated-references

## Overview

The change adds one consumer-view guard and the content fixes that make the
shipped guidance pass it. The guard is test support, not production code:
after `givn init`/`givn skills sync`, the acceptance suite extracts every
rooted file reference from the generated skills, commands, and printed
instructions and fails any reference that resolves to nothing in the
consumer project and is not owned by a named creation step. The content
fixes: the persona schema moves inline into the ideation guidance, the
characterization guidance requests the reverse review through the CLI in
both its surfaces, every `docs/use-case-model.md` pointer is dropped because
the rules are already inlined at each site, and the one example location that
is not a project claim is reworded.

The WHAT lives in `proposal.md`; the observable behaviour is locked by the
two delta features under `specs/fragments/`. The consumer surface is Markdown
guidance and CLI stdout, so the scenarios drive the real `givn init`,
`givn skills sync`, and `givn instructions` invocations and assert on the
rendered text.

## Decisions

- Decides D1: every `docs/use-case-model.md` pointer is **dropped**, not
  reworded as a repository-marked source. At all four sites the surrounding
  text already carries the complete rule — the eight scoping rules in the
  ideation skill, the placement test in the specs instruction the spec skill
  mandates, and the ten rejection criteria in the review instruction — and a
  path a consumer can never open is not part of its authoring guidance. This
  removes the need for a repository-marker reference class in the check;
  realized in the four assets below.
- Decides D2: the check grades both installed guidance (skills and commands)
  and the printed instructions — realized in the delta scenarios and the
  steps that build the two corpora.
- Decides D3: rootless relative references stay ungraded; the candidate
  extractor accepts only backticked references rooted at a known top-level
  directory — realized in the extraction rule below.
- Decides (design): the repository persona template
  (`assets/templates/persona.md`) stays. The schema is carried inline in the
  guidance, which is the authoring surface; deleting the asset would break
  the permanent scenario that inspects it, because that scenario still runs
  in the pre-archive full suite until a later change retires both together.
  The template remains a repository-side mirror pinned by its own scenario.
- Decides (design): the check is test support in
  `tests/common/reference_check.rs`, consumed only by the acceptance suite.
  Rejected: a `givn check` subcommand — no consumer needs to grade guidance
  Givn did not generate, and the guard must run against the shipped assets in
  the suite that owns them.
- Decides (design): the workflow-created table is narrow — a root template
  plus named leaf templates, each row naming its creator. A bare-prefix
  allowlist (`givn/` or `docs/`) is rejected: it would swallow typos of the
  same class this change exists to catch.
- Decides (design): a reference to a skill file in another agent target
  resolves by skill name across the configured targets, because the guidance
  itself says "or your harness's equivalent path"; the text stays.
- Decides (design): the `docs/adr/` example in the grill-with-docs command is
  reworded rather than allowlisted; an example location is not a project
  claim.
- Decides (design): the printed corpus covers every instruction Givn prints —
  the manifest artifacts plus `review-reverse`, `specs-reverse`,
  `tasks-reverse`, and `dialog` — so the route the characterization fix
  creates is itself graded. A manifest-only corpus would leave
  `givn instructions review-reverse` ungraded and could let the dead pointer
  return unnoticed.
- Decides (design): fail-closed behaviour is proven for both collection
  paths — one negative scenario per corpus — because the release note claims
  both surfaces and the installed corpus is the one the original defect hit.

## Exact file changes

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Replace the persona-template pointer with the inline persona schema; drop the `docs/use-case-model.md` clause. |
| `assets/skills/givn-spec/SKILL.md.tmpl` | Drop the `docs/use-case-model.md` clause. |
| `assets/skills/givn-characterize/SKILL.md.tmpl` | Request the reverse review through `givn instructions review-reverse` (one occurrence). |
| `assets/instructions/specs.md` | Drop the `docs/use-case-model.md` clause. |
| `assets/instructions/review.md` | Drop the durable-reference sentence. |
| `assets/commands/givn-characterize.md` | Request the reverse review through `givn instructions review-reverse` (two occurrences). |
| `assets/commands/grill-with-docs.md` | Reword the ADR example without the `docs/adr/` path. |
| `tests/common/reference_check.rs` | New: candidate extraction, resolution tables, resolvers, findings. |
| `tests/common/mod.rs` | Register the new module. |
| `tests/steps/instruction_integrity_steps.rs` | Add the check, override, corpus, failure, and characterization steps. |
| `docs/arc42/08-crosscutting-concepts.md` | Amend 8.7 (consumer-view resolution rule) and 8.22 (schema home). |
| `docs/arc42/10-quality-requirements.md` | Add quality scenario Q-97 and the matching 10.3 verification signal. |
| `docs/arc42/11-risks-and-technical-debt.md` | Add deliberate technical debt D-20 (graded-surface limits). |
| `docs/arc42/12-glossary.md` | Add `Consumer project` and `Generated guidance` rows. |

### New wording, for the implementer

**Ideation guidance** — the paragraph replacing the pointer. The replacement
contains a nested fenced block:

````text
Persona files use this schema — a stable core, an append-only running
record of dated entries per review or topic stance, and a dated
topic-stances appendix:

```markdown
# Persona: <name>

## Stable core

- Role: <one line>
- Authority boundary: <what it may do or decide>
- Evaluation capability: <what it can judge>
- Own vocabulary: <terms it uses or refuses>
- Success criterion: <what success means for it>

## Running record

Dated entries per review or topic stance; append-only, never rewriting
earlier entries.

- <YYYY-MM-DD> — <entry>

## Topic stances

Topic-specific demands, dated, never in the stable core.

- <YYYY-MM-DD> — <topic>: <stance>
```

A reused persona's topic-local file contains a header line plus new record
entries only — never a biography copy:
````

**Ideation guidance, Use Case Definition paragraph** becomes:

```markdown
A sequence of events that culminates in value is a use case. Size each one
with these falsifiable rules:
```

**Spec skill**, canonical interaction policy paragraph:

```markdown
Apply the capability placement test before routing a new capability.
```

(the mandated instruction already carries the test; the skill only
choreographs when to read it).

**Characterize skill**, reverse review epilogue:

```markdown
- Applies reverse-mode rules (`givn instructions review-reverse`):
```

**Specs instruction**, capability placement paragraph:

```markdown
Before recording a `NEW in <usecase-id>` capability, apply the placement
test: which actor goal does completing this capability advance? Channel is
never an answer — a browser or CLI variant of an action lives under the
action's goal. Capability names are goal-shaped, never component- or
channel-shaped (no server, backend, store, service, panel, browser, or CLI
names).
```

**Review instruction**, use-case scope criteria introduction:

```markdown
A review fails when any of these holds. Each criterion names the observable
document section that proves it.
```

**Characterize command**, two replacements:

```markdown
Read the reverse-review instruction (`givn instructions review-reverse`) in
full (passed to the review subagent).
```

```markdown
<paste the full content of `givn instructions review-reverse`>
```

**Grill-with-docs command**:

```markdown
- Any architecture decision records or the project's decisions log
```

**Chapter 8.22**, schema sentence:

```markdown
The ideation guidance carries the persona schema: a stable core (role,
authority boundary, evaluation capability, own vocabulary, success
criterion) and an append-only running record with a dated topic-stances
appendix; topic-specific demands never enter the stable core.
```

**Chapter 8.7** gains one paragraph after the generated-guidance paragraph:

```markdown
Generated guidance is written for the consumer project. Every file
reference it carries must resolve in the project that receives it: a file
the project has, a location a named creation step creates, a project-owned
destination the guidance itself defines, or a skill resolved by name across
the configured agent targets. A reference into the Givn distribution
(`assets/…`, a repository-only document) is a defect; the acceptance suite
grades every generated skill, command, and printed instruction against this
rule.
```

## The reference check

Implementation: `tests/common/reference_check.rs`.

**Candidate extraction.** For each `(label, text)` source, scan backticked
spans (`` `…` `` on one line). A span is a candidate when it contains `/`, has
no whitespace, no `://`, no `=`, does not start with `/`, and its first
segment is one of `givn`, `docs`, `assets`, `.opencode`, `.agents`, `.claude`,
`.omp`. Record the label and the 1-based line. The trailing `/` is dropped
before matching, so a directory reference resolves whether or not it ends in
a slash. Candidates whose trailing segments are globs (`*`, `**`) are
trimmed to their longest glob-free prefix before resolution. Lines inside
fenced examples that are not backticked are outside the graded surface
(recorded as deliberate debt D-20).

**Resolution classes**, first match wins:

1. **Exists** — the normalized candidate resolves under the project root
   (file or directory).
2. **Agent-target skill file** — the candidate matches
   `<target>/skills/<skill>/SKILL.md` for a known agent target; it resolves
   when any configured target directory contains that skill file.
3. **Workflow-created** — the candidate matches a row of the table below;
   each row names the creator. Segment templates: a segment containing
   `<…>` matches any single non-empty segment (prefix/suffix around the
   placeholder is matched), and a trailing `**` matches zero or more
   remaining segments. The root alone matches the candidate with or without
   a trailing slash (so `givn/personas/`, `givn/changes/<change-id>/`, and
   `givn/ideation/<topic-id>/` resolve through their rows before
   `givn init` creates anything).
4. **Project-owned destination** — the candidate matches the rule-owned
   table below.
5. Otherwise: an **unresolved finding**.

**Workflow-created table.**

| Root template | Leaves | Creator |
|---|---|---|
| `givn/changes/<change-id>` | (the directory), `proposal.md`, `design.md`, `design-review.md`, `tasks.md`, `review.md`, `arc42.md`, `specs/**`, `evidence/**` | `givn new` (directory and proposal), the design, design-review, tasks, review, and arc42-docs steps, the spec archive merge, the visual review capture |
| `givn/archive/<change-id>` | same leaves | `givn archive` |
| `givn/specs/<usecase-id>` | (the directory), `usecase.md`, `<capability>.feature` | the spec archive merge; `givn init` for the directory |
| `givn/specs` | `**` | the spec archive merge; `givn init` for the directory |
| `givn/ideation/<topic-id>` | (the directory), `session.md`, `frame.md`, `eventstorming.md`, `use-case-definitions.md`, `domain-terms.md`, `questions.md`, `use-cases/**`, `personas/**`, `changes/**` | the ideation workflow; the handoff for `changes/**` |
| `givn/personas` | `<id>.md` | persona promotion |
| `docs/ideation` | `<topic-id>.md` | the ideation handoff |

**Rule-owned table.**

| Candidate pattern | Owner |
|---|---|
| `docs/design`, `docs/design/**` | the design step of a visual project, which records and maintains the durable design system |
| `docs/arc42/adr`, `docs/arc42/adr/**` | the arc42-docs ADR workflow, which writes the register entries and the individual MADR files (including the archive directory) |

**Printed corpus.** The step runs `givn graph --json` and requests
`givn instructions <id> --change <id>` for every artifact id, plus
`givn instructions review-reverse`, `specs-reverse`, `tasks-reverse`, and
`dialog`. Each output is labeled (`instructions/<id>`) as a source.

**Findings** carry the reference text, the source label, and the line so a
failure names what to fix. The negative scenarios plant
`givn/reference-that-resolves-nowhere.md` — one in a specs instruction
override, one in a `givn-ideation` skill override — and each requires at
least one finding naming that path. Together with the positive scenarios
(zero findings on the shipped corpus) the pair proves the check fails
closed and does not report everything: an always-reporting checker fails
the positives, a never-reporting checker fails the negatives.

## Step definitions

One file per capability; the change touches two capabilities.

`instruction-integrity` (`tests/steps/instruction_integrity_steps.rs`) gains:

| Step | Behaviour |
|---|---|
| `the author requests every instruction Givn prints` (When) | Runs `givn graph --json`, requests every artifact id plus the four non-artifact instruction ids, concatenates the labeled stdout into `additional_output`. |
| `every guidance reference resolves in the consumer project or is owned by a named creation step` (Then) | Builds the installed-guidance corpus from the configured targets of the fixture project and asserts no unresolved findings. |
| `every printed guidance reference resolves in the consumer project or is owned by a named creation step` (Then) | Runs the same classifier over `additional_output` and asserts no unresolved findings. |
| `a givn-ideation skill override that names the missing file "<path>"` (Given, regex) | Writes `givn/skills/givn-ideation/SKILL.md.tmpl` with valid frontmatter (name and description) and one body line naming `<path>` in backticks. |
| `a specs instruction override that names the missing file "<path>"` (Given, regex) | Writes `givn/artifacts/specs/instructions.md` whose body is `See \`<path>\` for details.` |
| `the installed reference check fails and names "<path>"` (Then, regex) | Runs the classifier over the fixture's installed guidance and asserts a finding for exactly `<path>`. |
| `the printed reference check fails and names "<path>"` (Then, regex) | Runs the classifier over `additional_output` and asserts a finding for exactly `<path>`. |
| `the generated givn-characterize guidance requests \`givn instructions review-reverse\`` (Then) | Reads the configured targets' `skills/givn-characterize/SKILL.md` and asserts the CLI request appears. |
| `the generated givn-characterize command requests \`givn instructions review-reverse\`` (Then) | Reads the command-owning targets' `commands/givn-characterize.md` and asserts the CLI request appears. |

The retired persona-template steps stay in the file: the permanent scenario
that inspects the repository template still runs until a later change retires
it, so its bindings must keep matching.

`generate-ideation-guidance` needs no new step: the scenario reuses the
generic `the generated givn-([a-z-]+) guidance contains "…"` step
(`tests/steps/prose_consolidation_steps.rs`) and the two existing fixtures
(addons on via `givn init`; addons off via `givn init --no-addons`).

`tests/common/reference_check.rs` exposes `GuidanceSource`,
`installed_guidance(project_root)`, `unresolved_references(project_root,
sources)`, and `Finding`. `installed_guidance` walks the configured targets
(`givn::config::overlay::load_skills_targets`) and returns every
`skills/*/SKILL.md` and `commands/*.md` file. No production code changes.

## Runner and strict mode

- Verify command (`givn/commands.yaml`): `./run-tests.sh`; E2E smoke:
  `./run-tests.sh --e2e`.
- Strict mode: `tests/features_runner.rs` calls `.fail_on_skipped()` and exits
  non-zero when `stats.failed > 0 || stats.skipped > 0`, so undefined or
  pending steps fail the run. `@wip` marks not-yet-implemented steps;
  `givn lint` reports them, and archive blocks while a non-removed `@wip`
  scenario exists in the delta.
- Change-scope run:
  `GIVN_FEATURES=givn/changes/validate-generated-references/specs ./run-tests.sh`
- Single scenario:
  `GIVN_FEATURES=givn/changes/validate-generated-references/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::An undeclared reference in an installed skill fails the check"`

## Use-case traceability

| Use-case element | Technical realization | E2E evidence |
|---|---|---|
| Capability `instruction-integrity` (fragment `support`): guidance remains complete and machine-readable | The consumer-view check (extraction, resolution tables, findings) plus the dropped dead pointers in the four assets and the CLI route in both characterization surfaces | Existing `@e2e` "An agent requests machine-readable artifact guidance" (real `givn instructions`); the delta scenarios assert the property over the rendered corpus |
| Capability `generate-ideation-guidance` (fragment `support`): the ideation skill ships self-contained | The persona schema inlined in `assets/skills/givn-ideation/SKILL.md.tmpl` | Existing `@e2e` "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" |
| Rule: generated guidance only points at files the consumer has or a named creation step creates | Candidate extraction with known roots; workflow-created table naming each creator; rule-owned table; fail-closed findings on both corpora | Delta scenarios "The generated guidance references resolve …" (addons on and off), "The printed instructions reference only resolvable files", and the two planted-reference negatives |
| Confirmed Personas: `agent--technical-role` (follows the references), `developer--primary-user` (owns the receiving project), `givn-maintainer--owner` (owns the guidance corpus and the suite) | The guidance text and the check address each role in its own terms | The capability has CLI text output, so Personas are named |

Actors in the scenarios (`the user`, `the author`) stay distinct from
Personas; the Persona row records review context only.

## Visual Design Contract

- Interface: terminal.
- Viewports: not applicable — nothing renders HTML, CSS, or JavaScript;
  `givn skills sync` and `givn instructions` write or print Markdown text, so
  a browser driver cannot apply. The opposite classification was tested: a
  screen that renders HTML, CSS, or JavaScript in a browser is a Web UI, and
  no scenario here does.
- Surfaces covered: the generated skill and command files and the printed
  instruction text consumed in a terminal.
- Evidence: no new evidence is required because the change adds no delta
  `@e2e` scenario; the existing `@e2e` transcripts remain and are unchanged.

## Interaction Coverage Matrix

The change adds no new consumer action, so no new inventory entry exists. The
two `instruction-integrity` rows are the fragment inventory
(`givn/specs/fragments/fragment.md`); the `generate-ideation-guidance` row
annotates the action its existing `@e2e` scenario covers.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `instruction-integrity`: request effective instructions by id (artifact, general policy, JSON, and change context are variants) | "An agent requests machine-readable artifact guidance" | CLI | Real `givn instructions` subprocess; stdout and JSON asserted |
| `instruction-integrity`: request normal session-start instructions | "Author starts a session with the universal dialog obligation" | CLI | Real `givn instructions` subprocess; stdout asserted |
| `generate-ideation-guidance`: synchronize skills for every configured target (no fragment inventory row exists; the capability's action is the sync) | "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" | CLI | Real `givn skills sync` subprocess; rendered files asserted |

## Version freshness

No language, framework, library, or image version is introduced or changed.
The change edits embedded text assets and test-support Rust; the lockfile is
untouched.

## Local runnability & digital twins

No external or third-party dependency is touched. Local run and test command:
`./run-tests.sh`; E2E smoke via the configured `verify.e2e_command`. No digital
twin is required.

## Coverage process boundaries

N/A — the change adds no instrumented process and no measurement command; it
adds test-support steps that drive the existing binary.

## Arc42 impact

Chapter 8 (crosscutting concepts): 8.7 Skill and Rule Generation gains the
consumer-view resolution rule; 8.22 Persona Identity and Reuse repoints the
schema sentence at the ideation guidance. Chapter 10 (quality requirements):
Q-97 records the new guidance-integrity property and 10.3 names the check as
its verification signal. Chapter 11 (risks and technical debt): D-20 records
the graded-surface limits and the table-upkeep obligation. Chapter 12
(glossary): the two new rows `Consumer project` and `Generated guidance` are
recorded and review-verified. The change's `arc42.md` carries the full
12-row assessment and the ADR verdict.

## Internal Primitives

None — no production module, struct, or function is added or changed. The
only new Rust is test support under `tests/common/` and the new steps.
