# Design: prepare-0-9-10

## Overview

Implement the agreed 0.9.10 preparation in coherent behavior batches using the existing Rust CLI, embedded assets, Gherkin runner and Git/Fossil adapters. This is not a new policy framework or product rewrite. Proposal and delta features own observable requirements; this document chooses the mechanisms and test seams. No version bump/tagging or release publication is authorized by the change.

## Authority and decisions

The operator's resolved audit choices override contradictory current instruction text for the behavior being changed. Specifically: real RED evidence is recorded contemporaneously; skills replace commands; outline/shape cease blocking; modular architecture, Personas, initial arc42 baseline and long-scenario dispositions stay mandatory. No 0.x compatibility layer, narrower archive exemptions, privacy subsystem, blanket retry revalidation or extra release governance is added.

All human scope decisions are answered in proposal.md. No open D-number decisions remain. Technical decisions follow; changes to observable scope return to proposal/specs rather than being hidden in implementation.

### Decisions

- Use existing locked dependencies; add none. Versions are the checkout's Cargo.toml/Cargo.lock on 2026-10-29, not remembered latest versions. Rust edition 2024, clap declared 4.6, serde_yaml_ng 0.10, gherkin 0.16, cucumber 0.23 and tokio 1 are manifest declarations; exact resolutions remain Cargo.lock under --locked.
- Preserve the flat CLI adapters but extract only small concrete domain helpers needed by duplicated contracts: scenario traversal/content, capability ownership, actual verdict recognition and verification witness. Do not create one preparation module or generic validator engine.
- Supported publication dialect: English Feature, optional Feature Background, Scenario, Scenario Outline with Examples, step tables/docstrings and inherited execution tags. Rule blocks are explicitly unsupported for this release and refused before any mutation/verification claim. Existing project uses no Rule blocks. Non-English source keywords must either remain losslessly supported or receive the same explicit preflight refusal; do not silently render a different dialect. Feature-level non-managed execution tags are retained. A change attempting to alter an existing feature's Background is rejected unless its effective complete feature context can be replaced without silently changing untouched scenarios; first implementation rejects conflicting Background replacement and explains the required explicit feature rewrite. New-feature Backgrounds and unchanged existing context are preserved.
- Behavior snapshots and exact-duplicate detection share executable-content encoding, but different identities: preservation includes scenario identity; duplicate evidence compares executable content while reporting different titles/paths as endpoints. Encoding includes step kind/value, docstrings/media type, tables, Examples headers/rows and inherited setup/tags (exclude managed givn tags and authoring @wip). Source paths do not establish executable equality.
- No-behavior-change is an explicit proposal field `Behavior change: none` plus skipped specs artifact and absence of delta features. Reject disagreement; retain applicable identity/integrity checks. Store an applicability result, not a fabricated green runner receipt. Existing maintenance cases receive the same route. No false exemption inferred from an empty directory.
- Future migration support is tested with synthetic 1.0-to-later fixtures through existing resolver seams; shipped 0.x routes disappear. Current-format 0.9.10 refresh is possible after manual configuration repair; configuration format and supported migration baseline are distinct. Known retired addon keys produce a targeted manual-transition error, not automatic conversion.

## Architecture Impact

### Contracts and Components

