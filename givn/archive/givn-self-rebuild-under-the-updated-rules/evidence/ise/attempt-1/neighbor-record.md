# Neighbor record — attempt 1

Trial mode: `Dry-Run Ise`, whole-product rebuild. The trial revision is
`746e660b97028991ecd273595e28047f21d8262d`; the spec copy is
`/tmp/givn-ise-self-rebuild-20261117T204903Z` and the manifest is
`attempt-1/manifest.yaml`. The record is a trial-review artifact, never a gate.

## Files changed outside the Specification or the replaced unit

| Path or consolidation | Kind | Reason | Evidence |
|---|---|---|---|
| (empty) | | The builder authored only the replaced unit: 24 files, 22 under `src/**` and 2 under `tests/**`. No declared Specification file was modified, added, or removed. | `attempt-1/builder-notes.md` `## Authored files`; `attempt-1/compare-after.txt` |

The build wrote `/tmp/givn-ise-self-rebuild-20261117T204903Z/target/**`. Those
files are the build's own output, not an authored change: `givn ise compare`
ignores paths outside the manifest's Specification set, and both subjects
report `match` after the build (`attempt-1/compare-after.txt`).

## Consolidations

| Path or consolidation | Kind | Reason | Evidence |
|---|---|---|---|
| (empty) | | No file was consolidated or merged. | `attempt-1/compare-after.txt` |

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

The frozen acceptance ran against the rebuilt binary with these inputs; none of
them constructs the product:

| Variable | Value | Role |
|---|---|---|
| `GIVN_BIN` | `/tmp/givn-ise-self-rebuild-20261117T204903Z/target/release/givn` | the rebuilt binary the suite drives |
| `GIVN_READY_FIXTURE` | built by `run-tests.sh` from the checkout's `tests/ready_fixture.rs` | test infrastructure |
| `GIVN_ROOT` | the checkout | the repository files the scenarios read |

## Promotion

Nothing was promoted. The rebuilt binary, the spec copy, and the manifest stay
outside the checkout; `S14` owns the promotion decision and this change does not
install, tag, release, or replace any product file.
