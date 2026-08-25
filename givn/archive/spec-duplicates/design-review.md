# Design Review: spec-duplicates

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---|---|---|---|---|
| 1 | Scope | Does the change contain exactly the complete permanent audit promised by the proposal? | Keep exact, minimal Fingerprint-based structural, and advisory E5 candidate reporting; exclude action-structure comparison, automatic decisions, index refresh, active changes, and archive contents. | Resolved. Proposal, feature, and design now use the same boundary and terminology. |
| 2 | Tech choices | Is a new structural/action comparison or a new model required? | Reuse parsed Gherkin, canonical behavior content, the existing masked assertion `Fingerprint` and fact model, and local E5 embeddings. | Resolved. No Given/When action-structure comparison, BGE/NLI model, or new dependency is planned. |
| 3 | Missing scenarios | Are the error, empty-corpus, exclusion, threshold, ordering, and persistence boundaries observable? | Add focused `@givn.added @wip` scenarios for malformed/unreadable paths, missing specs, archive exclusion, active/archive byte preservation, absent/malformed index, `0.9500`/`0.9499`, and mixed precedence/equal scores. | Resolved in the feature. The existing single `@e2e` inventory entry and scenario remain. |
| 4 | Testability | Can the exact output and semantic threshold cases fail before production implementation? | Use subprocess assertions for exact text/JSON bytes and a documented stub-only score seam keyed by normalized endpoint identity; keep local E5 as the production path. | Contract resolved. Production enforcement of the seam remains an implementation proof item below. |
| 5 | Risk | What is the most likely failure during implementation? | A complete pair scan becomes impractical or semantic scores are mistaken for duplicate decisions. | Mitigated in design with one embedding per scenario, a 288/41,328 baseline, a 500/124,750 trigger, labeled precision/recall, and advisory-only semantics. |
| 6 | E2E fidelity | Does the standalone E2E command run the active feature scope? | Use `GIVN_FEATURES=givn/changes/spec-duplicates/specs ./run-tests.sh --e2e`; explain that lifecycle hooks inject the same variable before the configured hook command. | Resolved. One CLI inventory entry and one real-subprocess E2E scenario remain. |
| 7 | Read-only boundary | What may the audit change? | Nothing in permanent specs, active changes, archive contents, or `.spec-index.json`; existing local model-cache creation is allowed. | Resolved in proposal, design, feature, and Arc42 docs. |
| 8 | Arc42 | Does the independent chapter assessment match the durable documentation? | Mark chapters 1-6 and 8-12 Yes, chapter 7 No; complete ADR-049 in repository MADR format and add matching quality/risk/glossary facts. | Resolved in the change assessment and durable chapters. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Structural evidence | `src/overlap/fingerprint.rs` exposes only masked assertion values and `Facts` fields for exit code, contains polarity, tables, `[DONE]`, and TTY. The hardened contract uses those facts and does not invent action-structure evidence. |
| Persistent index | `src/spec/retrieval.rs` has a separate `refresh_index` path. The audit design explicitly bypasses it and treats absent or malformed `.spec-index.json` as irrelevant and unchanged. |
| Production model path | `embed_texts` uses local fastembed/E5 when no stub override is present; existing `GIVN_SPEC_EMBEDDER=stub` is the deterministic fixture path. The new audit score seam is documented as stub-only and endpoint-keyed. |
| Empty directory | `collect_feature_files` returns an empty list for a missing directory, supporting the successful empty-corpus contract. Read/parse error handling must retain the offending path instead of silently dropping it. |
| Runner and hooks | `tests/features_runner.rs` uses `.fail_on_skipped()`. `run-tests.sh --e2e` filters `@e2e`, while the hook layer supplies `GIVN_FEATURES`; the standalone command therefore sets the active feature path explicitly. |
| Existing archive scope | Permanent files live under `givn/specs/`, active deltas under `givn/changes/`, and archived material under `givn/archive/`; the audit aggregate is limited to the first path. |

## Implementation acceptance criteria

