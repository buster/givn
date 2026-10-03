---
status: accepted
date: 2026-06-22
decision-makers:
  - givn maintainers
consulted:
  - agent integration design record
informed:
  - contributors and users
---

# Generate agent skills while preserving user-owned AGENTS.md content

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - generated paths and ownership rules are consumed by projects and agents.
- [x] **Shapes quality attributes** - the policy protects project context and keeps generated guidance maintainable.
- [x] **Structural** - it defines the boundary between generated skill/command files and the root `AGENTS.md`.
- [x] **Must be shared** - every sync and every agent target must follow the same ownership rule.

## Context and Problem Statement

Agents need on-demand workflow guidance, but a root `AGENTS.md` is project context that may contain unrelated user rules. Writing the entire file during synchronization would overwrite user content and mix always-on context with generated workflow guidance.

## Decision Drivers

- never overwrite user content
- provide neutral skills with supported mirrors
- make generated guidance refreshable
- keep target selection within the generated-agent ownership contract
- make a newly selected target explicit and persistent

## Considered Options

- Write all workflow guidance into `AGENTS.md`.
- Emit skills and commands to agent-specific directories and leave `AGENTS.md` user-owned.
- Emit no generated guidance and require manual agent setup.

## Decision Outcome

Chosen option: **Emit skills and commands to configured agent directories; manage only givn-owned fenced blocks in `AGENTS.md`**. Content outside `<!-- givn:begin:<name> -->` and matching end markers is user-owned and is not changed. Initialization records an explicitly selected target set, and synchronization writes only to those targets.

The `require-agent-target` change amended this decision: initialization
requires at least one selected target. The interactive prompt starts with
nothing marked, and an empty confirmation, an empty or unknown-only answer,
and EOF without the `--agents` flag all refuse with a non-zero exit before the
first write instead of falling back to `opencode`. Synchronization keeps the
`opencode` fallback for a configuration without a `skills:` block, so
projects initialized by older versions stay runnable.

### Consequences

- Skills are written under selected `.opencode/`, `.agents/`, and `.claude/` locations; opencode commands use `.opencode/commands/`.
- The always-on `givn-project` block and the `absolute-mode` block are separately managed when applicable.
- Generated files are snapshots and are refreshed by `givn skills sync`.
- Target selection is explicit project configuration; initialization refuses an
  empty or unknown-only selection rather than writing every target, and a
  scripted `givn init` must state its selection with `--agents`.
- Synchronization still falls back to `opencode` for a configuration without a
  `skills:` block.

### Confirmation

The target-selection, block-preservation, and skill synchronization tests assert that unrelated `AGENTS.md` content is byte-preserved while owned blocks are updated.

## Pros and Cons of the Options

### Skills plus owned fenced blocks

- Good, because project rules remain user-owned and workflow guidance is loaded on demand.
- Bad, because agents must load the relevant skill instead of finding all guidance in one always-on file.

### Write all guidance to AGENTS.md

- Good, because the guidance is always visible.
- Bad, because synchronization can overwrite project context and create permanent noise.

### No generated guidance

- Good, because givn writes fewer files.
- Bad, because every project must recreate and maintain the workflow instructions manually.

## More Information

Target selection and the implementation boundary are described in [chapter
8.7](../08-crosscutting-concepts.md#87-skill-and-rule-generation).