Narrow module APIs are contracts; introduce traits only where current adapters require substitution. Keep implementation details private and composition in CLI entrypoints/main.

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Verification witness | verify-change | tasks/review adapters | proof + focused verification orchestration | Scope execution and reporting start owning unrelated responsibilities |
| Candidate recovery | archive-change | archive command | archive orchestration consuming proof/SCM | Additional publication mechanism, not hypothetical backend |
| Executable feature content | archive-change, verify-change, discover-specifications | merge, snapshot, duplicate analysis | gherkin parse/content modules | Dialect growth requires a separate adapter |
| Capability ownership | verify-change, archive-change | routing validation and merge planning | spec ownership resolver | More owner kinds; not another independently maintained lookup |
| Effective project configuration | initialize-project, upgrade-a-project, operate-product | addon/init/run/skills adapters | config overlay + command settings parser | New distinct configuration authority |
| Selected revision publication | shared support | commit adapter | scm Backend + Git/Fossil selected-path methods | Additional supported SCM is explicitly out of scope |
| Guidance delivery | obtain-effective-guidance, create-change | instructions/skills adapters | config cascade + skills renderer | New actual target semantics |
| Documentation update | keep-documentation-current | README adapter | readme/run-block parsers | New managed block type |
| Structure report | assess-code-structure | outline command, authoring agent | existing Rust/Python analyzers/report | New language adapter post-1.0 |
| Development recording | inspect-development-history | harness reporters | existing best-effort intake/reporters | Recovery improvements post-1.0 |
| Status inspection | inspect-change-state | status/graph adapters | state/proof status projection | New actual proof state |
| Bounded product start | operate-product | run adapter | existing readiness/process orchestration | New protocol post-1.0 |
| Managed refresh | upgrade-a-project | upgrade adapter | existing allowlisted sync/publication | New supported migration after 1.0 |

External ports remain explicit: SCM commands via scm/git/fossil; filesystem through focused parsing/write helpers; runner processes via subprocess; readiness via loopback TCP; harness reporting via existing per-agent integrations. Domain comparisons do not execute commands or render terminal text. Adapters report errors without introducing recovery policy into the domain.

### Production modules touched

- src/cli/{check,archive,status,graph,instructions,commit,init,addons,skills,upgrade,migration,readme,run,git,absolute_mode,project_hint,agents_block}.rs; remove/rename obsolete command/tone surfaces as appropriate.
- src/config/{manifest,overlay,extensions,cascade,render}.rs, src/skills/{targets,render,rules}.rs; no parallel addon default resolution.
- src/{proof,integrity_hook,review_context,scm,git,fossil,cobertura,history}.rs and src/gherkin, src/spec, src/overlap, src/outline report/metrics/classify.
- Delete outline gate implementation only after callers/tests are retired; keep analyzer/model/report. Do not refactor analyzer resolution/parity under this change.
- assets instructions/templates/addons/skills/project hint; delete assets/commands, obsolete 0.x bundles and unused coverage template; keep integration reporter assets, replacing only timing/reliability behavior.

## Verification and recovery mechanics

### Stable execution witness

Create a concrete `VerificationInputs` value containing current revision, selected-feature digest, command digest, relevant content digest and declared scope. Capture immediately before runner launch, remove prior output, execute once, validate fresh result and compare after. Write proof only after equality. Any launch/result/stability failure leaves no new green receipt. Existing broader evidence exclusions remain unchanged and explicitly documented. Runner transiently changing then restoring an input is not authenticated; trusted environment and final-state stability are the boundary.

Keep verification scope explicit in GIVN_FEATURES/GIVN_RESULT_FILE/GIVN_RESULT_SCOPE. Correct run-tests.sh to preserve an explicitly supplied scope instead of overwriting change with regular; direct helper command/fixtures get matching four-count output. Missing/malformed/zero/failed/skipped/inconsistent fresh results fail; no stale results survive.

### Archive successful-scope reuse

Extend pending receipt with optional per-scope evidence: input/content digest, command digest, result-file digest/status and tested revision; persist each successful scope immediately so later failure does not erase it. On retry, compare relevant witnesses cheaply. Unchanged scope evidence is reused; changed inputs conservatively invalidate all scopes sharing those inputs. No speculative fine-grained dependency graph: if applicability is unknown, regular/E2E are both affected. Evidence-only revisions can rebind to the current candidate after equal content checks. Failed finalization republishes valid proof without suites; invalid proof returns to affected verification. Coverage reuse similarly binds measure/merge commands and relevant inputs and verifies output existence/digests. Keep existing archive-path exemptions and no retry re-review.

Schema: bump receipt schema only when necessary for new optional fields; legacy receipts remain readable but cannot provide missing reuse evidence, so reuse is not assumed. Current green change receipts remain accepted only when existing freshness contract holds. No legacy0.x migration bundles are needed for runtime receipt parsing; historical proven archives are never rewritten.

