# Design Review: make-usecases-canonical

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the strict parser replace all active group contracts? | Yes; no fallback aliases. | Resolved |
| 2 | Testability | Can ownership be observed through the real CLI? | Yes; tree output and lint run through the subprocess runner. | Resolved |
| 3 | Migration safety | Can stale narratives mutate the corpus? | No; base hash validation runs before writes and the backup covers narratives. | Resolved |
| 4 | Domain context | Are actors and Personas distinct? | Yes; Personas are references/review lenses and never Gherkin actors. | Resolved |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Existing parser | The old parser silently ignored unknown headings; the new parser rejects them. |
| Existing archive | The specs backup can restore the complete tree; narrative validation is added before merge. |
| Existing E2E runner | `cucumber-rs` uses `.fail_on_skipped()`. |

## Use-Case And Persona Context

| Check | Finding | User disposition |
|---|---|---|
| Use-case contract and capability ownership | Explicit `usecase.md` owns each capability. | Accepted |
| Typed relationships | Includes/Extends resolve to use cases or fragments. | Accepted |
| Confirmed Persona coverage | No Personas are declared by this tooling change. | N/A |
| Actor and Persona distinction | Feature steps use actors; Persona biographies are absent. | Accepted |

## Open questions

| # | Question |
|---|---|
| 1 | Release target version remains unresolved until release cutting. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All implementation questions are resolved.
- [x] `givn lint --change make-usecases-canonical` is clean apart from advisory overlap findings.

**DESIGN-REVIEW: PASS**
