# Proposal: prepare-0-9-10

## Release note

Givn prepares for a stable 1.0 with skills-only guidance, genuine test-first behavior batches, advisory structural analysis, and reliable, lean verification and recovery.

## Use-Case Context

- Primary use case: verify-change — verify a change without false assurance or unnecessary repeated work.
- Supporting use cases: initialize-project, obtain-effective-guidance, create-change, archive-change, assess-code-structure, inspect-change-state, discover-specifications, operate-product, keep-documentation-current, inspect-development-history, upgrade-a-project, publish-a-release; reusable support fragment.
- Ideation topic: none. Seed path: none.
- Confirmed Personas: developer--primary-user; agent--technical-role; givn-maintainer--owner. Their existing authority and goals are reused, not reconfirmed or invented. Outdated per-scenario-commit statements are superseded for this change by the agreed batch contract.
- Source: the operator's completed point-by-point audit discussion, including the final outline rules. The portable decisions are recorded here and in design.md; no execution depends on the agent-local scratch plan.
- Target preparation release: 0.9.10. Version bump, tagging and release publication are not part of this change.

## Review Context

- Review kind: independent
- Reason: major change to public command/configuration contracts, executable specification preservation, verification receipts and external integration behavior. One independent reviewer context covers design and final review; the author implements. No blanket delegation prescription is introduced into consumer skills.

## Problem / Opportunity

Users and agents encounter competing instructions, unnecessary dispositions and misleading assurances. Test-first practice is pursued by inspecting old commits instead of requiring an actual failing behavioral test before implementation. Configuration and upgrade examples promise unsupported behavior. Structural hints block unrelated work, while some verification results can reuse stale output or describe content not tested. Repeated review and archive work is expensive and encourages procedural workarounds. Supported specification content can be lost during publication. The 1.0 boundary must be understandable without preserving every experimental 0.x convention.

## Proposed Solution

### Reliable outcomes without extra ceremony

1. Verification requires fresh results and stable tested inputs, with useful independent diagnostics reported together. Changes declaring `Behavior change: none`, explicitly skipping specs, and containing no delta feature files report changed-scenario verification as not applicable, not falsely passed; contradictory deltas or a missing skip refuse before mutation. Applicable merged-candidate archive verification remains required.
2. Archive preserves supported English executable specifications or refuses unsupported constructs, explicit non-English dialects, and conflicting Background replacements before mutation. 0.9.10 supports English Gherkin only. Preservation comparisons include executable arguments and inherited setup, without claiming semantic equivalence.
3. Recovery is trusted, lean and fix-forward. Reuse valid successful evidence when relevant inputs match; do not rerun suites merely because evidence publication failed. Retain existing broad archive evidence exemptions and explain their limitation. No blanket retry re-review.
4. Scenario ownership, typed references, interaction identity and actual verdicts are checked consistently. Long scenarios still require a meaningful split-or-keep decision; shape similarity and outline findings do not block review or archive.
5. Coverage reports must be fresh and valid, but equal coverage is not a failure. Verification scopes are explicit; relative scenario counts and different command strings are not evidence of isolation.
6. Status separates artifact completion, verification state, pending candidates and proven archives without running tests. Bare check explains explicit checks and performs none.

### Genuine test-first work and maintainable architecture

7. Before production edits, each new behavior receives a meaningful observed RED. Actual RED, GREEN and REFACTOR commands/results are briefly recorded as they happen in the named task batch. Existing already-satisfied requirements may be protected by immediately passing tests without production edits. Never reconstruct RED or rewrite history. One coherent completed behavior batch is committed with explicit paths; separate GREEN refactoring commits improve encountered code.
8. Require cohesive modules, narrow consumer contracts, external adapters, separate composition and explicit dependency direction from the first implementation. Personas remain part of changes and domain vocabulary remains coupled to arc42.
9. With arc42 enabled, the first completed change establishes twelve proportionate chapters without unfinished scaffolds; honest specific unknowns and explained inapplicability are valid. Later changes update affected chapters. Qualify ADRs concisely once and track actionable risks, not every accepted downside.
10. Outline assists REFACTOR once per behavior batch for changed Rust/Python components and relevant neighbors. Ask about responsibility, narrow contracts, dependency direction and decomposition; confirm concerns against source. Reuse reports until structural changes make them stale. Unsupported languages such as Go/Java receive source-based assessment, not failure or waivers. Metrics are hints and unavailable measurements are not zero. Final review reuses relevant evidence without per-unit verdicts or metric waivers.
11. Review uses evidence available at its phase, then one targeted closure checks recorded findings and material fix regressions. Minor new observations are retained as deferred work, not another search-and-fix loop.

