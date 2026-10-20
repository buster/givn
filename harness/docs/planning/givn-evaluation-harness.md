# Givn evaluation harness: implementation and handoff plan

Date: 2026-10-19. Status: planning complete; implementation not started.

## 1. Goal and scope

Build a trustworthy, practical tool for answering:

- Did a givn version or instruction change improve delivered software, reliability, time, or cost?
- Which workflow components help, and which introduce avoidable friction?
- Where did an existing real-world run spend effort or encounter problems?

The end state supports **small, frequent checks and complex, multi-change evaluations**, using the same execution and reporting machinery. It also accepts existing `events.jsonl` files without rerunning an agent.

Two modes must remain distinct:

| Mode | Input and output | What it establishes |
|---|---|---|
| Controlled evaluation | Pinned task, givn version, environment and execution policy → delivered repositories, independent grades and measurements | Evidence about differences between controlled conditions. |
| Retrospective analysis | Existing records, optionally repository history and transcripts → evidence-linked diagnostics and limitations | What was observed; hypotheses for experiments, not causal proof or an invented product-quality score. |

This document supersedes conflicting proposals in `harness/improv-plan/claude-plan.md`. The old draft remains background, not implementation authority. This planning session creates no givn change. The user has authorized the future implementing agent to bootstrap the separate harness repository, import the needed existing harness files, and establish local Git subtree synchronization as described below. No files are moved in this planning session. This authority does not extend to unrelated givn product changes, remote hosting/publishing, or broader paid campaigns.

**Paid-evaluation authorization: USD 5 total for initial loop validation**, including failed attempts, subagents, retries and auxiliary requests. No calls were made for benchmark execution during planning. Later campaigns require a separately approved budget. Paid runs must not begin before the model restriction, isolation and spending guard are proven.

## 2. Confirmed decisions and reasons

| Decision | Reason and consequence |
|---|---|
| OpenCode, direct `deepseek` provider, DeepSeek Flash, maximum thinking only; existing selector `deepseek/deepseek-flash#max`. | Hold execution conditions fixed. No Pro, other provider/model, or fallback, including subagents and maintenance requests. Historical files from other setups may be analyzed offline. |
| Benchmark application uses a pinned Python stack; harness stays Python and givn stays Rust. | Minimize unrelated build work while proving the loop. Pin runtime, dependencies and startup contract, not internal application architecture. |
| Start with current-versus-candidate givn comparisons, not no-givn/core/full campaigns. | Optimize for improving givn, not first resolving the framework debate. Ablations come after the experiment is trustworthy. |
| Independent behavioral tests are the primary quality evidence. | Artifact existence, PASS markers and self-reported green tests do not prove that the requested product works. |
| Automatically accept recommended answers to open, in-scope choices. | Exercise dialog without human waiting or a separate product-owner model. The policy does not simulate discovery of another person's unstated business expectations. |
| Supplied requirements remain binding; defer optional scope expansion. | An accepted recommendation cannot legitimately override the five-loan limit or add an unrelated reservations product. Detectable violations are reported; automatic selection is not itself semantic validation. |
| Hide test cases, not unknowable requirements. | Test unexpected sequences against known obligations. Do not penalize an approved default because the grader secretly prefers another default. |
| Record accepted decisions; unsupported checks remain explicitly ungraded. | Arbitrary agent-written decisions cannot automatically become an independent oracle. Do not pretend an agent-generated test verifies the correctness of its own assumption. |
| Neutral driver; no forced lifecycle-step/scenario restart. | Givn must supply its own process. The current harness prompt duplicates workflow rules and confounds their evaluation. |
| Initial delivery followed by a fresh-session change request. | Measure whether the first delivery supports subsequent work and preserves behavior, not just one-shot output. |
| Independent per-trial state and external evidence capture; sandbox failure stops execution. | Prevent copying across trials and losing or modifying authoritative measurements. No unsandboxed fallback for graded runs. |
| Local files, Markdown and JSON first; no LLM judge initially. | Make results inspectable, reproducible and cheap without building a research platform before the first useful experiment. |
| Keep the small task and add harder tasks when needed. | Saturation on an easy task is not proof that givn cannot improve. Changing the measuring task silently destroys comparability. |
| Develop the harness with givn in its own repository; reuse the existing implementation rather than starting from scratch. | Keep Python harness development separate from Rust product development and avoid givn's nested-project restriction. The standalone repository is authoritative; `givn/harness/` is its synchronized subtree. |

