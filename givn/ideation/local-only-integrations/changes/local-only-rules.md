# Change seed: local-only-rules

Status: implemented in local-only-rules

- Use-case IDs: fragment `rules-policy` (new), `initialize-project`
  (modified), `upgrade-a-project` (modified), fragment
  `local-only-transition` (new), capability `attribute-commits` (modified),
  fragment `publication-cleanliness` (new)
- Topic: `local-only-integrations`
- Personas: `developer--primary-user`, `agent--technical-role`,
  `external-contributor--open-source-consumer`, `givn-maintainer--owner`
- Suggested start: first; `rewrite-history` depends on it
- Handoff: ready

## Why one change

The setting, the install path, the reconciliation, and the report are one
deliverable: any smaller split ships a release where `rules: local-only` is
not honored — a setting that exists but that `givn upgrade` still overrides
by force-staging the rules. The core must land as one unit. The destructive,
tool-gated rewrite is the only part that earns a separate change.

## Accepted decisions

### Q1 — Setting name, shape, and default

- Chosen: flat scalar `rules: local-only` (default) / `rules: versioned` in
  `givn/config.yaml`; absent resolves to `local-only` for new and existing
  projects; an unrecognized value warns and continues as `local-only`.
  Rejected: nested block; `integrations` naming; refuse-on-invalid.

### Q3 — The `AGENTS.md` boundary

- Chosen: keep the fenced block; Givn never stages or commits the file under
  either policy; a user commit is accepted and merging is their
  responsibility. Rejected: reference/import design (too complex).

### Q10 — Ignore destinations

- Chosen: both — the fenced tracked block (Git `.gitignore`, Fossil
  `.fossil-settings/ignore-glob`) and the local exclude (Git
  `.git/info/exclude`, Fossil checkout-local `ignore-glob`), file-precise,
  never a directory, never `AGENTS.md`, never a user-owned path. Rejected:
  single-destination variants.

### Q7 — Reconciliation

- Chosen: `givn init` reconciles tracked strict rules through the
  `local-only-transition` fragment and leaves the removals staged, committing
  nothing; `givn upgrade` folds the removals into its managed revision.
  Git `git rm --cached`; Fossil `fossil rm --soft`/`forget`, pinned against
  the user's `mv-rm-files` setting. Files are always preserved.
  Rejected: report-only init; refusing init.

### Managed revision content

- Chosen: under `local-only` the revision carries the scaffold, ignore
  entries, and removals only; under `versioned` it stages the rules per
  today's allowlist; `AGENTS.md` excluded under both. Rejected: unchanged
  force-staged allowlist; staging `AGENTS.md` in the versioned branch.

### `givn commit` stage set

- Chosen: the stage-all path excludes `AGENTS.md` and strict rules under
  `local-only`; named paths stay explicit, and naming `AGENTS.md` commits it.
  Rejected: unchanged stage-all.

### Q2 — History report (not the rewrite)

- Chosen: report tracked and reachable-history occurrences, name the
  boundary, and point at `rewrite-history`; never block. init and upgrade
  exit 0. Rejected: refusal; automatic rewrite.

### Report surfaces and exit contract

- Chosen: the init/upgrade installation report names the resolved policy and
  the cleanliness state; a dedicated on-demand check gives the detailed
  report and exits non-zero on a violation. Rejected: a status line on every
  run (slow); advisory-only exit.

## Open questions

- Q5: precise "non-Givn rule file" wording; the preservation policy is agreed.
- Q9: this repository's own self-hosting choice.
- Q11: one switch or per-category switches; the model assumes one.
- Q12/Q14 (gated): policy-change trigger set and per-command idempotence.
- Q13: the refusal text when ignore entries are missing and archive blocks.
- Q15: whether the check verifies a fresh clone builds and tests with Givn
  absent.
- Q16: `givn/` content visibility is accepted; confirm at proposal time.
- Q19: the Claude hook home (`.claude/settings.local.json` is the candidate).
- Q21: the dedicated check command name and interface.

## Scope note

Makes `rules: local-only` true end to end: config scalar and resolution, both
ignore destinations, `AGENTS.md` never staged, setup reconciliation, the
upgrade's policy-aware managed revision, the commit stage-set change, the
report in init/upgrade, and the dedicated check. Introduces the permanent
fragments `rules-policy`, `local-only-transition`, and
`publication-cleanliness`; modifies `initialize-project`,
`upgrade-a-project`, and the `attribute-commits` capability. Amends
constraint 2.7, Q-57, R-33, chapter 8.7, migration docs, and the glossary
rows "AGENTS.md ownership", "Commit-attribution hook", "Hook activation
launcher", "Managed upgrade path", and "Upgrade revision".
