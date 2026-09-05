# arc42 Impact Assessment: add-ideation-skill

This change ships a new synchronized facilitation skill. It adds one
user-facing capability (pre-proposal ideation), one risk register entry for
the unenforced state convention, and the ideation vocabulary in the glossary.
It introduces no new runtime component, interface mechanism, deployment
behavior, or qualified architecture decision.

ADR qualification (recorded, no ADR written): the candidate decisions —
always-on default-skill registration, topic state under `givn/ideation/`, and
two-tier persona persistence — fail the gate. Registration is reversible with
one list entry plus asset removal (durable_consequence fails), and the
complete rationale is process guidance owned by the skill asset and this
change's design (lower_level_artifact fails). Routing: `CANONICAL_ARTIFACT`
(skill template + design.md).

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | No | Agent-guidance capability; the CLI lifecycle graph, functional requirements, stakeholders, and quality attributes are unchanged — ideation sits before the lifecycle like `givn-explore`, which is also outside the graph. |
| 2 | Architecture Constraints | No | No legal, technical, organizational, or platform constraint changes. |
| 3 | Context and Scope | Yes | New user-facing capability in scope: one In-scope line for the synchronized `givn-ideation` skill as the artifact-producing alternative to `givn-explore`. |
| 4 | Solution Strategy | No | The existing cascade, sync, and default-skill strategy is reused unchanged. |
| 5 | Building-Block View | No | The new skill and command assets ride the existing embedded-assets block; `cli::skills` and renderer responsibilities are unchanged. |
| 6 | Runtime View | No | `givn skills sync` and instruction resolution flows are unchanged; the ideation phases run in the agent, not in givn's runtime. |
| 7 | Deployment View | No | No deployment or environment change. |
| 8 | Crosscutting Concepts | No | No new error-handling, security, or configuration concept. |
| 9 | Architecture Decisions | No | No decision passes the five-dimension gate (see qualification note above); no register entry. |
| 10 | Quality Requirements | No | Scenarios assert shipped guidance content; no new system quality requirement. |
| 11 | Risks and Technical Debt | Yes | R-46 added: ideation and persona state are agent-written conventions without binary validation; mitigated by spec-asserted skill contract phrases and handoff review. |
| 12 | Glossary | Yes | 14 ideation terms recorded (Ideation, Ideation topic, Frame, Event Storming, Use Case Definition, Use Case Elaboration, Persona, Stakeholder type, Persona segment, Persona promotion, Change seed, Quality review, Open question, Handoff). |

## Status

STATUS: DONE