The score-control seam is a resolved design constraint, not an additional
product decision. Its implementation acceptance criteria are carried into the
scenario tasks: `GIVN_SPEC_DUPLICATES_TEST_SCORES` is honored only with the
stub embedder, uses normalized unordered endpoint identities, rejects malformed
keys and values, and cannot alter production local-E5 scoring. The threshold,
below-boundary, equal-score, and production-path scenarios provide the required
proof during implementation.

## Architecture documentation (arc42) check

The independent assessment was made from the hardened proposal, feature, and
design before comparing the change-level marker. All twelve chapter files were
checked for existence, non-placeholder content, and prohibited ASCII-art
diagrams; no chapter selection is fabricated beyond the duplicate-audit impact.

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Yes | Yes | Yes - FR-32, deterministic output, and measured model quality are recorded. |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes - read-only boundary and permitted model-cache side effect are recorded. |
| 3 | 03 context-and-scope | Yes | Yes | Yes | Yes - CLI surface, evidence scope, and fixed output contract are recorded. |
| 4 | 04 solution-strategy | Yes | Yes | Yes | Yes - precedence, Fingerprint evidence, threshold, score semantics, and cost trigger are recorded. |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes - retrieval renderer, test modules, and score seam are recorded. |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes - parse, classify, embed, rank, render, and non-mutation flow is recorded. |
| 7 | 07 deployment-view | No | No | Yes | N/A - no deployment, service, or distribution change. |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes - structural vocabulary, output grammar, threshold, ranks, and boundary are recorded. |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes - ADR-049 uses the repository MADR metadata, outcome, consequences, confirmation, and option-by-option pros/cons. |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes - Q-75 through Q-77 cover exact evidence, non-mutation, and model metrics. |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes - R-50 measures quadratic growth and R-51 measures model/advisory risk. |
| 12 | 12 glossary | Yes | Yes | Yes | Yes - audit, structural, subset-direction, and semantic advisory terms are aligned. |

- [x] All 12 rows assessed independently against proposal.md/design.md.
- [x] No row is "Match? = No" with the reviewer saying "Yes" and arc42.md saying "No" or omitting the row.
- [x] Every non-trivial architecture decision in design.md has a MADR entry in chapter 09.
- [x] Every ADR-049 "Bad, because..." consequence has a counterpart in chapter 11.
- [x] No claimed affected chapter is scaffold-only or contains a prohibited ASCII-art diagram.
- [x] Arc42 index and affected chapter content remain consistent with the hardened design.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| proposal.md | Resolved structural terminology, output/read-only behavior, threshold semantics, advisory model scope, and remaining implementation-only proof boundary. |
| specs/spec-duplicates/spec-duplicates.feature | Strengthened exact output assertions and added focused malformed/unreadable, empty, archive, active/archive, index, threshold, and ordering scenarios. |
| design.md | Defined the complete text/JSON contract, Fingerprint evidence, relation enum, score/null rules, invariants, endpoint-keyed score seam, E2E scope, performance baseline/trigger, and model metrics. |
| arc42.md | Updated the full twelve-row assessment to match the hardened planning scope. |
| docs/arc42/09-architecture-decisions.md | Completed ADR-049 in repository MADR format with consulted/informed metadata and explicit pros/cons for all options. |
| docs/arc42/01-introduction-and-goals.md, 02-architecture-constraints.md, 03-context-and-scope.md, 04-solution-strategy.md, 05-building-block-view.md, 06-runtime-view.md, 08-crosscutting-concepts.md, 10-quality-requirements.md, 11-risks-and-technical-debt.md, 12-glossary.md, README.md | Aligned affected durable Arc42 content with the fixed contract, minimal structural evidence, boundary, metrics, and risks. |

## Sign-off

- [x] Every branch of the design tree has been walked.
- [x] All questions are resolved.
- [x] `design.md` reflects decisions reached during grilling.
- [x] `specs/**/*.feature` updated for the missing scenarios.
- [x] `givn lint --change spec-duplicates` exits 2 with no parse errors.
- [x] Architecture documentation check completed above.

DESIGN-REVIEW: PASS
