# Review: prepare-0-9-10

Review kind: independent
Author: maki-author
Reviewer context: preparation-reviewer

## Findings

| ID | Severity | Finding and evidence | Disposition |
|---|---|---|---|
| F-01 | critical | `design.md:256,257,261,270` named four step-definition files that do not exist: `tests/steps/refresh_generated_guidance_steps.rs`, `tests/steps/review_architecture_documentation_steps.rs`, `tests/steps/report_change_status_steps.rs`, `tests/steps/check_specification_health_steps.rs`. `givn check review --change prepare-0-9-10` printed `[integrity] Step Definitions: file ... does not exist` and `integrity check failed: promised files`. The behavior bindings existed in other durable files (`skill_rewrites_steps.rs`, `arc42_review_alignment_steps.rs`/`instruction_integrity_steps.rs`, `run_command_steps.rs`, `spec_overlap_gates_steps.rs`/`workflow_enforcement_steps.rs`), so the design table, not the behavior, was wrong. Separately, `design.md:251` names `tests/steps/llm_attribution_installation_steps.rs` as a regular step file, but it has zero `#[given]`/`#[when]`/`#[then]` bindings (it is a helper) — see the residual note below. | Closed in `7bef6352`. The table now names the four actual durable files; each exists, is registered in `tests/steps/mod.rs`, and holds the bindings. `givn check review` no longer reports a promised-file failure, and design-review records the correction as PR-07. |
| F-02 | low | `src/cli/outline.rs:26` shipped the author's harness-project name as the outline help example: `givn outline report --path catopia --path app.py`. Proposal item 18 requires removing harness-project leakage from current shipped guidance/help, and Catopia is outside historical evidence here. | Closed in `7bef6352`. The help now reads `givn outline report --path src --path app.py`. A search of `src/`, `tests/`, `assets/`, `README.md`, and `givn/specs` finds `catopia` only in the leakage-guard marker list at `tests/steps/instruction_integrity_steps.rs:3166`, which asserts its absence; no shipped surface carries it. |

Targeted closure (this revision): F-01 and F-02 were returned and closed in `7bef6352`; no source, spec, or design edit was made in this closure round beyond that commit, and this round changed `review.md` and `design-review.md` only. The design change after the initial design-review sign-off is recorded there as PR-07 (path correction in one table, no decision, matrix row, use case, or scenario changed). Gate re-run: `cargo run --quiet -- check review --change prepare-0-9-10` exits 0 with `✓ integrity checks passed`, `✓ run declaration passed`, `givn lint: 29 file(s) checked — clean`, `✓ overlap dispositions passed`, `✓ scenario commits passed (245 scenario(s))`, and `✓ change verification receipt reused across documentation-only revision 08a34df8… (tested at f83289ff…, 333 scenario(s))`; the remaining output is 6 shape and 995 subset advisories, no failures and no promised-file error.

Remaining risks, not additional findings:

- `design.md:251` (`enable-commit-attribution`) still lists `tests/steps/llm_attribution_installation_steps.rs`, which is a registered shared fixture module with zero step bindings. That capability's delta scenarios are all `@e2e` and bind in `llm_attribution_installation_e2e_steps.rs`, so no behavior is unbound and no gate fails; the table row is imprecise, not uncovered.

- The per-batch `RED:`/`GREEN:` records in `tasks.md` are prose summaries; the RED lines for many batches are repeats and the GREEN lines repeat the same failure wording. They do not name the focused failing command or its output. The executable tests and the 333-scenario receipt are real; the test-first record itself is not independently reproducible from `tasks.md`.
- The strictness mechanism is present by source inspection (`tests/features_runner.rs:55` `.fail_on_skipped()`, `tests/features_runner.rs:68-70` failed/skipped exit), but `tasks.md` records the mechanism rather than a fresh strict-mode output.
- Four checked non-scenario closeout tasks (`tasks.md:759-761`) have no `COMMIT:` line; they are documentation/evidence closeout, not behavior scenarios.
- Sixteen of the 30 terminal transcripts use `argv: givn (scenario invocation)` because the capture helper has no `last_command` for those scenarios. Each transcript's first line exactly matches the scenario title and the body is the actual captured stdout/stderr/exit; only the argv label is generic.
- No archive-scope coverage measurement exists for this change yet; the archive receipt remains the authority and the change is an unproven candidate.

