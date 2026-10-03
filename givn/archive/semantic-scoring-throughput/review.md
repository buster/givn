# Review: semantic-scoring-throughput

Zero-scenario change by user decision (recorded in proposal.md
"Implementation Scope Revision", tasks.md "Scope decision", and the delta
specs header). The retained deliverable is code-level: one
`GIVN_SPEC_THREADS` setting defaulting to `available_parallelism()`,
validated by name, applied to the E5/BGE/NLI `with_intra_threads` sites in
the real-model paths.

## Fabrication audit

- **@e2e tag integrity**: no delta scenarios exist, so no `@e2e` tag was
  removed and no e2e coverage was planned-then-dropped. The delta specs
  header documents the zero-scenario decision explicitly.
- **Empty step bodies**: 0 step definition files added by this change;
  `0 empty step bodies found across 0 new step files`.
- **Checked tasks have production commits**: task `4d7f74f` is the
  implementation commit touching `src/spec/retrieval.rs` (production
  source) plus `run-tests.sh`, the delta spec, and the design/arc42
  records. `ae065df` updates the permanent ADR-register corpus
  (`givn/specs` + `tests/steps`) closed by this change's ADR-064. No
  spec-only or stub-only diff is recorded as a completed task.
- **Promised components exist**: `resolve_thread_budget()`
  (`src/spec/retrieval.rs:127`) with const `GIVN_SPEC_THREADS`
  (`:118`); eager validation call at the top of `review_enabled`;
  `with_intra_threads(resolve_thread_budget()?)` at the three real-model
  sites (E5 fastembed, BGE reranker options, NLI ort session builder);
  the `run-tests.sh` zero-scenario scope guard. All verified by grep and
  compile.
- **Strict-mode proof**: strict mode is pre-existing
  (`GivnWorld::cucumber().fail_on_skipped()` at
  `tests/features_runner.rs:20`). No RED→GREEN runs exist because the
  change carries zero scenarios by decision; strictness is demonstrated
  by the full permanent suite exiting 0 (any unmatched step would
  fail `features_runner`).
- **E2E reading**: no `@e2e` scenarios in scope for this change.
- **`verify.e2e_command` scoping**: `verify.command` = `./run-tests.sh`,
  `verify.e2e_command` = `./run-tests.sh --e2e` (`givn/commands.yaml`) —
  distinct strings, differently filtered. Scenario-count comparison is
  vacuous: the change scope holds zero scenarios, so both commands exit 0
  via the zero-scenario guard without invoking cucumber; stated
  explicitly rather than compared.
- **design.md conformance**: implementation matches the retained design
  exactly — resolver with env override, three call sites, eager
  review-path validation, zero-scenario verification approach, no grouped
  BGE, no status-line reporting. design.md was rewritten during this
  close-out to match (the deviation from the originally reviewed grouped/
  status-line design was decided by the user and re-recorded in
  design.md, proposal.md, and design-review.md, not left silent).
- **Interaction coverage**: zero inventory entries, zero matrix rows, zero
  `@e2e` scenarios — all by decision; nothing to cross-reference.

## Deterministic overlap dispositions

`givn check review` reported `net delta: 0 added, 0 modified, 0 removed`
and `overlap dispositions passed`. No shape matches exist for this change.

| Scenario A | Scenario B | Decision |
| --- | --- | --- |
| — (none) | — (none) | — |

Split-or-keep: not applicable – no scenarios.

## Semantic review classifications (semantic-review.md)