## 3. Starting point and implementation location

### Starting state and paths

The user will start the implementing agent in **`/home/buster/projects/givn-harness`**. The user created that directory but moved no files. It was verified empty during this planning session; do not assume it already contains a Git repository, givn project or copied implementation. Recheck before writing in case it has since changed.

| Location | Role |
|---|---|
| `/home/buster/projects/givn-harness` | Destination, future standalone Git/givn root and authoritative development repository. |
| `/home/buster/projects/givn/harness` | Existing harness source, tests, tasks, documentation and givn state to inspect/import; afterward the synchronized subtree copy. |
| `/home/buster/projects/givn` | Parent Rust product repository. Limit changes here to the agreed harness extraction/subtree integration; do not turn this work into a root givn product change. |
| `/home/buster/.opencode/plan/givn-evaluation-harness.md` | This handoff plan; remains available outside both repositories. |

Reuse the existing Python implementation, CLI, tests and reporting where suitable. **A new repository does not mean a from-scratch rewrite.** Developing the harness with givn inside its own root avoids the current restriction that the givn project root must equal the Git worktree top level (`/home/buster/projects/givn/src/scm.rs`). Do not add nested-root support or Python test dependencies to the Rust product to enable this work.

### Bootstrap responsibility of the implementing agent

1. **Inventory and preserve.** From the new directory, inspect the source repository and its applicable instructions, tracked history, staged/unstaged differences and relevant untracked harness files. Preserve these before extraction. A history-only split misses current local work. Do not reset, clean, discard or commit unrelated changes to obtain a convenient starting state.
2. **Import the required existing project.** Establish the standalone repository using a history-preserving extraction of the harness where feasible, then explicitly carry over relevant current changes/untracked source. Include source, tests, public task assets, scripts, dependency manifests/lockfile, documentation and relevant givn artifacts. Inspect generated files rather than blindly copying everything. Do not import credentials, `.venv`, package caches, runtime state or private reports/logs as source. Preserve existing data in place unless its relocation is actually needed and safe.
3. **Establish the development workflow.** Inspect and deliberately reconcile/migrate the harness's existing givn state and agent integration in the new repository. Do not delete old specifications/history or blindly run the earlier draft's version-specific migrations. Repository bootstrap is setup work; subsequent implementation increments use givn in the standalone root, not one enormous change for the whole plan.
4. **Create the local subtree relationship.** Establish local Git remote/ref relationships and the necessary subtree history so the parent repository's existing `harness/` prefix can synchronize from the standalone repository. This means Git subtree synchronization, not a filesystem symlink or submodule. The prefix already exists: do not attempt an add-over-existing-prefix operation or delete it blindly. Validate the conversion in a disposable copy first if needed, preserve parent history/local work, and document the exact split/join base and repeatable synchronization procedure. No remote hosting or network push is required or authorized.
5. **Verify the handoff.** Confirm intended source content survived, the new directory is its own Git/givn root, deterministic harness tests run there, and an explicit subtree synchronization reproduces the intended contents under `givn/harness/`. Keep transient/private data out of synchronization. Do not count a remote entry alone as a working subtree relationship.

Treat “move the files it needs” as a safe extraction/import followed by establishing the synchronized prefix, not permission to erase the original directory before preservation is verified. Develop only in the standalone repository afterward; the parent prefix is a downstream synchronized copy, not a second independently edited source tree. Document when/how to synchronize focused harness revisions back to givn.

Use a known, pinned givn installation to develop the harness. Pin the **givn binary under evaluation separately for each trial**; a harness-development upgrade must not silently alter an experiment. Using givn to develop the evaluator does not make its approval the benchmark oracle: independent behavioral tests determine delivered-software outcomes.