## Implementation Evidence Check (do this first)

### Step definition scan

Command: a Python scan of every `tests/steps/*.rs` file for step functions with empty bodies, whitespace-only/comment-only bodies, or `unimplemented!`/`todo!` stubs.

Result: 0 empty behavior-step bodies across 80 step-definition files. Four deliberate strict-mode fixture steps exist in `tests/steps/initialize_a_project_e2e_steps.rs:1267-1281` (`fixture_assertion_fails`, `fixture_step_pending`, and two ambiguous-binding steps). They are bound only to the runner's own fault-injection feature strings at lines 1155-1167, not to any delta behavior scenario, and prove the runner fails on failing, pending, and ambiguous steps. They are not behavior evidence for a changed capability. Two comment-only files (`tests/steps/run_command_e2e_steps.rs`, `tests/steps/serve_extended_rules_e2e_steps.rs`) are pre-existing skeletons; neither is named by this change's design table and neither is used as evidence.

### Commit cross-check

245 delta scenarios are checked, each with a reachable `COMMIT:` revision and a `Batch:` label. Six unique revisions cover them; `givn check review` reports `scenario commits passed (245 scenario(s))`. Spot-checked `git show --stat` for all six:

| Batch revision | Scenarios | Production/test content in the revision |
|---|---|---|
| `a7e9f30b` | 70 | 3 `src/` files, 10 test files, 19 assets/instruction templates, README/docs evidence |
| `8d576d01` | 66 | 4 `src/` files, 4 test files (skills-only init/upgrade, attribution, hooks) |
| `595abe2a` | 59 | 7 `src/` files, 7 test files (verification/status/review behavior) |
| `64ba38e7` | 33 | 5 `src/` files, 6 test files (archive proof, supported Gherkin, scope reuse) |
| `1b2ef01f` | 10 | 3 `src/` files, 18 test files, 7 skill templates (full-suite reconciliation) |
| `3bbf3cc0` | 8 | 19 `src/` files, 5 test files (advisory outline, gate removal) |

No checked scenario lacks a commit. The four checked closeout tasks listed under "Remaining risks" are the only checked boxes without a `COMMIT:` line.

### Promised-architecture cross-check

The changed behavior components exist: `proof::VerificationInputs`, `proof::ScopeEvidence`, `proof::Receipt::reusable_scope`, `gherkin::merge::validate_supported_feature`/`parse_supported_source`, `git::GitRepo::commit_selected`, `fossil::FossilCheckout::commit_selected`, `skills::targets::reconcile_legacy_commands`, `cli::run::probe_before_deadline`, `state::actual_entries`, and the advisory outline path without `outline::gate`. Consumers use those module APIs; no consumer imports an implementation-only path. The Step Definitions table is the exception: see F-01.

### Strict-mode proof

Present by source inspection: `tests/features_runner.rs:55` calls `.fail_on_skipped()`, and `tests/features_runner.rs:68-70` exits non-zero when `stats.failed > 0 || stats.skipped > 0`. `tasks.md:7` states the same mechanism. No fresh strict-mode command/output is recorded in `tasks.md`; see the remaining risk above.

### Downgraded e2e scan

All 30 delta `@e2e` scenarios were read. Their Then steps assert CLI exit status, stdout/stderr, generated files, or a repository-side effect of the real CLI; every scenario has a primary CLI/file-output assertion and a terminal transcript capture step. No step implementation sends HTTP, calls `fetch()`/`XMLHttpRequest`, or substitutes a server call for the CLI. The only browser driver in the delta E2E files is the pre-existing `BrowserDriver` capture used by the agent-activity/timeline report; it renders the report the scenario's own record produced and does not replace the measured CLI interaction. No downgraded scenario was found.

