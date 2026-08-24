# Review: spec-progress-output

## Fabrication audit

### 0. @e2e tag integrity

- Delta feature: `givn/changes/spec-progress-output/specs/spec-progress-output/spec-progress-output.feature`.
- `@e2e` count in the first tracked commit (`62eb89b`): 4.
- `@e2e` count in the current tree: 4. No tag was removed; `@wip` was
  removed per scenario as RED progressed. CLEAN.

### 1. Empty/no-op step bodies

- Scanned `tests/steps/spec_progress_output_steps.rs` and
  `tests/steps/spec_progress_output_e2e_steps.rs` for `{}`, bare
  `pass`/`return`, and `unimplemented!()`: no stubs remain. The only `{}`
  matches are format strings in assert messages. CLEAN.

### 2. Task-commit cross-check

| Task | Commit | Production diff |
|---|---|---|
| Setup + warm cache | `62eb89b` | `src/spec/retrieval.rs` (notice, embedder-preparation, embedding count/complete) |
| Failed download | `5bcce96` | `src/spec/retrieval.rs` (acquisition start notice placement) |
| Disabled evidence | `58f993c` | none — shared notices only; new assertion step |
| Embedding multiple | `566e8ba` | `src/spec/retrieval.rs` (progress callback) |
| Cold-cache index e2e | `c6082dc` | `src/spec/retrieval.rs` (StderrDownloadProgress, download_with_progress, acquisition notices) |
| Search e2e | `4173c75` | none — shared notices; fixtures/steps only |
| Duplicates e2e | `8c5be3b` | `src/spec/retrieval.rs` (BGE/NLI acquisition notices) |
| Advisory review e2e | `bab8d25` | none — shared notices; fixtures/steps only |

The three no-production-diff commits are legitimate step-reuse cases
documented in tasks.md: their observable behavior comes from the shared
acquisition/embedding notices implemented in the other scenarios.

### 3. Components promised vs present

- `notice` helper: present (`src/spec/retrieval.rs:1067`).
- `StderrDownloadProgress` (+ `hf_hub::api::Progress` impl): present
  (`src/spec/retrieval.rs:1078-1135`).
- `download_with_progress` switch in `fetch_hf_artifacts`: present
  (`src/spec/retrieval.rs:1227`).
- Progress callback parameter in `embed_texts_with_policy`: present.
- `hf_endpoint_override` world field + env propagation: present
  (`tests/steps/mod.rs`).
- Both step files registered: present.

### 4. Strict-mode proof

tasks.md setup task records the failing run (`1 scenario (1 failed)`,
`Step panicked. Captured output: not implemented`, non-zero exit) against
the `.fail_on_skipped()` runner. Present.

### 5.-6. E2E assertion/driver fidelity

CLI capability: all four `@e2e` Then steps assert on the subprocess
interface (stdout JSON/ranks/evidence, stderr announcements), never on
repository state as the primary assertion. No browser/HTTP-substitute
concerns apply.

### 7. E2E implementation uniqueness

`verify.e2e_command` is `./run-tests.sh --e2e` (givn/commands.yaml), which
drives `tests/features_runner.rs` with `@e2e` tag filters. The only e2e
step implementation for this capability is
`tests/steps/spec_progress_output_e2e_steps.rs`; no second implementation
exists in the tree (tracked or untracked).

### 8. Canonical E2E scope

Inventory has 4 entries; exactly 4 `@e2e` scenarios exist, one per action
(index, duplicates, review, search). No excess `@e2e` scenarios.

### 9.-10. Local run + runner isolation

- Local run: CLI + in-process `ArtifactFixtureServer` (digital twin for
  huggingface.co); no containers or live third-party services.
- `verify.command` (`./run-tests.sh`) and `verify.e2e_command`
  (`./run-tests.sh --e2e`) differ; the e2e run reports 4 scenarios vs 6 for
  the non-e2e delta run — strictly fewer, isolation proven.

### 11. Design conformance

Step file names, world field, notice vocabulary, callback signature,
throttling rule, and repair-loop announcement placement all match
design.md as reviewed. No silent deviation.