Read applicable repository instructions before implementation. This document is not permission to ignore them, discard work, publish externally, or make unrelated givn product changes. A fresh agent must inherit the model restriction, USD 5 ceiling, accepted decisions and effective question policy; ask only genuinely unresolved questions, one at a time. Stop if actual local-work ownership or a safe subtree conversion cannot be established, rather than inventing permission to overwrite it.

Verified implementation concerns to address in the relevant stages:

- `evaluator.py`: mostly form checks; a nonempty step file is not an executable assertion; review checks use substrings.
- `profile.py`: full-task completion accepts any archive entry. That would falsely complete a follow-up merely because the initial change is archived.
- `assets/ralph-givn-prompt.md` and `runner.py`: force one-step execution, repeat givn rules, leave unused placeholders, and differ between driver paths.
- `cli.py`: unanswered questions exit before reporting; LLM grading runs by default; resumed measurement starts over; unavailable sandbox merely warns.
- `sandbox.py`: mounts host agent state and broad caches; shares host networking; calculates resource-limit information without propagating it into reports.
- `runmeta.py`: revision identity/dirty-path handling and effective prompt provenance need tests; a version string alone does not identify a binary or instruction overlay.
- `scripts/check-step-bodies.sh`: failure state is assigned inside a pipeline subshell; test that a deliberately empty step actually makes the script fail.
- Existing specifications include legacy sandbox/fallback and judge expectations. Update affected contracts intentionally; do not preserve misleading behavior merely to keep old assertions green.

Historical numerical findings in the interrupted draft are hypotheses, not new measurements. Do not import its campaign cost estimates, noise estimates, model-price comparisons, or phase timings as facts.

## 4. Architecture and user operations

Use one shared normalization/accounting pipeline for imported records and new trial bundles. Keep execution, grading and analysis separate; introducing one dataclass/module per real responsibility is sufficient. No plugin framework or database is required.

Target user operations below describe **new behavior**, not commands that already work:

- `givn-harness analyze <events.jsonl> --output <directory>`: local, read-only process analysis; optional scope/session/time filters, repository and metadata sidecar.
- `givn-harness run --benchmark <manifest> --givn-binary <path> --output <directory>`: execute the benchmark's stages and preserve a bundle.
- `givn-harness grade <bundle> --grader <manifest>`: independently grade saved snapshots; no model request.
- `givn-harness compare <baseline-bundles> <candidate-bundles>`: show comparable results, differences and missing evidence. A later campaign option can schedule both conditions.

Finalize exact flags during CLI implementation. Existing `run`/`compare` have different semantics; provide explicit compatibility or migration errors rather than silently changing old invocations. Legacy smoke tasks remain transport/format tests, not product-quality benchmarks.

### Durable trial bundle

Preserve a versioned manifest, append-only execution journal, raw record/transcripts, question-and-answer log, stage snapshots, independent test results, normalized metrics, evidence-linked findings and Markdown report. Each analysis/grade records its own version and input checksums; regrading adds a result instead of overwriting history.

The manifest identifies the harness revision/dirty state, givn binary checksum and source ref when available, served instruction/overlay configuration, OpenCode/runtime versions, effective model/variant, benchmark/grader hashes, resource/network policy, budgets, repeats and stage boundaries. Resume appends segments and preserves previous work, spend and timing.

Git history alone omits uncommitted/untracked output. Preserve those files and status too, with checksums and safe snapshot extraction. Mark conflicts between the committed archive receipt and the final working tree. Freeze output before grading. The agent cannot modify the authoritative bundle.

Every measurement carries a unit, provenance, evidence references and a status such as observed, inferred, partial or unavailable. Outcome, workflow completion and measurement validity are separate dimensions, not one `overall_pass` boolean.

## 5. Retrospective `events.jsonl` analysis

Deliver this early: it provides immediate value without any paid executions and exercises the same accounting needed by benchmarks.

### Inputs and limits