### E2E scope, duplicate implementation, and local environment

| `verify.command` | `verify.e2e_command` | Explicit scope selection | Regular scope evidence | E2E scope evidence | Zero failed/skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | `run-tests.sh` keeps caller `GIVN_RESULT_SCOPE`, otherwise derives `e2e` from `--e2e` and `regular` otherwise; archive passes `GIVN_RESULT_SCOPE` explicitly | `verification/tasks-verification.json`: passed, 333 total, 0 failed, 0 skipped, exit 0, at `56519c8a` | 30 `@e2e` transcripts under `evidence/visual/<scenario-slug>/transcript.txt`; no archive e2e receipt yet | Regular: yes. E2E: no receipt, transcripts are non-empty and title-bound |
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e` | `GIVN_FEATURES` selects the delta or permanent corpus; the runner scope is explicit | 333 changed scenarios | 30 E2E scenarios | Same as above |

`verify.e2e_command` resolves to `run-tests.sh --e2e` → `tests/features_runner.rs` → the registered `tests/steps/*_e2e_steps.rs` modules. A tree-wide search, including `git status --porcelain` (clean), found no parallel or stronger E2E implementation for these capabilities. The local run declaration is `cargo run -- instructions`; the project has no server/database stack, containers, external network, or digital twins, so there is no fuller stack to start.

### Design.md conformance

| Decision named by `design.md` | design.md says | Actually built | Match? |
|---|---|---|---|
| Verify commands | `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` | Same values in `givn/commands.yaml` | yes |
| Runner | `tests/features_runner.rs`, cucumber-rs, fail-on-skipped/failed exit | Same; the change adds a scenario-title `before` hook only | yes |
| Terminal evidence | one transcript per E2E under `evidence/visual/<scenario-slug>/transcript.txt` | 30 transcripts, exact title first line | yes |
| Step-definition files | exact durable paths in the table | the four corrected paths exist, are registered, and hold the bindings (F-01 closed in `7bef6352`); one row still names a fixture helper | yes — F-01 closed; fixture-row imprecision recorded as a risk |
| Coverage scripts | existing `measure-coverage.sh` / `merge-coverages.sh` | Same scripts; commands wired in `givn/commands.yaml` | yes |
| Outline | advisory report, no gate/verdict | `src/outline/gate.rs` absent; report returns code 0 and no verdict | yes |
| Archive scope reuse | per-scope witness, result digest, shared/exclusive command digests | `proof::VerificationInputs::{capture_archive,reusable_for}` and `Receipt::reusable_scope` | yes |

### Interaction coverage

Cross-reference: every delta use-case/fragment `## Interactions` row was parsed, matched to the design matrix title, matched to an existing `@e2e` scenario, and checked against the registered binding's driver. All 74 inventory rows map to a matrix row and an `@e2e` scenario. All drivers are actual CLI subprocess/PTY/harness invocations; no HTTP or in-page fetch is used.

| Owning document | Inventory rows | Matrix rows | `@e2e` scenarios | Driver | Match |
|---|---:|---:|---:|---|---|
| archive-change | 9 | 9 | 9 | CLI subprocess/PTY | yes |
| assess-code-structure | 1 | 1 | 1 | CLI subprocess | yes |
| create-change | 7 | 7 | 7 | CLI subprocess/PTY | yes |
| discover-specifications | 2 | 2 | 2 | CLI subprocess | yes |
| fragments | 13 | 13 | 13 | CLI subprocess/PTY | yes |
| initialize-project | 4 | 4 | 4 | CLI subprocess/PTY | yes |
| inspect-change-state | 4 | 4 | 4 | CLI subprocess | yes |
| keep-documentation-current | 5 | 5 | 5 | CLI subprocess | yes |
| obtain-effective-guidance | 1 | 1 | 1 | CLI subprocess | yes |
| operate-product | 10 | 10 | 10 | CLI subprocess/PTY | yes |
| publish-a-release | 1 | 1 | 1 | CLI subprocess | yes |
| upgrade-a-project | 4 | 4 | 4 | CLI subprocess | yes |
| verify-change | 13 | 13 | 13 | CLI subprocess/PTY | yes |
| **Total** | **74** | **74** | **74** | | **74/74** |

The design matrix has seven additional retained E2E rows (per-harness attribution variants, the two match-guidance addon states, the removed domain-skill variant, and the agent reporter). Those are the design-review PR-06 projected retained E2Es, not new delta inventory actions; they do not create an unmapped delta interaction. The seven projections were already signed off in design-review and are not reopened here.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `specs/*/usecase.md` delta narratives retain Goal, Main flow, Rules, guarantees and capabilities; no new user goal or capability owner was introduced; the only retirement is `outline-review-gate`, disclosed below | yes |
| Capability ownership and typed relationships preserved | `givn check review` reports routing consistency; all 245 delta scenarios resolve to existing owners; no typed-reference error | yes |
| Confirmed Persona-relevant outcomes preserved | `givn/personas/developer--primary-user.md`, `agent--technical-role.md`, `givn-maintainer--owner.md` exist; the review lens table below dispositions all three | yes |
| Actors remain distinct from Persona biographies | use-case `## Actors` name interaction roles (Developer, Reviewer, Maintainer, Agent); Persona files remain stakeholder lenses | yes |
| Migration ledger and coverage evidence complete, if applicable | no semantic corpus migration in this change; no migration ledger required; coverage measurement status is recorded below | yes |

