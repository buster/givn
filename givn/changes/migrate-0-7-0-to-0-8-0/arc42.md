# arc42 Documentation Update: migrate-0-7-0-to-0-8-0

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The migration inventories existing artifacts; no product goal, stakeholder, or quality goal changes. | |
| 2 | 02 architecture-constraints | No | No new constraint; the migration applies the 0.8.0 authoring rules to the existing corpus. | |
| 3 | 03 context-and-scope | No | No interface or external-entity change. | |
| 4 | 04 solution-strategy | No | The migration is a consumer-side maintenance change; no strategy change. | |
| 5 | 05 building-block-view | No | No module or responsibility changes; the follow-ups rename capabilities, not components. | |
| 6 | 06 runtime-view | No | The upgrade runtime is unchanged; this change is the generated plan it produced. | |
| 7 | 07 deployment-view | No | No packaging or distribution change. | |
| 8 | 08 crosscutting-concepts | No | No concept change; the follow-ups restate vocabulary and names. | |
| 9 | 09 architecture-decisions | No | This change records the re-qualification verdicts below and names the follow-ups; the register and README edits happen in those follow-ups, not here. | |
| 10 | 10 quality-requirements | No | No quality scenario changes. | |
| 11 | 11 risks-and-technical-debt | No | No new risk; the obsolete records are residue already covered by ADR-069 and tracked by the follow-ups. | |
| 12 | 12 glossary | No | No term is added, removed, or redefined here; the term-record migration is a follow-up. | |

## ADR re-qualification verdicts

All 28 accepted records in `docs/arc42/adr/` were re-run against the five
mandatory dimensions, the three falsification tests, and the cheaper-home
challenge. 24 stay; 3 are obsolete because their subject was removed with the
semantic engine; 1 fails the gate. No `proposed` record exists.

| Record | Verdict | Audit note |
|---|---|---|
| ADR-001 | qualified | Structural product boundary; not single-change reversible. |
| ADR-002 | qualified | Language and ecosystem boundary; hard to reverse. |
| ADR-004 | qualified | Deployment boundary and installability contract. |
| ADR-005 | qualified | Override precedence contract; upgradeability across changes. |
| ADR-008 | qualified | Filesystem boundary for configuration and state. |
| ADR-009 | qualified | Manifest format contract shared by every artifact. |
| ADR-010 | qualified | Customization contract; constrains overlay consumers. |
| ADR-011 | qualified | The specification boundary itself; executable contract. |
| ADR-013 | qualified | Generated-versus-user-owned file boundary. |
| ADR-014 | qualified | State model boundary; no separate store. |
| ADR-015 | qualified | Runner declaration contract; fail-closed behavior. |
| ADR-017 | qualified | Product boundary; no dual-layout runtime. |
| ADR-020 | qualified | Delta tag namespace contract shared by parser, lint, and merge. |
| ADR-023 | qualified | Upgrade contract; override preservation across releases. |
| ADR-024 | qualified | Lifecycle graph contract; project-owned variations. |
| ADR-026 | qualified | Addon packaging and activation boundary. |
| ADR-028 | qualified | Gate contract; mandatory checks cannot be configured away. |
| ADR-037 | qualified | Deterministic review-evidence boundary; explicitly retained by ADR-069. |
| ADR-048 | qualified | Instruction and JSON metadata contract for agents. |
| ADR-067 | qualified | New subsystem and machine contract inside the single binary. |
| ADR-069 | qualified | Build contract and removed subsystem; hard to reverse. |
| ADR-070 | qualified | Publication and proof contract; forward-only repair. |
| ADR-071 | qualified | One detection seam; named refusal boundary. |
| ADR-072 | qualified | Publication contract across both backends. |
| ADR-027 | NOT_QUALIFIED | Fails single-change reversibility, the no-multi-change-constraint test, and the product-behavior exclusion: the command name and its exit codes are observable behavior ownable by `givn/specs/verify-change/usecase.md` Rules plus the lint capability specification. Destination: those two artifacts; the record leaves the register and README. Follow-up: `retire-lint-command-adr`. |
| ADR-044 | obsolete | Describes the removed `spec explore --change` surface; the engine-backed commands left the CLI with ADR-069. Destination: `docs/arc42/adr/archive/` as superseded by ADR-069. Follow-up: `retire-semantic-adr-residue`. |
| ADR-045 | obsolete | The retrieval identity mechanism (permanent, delta-source, projected-target, and file-query identities) was removed with the engine. Destination: `docs/arc42/adr/archive/` as superseded by ADR-069. Follow-up: `retire-semantic-adr-residue`. |
| ADR-054 | obsolete | The E5/BGE/NLI evidence-output schemas were removed with the engine. Destination: `docs/arc42/adr/archive/` as superseded by ADR-069. Follow-up: `retire-semantic-adr-residue`. |

No record fails the five dimensions while still being current; the only
NOT_QUALIFIED record is a naming decision whose behavior contract survives in
lower-level artifacts.

## Notes

The maintenance change deliberately edits no durable chapter. The inventory in
`design.md` is the durable record of the deviations; the follow-up register
there names every repair. The one corpus edit this change makes is the
collision-check evidence in `givn/ideation/multi-scm-support/domain-terms.md`.

---

## Status

STATUS: DONE