- A bare record is enough for observed event counts, available durations/usage, errors and repeated patterns. Accept multi-project/multi-session files and filters; never assume one file equals one trial.
- An optional repository, transcript or manifest enriches revision/phase attribution and provenance. Missing supplemental inputs do not prevent analysis; they limit claims.
- Snapshot/hash the input or fix an explicit byte boundary when it is still being appended. Never modify the source record or repository. Do not run imported commands, tests, hooks, plugins or repository code as part of analysis.
- Streaming parsing should tolerate large files, unknown kinds/schema versions, malformed lines and an incomplete final line. Count and locate unsupported/damaged input rather than silently dropping it.
- Namespace event IDs by source record. Preserve parent/session/scope links. Deduplicate only where identity is proven; conflicting IDs, repeated exports and ambiguous subagent spans produce diagnostics rather than guessed merges.

### Accounting rules that must have fixtures

1. `at` is an end timestamp. A known interval is `[at - duration_ms, at]`; absent duration stays absent. Retain original order and timestamps, and diagnose inconsistencies.
2. Nested commands, tools, turns and subagents overlap. Report interval unions and per-category totals separately; never sum all durations into wall time. A bare log's observed envelope is not full trial elapsed time or uninterrupted active work.
3. Usage can appear inline on `turn` and again in a parent-linked `usage` event. Choose one authoritative representation per request; never add both. Preserve ambiguous/unlinked observations and flag total uncertainty. Do not double-count reasoning/cache fields whose inclusion semantics are unknown.
4. Cost may be numeric, a component object, per-request, or a cumulative rollup depending on the producer. Normalize only known semantics. Report observed cost, completeness and billing basis; do not sum cumulative totals or call list-price estimates actual charges. Missing cache/usage information stays missing. Any price estimate names its dated price source and assumptions.
5. Keep recorder plumbing (`history ingest`, hook intake, attribution) in the raw record but exclude it from user-command friction totals through tested classification rules.
6. Current `phase` markers mean first observation of an artifact being Done, not necessarily work starting. Prefer externally captured stage/workflow boundaries. Git/artifact activity and transcript evidence may support labeled estimates; commit timestamps are not ground truth for time spent. Uncertain phase shares remain unattributed.
7. Tool results may expose failures missing from givn's command records. Correlate using IDs/session/input/time where justified; retain unmatched evidence. Classify first error lines without losing the original locations.
8. An error or nonzero command can be expected evidence, such as a RED test. A repeated command is not automatically waste; distinguish explicit failures, eventual recovery and suspected avoidable rework.
9. Questions may contain text/wait duration but no answer. Do not infer approval from silence, counts or final artifacts. Bounded/truncated content cannot be reconstructed.
10. Provider/model/harness/variant identities may be incomplete. Accept user-supplied metadata as declared, not observed. Never infer producer identity solely from a model name.

The current root record was inspected structurally during planning: schema 1, about 5.25 MB and 13,795 events, containing both turn and usage entries and object-shaped costs. These figures only establish that a real input exists; do not freeze them into tests. Create small sanitized fixtures with explicit permissions instead of committing private full logs.

### Useful output

Report data-quality gaps first, then a timeline, measured/estimated time and cost breakdowns, expensive operations, failure/retry clusters, archive friction, questions, compaction/context pressure where available, and evidence-linked improvement hypotheses. Example: “archive was attempted four times; three outputs report a dirty tree; the following event IDs show recovery.” Do not assert that all intervening time/cost was caused by those errors.

A record alone cannot establish product correctness, an independent false-green rate, whether a review prevented a defect, or whether removing a rule would improve results. Label those as unmeasured. Optional executable re-evaluation of a supplied snapshot is a separate, explicitly requested sandboxed operation.

## 6. Benchmark contract and independent grading

The first benchmark is a small lending register, not the evaluation harness implementation.

| Stage | Required behavior |
|---|---|
| Initial delivery | Set up members/books through the documented interface; borrow an available book; reject an already-loaned book; enforce five active loans per member; return a book and release capacity; retain state across restarts. Rejections must not corrupt state. |
| Follow-up change | Staff can mark a book reference-only. Reject new borrowing of that book while allowing an existing loan to be returned. Preserve unrelated lending behavior. |

