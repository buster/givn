# Design review: change-verification-receipt

Fresh-context review branches walked against the proposal, delta specs, design,
arc42 assessment, and source.

| Branch | Finding |
|---|---|
| Scope | The receipt, the result helper, review validation, archive validation, and the merged-suite-once rule are all specified; `scope` stays optional. No extra feature was added. |
| Tech choices | One receipt file, three digests, a four-count helper, and validation through the existing SCM seam are the simplest fit. The design rejects a dirty-tree digest because it adds a second provenance model. |
| Missing scenarios | Added: missing receipt, stale receipt after a spec change, stale receipt after a command change, current receipt, result helper, archive stale refusal, archive merged-suite-only. |
| Testability | Every scenario has a RED path. The receipt test binds head/features/command; the result helper has unit tests; the archive scenarios assert the invocation log. Single-scenario command and `.fail_on_skipped()` are recorded in design. |
| E2E fidelity | The new check-tasks action is a real compiled-CLI invocation; no browser surface. |
| Interaction coverage | Two new inventory rows (run the change checks, write a runner result); the matrix maps them. |
| Risk | The clean-tree requirement forces one commit; the error names the offending paths. A runner that ignores `GIVN_FEATURES` is recorded as a limitation, not hidden. |
| Use case and Persona | The verify-change and archive-change contracts are updated; no new Persona. |
| Decision ledger | D1-D3 resolved; D4-D6 add realization sites. |
| Visual contract | `Interface: none`; no delta E2E transcript. |
| ADR | ADR-070/072 checked; canonical destination `design.md`; no new ADR. |
| Arc42 | All 12 rows independently assessed; affected chapters updated; `arc42.md` matches. |
| Ubiquitous language | "Change verification receipt" and "Runner result" are in the glossary and used consistently. |

DESIGN-REVIEW: PASS
