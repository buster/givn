# Design Review: bge-reranking-evidence

## Grilling log

| # | Branch | Question | Recommended answer | Outcome |
|---:|---|---|---|---|
| 1 | Scope | Does the bounded BGE layer match the proposal without turning relevance into classification? | Keep E5 raw candidates visible, make only approved BGE recommendations blocking, and keep filtered evidence non-blocking across the five named surfaces. | Resolved. |
| 2 | Tech choices | Is local FastEmbed-rs the correct implementation boundary? | Use locked FastEmbed-rs 5.17.2; use the user-defined offline loader for BGE and retain E5 as candidate generator. | Resolved. |
| 3 | Missing scenarios | Are failure, boundary, compatibility, and isolation paths observable? | Cover disabled/approval-failed E5-only paths, missing/corrupt artifacts, malformed seams, exact 512/513 pools, successful raw-513/top-K-512 partial coverage, and NLI isolation. | Resolved in the feature delta. |
| 4 | Testability | Can each scenario fail in RED with concrete assertions? | Use compiled CLI subprocesses, deterministic endpoint/E5 pool fixtures, endpoint-keyed directional overrides, local artifact fixtures, and strict JSON/report assertions. | Resolved. |
| 5 | E2E fidelity | Do the inventory and matrix use the real interface? | Keep the six primary CLI interactions as `@e2e`; use the compiled `givn` subprocess and stdout/exit/generated report as primary assertions. | Resolved. |
| 6 | Risk | What prevents false-negative admission and unbounded inference? | Require enforced benchmark approval with Rust parity attestation, preserve raw/unscored evidence, and refuse the complete BGE run above 512 pairs before model load. | Resolved. |
| 7 | Dispatch | Can BGE, E5 gate, and NLI be selected independently? | Read `GIVN_SPEC_BGE` only for BGE, cfg-gate NLI behind `nli-evidence`, and keep `spec-similarity-gate` as an independent E5 gate. | Resolved by the explicit migration contract and future NLI scenario. |
| 8 | Approval | Can a forged or incomplete benchmark approve product BGE? | Require `--enforce`, external `--rust-parity`, `--givn-binary`, and `--bge-model`; revalidate the exact parity output, model files, hashes, constants, and approval schema. | Resolved. |

## Resolved by codebase exploration

| Branch | Finding |
|---|---|
| Candidate pipeline | Existing Rust retrieval already centralizes E5 ranking, deterministic exclusions, endpoint identity, and stable ordering; BGE is attached after that pipeline. |
| FastEmbed API | Locked FastEmbed-rs 5.17.2 exposes `TextRerank::try_new_from_user_defined` with the four tokenizer files required by the local artifact contract. |
| CLI boundary | `tests/features_runner.rs` runs the actual compiled binary through existing black-box helpers; no server or alternate interface is needed. |
| Arc42 | The addon is enabled; all twelve selected chapters contain decision-specific content, ADR-052..057 cover the non-trivial decisions, and risks R-52..R-61 record their consequences. |

## Open questions

| # | Question |
|---:|---|
| 1 | None. The absence of production step modules is an implementation task, not an unresolved design decision. |

## Architecture documentation (arc42) check

| # | Chapter | Reviewer's own assessment (Yes/No) | arc42.md's claim (Yes/No) | Match? | Content matches design.md? |
|---:|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Yes | Yes | Yes |
| 2 | 02 architecture-constraints | Yes | Yes | Yes | Yes |
| 3 | 03 context-and-scope | Yes | Yes | Yes | Yes |
| 4 | 04 solution-strategy | Yes | Yes | Yes | Yes |
| 5 | 05 building-block-view | Yes | Yes | Yes | Yes |
| 6 | 06 runtime-view | Yes | Yes | Yes | Yes |
| 7 | 07 deployment-view | Yes | Yes | Yes | Yes |
| 8 | 08 crosscutting-concepts | Yes | Yes | Yes | Yes |
| 9 | 09 architecture-decisions | Yes | Yes | Yes | Yes |
| 10 | 10 quality-requirements | Yes | Yes | Yes | Yes |
| 11 | 11 risks-and-technical-debt | Yes | Yes | Yes | Yes |
| 12 | 12 glossary | Yes | Yes | Yes | Yes |

- [x] All 12 rows assessed independently against proposal.md/design.md before reading arc42.md's claims.
- [x] No selection mismatch exists.
- [x] Every non-trivial architecture/technology decision in design.md has a corresponding MADR entry in chapter 09.
- [x] Every MADR consequence has a counterpart entry in chapter 11.
- [x] No chapter claimed as updated is scaffolded placeholder text.
- [x] No prohibited box-drawing or pseudo-diagram content was found; directory trees remain plain text listings.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `design.md` | Resolved overflow versus partial-unscored semantics; specified approval schema, Rust parity command, local artifact set, dispatch order, runtime policy, and deterministic pool cases. |
| `bge-reranking-evidence.feature` | Added disabled/approval compatibility, successful partial-boundary, malformed seam, artifact, approval-writer, and future NLI scenarios. |
| `evaluate_bge_admission.py` | Added exact approval writer gating, Rust binary attestation verification, artifact/hash revalidation, and runtime-constant checks. |
| `compare.py` | Marked the Python reference pipeline and validated the complete FastEmbed artifact set. |
| `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs` | Added the Rust-generated hidden `givn spec bge-parity` attestation producer and fixed pipeline constants. |
| `docs/arc42/*.md`, `docs/bge-reranking-benchmark.md`, `assets/instructions/specs.md`, `run-tests.sh` | Aligned durable architecture, benchmark commands, guidance, and runner scope with the hardened contract; `@bge-approval` scenarios are routed through the BGE feature matrix rather than the E5-only matrix. |

## Sign-off

- [x] Every branch of the design tree was walked.
- [x] All questions are resolved.
- [x] `design.md` reflects decisions reached during grilling.
- [x] `specs/**/*.feature` was updated for identified gaps.
- [x] `givn lint --change bge-reranking-evidence` parses successfully with no remaining `@wip` scenarios.
- [x] Architecture documentation audit completed.

**DESIGN-REVIEW: PASS**
