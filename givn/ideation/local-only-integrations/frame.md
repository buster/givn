# Frame: local-only-integrations

## Problem

`givn init` and `givn skills sync` write Givn rules into the
project: skills and commands for each selected agent target, attribution
plugins/hooks, and the Givn-owned fenced blocks inside `AGENTS.md`. `givn
upgrade` then force-stages the calculated allowlist into a managed revision
even when the project `.gitignore` excludes those paths (constraint 2.7,
quality requirement Q-57, risk R-33). A project that wants a lean repository —
source, specs, docs, `givn/` — has no supported way to keep the generated
plumbing out. Hand-written ignore rules are deliberately bypassed by upgrade,
and earlier commits still carry the files.

Requested outcome: one Givn setting, default "do not version generated Givn
files", that makes the Givn rules local installation output:

- installation writes them locally and keeps them out of the repository;
- upgrade untracks paths a previous version tracked and never re-adds them;
- Givn can state whether the repository or its history is clean of them, and
  what it does when it is not.

Why now: Givn is closed source while it is being developed, but the projects
created with it are meant to be published as open source. Generated
rules embed Givn's workflow guidance, so a published repository must
not carry them. The published project must also build and test with no Givn
installed; Givn is a development-time tool, never a runtime or test
dependency.

## Vision

A project created with Givn is publishable as-is: clone it, read the README,
run the test script — no Givn required, no Givn rules in sight. The developer's
machine keeps the full workflow: skills, commands, `AGENTS.md` pointer,
attribution, all local installation output. The repository carries only
project content: source, tests, `givn/` artifacts, specs, docs. Maintenance and
further development run through Givn locally, which understands the config,
the changes, and the markers. Givn treats generated rules as local
state everywhere: init, sync, upgrade, archive, and status.

## Scope boundary

In scope:

- The versioned/local state of Givn-generated paths and Givn-owned fenced
  blocks: skills, commands, plugins, hooks, `AGENTS.md` blocks, Claude
  settings merge.
- The setting that selects the state, its default, and where it lives
  (`givn/config.yaml`).
- `givn init`, `givn skills sync`, `givn upgrade` repository side effects,
  including untracking and ignore rules.
- The "clean history" guarantee: what Givn detects, reports, refuses, or
  rewrites.
- Backend-neutral semantics for Git and Fossil (the multi-scm-support
  vocabulary: revision, clean state, local-only/versioned).
- The publishability guarantee: a clone with no Givn installed can run the
  project and its tests, and the repository and its history carry no generated
  Givn rule files.
- Spec, constraint, quality-requirement, and migration documentation updates
  that own the current force-stage and "one commit contains the generated
  rules" guarantees.

Out of scope (candidate; confirm):

