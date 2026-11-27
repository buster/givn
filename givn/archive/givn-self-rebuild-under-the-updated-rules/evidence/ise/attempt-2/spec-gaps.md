# Spec gaps — attempt 2

Trial mode: `Dry-Run Ise`, whole-product rebuild. This file classifies
`attempt-2`, the last attempt with a completed End-to-end run. The trial
revision is `746e660b97028991ecd273595e28047f21d8262d`, recorded in
`evidence/ise/trial-revision.txt`. The spec copy is
`/tmp/givn-ise-self-rebuild-20261117T204903Z`; the copy and manifest were
prepared by attempt 1 (`attempt-1/manifest.yaml`), and this attempt rebuilds
the same copy after the `implementation defect` repair round without repeating
`rebuild-all`.

The frozen acceptance command was:

```sh
GIVN_BIN=/tmp/givn-ise-self-rebuild-20261117T204903Z/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-2/e2e-result.json \
./run-tests.sh --e2e --package e2e
```

`GIVN_BIN` names the rebuilt binary. `GIVN_READY_FIXTURE` and `GIVN_ROOT`
are the checkout's test infrastructure and repository files; neither
constructs the product. The per-difference table is
`attempt-2/gap-table.md`.

## Attempt counts

| Attempt | Total | Passed | Failed | Skipped | Binary SHA-256 | Result file | Run log |
|---|---|---|---|---|---|---|---|
| `attempt-1` | 120 | 33 | 87 | 0 | `f943ab19a5e802985f153216a1cc4339be98ceafc514f5eecf7c4597d0146fb8` | `attempt-1/e2e-result.json` | `attempt-1/e2e-run.log` |
| `attempt-2` | 120 | 41 | 79 | 0 | `800939a056038a7f7c5cd04c35ba8e142a8f81945e3f61f409c550229861c719` | `attempt-2/e2e-result.json` | `attempt-2/e2e-run.log` |

The repair round edited 13 files under `src/**` in the copy and added none; the
declared Specification and the frozen corpus were not touched. The repair fixed
8 rows and introduced no failure:

| Fixed row | Feature |
|---|---|
| `An upgrade preserves the extension and the next request serves it` | A project extends an instruction's built-in rules |
| `Use-case tree reports explicit use-case ownership` | Canonical use-case contract |
| `Developer rebuilds the whole product from its Specification` | Clean-room preparation |
| `Coverage-enabled project receives a complete README coverage example` | Coverage README guidance |
| `The spec tree reports per-use-case counts and the use-case narrative` | Find existing scenarios |
| `Coverage-enabled project is guided to place the coverage badge directly after the README title` | Show coverage in the README |
| `Consumer invokes givn check with an unknown artifact and gets a helpful error` | Validate artifact ids |
| `A duplicate title across two permanent spec files is a hard error` | spec-overlap-gates |

The counts include the 115 selected `@e2e` definitions and the 5 outline
examples.

## Setup failures

None. The run compiled the frozen package, started the runner with the rebuilt
binary, and completed with exit 1 because 79 scenarios failed. No scenario was
skipped and the result file matches the run log's summary.

## Outcome counts

| Outcome | Rows | Wrong artifact |
|---|---|---|
| `implementation defect` | 79 | the rebuilt file in the spec copy, named per row in `gap-table.md` |
| `specification defect` | 0 | none |
| `E2E test defect` | 0 | none |
| `acceptable undocumented change` | 0 | none |

Every difference received exactly one outcome before any edit. No difference
was closed as drift. The 79 `implementation defect` rows are classification
evidence; they are not repaired further in this change, and the rebuilt files
stay in the spec copy.

## The five former Specification defects

The prior trial recorded five `specification defect` rows against
`givn/specification.yaml`: four `attribution_asset` surfaces (OpenCode, Claude,
omp, and Maki embedded integration text) and the product version (`0.9.10`
where the frozen test reads `0.9.20`). The declaration now names
`assets/agent-integrations/**` under `behavior` and the manifest and toolchain
pin under `contracts`/`limits`; both compare runs report `match` before and
after the build.

| Former Specification defect | Attempt-2 evidence | Resolution | Residual difference |
|---|---|---|---|
| OpenCode embedded integration text | The copy holds `assets/agent-integrations/opencode/**`; `OpenCode prepares attribution using its selected model` passes every step. | Resolved by the declaration. | None. |
| Claude embedded integration text | The copy holds `assets/agent-integrations/claude/**`; the scenario passes asset setup and model discovery, then fails when the hook calls the rebuilt binary. | Resolved as a Specification defect. | `implementation defect`: the rebuilt binary lacks the Claude pre-tool-use subcommand the integration calls (`gap-table.md` row 19). |
| omp embedded integration text | The copy holds `assets/agent-integrations/omp/**`; the scenario reaches the commit and the committer identifies the LLM and harness. | Resolved as a Specification defect. | `implementation defect`: the rebuilt stamp writes no LLM co-author trailer (`gap-table.md` row 20). |
| Maki embedded integration text | The copy holds `assets/agent-integrations/maki/**`; the generated `init.lua` loads. | Resolved as a Specification defect. | `implementation defect`: the generated integration's revision command fails (`gap-table.md` row 21). |
| Product version `0.9.10` versus `0.9.20` | `rebuilt-version.txt` records `givn 0.9.20`; `Developer rebuilds the whole product from its Specification` passes every step, including the manifest-version assertion. | Resolved by the declared `Cargo.toml`. | None. |

Five of five former `specification defect` rows are resolved. The final table
holds no `specification defect` row for those surfaces.

## Residual implementation defects

`gap-table.md` holds 79 rows in 24 groups. The rebuilt files named there are the
files the builder authored in the spec copy under `src/**`; none is a declared
Specification file. The largest groups are attribution integration protocol (8),
the development report (8), archive behavior (15), init, upgrade, and skills
sync (10), run declaration and start (10), check and receipt (7), history JSON
(6), and guidance (4). The trial promotes nothing: the rebuilt binary, the spec
copy, and the manifest remain outside the checkout.

## Neighbor record

`attempt-2/neighbor-record.md` records that the builder authored only the
replaced unit (24 files: 22 under `src/**`, 2 under `tests/**`, 13 of them
edited by the repair), that no declared Specification file was modified, added,
or removed, that the clean-room scan printed nothing for the three Cargo
stores, and that no rebuilt file was promoted. Attempt 1's binary hash is no
longer present in the copy because the repair rebuilt it in place; attempt 1's
result files remain the record of that attempt.
