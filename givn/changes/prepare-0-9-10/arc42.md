# Architecture impact: prepare-0-9-10

This is the planning assessment, not a claim that implementation or durable-document reconciliation is complete. Existing project chapters supply the baseline; update affected facts alongside implementation and complete this assessment before final review/archive. The operator requested phase-by-phase commits and implementation after design review. Do not fabricate completed chapter updates to satisfy a marker.

| # | Chapter | Affected | Planned update / present assessment |
|---|---|---|---|
| 1 | introduction-and-goals | Yes | Concise trustworthy guidance, retained modularity/Personas, lean fix-forward workflow. |
| 2 | architecture-constraints | Yes | Explicit-path commits, skills-only, configuration authority and future1.0 migration baseline. |
| 3 | context-and-scope | Yes | Agent skill entrypoints, retained Git/Fossil contracts, advisory structure tools. |
| 4 | solution-strategy | Yes | Small shared witness/content/owner contracts, no generic engine; source-confirmed outline-assisted REFACTOR. |
| 5 | building-block-view | Yes | Remove command generation/outline gate; focused parsing/ownership/witness responsibilities. |
| 6 | runtime-view | Yes | Fresh verification and persisted recovery; noncircular review, bounded readiness/recording, explicit selected commits. |
| 7 | deployment-view | Yes | Skills-only target file inventory and generated-command retirement; no new deployed service. |
| 8 | crosscutting-concepts | Yes | Canonical authority, config/defaults, tests-first batches, scope applicability, marker parsing and recording limitations. |
| 9 | architecture-decisions | Yes | Amend accepted records contradicted by chosen skills/config/verification boundaries; retain original rationale with current links. |
| 10 | quality-requirements | Yes | Fresh stable receipts, no silent merge loss, safe writes/timeouts, correct diagnostics, advisory metrics and no review loops. |
| 11 | risks-and-technical-debt | Yes | One concrete post-1.0 backlog: matching quality, outline coverage/metrics/parity, recording recovery, optional dialect/adapters/editor support; retain evidence and defer reasons. |
| 12 | glossary | Yes | Canonical agreed vocabulary below plus correction of stale duplicated terms during implementation. |

## Qualification disposition

| Decision | Qualification | Canonical home / existing record |
|---|---|---|
| Skills-only entry surface | Durable consumer integration boundary; real alternative thin generated commands rejected | Amend ADR-013-agent-integration and related current reference; do not create a duplicate integration ADR. |
| Policy versus executable settings authority | Durable configuration contract; alternative two full policy overlays rejected | Amend ADR-010-manifest-overlay and reference after checking recorded details. |
| Content-bound fresh verification and lean recovery | Durable proof/publication contract; stale evidence and blanket repeated suites rejected | Amend ADR-070-forward-only-archive-proof / ADR-072-witness-bounded-publication where boundary already owned; preserve historical reasoning. |
| Advisory outline / shape, task RED records, ADR qualification, dialog/challenge style | Observable workflow/authoring policy, no independent new architecture boundary | Owning feature/instruction/design; no new ADR. |
| Timeout/socket and safe marker corrections | Local implementation of existing guarantees | Design/source/regression scenarios, not ADR. |

No ADR drafted here: existing-owner search identifies amendment targets, not new independent architecture records. Exact durable amendments follow implementation facts. Qualification is recorded here once, not duplicated in every ADR.

## Vocabulary settled by the operator

- Communication policy: the optional presentation policy, not the mandatory dialog policy. Example: disabling terse style does not disable answerable questions. Source setting: communication_policy.
- Challenge: evidence-first examination of a supplied proposal, test, specification, design, implementation or whole change. It is not a plan-only phase or mandatory gate. Source addon/skill: challenge / givn-challenge.
- Behavior batch: related scenarios implemented through actual RED/GREEN/REFACTOR and one coherent completed explicit-path revision; not a separate commit per phase or necessarily per scenario.
- Existing behavior protected: an immediately passing new regression test for a distinct already-satisfied requirement, without a production edit for that requirement; not fabricated RED.
- Advisory outline: supported-language structural evidence used during REFACTOR/review, not a verdict or complete runtime trace. Unsupported languages receive source assessment.
- Targeted closure: verification of recorded findings and material regressions introduced by their fixes, not an invitation to find additional minor polish. Deferred work remains recorded.

## Status

STATUS: PARTIAL
