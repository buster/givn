# Changelog

All notable changes to Givn are documented in this file.

The release sections are generated with [git-cliff](https://git-cliff.org/).
Versions are selected manually and use annotated `vX.Y.Z` Git tags.
## [Unreleased]

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
- Added the LLM migration playbook at
  [`docs/migrations/0.2.0-to-0.3.0.md`](docs/migrations/0.2.0-to-0.3.0.md).
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

[0.8.3]: https://github.com/buster/givn/compare/v0.8.0...v0.8.3

[0.8.4]: https://github.com/buster/givn/compare/v0.8.3...v0.8.4

[0.8.5]: https://github.com/buster/givn/compare/v0.8.4...v0.8.5

[0.9.0]: https://github.com/buster/givn/compare/v0.8.5...v0.9.0
