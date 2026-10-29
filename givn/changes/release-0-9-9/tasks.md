# Tasks: release-0-9-9

Release-owned maintenance change: `specs` is skipped via `.givn-skip`. The
deliverable is the 0.9.9 version bump plus the changelog section naming the
archived `bound-review-context` and `default-all-addons` changes. No production
behavior changes in this change.

## Setup

- [x] No new runner or step definitions: `specs` is skipped and the existing
      suite is inherited unchanged. `tests/features_runner.rs` keeps
      `.fail_on_skipped()`.
- [x] Version surface confirmed: `env!("CARGO_PKG_VERSION")` feeds
      `givn init`, `{{GIVN_VERSION}}`, and the `givn upgrade` target version,
      so the Cargo bump is the single version source.

## Release deliverables

### Version bump to 0.9.9

- [x] `Cargo.toml` is `version = "0.9.9"`.
- [x] `Cargo.lock` workspace package `givn` is `0.9.9`.
- [x] `givn/config.yaml` is `givn_config_version: "0.9.9"`.
- [x] `cargo check --locked` succeeds and `cargo run -- --version` prints
      `givn 0.9.9`.
- [x] COMMIT: `67969e49` -- `release(release-0-9-9): 0.9.9 version bump` (Batch: release-0-9-9)

### Changelog

- [x] `CHANGELOG.md` contains `## [0.9.9] - 2026-10-29`.
- [x] The Features section names the bounded review context and the
      all-add-ons-enabled opt-out default.
- [x] The Bug Fixes section names the routing diagnostic.
- [x] COMMIT: `67969e49` -- `release(release-0-9-9): 0.9.9 version bump` (Batch: release-0-9-9)

## Closeout

- [x] `./run-tests.sh --non-e2e` exits 0; `731 tests passed`.
- [x] `./run-tests.sh --e2e` exits 0; `112 tests passed`.
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` is clean.
- [x] `givn check review --change release-0-9-9` passed: integrity, receipt,
      overlap, and `outline review: PASS` with zero touched units.
- [x] `givn archive --change release-0-9-9` runs in the same pass; the archive
      gate reuses the current receipt and runs the merged permanent suite.
