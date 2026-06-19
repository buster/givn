# Instruction: arc42-docs

Maintain the project's durable arc42 architecture documentation.

These documents live under `docs/arc42/` (or the configured `arc42.output_dir`).
They are **project-level and evolve across changes** — they are not archived with
the change. Your task is to create missing chapters and update affected ones.

## What to do

**If a chapter file does not exist:** create it from the corresponding template
(see `assets/addons/arc42/templates/`). Fill in what is known from the project
context; leave placeholder text for sections that cannot be answered yet.

**If a chapter file already exists:** update only the sections affected by the
current change. Do not rewrite unrelated sections. Prefer surgical edits that
preserve existing content.

## Which chapters to touch

Base the decision on the current change's `proposal.md`, `specs/`, and
`design.md`:

| If the change introduces or modifies... | Update chapter(s) |
|---|---|
| Goals, stakeholders, quality attributes | 01 introduction-and-goals, 10 quality-requirements |
| Constraints (legal, tech, org) | 02 architecture-constraints |
| External systems, interfaces, context | 03 context-and-scope |
| Major technical strategy or approach | 04 solution-strategy |
| New building blocks, modules, components | 05 building-block-view |
| New runtime flows, sequences | 06 runtime-view |
| Deployment changes | 07 deployment-view |
| Cross-cutting concepts (error handling, security, config) | 08 crosscutting-concepts |
| Architecture decisions (tradeoffs, ADRs) | 09 architecture-decisions |
| New quality scenarios | 10 quality-requirements |
| New risks or technical debt | 11 risks-and-technical-debt |
| New domain terms | 12 glossary |

## Output

1. Write or update the chapter files under `docs/arc42/` (one file per chapter).
2. Update (or create) `docs/arc42/README.md` as the index.
3. Write a brief `arc42.md` in the change directory (`givn/changes/<id>/arc42.md`)
   recording which chapters were created or updated and a one-line summary of
   each change. This marker file is what givn tracks as the artifact's completion.

## Rules

- Write in clear, concise technical prose. Arc42 docs are for humans, not machines.
- Do not leak implementation detail into specification-layer sections (chapters 1–3).
- Chapter 9 (architecture decisions) uses ADR format: context → decision → rationale → consequences.
- Do not delete existing content unless it is factually wrong.
- The marker file (`arc42.md` in the change dir) must end with a `## Status` section
  containing either `STATUS: DONE` or `STATUS: PARTIAL` on its own line.

**Artifact complete when:** `arc42.md` in the change directory exists and contains
`STATUS: DONE`.