### Diagnostics and actual verdicts

Collect independent cheap findings into a bounded diagnostic result; root schema failure suppresses only dependent checks. Do not execute suites after prerequisite failure. Share actual-marker parsing stripping code fences/HTML comments and matching complete documented verdict entries, allowing current Markdown emphasis wrappers but not arbitrary substring prose. Distinguish missing/conflicting/FAIL. Keep generic review provenance requirements; remove separate outline identity/version obligations.

Bare check prints help with zero execution; status/graph computes cheap receipt state when possible and labels unavailable inspection not checked. Historical receipt-less archive remains legacy/unproven-for-current-guarantee, never counterfeit proven or automatically retried.

## Configuration, skills and external operations

### Configuration authority and editing

Parse commands.yaml with a dedicated allowed-field patch (verify/run/coverage and existing documented command/runtime settings), not Manifest overlay. Reject policy keys there before mutation. Preserve existing library declarations unless intentionally changed; config.yaml owns artifact/addon/rules/targets/extensions. Apply full policy patches, resolve omitted defaults once, then expand effective addon artifacts once. Re-init preserves stored explicit selections/opt-outs.

For existing YAML edit commands, validate block shape before writing; if a flow-style/nested shape cannot be safely updated with current tooling, refuse unchanged with actionable manual editing instructions. No new YAML-preserving dependency. Remove the misleading scalar-generates/inline-instruction/protected-removal examples and test supported examples against actual parsers.

### Skills-only and canonical guidance

Keep selected target directories for agents/Claude/OpenCode/OMP/Maki. Remove owns_commands and command registry/render/copy paths. Add givn-challenge skill and canonical challenge instruction; replace two grill flags with challenge and absolute_mode with communication_policy. No aliases; recognize retired keys only to name manual remediation. Preserve user overrides. Recognize old generated command bytes/managed signatures conservatively before deletion; an unknown/custom file remains unchanged with a manual notice.

Bootstrap includes dialog load-once obligation and resident batch rules from canonical assets. Skills only contain metadata/input and one served-instruction direction, not policy/delegation/review loops. New dialog-examples served ID provides all preserved regression examples; dialog indexes type-to-section and requires relevant first-type reading. Extend known-ID resolution and text/JSON reference validation accordingly. Not an automatic autoload claim.

### Explicit-path SCM publication

Check nonempty paths before any staging. Reject escaping/nonexistent/unsafe selections. Use existing selected-path SCM seam, with Git isolated index (`GIT_INDEX_FILE` temporary index seeded from HEAD, stage selected working contents, commit with isolated index) and reconcile only selected entries in original index to the resulting commit while preserving unrelated staged content. Preserve unrelated staged deletions/additions/conflicts; if index reconciliation cannot be proven safe, refuse before commit rather than consume work. Avoid ordinary unrestricted commit_staged. Fossil uses explicit selected paths, preserving unrelated pending additions. Tests must assert resulting revision content AND index/worktree preservation. Directory selections are explicit recursive choices; guidance uses narrow paths. No implicit all/stage-all public path.

Hook install compares canonical/equivalent shared/activation paths before renaming/writing launchers and detects managed backup recursion. Missing optional helper skips attribution, preserving user-hook failure semantics. No bypass of user hooks or force/no-verify flags.

### Bounded readiness and recording

Use one Instant deadline across readiness retries. Each connect/write/read receives remaining-budget timeout (bounded by existing probe interval where applicable), checks zero remaining budget before IO and returns timeout to existing cleanup/signal path. A server accepting but never replying cannot hang indefinitely.

Reporters use a 2-second best-effort helper deadline (settings/intake), kill/abandon stalled helper and continue without repeated synchronous retries; no primary command outcome change. Keep bounded queues where already present; do not add recovery/privacy machinery. Recordings stay ignored and untracked; warn of sensitive content. Validate no release/runtime-data leak.

### README and upgrade

Parse managed marker pairs before edit, require unique ordered begin/end, preserve surrounding bytes, and reject malformed ambiguity. Missing blocks follow existing documented insertion behavior, distinct from damaged partial pairs.