The benchmark author freezes a minimal structured CLI contract before writing the reference: startup command, isolated data directory, command/result schemas, identifiers and required error codes. A JSON request/result interface is suitable. All independently graded obligations must appear in the agent-visible contract; tests hide combinations and boundary cases, not expectations. Pin Python and dependency/tool versions, preferably using the standard library for this task. No network service, UI or external integration is necessary.

Leave architecture/storage implementation open. Automatically accepted defaults are logged; grade their consistency only where an independent check exists. A renewal policy or due-date choice is not a hidden requirement and must not inflate the delivered-requirement count.

The benchmark package explicitly separates public task/scaffold from private tests, references and seeded-defect outputs. Copy only allowlisted public files, not sibling files selected by substring. Private material never enters the agent sandbox or public subtree.

Before paid use, require the reference to pass **all** required checks; the empty scaffold must fail the substantive requirements. Seed defects for the loan limit, book exclusivity, persistence and reference-only returns, and verify the intended checks fail. Do not assume an instruction to skip testing necessarily produces a behavioral defect.

Grade a frozen snapshot separately from development. Keep the trusted grader/assertions outside the submitted program's readable namespace; invoke the application through its black-box entry point in a submission sandbox rather than importing its code into the grader's interpreter. Use clean grading data and deterministic fixtures. Treat outputs as untrusted; no host credentials/network, unsafe archive extraction, host sockets or unrestricted process access. Record installation/startup failures without repairing submissions or inventing interface adapters after seeing a solution.

Include cross-version state checks: create an active loan through the initial snapshot, then run the follow-up snapshot against that same data directory and verify it can return the loan. Merely creating and returning a new loan under the follow-up version would not prove preservation of existing persisted data. Keep this private grading state out of the development workspace and do not send grading feedback to the agent.

Report requirement-level results, critical failures, full-delivery success, preserved behavior and new regressions. Do not treat twenty assertions for one rule as twenty independent requirements. Compare only still-applicable requirements across stages; version and explain deliberately superseded rules. Own-suite green versus independent failure is a false-green observation only if own-suite execution evidence exists.

## 7. Execution, answers, isolation and spending

### Neutral execution

The driver delivers the task, permitted execution environment, answer policy and completion protocol. Givn-installed guidance supplies givn's lifecycle, testing and commit requirements. Do not repeat those rules in the driver or prohibit implementation as the old smoke task does.

Use a private OpenCode server and an event/API adapter that answers native question requests without restarting after each artifact, scenario or question. Preserve session continuity, compaction and child-session behavior within a stage. Test against the installed V2 API before choosing exact calls. If same-session handling is unavailable, document the limitation rather than silently substituting an incomparable restart regime.

Require one unambiguous recommended option for a genuine open choice. Log the full question/options and selected answer. Missing/ambiguous recommendations get a bounded protocol correction, then a recorded protocol failure—not an arbitrary default. The responder never exposes hidden tests or consults a private answer brief. Clearly detectable requirement conflicts are not approved; arbitrary semantic compliance remains independently tested or ungraded, not magically solved by the responder.

Start the follow-up in a fresh session without injecting prior conversational context; preserve the produced repository, including its declared givn artifacts/record. Record those handoff materials. Do not claim this isolates the value of specifications alone: repository history or captured records may also help. A later controlled handoff ablation can separate them.

Workflow completion must concern the newly requested change, not any preexisting archive or PASS substring. Validate version-appropriate archive/tested-revision evidence without using the host's newer givn binary to validate an older trial. Grade every stopped stage, even when workflow completion fails. Deliver the follow-up only after initial workflow completion and a working public entry point; **hidden test success is not a continuation signal**. Otherwise record follow-up `not_reached` and retain the incomplete trial in results.

### Isolation

