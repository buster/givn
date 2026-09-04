# Design: default-build-is-feature-free

## Overview

One scenario modified to pin the feature-free default contract. No new
components; one step-definition update in `release_0_3_0_steps.rs`.

## Technology decisions

None beyond reading `cargo metadata` (existing step). The Then steps
assert `features.default == []` and that the four optional model
features are declared but not default.

## Step Definitions

| Cap | File |
|---|---|
| release-0-3-0 | `tests/steps/release_0_3_0_steps.rs` |

## Architecture Impact

None — pins the already-made Cargo.toml decision.
