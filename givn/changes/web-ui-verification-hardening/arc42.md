# arc42 Documentation Update: web-ui-verification-hardening

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or top-level quality goal changes; the change hardens existing verification guidance and gates. | |
| 2 | 02 architecture-constraints | No | No new legal, technical, or organizational constraint; givn still adds no dependency. | |
| 3 | 03 context-and-scope | No | No new external system or interface; the browser driver remains the consuming project's tool. | |
| 4 | 04 solution-strategy | No | The existing single-binary, embedded-guidance, machine-gate strategy is applied, not changed. | |
| 5 | 05 building-block-view | Yes | The `integrity_hook` responsibility grows by the persona disposition, state evidence, and deferral checks. | The `integrity_hook` row names the new checks. |
| 6 | 06 runtime-view | Yes | The `givn check review` flow gains the persona, state-evidence, and deferral checks. | Section 6.6 documents the extended gate semantics. |
| 7 | 07 deployment-view | No | No deployment topology, port, or artifact change. | |
| 8 | 08 crosscutting-concepts | Yes | The visual review discipline gains fail-closed classification, per-state evidence, persona disposition, and the deferral decision. | Section 8.17 is extended. |
| 9 | 09 architecture-decisions | Yes | The new mandatory checks extend ADR-028's boundary; a refinement amends ADR-028 rather than creating a new record. | Chapter 09 records the third ADR-028 amendment. |
| 10 | 10 quality-requirements | Yes | New gate-safety quality scenarios for state evidence, persona disposition, deferral, and classification. | Q-88 to Q-91. |
| 11 | 11 risks-and-technical-debt | Yes | New risks: marker-only persona evidence, omitted deferrals, state-evidence growth; new guidance-level debt; R-57's evidence scope is revised. | R-57 revised; R-60 to R-62 and D-19 added. |
| 12 | 12 glossary | Yes | New domain terms and invariants. | Web UI classification, UI state evidence, Persona disposition, Deferral decision, and the extended visual-review invariants. |

## Notes

- The classification rule, the driver default, and the evidence conventions
  are owned by `assets/instructions/specs.md` and
  `assets/instructions/design.md`; the review obligations are owned by
  `assets/instructions/review.md` and `assets/instructions/design-review.md`.
- The gate is deliberately mechanical only for decidable artifact facts:
  a `PERSONAS:` marker, a `DEFERRAL:`/`OPERATOR-DECISION:` pair, and the
  presence of state evidence files. The classification judgment and the
  quality of a persona disposition remain review guidance (D-19).
- No diagram in the affected chapters uses ASCII art.

ADR action: AMEND_ADR
ADR target: ADR-028

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: AMEND_ADR
canonical_artifact: null
target_adr: ADR-028
replacement_adr: null

- alternatives: the new checks can run inside the non-deactivatable integrity
  member (chosen) or as removable `post` hooks (rejected by the ADR-028
  boundary).
- architectural_impact: the decision fixes which checks no `post` entry may
  disable.
- durable_consequence: reversal would move checks back out of the integrity
  member and re-open the boundary the ADR exists to close.
- lower_level_artifact: the check semantics live in `design.md` and the
  review guidance; only the boundary membership belongs to ADR-028.
- existing_adr_check: the active and archived registers were searched; ADR-028
  is the matching boundary and already carries two amendments of the same
  kind, so a new record would duplicate it.

The amendment is recorded in chapter 09 and in the ADR-028 body's Decision
Outcome, consistent with the two prior amendments.

---

## Status

STATUS: DONE