Upgrade captures existing checkpoint and reports actual completed writes/revisions/pending work. Preserve user files; no reset/clean or complete rollback promise. Best-effort restoration may remain if errors reported honestly; no broader transaction. Remove pre1.0 route bodies/catalog entries and update release checks currently assuming every older minor has a route so 1.0 baseline is explicit. Retain future route resolver with synthetic fixture injection, not fabricated shipped routes. Target0.9.10 format can be manually prepared without marker claiming released1.0.

## Advisory analysis and architecture guidance

Remove shape disposition enforcement but retain explicit discovery; long-scenario split/keep validation remains with keep reason/completed split. Literal supported executable content determines exact_duplicate; structural matching remains current simple heuristic with honest labels/sorted endpoints. No threshold/semantic model work.

Remove outline gate calls from check/archive, module exports, marker/waiver templates, tests and glossary. Report text/JSON uses optional metrics (`null` where unavailable); omit unmeasured LCC, do not substitute TCC. Retain measured Rust/Python facts with limitations; no new language/flow correctness promises. Unsupported-only scope reports Rust/Python support explicitly. Four-question outline-assisted REFACTOR cadence lives canonically in steps; review refers to it, reuses valid reports, and does not trigger loops.

Strengthen architecture-principles for modularity/ISP/adapters/composition without invented providers. Arc42 baseline completion and subsequent impact maintenance stay distinct. ADR assessment once; ADR records rationale not another qualification schema. Existing1.0 prep Personas remain review lenses; update stale per-scenario Persona statements as historical notes/current batches without inventing identities.

## Documentation ownership and deferred work

README orientation; workflow map; reference owns CLI/config/runner/recovery; served instructions own agent obligations; templates fields; arc42/ADRs rationale/current architecture. Consolidate workflow-details unique content into workflow/reference and update links. Mark old accepted decisions amended/superseded rather than erase their history. Mark completed ideation historical. Fix stale default/review/coverage/attribution/agent inventories and sample/frontmatter defects. Remove current harness naming, preserve historical evidence. Manual pre1.0 transition replaces automatic route guides.

Use docs/arc42/11-risks-and-technical-debt.md as the one existing backlog, extending entries with concrete location/consequence/reproducer and deferred reason. Include shape matching improvements, outline coverage/language/backend/metrics, interrupted/concurrent recording recovery, broader YAML/Gherkin/probes, unsupported custom-artifact instructions and unrelated internal registry/corpus duplication. Minor closure observations enter that backlog without reopening acceptance. No new release-governance file.

## Visual Design Contract

- Interface: terminal
- Screens: init/sync/config diagnostics, explicit commit errors, runner/setup/check/status output, outline report, archive/recovery/upgrade outcomes, effective instruction text.
- Viewports: not applicable; line-oriented CLI output, no new TUI layout or browser screen.
- Design system: existing line-oriented CLI conventions; no new colors/icons/design tokens. Success/error/pending/N/A states use explicit textual labels, no color-only meaning or internal object IDs as unexplained UI.
- Accessibility: readable headings, actionable errors with paths/next action, deterministic ordering; captured subprocess stdout/stderr is checked. Preserve locale assumptions, no animations. Motion/reduced-motion and browser focus are not applicable.
- Evidence: terminal transcript per E2E under evidence/visual/<scenario-slug>/transcript.txt, captured after primary return/output assertions. Capture actual bytes/exit, not fabricated snapshots; no screenshots or browser driver added. Existing timeline browser interface is unchanged.
- Confirmed Personas: developer--primary-user (clear repair/no unexpected work), agent--technical-role (stable counts/status/explicit actions), givn-maintainer--owner (truthful scope/retained history). No new Persona confirmation.

## Use-Case Traceability

Owning delta narratives and interactions are under specs/<owner>/usecase.md or fragments/fragment.md with full-base SHA stamps. No new independent user goal is introduced. Every selected capability is EXTEND; outline-review-gate is retired. Preserve identity/schema/relationships, adapting only agreed contract changes. Exact old titles identify modifications, not retained obsolete semantics.