- The `givn/` directory itself — changes, specs, archive, personas, ideation,
  and its config scaffold stay versioned (user decision, 2026-09-23: "The
  whole givn directory is fine").
- Non-Givn files, even inside the same directories — never ignored, never
  untracked, never deleted.
- Automatic history rewriting; an explicit, opt-in rewrite command is in
  scope (G1-G4), never automatic and never a side effect of another command.
- Cross-repository policy (organization-wide hooks, CI templates).

## Stakeholders

| Stakeholder type | Interest |
|---|---|
| Developer (consumer) | Wants generated agent plumbing out of commits and diffs; expects every Givn command to keep working and a fresh clone to be recoverable with one command. |
| Collaborator / agent operator on another machine | Needs the Givn pointer, skills, and commands restored after clone; cannot rely on an untracked `AGENTS.md` to explain how. |
| External contributor (open-source consumer) | Clones the published project, builds it, runs its tests, and commits a change with no Givn installed. Must never encounter Givn rules or Givn-specific tooling. |
| Technical role (authoring agent / automation caller) | Drives the lifecycle and the machine contract on the developer's machine; on a published clone it must work with no Givn rule present, and no managed revision may carry generated rules or `AGENTS.md`. |
| Repository reviewer | Wants no generated noise in history, and wants untracking/ignoring to be explicit and reviewable rather than a silent mutation. |
| Givn maintainer | Must keep the upgrade and migration specs, Q-57, constraint 2.7, and the migration docs truthful — the force-stage rule and the managed-commit content guarantee both change. This repository currently versions its generated rules; whether it keeps doing so under the new default is a pending decision (Q9). |
| Non-Givn rule author | Owns files in the same directory trees (for example a hand-written skill under `.agents/skills/`, user hooks in `.claude/settings.json`, `AGENTS.md` prose). A directory-level ignore would silently drop their work. |

## Research protocol

### What exists

- Configuration: `givn/config.yaml` (`skills.targets`, addons, arc42),
  `givn/commands.yaml`; parsed through `src/config/overlay.rs`; the managed
  scaffold is re-rendered by upgrade (`src/cli/upgrade.rs`, ADR-023).
- Generated paths: `managed_generated_paths()` (`src/skills/targets.rs:330`)
  computes the explicit allowlist — `AGENTS.md`, `.githooks/commit-msg`,
  `.opencode/skills|commands|plugins`, `.agents/skills`,
  `.claude/skills|hooks|settings.json`, plus conditional grill commands.
  `AGENTS.md` and `.claude/settings.json` are mixed-ownership: user content
  next to Givn-generated content.
- Installation: `givn init` (`src/cli/init.rs`) scaffolds `givn/`, writes
  config, enables addons, and syncs skills; it does not stage or commit.
  `givn skills sync` (`src/skills/targets.rs`) re-renders, reconciles removed
  addon artifacts, and installs the local Git hook dispatcher
  (`src/cli/git.rs`).
- Upgrade: `prepare_managed_upgrade` (`src/cli/migration.rs:343`) takes a
  file snapshot, syncs, then `stage_paths(..., force = true)` commits the
  allowlist and config scaffold in one revision. A clean checkout is a
  preflight (Q-49). `relative_managed_paths` filters to existing paths only.
- Backend: Git and Fossil behind `crate::scm::Backend` (scm-backend seed);
  glossary fixes "revision" as the neutral term. Fossil has no hook mechanism;
  attribution exists only through `givn commit`.
- Archive: requires a clean state where untracked non-ignored files count as
  dirty (`dirty_entries`, `-uall`); ignored files are invisible there.
  Evidence paths are staged individually.
- Existing documentation that owns today's behavior: constraint 2.7 (managed
  migration ownership and force-staging), Q-49 (dirty checkout refusal), Q-57
  (ignored generated files are force-staged), R-33 (force-staging risk),
  `apply-a-managed-upgrade.feature` ("one commit contains only the managed
  scaffold and generated agent integration changes"), migration docs
  (`docs/migrations/0.2.0-to-0.3.0.md`, line 115), arc42 chapter 8.7.

### What the permanent specs already own

- `initialize-a-project.feature` owns target selection, persisted
  `skills.targets`, non-interactive defaults, and backend-aware init output.
- `apply-a-managed-upgrade.feature` owns the clean preflight, the one managed
  upgrade revision, migration preparation, and Fossil parity.
- Fragments `attribute-commits.feature` and `enable-commit-attribution.feature`
  own trailer stamping and hook installation; `refresh-generated-guidance`
  owns generated guidance refresh.
- `archive-proof.feature` owns clean-state and untracked-file semantics.

### Gaps

| ID | Gap |
|---|---|
| G1 | No config key expresses whether generated rules are repository content or local installation output. |
| G2 | Upgrade force-stages ignored managed paths (constraint 2.7 / Q-57); it has no mode that instead removes them from the index. |
| G3 | No command consults the existing revision-membership check (`Backend::revision_contains`) for managed paths, and it covers only the current revision, not history. |
| G4 | The Claude attribution hook is merged into the shared `.claude/settings.json`; with `AGENTS.md` the only allowed mixed-ownership exception, the hook mechanics must move to a project-local file (Q19). |
| G5 | Ignore semantics are backend-neutral in no place: `.gitignore` is assumed; Fossil ignore (`ignore-glob`) has no Givn equivalent. |
| G6 | Fresh-clone onboarding has no channel if `AGENTS.md` itself is not in the repository. |
| G7 | Existing specs, quality requirements, constraints, and migration docs assert the opposite behavior and must be amended (enumerated in Q9). |
| G8 | Non-Givn files share the generated directories (for example `.agents/skills/gh-stack/SKILL.md` in this repository); any ignore mechanism must be file-precise, never directory-wide. |
| G9 | `.githooks/commit-msg` is shared attribution plumbing, currently versioned; its local-only story is undefined. |
| G10 | Nothing verifies publishability: no check that a clone of the repository builds and tests without Givn, and no check that the repository (or its history) is free of generated Givn rules. |

## AGENTS.md boundary (confirmed 2026-09-23)

- Givn keeps managing its fenced blocks in `AGENTS.md` on init, sync, and
  upgrade; the user-owned prose outside the fence is untouched.
- Givn never stages or commits `AGENTS.md` on its own. If the user checks it
  in, that is accepted; merging and diffing the file are the user's
  responsibility.
- The strict local-only and clean-history guarantees apply to every other
  generated rule; `AGENTS.md` is the single, explicit exception among rule
  files. A second Givn-authored artifact is sanctioned as project content:
  the fenced ignore block inside `.gitignore` / the Fossil ignore file, whose
  entries are not rules and whose surrounding user content is never touched
  (confirmed 2026-09-23).
- Consequence: the Claude attribution hook cannot stay merged into the
  shared `.claude/settings.json`; the candidate local home is
  `.claude/settings.local.json` (verified supported in the Claude Code
  settings documentation). Parked as Q19.

## Open questions

See `questions.md` (Q1-Q22).
