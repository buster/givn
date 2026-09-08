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
| R-9 | The retrieval model or tokenizer is unavailable. | Medium | High | Explicit semantic use attempts validated acquisition in the XDG cache; feature-free retrieval still fails explicitly, and feature-enabled commands report setup failure rather than a semantic pass. |
| R-10 | The persistent E5 index is stale or malformed. | Medium | High | Refresh validates model/version, identities, dimensions, and hashes and rebuilds permanent entries when necessary. |
| R-11 | Complete semantic serialization increases token count and changes vector reuse. | Medium | High | Hash the exact serialization, preflight actual tokenizers, expose cap evidence, and re-embed on hash mismatch. |
| R-12 | The complete Duplicate Audit is quadratic in permanent corpus size. | Medium | Medium | Embed each fitting scenario once and score remaining pairs in memory; measure runtime before introducing a bounded/indexed redesign. |
| R-13 | E5 candidate generation misses a semantic duplicate. | Medium | Critical | Keep the `0.9000` admission boundary with hybrid selection and measure candidate recall against labeled evidence; E5 remains the recall boundary for BGE/NLI. |
| R-14 | BGE filters a true duplicate or variant. | Medium | Critical | Preserve raw E5 evidence, keep benchmark approval separate from runtime activation, and treat the BGE operating point as non-classifying evidence. |
| R-15 | NLI filters a true duplicate or variant. | Medium | Critical | Preserve raw E5 and directional NLI evidence, keep NLI independent and non-authoritative, and measure labeled admission quality. |
| R-16 | BGE and NLI pool descriptions diverge from implementation by surface. | Medium | High | Duplicate Audit BGE uses its top-five endpoint union while Duplicate Audit NLI uses the full E5 semantic set; blocking review uses complete report candidates, not a universal top-five pool. |
| R-17 | Benchmark approval is mistaken for runtime artifact integrity. | Medium | High | Runtime approval checks schema/model/threshold/top-k/budget/pipeline/pass fields; separate artifact preflight checks local files. The current runtime does not recheck benchmark file hashes. |
| R-18 | Prior semantic classifications survive malformed or changed report state. | Medium | Critical | Current extraction requires recognized IDs, allowed classes, rationale, and matching endpoint evidence hashes, but it is line-oriented rather than a strict full grammar parser. Strengthen the parser before claiming strict rejection. |
| R-19 | A classified `delta-to-delta` candidate cannot be verified by the current remediation scope check. | High | High | Treat it as an explicit limitation; the verifier currently requires `current-change` to `permanent` endpoint scopes. Do not document delta-to-delta remediation as supported. |
| R-20 | Review refreshes the persistent index before parsing delta content. | Medium | High | `spec explore --change` validates before refresh, but advisory and blocking review refresh first. Preserve the distinction and add a transactional ordering fix in a future change. |
| R-21 | Rulebook selection leaves pairs beyond the neighbour count and pair cap without model evidence. | Medium | High | Selection is deterministic by E5 score; unselected pairs are not candidates. The emergency budget tail stays `*_UNSCORED` and blocking, separate from `*_FILTERED`. |
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
| R-32 | Enabling all declared retrieval features by default increases native build cost or triggers large first-use downloads. | Medium | High | Keep features independently selectable, download only on explicit semantic use into the XDG cache, report missing artifacts explicitly, document `--no-default-features`, and retain the same non-silent unavailable behavior. |
| R-33 | Force-staging an ignored managed path could accidentally include unrelated user content. | Medium | Critical | Resolve individual managed paths before staging, use `git add -f --` only for that allowlist, never stage directories, and retain the clean-worktree preflight. |
| R-34 | A shared user cache can be corrupted or changed outside a project review. | Medium | High | Validate every required file before use, retain explicit local artifact overrides, and distinguish artifact integrity from BGE admission. |
| R-35 | Automatic model acquisition consumes disk space or network bandwidth unexpectedly. | Medium | Medium | Trigger downloads only from explicit semantic commands, keep role-specific cache directories, reuse snapshots across projects, and report the resolved path. |
| R-36 | The XDG environment is incomplete or nonconforming. | Low | Medium | Require an absolute `XDG_CACHE_HOME` or fall back to `$HOME/.cache`; fail with the required environment and explicit override when neither is usable. |
| R-37 | Benchmark tooling reintroduces project-local or `/tmp` model defaults. | Medium | Medium | Derive E5, BGE, NLI, and flat-export defaults from the shared XDG cache and assert the shell helper's default output through its real subprocess path. |
| R-38 | `spec search --evidence` incurs a second-stage model load per invocation and may be slow without artifacts. | Medium | Low | Keep the flag opt-in, reuse the shared XDG caches and existing unavailable fallback, and report an unchanged E5-only ranking when a layer cannot score. |
| R-39 | A true duplicate above the E5 threshold that both second-stage models filter passes review unflagged under corroborated gating. | Low | High | Preserve raw E5 evidence in verbose output, JSON, and findings documents; keep benchmark approval separate from runtime activation; treat the union rule as fail-safe (any layer unable to score restores the binding threshold). |
| R-40 | One shared thread budget forces per-stage tuning trades and can oversubscribe container quotas. | Medium | Medium | Default to quota-aware `available_parallelism()`, document that any future inter-stage overlap must split the budget first, and let operators cap compute with `GIVN_SPEC_THREADS`. |
| R-41 | The ephemeral port allocated for a `port_env` product is claimed by another process between givn's bind-0 probe and the child's bind (TOCTOU). | Low | Medium | Accept the narrow race; givn reports the child's early exit or the readiness timeout instead of silently retrying allocation, and the ready line names the actual port. |
| R-42 | `givn run` signal forwarding is unix-only; a Windows host running a server product gets a preflight refusal, which may surprise cross-platform projects. | Medium | Medium | The refusal is explicit at preflight with exit 1 and a named platform constraint; cli/library products work everywhere; the constraint is documented in the template and specs. |
| R-43 | A project declares `product_type: library` to escape the server requirements (declaration spoofing). | Medium | Medium | Deliberately uncountered inside givn: a heuristic would guess and block on guesses. The declaration is versioned and README-rendered, so a wrong entry is visible in review, and the caller's preview gate treats a falsified entry as an obviously wrong record, not a green start. |
| R-44 | Signal forwarding reaches only givn's direct `sh -c` child; grandchildren of compound start commands (`a && b`, pipelines) survive the interrupt. | Medium | Medium | Documented residual (unix-boring tradeoff): start commands should use `exec` so the shell process is replaced by the product; givn does not reimplement process-group supervision, and the trap-marker teardown scenarios cover the direct child. |
| R-45 | The `givn run` machine contract (ready line format, exit codes, signal behaviour) becomes a durable public compatibility obligation once it ships; breaking it breaks kiekste and caller scripts. | Medium | High | The contract is pinned by executable specs — the `run-execution` @e2e ready-line/exit-code scenarios and the spec'd exit-code table; changing any of these surfaces requires a spec change, which is visible in review. |
| R-46 | Ideation topic state (`givn/ideation/`) and promoted personas (`givn/personas/`) are agent-written conventions with no binary validation; a drifting or malformed state silently degrades resume and handoff. | Medium | Medium | The generated skill's contract phrases (phase ladder, persona rules, state paths, handoff safety) are executable-spec asserted at every `skills sync`; state-file structure itself stays convention and is reviewed at handoff, not machine-enforced. |
| R-47 | A semantic migration can misclassify a mixed domain area as one use case or fragment. | Medium | High | Require a confirmed mapping ledger, stable behavior hashes, explicit operations, and no semantic move without user confirmation. |
| R-48 | A use-case narrative can be replaced from a stale active change. | Medium | Critical | Record a base SHA-256 revision and reject stale replacement before archive mutation; back up narratives with feature files. |

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
| D-14 | `GIVN_SPEC_NLI_REPORT_LIMIT` is parsed but silently ignored. | Deprecated compatibility alias kept so scripts and fixtures setting it neither break nor change behaviour. | Remove the parse path after one release cycle once external users have migrated. |
| D-9 | Rule-nested WIP/E2E checks use existing direct-scenario traversal limits. | Current parser helper limitation. | Extend traversal and add regression coverage. |
| D-10 | Prior semantic report parsing is not strict and does not reject arbitrary unknown/reordered fields. | Current line-oriented implementation. | Implement a strict grammar parser before treating the wire format as enforced input validation. |
| D-11 | The remediation log reader follows the same line-oriented approach as the prior report extraction; agent-edited fields are not yet strict-grammar validated. | Introduced by the semantic-remediation-log change, joining the D-10 class. | Tighten to the strict wire-grammar parser once D-10 lands; the gate already rejects missing or stamp-mismatched evidence. |
| D-11 | Semantic remediation does not support prior `delta-to-delta` entries. | Scope precondition rejects them. | Add a deliberate projected-delta remediation design and tests. |
| D-12 | The current second-stage budget change is unfinished. | `givn status --change semantic-reranker-budget` reports open tasks and final verification. | Complete RED/GREEN/REFACTOR, full/E2E tests, real-model evidence, and blocking review before accepting ADR-063. |
| D-13 | ADR qualification is guidance-level and the register has no semantic validator. | The LLM/author and design review apply the policy; runtime integrity checks only verify mechanical file conditions. | Add a reviewed semantic register validator only if a future requirement justifies runtime enforcement; until then retain explicit review evidence. |
| D-15 | NLI batched-tensor inference is deferred. | `semantic-scoring-throughput` changes only the NLI session thread count; NLI keeps its per-sequence inference loop. | Implement the named follow-up change `semantic-nli-batched-inference` after measuring wall-time gains on a large corpus. |
| D-16 | The release-only branches of the model-channel gate (`spec_env`/`release_blocks`) execute only inside the release givn binary. | Coverage instruments the debug harness, so those branches show as uncovered in Cobertura while the `@release-proof` e2e group exercises them through a real release binary. | Keep the release-proof e2e group green; revisit a release-instrumented coverage pass only if the line-coverage delta ever becomes material. |
| D-17 | The README legacy-matcher list hard-codes the previous scaffold's placeholder sentences. | Detection is exact-text with no false positives, but a future scaffold-text change must keep the list in sync. | Update the matcher list whenever the scaffold template's wording changes, verified by the readme-enforcement scenarios. |

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
