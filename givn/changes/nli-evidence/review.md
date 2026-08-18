# Review: nli-evidence

## Fabrication Audit

The audit was performed before relying on the checked task list.

| Check | Evidence | Result |
|---|---|---|
| E2E tag integrity | The delta contains exactly five `@givn.added @nli @e2e` scenarios. The `@e2e` tag was never removed; only `@wip` was removed as each scenario completed. | CLEAN |
| Empty/no-op step bodies | `tests/steps/nli_evidence_steps.rs` and `tests/steps/nli_evidence_e2e_steps.rs` contain no `unimplemented!`, `todo!`, empty body, bare `return`, or no-op step. The repository step modules were scanned for the same stub patterns. | CLEAN |
| Checked task commits | Every scenario task names a commit touching `src/spec/retrieval.rs` or the real CLI E2E step module. Setup strict proof is recorded under `6c5fbf0`; the scenario commits are listed below. | CLEAN |
| Promised components | `src/spec/retrieval.rs`, `run-tests.sh`, `measure-coverage.sh`, `tests/steps/nli_evidence_steps.rs`, `tests/steps/nli_evidence_e2e_steps.rs`, and the delta feature all exist. | CLEAN |
| Strict-mode proof | The targeted first run exited `101` at an undefined step under `.fail_on_skipped()`. The setup evidence is recorded in `tasks.md`. | CLEAN |
| E2E primary assertions | All five E2E scenarios invoke the compiled `givn` CLI as a subprocess and assert CLI stdout/exit status. The blocking report is secondary evidence for the CLI action. | CLEAN |
| E2E command | `givn/commands.yaml` configures `./run-tests.sh` and `./run-tests.sh --e2e`; the latter invokes the real runner with `@e2e` filters. They are not identical. | CLEAN |
| Local infrastructure | The local command is `./run-tests.sh`; the CLI has no server, database, cloud service, or external dependency requiring a digital twin. | CLEAN |
| Design adherence | The implementation uses the two step files, `cucumber-rs`, the compiled CLI, local stubs, the shared bounded E5 pool, and the coverage matrix described in `design.md`. | CLEAN |

### Scenario Commit Evidence

| Scenario | Production/e2e commit |
|---|---|
| NLI reports a symmetric probability summary from both directions | `a0cbb6a` |
| BGE and NLI jointly reduce the recommended candidate set | `e42afc6` |
| An unavailable NLI layer keeps a combined candidate blocking | `6d4dd95` |
| NLI refuses an overflowing bounded pair pool before model loading | `3f7bf6e` |
| NLI keeps candidates outside a partial bounded pool blocking | `77fceea` |
| NLI JSON preserves the raw E5 contract and uses the NLI schema extension | `b5f1c20` |
| Disabling NLI preserves the E5-only contract in an NLI-capable build | `5c2fb58` |
| Missing NLI artifacts preserve the explicit E5 fallback | `2760564` |
| The NLI admission boundary includes a symmetric entailment probability of 0.5000 | `71ecbfb` |
| An NLI-recommended candidate still blocks unresolved semantic review | `4360171`, follow-up `a4ee76c` |
| The independent NLI capability owns the legacy diagnostic selector | `b6b7aac`, follow-up `2d14cea` |
| NLI reduces permanent exploration candidates without changing E5 raw results | `444ef0e` |
| NLI reduces change-aware exploration candidates without changing E5 raw results | `d1dc253` |
| NLI reduces advisory review candidates without assigning a Givn class | `39b18ee` |
| NLI filtered evidence does not block the blocking review | `002887d` |
| NLI uses the bounded E5 neighbour union for the permanent duplicate audit | `bcef01a`, follow-up `ac144ca` |

## Interaction Coverage

The canonical inventory has five entries. Each maps to exactly one E2E
scenario, and every driver is a real compiled CLI subprocess.

| Inventory entry | E2E scenario | Step file and primary assertion |
|---|---|---|
| Explore a draft against permanent candidates | NLI reduces permanent exploration candidates without changing E5 raw results | `tests/steps/nli_evidence_e2e_steps.rs`; `givn spec explore --file`, stdout and exit code |
| Explore a draft against permanent and active-change candidates | NLI reduces change-aware exploration candidates without changing E5 raw results | `tests/steps/nli_evidence_e2e_steps.rs`; `givn spec explore --change --file`, scopes and stdout |
| Run advisory change review with NLI evidence | NLI reduces advisory review candidates without assigning a Givn class | `tests/steps/nli_evidence_e2e_steps.rs`; `givn spec review --change`, advisory stdout |
| Run blocking change review with NLI evidence | NLI filtered evidence does not block the blocking review | `tests/steps/nli_evidence_e2e_steps.rs`; `givn check review --change`, exit status and report |
| Run the permanent duplicate audit with NLI evidence | NLI uses the bounded E5 neighbour union for the permanent duplicate audit | `tests/steps/nli_evidence_e2e_steps.rs`; `givn spec duplicates`, text output |