- Retain Bubblewrap, but use an allowlisted runtime/root filesystem, private HOME/XDG directories, private temporary storage and isolated process/IPC namespaces. Mount tools read-only and only this trial's workspace/state writable.
- Do not mount host agent history/configuration, broad home/cache trees, personal Git configuration, host sockets or host `/etc` wholesale. Supply the minimal certificates/resolver/system files actually needed. Use a pinned cache seed plus per-trial writable caches, not a shared mutable agent cache.
- Start OpenCode with a private server. Verify all effective paths using `opencode debug paths`; `--standalone` alone does not isolate disk state. Inspect system and ancestor instruction/config discovery too.
- Inject only the necessary DeepSeek credential. Do not copy the normal OpenCode database: V2 stores session data and credentials there. Do not log secrets; protect sensitive raw transcripts and make exported reports redacted by default.
- Use a private network with constrained outbound access to the direct DeepSeek endpoint. The current `--share-net` and proxy variables alone do not enforce this. Implement and test a restricted egress route/gateway without substituting another model provider. Preinstall benchmark dependencies. Later complex tasks declare additional services/dependencies explicitly; never enable unrestricted networking silently.
- Keep authoritative capture and private grading outside the execution sandbox. Apply CPU/memory/process/disk limits, record enforced values, terminate child processes on stop and check for leftovers. Shared-kernel isolation does not eliminate hardware contention or constitute a VM-grade hostile-code boundary.
- Prove access denial using harmless sentinel files and fake host-local services. Prove allowed tool/provider access separately. Unavailable isolation means no graded execution.

### Model and USD 5 enforcement

Allowlist the exact provider, Flash model and max variant across primary, subagent, compaction, transient generation and any title/summary requests. Supply deterministic titles or disable unnecessary auxiliary generation. Disable automatic updates/warming and unintended integrations. Reject incompatible givn-generated agent selections before they dispatch; record this rather than hiding it through a fallback.

OpenCode V2's root model default does not retain a variant and unavailable defaults can fall back. Therefore a CLI selector alone is insufficient: use protected configuration plus a minimal read-only guard integration to constrain the available model inventory and validate native outgoing requests. V2 documents model transforms and request hooks; verify the installed version and test primary/auxiliary paths with intercepted transport before paid use. Provider-only policies do not prohibit DeepSeek Pro, and network hostname rules do not prohibit another model at the same endpoint.

Maintain one durable campaign ledger outside the sandbox, initially sequential. Account for all requests and reserve a conservative maximum for each pending request before dispatch, using verified prices and enforced request/output bounds; a cancellation does not refund a dispatched request. After crashes, retain uncertain charges as reservations rather than restarting the budget. Do not begin another request when charged plus reserved exposure could exceed USD 5.

Post-response totals alone cannot guarantee a hard invoice ceiling. Before real execution, prove conservative admission accounting or use a verified provider-side spending limit. If neither is possible, stop before paid calls and report the blocker; do not claim an exact cap or overspend silently. Missing usage/cost also stops additional paid execution until reconciled. Failing to finish the pilot within USD 5 is an acceptable reported outcome, not permission to raise the cap.

## 8. Comparison and interpretation

First run the same givn condition more than once if the budget permits. This exercises reproducibility and begins exposing stochastic variation; it is not a requirement that identical versions yield identical costs or outputs.

Then vary one condition at a time, initially givn revision. Fix task/grader, stack, model/variant, OpenCode version, driver, answer policy, capture mode and resource budgets. Build isolated binaries and record dirty-source differences. Interleave/randomize baseline/candidate execution; start sequentially to simplify accounting and resource contention. Concurrent runs require concurrency-safe spend reservation and documented resource allocation.

One trial is the initial-plus-follow-up sequence. Each condition receives the repository **it produced**, not the other condition's repository. Frozen-checkpoint replay is a separately labeled experiment, useful later for fast instruction checks but not equivalent to idea-to-archive evaluation.

Show individual trials, completion/failure counts, requirement results, regressions, cost/time distributions and evidence validity. Include every attempt and make exclusion reasons visible. External outages, agent/task failures, budget exhaustion and capture failures must not be conflated; outages remain in operational reliability reporting even when a conditional comparison excludes them.

Do not collapse quality and resource use into one score. A faster version that breaks returns is not an unqualified improvement. Start descriptively; report “inconclusive” when evidence is weak. Do not promise that three/five repeats establish significance, or derive confidence from the number of correlated assertions. Later uncertainty estimates resample independent trials/tasks appropriately, and practical quality margins/cost guardrails are agreed before interpreting a campaign.

