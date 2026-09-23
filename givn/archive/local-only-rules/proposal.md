# Proposal: local-only-rules

## Use-Case Context

- Use-case IDs: fragment `rules-policy` (new), `initialize-project`
  (modified), fragment `local-only-transition` (new), capability
  `attribute-commits` (modified), `upgrade-a-project` (modified), fragment
  `publication-cleanliness` (new)
- Ideation topic: `local-only-integrations`
- Seed path: `givn/ideation/local-only-integrations/changes/local-only-rules.md`
- Confirmed Personas: `developer--primary-user`,
  `agent--technical-role`, `external-contributor--open-source-consumer`,
  `givn-maintainer--owner`

## Problem / Opportunity

A project created with Givn cannot keep the generated rules — skills,
commands, plugins, hooks, attribution plumbing — out of its repository.
`givn init` writes them, `givn upgrade` force-stages them even when the
project ignores them (arc42 constraint 2.7, quality requirement Q-57, risk
R-33), and `givn commit` stages everything pending. A project meant to be
published as open source therefore carries Givn's closed-source guidance in
its history, and a clone without Givn cannot tell which files are project
content and which are tool state.

Impact: the developer either publishes Givn's guidance or hand-maintains
ignore rules that upgrade deliberately bypasses; external contributors see
tool artifacts instead of project content.

## Proposed Solution

Observable outcomes:

1. One setting — `rules: local-only` (default) or `rules: versioned`, a flat
   scalar in `givn/config.yaml`. An absent key resolves to `local-only` for
   new and existing projects; an unrecognized value warns and continues as
   `local-only`, naming the valid values.
2. Under `local-only`, `givn init` and `givn skills sync` write the rules for
   the selected targets and keep them out of the repository: file-precise
   entries in both the tracked ignore block (`.gitignore` /
   `.fossil-settings/ignore-glob`) and the local exclude
   (`.git/info/exclude` / checkout-local `ignore-glob`), never a directory,
   never `AGENTS.md`, never a user-owned path.
3. `AGENTS.md` keeps its fence and is never staged or committed by Givn under
   either policy; if the developer commits it, that is accepted and merging
   it is their responsibility.
4. A tracked rule becomes local-only without deleting the file: `givn init`
   leaves the removals staged for the developer to commit; `givn upgrade`,
   run on a clean checkout, performs the same transition and folds the
   removals into its managed revision.
5. Under `local-only`, the managed upgrade revision contains the config
   scaffold, commands scaffold, ignore-block changes, and the removals only;
   under `versioned` it stages the rules per today's allowlist. `AGENTS.md`
   is excluded under both.
6. Under `local-only`, `givn commit` with no paths stages project content
   only; naming `AGENTS.md` commits it explicitly.
7. `givn init` and `givn upgrade` report the resolved policy and the
   cleanliness state (tracked rules, reachable-history occurrences, and the
   boundary). A dedicated check gives the detailed report and exits non-zero
   on a violation. Reporting never blocks init or upgrade.
8. A clone of a published project builds, runs, and tests with no Givn
   installed.

## Capability Routing

`givn spec route --change local-only-rules` reports no signal (top
candidates tie).

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `rules-policy` | no signal | `NEW in fragments` | New reusable fragment capability. |
| `local-only-transition` | no signal | `NEW in fragments` | New reusable fragment capability. |
| `publication-cleanliness` | no signal | `NEW in fragments` | New reusable fragment capability. |
| `attribute-commits` | no signal | `NEW in fragments` | Existing fragment capability; the delta extends its stage set. |
| `initialize-a-project` | no signal | `EXTEND initialize-a-project` | Existing capability under `initialize-project`. |
| `enable-optional-addons` | no signal | `EXTEND enable-optional-addons` | Existing capability under `initialize-project`; the report names the policy. |
| `apply-a-managed-upgrade` | no signal | `EXTEND apply-a-managed-upgrade` | Existing capability under `upgrade-a-project`. |

## Out of Scope

- Per-category switches (skills vs commands vs hooks); one setting.
- History rewriting; this change reports only. The opt-in rewrite is its own
  change seed.
- The `givn/` directory: it stays versioned; only generated rules are
  governed.
- Remote operations (push, clone, fetch) and fork cleanup.

## Open Questions

### D1 — This repository's own rules policy

- Question: after the default flips, should the Givn repository itself set
  `rules: versioned` (keep committing its generated guidance) or adopt
  `local-only` (untrack it)?
- Options:
  1. **Set `rules: versioned` explicitly.** Meaning: the repository states
     its choice and keeps versioning its guidance; gain: contributors and CI
     get guidance from a clone without running sync; cost: the repository
     keeps carrying generated files. In practice: `givn/config.yaml` gains
     the active scalar.
  2. **Adopt `local-only`.** Meaning: the repository untracks its generated
     guidance; gain: a smaller tree; cost: every clone must run `givn skills
     sync` before agents get guidance, and this change must untrack them.
- Disposition: proposed option 1; carried to proposal review (user decides).

### D2 — Policy-change trigger and idempotence (seed Q12/Q14)

- Question: when `rules:` changes, what triggers the reconciliation, and is
  it idempotent on every trigger?
- Options:
  1. **The next applicable command reconciles; repeated runs change
     nothing.** In practice: flipping to `local-only` then running upgrade
     untracks once; a second upgrade publishes no removal. Gain: no new
     command; cost: the flip is applied at the next natural write.
  2. **Only `givn upgrade` reconciles.** Gain: one trigger; cost: init/sync
     leave a tracked rule in place until an upgrade.
- Disposition: proposed option 1; carried to design (the design records the
  trigger set and idempotence).

### D3 — Fresh-clone verification (seed Q15)

- Question: does the dedicated check verify that a fresh clone builds and
  tests with `givn` absent?
- Options:
  1. **No — the check reports repository state; the guarantee is structural.**
     In practice: rules are never tracked, so a clone needs no Givn; the
     check reports tracked/historical occurrences only. Gain: a fast,
     side-effect-free check; cost: no executed proof.
  2. **Yes — the check clones and runs the test command with `givn` absent.**
     Gain: executed evidence; cost: slow, environment-dependent, and clones
     remotes or local paths.
- Disposition: proposed option 1; carried to design.

### D4 — Claude hook home (seed Q19)

- Question: in local-only mode, where does the Claude attribution hook live,
  given the shared `.claude/settings.json` is user-owned?
- Options:
  1. **Write the hook entry into `.claude/settings.local.json`** (project
     local, outranks the shared file, kept out of git) and stop merging into
     the shared file. Gain: no mixed-ownership exception; cost: a migration
     for projects whose hook is already merged into the shared file.
  2. **Keep merging into `.claude/settings.json` and treat it as a second
     exception.** Gain: no migration; cost: violates the single-exception
     decision.
- Disposition: proposed option 1; carried to design.

### D5 — Dedicated check name and interface (seed Q21)

- Question: what is the dedicated cleanliness check called, and how is it
  invoked?
- Options:
  1. **`givn check rules`** — extends the existing `givn check` surface.
     In practice: `givn check rules` prints the tracked and historical
     violations and exits non-zero. Gain: no new top-level command; cost:
     `rules` becomes a reserved pseudo-artifact name.
  2. **`givn status --rules`** — a variant of status. Gain: reuses status;
     cost: status becomes mode-dependent.
- Disposition: proposed option 1; carried to design.
