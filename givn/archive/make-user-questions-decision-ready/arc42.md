# arc42 Documentation Update: make-user-questions-decision-ready

## Scope and evidence boundary

This assessment records the approved proposal and authored delta/design. Architecture updates describe the target as pending implementation; they do not assert that the new policy, startup overview, or generated references are deployed or tested. Completing this documentation artifact is not completing the feature.

Read inputs: proposal.md, all three delta feature files and three owning contract snapshots under specs/, design.md, the resolved arc42-docs instruction, all twelve durable chapter subjects, and the active/historical ADR register search. The policy and examples remain owned by the effective dialog instruction; chapters summarize ownership and verification boundaries rather than copying the full rules.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The approved informed-question target refines agent ergonomics and outcomes for the existing Developer, Authoring agent and Givn maintainer; priorities and product purpose remain unchanged. | Added a narrow pending stakeholder quality objective in chapter 01, linked to Q-93 through Q-95 in chapter 10. |
| 2 | 02 architecture-constraints | No | Preserve the feature-free binary, override precedence, owned-block and gate boundaries; introduce no runtime policy gate or command configuration. | Unchanged. |
| 3 | 03 context-and-scope | Yes | The dialog interface's explicit authoring scope expands to all user-facing questions and discovery contexts. | Refined the dialog interface row and added a pending universal-discovery scope note, explicitly excluding chat interception. |
| 4 | 04 solution-strategy | No | Reuse the existing embedded-asset cascade, template renderer and managed-block strategy; no new mechanism or boundary. | Unchanged. |
| 5 | 05 building-block-view | Yes | Existing instruction/template/project-hint owners acquire target-content obligations; no new production module is added. | Added a pending responsibility paragraph distinguishing policy ownership, delivery and semantic review. |
| 6 | 06 runtime-view | Yes | Startup and init/sync discovery plus explicit fresh-context delegation are part of the proposed flow. | Added pending unconditional project-block and generic-overview/delegation flow notes without claiming code execution has changed. |
| 7 | 07 deployment-view | Yes | New embedded content needs the rebuilt binary and normal sync; project-owned replacements must remain untouched. | Documented the existing-footprint target and update/override limitations; no new service or distribution channel. |
| 8 | 08 crosscutting-concepts | Yes | Universal dialog scope, thin derived references, addon independence and effective precedence refine normative guidance. | Updated sections 8.3a and 8.7 with pending scope/discovery, semantic-evidence boundary and unconditional managed project ownership. |
| 9 | 09 architecture-decisions | No | The full question contract is process guidance with an existing canonical owner; accepted cascade/integration/metadata/build boundaries remain intact. | No new ADR, amendment, register change or parallel decision record. See the non-qualification verdict below. |
| 10 | 10 quality-requirements | Yes | Distribution availability, override/preservation and informed question presentation need explicit acceptance targets. | Added Q-93 through Q-95 and separated deterministic delivery from fresh-context review signals; execution is pending. |
| 11 | 11 risks-and-technical-debt | Yes | Missing contexts, reference-without-compliance claims and stale replacement templates remain distinct risks. | Broadened R-67 and added R-73/R-74 with ownership and evidence mitigations. No new model-based enforcement is claimed. |
| 12 | 12 glossary | Yes | Refine the existing Dialog policy and proportional Question record semantics; qualify visual evidence by interface. | The pending dialog target remains explicit; actual decisions retain alternatives while short information requests invent none; Web UI screenshots/states are distinguished from terminal transcripts. |

Also updated `docs/arc42/README.md` Documentation Status to distinguish this approved target from current runtime evidence.

## Affected files

- `docs/arc42/01-introduction-and-goals.md`
- `docs/arc42/03-context-and-scope.md`
- `docs/arc42/05-building-block-view.md`
- `docs/arc42/06-runtime-view.md`
- `docs/arc42/07-deployment-view.md`
- `docs/arc42/08-crosscutting-concepts.md`
- `docs/arc42/10-quality-requirements.md`
- `docs/arc42/11-risks-and-technical-debt.md`
- `docs/arc42/12-glossary.md`
- `docs/arc42/README.md`

Only text, existing definition and assessment rows changed; existing Mermaid diagrams and unrelated architecture content remain unchanged.

## ADR qualification verdict

ADR action: CANONICAL_ARTIFACT
ADR target: assets/instructions/dialog.md

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "FAIL",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "assets/instructions/dialog.md",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Proposal D1 records full project-only policy, dialog-only discovery, canonical policy with universal references, full copies everywhere, and runtime interception; the operator selected universal references."
    ],
    "architectural_impact": [
      "No module, component, ownership precedence, JSON schema, deployment service, data boundary or runtime interception contract moves. Existing owners receive content obligations."
    ],
    "durable_consequence": [
      "The local distribution choice is revisable by one guidance/source change with normal rebuild/sync; it introduces no stored-data migration, new renderer protocol or public command shape."
    ],
    "lower_level_artifact": [
      "The complete rationale is correctly owned by the canonical process instruction; delivered behavior is specified by the existing instruction-integrity, prose-consolidation and initialize-a-project capabilities. The HOW belongs in design.md, not an ADR."
    ],
    "existing_adr_check": [
      "Read the chapter-09 and adr/README.md registers and searched docs/arc42/adr recursively, including historical storage, for dialog, canonical authoring, effective authoring, AGENTS.md and runtime include.",
      "ADR-005 retains facet precedence; ADR-013 retains selected-target and fenced-block ownership; ADR-048 retains effective policy/metadata and excludes an unnecessary runtime include mechanism; ADR-069 retains the model-free single-binary boundary. No amendment or supersession is needed."
    ]
  }
}
```

The process-policy classification fails the cheaper-home dimension and the product/workflow-behavior exclusion. Many source assets being touched is not evidence for a new component boundary or ADR. Shared author knowledge is supporting evidence only. No MADR file or duplicate rationale is added to the architecture registers.

## Verification and remaining lifecycle work

- The earlier author lint parsed three feature files with deliberate WIP findings; the final hardening lint and corrected counts are recorded in design-review.md, not inferred from the earlier run.
- The hardened design records the runner-owned removal exclusion required in both plain and nextest paths, mandatory tasks verification, exact targeted commands, normalized action matrix, scenario assertion/RED map, terminal reference and baseline, and unseen fresh-context review.
- Documentation completeness is assessed here; production implementation, strictness probes, scenario execution, final consumer transcripts, fresh-context behavior review, task commits and archive proof have not occurred.
- FreshPlanGrill independently recorded all twelve selections before opening this marker. Its original row 1 Yes/author No is corrected; its row 7 No/author Yes is a compatible broader content-adoption note through the unchanged deployment mechanism. The original provenance and full content check are preserved in design-review.md.

## Status

STATUS: DONE
