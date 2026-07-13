# Instruction: arc42-docs

Maintain the project's durable arc42 architecture documentation.

These documents live under `docs/arc42/` (or the configured `arc42.output_dir`).
They are **project-level and evolve across changes** — they are not archived with
the change. Your task is to create missing chapters and update affected ones.

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
3. Write `arc42.md` in the change directory (`givn/changes/<id>/arc42.md`)
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
- Any diagram (component, sequence, context, quality tree, flow) MUST be
  Mermaid (` ```mermaid ` fenced code block), never ASCII art. A plain
  directory/file-tree listing is not a diagram and may stay as plain text.
- Do not leak implementation detail into specification-layer sections (chapters 1–3).
- Chapter 9 (architecture decisions) uses MADR format (Markdown Any Decision Records).
  The chapter file contains the full MADR template as a comment block — copy it for
  each new decision. One decision = one section with its own status/date/decision-makers
  front-matter, considered options, decision outcome, consequences, confirmation, and
  pros/cons.
- Do not delete existing content unless it is factually wrong.
- The marker file (`arc42.md` in the change dir) must end with a `## Status` section
  containing either `STATUS: DONE` or `STATUS: PARTIAL` on its own line.

**Artifact complete when:** `arc42.md` in the change directory exists and contains
`STATUS: DONE`.