### 12. Findings

None. Fabrication audit CLEAN.

### 13. Interaction coverage cross-reference

| Inventory entry | Matrix row | @e2e scenario | Driving mechanism (verified in step file) |
|---|---|---|---|
| build the permanent semantic index through the givn CLI | yes | Cold-cache indexing announces each phase on stderr | `run_givn_with_envs` subprocess (download-stub, XDG, HF_ENDPOINT -> ArtifactFixtureServer) |
| audit permanent scenarios with JSON output through the givn CLI | yes | Duplicate audit keeps stdout pure JSON while progress runs on stderr | `run_givn_with_envs` subprocess (auto/nli-auto, local stubs, score fixtures, HF_ENDPOINT) |
| run the advisory semantic review of an active change through the givn CLI | yes | Advisory review announces second-stage model acquisition during the run | `run_givn_with_envs` subprocess (stub, auto/nli-auto, local stubs, HF_ENDPOINT) |
| search the permanent semantic index through the givn CLI | yes | Search with a cold E5 cache announces model acquisition before results | `run_givn_with_envs` subprocess (download-stub, XDG, HF_ENDPOINT) |

All four rows verified 1:1; driving mechanisms match the implementations.

### 14. Coverage measurement validity

- Instrumented: `cargo llvm-cov` runs the compiled binary under test with
  `--cobertura` output; both the non-e2e and e2e profiles were produced by
  `measure-coverage.sh` and merged by `merge-coverages.sh` into
  `coverage/spec-progress-cobertura.xml`.
- Known production path non-zero: `src/spec/retrieval.rs` reports
  `2310/5605` lines hit, including every new progress/notice line.
- Coverage measurement VALID.

Coverage gap classification for this change's new code:

| Region | Hits | Bucket |
|---|---|---|
| `notice`, all announcement call sites | > 0 | covered |
| `StderrDownloadProgress::new/report/init/update/finish` | > 0 (32-128) | covered |
| `report` early return `total == 0` (line 1096) | 0 | Legitimately hard to test: hf-hub always calls `init` before `update`, so `total` is 0 only for a zero-byte artifact, which `validate_required_files` rejects before any score participates; the branch is a defensive guard for a state the validated acquisition path cannot reach. No scenario can exercise it without fabricating an invalid artifact source, which would test hf-hub internals rather than givn behavior. |

## Arc42 implementation conformance

| Chapter | Arc42 claim | Implementation evidence | Match |
|---|---|---|---|
| 05 building-block-view | `spec::retrieval` responsibility includes stderr phase status and per-file download progress | `notice`/`StderrDownloadProgress` in `src/spec/retrieval.rs` | Yes |
| 06 runtime-view (6.8.1) | index announces acquisition start/end, per-file progress, pending count, completion; warm cache silent; disabled layers silent | implemented in `auto_*_artifacts` + `refresh_index`; asserted by scenarios 2 and 5 | Yes |
| 08 crosscutting-concepts (8.5a.1) | stdout results / stderr progress, always; phases incl. embedder preparation | `eprintln!` via `notice`; stdout untouched (JSON purity scenario GREEN) | Yes |
| 10 quality-requirements | Q-63, Q-64 usability scenarios | covered by the 8 delta scenarios | Yes |

ARC42 CONFORMANCE: CLEAN.

## Overlap dispositions

No deterministic overlap findings for this change: the delta adds no
scenario titles that collide with permanent specs (`givn lint --change
spec-progress-output` exit 0), no long-scenario findings, and no
removed+added pairs.

## Verification runs

- `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh`:
  `6 scenarios (6 passed)`.
- `GIVN_FEATURES=givn/changes/spec-progress-output/specs ./run-tests.sh --e2e`:
  `2 + 2 scenarios, all passed`.
- Default `./run-tests.sh`: permanent specs GREEN (349+34+9+8+6).
- Default `./run-tests.sh --e2e`: permanent e2e GREEN (53+7+4+2).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `givn check tasks --change spec-progress-output`: exit 0.

REVIEW: PASS