### One guidance surface and safe operations

12. Generate skills only, retaining the selected agent targets and project-owned replacements. Remove recognizable obsolete generated commands safely. Claude/OpenCode use supported skill invocation; OMP uses its skill syntax; Maki relies on explicit requests or skill selection, not generated slash commands.
13. Rename the presentation addon to communication_policy; dialog correctness stays mandatory. Bootstrap loads dialog once per context. Preserve real-life question regression examples in a companion, with required indexed reading before the first question of each type.
14. Consolidate grill addons into challenge / givn-challenge for proposals, tests, code and changes. Investigate first, cite available evidence, label hypotheses and ask only unresolved human decisions; no added lifecycle gate.
15. Initialization preserves explicit choices and reports required skill-generation failures honestly. Configuration editing changes the intended setting or refuses before writing. Policy belongs in config.yaml; executable commands/runtime settings belong in commands.yaml. Defaults and partial blocks resolve consistently.
16. Commits require explicit paths and preserve unrelated work on Git/Fossil. Hook installation cannot self-dispatch. Readiness waits have a real deadline. README updates refuse ambiguous managed markers before writing. Recorders have bounded waits and cannot prevent primary work.
17. Recordings remain local, ignored and outside repository history; warn that they may contain sensitive content. No new recording privacy/filtering program.
18. Correct shipped templates/examples, remove harness-project leakage from current shipped guidance/help, consolidate duplicated docs and clearly label historical decisions. Establish 1.0 as the future migration baseline, remove embedded 0.x routes, and document manual transition; 0.x configuration compatibility is not required. Preserve user-owned specifications/files and keep future migration machinery. The shipped route catalog may be empty at the 1.0 migration baseline, contains no pre-1.0 routes, and still declares its schema; synthetic future-route fixtures retain complete bundle-validation coverage.

## Capability Routing

