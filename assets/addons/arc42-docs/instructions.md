# Instruction: arc42-docs

Maintain the project's durable arc42 architecture documentation.

These documents live under the fixed `docs/arc42/` path in the current binary.
They are **project-level and evolve across changes** — they are not archived with
the change. Your task is to create missing chapters and update affected ones.

## Diagram policy

Any diagram in any arc42 chapter (component, context, sequence, quality tree,
flow, deployment topology) MUST be a Mermaid fenced code block
(` ```mermaid `). ASCII art — inline box-drawing characters, pre-formatted
text diagrams — is not permitted. A plain directory/file-tree listing is not a
diagram and may stay as plain text.

## What to do

**First: assess impact against every chapter, not just the obvious ones.**
Read the current change's `proposal.md`, `specs/`, and `design.md` in full.
Then walk **all 12 rows** of the table below, in order, and make an explicit
yes/no decision for each one — do not stop once you've found two or three
obviously-affected chapters. A change that touches building blocks almost
always also touches runtime-view (new flows) and often context-and-scope
(new user-facing surface) or risks-and-technical-debt (every non-trivial ADR
"Bad, because..." consequence is a risk or a piece of technical debt by
definition — if chapter 09 gets a new ADR, chapter 11 needs at least a line).
Skimming for the first plausible match and stopping there is the failure
mode this instruction exists to prevent.

**If, after walking all 12 rows, none are affected:** write `arc42.md` in the
change directory with `STATUS: DONE`, the full 12-row assessment table (see
`## Output` below) with every row marked "No" and a one-line reason, and an
overall one-line note (e.g. "Pure tooling change — no architectural impact.").
You are done.

**If chapters are affected:**

- **If a chapter file does not exist:** it was scaffolded as a blank template
  when arc42 was enabled (`givn addons enable arc42`). Fill it in from the
  project context; leave placeholder text for sections that cannot be answered
  yet.
- **If a chapter file already exists:** update only the sections affected by
  this change. Do not rewrite unrelated sections. Prefer surgical edits that
  preserve existing content.

## Architecture Decision Records

### Unit and storage

One architecturally significant decision gets exactly one Architecture Decision
Record (ADR), and one ADR records exactly one decision. Do not combine several
decisions in one ADR file or split one decision across several ADR files.

The default storage contract is:

- The ADR body is a separate Markdown file at
  `docs/arc42/adr/ADR-NNNN-short-slug.md`.
- `docs/arc42/09-architecture-decisions.md` is the chapter-09 register/index.
  It contains links, status, date, and short register information; it is not a
  container for the bodies of multiple ADRs and must not contain a sequence of
  ADR sections.
- `docs/arc42/adr/README.md` is the index for the ADR directory. Keep it in
  sync with the register and the individual files.
- Historical ADRs may be placed at
  `docs/arc42/adr/archive/ADR-NNNN-short-slug.md`. Preserve the ADR number and
  links in both indexes when moving a record there. Do not place an ADR body in
  `givn/archive/<change>/`; that is the change archive, not the architecture
  decision archive.

Use the paths above verbatim. The current binary does not implement an
`arc42.output_dir` override.

Create the ADR directory, individual file, and index entries manually as
needed. This addon guidance does not provide or claim automatic scaffolding for
individual ADRs. Do not state that enabling the addon creates ADR files.

### Qualification gate

Before writing an ADR or adding it to either index, run the Qualification
Checklist below. The decision passes only when at least one box is checked
(`[x]`) and has a clear, decision-specific justification. If none of the boxes
can be checked with a clear justification, do not create an ADR. Handle the
matter as regular development, a team convention, or an inline comment instead.

Use all four checklist items in the ADR's `## Qualification` section. Each item
must have a concrete, decision-specific reason, whether checked or unchecked,
before the file is considered complete. The gate passes only when at least one
item is checked (`[x]`) with a clear reason:

- [ ] **Hard to reverse** - Reverting later would cost significant effort
  because: _specific reason_.
- [ ] **Shapes quality attributes** - It materially affects
  [performance / scalability / security / availability / maintainability /
  testability] because: _specific reason_.
- [ ] **Structural** - It defines a boundary, contract, or dependency
  direction because: _specific reason_.
- [ ] **Must be shared** - Developers unfamiliar with it would systematically
  violate the architecture because: _specific reason_.

An illustrative minimum passing result is one checked item with a concrete
reason, for example:

- [x] **Structural** - This decision fixes the dependency direction between
  the ingestion and persistence components for all future features.

The example is not a qualification for a real project decision. Replace it
with the actual reason. A checkbox that only repeats the decision title is not
justification and does not pass the gate.

### Qualification criteria

The decision qualifies when it meets one or more of these criteria. Evaluate
them in this order of priority:

1. **Hard to reverse (primary criterion).** Correcting the decision later
   would require significant effort, such as touching many parts of the
   codebase, migrating data, breaking an existing interface, or forcing the
   team to relearn tooling or patterns. Ask: "How expensive would it be to
   undo this in six months?"
2. **Shapes quality attributes.** The decision directly and materially
   influences a lasting non-functional constraint or enabler for performance,
   scalability, security, availability, maintainability, or testability. The
   effect must be broader than one local implementation. Ask: "Does this
   decision set a ceiling or a floor on a key quality attribute?"
3. **Structural.** The decision defines module or service boundaries, assigns
   responsibilities, establishes dependency directions, or fixes a public
   interface or contract such as an API schema, event format, or plugin
   protocol. Internal restructuring of one class or private package layout
   does not qualify. Ask: "Does this decision define who owns what and how
   parts talk to each other?"
4. **Must be shared.** Every developer joining the project must know and
   respect the decision before contributing effectively. Ignorance would
   cause systematic violations of a fundamental architecture rule. Ask: "Would
   a new developer break something fundamental without knowing this?"

These are not qualification criteria by themselves: system relevance,
implementation effort, a long discussion, or use of a known pattern. "It
affects the system" is too broad to qualify a decision in a microservice or
CLI, and effort is not architectural significance. A known pattern still needs
to satisfy at least one of the four criteria above.

The following are generally not ADRs when they have no qualifying impact:

- choosing a logging library;
- internal variable or method naming;
- formatting rules such as tabs versus spaces;
- refactoring the internals of one module without an interface change;
- choosing an interchangeable HTTP client package without quality-attribute
  implications.

System-wide error handling is borderline. Record it as an ADR only when it is
an enforced, cross-cutting contract that all relevant components must obey. Do
not record per-endpoint error handling with no shared contract as an ADR.

Examples that normally pass the gate include choosing REST versus gRPC as a
service API protocol, adopting Event Sourcing instead of CRUD, delegating
authentication through OAuth2/JWT, establishing a system-wide retry and
circuit-breaker strategy, defining a layering model, choosing synchronous
versus asynchronous inter-service communication, adding a plugin system, and
choosing an ORM or database access strategy. An ORM choice almost always
warrants an ADR when its lifecycle assumptions affect maintainability,
debuggability, query expressiveness, or the domain model, especially for
event-driven systems or complex aggregates.

### MADR file contract

Every ADR file uses MADR (Markdown Any Decision Records) and starts with this
frontmatter:

```yaml
---
status: proposed
date: YYYY-MM-DD
decision-makers: list everyone involved
consulted: list subject-matter experts consulted
informed: list everyone kept up-to-date
---
```

The allowed status values are exactly `proposed`, `rejected`, `accepted`,
`deprecated`, and `superseded by ADR-NNNN`. A superseding status must contain a
concrete existing ADR ID, for example `superseded by ADR-027` or
`superseded by ADR-0042`, and that replacement record must be identifiable in
the indexes. `ADR-NNNN` is only template notation; it must not remain as a
placeholder in a completed ADR.

The frontmatter must contain real values: an actual ISO date and concrete
decision-makers, consulted people, and informed people. Use an explicit value
such as `none` when a list is empty. A completed ADR contains no template
placeholders, ellipses, generic option names, or unfilled prompts.

Use every MADR section below. Do not omit a section because the decision seems
small:

~~~markdown
---
status: proposed
date: YYYY-MM-DD
decision-makers: list everyone involved
consulted: list subject-matter experts consulted
informed: list everyone kept up-to-date
---

# Short title representative of solved problem and found solution

## Qualification

- [ ] **Hard to reverse** - Reverting later would cost significant effort because: _..._
- [ ] **Shapes quality attributes** - It materially affects [quality attribute] because: _..._
- [ ] **Structural** - It defines a boundary, contract, or dependency direction because: _..._
- [ ] **Must be shared** - Developers unfamiliar with it would systematically violate the architecture because: _..._

## Context and Problem Statement

Describe the context and problem in 2-4 sentences. Make the scope explicit by
pointing to structural elements such as components and connectors.

## Decision Drivers

- desired software quality, constraint, or concern
- ...

## Considered Options

- Option title 1
- Option title 2

## Decision Outcome

Chosen option: "Option title N", because justification.

### Consequences

- Good, because project-specific positive consequence
- Bad, because project-specific negative consequence

### Confirmation

Name the concrete test type, fitness function, linting rule, or review
checkpoint that confirms implementation and compliance.

## Pros and Cons of the Options

### Option title 1

- Good, because argument
- Neutral, because argument
- Bad, because argument

### Option title 2

- Good, because argument
- Bad, because argument

## More Information

Additional evidence, team agreement, links to related decisions, or resources.
~~~

The code block is a template only. Replace every prompt before completing an
ADR. The Context and Problem Statement is 2-4 sentences, Considered Options
lists real, distinct options rather than strawmen, Decision Outcome is brief
and gives the chosen option plus its single most important reason, and
Consequences contain only non-obvious or project-specific effects. Confirmation
must be concrete: name an automated test, fitness function, lint rule, or
specific review checkpoint, not merely "we will review it".

## Which chapters to touch

Base the decision on the current change's `proposal.md`, `specs/`, and
`design.md`. Evaluate every row — a "No" is only valid if you actually
checked the change's scope against that row and found nothing, not because
you stopped looking after an earlier row matched:

| # | If the change introduces or modifies... | Chapter(s) |
|---|---|---|
| 1 | Goals, stakeholders, quality attributes | 01 introduction-and-goals, 10 quality-requirements |
| 2 | Constraints (legal, tech, org) | 02 architecture-constraints |
| 3 | External systems, interfaces, context, new user-facing surface | 03 context-and-scope |
| 4 | Major technical strategy or approach | 04 solution-strategy |
| 5 | New building blocks, modules, components | 05 building-block-view |
| 6 | New runtime flows, sequences | 06 runtime-view |
| 7 | Deployment changes | 07 deployment-view |
| 8 | Cross-cutting concepts (error handling, security, config) | 08 crosscutting-concepts |
| 9 | Architecture decisions (tradeoffs, ADRs) | 09 architecture-decisions |
| 10 | New quality scenarios | 10 quality-requirements |
| 11 | New risks or technical debt (including every "Bad, because..." consequence recorded against a new ADR in chapter 09) | 11 risks-and-technical-debt |
| 12 | New domain terms | 12 glossary |

Rows are not mutually exclusive: a single change routinely triggers rows 3,
5, 6, 9, and 11 together (new user-facing capability -> new context, new
modules, new flow, a tradeoff decision, and the consequence of that
tradeoff). Treat correlated rows as a prompt to double-check, not evidence
you can skip re-reading the table for each one.

## Output

1. Write or update the chapter files under `docs/arc42/` (one file per chapter).
2. Update (or create) `docs/arc42/README.md` as the index.
3. For an architectural decision, update the chapter-09 register and
   `docs/arc42/adr/README.md`, then create the individual ADR file at the
   prescribed path. Do not paste its body into chapter 09. Use the fixed
   `docs/arc42/` output path used by the current binary.
4. Write `arc42.md` in the change directory (`givn/changes/<id>/arc42.md`)
   containing the **full 12-row assessment table** from the template
   (`givn eject arc42-docs:template` to see it, or `docs/arc42/README.md`
   for the chapter list) — every row gets an explicit Yes/No and a one-line
   reason, not just the rows you decided to update. Rows marked "Yes" also
   get a one-line summary of what changed. This marker file is what givn
   tracks as the artifact's completion, and it is what design-review
   cross-checks against its own independent read of the table — a missing
   row, or a "No" with no reason, is treated as unverifiable and blocks
   design-review sign-off.

## Rules

- Write in clear, concise technical prose. Arc42 docs are for humans, not machines.
- Diagram policy: see the `## Diagram policy` section above. No diagram may use
  ASCII art.
- Do not leak implementation detail into specification-layer sections (chapters 1–3).
- Chapter 9 (architecture decisions) is a register/index, not an ADR container.
  Each decision has exactly one separate MADR file under `docs/arc42/adr/`, and
  the chapter-09 register links to it. Each ADR file has its own
  status/date/decision-makers frontmatter, Qualification, Context and Problem
  Statement, Decision Drivers, Considered Options, Decision Outcome,
  Consequences, Confirmation, Pros and Cons of the Options, and More
  Information sections.
- No ADR file may be created before the Qualification gate passes with at least
  one clearly justified checked criterion. Keep the status and concrete
  supersession-ID contract from `## MADR file contract`.
- Do not delete existing content unless it is factually wrong.
- The marker file (`arc42.md` in the change dir) must end with a `## Status` section
  containing either `STATUS: DONE` or `STATUS: PARTIAL` on its own line.

**Artifact complete when:** `arc42.md` in the change directory exists and contains
`STATUS: DONE`.
