# Design review: drop-embedding-features

Grilling agent walked all required branches against the full plan and the
codebase; hardening applied the mechanical findings. Codebase-answerable
findings were resolved by exploration and are not user questions.

## Findings and dispositions

| # | Branch | Finding | Disposition |
|---|---|---|---|
| 1 | Scope | `SpecCommands::BgeParity` (hidden) and `run_bge_parity` call the engine unconditionally; missing from the removal inventory. | Added to design.md removal inventory (with `Review`). |
| 2 | Scope | `spec review` / `run_review` missing from the inventory; its non-feature stub would keep a permanently broken command. | Added to design.md removal inventory; command deleted with the engine. |
| 3 | Missing scenarios | Live permanent scenario "Retrieval guidance distinguishes build capabilities" (fragments/instruction-integrity.feature) requires engine guidance wording; generated-guidance assets (`assets/skills/givn-spec/SKILL.md.tmpl`, `assets/commands/givn-spec.md`) missing from the inventory. | New `@givn.modified` delta scenario pinning guidance absence; asset files added to design.md inventory. |
| 4 | Test surface | Residual engine wiring outside the retired capability's steps: "semantic gate is offline" step sets retrieval world fields; `cfg!(feature = "spec-retrieval")` + `GIVN_SPEC_EMBEDDER` in review-disposition steps; `env_remove("GIVN_SPEC_MAX_RERANK_PAIRS")` residue; orphaned Then-steps in release_0_3_0_steps. | All added to design.md test inventory with per-file dispositions. |
| 5 | Testability | RED is mechanically sound (feature metadata, undefined steps, changed error text). No spec pins env inertness — meaningless post-removal; report distinctness stays pinned by the coverage-script-workflow scenarios and the assert-distinct gate. | Accepted gap, recorded. |
| 6 | Risk | Most likely failure: compiler-green but suite-red from missed live references in tests/assets/specs. | Completion checklist added to design.md risks (grep sweep + full suite run before archive). |
| 7 | ADR qualification | ADR-047 (semantic remediation vs projected tree) omitted from ADR-069's supersession list while its subject implementation is deleted. | ADR-047 superseded by ADR-069, moved to `adr/archive/`, register updated. No further qualifying design choice; `must_be_shared` SUPPORTING is correct. |
| 8 | arc42 integrity | All 12 rows independently re-derived as Yes, matching arc42.md; chapter files present, no ASCII-art diagrams; edits are working-tree changes to land with the change's commits. | Verified; doc edits land with this change's commits; stale design.md glossary row reworded. |
| — | Scope | Spec contains exactly the proposal's scope; "done" is unambiguous (single binary, one suite, tombstone, deterministic contracts unchanged). | no issue found |
| — | Tech choices | Stack unchanged; removals only; sha2 retention verified. | no issue found |
| — | Use-case/Persona context | Routing deviations documented with rationale; no Personas invented; Actors in Gherkin unchanged. | no issue found |

## Hardening edits applied

- design.md: removal inventory extended (BgeParity, Review, guidance assets,
  test-surface residue rows), glossary row reworded, completion checklist
  added to risks.
- specs/fragments/instruction-integrity.feature: added `@givn.modified`
  scenario (guidance asserts absence of engine paths).
- adr: ADR-047 superseded and archived; ADR-069 supersession list and
  register updated.

DESIGN-REVIEW: PASS
