# Tasks: enforce-spec-interaction-integrity

## Setup

- [x] Projection substrate, fixture audit, and corpus repair
  - Design constraints: `spec::corpus` reads direct use-case/fragments documents and feature files into `SpecCorpus` maps keyed by project-relative paths including `givn/specs/`; a missing root is empty, a read failure is a named error, a nested feature/document is a recorded limitation. `spec::merge_plan::merge_feature_text` wraps `gherkin::merge::apply_feature_deltas` plus a new public `apply_feature_deltas_retry`; `spec::merge_plan::write_feature` is the thin read/write wrapper and preserves the `is_delta_feature` guard and atomic `.givn-tmp` rename. `spec::merge_plan` delegates narrative overlay and retirement to `SpecCorpus::overlay_narrative`/`SpecCorpus::remove_capability`; retirement mirrors `retire_capability` including the `fragments` exemption and absent-feature no-op. `MergeEntry` carries `target_rel` with the `givn/specs/` prefix; a target outside `givn/specs` is refused; `plan_from_receipt` recomputes it. `build_merge_plan` returns a typed `MergePlanError::UnplaceableCapability`.
  - Runner and strict proof: `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` use `tests/features_runner.rs` with `.fail_on_skipped()`. Prove strictness with one new `@wip` scenario under `GIVN_FEATURES=givn/changes/enforce-spec-interaction-integrity/specs`; confirm non-zero for the undefined step, record the command and output, then remove the probe. Do not change `verify.command`.
  - Fixture repair: audit every writer that emits a `## Interactions` row. Repair `archive_proof_support.rs`, `enforce_post_merge_e2e_gate_steps.rs`, `review_disposition_gates_steps.rs`, `deduplicate_change_dir_detection_steps.rs`, `fix_uppercase_checkbox_parsing_steps.rs`, `workflow_enforcement_steps.rs`, `extract_coverage_gate_steps.rs`, `readme_enforcement_steps.rs`, `grouped_merge_steps.rs`, `preserve_docstrings_on_archive_steps.rs`, `repository_ownership_fixtures_support.rs`, the persona narrative fixture, and `tests/common/mod.rs::write_fixture_usecase`, generating rows from actual `@e2e` titles (including `Scenario Outline:`) or `- none`. Add the no-document documents at `readme_enforcement_steps.rs:244-279`, `workflow_enforcement_steps.rs:910-916,1025-1063`, `enforce_post_merge_e2e_gate_steps.rs:47-59,209-221`, `preserve_docstrings_on_archive_steps.rs:99-110,180-206,267-284`, `grouped_merge_steps.rs:521-555`, and both `repository_ownership_fixtures_support.rs` directories. The persona repair declares `persona-change` and sets `## Interactions` to `- none` beside a non-`@e2e` feature; the last-capability retirement removes the `specs/only-cap/usecase.md` write. Keep the intentional `register-readiness` schema-error fixture as an exception. Add the `write_pending_receipt` merged-candidate fixture.
  - Corpus repair: the delta `specs/fragments/fragment.md` keeps base hash `a219ed71…78c22b` and the 19 added rows; the init delta adds `@e2e` to the existing scenario. No permanent spec is edited directly.
  - Unit tests: `SpecCorpus` read (missing root, unreadable, nested limitation), overlay, retirement (sibling, last-capability, fragments, already-applied), `target_rel` normalization/outside refusal/receipt recomputation/lookup, merge-text failure, legacy retry, and the projection's no-temp/no-write behavior.
  - [x] COMMIT: 3a33fb39 -- Batch: projection-substrate

## Batch: projected-lint

- [x] Scenario: Change-scoped lint reports an E2E scenario the projected inventory does not declare
  - Design constraints: `cli::lint::run_with_root` runs the projection for `--change` after `scan_tree` has set the parse/use-case flags; `SpecCorpus` projection calls `merge_feature_text`, then `interaction_gate_findings`. Exit `1`; the finding names `use case 'fragments'`, `capability 'place-order'`, and `unlisted @e2e scenario 'Consumer cancels an order'`. No tempdir and no file write. The fixture uses the permanent fragment with one declared interaction and a delta that adds an E2E scenario without a row.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Change-scoped lint reports a declared interaction whose projected corpus has no E2E scenario
  - Design constraints: the projection applies `@givn.removed` to the in-memory corpus; the inventory still declares the interaction. Exit `1`; the finding names the use case, capability, and `missing @e2e scenario 'Consumer submits an order'`. The merge text error path is not used for a valid removal.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Change-scoped lint reports a feature the projected inventory does not declare
  - Design constraints: the reverse check runs in `interaction_gate_findings(&SpecCorpus)`: every direct feature must be named in the document's `## Capabilities`; `- none` is an empty list. Exit `1`; `undeclared_capability` names `use case 'fragments'`, `undeclared capability 'cancellations'`, and the E2E title. A feature directory with no document is also undeclared.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Default lint reports an undeclared capability
  - Design constraints: default `scan_tree` builds the permanent `SpecCorpus` and calls the same gate; `undeclared_capability` is in `severity_for` and the `has_use_case_error` exit match. Exit `1`, not only a blocking severity. A corpus read error becomes a blocking finding; a parse error is a blocking `usecase_invalid`.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Change-scoped lint reports a delta capability with no projected owner
  - Design constraints: `build_merge_plan` returns `MergePlanError::UnplaceableCapability { capability, detail }`; the projection matches that variant and returns `interaction_mismatch: capability '<cap>' cannot be placed in the projected corpus: <detail>`. Exit `1`. The raw-delta fixture writes no `NEW in` routing. The `.feature file at` fixtures keep their routing and `write_fixture_usecase` `- none`.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Change-scoped lint accepts a delta that keeps the projected inventory consistent
  - Design constraints: the delta narrative declares the new interaction and the projected feature carries the matching `@e2e` title; the gate returns no finding. `givn lint --change` exits `0` and prints a clean summary. The positive guard must not over-block.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