| Guarantee/rule | Decision | Evidence |
|---|---|---|
| Verification proves fresh stable inputs | Pre/post witness, remove old output, N/A explicit | givn-workflow-enforcement delta and existing schema-review smoke |
| Archive preserves specs and repairs forward | Supported dialect, content encoding, persisted scope evidence | archive-proof/preserve-scenario-docstrings/grouped-merge deltas |
| Scope/accounting is unambiguous | owner resolver + typed graph + capability interaction identity | grouped-merge/check-specification-health deltas |
| Project choices/user work preserved | safe edit/refusal, selected-path SCM, no recursive hook | init/addon/commit/readme/upgrade deltas |
| Clear effective guidance with no duplication | thin skills, bootstrap/examples, challenge, honest templates | instruction-integrity/prose/serve/refresh/ADR deltas |
| Modularity/Personas/arc42 remain | Canonical architecture/checklists, initial baseline, ADR once | review-architecture-documentation delta |
| Structural hints do not gate | Remove outline gate/shape obligations, honest report metrics | retirement marker, outline-report/review-disposition deltas |
| Product/recorders cannot stall main work | bounded deadline/helper wait | run/agent-activity deltas |
| Users distinguish pending/proven | cheap status and nonexecuting bare check | status/workflow deltas |

## Interaction Coverage Matrix

Each modified E2E title remains in its owning narrative; unchanged existing smokes remain permanent. Reclassifications remove duplicate action rows, not required behavior. The delta matrix below is generated from actual @e2e scenarios; all are real subprocess CLI/harness interactions, with transcripts and primary exit/stdout/stderr/file outcomes.

| Capability | E2E scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| archive-proof | Consumer repairs a blocked archive and the rerun proves it | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| grouped-merge-for-new-capabilities | Consumer archives a new capability and the merged spec lands in its declared use case | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| outline-report | A developer runs a outline report and sees the classified units | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| prose-consolidation | Generated guidance points to the canonical E2E policy | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| spec-duplicates | Exact duplicates in one feature are reported together | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| adr-qualification-gate | An author requests the strict ADR guidance through the Givn CLI | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| attribute-commits | Consumer initializes a project and receives shared attribution plumbing | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| attribute-commits | An attributed commit exposes the current LLM and harness | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| enable-commit-attribution | Consumer sees the commit attribution installation report | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| enable-commit-attribution | Consumer upgrades an existing project with attribution plumbing missing | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| enable-commit-attribution | Consumer initializes a Fossil project and receives the givn commit path without a hook | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| instruction-integrity | Author starts a session with the universal dialog obligation | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| match-guidance-to-enabled-addons | A consumer synchronizes skills and the generated guidance omits disabled-addon sections | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| match-guidance-to-enabled-addons | A consumer enables the addons a generated section needs and the guidance gains it | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| refresh-generated-guidance | Synchronized guidance explains how to resolve the deterministic gates | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| review-architecture-documentation | Arc42-enabled review instructions require conformance evidence | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| review-architecture-documentation | The review template includes the Arc42 conformance block | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| enable-optional-addons | Consumer enables the outline addon and it is listed as enabled | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| initialize-a-project | Consumer runs init non-interactively and gets opencode skills | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| initialize-a-project | Consumer re-syncs skills after init and the selection is honoured | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| report-change-status | Detail mode — active change with all required done (text) | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| agent-activity-reporting | Consumer installs the agent reporter and its reported activity lands in the record | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| readme-enforcement | Archiving refreshes the development block | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| start-the-declared-product | The started server reports readiness and forwards a clean exit | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| apply-a-managed-upgrade | A clean project without migration requirements receives one managed upgrade commit | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| apply-a-managed-upgrade | A completed maintenance migration can be archived without a product specification | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| check-specification-health | All clean .feature files report a clean summary on stdout | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| givn-workflow-enforcement | Consumer runs check tasks and receives a change verification receipt | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |
| givn-workflow-enforcement | Consumer invokes bare check and receives explicit non-executing guidance | CLI / harness subprocess | Existing isolated TestEnv, selected backend and actual binary/reporting integration; assert returned output and capture transcript |