The allowed model alias may evolve upstream. Store any returned model/version identifiers and request dates. Concurrent/interleaved comparisons reduce temporal confounding; they do not guarantee it cancels. Longitudinal same-model canaries can detect changed outcomes but cannot prove which upstream change caused them.

## 9. Delivery stages and exit gates

Implement each stage as a focused increment with deterministic tests and a short handoff recording files, decisions, test evidence, limitations and remaining budget. Preserve previous useful deliverables; no paid run is required to finish the first five stages.

| Stage | Deliverable | Exit gate |
|---|---|---|
| Bootstrap the standalone repository | Start in the user-created `/home/buster/projects/givn-harness`; preserve/import the needed existing harness code and local work, reconcile its givn setup, and establish tested local subtree synchronization with `/home/buster/projects/givn/harness`. Record tools, fixtures and test entry points. | Standalone Git/givn root works; intended content/history/local work survive; deterministic tests run there; subtree synchronization is demonstrated; no unrelated product changes or external publication. |
| Shared analyzer | Streaming record reader, accounting, evidence-linked findings, `analyze` CLI, Markdown/JSON outputs. | Small fixtures cover overlap, duplicate usage, cumulative/missing cost, schema damage, truncation and privacy; a real local file produces a useful report without network/code execution. |
| Small benchmark and grader | Public lending contract, two stages, private reference/defects, standalone grader. | Correct reference passes all required checks; each seeded defect is caught; saved snapshots regrade deterministically. |
| Safe execution foundation | Trial manifests/journal, sandbox, protected model configuration, spend ledger and boundary probes. | Fake/intercepted executions prove isolation, all model paths, budget admission, missing-cost stop and crash/resume behavior. No paid call before this gate. |
| Neutral staged driver | Automatic answers, target-specific completion, fresh-session follow-up, snapshots and durable bundles. | Fake agents exercise success, conflicting/missing recommendations, failure, timeout, interruption, resume and follow-up not reached; no result disappears. |
| Real loop validation | One real two-stage run, then same-version repetitions only as funds permit. | Inspect actual answers, model selection, grading, measurements and regrading; report spend/remaining reserve. Correct infrastructure defects before claiming givn findings. If blocked or out of funds, stop with evidence. |
| Version comparison | Baseline/candidate scheduling and reports, then one real comparison if remaining authorization permits. | Synthetic known differences and equal-condition fixtures report correctly; real results expose limitations rather than manufacture a winner. Larger campaigns need new funding. |
| Complex-task support | Multi-stage manifests, longer brownfield/evolution task and resource profiles using the same runner/grader/bundle. | At least one deterministic multi-change fixture and reference exercise carry-forward state, superseded rules, partial failure and resume. Real complex evaluation requires a separately approved budget. |

Use the project's existing Python test conventions. Fake agents/protocol fixtures must not contact providers. Separate deterministic, sandbox-integration and paid tests; running the default suite must never spend money. Fix/test the step-body audit and assert real behavior, not just that report files exist.

## 10. Fast and complex evaluation, without two harnesses

| Tier | Purpose | Scope and interpretation |
|---|---|---|
| Offline analysis/regrading | Immediate diagnosis and instrument verification. | Existing records and snapshots; no model spend. Useful for real-world friction mining. |
| Small complete workflow | Frequent givn/instruction regression checks. | Lending task plus follow-up; repeats and turnaround sized from observed pilot costs, not a promised minute count. |
| Complex evolution | Realistic maintainability and cross-cutting regressions. | A brownfield application with several ordered changes and fresh sessions, independently checked invariants, pinned dependencies and larger budgets. |
| Phase/checkpoint replay, later | Cheap targeted screening. | Resume a frozen input at one workflow boundary. Report the artificial starting condition; confirm promising results end to end. |

For the first complex task, prefer a curated existing shopping-list/evolution project if a runnable reference and stable observable interface are available. Examples include quantities followed by recipient-hidden gifts, which stress preservation across changes. Do not automatically port hundreds of historical scenarios or select a task merely because it is large. Start with an inspectable set of cross-cutting requirements, then expand diversity. Browser/UI tasks require their own pinned public interaction contract and grader before joining quality comparisons.