Confirmed Personas: `developer--primary-user` (clear repair/no unexpected work), `agent--technical-role` (stable counts/status/explicit actions), `givn-maintainer--owner` (truthful scope/retained history). These are the existing identities; no Persona was invented or reconfirmed.

PERSONAS: developer--primary-user

## Visual Review

The design's `## Visual Design Contract` declares `Interface: terminal`. The opposite classification — a rendered page is a Web UI — does not hold: every changed surface is line-oriented CLI stdout/stderr or a generated file, the transcripts assert those bytes, and the only browser-rendered surface (the timeline report) is pre-existing and unchanged. No new browser screen or viewport was added, so no screenshots are required.

Evidence completeness: 30 delta `@e2e` scenarios have 30 non-empty `evidence/visual/<scenario-slug>/transcript.txt` files. Each transcript's first line exactly matches its scenario title; each carries `exit code`, `--- stdout ---`, and `--- stderr ---`. All 30 are committed in `24808c77`; no fabricate/snapshot file was found.

| Declared screen/state | Representative transcript | Present and non-empty? |
|---|---|---|
| init/sync diagnostics | `consumer-runs-init-non-interactively-and-gets-opencode-skills/transcript.txt`, `consumer-re-syncs-skills-after-init-and-the-selection-is-honoured/transcript.txt` | yes |
| explicit commit errors/attribution | `an-attributed-commit-exposes-the-current-llm-and-harness/transcript.txt`, `consumer-sees-the-commit-attribution-installation-report/transcript.txt` | yes |
| runner/setup/check/status output | `consumer-runs-check-tasks-and-receives-a-change-verification-receipt/transcript.txt`, `detail-mode-active-change-with-all-required-done-text/transcript.txt`, `consumer-invokes-bare-check-and-receives-explicit-non-executing-guidance/transcript.txt` | yes |
| outline report | `a-developer-runs-a-outline-report-and-sees-the-classified-units/transcript.txt` | yes |
| archive/recovery/upgrade outcomes | `consumer-repairs-a-blocked-archive-and-the-rerun-proves-it/transcript.txt`, `a-clean-project-without-migration-requirements-receives-one-managed-upgrade-commit/transcript.txt`, `a-completed-maintenance-migration-can-be-archived-without-a-product-specification/transcript.txt` | yes |
| effective instruction text | `author-starts-a-session-with-the-universal-dialog-obligation/transcript.txt`, `arc42-enabled-review-instructions-require-conformance-evidence/transcript.txt`, `generated-guidance-points-to-the-canonical-e2e-policy/transcript.txt` | yes |

