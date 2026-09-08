# 11. Risks and Technical Debt

This chapter distinguishes implemented safeguards from open limitations. A
historical measurement is not a current quality claim.

## 11.1 Risks

| ID | Risk | Likelihood | Impact | Mitigation / current boundary |
|---|---|---:|---:|---|
| R-1 | Replaced scenarios cannot preserve every original docstring delimiter detail. | Medium | Medium | The text-level merge preserves content and tables but renders replacement docstrings from the parsed AST. |
| R-2 | Overlay semantics are misunderstood. | Medium | High | Artifact patches deep-merge by id; top-level `verify`, `coverage`, and `apply` overlays replace blocks. Keep the effective-manifest order documented. |
| R-3 | Static native linking is not proven for every target. | Medium | High | Treat musl, Windows, and release artifacts as unverified until built. |
| R-4 | Users assume Givn owns all of `AGENTS.md`. | Medium | High | Givn owns only its matching fenced blocks; all surrounding content is user-owned and must be preserved. |
| R-5 | Archive failure could leave a changed permanent tree or README. | Low | Critical | Back up permanent specs and README, validate merge preconditions, and roll back post-merge verify, E2E, and coverage failures. |
| R-6 | Arc42 documentation becomes stale or is treated as a generated copy. | Medium | High | Arc42 completion review compares durable chapters, change assessment, design, tasks, and implementation; integrity checks resolvable affected files against templates, while missing files remain a current helper gap. |
| R-7 | Generated config comments describe old post-hook defaults or nonexistent Arc42 settings. | High | Medium | The runtime contract is the current embedded manifest and source: standard post lists are empty, mandatory checks are binary-owned, and Arc42 output is fixed. |
| R-8 | E2E detection misses scenarios nested under `Rule:` because the current helper checks direct feature scenarios only. | Medium | High | Keep the limitation explicit in review and add coverage before treating nested E2E detection as complete. |
| R-23 | Ordinary design, process, or implementation choices inflate the active ADR contract. | High | High | Apply the strict five-dimension AND gate, reject shared-guidance-only claims, and route each non-ADR decision to exactly one canonical artifact. |
| R-24 | ADR consolidation loses unique rationale, relationships, or ownership. | Medium | High | Use the complete ID-by-ID disposition manifest, preserve only genuine superseded architecture history, and review every target link and canonical source before removing a record. |
| R-25 | The qualification procedure drifts between the canonical addon instruction and derived design-review guidance. | Medium | High | Keep the complete procedure and MADR template in one source, make derived assets thin, and verify effective/generated guidance parity. |
| R-26 | Authors treat the LLM verdict as machine-enforced architecture validation. | Medium | High | State the non-runtime boundary explicitly and require human/design-review evidence; the current integrity hooks do not infer architectural significance. |
| R-27 | Automatic upgrade commits a generated file that a project has treated as user-owned. | Medium | High | Require a clean worktree, stage an explicit known-path allowlist, preserve ejected facets, and expose both commit diffs for review. |
| R-28 | Commit A succeeds but Commit B fails, leaving the generated migration plan uncommitted. | Low | High | Never reset history; keep the plan visible, report the successful hash and remaining files, and make rerun behavior preserve the existing plan. |
| R-29 | A project interprets `givn_config_version: 0.3.0` as proof that semantic migration is complete. | High | High | Create the visible maintenance change, record the distinction in its design, and require normal LLM review/archive completion. |
| R-30 | A future migration catalog contains a broken edge or incomplete bundle and leaves a version gap unclear. | Medium | High | Validate schema, unique IDs, advancing versions, referenced assets, ordered paths, and fail explicitly once a declared route has started. |
| R-31 | Multiple required bundle phases are treated as independent concurrent changes. | Medium | High | Materialize one aggregate maintenance change with `migration.yaml`, explicit sequence numbers, and prompts that require strict phase order. |
| R-33 | Force-staging an ignored managed path could accidentally include unrelated user content. | Medium | Critical | Resolve individual managed paths before staging, use `git add -f --` only for that allowlist, never stage directories, and retain the clean-worktree preflight. |
| R-41 | The ephemeral port allocated for a `port_env` product is claimed by another process between givn's bind-0 probe and the child's bind (TOCTOU). | Low | Medium | Accept the narrow race; givn reports the child's early exit or the readiness timeout instead of silently retrying allocation, and the ready line names the actual port. |
| R-42 | `givn run` signal forwarding is unix-only; a Windows host running a server product gets a preflight refusal, which may surprise cross-platform projects. | Medium | Medium | The refusal is explicit at preflight with exit 1 and a named platform constraint; cli/library products work everywhere; the constraint is documented in the template and specs. |
| R-43 | A project declares `product_type: library` to escape the server requirements (declaration spoofing). | Medium | Medium | Deliberately uncountered inside givn: a heuristic would guess and block on guesses. The declaration is versioned and README-rendered, so a wrong entry is visible in review, and the caller's preview gate treats a falsified entry as an obviously wrong record, not a green start. |
| R-44 | Signal forwarding reaches only givn's direct `sh -c` child; grandchildren of compound start commands (`a && b`, pipelines) survive the interrupt. | Medium | Medium | Documented residual (unix-boring tradeoff): start commands should use `exec` so the shell process is replaced by the product; givn does not reimplement process-group supervision, and the trap-marker teardown scenarios cover the direct child. |
| R-45 | The `givn run` machine contract (ready line format, exit codes, signal behaviour) becomes a durable public compatibility obligation once it ships; breaking it breaks kiekste and caller scripts. | Medium | High | The contract is pinned by executable specs — the `run-execution` @e2e ready-line/exit-code scenarios and the spec'd exit-code table; changing any of these surfaces requires a spec change, which is visible in review. |
| R-46 | Ideation topic state (`givn/ideation/`) and promoted personas (`givn/personas/`) are agent-written conventions with no binary validation; a drifting or malformed state silently degrades resume and handoff. | Medium | Medium | The generated skill's contract phrases (phase ladder, persona rules, state paths, handoff safety) are executable-spec asserted at every `skills sync`; state-file structure itself stays convention and is reviewed at handoff, not machine-enforced. |
| R-47 | A semantic migration can misclassify a mixed domain area as one use case or fragment. | Medium | High | Require a confirmed mapping ledger, stable behavior hashes, explicit operations, and no semantic move without user confirmation. |
| R-48 | A use-case narrative can be replaced from a stale active change. | Medium | Critical | Record a base SHA-256 revision and reject stale replacement before archive mutation; back up narratives with feature files. |
| R-49 | A release can claim semantic migration completion without retained Givn self-migration evidence. | Medium | High | Require executable migration-gate scenarios, before/after receipts, a complete ledger, and a concrete release target before publishing the bundle. |

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
| D-13 | ADR qualification is guidance-level and the register has no semantic validator. | The LLM/author and design review apply the policy; runtime integrity checks only verify mechanical file conditions. | Add a reviewed semantic register validator only if a future requirement justifies runtime enforcement; until then retain explicit review evidence. |
| D-17 | The README legacy-matcher list hard-codes the previous scaffold's placeholder sentences. | Detection is exact-text with no false positives, but a future scaffold-text change must keep the list in sync. | Update the matcher list whenever the scaffold template's wording changes, verified by the readme-enforcement scenarios. |

## 11.4 Resolved Boundaries

- Active change ids are validated as lowercase ASCII kebab-case before the
  shared change path is constructed.
- Givn does not overwrite user content outside its own `AGENTS.md` fenced blocks.
- The semantic retrieval engine (local E5 retrieval, BGE/NLI evidence,
  model-cache acquisition, and the semantic review gates) was removed by the
  `drop-embedding-features` change; ADR-069 records the single feature-free
  binary without model dependencies. The engine risks and debts formerly
  tracked here (model download, tokenizer caps, semantic review debt, and the
  feature-matrix churn) are resolved by that removal.