The benchmark manifest declares public inputs, private grader/reference locations, stack/runtime, stage requests, applicable/superseded requirement sets, startup/test contracts and resource budgets. The runner iterates stages; no separate “complex harness” implementation. Freeze/version old tasks, and add tasks when the small one saturates. Functional ceiling, cost improvements and reliability improvements are distinct signals.

## 11. How the harness improves givn

Use a repeatable loop: **observe → form a narrow hypothesis → change one thing → compare → retain/revise**.

Example: imported real runs show repeated archive rejection associated with generated files dirtying the tree. Inspect the cited outputs; propose a focused fix; compare baseline/candidate on the small workflow; confirm promising results on a complex case. A costly review step is a candidate for investigation, not proof that it is unnecessary. Compliance without a prompt is not sufficient evidence to delete that prompt.

Later component ablations can compare shipped givn against one removed/changed component, keeping all other guidance equivalent. Only after that substrate works consider no-givn versus spec-core comparisons. They require fair stopping rules and neutral prompts, not the current form checks. No alternate live model/provider or harness is authorized by this plan; those extensions require an explicit revision of the execution constraint. Offline import remains permissive.

Defer LLM artifact scores, autonomous improvement loops, mutation campaigns, dashboards, databases, broad model matrices and changes to givn's event schema. If independent measurements later justify an LLM judge, it remains restricted to the authorized model, is calibrated on seeded defects/human labels, and is disclosed as correlated rather than an independent oracle. Small givn recorder fixes can be separate product changes; the analyzer must tolerate old logs regardless.

## 12. Definition of done and sources

The usable harness can: analyze a supplied real-world record with honest limitations; run/regrade a small complete workflow; compare pinned givn revisions without losing failures; execute a multi-stage complex benchmark through the same machinery; and preserve enough evidence for another person/agent to inspect each claim. README examples identify which operations are offline, isolated or paid, which commands are new, and what budget approval they require.

No more domain or preference questions are required to begin the deterministic implementation stages. Stop for actual blockers, ambiguous user-work ownership, unsupported enforcement or new paid authority rather than silently relaxing decisions. Implementation must occur outside Plan mode.

Sources for the implementing agent:

- Unless an absolute path is shown, source paths below are relative to `/home/buster/projects/givn`. After extraction, a source path such as `harness/src/givn_harness/cli.py` becomes `src/givn_harness/cli.py` in the standalone repository. Rust record/CLI sources remain in the givn product repository and are read-only references for this work.
- User's latest bootstrap instruction: `/home/buster/projects/givn-harness` exists, no files have been moved, and the implementing agent must import the needed harness files and establish the local subtree relationship.
- User discussion and `harness/improv-plan/idea.md`: goals, automatic recommendations, fixed Python application stack, exact model restriction and USD 5 ceiling.
- `harness/improv-plan/claude-plan.md`: earlier investigation and recorded repository/subtree choice; superseded where it conflicts with this document.
- `harness/README.md`, `harness/src/givn_harness/{cli,runner,sandbox,workspace,profile,evaluator,model,runmeta,judge}.py`, existing tests and scripts: implementation starting point.
- `src/history.rs` and `src/cli/history.rs`: event envelope, end timestamps, missing-value semantics, linked usage, bounded content and intake behavior. `src/cli/timeline.rs`: existing report heuristics, not an unquestioned accounting oracle.
- OpenCode V2 documentation checked during planning: https://opencode.ai/v2/docs/config, https://opencode.ai/v2/docs/cli, https://opencode.ai/v2/docs/cli/providers, https://opencode.ai/v2/docs/models, https://opencode.ai/v2/docs/agents, https://opencode.ai/v2/docs/policies, https://opencode.ai/v2/docs/build/plugins. Recheck against the installed version before coding API calls.
- Bubblewrap documentation: https://github.com/containers/bubblewrap. Namespace/filesystem primitives do not automatically provide an outbound allowlist or a complete security policy.
