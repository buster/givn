# Changelog

All notable changes to Givn are documented in this file.

The release sections are generated with [git-cliff](https://git-cliff.org/).
Versions are selected manually and use annotated `vX.Y.Z` Git tags.
## [0.9.10] - Unreleased

### Breaking Changes

- **guidance:** generated agent command files are removed. Givn now emits
  skills only: Claude Code and OpenCode use their skill invocation, the neutral
  `.agents` target uses skill discovery, OMP uses `/skill:givn-<step>`, and
  Maki uses an explicit request or an agent-selected skill. `givn skills sync`
  and `givn upgrade` remove **recognizable Givn-generated** command files from
  the selected targets; custom or modified command files are preserved and
  reported for manual transition. No replacement `/givn-*` command files are
  generated.
- **commit:** `givn commit` now requires explicit paths. Invoking it without
  paths fails before staging. Only the named paths are committed; unrelated
  staged and unstaged work is preserved on Git and Fossil.
- **configuration:** `givn/commands.yaml` now accepts only command/runtime
  settings (`verify`, `coverage`, and `run`). Addon, artifact, apply, and other
  policy configuration must live in `givn/config.yaml`; misplaced keys are
  rejected with a file-ownership diagnostic instead of being silently applied.
- **addons:** `absolute_mode` is renamed to `communication_policy`, and
  `grill_me` plus `grill_with_docs` are consolidated into `challenge`
  (`givn-challenge`). There are no compatibility aliases; retired keys are
  rejected with manual-transition guidance.
- **migration:** 1.0 is the supported migration baseline. Embedded 0.7/0.8
  migration bundles and their aggregate routes are removed. Pre-1.0 projects
  may need manual configuration repair; Givn does not silently convert old
  configuration or apply a compatibility alias. Future migration routes remain
  supported and are validated through disposable synthetic fixtures.
- **review checks:** the outline review is advisory, not a blocking verdict
  gate. Per-unit `OUTLINE: PASS/FAIL`, metric waivers, and forced outline
  reviewer context are removed. Shape matches are advisory and no longer
  require disposition rows. Long scenarios still require an explicit
  `split`/`keep` decision with a reason; a promised split must be completed.
- **commands:** bare `givn check` no longer runs configured artifact hooks or
  implies verification. It prints the explicit check commands and executes
  nothing.

### Features

- **verification:** change verification removes stale runner output, snapshots
  the revision and relevant inputs before execution, requires fresh valid
  counts, and checks input stability afterward. A successful no-op runner can no
  longer reuse old results.
- **archive:** successful regular/E2E scope evidence is persisted and reused on
  repair when its relevant inputs are unchanged. Evidence-only revisions do not
  rerun suites; a changed verify command invalidates only its scope, while shared
  source/content changes conservatively invalidate both. Existing broad
  archive-evidence exemptions remain documented.
- **no-behavior-change changes:** an explicit `Behavior change: none`
  declaration with skipped specs records changed-scenario verification as not
  applicable. It does not invent scenarios or substitute an automatic
  full-suite run; archive still verifies the merged candidate.
- **Gherkin:** supported publication preserves Feature Background, Scenario
  Outline Examples, docstrings, tables, inherited execution tags, and moving
  specifications. Rule blocks and non-English dialects are rejected before
  mutation with actionable diagnostics instead of being silently changed.
- **content comparison:** preservation and duplicate audits include supported
  executable arguments and inherited setup while remaining path-independent.
  `exact_duplicate` means literal supported executable equality; masked
  structural similarity is reported separately as advisory.
- **specification ownership:** review and archive share capability-owner
  resolution for use cases and fragments, including `NEW`/`EXTEND` and ambiguity
  refusal. Typed `usecase:`/`fragment:` relationships resolve only in their
  declared namespace, and interaction evidence is keyed by capability plus
  title.
- **verification output:** independent cheap validation failures are reported
  together; dependent checks are marked not performed after a prerequisite
  failure. `givn status` distinguishes artifacts complete, verification
  current/stale/not checked, pending archive candidates, and proven archives
  without executing tests.
- **commits:** explicit-path Git publication preserves unrelated staged work,
  supports tracked deletions, refuses unmerged index states, and reports the
  resulting revision. Optional attribution installation is repeatable with
  `core.hooksPath=.githooks`, preserves user hooks, and cannot self-dispatch.
- **runtime:** readiness waits use one total deadline across connect, write,
  and read, so a server that accepts but never responds cannot hang the start
  command. Reporter helpers have bounded waits and cannot block primary agent
  work.
- **configuration:** addon defaults and partial blocks resolve consistently;
  repeated initialization preserves explicit opt-outs; addon edits update only
  the intended setting or refuse unsupported YAML before writing. Required
  skill-generation failures produce a nonzero, accurate partial-initialization
  report.
- **guidance:** bootstrap owns loading the dialog policy once per context; a
  concise policy indexes preserved real-life examples by question type. Skills
  are thin directions to canonical instructions and no longer repeat dialog,
  delegation, review-loop, or commit prescriptions.
- **challenge and communication:** new `givn-challenge` reviews proposals,
  tests, specs, design, code, or complete changes by investigating first,
  citing available sources, labeling hypotheses, and asking only unresolved
  human decisions. `communication_policy` is presentation-only; mandatory
  dialog correctness is independent of it.
- **outline:** the report remains available as advisory structural evidence
  for Rust and Python. Unsupported-only scopes distinguish “no supported
  production sources” from “no production sources”; unavailable measurements
  render as unavailable/`null`, not zero; unmeasured LCC is omitted.
- **coverage and E2E:** equal coverage from honest runs no longer blocks proof;
  reports must be fresh and valid. E2E scope is declared explicitly rather than
  inferred from command strings or relative scenario counts. Justified
  reclassification is allowed while required real-interface coverage remains.
- **architecture:** modularity, narrow consumer contracts, adapters, and
  explicit dependency direction are required from the first implementation.
  The first completed arc42 change establishes all twelve proportionate
  chapters; later changes maintain affected chapters. ADR qualification is
  concise and recorded once.
- **documentation:** README, workflow/reference guides, migration guidance,
  ADRs, glossary, and the twelve arc42 chapters are reconciled to current
  behavior. Post-1.0 work is tracked with concrete evidence instead of
  duplicated operational rules.

### Bug Fixes

- Reject stale change results and receipts bound to inputs modified after
  testing.
- Keep archive proof valid through rejected finalization publications and
  evidence-only repairs without unnecessary suite runs.
- Preserve supported Gherkin instead of dropping Backgrounds or misreading
  docstrings as structure; refuse conflicting Background replacement and
  unsupported dialect before mutation.
- Fix UTF-8-unsafe malformed use-case headings so invalid documents produce a
  diagnostic instead of a panic.
- Fix Git hook self-dispatch and preserve user hooks while keeping optional
  attribution non-blocking.
- Refuse ambiguous README managed markers before writing; preserve prose
  outside managed blocks.
- Parse actual verdicts instead of matching arbitrary substrings or examples
  in comments and fenced blocks.
- Correct stale agent-target, default-addon, coverage, attribution, commit, and
  migration examples; remove Catopia/harness leakage from current shipped help
  and guidance while preserving historical evidence.
- Remove the retired outline gate, coverage equality gate, shape-disposition
  paperwork, generated command surface, obsolete coverage template, and
  embedded 0.x migration bundles.
- Make validation diagnostics aggregate independent failures and name the
  owning configuration file.

### Chores

- Compile the binary through the library module tree, eliminating duplicate
  module compilation and dead-code warnings.
- Ignore machine-local Maki permission state and exclude local recordings and
  harness fixtures from release distribution.
- Resynchronize all selected agent skills from the final canonical templates.
- Add executable acceptance coverage for the changed behavior: 333 changed
  scenarios/1,997 steps in the change suite, 30 E2E scenarios with individual
  transcripts, and 444 library tests; strict Clippy is clean.

## [0.9.9] - 2026-10-29

### Breaking Changes

- **review context:** a change now runs in one context by default. A fresh
  independent review context is used only for a major change or an explicit
  user request, and the recorded `Review kind` is authoritative. Projects that
  relied on every review starting a fresh subagent must request independent
  review explicitly.
- **addons:** every add-on is enabled by default. Existing projects that
  relied on add-ons being opt-in may gain behavior when reinitializing or
  synchronizing. `givn init --no-addons` writes an explicit `false` for every
  known add-on; a partial `addons:` block changes only the named keys.

### Features

- **review:** cap the review at one independent context, record the review
  context, and make the outline gate use the recorded self/independent context.
  Prescriptive subagent orchestration is removed from generated guidance; the
  review instruction is its sole owner.
- **addons:** enable all add-ons by default, report the effective default from
  `givn addons list`, and let `givn init --no-addons` produce an explicit
  opt-out for every known add-on.

### Bug Fixes

- **spec routing:** name the missing `NEW in <usecase-id>` or
  `EXTEND <capability>` decision so an archive failure explains the absent
  routing record.
- **guidance:** remove the unconditional fresh-delegated-context prescription
  from the generic overview; handoff guidance applies when work is actually
  handed to another context.

## [0.9.8] - 2026-10-26

### Features

- **review:** run the mechanical preflight before spawning one fresh review
  subagent; after fixes, check closure of the recorded finding IDs instead of
  another full audit. The reviewer writes only `review.md`.
- **coverage:** keep branch instrumentation; a line-only report is a finding.

### Bug Fixes

- **tasks:** remove `TEST-FIRST` and RED evidence fields. The design and tasks
  guide the implementer's test-first order; the task list records only the
  batch commit.
- **review:** stop auditing TDD order, test revisions, and reconstructed RED
  history. A high-risk discrimination proof remains an optional, risk-based
  review technique.
- **workflow:** remove test-first evidence claims from the workflow docs and
  task template.

## [0.9.7] - 2026-10-26

### Features

- **architecture:** define ports and adapters, require one component per
  contract, and split a unit that owns more than one contract, capability, or
  external adapter. A single-file implementation remains valid for one external
  dependency and simple business rules; complex changes use ports and adapters.
- **design-review:** review the plan and expected components, treat existing
  code as context only, and require the task list to wait for the review.
- **review:** require one fresh review subagent after implementation. It runs
  the outline, reads the metrics, applies the architecture check, and records a
  four-field waiver when it accepts a remaining metric finding.
- **ADR:** treat port/adapter boundaries, component splits, and durable storage
  mechanisms as candidates; a boundary decision is not local merely because it
  touches one component.

### Bug Fixes

- **guidance:** remove the deferred component split and the
  one-component-many-contracts escape hatch.

## [0.9.6] - 2026-10-26

### Features

- **outline:** `givn outline report` requires an explicit `--path` when `src/`
  is absent, and fails instead of returning an empty successful report when the
  scope contains no production sources. The review gate reuses the reviewed
  `OUTLINE-COMMAND` paths and refuses a change whose outline scope is missing.
- **specs:** scenario writing derives examples from the owning use case and its
  authoritative requirements. It removes the separate rule catalogue and
  rule-to-scenario ratio; duplication requires the same rule, path, assertion,
  and observable state, not merely the same outcome.

### Bug Fixes

- **guidance:** instruction assets no longer use benchmark-specific law
  vocabulary.

## [0.9.5] - 2026-10-26

### Features

- **workflow:** `givn archive` refuses a change whose design review is missing
  or unsigned, unless the artifact is skipped.
- **workflow:** `givn archive` re-runs the static review gates — integrity,
  overlap dispositions, retirement disclosure, capability placement, and the
  outline gate — before mutation, without running a test suite.
- **lint:** `givn check review --change` runs the change-scoped lint gate;
  parse errors, duplicate titles, empty features, invalid use-case documents,
  and interaction mismatches block review.
- **markers:** `REVIEW: PASS`, `DESIGN-REVIEW: PASS`, and `STATUS: DONE` count
  only as the final non-empty line outside fenced code blocks and HTML
  comments.
- **guidance:** the review check is named implementation evidence check, and
  the workflow documentation is split into an overview and
  `docs/workflow-details.md`.

## [0.9.2] - 2026-10-17

### Bug Fixes

- **development-report:** Align the no-network assertion with the design's inline script

- **development-timeline:** Reconcile the all-targets pin and document the maki target

- **development-report:** Reconcile the hybrid report's readings and lane-log facts

- **test-evidence:** Follow the change into the archive when capturing evidence


### Features

- **development-record:** Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

- **development-record:** Record an artifact's appearance as one phase transition

- **development-report:** Carry the hybrid workflow phase scaffold

- **development-report:** Show the empty state and draw no workflow phase

- **development-report:** Derive the current workflow state from a live change

- **development-report:** Lay parallel sessions side by side

- **development-report:** Mark a skipped workflow phase

- **development-report:** Mark every phase done for an archived change

- **development-report:** State a record with no stage transition

- **development-report:** Group repeated steps within a phase

- **development-report:** Collapse dense entries into phase aggregate ticks

- **development-report:** Present the workflow in the written report's text view

- **development-record:** An ideation artifact appearance is recorded as one phase start

- **development-report:** The report model carries the ideation workflow and a linked ideation group

- **development-report:** Resolve a change's ideation link from its proposal

- **development-report:** An ideation topic report uses the ideation workflow

- **development-report:** A change report shows its linked ideation group

- **development-report:** A linked ideation with no recorded activity is stated

- **development-report:** A change whose proposal names no ideation topic shows no linked group

- **development-report:** An archived change resolves its linked ideation from the archive

## [0.9.1] - 2026-10-15

### Bug Fixes

- **e2e-gate:** Attribute verify.e2e_command to givn/commands.yaml in error strings

- **spec:** Describe the spec subcommands instead of the removed retrieval index

- **instructions:** Point the duplicate rule at the overlap review

- **migration:** Cover source patch releases in the migration route


### Features

- **enforce-post-merge-e2e-gate:** Givn drops its stale residue — the retired history rewrite, the vestigial `extras` flag, the legacy coverage commands, and the pre-0.8 migration bundles and guides — so every documented claim and migration route matches what the tool actually does.

- **config:** Remove the dead extras manifest flag

- **coverage:** Remove the legacy two-command coverage schema and in-tool merge

- **attribution:** Stamp only the managed co-author line

## [0.9.0] - 2026-10-14

### Bug Fixes

- **generate-ideation-guidance:** Keep the served event-storming phrases contiguous across line wraps

- **instruction-integrity:** Point the reverse task breakdown at the served steps instruction and the single commit contract

- **instruction-integrity:** Keep the delegated-dialog obligation in the generated delegation prompts

- **instruction-integrity:** Serve the review conformance rules and document the served-rules model

- **extended-rules:** Repair the review-audit findings

- **guide-domain-modeling:** Restore the nine re-point scenarios lost to the repair write


### Features

- **prose-consolidation:** Givn serves every step's rules through `givn instructions` — installed skills and commands direct agents there instead of carrying rule copies — and a project can extend any served instruction with its own rules, which synchronization and upgrades never fight.

- **migrate-0-8-0-to-0-9-0:** Catalog the 0.8.0 to 0.9.0 route and ship the bundle

## [0.8.5] - 2026-10-12

### Bug Fixes

- **instruction-integrity:** Generated guidance only sends its reader to files the consumer project has or that a named creation step creates — the persona schema travels inline, reverse review is requested from Givn itself, and Givn rejects any guidance reference that resolves nowhere.


### Features

- **generate-ideation-guidance:** Givn asks the user to verify every newly coined domain term — failed names immediately, with the concept's description and concrete candidate names, the rest at the phase's term check — before any canvas, use case, or change seed relies on it.

- **generate-ideation-guidance:** Generated guidance only sends its reader to files the consumer project has or that a named creation step creates — the persona schema travels inline, reverse review is requested from Givn itself, and Givn rejects any guidance reference that resolves nowhere.

- **instruction-integrity:** Givn's guidance reference check now follows the production agent-target list, so a harness added to Givn is covered automatically and a configured target without synchronized guidance fails the check instead of passing silently.

## [0.8.4] - 2026-10-09

### Features

- **initialize-a-project:** Givn installs the workflow skills, the workflow commands, and a commit attribution hook for the omp coding agent when omp is a selected agent target.

- **license:** Relicense givn under GPL-3.0-or-later with an output exception

- **attribute-commits:** OpenCode 2 sessions attribute their commits to the model that produced them.

## [0.8.3] - 2026-10-05

### Bug Fixes

- **adr-qualification-gate:** Restore the blank ADR template from the 0.8.0 asset

- **adr-qualification-gate:** Retire the semantic-engine ADR residue

- **adr-qualification-gate:** Re-eject the impact assessment template from 0.8.0

- **confirm-persona-coverage:** Add the missing base hashes to two delta narratives

- **migration:** Preserve word spacing in the aggregate proposal wrapper

- **require-agent-target:** Name the agent selection in the nested Fossil fixture


### Features

- **migrate-0-7-0-to-0-8-0:** Catalog the 0.7.0 to 0.8.0 route and ship the bundle

- **migration-retrofit-plan:** Amend the 0.8.0 bundle and guide to the inventory-and-follow-up model

- **persona-identity-reuse:** Confirm persona coverage on five use cases

- **use-case-model-scoping:** Split author-change into a summary and two leaves

- **use-case-model-scoping:** Reshape archive-change to the 0.8.0 model

- **use-case-model-scoping:** Split maintain-project into a summary and three leaves

- **use-case-model-scoping:** Reshape verify-change to the 0.8.0 model

- **use-case-model-scoping:** Reshape operate-product to the 0.8.0 model

- **use-case-model-scoping:** Reshape inspect-change-state to the 0.8.0 model

- **use-case-model-scoping:** Reshape discover-specifications to the 0.8.0 model

- **use-case-model-scoping:** Anchor the make-usecases-canonical extension

- **use-case-model-scoping:** Rename the support-fragment capabilities

- **rules-policy:** Resolve the generated-rules policy

- **local-only-transition:** Keep tracked rules out of the repository

- **attribute-commits:** Stage project content only under local-only

- **apply-a-managed-upgrade:** Publish a policy-aware managed revision

- **publication-cleanliness:** Report tracked and historical rules

- **history-rewrite:** Purge generated rules from reachable history

- **initialize-a-project:** Mask generated rule names in the ignore block

- **initialize-a-project:** `givn init` refuses to initialize unless at least one agent target is selected, instead of silently installing OpenCode skills

- **initialize-a-project:** `givn init` keeps its interactive agent-target prompt open with OpenCode pre-selected and reports an error when the confirmation marks no target, instead of closing the dialog

- **nix:** Add flake with givn package and overlay

- **instruction-integrity:** Givn guides every user-facing question through one canonical dialog policy with brief sourced context, concrete answer consequences, material change costs, and multiple bad/good examples.

## [0.8.0] - 2026-09-17

### Added

- **Fossil support across the lifecycle**: backend selection from project
  markers, the archive write path (witness-bounded candidate and finalization
  revisions, resume republication after a rejected hook), the managed upgrade
  and migration plans, and `givn commit` — the SCM-neutral attributed commit
  path (raw `fossil commit` carries no attribution; the Git hook still stamps
  raw Git commits).
- `givn status` reports the active backend (`SCM: git`, `SCM: fossil`, or
  `SCM: none` with the unavailable note).
- ADR-072 (witness-bounded publication) and the ADR-071 amendment for the
  managed upgrade on both backends.
- `givn instructions dialog`: the canonical dialog policy for every
  interactive step — named artifacts, the four-part option treatment,
  cross-reference glosses, standalone answerability, rejected alternatives,
  one question at a time.
- Change seeds gain a content schema and a status lifecycle (`ready`,
  `proposed by`, `implemented in`, `superseded by`, `withdrawn`), with
  propose- and archive-time stamping and an advisory `[SEED ]` lint check.
- Term records gain a status/contrast/collision-check schema, the
  search-before-coin procedure, and the term introduction test.
- `docs/use-case-model.md`: the canonical use-case definition, the eight
  scoping rules, the authoring requirements, the ten review rejection
  criteria (R1-R10), and the worked before/after example.
- Persona identity criteria (authority boundary, evaluation capability, own
  vocabulary), a stable-core/running-record template, the reuse-first
  protocol, and persona reference validation (`[PERS ]` lint advisories and
  the visual `PERSONAS:` review gate).
- `(open: Q<N>)` guarantee qualifiers and the `D<n>` question IDs with the
  design's `Decides D<n>` decision ledger.
- Lint findings carry a severity (`advisory`, `finding`, `blocking`), and
  `givn spec route` reports explicit confidence with ownership-weighted
  scoring.

### Changed

- `givn init` installs attribution plumbing only where the backend supports
  it (Fossil reports `commit attribution: givn commit path only`), warns
  instead of refusing when no backend exists, and the generated guidance
  names the backend-valid commit path.
- Integrity and receipt vocabulary is backend-neutral ("revision
  identifier", "candidate/tested revision").
- The archive/upgrade scenarios and documentation no longer assume Git.
- Release 0.8.0 ships the `migrate-0-7-0-to-0-8-0` migration bundle for
  projects created with 0.7.0: it inventories existing use-case documents,
  personas, and ADRs against the hardened model, records every deviation with
  the follow-up change that repairs it, adopts the dialog policy, seed
  statuses, term records, and the decision ledger, and re-synchronizes
  ejected overrides. The managed refresh and `givn_config_version` bump remain
  with `givn upgrade`.

## [0.7.0] - 2026-09-10

### Changed

- Release 0.7.0 ships the `migrate-0-6-0-to-0-7-0` migration bundle for
  projects created with 0.6.0: it walks the project LLM through making each
  verification scope write the fail-closed `GIVN_RESULT_FILE`, confirming the
  project is a Git worktree rooted at the project root, and proving the first
  archive under the receipt model. The managed refresh and
  `givn_config_version` bump remain with `givn upgrade`.
- Archive publication is now forward-only and Git-backed: the merged archive
  candidate is committed before verification, each verification scope runs
  once with a fail-closed machine-readable result, and a verification receipt
  binds the proven archive to the tested commit. A failed candidate stays in
  place for forward repair and `givn archive` resumes it; the backup/rollback
  pipeline is removed. `givn archive` now requires a Git worktree whose root is
  the project root and a committed project state.
- `givn check review` no longer executes `verify.command` or
  `verify.e2e_command`; the archive verification receipt is the execution
  evidence, and the review sign-off is based on the fabrication audit, static
  checks, and the receipt.

## [0.4.0] - 2026-08-27

### Features

- **ci:** Add release, CI, and security pipelines

- **release:** Prepare 0.4.0 with a chained 0.3.0 to 0.4.0 migration

## [0.3.1] - 2026-08-25

### Added

- Added `givn spec search --evidence "<query>"`, an opt-in cross-encoder
  evidence surface that appends BGE and NLI verdicts (`BGE_RECOMMENDED`,
  `BGE_FILTERED`, `BGE_UNSCORED`, `BGE_TOKEN_CAP` and the NLI
  counterparts) to the existing E5 ranking without changing the flag-off
  output. Absent layers are reported as `BGE evidence: unavailable` /
  `NLI evidence: unavailable` and never as model verdicts. Candidates beyond
  the pair budget stay `BGE_UNSCORED`/`NLI_UNSCORED`; complete pairs over
  the model's 512-token input limit receive visible token-cap evidence.
- Added a configurable second-stage candidate budget
  (`GIVN_SPEC_MAX_RERANK_PAIRS`, default `2048`). The fixed pair-count
  refusal is gone: E5 candidates are ordered by descending E5 score and the
  configured prefix is scored by BGE and NLI independently, with the tail
  reported as explicitly unscored instead of as an unavailable model run.
  Invalid, empty, zero, negative, malformed, and overflowing setting values
  are rejected with an error naming the setting.
- Added the explicit ADR qualification gate: strict AND-semantics
  qualification guidance, canonical routing, a consolidated Arc42 ADR
  register, `ADR-063` kept proposed and owned by its active change, and
  comprehensive removal/absorption/supersession coverage audit.
- Added `docs/workflow.md`, the detailed process documentation: phases with
  artifacts and completion criteria, the scenario TDD loop semantics,
  advisory versus blocking gates, semantic duplicate assurance with model
  thresholds, status semantics, and step-by-step archive mechanics including
  rollback.
- Added `docs/reference.md` with the complete CLI command table, agent skill
  list, addons, extras, configuration key sections, generated project layout,
  and repository development scripts.
- Added `docs/workflow.html`, a self-contained offline HTML swimlane view of
  the change lifecycle, and `docs/workflow.svg`, a GitHub-safe SVG rendering
  sized for the README preview column, generated by
  `scripts/render-workflow-svg.py`.
- Registered the new documents in `docs/INDEX.md`.

### Changed

- The semantic classification vocabulary is now English: candidates are
  classified as `DUPLICATE`, `VARIANT`, or `VALID-BOUNDARY` (previously
  `DUPLIKAT` / `VARIANTE` / `VALID-BOUNDARY`). Renamed across the review
  gate validator and remediation guidance, embedded instructions, templates,
  skills, the executable specs, and the benchmark harness.
- The README is reduced to essentials: what the tool is, installation,
  quickstart, and an embedded lifecycle diagram. Comparison tables, core-idea
  essays, CLI reference, skills/addons/extras lists, configuration details,
  and project layout moved to `docs/reference.md`, `docs/workflow.md`, and
  the existing arc42 documentation.
- The package and CLI version are now `0.3.1`.
- The ADR migration-manifest and `ADR-063` ownership steps resolve both the
  active-change and the archived-change path so the permanent suite stays
  green after archiving the qualifying changes.

### Fixed

- The consolidated-ADR-register scenario reads the migration manifest from
  the archived change directory when the active directory no longer exists.

## [0.3.0] - 2026-08-23

### Added

- Added deterministic overlap analysis to `givn lint`, including duplicate
  title errors, structural shape/subset findings, long-scenario warnings,
  machine-readable JSON output, review dispositions, and net-delta receipts.
- Added optional local semantic retrieval with `givn spec index`, `search`,
  `explore`, `review`, and `duplicates` surfaces. Permanent index entries are
  reused only when their complete serialized content still matches.
- Added complete Gherkin semantic serialization, project-relative identities,
  active-change exploration, change-local semantic reports, projected
  remediation verification, and deterministic evidence hashes.
- Added optional BGE and NLI evidence paths with independent pools, explicit
  recommended/filtered/unscored/unavailable states, actual-tokenizer limits,
  and visible token-cap evidence. These paths never assign Givn classifications
  or edit feature files automatically.
- Added canonical black-box-first and E2E interaction guidance. Generated
  instructions now delegate normative interaction policy to one resolved source
  instead of maintaining competing copies in every skill.
- Added repository-wide acceptance fixtures and rollback coverage for review,
  archive, semantic evidence, and generated project ownership boundaries.
- Added strict ADR qualification guidance, canonical artifact routing, and a
  consolidated Arc42 ADR register that preserves independent architecture
  history without retaining non-ADR records.
- Added the LLM migration playbook at `docs/migrations/0.2.0-to-0.3.0.md`.
- Added the embedded migration catalog at `assets/migrations/catalog.yaml`.
  Future release-owned migration bundles can be chained and delivered as one
  ordered aggregate maintenance change.
- Added a normal Cargo installation that enables every declared retrieval,
  strict E5, BGE, and NLI feature; `--no-default-features` remains the explicit
  feature-free build.

### Changed

- `givn check review --change <id>` now runs the structural overlap gate and,
  in retrieval-enabled builds, the blocking semantic review gate.
- `givn archive` rechecks the blocking semantic gate immediately before
  modifying permanent specifications and refreshes the permanent retrieval
  index after a successful archive.
- Retrieval remains feature-gated and independently selectable. The normal
  Cargo installation enables all declared retrieval capabilities, while a
  feature-free binary reports explicit retrieval unavailability instead of
  silently passing a semantic gate.
- Generated review, design, task, and skill guidance now requires concrete
  evidence for strict runners, real-interface E2E tests, overlap decisions,
  semantic remediation, and architecture conformance.
- Arc42 documentation is consolidated into durable chapter files and a
  one-file-per-decision ADR register. `ADR-023` is refined by the 0.3.0
  migration protocol.
- The package and CLI version are now `0.3.0`.

### Migration

Projects receive an automatic, reviewable managed preparation when they run
`givn upgrade` in a clean Git worktree. If the embedded migration catalog
contains a path from their config version to the installed binary, Givn creates
a managed upgrade commit and a separate aggregate maintenance migration change
that records the first commit and contains every ordered bundle's LLM prompts.
If no migration route applies, Givn creates only the managed upgrade commit and
no migration change. The migration guide is the entry point:

```text
docs/migrations/0.2.0-to-0.3.0.md
```

The automatic preparation never changes domain specifications, source code,
ejected facets, durable Arc42 facts, or user-authored files. It refuses dirty
or non-Git projects before writing, force-stages only its explicit managed
allowlist when generated paths are ignored, and never stages the complete
worktree.

### Verification Boundary

The release includes the retrieval and second-stage evidence implementation,
but proposed `ADR-063` and the active `semantic-reranker-budget` change remain
explicitly incomplete until their own verification and review gates pass.
Release distribution binaries, Homebrew, and shell-installer channels remain
planned work.

## [0.2.0]

The master-branch baseline for the 0.3.0 migration. See the repository history
for the complete 0.2.0 feature set.

[0.9.9]: https://github.com/buster/givn/compare/v0.9.2...v0.9.9

[0.9.10]: https://github.com/buster/givn/compare/v0.9.9...v0.9.10

[0.8.3]: https://github.com/buster/givn/compare/v0.8.0...v0.8.3

[0.8.4]: https://github.com/buster/givn/compare/v0.8.3...v0.8.4

[0.8.5]: https://github.com/buster/givn/compare/v0.8.4...v0.8.5

[0.9.0]: https://github.com/buster/givn/compare/v0.8.5...v0.9.0

[0.9.1]: https://github.com/buster/givn/compare/v0.9.0...v0.9.1

[0.9.2]: https://github.com/buster/givn/compare/v0.9.1...v0.9.2