## Step Definitions

Reuse existing permanent bindings wherever the phrase expresses the same requirement; add discriminating assertions rather than giant output snapshots. New phrases are implemented only in durable tests/steps files, never in delta copies. Each capability has its own regular and, when applicable, separate E2E file; shared fixture support contains no step bindings. tests/steps/mod.rs registers modules and narrow per-capability world state. Do not create one preparation catch-all step file. The second column below matches the actual existing integrity parser; this change corrects contrary template prose.

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| archive-proof | tests/steps/archive_proof_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| extract-coverage-gate | tests/steps/extract_coverage_gate_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| grouped-merge-for-new-capabilities | tests/steps/grouped_merge_for_new_capabilities_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| preserve-scenario-docstrings | tests/steps/preserve_scenario_docstrings_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| outline-report | tests/steps/outline_report_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| prose-consolidation | tests/steps/prose_consolidation_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| spec-duplicates | tests/steps/spec_duplicates_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| adr-qualification-gate | tests/steps/adr_qualification_gate_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| attribute-commits | tests/steps/attribute_commits_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| enable-commit-attribution | tests/steps/enable_commit_attribution_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| instruction-integrity | tests/steps/instruction_integrity_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| match-guidance-to-enabled-addons | tests/steps/match_guidance_to_enabled_addons_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| refresh-generated-guidance | tests/steps/refresh_generated_guidance_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| review-architecture-documentation | tests/steps/review_architecture_documentation_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| enable-optional-addons | tests/steps/enable_optional_addons_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| initialize-a-project | tests/steps/initialize_a_project_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| report-change-status | tests/steps/report_change_status_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| agent-activity-reporting | tests/steps/agent_activity_reporting_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| readme-enforcement | tests/steps/readme_enforcement_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| serve-extended-rules | tests/steps/serve_extended_rules_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| start-the-declared-product | tests/steps/start_the_declared_product_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| apply-a-managed-upgrade | tests/steps/apply_a_managed_upgrade_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| check-specification-health | tests/steps/check_specification_health_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| givn-workflow-enforcement | tests/steps/givn_workflow_enforcement_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |
| review-disposition-gates | tests/steps/review_disposition_gates_steps.rs | same durable file; reuse current alternative filename where already registered, never duplicate bindings |

Existing name exceptions: givn-workflow-enforcement -> workflow_enforcement_steps.rs / workflow_enforcement_e2e_steps.rs; initialize-a-project -> initialize_a_project_e2e_steps.rs plus existing init fixtures; attribute-commits -> llm_attribution_steps.rs / llm_attribution_e2e_steps.rs; enable-commit-attribution -> llm_attribution_installation_steps.rs / llm_attribution_installation_e2e_steps.rs; start-the-declared-product -> run_execution_steps.rs / run_execution_e2e_steps.rs; report-change-status and check-specification-health reuse registered common steps where identical and get per-capability files only for distinct new phrases. All new files declared here are planned, not claimed to exist. Exact E2E locations follow <capability>_e2e_steps.rs only when no existing exception; register once.

## Runner, strictness, local execution and coverage

