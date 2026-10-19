# Proposal: review-preflight-and-cli-evidence

## Release note

Surface task-integrity and README-impact requirements before implementation, and preserve plain-text CLI interaction evidence without forcing a visual-review sign-off.

## Use-Case Context

- Use-case ID: verify-change, create-change
- Ideation topic: none — derived from a real evaluation run
- Seed path: none
- Confirmed Personas: none — operator confirmed; this affects operator and agent tooling; CLI transcripts are protocol evidence, not a rendered visual interface. The operator is an actor, not a Persona.

## Problem / Opportunity

Task revision rules and the README impact decision are discovered only at `check review`, after implementation and coverage work. In a real trial this caused 25 review failures, duplicate revision identifiers, and fabricated revision placeholders.

Visual classification is also discovered late. An early review carried `VISUAL-REVIEW: UNVERIFIED` for a machine-only CLI; the final design correctly declares `Interface: none`. The human visual sign-off gate is correct for rendered interfaces, but a CLI needs a clear not-applicable path.

Plain-text CLI transcripts are valuable evidence. Archive currently rejects transcript files under `givn/changes/<id>/evidence/` as undeclared tested-tree changes.

## Proposed Solution

Before implementation, a task-integrity preflight reports missing or duplicate revision slots, missing README impact decisions, and an unsupported one-pass task structure in one pass. The task template makes the revision choice explicit instead of forcing fabricated per-scenario hashes.

A machine-only product can record `VISUAL-REVIEW: NOT APPLICABLE` and keep plain-text interaction transcripts as evidence. No human visual sign-off is required for a CLI. A genuinely rendered interface still requires visual review and human sign-off.

Change evidence under `givn/changes/<id>/evidence/` is declared evidence for the tested-tree check and does not invalidate finalization.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| Visual verdict for non-visual interfaces | `EXTEND visual-review-gate` (verify-change, 15.00) | `EXTEND visual-review-gate` | The existing gate owns visual classification and the not-applicable path. |
| Task-integrity and README-impact preflight | `EXTEND givn-workflow-enforcement` (verify-change, 14.00) | `EXTEND givn-workflow-enforcement` | The preflight reuses the existing review/integrity rules but runs earlier. |
| Visual contract classification in change authoring | not ranked | `EXTEND visual-design-contract` | The design template must make `Interface: none` explicit for machine-only products. |

## Out of Scope

- Weakening the human visual sign-off for genuinely rendered interfaces.
- Making screenshots or browser automation mandatory for a CLI.
- Removing README impact tracking.
- Removing the requirement for real revision identifiers.
- Adding another test execution.

## Open Questions

### D1 — How should a CLI record its interaction evidence?

- Question: Should CLI transcripts use a new evidence category or remain under the existing visual evidence path?
- Options:
  1. **Keep existing paths** — record `VISUAL-REVIEW: NOT APPLICABLE` and keep transcripts under `evidence/visual/` for compatibility. Gain: no migration. Cost: the name remains misleading. In practice: a CLI transcript is still protocol evidence.
  2. **Add an interaction category** — write `evidence/interaction/` and exempt both paths. Gain: clearer vocabulary. Cost: migration and dual-path support. In practice: CLI evidence is clearly separate from screenshots.
- Disposition: carried to `design.md` — choose the path policy while preserving existing evidence.

### D2 — Should the task template support a one-pass implementation explicitly?

- Question: The current task template implies one revision per scenario. Should one-pass disclosure be a supported mode?
- Options:
  1. **Support disclosure** — add an explicit one-pass disclosure form that records one production revision and replay evidence. Gain: no fabrication. Cost: the rule is less strict. In practice: a one-pass implementation records one honest revision.
  2. **Require per-scenario commits** — keep the strict template. Gain: traceability per scenario. Cost: the agent may fabricate hashes or rewrite history. In practice: the workflow must genuinely commit per scenario.
- Disposition: carried to `design.md` — decide the supported form and the preflight consequence.

### D3 — Is visual review ever required for a CLI?

- Question: Does plain-text CLI output count as a terminal interface requiring visual review?
- Options:
  1. **No visual sign-off** — CLI text is interaction evidence; classify the interface as none for visual review. Gain: no human blocker. Cost: no visual inspection of terminal formatting. In practice: a line-oriented JSON CLI needs protocol transcripts, not screenshots.
  2. **Visual review for terminal UI** — require sign-off for rendered terminal interfaces. Gain: covers real TUI surfaces. Cost: over-applies to machine output. In practice: an interactive TUI with layout and prompts is visual; JSON lines are not.
- Disposition: answered — no human visual sign-off for line-oriented CLI output. Actual rendered terminal interfaces remain visual.