The initial route command recommended EXTEND givn-workflow-enforcement (signal, score 17); it ran against the scaffold. That is the primary verification owner, not an instruction to place all behavior in it. The following existing owners are selected from their permanent contracts. No new capability or umbrella use case is introduced.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| archive-proof | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND archive-proof | Existing archive-change ownership; do not concentrate unrelated goals in verification. |
| extract-coverage-gate | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND extract-coverage-gate | Existing archive-change ownership; do not concentrate unrelated goals in verification. |
| grouped-merge-for-new-capabilities | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND grouped-merge-for-new-capabilities | Existing archive-change ownership; do not concentrate unrelated goals in verification. |
| preserve-scenario-docstrings | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND preserve-scenario-docstrings | Existing archive-change ownership; do not concentrate unrelated goals in verification. |
| outline-report | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND outline-report | Existing assess-code-structure ownership; do not concentrate unrelated goals in verification. |
| prose-consolidation | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND prose-consolidation | Existing create-change ownership; do not concentrate unrelated goals in verification. |
| spec-duplicates | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND spec-duplicates | Existing discover-specifications ownership; do not concentrate unrelated goals in verification. |
| adr-qualification-gate | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND adr-qualification-gate | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| attribute-commits | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND attribute-commits | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| enable-commit-attribution | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND enable-commit-attribution | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| instruction-integrity | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND instruction-integrity | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| match-guidance-to-enabled-addons | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND match-guidance-to-enabled-addons | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| refresh-generated-guidance | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND refresh-generated-guidance | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| review-architecture-documentation | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND review-architecture-documentation | Existing fragments ownership; do not concentrate unrelated goals in verification. |
| enable-optional-addons | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND enable-optional-addons | Existing initialize-project ownership; do not concentrate unrelated goals in verification. |
| initialize-a-project | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND initialize-a-project | Existing initialize-project ownership; do not concentrate unrelated goals in verification. |
| report-change-status | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND report-change-status | Existing inspect-change-state ownership; do not concentrate unrelated goals in verification. |
| agent-activity-reporting | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND agent-activity-reporting | Existing inspect-development-history ownership; do not concentrate unrelated goals in verification. |
| readme-enforcement | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND readme-enforcement | Existing keep-documentation-current ownership; do not concentrate unrelated goals in verification. |
| serve-extended-rules | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND serve-extended-rules | Existing obtain-effective-guidance ownership; do not concentrate unrelated goals in verification. |
| start-the-declared-product | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND start-the-declared-product | Existing operate-product ownership; do not concentrate unrelated goals in verification. |
| apply-a-managed-upgrade | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND apply-a-managed-upgrade | Existing upgrade-a-project ownership; do not concentrate unrelated goals in verification. |
| describe-the-release | No signal on targeted recheck | EXTEND describe-the-release | Existing publish-a-release owner; permit the empty shipped 1.0-baseline route catalog while retaining complete future bundle validation. |
| check-specification-health | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND check-specification-health | Existing verify-change ownership; do not concentrate unrelated goals in verification. |
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND givn-workflow-enforcement | Primary verification behavior. |
| outline-review-gate | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND outline-review-gate | Retire blocking outline enforcement; retain advisory outline-report. |
| review-disposition-gates | EXTEND givn-workflow-enforcement (initial global signal) | EXTEND review-disposition-gates | Existing verify-change ownership; do not concentrate unrelated goals in verification. |
| generate-ideation-guidance | EXTEND generate-ideation-guidance (existing support capability) | EXTEND generate-ideation-guidance | Skills-only synchronization preserves ideation guidance without a generated command. |
| measure-coverage-per-project | EXTEND measure-coverage-per-project (existing support capability) | EXTEND measure-coverage-per-project | Documentation follows the configured coverage scripts under the current setup contract. |

## Compatibility and user interactions

Use-case delta narratives preserve Actors, confirmed Personas and typed relationships and update affected interaction inventories. Existing real-interface smokes are reused; input/backend/format variants remain regular scenarios. The sole removed capability is outline-review-gate; advisory reporting remains. Exact old scenario titles identify replacements even when their wording names retired commands.

Deliberate pre-1.0 breaks: explicit commit paths; skills-only invocation; communication_policy and challenge settings; no embedded automatic 0.x migrations. Manual transition is acceptable. No user-owned file is silently deleted.

## Out of Scope

- Release publication, version bump/tagging, extra RC policy, or a new review artifact.
- Full Gherkin support, Java/Go outline adapters, improved heuristic matching, complete outline coverage/parity or missing metric implementations.
- Narrow archive output allowlists, adversarial operator protections, blanket retry prerequisite revalidation, extra full-suite runs, or automatic rollback/reset/history rewriting.
- Recording redaction/encryption/retention machinery, comprehensive event recovery, or telemetry completeness promises.
- Standalone glossary, removal of Personas, relaxation of initial arc42 baseline, scenario-length decisions or architectural boundaries.
- General validator/plugin/transaction frameworks or unrelated broad internal rewrites.

## Open Questions

No unresolved human requirement decisions. The discussion explicitly accepted the solutions above and rejected blanket archive revalidation, narrow archive exemptions, recording privacy restrictions, expanded rollback, relaxed Personas/architecture and extra release governance. Technical choices are resolved in design.md; future work is listed there with evidence and is not an implementation dependency.
