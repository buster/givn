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
- [x] **Structural** - it defines the boundary between generated skill files and the root `AGENTS.md`.
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

Chosen option: **Emit generated guidance to configured agent directories; manage only givn-owned fenced blocks in `AGENTS.md`**. Content outside `<!-- givn:begin:<name> -->` and matching end markers is user-owned and is not changed. Initialization records an explicitly selected target set, and synchronization writes only to those targets. The original decision generated skills and commands; the current amendment below retires the command half.

The `require-agent-target` change amended this decision (its interactive
prompt wording is refined by `keep-agent-prompt-open`): initialization
requires at least one selected target. The interactive prompt opens with
`.opencode` marked and re-asks, with an error naming the requirement, until a
target is marked; an empty or unknown-only answer and EOF without the
`--agents` flag refuse with a non-zero exit before the first write instead of
falling back to `opencode`. Synchronization keeps the `opencode` fallback for
a configuration without a `skills:` block, so projects initialized by older
versions stay runnable.

The `development-timeline` change amended this decision again: the
generated-guidance destinations gain `maki` (`.maki`), alongside `.opencode`,
`.agents`, `.claude`, and `.omp`. It is a full skills target and also receives
the in-project development reporter: the Lua module under `.maki/lua/`, its
grants merged into `.maki/plugin.toml`, and a managed `-- givn:begin:timeline`
fence inside `.maki/init.lua` (Maki runs only that file, so a module under
`lua/` stays inert until it is required). Only the fenced region and the grant
keys are touched in those two user-owned files; all other content stays as the
user wrote it.

**Current amendment (prepare-0-9-10).** The generated invocation surface is
skills-only. Givn writes `<target>/skills/<name>/SKILL.md` and generates no
command files for any target. Recognizable command files from earlier releases
are removed during sync; a custom file with a known Givn name is preserved and
reported for manual transition. Maki uses an explicit request or an
agent-selected skill, not a generated command. The current invocation paths are
Claude Code and OpenCode skill slash, OMP `/skill:<name>`, the neutral `agents`
target's skill discovery, and Maki's explicit/agent-selected skill. The
ownership rule for surrounding content is unchanged.

The `maki-commit-attribution` change amended this decision again: a selected
`maki` target also receives the generated commit-attribution module under
`.maki/lua/`, loaded by a second managed `-- givn:begin:attribution` fence in
`.maki/init.lua`, and `.maki/plugin.toml` gains the harness-required
`session_write` grant. The merge discipline is unchanged; an exact prior
generated manifest is the only revision Givn rewrites, and an edited manifest
keeps its explicit revocation.

### Consequences

- Skills are written under selected `.opencode/`, `.agents/`, `.claude/`,
  `.omp/`, and `.maki/` locations. No target receives generated commands.
- A selected `maki` target receives skills, the development reporter, and the
  commit-attribution integration; `.maki/init.lua` and `.maki/plugin.toml` are
  merged, never replaced, so a hand-written Maki config keeps its content.
- The always-on `givn-project` block and the optional `communication-policy`
  block are separately managed when applicable; the retired `absolute-mode`
  block is removed during sync.
- Generated files are snapshots and are refreshed by `givn skills sync`.
- Target selection is explicit project configuration; the interactive prompt
  re-asks until a target is marked, a non-terminal empty or unknown-only
  selection is refused rather than writing every target, and a scripted
  `givn init` must state its selection with `--agents`.
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