| Confirmed Persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | repair a blocked archive or commit and see the actual result without unexpected extra work | none; the archive/commit transcripts show explicit success/failure and the preserved-work assertions | no action |
| agent--technical-role | receive stable counts/status and explicit actions | none; `check tasks` reports `333 scenario(s), 0 failed, 0 skipped`, and bare check explains that it executed nothing | no action |
| givn-maintainer--owner | see truthful scope, retained history, and no hidden destructive step | none; transcripts show explicit paths, preserved user prose, and a proven/blocked archive state rather than a silent rewrite | no action |

VISUAL-REVIEW: PASS

## Retirements

RETIRE: outline-review-gate — blocking outline enforcement is removed; the advisory `outline` report, its `assess-code-structure` owner, and the no-verdict/no-waiver behavior are retained. The delta capability has zero scenarios, its tests and gate module are removed, and ADR-028 records the retirement.

## Arc42 implementation conformance

The twelve affected chapters under `docs/arc42/` were last updated in `bf248e84`; `arc42.md` claims every chapter affected. The following table compares those claims with `design.md`, `tasks.md`, and the completed implementation.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Introduction/goals | `docs/arc42/01-introduction-and-goals.md` | concise trustworthy guidance, retained modularity/Personas, lean fix-forward | lines 7-21 authority/decisions | batch `guidance-acceptance`, `full-suite-reconciliation` | README and served instructions current | yes |
| Constraints | `docs/arc42/02-architecture-constraints.md` | explicit-path commits, skills-only, configuration authority, 1.0 baseline | lines 74-105 | `6fc93b50`, `60e70b13`, `8d576d01` | `src/config/overlay.rs`, `src/skills/targets.rs`, `src/cli/commit.rs` | yes |
| Context/scope | `docs/arc42/03-context-and-scope.md` | agent skill entrypoints, retained Git/Fossil contracts, advisory structure | lines 82-93, 106-113 | `8d576d01`, `3bbf3cc0` | `src/skills/targets.rs`, `src/scm.rs`, `src/cli/outline.rs` | yes |
| Solution strategy | `docs/arc42/04-solution-strategy.md` | shared witness/content/owner contracts, no generic engine | lines 22-45 | `64ba38e7`, `2219d02b` | `src/proof.rs`, `src/spec/owner.rs` | yes |
| Building blocks | `docs/arc42/05-building-block-view.md` | remove command generation/outline gate; focused parsing/ownership/witness | lines 24-45, 106-113 | `3bbf3cc0`, `64ba38e7` | `src/outline/`, `src/proof.rs`, `src/gherkin/` | yes |
| Runtime | `docs/arc42/06-runtime-view.md` | fresh verification, persisted recovery, noncircular review, bounded readiness/recording, selected commits | lines 54-73 | `c6d33045`, `64ba38e7`, `595abe2a` | `src/proof.rs`, `src/cli/archive.rs`, `src/cli/run.rs` | yes |
| Deployment | `docs/arc42/07-deployment-view.md` | skills-only target inventory, no new deployed service | lines 82-87 | `8d576d01`, `60e70b13` | generated skills, no command files | yes |
| Crosscutting | `docs/arc42/08-crosscutting-concepts.md` | canonical authority, config/defaults, tests-first batches, scope applicability, marker parsing, recording limits | lines 68-105 | `a7e9f30b`, `595abe2a`, `7788ec80` | `src/state.rs`, `src/config/overlay.rs`, `src/integrity_hook.rs` | yes |
| Architecture decisions | `docs/arc42/09-architecture-decisions.md` | amend ADR-010/013/023/028/070/071/072/074, retain history | lines 7-21, 310-313 | `bf248e84` | ADR amendments present with prepare-0-9-10 notes | yes |
| Quality requirements | `docs/arc42/10-quality-requirements.md` | fresh stable receipts, safe writes/timeouts, correct diagnostics, advisory metrics, no review loops | lines 54-73 | `c6d33045`, `7788ec80` | proof/readme/run/outline code and tests | yes |
| Risks and debt | `docs/arc42/11-risks-and-technical-debt.md` | one concrete post-1.0 backlog with locations and reproduce reasons | lines 114-119 | closeout task `tasks.md:760` | R-/D- entries present | yes |
| Glossary | `docs/arc42/12-glossary.md` | canonical vocabulary for behavior batch, existing behavior protected, advisory outline, challenge, targeted closure | lines 32-39 | `a7e9f30b` | terms present in glossary; specs/design reuse them | yes |