No browser or HTTP interface exists. No E2E scenario is repository-only or a
downgraded lower-level substitute.

## Arc42 Implementation Conformance

The Arc42 addon is enabled. The independent completed-implementation assessment
matches the change-level claim in `arc42.md` for all twelve chapters.

| Arc42 chapter/fact | Durable source | Design/tasks mapping | Implementation evidence | Match |
|---|---|---|---|---|
| Goals and NLI worklist reduction | `01-introduction-and-goals.md` FR-34 | Product boundary and reduction scenarios | NLI admissions retain E5 raw candidates | Yes |
| Shared bounded local constraint | `02-architecture-constraints.md` TC-21 | Shared top-K/512 design; pair-cap task | `NLI_MAX_PAIRS`, shared pool, explicit refusal | Yes |
| CLI context and output surfaces | `03-context-and-scope.md` | Five inventory rows and E2E matrix | Explore/review/duplicate dispatch | Yes |
| Retrieve-then-rerank strategy | `04-solution-strategy.md` | Independent feature matrix | `nli-evidence` plus BGE intersection | Yes |
| Retrieval building block | `05-building-block-view.md` | `src/spec/retrieval.rs` task evidence | `NliEvidence`, `NliRun`, renderers | Yes |
| NLI runtime sequence | `06-runtime-view.md` | Directional, unavailable, and combined scenarios | Bidirectional scoring and fallback paths | Yes |
| Local artifact deployment | `07-deployment-view.md` | Artifact-preflight design | `.spec-nli-cache` preflight; no downloader | Yes |
| Cross-cutting statuses and schemas | `08-crosscutting-concepts.md` | NLI JSON/report tasks | `NLI_RECOMMENDED`, `NLI_FILTERED`, `NLI_UNAVAILABLE`, schema 3 | Yes |
| ADR-058 independent NLI layer | `09-architecture-decisions.md` | Design-review and implementation commits | Shared bounded pool and intersection | Yes |
| NLI quality scenarios | `10-quality-requirements.md` | 16 NLI scenarios and matrix | Full NLI and E2E runs green | Yes |
| NLI risks R-62 through R-65 | `11-risks-and-technical-debt.md` | Fallback, calibration, pool-parity tasks | Raw/unscored preservation and local preflight | Yes |
| NLI vocabulary | `12-glossary.md` | Proposal/design terminology | Status/evidence names match output | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Deterministic Gates

`givn lint --change nli-evidence` is clean. The review run reported the net
delta as `15 added, 1 modified, 0 removed` and `overlap dispositions passed`.
No unresolved shape match or long-scenario split-or-keep finding remains. The
single modified legacy scenario is an intentional replacement of the dormant
NLI diagnostic scenario; no removed+added pair exists.

## Coverage Measurement

`measure-coverage.sh` ran the instrumented `features_runner` and compiled CLI
for the active NLI feature path. `merge-coverages.sh` produced a fresh
`coverage/cobertura-coverage.xml`.

| Metric | Covered | Valid | Result |
|---|---:|---:|---:|
| Lines | 6307 | 16894 | 37.33% |
| Branches | 0 | 0 | n/a |

The merged report includes both the non-E2E and E2E Gherkin runner processes;
the known CLI Explore/review/duplicate paths have non-zero production hits.
Coverage classification uses only the three allowed buckets:

| Gap | Bucket | Resolution |
|---|---|---|
| Obsolete diagnostic-only NLI selector helpers | Dead code | Removed from `src/spec/retrieval.rs` before sign-off. |
| Real local ONNX NLI inference with a provisioned multilingual model | Legitimately hard to test | Network and model downloads are forbidden; deterministic stub, directional seams, artifact preflight, and unavailable paths cover the observable contract. |
| Existing repository paths outside this change with uncovered lines | Missing test coverage | Not introduced by this change; the existing permanent feature matrix remains the owning coverage surface. No NLI behavior is excused by this classification. |

`COVERAGE MEASUREMENT: VALID`

## Verification

`givn lint --change nli-evidence` exits 0. `./run-tests.sh` exits 0 across the
E5, BGE, NLI-only, combined, and disabled matrices. The full NLI feature scope
passes 16/16 scenarios; the E2E scope passes 5/5. `cargo fmt --check`, the
standard workspace clippy command, retrieval-enabled checks, and the
feature-free compatibility check pass.

## Sign-Off

Fabrication audit: clean. All promised components exist. No `@wip` tags remain.
The canonical E2E policy is satisfied. All interaction rows have one real CLI
E2E scenario. No implementation-layer detail appears in the feature spec.

REVIEW: PASS
