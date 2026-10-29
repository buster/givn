# Design review: prepare-0-9-10

Review kind: independent
Author: maki-author
Reviewer context: preparation-reviewer

## Scope and method

One substantive independent artifact review followed by targeted closure of its six findings, not a renewed search for minor improvements. The reviewer read proposal, design, architecture assessment, all delta features/narratives and relevant permanent contracts/source. No production tests or implementation sign-off is claimed. The same recorded reviewer identity owns final review; returned context summaries preserve its prior findings for tool sessions that do not retain live agent state.

The operator's completed scope discussion governs deliberate changes to current rules: genuine RED evidence, skills-only, advisory outline/shape, retained scenario-length decisions, modular architecture/Personas/arc42 baseline, trusted lean recovery, explicit commits, manual0.x transition, no privacy/rollback/extra release policy. No settled preference was re-asked. Discoverable technical gaps were researched and repaired by the author.

## Findings and closure

| Finding | Initial defect | Author hardening | Closure |
|---|---|---|---|
| PR-01 | Legacy release test required a nonempty catalog; missing archive narrative contradicted N/A/reuse | Modify release-owned scenario/owner narrative, empty1.0 baseline catalog with synthetic future routes; full archive owner replacement covers N/A and lean recovery | PASS — preparation-reviewer |
| PR-02 | Scope-specific reuse lacked a concrete exclusive-input rule | Shared content/canonical other-settings digest; per-scope command digests. Only exclusive command repair preserves the other scope; shared changes invalidate both | PASS — preparation-reviewer |
| PR-03 | Nonexistent selection rejected tracked deletions; custom isolated-index transaction unspecified | Accept tracked deletions, native git commit --only, preflight conflicts and native locking, preserve unrelated index/worktree; tests for selected deletion and staged neighbors | PASS — preparation-reviewer |
| PR-04 | Step table literal paths disagreed with prose exceptions | Exact registered durable paths in table, genuinely new files explicitly planned | PASS — preparation-reviewer |
| PR-05 | Conflicting Background/non-English/N/A disagreement safety not executable | Add pre-mutation refusal and contradictory applicability scenarios; English-only publication is explicit | PASS — preparation-reviewer |
| PR-06 | Matrix omitted retained actions and demoted archive inventory mismatch | Full owner/fragment matrix plus projected retained E2Es, normalize archive variants; final closure includes missing reporter and OpenCode/Claude/OMP rows | PASS — preparation-reviewer |
| PR-07 | Step Definitions table named four files that do not exist (`refresh_generated_guidance_steps.rs`, `review_architecture_documentation_steps.rs`, `report_change_status_steps.rs`, `check_specification_health_steps.rs`); found by independent final review as F-01, after the design review was signed off | Table corrected in `7bef6352` to the durable binding files (`skill_rewrites_steps.rs`, `arc42_review_alignment_steps.rs`, `run_command_steps.rs`, `spec_overlap_gates_steps.rs`); same commit removes the harness-name example from the outline help (final-review F-02) | PASS — preparation-reviewer, targeted closure |

Final targeted response: DESIGN-REVIEW PASS. PR-01–PR-05 passed first closure; PR-06 required one bounded completeness correction and then passed. No additional broad audit requested. PR-07 is a post-sign-off path correction surfaced by the independent final review and closed in a targeted re-check, not a new review round.

## Review branch dispositions

- Scope: approved behavior matched; retained release contract and archive narrative repaired in PR-01.
- Technology: existing locked Rust stack and focused helpers appropriate; PR-02/03 choose concrete conservative mechanisms, not frameworks.
- Missing scenarios/testability: PR-05 repaired; meaningful RED commands, fail_on_skipped and failing stubs named; undefined bindings do not count as behavioral RED.
- E2E fidelity/interactions: actual subprocess/PTY/harness drivers and terminal evidence; no changed browser interface; PR-06 closes full retained inventory. Reclassifications are justified action variants, not coverage bypass.
- Risk: incorrect scope evidence reuse mitigated by explicit command-only exclusivity and conservative shared invalidation.
- Use cases/Personas: existing identities, relationships and stakeholder lenses retained; no invented confirmation.
- Decision ledger/timing: no unresolved human decisions; technical reuse/publication gaps resolved before tasks.
- Contracts/components: narrow existing module APIs, explicit external seams and separate composition fit the retained architecture; final review must check actual dependencies.
- Visual classification: existing terminal conventions, no new rendered browser screen; transcript assertions fit; no artificial browser driver required.
- Deferrals: post1.0 matching/outline/recording work tracked, no silent waiver of retained requirements.
- ADRs: existing amendment targets identified; no duplicate ADR or worksheet. Durable text follows implemented facts.
- Arc42: reviewer independently found all twelve chapters affected, matching assessment. Existing chapters substantive; STATUS: PARTIAL truthfully defers future-state edits until implementation, not fabricated completion.
- Vocabulary: agreed terms explicit in assessment; glossary reconciliation is implementation work.

## Remaining phase boundary

Tasks may now be authored. Architecture assessment remains partial until implementation/documentation reconciliation; this review does not mark it DONE. No feature is implemented or green merely because its specification parses.

## Targeted closure after final review

`design.md` changed after the initial sign-off. Commit `7bef6352` replaced four Step Definitions rows that named planned-but-never-created paths with the files that actually hold the bindings, and replaced the shipped outline help example `--path catopia` with `--path src` (`src/cli/outline.rs:26`).

Verification of the corrected rows: all four named files exist, are registered in `tests/steps/mod.rs`, and contain the capability bindings; `givn check review --change prepare-0-9-10` reports no promised-file failure. The correction touches one table and one help string — no decision, matrix row, use case, or scenario changed, so PR-01–PR-06 conclusions stand unchanged. Retained as a non-blocking accuracy note for the final review: the `enable-commit-attribution` row still points at `tests/steps/llm_attribution_installation_steps.rs`, a registered shared fixture module with zero bindings; that capability's delta scenarios are all `@e2e` and bind in `llm_attribution_installation_e2e_steps.rs`, so no behavior is unbound and no gate fails.

## Sign-off

DESIGN-REVIEW: PASS
