# Review: retire-semantic-capabilities

## Fabrication Audit (do this first)

**Step definition scan** — no step definitions were added, changed, or
removed by this change. Every delta file is a zero-scenario
`@givn.retired` marker; there is nothing to implement or stub. Verified by
inspection of `givn/changes/retire-semantic-capabilities/specs/**/*.feature`
(15 files, each exactly two lines: tags + Feature line).

**Commit cross-check** — the change's artifacts commit contains the
proposal, design, tasks, review, and the 15 retirement deltas; the archive
commit contains the actual corpus deletions and the tombstone. No source
files are touched, so there are no scenario-level production commits to
cross-check (per tasks.md's stated adaptation of the commit rule to a
pure-deletion change).

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| 15 retirement deltas + artifacts | (artifacts commit) | N/A — deletion change, no production code |
| Teardown verification | (archive commit) | deletion of 15 spec directories + tombstone |

**Promised-architecture cross-check** — design.md promises no components;
none were created. The only new filesystem artifact is the tombstone
`givn/specs/.retired.yaml`, produced by the already-shipped retirement
mechanism (archived in the binary as of Phase 0 of the retirement plan).

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none promised | n/a | n/a |

**Strict-mode proof** — N/A: no scenarios authored, nothing to run strict
mode against. `cargo test` green is the only suite-level requirement.

**Downgraded e2e scenario scan** — N/A: zero scenarios in this change.

**Local environment check** — N/A: no run surface touched.

**E2e isolation check** — N/A: no e2e scenarios authored.

**Design.md conformance check** — the change does exactly what design.md
says: no new components, no step definitions, deletion only.

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Retirement mechanism | already shipped, used as-is | yes | yes | n/a |

## Retirements

RETIRE: bge-reranking-evidence — specifies the BGE reranker stage of the semantic gate being removed
RETIRE: nli-evidence — specifies the NLI evidence stage of the semantic gate being removed
RETIRE: search-evidence — specifies cross-encoder evidence for semantic search being removed
RETIRE: unified-semantic-rulebook — specifies the semantic review rulebook engine being removed
RETIRE: spec-retrieval — specifies the E5-embedding retrieval/indexing engine being removed
RETIRE: semantic-compact-serialization — specifies compact serialization of semantic passages being removed
RETIRE: semantic-e5-token-cap — specifies E5 token-cap handling being removed
RETIRE: semantic-model-download — specifies automatic embedding-model download being removed
RETIRE: semantic-remediation-log — specifies the remediation log of the semantic gate being removed
RETIRE: semantic-reranker-budget — specifies the BGE reranker budget being removed
RETIRE: semantic-review — specifies semantic review report generation being removed
RETIRE: semantic-review-output — specifies semantic review output formatting being removed
RETIRE: semantic-scoring-throughput — already an empty 1-line corpse; dead capability
RETIRE: semantic-token-cap-review — specifies token-cap review evidence being removed
RETIRE: ci-debuginfo-optimization — already an empty 1-line corpse; dead capability

## Sign-off

REVIEW: PASS

- Fabrication audit: CLEAN
- All retirement deltas lint-clean (`givn lint --change retire-semantic-capabilities`: 15 files, 0 findings)
- Retirement list matches the verified 15-capability list; the four
  surviving `spec-*` subcommand capabilities are explicitly not retired

README-IMPACT: none
