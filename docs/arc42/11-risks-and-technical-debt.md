# 11. Risks and Technical Debt

This chapter distinguishes implemented safeguards from open limitations. A
historical measurement is not a current quality claim.

## 11.1 Risks

| ID | Risk | Likelihood | Impact | Mitigation / current boundary |
|---|---|---:|---:|---|
| R-1 | Replaced scenarios cannot preserve every original docstring delimiter detail. | Medium | Medium | The text-level merge preserves content and tables but renders replacement docstrings from the parsed AST. |
| R-2 | Overlay semantics are misunderstood. | Medium | High | Artifact patches deep-merge by id; top-level `verify`, `coverage`, and `apply` overlays replace blocks. Keep the effective-manifest order documented. |
| R-3 | Static or feature-enabled native linking is not proven for every target. | Medium | High | Keep retrieval feature-gated and treat musl, Windows, and release artifacts as unverified until built. |
| R-4 | Users assume Givn owns all of `AGENTS.md`. | Medium | High | Givn owns only its matching fenced blocks; all surrounding content is user-owned and must be preserved. |
| R-5 | Archive failure could leave a changed permanent tree or README. | Low | Critical | Back up permanent specs and README, validate merge preconditions, and roll back post-merge verify, E2E, and coverage failures. |
| R-6 | Arc42 documentation becomes stale or is treated as a generated copy. | Medium | High | Arc42 completion review compares durable chapters, change assessment, design, tasks, and implementation; integrity checks resolvable affected files against templates, while missing files remain a current helper gap. |
| R-7 | Generated config comments describe old post-hook defaults or nonexistent Arc42 settings. | High | Medium | The runtime contract is the current embedded manifest and source: standard post lists are empty, mandatory checks are binary-owned, and Arc42 output is fixed. |
| R-8 | E2E detection misses scenarios nested under `Rule:` because the current helper checks direct feature scenarios only. | Medium | High | Keep the limitation explicit in review and add coverage before treating nested E2E detection as complete. |
| R-9 | The retrieval model or tokenizer is unavailable. | Medium | High | Feature-free retrieval fails explicitly; feature-enabled commands report setup failure rather than a semantic pass. |
| R-10 | The persistent E5 index is stale or malformed. | Medium | High | Refresh validates model/version, identities, dimensions, and hashes and rebuilds permanent entries when necessary. |
| R-11 | Complete semantic serialization increases token count and changes vector reuse. | Medium | High | Hash the exact serialization, preflight actual tokenizers, expose cap evidence, and re-embed on hash mismatch. |
| R-12 | The complete Duplicate Audit is quadratic in permanent corpus size. | Medium | Medium | Embed each fitting scenario once and score remaining pairs in memory; measure runtime before introducing a bounded/indexed redesign. |
| R-13 | E5 candidate generation misses a semantic duplicate. | Medium | Critical | Keep the `0.9500` gate and measure candidate recall against labeled evidence; E5 remains the recall boundary for BGE/NLI. |
| R-14 | BGE filters a true duplicate or variant. | Medium | Critical | Preserve raw E5 evidence, require matching approval for default BGE, and treat the BGE operating point as non-classifying evidence. |
| R-15 | NLI filters a true duplicate or variant. | Medium | Critical | Preserve raw E5 and directional NLI evidence, keep NLI independent and non-authoritative, and measure labeled admission quality. |
| R-16 | BGE and NLI pool descriptions diverge from implementation by surface. | Medium | High | Duplicate Audit BGE uses its top-five endpoint union while Duplicate Audit NLI uses the full E5 semantic set; blocking review uses complete report candidates, not a universal top-five pool. |
| R-17 | Benchmark approval is mistaken for runtime artifact integrity. | Medium | High | Runtime approval checks schema/model/threshold/top-k/budget/pipeline/pass fields; separate artifact preflight checks local files. The current runtime does not recheck benchmark file hashes. |
| R-18 | Prior semantic classifications survive malformed or changed report state. | Medium | Critical | Current extraction requires recognized IDs, allowed classes, rationale, and matching endpoint evidence hashes, but it is line-oriented rather than a strict full grammar parser. Strengthen the parser before claiming strict rejection. |
| R-19 | A classified `delta-to-delta` candidate cannot be verified by the current remediation scope check. | High | High | Treat it as an explicit limitation; the verifier currently requires `current-change` to `permanent` endpoint scopes. Do not document delta-to-delta remediation as supported. |
| R-20 | Review refreshes the persistent index before parsing delta content. | Medium | High | `spec explore --change` validates before refresh, but advisory and blocking review refresh first. Preserve the distinction and add a transactional ordering fix in a future change. |
| R-21 | The second-stage budget leaves lower-ranked candidates without model evidence. | Medium | High | Keep `*_UNSCORED` separate from `*_FILTERED`, retain E5 evidence, and require completion of the open `semantic-reranker-budget` change before accepting its ADR. |
| R-22 | PR #27 measurements are mistaken for reproducible repository state. | Medium | Medium | The current report records `394/1274/795`, but it depends on local ephemeral raw JSON reports. Link the report and label all earlier measurements as historical. |
| R-23 | Ordinary design, process, or implementation choices inflate the active ADR contract. | High | High | Apply the strict five-dimension AND gate, reject shared-guidance-only claims, and route each non-ADR decision to exactly one canonical artifact. |
| R-24 | ADR consolidation loses unique rationale, relationships, or ownership. | Medium | High | Use the complete ID-by-ID disposition manifest, preserve only genuine superseded architecture history, and review every target link and canonical source before removing a record. |
| R-25 | The qualification procedure drifts between the canonical addon instruction and derived design-review guidance. | Medium | High | Keep the complete procedure and MADR template in one source, make derived assets thin, and verify effective/generated guidance parity. |
| R-26 | Authors treat the LLM verdict as machine-enforced architecture validation. | Medium | High | State the non-runtime boundary explicitly and require human/design-review evidence; the current integrity hooks do not infer architectural significance. |
| R-27 | Automatic upgrade commits a generated file that a project has treated as user-owned. | Medium | High | Require a clean worktree, stage an explicit known-path allowlist, preserve ejected facets, and expose both commit diffs for review. |
| R-28 | Commit A succeeds but Commit B fails, leaving the generated migration plan uncommitted. | Low | High | Never reset history; keep the plan visible, report the successful hash and remaining files, and make rerun behavior preserve the existing plan. |
| R-29 | A project interprets `givn_config_version: 0.3.0` as proof that semantic migration is complete. | High | High | Create the visible maintenance change, record the distinction in its design, and require normal LLM review/archive completion. |
| R-30 | A future migration catalog contains a broken edge or incomplete bundle and leaves a version gap unclear. | Medium | High | Validate schema, unique IDs, advancing versions, referenced assets, ordered paths, and fail explicitly once a declared route has started. |
| R-31 | Multiple required bundle phases are treated as independent concurrent changes. | Medium | High | Materialize one aggregate maintenance change with `migration.yaml`, explicit sequence numbers, and prompts that require strict phase order. |
| R-32 | Enabling all declared retrieval features by default increases native build cost or exposes unavailable local model artifacts at first use. | Medium | High | Keep features independently selectable, report missing artifacts explicitly, document `--no-default-features`, and retain the same non-silent unavailable behavior. |