- Current verify.command: `./run-tests.sh --non-e2e`; current E2E command: `./run-tests.sh --e2e` (givn/commands.yaml, inspected). They execute actual features via tests/features_runner.rs.
- Target one scenario exactly: `GIVN_FEATURES=givn/changes/prepare-0-9-10/specs cargo test --locked --test features_runner -- --name '^A successful runner without new output cannot reuse a stale result$'`. Substitute escaped exact title; do not combine --name and --tags. Harness nextest exact unit is an alternative, not additional run.
- Batch: `GIVN_FEATURES=givn/changes/prepare-0-9-10/specs/<owner>/<capability>.feature cargo test --locked --test features_runner -- --tags 'not @givn.removed'`. This intentionally includes @wip during real test-first work; undefined bindings must first be implemented to expose meaningful RED.
- Existing strict mechanism: `.fail_on_skipped()` and final failed/skipped exit check in tests/features_runner.rs:54-65. Reprove only when changing runner configuration; do not claim prior infrastructure failure as behavioral RED. New unimplemented Rust bodies panic via unimplemented!, never empty bodies.
- Full project validation uses cargo check/fmt/clippy and existing run-tests scopes; archive regular/E2E/coverage authority is retained without extra review runs.
- Local run: `cargo run -- instructions`; no server stack, database, containers, external network or fabricated digital twins. Dependencies are Git/Fossil, shell, script/PTY and temporary loopback server fixtures; existing git-cliff release tests remain unless changed scope requires updating migration assumptions.
- Test fixture boundaries: temporary filesystem/repos; wrong owner/namespace, malformed YAML/JSON/markers, timeout/silent socket, partial result/write/publication, repeated init/hooks, stale outputs/replayed evidence, unrelated staged edits. No live external service.
- Coverage population: src production code/CLI, excluding tests/generated fixtures/runtime data. Keep existing measure-coverage.sh/merge-coverages.sh instrumentation and collision-safe LLVM_PROFILE_FILE values; inspect before edits. Cucumber test runner is instrumented and launches the instrumented givn from TestEnv. Rust ready_fixture starts an instrumented readiness child. JS/Lua reporters have subprocess behavior tests but are not claimed as measured Rust coverage.

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| features_runner | measurement script | cargo llvm-cov-produced features_runner | existing collision-safe LLVM_PROFILE_FILE under target | existing llvm-profdata/llvm-cov script | public CLI/check fixture paths |
| givn CLI | TestEnv or shell reporter | instrumented CARGO_BIN_EXE_givn selected by coverage harness | inherited per-process profile pattern | same exported scope merge | check/init/archive/commit production entrypoints |
| ready_fixture | run execution fixture | instrumented ready_fixture built before suite | inherited unique per-process profile pattern | same scope merge | readiness/start lifecycle |

Before first measurement, verify actual script values match this process declaration; correct instrumentation gaps without excluding inconvenient production code. No new percentage threshold.

## Implementation batches and phase commits

Planning phases are separately committed (proposal; specs; design/arc42; independent design-review; tasks). Implementation batches follow genuine RED/GREEN/REFACTOR with compact contemporaneous evidence in tasks.md, actual coherent explicit-path commit and immediate scenario checkoff after batch evidence. Do not commit separate RED phase merely for appearance, rewrite history, or delay all task updates to end.

1. Supported executable Gherkin, complete preservation and ownership/interaction identity.
2. Fresh change proof, explicit N/A, lean persisted archive scope recovery and coverage freshness.
3. Explicit-path commits, safe hooks/readiness/config/init/README outcomes.
4. Skills-only, addon names/defaults, bootstrap/dialog examples/challenge and supported setup.
5. Canonical test-first/architecture/arc42/ADR/review/closure instructions and templates.
6. Advisory outline/shape, long disposition validation, metrics labels, deterministic duplicate/status/check output.
7. Bounded recording helpers, removed0.x assets/manual transition and documentation reconciliation/backlog.

Within each, use capability-sized parallel independent work only if it respects existing user constraints and does not create fresh review contexts. The one independent reviewer is reused for final review; closure checks only its recorded findings and material fix regressions. Concrete encountered cleanup gets a separate GREEN refactoring commit, never an invitation to add features. Do not let schema-only/WIP artifacts count as implementation evidence.

## Deferred work and known limits

Preserve matching/outline reliability/recording recovery findings in existing chapter11 with locations and reproductions. No archive evidence allowlist tightening, extra privacy validation, broad compatibility shim, full Rule/language support, semantic model, generic framework or release-policy artifact. Trusted final-state input equality is not adversarial execution authentication. Analyzer structural order is not dynamic tracing. Maki skill auto-selection is not guaranteed; explicit skill requests are the reliable manual entry.

## Open Technical Questions

None required to begin tasks after design review. If independent review finds a concrete contract gap, amend proposal/spec/design explicitly. Unrelated minor polish remains tracked, not another audit cycle.
