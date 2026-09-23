# Local-only Givn rules

Topic: `local-only-integrations` — ideation handoff, 2026-09-23.

A project created with Givn can be published as open source without carrying
Givn's rules in the repository or its history, while the developer's machine
keeps the full workflow. This document is the readable record; the working
state lives under `givn/ideation/local-only-integrations/`.

## Why

Givn is closed source while it is developed, but the projects created with it
are published as open source. Givn rules embed Givn's workflow guidance, so a
published repository must not carry them. The published project must also
build and test with no Givn installed: Givn is a development-time tool, never
a runtime or test dependency. Today, `givn upgrade` force-stages generated
rules even when the project ignores them (arc42 constraint 2.7, quality
requirement Q-57, risk R-33), so there is no supported way to keep a
repository clean.

## Vision

A project created with Givn is publishable as-is: clone it, read the README,
run the test script — no Givn required, no Givn rules in sight. The
developer's machine and every machine that runs one Givn command has the full
workflow: skills, commands, `AGENTS.md` pointer, attribution. The repository
carries only project content. Maintenance and further development run through
Givn locally, which understands the config, the changes, and the markers.

## The model

**One setting.** `rules: local-only` (default) or `rules: versioned`, a flat
scalar in `givn/config.yaml`. Absent resolves to `local-only` for new and
existing projects alike; an unrecognized value warns and continues as
`local-only`. Changing the value reconciles on the next applicable command.

**Strict rules.** Every Givn rule — skills, commands, plugins, hooks, the
shared `commit-msg` dispatcher, the Claude hook — except the `AGENTS.md`
fence. Under `local-only` they are never tracked, never staged, and kept out
of both ignore destinations; under `versioned` they are repository content.

**The `AGENTS.md` boundary (user decision).** Givn keeps writing and
refreshing its fenced block. It never stages or commits the file under either
policy; if the user checks it in, that is accepted, and merging and diffing
it are the user's responsibility. No reference or import machinery is
designed.

**Ignore destinations (both).** A fenced, Givn-managed block in the tracked
ignore file (Git `.gitignore`, Fossil `.fossil-settings/ignore-glob`) gives
every clone protection; a local exclude (Git `.git/info/exclude`, Fossil
checkout-local `ignore-glob`) takes effect before the block is committed.
Entries are file-precise: never a directory, never `AGENTS.md`, never a
user-owned or non-Givn path. The ignore block is project content, not a rule;
surrounding user content is never touched.

**The transition.** A tracked strict rule becomes local-only with
`git rm --cached` (Git) or `fossil rm --soft`/`forget` (Fossil), pinned so a
user's `mv-rm-files` setting cannot delete files. `givn init` leaves the
removals staged for the developer; `givn upgrade` folds them into its managed
revision. Files are always preserved.

**Upgrade.** Under `local-only` the managed revision contains the config
scaffold, commands scaffold, ignore-block changes, and the removals only — no
strict rule and no `AGENTS.md`. Under `versioned` it stages the rules per
today's allowlist, `AGENTS.md` still excluded. The clean preflight is
unchanged.

**Commit.** Under `local-only`, `givn commit` with no paths stages project
content only: `AGENTS.md` and strict rules are excluded. Named paths stay
explicit; naming `AGENTS.md` commits it.

**Check and rewrite.** Every command that checks cleanliness reports strict
rules tracked at the current revision and in reachable history, names the
boundary (unreachable objects, remotes, and forks are outside the guarantee),
and points at remediation. Reporting never blocks: init and upgrade exit 0.
A dedicated on-demand check exits non-zero on a violation. An explicit,
never-automatic `rewrite-history` command cleans reachable history on Git
through `git-filter-repo` (refused by name when absent) and refuses by name
on Fossil. Confirmation is `--yes`, a terminal prompt, or a piped-input
refusal.

## Stakeholders and personas

| Persona | Interest |
|---|---|
| Primary user (developer / publisher) | Publishes open source, keeps Givn rules local, owns the `AGENTS.md` tracking decision and their own rule files, restores the workflow with one command on a new machine. |
| Technical role (authoring agent / automation caller) | Deterministic commands and reports; no Givn workflow on a published clone. |
| External contributor (open-source consumer) | Clone, build, test, and commit with no Givn installed and no Givn traces in the repository or history. |
| Givn maintainer (owner) | Truthful upgrade, archive, and quality records; the default flips and this repository's self-hosting choice is an explicit, separate decision. |