## 11.2 Deliberate Technical Debt

| ID | Debt | Current state | Repayment |
|---|---|---|---|
| D-1 | No `givn migrate` from OpenSpec. | Clean break is current. | Add an explicit reviewed migration command if demand justifies it. |
| D-2 | Release binaries, Homebrew, shell installer, and target matrix are not wired in this checkout. | Distribution is planned. | Implement and verify the release pipeline before claiming those channels. |
| D-3 | Generated config comments can lag the embedded runtime manifest. | `commands.yaml` and mandatory-check behavior moved ahead of older comments. | Update the asset template in a separate code change; durable Arc42 docs describe runtime behavior, not stale comments. |
| D-4 | `givn new` writes the proposal template verbatim, including its literal placeholder. | Characterized behavior. | Add a reviewed delta to substitute or remove the placeholder. |
| D-5 | `givn new` checks existing change before missing `givn/`, making one branch unreachable in practice. | Characterized behavior. | Reorder checks in a reviewed change if the error distinction matters. |
| D-6 | Natural `givn new -foo` is rejected by `clap`, while `givn new -- -foo` reaches the id validator. | Characterized behavior. | Unify the two input paths if a consistent diagnostic is required. |
| D-7 | `givn init` does not create `givn/artifacts/` proactively. | Eject can create needed parent paths, but the directory is absent initially. | Add explicit scaffolding or improve eject guidance. |
| D-8 | Replaced scenario docstrings may lose delimiter/content-type formatting. | Content is retained; source delimiter style is not. | Carry source block formatting in a future merge change. |
| D-9 | Rule-nested WIP/E2E checks use existing direct-scenario traversal limits. | Current parser helper limitation. | Extend traversal and add regression coverage. |
| D-10 | Prior semantic report parsing is not strict and does not reject arbitrary unknown/reordered fields. | Current line-oriented implementation. | Implement a strict grammar parser before treating the wire format as enforced input validation. |
| D-11 | Semantic remediation does not support prior `delta-to-delta` entries. | Scope precondition rejects them. | Add a deliberate projected-delta remediation design and tests. |
| D-12 | The current second-stage budget change is unfinished. | `givn status --change semantic-reranker-budget` reports open tasks and final verification. | Complete RED/GREEN/REFACTOR, full/E2E tests, real-model evidence, and blocking review before accepting ADR-063. |
| D-13 | ADR qualification is guidance-level and the register has no semantic validator. | The LLM/author and design review apply the policy; runtime integrity checks only verify mechanical file conditions. | Add a reviewed semantic register validator only if a future requirement justifies runtime enforcement; until then retain explicit review evidence. |

## 11.3 Historical Measurements

The following values came from earlier planning or corpus snapshots and are not
the current baseline: `288/414?`, `385/100`, and `397/1311/803`.

The current PR #27 report records `394` permanent scenarios, `1274` E5 semantic
candidates, and `795` BGE top-five-union pairs. It also records that BGE and NLI
have different audit pools and that raw JSON reports were stored locally and
ephemerally. Use the [current report](../semantic-duplicates-real-model-filter-report.md)
for its scope and provenance; do not turn its counts into a permanent invariant.

## 11.4 Resolved Boundaries

- Active change ids are validated as lowercase ASCII kebab-case before the
  shared change path is constructed.
- E5, BGE, and NLI do not silently truncate over-cap inputs in production.
- A capped E5 passage does not discard fitting vectors from the same refresh.
- Duplicate Audit does not write `.spec-index.json`, active changes, archive
  contents, or feature files.
- BGE/NLI statuses do not assign Givn classifications or perform feature edits.
- Givn does not overwrite user content outside its own `AGENTS.md` fenced blocks.
