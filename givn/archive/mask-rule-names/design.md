# Design: mask-rule-names

## Overview

`RulesPolicy::LocalOnly` writes prefix patterns into both ignore
destinations instead of enumerating rule paths. Discovery and the transition
stay predicate-based, so tracked rules are still found.

## Decisions

- The entry set is a fixed list of prefix patterns
  (`src/skills/rules.rs::ignore_entries`): target directories under
  `givn-*`, command files under `givn-*.md` and `grill-*.md`, and the fixed
  integration files. No per-file paths.
- Fossil patterns drop the trailing slash for the tracked
  `.fossil-settings/ignore-glob` block.

## Exact file changes

| Path | Change |
|---|---|
| `src/skills/rules.rs` | `ignore_entries()`; `write_ignores` uses it; Fossil strips the trailing slash. |
| `tests/steps/local_only_rules_steps.rs` | The negative block assertion checks substrings, so a masked name cannot appear anywhere in the block. |
| `givn/changes/mask-rule-names/specs/initialize-project/initialize-a-project.feature` | Modified scenario asserts the pattern and the absence of a skill name. |
| `docs/arc42/08-crosscutting-concepts.md` | 8.7 names the pattern form and the predicate-based discovery. |

## Step definitions

Reused; the negative assertion becomes substring-based.

## Runner and strict mode

`./run-tests.sh`; strict `.fail_on_skipped()`. Single scenario:
`GIVN_FEATURES=givn/changes/mask-rule-names/specs cargo test --test features_runner -- --exact "initialize-project/initialize-a-project.feature::Initialization keeps the generated rules out of the repository"`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| initialize a project | Consumer initializes a project and receives shared attribution plumbing | CLI | Real `givn init` subprocess |

## Use-case traceability

| Contract element | Technical decision |
|---|---|
| Rules stay out of the repository | Patterns ignore the reserved namespaces; files remain untracked. |
| No name leak | The block carries no individual rule name. |
| Transition still works | Predicate discovery is unchanged. |
| Personas | `developer--primary-user`, `external-contributor--open-source-consumer`, `givn-maintainer--owner`. CLI text output, Personas named. |

## Version freshness, local runnability, coverage

No versioned technology; no stack; coverage boundaries N/A.

## Arc42 impact

Chapter 8.7 describes the ignore block; it now names the pattern form. No ADR
qualifies. Recorded in `arc42.md`.

## Internal Primitives

None.
