# Arc42 Documentation Update: release-strict-model-overrides

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal change; the workflow and gate contract are unchanged. The strict-install guarantee is captured as quality requirements, not as a goal. | None. |
| 2 | 02 architecture-constraints | Yes | A fixed build-kind constraint now exists: fixture requests are a development-build capability; installed builds always run the real models. | Section 2.4: added the installed/dev build-kind split paragraph. |
| 3 | 03 context-and-scope | No | No new external surface; the CLI boundary is unchanged. | None. |
| 4 | 04 solution-strategy | No | The mechanism (compile-time profile switch) is a design decision owned by `design.md`, not a new strategy. | None. |
| 5 | 05 building-block-view | Yes | `spec::retrieval` gains the single gated model-channel accessor. | Section 5.4: added the gated-accessor paragraph. |
| 6 | 06 runtime-view | No | No new runtime path, daemon, or flow. | None. |
| 7 | 07 deployment-view | Yes | Installed vs development build behavior differs; `GIVN_SPEC_BGE=disabled` is no longer a release opt-out. | Section 7.2: corrected the BGE opt-out sentence; added the installed-build strictness paragraph. |
| 8 | 08 crosscutting-concepts | Yes | The model-channel taxonomy (fixture request vs model-location request) is a cross-cutting concept. | Section 8.5a: added the two-channel-class paragraph. |
| 9 | 09 architecture-decisions | No | The qualified decision is routed to canonical artifacts (spec + `design.md` + glossary), not a new ADR; no register entry added. | None. |
| 10 | 10 quality-requirements | Yes | New gate-safety and model-availability requirements for the installed build. | Section 10.2: added Q-67 and Q-68. |
| 11 | 11 risks-and-technical-debt | Yes | Release-only gate branches are not executed by the debug-instrumented coverage reports. | Section 11.2: added D-16. |
| 12 | 12 glossary | Yes | New domain terms for the two build kinds and the two channel classes. | Added rows: Installed build, Development build, Fixture request, Model-location request. |

## Notes

Installed (release) builds ignore every fixture request and always evaluate
the semantic gates with the real models; development builds keep honouring
fixture requests so the fixture suite is unchanged. Model-location requests
work in both build kinds. The enforcement surface is the Cargo profile
`debug_assertions`, so a release binary cannot be flipped back to honouring
fixture requests through configuration. The decision is documented as
behavior (spec), mechanism (`design.md`), and vocabulary (glossary); it raises
no new ADR.

## Status

STATUS: DONE
