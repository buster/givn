# Proposal: review-preflight-and-cli-evidence

## Release note

Enforce one real revision per scenario before review, surface task-integrity and README-impact requirements early, and preserve plain-text CLI interaction evidence without forcing a visual-review sign-off.

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

Before review, a blocking task-integrity check requires one real, unique revision per checked scenario, verifies it exists and is reachable, and requires the commit message to name the scenario. There is no shared or one-pass mode. The preflight reports every finding in one pass.

A machine-only product can record `VISUAL-REVIEW: NOT APPLICABLE` and keep plain-text interaction transcripts as evidence. No human visual sign-off is required for a CLI. A genuinely rendered interface still requires visual review and human sign-off.

Change evidence under `givn/changes/<id>/evidence/` is declared evidence for the tested-tree check and does not invalidate finalization.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| visual-review-gate | `EXTEND visual-review-gate` (verify-change, 15.00) | `EXTEND visual-review-gate` | The existing gate owns visual classification and the not-applicable path. |
| givn-workflow-enforcement | `EXTEND givn-workflow-enforcement` (verify-change, 14.00) | `EXTEND givn-workflow-enforcement` | The preflight reuses the existing review/integrity rules but runs earlier. |
| archive-proof | not ranked | `EXTEND archive-proof` | The archive-proof capability owns the evidence predicate that accepts CLI transcripts without a tested-tree rejection. |

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

- Question: Should a one-pass implementation be an accepted task structure?
- Options:
  1. **No** — every scenario needs its own real revision whose message names the scenario. Gain: traceable evidence. Cost: an agent that implemented in one pass must split the commits before review. In practice: a one-pass implementation fails `check tasks` until the commits exist.
  2. **Yes, with disclosure** — one revision covers every scenario. Gain: less commit work. Cost: the agent can redefine the rule after the fact and hide which work belongs to which scenario. In practice: a retroactive disclosure turns a failed rule into a pass.
- Disposition: answered — option 1. There is no one-pass mode. A shared revision is rejected even when disclosed.

### D3 — Is visual review ever required for a CLI?

- Question: Does plain-text CLI output count as a terminal interface requiring visual review?
- Options:
  1. **No visual sign-off** — CLI text is interaction evidence; classify the interface as none for visual review. Gain: no human blocker. Cost: no visual inspection of terminal formatting. In practice: a line-oriented JSON CLI needs protocol transcripts, not screenshots.
  2. **Visual review for terminal UI** — require sign-off for rendered terminal interfaces. Gain: covers real TUI surfaces. Cost: over-applies to machine output. In practice: an interactive TUI with layout and prompts is visual; JSON lines are not.
- Disposition: answered — no human visual sign-off for line-oriented CLI output. Actual rendered terminal interfaces remain visual.
