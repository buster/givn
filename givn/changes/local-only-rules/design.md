# Design: local-only-rules

## Overview

The generated-rules policy becomes real end to end. One scalar
(`rules: local-only | versioned`) is resolved by every writer; under
`local-only` the strict rules are written locally, ignored in both
destinations, never staged by `givn commit` or the managed upgrade, and a
tracked rule becomes local-only without deleting its file. A cleanliness
report runs in init, sync, and upgrade, and the dedicated `givn check rules`
exits non-zero on a violation. History rewriting is out of scope.

## Decisions

- Decides D1: this repository sets `rules: versioned` explicitly and keeps
  versioning its generated guidance — realized in `givn/config.yaml`.
- Decides D2: the next applicable command reconciles; sync reconciles the
  transition and ignore entries, and repeated runs change nothing —
  realized in `src/skills/targets.rs` (`sync`) and
  `src/skills/rules.rs` (`reconcile_tracked_rules`).
- Decides D3: the check reports repository state only; no fresh-clone run —
  realized in `src/skills/rules.rs` (`cleanliness`) and
  `src/cli/check.rs`.
- Decides D4: the Claude attribution hook entry moves to
  `.claude/settings.local.json`; the shared `.claude/settings.json` is no
  longer managed or merged — realized in `src/skills/targets.rs`.
- Decides D5: the dedicated check is `givn check rules`, reporting and
  exiting non-zero on a violation while init, sync, and upgrade exit 0 —
  realized in `src/cli/check.rs`.

## Exact file changes

| Path | Change |
|---|---|
| `src/config/overlay.rs` | `RulesPolicy`, `load_rules_policy` (absent/invalid → local-only, invalid warns), `rules` overlay field. |
| `src/skills/rules.rs` | New: strict rule set, fenced ignore blocks (tracked + local, Git/Fossil), tracked-rule transition, cleanliness scan and report. |
| `src/skills/mod.rs` | Register `rules`. |
| `src/skills/targets.rs` | Sync applies ignores, transition, and the report; Claude hook moved to `.claude/settings.local.json`; managed paths update. |
| `src/cli/init.rs` | Fresh config records the active `rules: local-only` scalar. |
| `src/cli/commit.rs` | Under local-only the no-path stage set excludes `AGENTS.md` and the strict rules; named paths stay explicit. |
| `src/cli/migration.rs` | Managed revision excludes `AGENTS.md`, stages the scaffold plus ignore files at local-only without force, keeps removals, prints the report. |
| `src/cli/check.rs` | `givn check rules`: report, exit non-zero on violation, informational under versioned. |
| `src/git.rs`, `src/fossil.rs`, `src/scm.rs` | `remove_from_index` (Git `rm --cached`; Fossil `rm --soft`) and `history_occurrences` (Git `log --all`; Fossil `finfo -b`). |
| `assets/config.yaml.tmpl` | Documents the policy scalar and both values. |
| `givn/specs/fragments/fragment.md` | Declares the three new fragment capabilities. |
| `givn/config.yaml` | This repository opts in to `rules: versioned` (D1). |
| `givn/changes/local-only-rules/specs/**` | Seven delta documents (six features plus the fragment document). |
| `tests/steps/local_only_rules_steps.rs` | New step file for the capability set (one file per capability group). |
| `tests/steps/release_0_3_0_e2e_steps.rs` | Managed-revision assertion updated for the local-only default (no `AGENTS.md`, scaffold plus removals). |
| `tests/steps/llm_attribution_installation_e2e_steps.rs`, `tests/steps/llm_attribution_e2e_steps.rs` | Claude hook home is `.claude/settings.local.json`; the plumbing-refresh fixture opts in to `versioned`. |

## Step definitions

`tests/steps/local_only_rules_steps.rs` (new). Reused steps stay in
`tests/steps/mod.rs` (`the file "X" contains`, `stdout contains`, exit
codes) and `tests/steps/prose_consolidation_steps.rs`.

## Runner and strict mode

- Runner: `./run-tests.sh` (`cargo nextest run`, `tests/features_runner`).
- Strict mode: `.fail_on_skipped()` in `tests/features_runner.rs`.
- Single-scenario command:
  `GIVN_FEATURES=givn/changes/local-only-rules/specs cargo test --test features_runner -- --exact "<feature>::<scenario>"`.
- E2E: no new `@e2e` scenario; the capability's existing `@e2e` scenarios
  (`Consumer initializes a project and receives shared attribution
  plumbing`, upgrade shapes) cover the real interfaces.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| initialize a project and receive attribution plumbing | Consumer initializes a project and receives shared attribution plumbing | CLI | Real `givn init` subprocess; files and git state asserted |
| upgrade a legacy project | A clean 0.2.0 project receives a reviewable aggregate migration plan | CLI | Real `givn upgrade` subprocess; revision contents asserted |

## Use-case traceability

| Contract element | Technical decision |
|---|---|
| Fragment `rules-policy`: absent → local-only, invalid warns | `load_rules_policy`; init writes the scalar; sync/upgrade report it. |
| Fragment `local-only-transition`: files kept, removals staged | `reconcile_tracked_rules` + `remove_from_index`. |
| Fragment `publication-cleanliness`: report + non-zero check | `cleanliness` + `givn check rules`. |
| `initialize-project`: local install and ignores | `sync` writes both destinations; `AGENTS.md` never staged. |
| `attribute-commits`: stage set under local-only | `commit::run` filters the no-path set. |
| `upgrade-a-project`: policy-aware revision, history report | `migration.rs` staging + `print_report`. |
| Personas | `developer--primary-user` (repository stays clean), `agent--technical-role` (deterministic commands), `external-contributor--open-source-consumer` (clone needs no Givn), `givn-maintainer--owner` (truthful records). CLI text output, so Personas are named. |

## Version freshness

No versioned technology is introduced; the change uses the existing Rust
toolchain and shell-outs to the SCM CLIs.

## Local runnability & digital twins

None — the system is a CLI; local run and test are `cargo run` and
`./run-tests.sh`. The archive's `run` declaration is unchanged.

## Coverage process boundaries

N/A — no instrumented production process is added.

## Arc42 impact

Sections 8.7 (skill and rule generation) is updated with the policy; chapter 5
gains the `skills::rules` block row; chapters 2, 10, and 11 have the
force-staging records amended. Chapter 12 already carries the policy terms
from the topic handoff. No ADR qualifies (the policy applies existing
boundaries; the canonical home is the generated guidance plus arc42 8.7).
Recorded in `arc42.md`.

## Internal Primitives

- `RulesPolicy` (LocalOnly / Versioned) — the resolved policy value.
- `Cleanliness` — the tracked/history scan result.
