# Neighbor record — attempt 2

Trial mode: `Dry-Run Ise`, whole-product rebuild. The trial revision is
`746e660b97028991ecd273595e28047f21d8262d`; the spec copy is
`/tmp/givn-ise-self-rebuild-20261117T204903Z` and the manifest is
`attempt-1/manifest.yaml` (the copy was prepared once, by attempt 1; this
attempt rebuilds the same copy after an `implementation defect` repair round).
The record is a trial-review artifact, never a gate.

## Files changed outside the Specification or the replaced unit

| Path or consolidation | Kind | Reason | Evidence |
|---|---|---|---|
| (empty) | | The builder authored only the replaced unit: 24 files, 22 under `src/**` and 2 under `tests/**`. The repair round edited 13 of those `src/**` files and added none. No declared Specification file was modified, added, or removed. | `attempt-2/builder-notes.md` `## Authored files`; `attempt-2/compare-after.txt` |

The build wrote `/tmp/givn-ise-self-rebuild-20261117T204903Z/target/**`. Those
files are the build's own output, not an authored change: `givn ise compare`
ignores paths outside the manifest's Specification set, and both subjects
report `match` after this attempt's build (`attempt-2/compare-after.txt`).

The repair rebuilt the copy in place, so the binary recorded by attempt 1
(`f943ab19a5e802985f153216a1cc4339be98ceafc514f5eecf7c4597d0146fb8`) is no
longer present in the copy. Attempt 1's `rebuilt-version.txt` and
`e2e-result.json` remain the record of that attempt; this attempt's
`rebuilt-version.txt` records `800939a056038a7f7c5cd04c35ba8e142a8f81945e3f61f409c550229861c719`.

## Consolidations

| Path or consolidation | Kind | Reason | Evidence |
|---|---|---|---|
| (empty) | | No file was consolidated or merged. | `attempt-2/compare-after.txt` |

## Clean-room scan

The scan read the three Cargo stores the sandbox binds and printed nothing:

| Store | Result |
|---|---|
| `~/.cargo/registry/src` | present, no `givn` or `givn-*` path |
| `~/.cargo/registry/cache` | present, no `givn` or `givn-*` path |
| `~/.cargo/git` | present, no `givn` or `givn-*` path |

No path was quarantined and no fresh `CARGO_HOME` was needed. The empty
transcript is `evidence/ise/clean-room-scan.txt`.

## Test-infrastructure inputs

| Variable | Value | Role |
|---|---|---|
| `GIVN_BIN` | `/tmp/givn-ise-self-rebuild-20261117T204903Z/target/release/givn` | the rebuilt binary the suite drives |
| `GIVN_READY_FIXTURE` | built by `run-tests.sh` from the checkout's `tests/ready_fixture.rs` | test infrastructure |
| `GIVN_ROOT` | the checkout | the repository files the scenarios read |

## Promotion

Nothing was promoted. The rebuilt binary, the spec copy, and the manifest stay
outside the checkout; `S14` owns the promotion decision and this change does
not install, tag, release, or replace any product file.