`Status: PASS`, `Candidate count: 0`. No candidates to classify.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `GIVN_SPEC_THREADS`, default `available_parallelism()` | 08 8.5i; ADR-064 | row 8 (08 crosscutting) | Thread budget section | implementation tasks | `resolve_thread_budget()` retrieval.rs:127 | YES |
| By-name rejection; eager review-path validation | ADR-064; 08 8.5i | row 8 | Thread budget section | implementation tasks | gate at top of `review_enabled`; `max_rerank_pairs` error vocabulary | YES |
| Applied at E5/BGE/NLI real-model sites only | ADR-064; 08 8.5i | row 8 | Thread budget section | implementation tasks | three `with_intra_threads` sites | YES |
| NLI batched inference deferred | 11 D-15; ADR-064 | row 11 | Decisions section | implementation tasks | no batched tensor code; session thread count only | YES |
| ADR-064 proposed; register row; corpus count 1→2 | adr/README.md; ADR-064 | row 9 | Decisions section | implementation tasks | ADR-064 file, README row/relationship, permanent spec `35 accepted/2 proposed` (ae065df) | YES |
| FR-40 user-facing requirement | 01-introduction-and-goals | row 1 | design | implementation tasks | FR-40 row present | YES |
| Sequential stages ⇒ budget never contended; split-on-overlap rule | ADR-064; 08 8.5i | row 8 | resolver doc comment | implementation tasks | resolver doc comment retrieval.rs:122-126 | YES |

ARC42 CONFORMANCE: CLEAN

## Coverage

Measured across all test binaries (covers the Gherkin `features_runner`
binary) via `cargo llvm-cov` replayed over the full group-partitioned
suite (`measure-coverage.sh` + `merge-coverages.sh`, both exit 0):

- Project line-rate: 84.13% (18792/22338 lines covered).
- `src/spec/retrieval.rs` line-rate: 88.39%.
- `resolve_thread_budget` default path (`GIVN_SPEC_THREADS` unset →
  `available_parallelism()`, and the parse-into value return): covered,
  240 hits in the merged report through the real-model E5/BGE/NLI review
  runs.
- Override/validation branches (fetching a set `GIVN_SPEC_THREADS`,
  UTF-8 reject, parse reject, zero reject): uncovered by the scenario
  suite.

### Coverage classification

- Dead code: none — the resolver and its three call sites are live.
- Missing test coverage: none added; a scenario asserting thread counts
  through the stub backends is impossible (stubs never construct real
  models), and probing real ONNX models for thread counts is not viable.
- Legitimately hard to test (bucket 3): the override/validation branches
  of `resolve_thread_budget` execute only when a real model constructs
  with `GIVN_SPEC_THREADS` set — not reachable through deterministic stub
  environments, and thread counts are not observable through any backend
  output. Concrete justification: this is exactly why the change carries
  zero scenario tests by user decision (proposal.md, tasks.md). The
  resolver's shape mirrors the already-reviewed `max_rerank_pairs`
  validator (same error vocabulary), and the default path is exercised by
  the real-model review gate.
- Redundant unit tests: none removed (no change-local unit tests).

## Sign-off checklist

- [x] Fabrication audit: clean (all items vacuous for a zero-scenario change or verified above).
- [x] Every checked task has a verified commit touching production code (`4d7f74f`).
- [x] Every promised component exists (`resolve_thread_budget`, gate, three sites, runner guard).
- [x] Strict-mode proof: strict mode pre-existing and the full suite passes; no RED runs exist by zero-scenario decision.
- [x] `verify.command` and `verify.e2e_command` both exit 0 (full suite and `--e2e`).
- [x] Coverage measured across all test binaries, including the Gherkin runner (see Coverage).
- [x] Every coverage gap classified and resolved (bucket 3 with concrete justification).
- [x] Dead code deleted. Missing tests added. Hard-to-test gaps justified.
- [x] Redundant unit tests removed (none existed for this change).
- [x] No `@wip` tags remain. No implementation-layer detail in the spec.
- [x] Canonical E2E policy read from the resolved specs instruction; zero inventory actions ⇒ zero `@e2e` scenarios.
- [x] No `@e2e` scenarios exist; real-interface/driver checks vacuous.
- [x] Local run command starts cleanly: `./run-tests.sh` and `./run-tests.sh --e2e` both exit 0.
- [x] `verify.e2e_command` file(s) identified (`run-tests.sh --e2e`); zero in-scope `@e2e`, so no second implementation to cross-check.
- [x] `verify.e2e_command` is not identical to `verify.command`; zero in-scope scenarios stated explicitly (no cucumber count to compare).
- [x] Implementation matches design.md (design.md updated with the user-directed scope revision, proposal.md/design-review.md note the change; not silent).
- [x] Interaction coverage verified: zero inventory rows by decision.
- [x] No finding excused with a classification outside the three buckets.

## Result

REVIEW: PASS
