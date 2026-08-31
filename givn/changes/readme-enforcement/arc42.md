# arc42 assessment: readme-enforcement

Full 12-row walk against `proposal.md`, `specs/readme-enforcement/readme-enforcement.feature`, and `design.md`.

| # | Row | Affected | Reason / change summary |
|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | The gate set extends within the existing quality goals (blocking gates, proof-first); no goal or stakeholder change. |
| 2 | Constraints | No | No new legal, tech, or organizational constraint. |
| 3 | External systems, interfaces, context, new user-facing surface | Yes | New CLI surface `givn readme check`; ch. 03 interfaces table and in-scope list updated. |
| 4 | Major technical strategy or approach | No | Follows the established coverage-gate precedent (archive-level blocking gate + managed blocks); no new strategy. |
| 5 | New building blocks, modules, components | Yes | `cli::readme` responsibility row added; `cli::archive` and `integrity_hook` rows extended; ch. 05 updated. |
| 6 | New runtime flows, sequences | Yes | Archive stage list gains the pre-merge completeness gate (5) and the post-merge development-block refresh (10); instructions nudge documented in 6.3; ch. 06 updated. |
| 7 | Deployment changes | No | Single-binary artifact unchanged. |
| 8 | Cross-cutting concepts | Yes | New section 8.9a: marker conventions, management markers/unmanaged rule, legacy matcher list, completeness gate, README-IMPACT marker. |
| 9 | Architecture decisions | No | ADR routing is `CANONICAL_ARTIFACT` (five qualification gates pass, but the decision stays inside the ADR-028 mandatory-check boundary); canonical homes are ch. 08 and ch. 12. No MADR file. |
| 10 | New quality scenarios | No | Enforcement behaviour is functional and specified in Gherkin; no new non-functional scenario. |
| 11 | New risks or technical debt | Yes | Debt row D-17 added: the legacy-matcher list must track future scaffold-wording changes. |
| 12 | New domain terms | Yes | Six terms recorded: open placeholder, README check, README completeness gate, README impact decision, README migration, managed block. |

## Status

STATUS: DONE
