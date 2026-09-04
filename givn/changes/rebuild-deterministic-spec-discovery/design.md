# Design: rebuild-deterministic-spec-discovery

## Overview

Wholesale rewrite of four capability specs onto the deterministic
discovery commands shipped in Phases 4.1–4.5 (`spec tree`, `spec find`,
`spec duplicates`, `spec route`). The delta carries `@givn.removed` for
every superseded scenario and `@givn.added` for the replacements; the
capability-level behaviour lives in new step definitions that drive the
compiled CLI.

## Technology decisions

- New step-definition file `tests/steps/spec_discovery_deterministic_steps.rs`
  (one file per capability family, CLI-subprocess driven, grouped-layout
  fixtures).
- Old step definitions stay in place: the removal placeholders must still
  match a step function until the change archives; they become dead code
  removed with the feature flags in task 1.2.
- `givn spec duplicates` gains a tolerant missing-corpus path (empty
  corpus, exit 0) and direction-correct assertion-subset matching.

## Step Definitions

| Cap | File |
|---|---|
| spec-duplicates | `tests/steps/spec_discovery_deterministic_steps.rs` |
| spec-explore-change | `tests/steps/spec_discovery_deterministic_steps.rs` |
| spec-explore-permanent | `tests/steps/spec_discovery_deterministic_steps.rs` |
| spec-progress-output | `tests/steps/spec_discovery_deterministic_steps.rs` |

## Architecture Impact

None beyond the documented CLI behaviour. The four capabilities now hold
only deterministic scenarios, so a `--no-default-features` build can
execute them — the precondition for task 1.2.