- [x] Scenario: Change-scoped lint reports a pre-existing inventory mismatch
  - Design constraints: the projection deliberately does not subtract pre-existing findings; the permanent fragment already has an unlisted E2E scenario. A change with no delta features still projects and exits `1`, naming the unlisted scenario. The empty-feature early return must not preempt the projection.
  - [x] COMMIT: 3a33fb39 -- Batch: projected-lint

## Batch: review-projection

- [x] Scenario: Review refuses a projected interaction-inventory mismatch before archive
  - Design constraints: `givn check review` calls `lint::run_scoped`, which runs the same projection and bails on a non-zero lint code. The fixture is the `test-change` review baseline; the delta adds an unlisted E2E scenario to `demo-cap`. Exit non-zero; stderr contains the interaction title, `use case 'demo-cap'`, `capability 'demo-cap'`, and `unlisted @e2e scenario 'Consumer cancels an order'`. No archive candidate is created.
  - [x] COMMIT: 3a33fb39 -- Batch: review-projection

- [x] Scenario: Review accepts a delta whose projected interaction inventory is consistent
  - Design constraints: the delta adds the interaction row and the E2E scenario; the projection is clean and review reaches its normal completion checks. The fixture must be a complete valid review baseline; the positive guard must not over-block. The fixture uses `write_fixture_usecase`/`- none` only for new capabilities with no E2E titles.
  - [x] COMMIT: 3a33fb39 -- Batch: review-projection

## Batch: archive-projection

- [x] Scenario: Archive refuses a projected interaction-inventory mismatch before moving the change
  - Design constraints: fresh archive calls `projection::interaction_findings_for_plan(project_root, source_specs, plan, MergeProgress::empty())` after `build_merge_plan` and before `ensure_e2e_configured`, the pending receipt, and the rename. The finding is a blocking `interaction_mismatch`; exit non-zero; stderr names `use case 'demo-cap'`, `capability 'demo-cap'`, and the unlisted scenario; the change stays under `givn/changes/` and no archive candidate exists. The fixture commits and refreshes the receipt before the clean-state witness.
  - [x] COMMIT: 3a33fb39 -- Batch: archive-projection

- [x] Scenario: Archive publishes a consistent projected interaction inventory
  - Design constraints: the delta adds the interaction row and the E2E scenario; the projection is clean. The real merge uses `spec::merge_plan::write_feature`/`merge_feature_text` (same merge core) and the archive's existing publication and verification flow. Exit `0`; the change leaves `givn/changes/`.
  - [x] COMMIT: 3a33fb39 -- Batch: archive-projection

- [x] Scenario: Archive refuses a merged candidate whose interaction inventory is inconsistent
  - Design constraints: `write_pending_receipt` creates an unproven candidate with `merge_completed: true`, empty `scopes`, and empty `scope_evidence`; the merged corpus comes from `ready_change` + `copy_dir_all`. Before the verification scopes, `projection::permanent_interaction_findings(project_root)` reads the actual `SpecCorpus` and returns the mismatch. Exit non-zero; stderr names the use case and capability; the candidate remains unproven and repairable; no verification scope ran.
  - [x] COMMIT: 3a33fb39 -- Batch: archive-projection

## Closeout

- [x] Docs and full-suite closeout
  - Design constraints: update the lint exit-code, sentinel, glossary, workflow, and 8.24 docs; keep ADR-075 and the arc42 chapters in memory wording; no temporary-copy claim. Run `./run-tests.sh --non-e2e`, `./run-tests.sh --e2e`, `givn lint` on the permanent corpus after the archive merge, and `givn readme check`. Verify the merged step phrases are bound in the permanent tree before archive.
  - [x] COMMIT: 3a33fb39 -- Batch: docs-closeout