## Change program

Two changes, cut by deliverable guarantee rather than by use case. A smaller
split would ship releases where `rules: local-only` is not honored — the
setting exists but upgrade still force-stages the rules — so the core lands
as one unit. The destructive, tool-gated rewrite is the only part that earns
separation.

| Order | Change | Covers | Depends on |
|---|---|---|---|
| 1 | `local-only-rules` | fragment `rules-policy` (new), `initialize-project` (modified), fragment `local-only-transition` (new), capability `attribute-commits` (modified), `upgrade-a-project` (modified), fragment `publication-cleanliness` (new) | — |
| 2 | `rewrite-history` | use case `rewrite-history` (new) | 1 |

Permanent homes: the setting and the transition become new permanent
fragments (`policy-resolution`, `local-only-transition`); the report becomes
the `publication-cleanliness` fragment; the rewrite becomes a new permanent
use case with capability `history-rewrite`; `initialize-project` and
`upgrade-a-project` keep their IDs and gain the delta. `author-change`,
`create-change`, `archive-change`, `maintain-project`,
`keep-documentation-current`, and `publish-a-release` are untouched;
`verify-change` stays a candidate.

Existing durable records these changes amend when they land: arc42 constraint
2.7, quality requirement Q-57, risk R-33, arc42 chapter 8.7, the upgrade and
init scenarios, the migration docs, and the glossary rows "AGENTS.md
ownership", "Commit-attribution hook", "Hook activation launcher", "Managed
upgrade path", and "Upgrade revision".

## Open questions

| ID | Question |
|---|---|
| Q5 | Precise scope of "non-Givn rule files"; the preservation policy is agreed, the mixed-file wording settles at implementation. |
| Q9 | Does this repository version its own generated rules (self-hosting), now that the default is local-only? |
| Q11 | One policy switch or per-category switches; the model assumes one. |
| Q12/Q14 | Policy-change trigger set and idempotence per command. |
| Q13 | Are ignore entries mandatory for archive coexistence (yes in practice; confirm the refusal text). |
| Q15 | Does the check verify a fresh clone builds and tests with Givn absent? |
| Q16 | Is `givn/` content (config scaffold naming addons, specs) acceptable to publish? The user decided the directory stays versioned; the closed-source motivation accepts the workflow terms it reveals. |
| Q19 | Claude hook home in local-only mode: `.claude/settings.local.json` is the candidate. |
| Q20 | Remaining rewrite safety: backup policy for original refs, remote guidance wording. |
| Q21 | Dedicated cleanliness check command name and interface. |

## Decision log

- 2026-09-23 — Default: generated rules stay local; `givn/` stays versioned.
- 2026-09-23 — `AGENTS.md`: fenced block kept, never auto-committed, user owns
  tracking and merges; single exception among rule files.
- 2026-09-23 — Run and test without Givn is a hard requirement.
- 2026-09-23 — Noun is `rules`; setting is `rules: local-only` /
  `rules: versioned`.
- 2026-09-23 — Ignore entries in both destinations; ignore block sanctioned
  as project content.
- 2026-09-23 — `givn init` reconciles tracked rules too, leaving staged
  removals; upgrade folds them into its managed revision.
- 2026-09-23 — `givn commit` stage-all excludes `AGENTS.md` and strict rules.
- 2026-09-23 — History: report and never block; explicit opt-in rewrite.
- 2026-09-23 — Git rewrite uses `git-filter-repo` or refuses by name; Fossil
  refuses by name; confirmation is `--yes`, TTY prompt, or piped refusal.
- 2026-09-23 — Dedicated check exits non-zero on violations; init and upgrade
  exit 0.
- 2026-09-23 — Change cut revised per the user: two changes, not five. The
  core (`local-only-rules`) must be one unit so no release ships a setting
  upgrade does not honor; the rewrite ships separately because it is
  destructive and tool-gated.
