# Proposal: install-extras

## Problem / Opportunity

Givn ships a rich set of workflow skills — propose, spec, design, implement,
review, archive — that guide users through the change lifecycle. But there is
no built-in mechanism for **project-level productivity skills** that are
independent of any specific change.

Teams want optional tools they can opt into without affecting their workflow
graph:
- A relentless design-grilling interview that catches weak assumptions before
  code is written.
- A domain-modelling assistant that builds and maintains a project glossary
  and architecture decision records, challenging fuzzy language.
- An always-available skill that injects development principles — KISS, YAGNI,
  testability, clean code over documentation — into every agent session.

Today, users who want these must hand-write skill files, copy them between
projects, and remember to keep them in sync. There is no `givn` command to
install, sync, or remove them. The `givn skills sync` command only generates
core workflow skills — it has no notion of optional extras.

## Proposed Solution

Add an **extras** system: a curated set of optional, project-level skills and
slash commands that users install into any givn project. Extras are independent
of the change lifecycle — they don't appear in `givn status`, don't gate
archive, and don't modify the artifact graph. They are pure productivity
enhancements for AI-assisted development.

### Four extras ship initially

**Commands** (slash-commandable in the agent):

- `/grill-me` — A relentless design interview. The agent asks one probing
  question at a time about edge cases, assumptions, failure modes, and
  tradeoffs. It never proposes solutions — only questions. The interview ends
  when the user says "done" or the design holds up to scrutiny.

- `/grill-with-docs` — The grilling interview plus domain-modeling capture.
  As glossary terms and architecture decisions crystallize during grilling,
  the agent writes them to the project's CONTEXT.md and docs/adr/ directory.

**Skills** (auto-loaded into the agent's skill roster):

- `givn-domain-modeling` — Builds and maintains a project glossary
  (CONTEXT.md) and architecture decision records (docs/adr/). Challenges
  fuzzy language, invents edge-case scenarios, cross-references with code
  to keep the model honest.

- `givn-dev-principles` — Always-available skill that injects development
  principles: KISS, YAGNI, no huge files (500–600 LOC is a smell), clean code
  over documentation (extract helpers with clear names), testability
  (production code must be testable; don't mirror production in tests),
  black-box testing from the user's perspective.

### Commands

- `givn init` generates extras by default. `--no-extras` disables.
- `givn extras install` enables extras and syncs all skills (core + extras).
  Idempotent: safe to run multiple times.
- `givn extras uninstall` disables extras and removes all generated
  skill/command files. Idempotent.
- `givn skills sync` reads the extras config flag. When `extras: true`,
  it includes extras skills and commands alongside core workflow skills.
  When `extras: false`, it generates only core skills.

### Config

A new `extras: true/false` field in `givn/config.yaml`. Users toggle it
directly or via the `givn extras install/uninstall` commands.

## Out of Scope

- Extras are **not** addons. Addons (like arc42) expand the givn workflow
  artifact graph. Extras are project-level skills and commands — they never
  appear in `givn status`, gate archive, or modify the artifact DAG.
- Custom or user-authored extras. The initial set is curated and built-in.
- Per-change extras. Extras are project-global and generated once.
- An `extras list` command. The initial set is small and documented in the
  config template comments.
- Extras that modify `AGENTS.md`. Like core skills, extras are written to
  `.agents/skills/`, `.claude/skills/`, and `.opencode/` only.

## Open Questions

- None — all design decisions are resolved in the design artifact.