- [x] Every chapter marked affected by either the review or `arc42.md` was opened and checked for stale, contradictory, or placeholder content.
- [x] Every Arc42 fact or architecture decision relevant to this change is represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains in this Arc42 comparison. F-01 was a design/step-table conformance defect outside the Arc42 chapters; it is closed in `7bef6352` and recorded as PR-07 in design-review.

ARC42 CONFORMANCE: CLEAN

## Coverage

Measured evidence status: this change has not been archived, so no archive-scope coverage receipt exists for it. The durable merged report currently published in README is the last measured project report: line coverage 79% (16678/21177), branch coverage n/a (0/0) with the documented Cobertura parser limitation (branch identity is not retained by the merge script). Static inspection of the changed modules found no uncovered changed behavior and no dead production path left behind; the archive will record the authoritative merged measurement.

```
README merged coverage: 79% line (16678/21177), branch n/a (0/0)
Change archive receipt: absent — unproven candidate
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| Changed archive/proof/Gherkin behavior | `src/proof.rs`, `src/cli/archive.rs`, `src/gherkin/merge.rs`, `src/gherkin/parse.rs` | no gap from inspection; exercised by the 333-scenario change receipt and the archive delta scenarios | none; archive will measure |
| Changed verification/status/review behavior | `src/cli/check.rs`, `src/cli/status.rs`, `src/state.rs`, `src/overlap` | no gap from inspection; exercised by delta scenarios and 444 unit tests | none |
| Changed configuration/skills/commit behavior | `src/config`, `src/skills`, `src/cli/commit.rs`, `src/git.rs`, `src/fossil.rs`, `src/hooks.rs` | no gap from inspection; exercised by delta scenarios and unit tests | none |
| Removed outline gate and generated-command paths | `src/outline/gate.rs`, generated command assets | dead code removed; no uncovered live path | none |
| Branch coverage | project report | n/a denominator documented by the merge script; not a removed branch instrumentation | none |

No coverage gap was excused outside the three buckets. No archive measurement has been fabricated to replace the missing receipt.

## Scenario execution evidence

Review did not re-run the suite. The change-scoped receipt is:

```text
change: prepare-0-9-10
status: passed
head: 56519c8aec1f50e39a51a007b00c9285c404ff90
features_path: givn/changes/prepare-0-9-10/specs
command: ./run-tests.sh --non-e2e
command_digest: 917f6703608df37a10c1669eed1e777a674fb366751a173bb0e097edf9583767
features_digest: 6fac139f7a118954cc2543b6376bf5d793082ed39bd617017e824e9ba8eebfb7
tested_content_digest: bba63eee0b5cb741d1368491a5a0ccd70c3ae78c26b488cf2679388ca49d4f0c
content_scope: change-proof/v1
runner_exit: 0
result: {"scope":"change","total":333,"passed":333,"failed":0,"skipped":0}
```

`givn check review` reused this receipt across the later documentation-only revisions `e31f5196` and `08a34df8`, and across this targeted closure round, which changes only `review.md` and `design-review.md`. No archive receipt exists yet: receipt status `unproven candidate`. The 30 `@e2e` scenarios have their committed terminal transcripts and the `tasks.md` GREEN record; no per-scope archive E2E receipt is claimed.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The SCM-neutral commit path attributes a Git revision | The SCM-neutral commit path attributes a Fossil check-in | variant |
| An unclean project is protected from automatic migration commits | A Fossil project with uncommitted changes is protected from automatic migration commits | variant |
| The SCM-neutral commit path attributes a Git revision | The SCM-neutral commit path attributes a Fossil check-in | boundary |
| An unknown addon name in a conditional fails the synchronization and names the template | An unbalanced conditional in a skill template fails the synchronization | variant |
| An unbalanced conditional in a command template fails the synchronization | An unbalanced conditional in a skill template fails the synchronization | variant |

The Git/Fossil pairs are backend variants of the same requirement. The cross-file pair is a boundary between the retained permanent scenario and the delta projection of the same scenario. The conditional pairs are distinct failure modes (unknown addon vs unbalanced command/skill template). None is a duplicate merge candidate. The current gate reports no undisposed delta shape match and no long-scenario finding.

## Split-or-keep

None required. `givn check review` and `givn lint --change prepare-0-9-10 --json` report no `long_scenario` finding for the delta specs; no split-or-keep row applies.

## README impact

The change edits the public face: skills-only invocation, explicit-path `givn commit`, the 1.0 migration baseline, and the current configuration/readme/coverage descriptions. The README sections below were actually edited in `bf248e84`, not merely declared:

README-IMPACT: updated - What it is, Install, Quick start, Agent targets and commit attribution, Documentation, License, Coverage

## Sign-off

- [x] Fabrication audit: F-01's promised step-definition files are corrected to real, registered, populated files (`7bef6352`); all other checks (0 empty behavior-step bodies, reachable batch commits, behavior components, real-interface E2E, no HTTP/fetch substitution, no over-produced E2E) pass.
- [x] Scenario execution evidence: change-scoped regular receipt 333/0/0 at `56519c8a`, reused across documentation-only revisions; E2E evidence is 30 title-bound transcripts; no archive receipt exists yet and none is claimed.
- [x] Every coverage gap is classified from the latest measured evidence plus static inspection; the archive measurement is not fabricated.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail beyond observable CLI/file outcomes.
- [x] The local run command (`cargo run -- instructions`) starts the declared product surface; the project has no server/database/digital-twin stack.
- [x] The exact files `verify.e2e_command` invokes were identified; the tree (including untracked files) was searched for a parallel E2E implementation; none exists.
- [x] E2E scope is explicit (`--non-e2e` / `--e2e`, with `GIVN_RESULT_SCOPE` preserved by `run-tests.sh`); the regular receipt has zero failures/skips.
- [x] Implementation (commands, file layout, framework/driver) matches `design.md` as reviewed: commands/driver/evidence match, and the Step Definitions table matches the built files after `7bef6352` (F-01 closed).
- [x] Interaction coverage verified: all 74 delta inventory rows map to matrix rows, `@e2e` scenarios and CLI drivers; the seven retained matrix projections are the PR-06 design decision. The step-file mismatch is F-01, closed in `7bef6352`.
- [x] Design.md's `## Internal Primitives` section: the current design has no such section; the changed primitives (`VerificationInputs`, `ScopeEvidence`, `Receipt::reusable_scope`) are named in the Stable execution witness section and used by the implementation. No omission.
- [x] arc42 slug mapping verified: all twelve slugs in `arc42.md` resolve to real `docs/arc42/<slug>.md` files.
- [x] No finding is excused with a classification outside the three buckets; the remaining risks are recorded as risks, not as a fourth coverage bucket.

REVIEW: PASS
