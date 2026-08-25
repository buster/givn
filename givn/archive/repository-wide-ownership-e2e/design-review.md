# Design Review: repository-wide-ownership-e2e

## Grilling Log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Does the change move only generic Givn workflow evidence? | PASS. Watn-specific canonical decisions remain in Watn; this change owns only generic review/archive fixture behavior. |
| 2 | Base contract | Does the implementation branch contain the overlap gate under test? | PASS. The branch is rebased on `spec-overlap-gates` PR #5; no overlap implementation is duplicated here. |
| 3 | Technology | Is a real CLI fixture simpler and more faithful than in-process calls? | PASS. `CARGO_BIN_EXE_givn` with a fresh `TempDir` proves the shipped CLI boundary and avoids consumer coupling. |
| 4 | Fixture determinism | Does the fixture produce exactly one required disposition and the expected net delta? | PASS. The design includes exact cross-capability feature contents, tags, disposition row, and unrelated scenario fingerprint. |
| 5 | Failure phase | Is the rollback claim precise? | PASS. The failure scenario uses `verify.command: "false"` and explicitly tests the pre-merge archive gate before backup or merge. Existing archive-gate scenarios cover post-merge rollback. |
| 6 | Runner commands | Are full, filtered, and named commands valid? | PASS. The invalid combined `--tags`/`--name` command was removed. Full active scope, non-E2E projection, E2E projection, and name-only targeting are distinct. |
| 7 | E2E fidelity | Do all inventory interactions use the real interface? | PASS. All three scenarios invoke the compiled Givn CLI and assert exit status/stdout first; tree checks are secondary. |
| 8 | E2E scope | Is every inventory entry mapped exactly once? | PASS. Review, successful archive, and pre-merge failure each have one `@e2e` scenario and one matrix row. All in-scope scenarios are E2E, so equal full/E2E counts are explicit. |
| 9 | Strictness | Can pending steps silently pass? | PASS. `tests/features_runner.rs` uses `.fail_on_skipped()` and the RED stub is `unimplemented!()`. |
| 10 | Addons | Can the temporary fixture run without unrelated addon gates? | PASS. The fixture uses `givn init --no-addons` and writes explicit `true`/`false` verify commands; coverage is disabled in the fixture and remains an outer Givn gate. |
| 11 | Coverage | Is child-process coverage evidence concrete? | PASS. Review runs measurement and merge, then requires non-zero `lines-covered` for `src/cli/check.rs` and `src/cli/archive.rs` in the E2E and merged Cobertura reports. Missing/zero evidence is invalid, not excused. |
| 12 | Architecture | Does Arc42 reflect the test-boundary decision? | PASS. All twelve rows are assessed; affected chapters, ADR-040, quality scenarios, risks, and glossary terms are updated. ADR-037 remains reserved for deterministic overlap fingerprints. |

## Resolved Findings

- The original design did not state that the branch depends on the existing
  overlap-gate implementation. It now explicitly requires `spec-overlap-gates`
  as the stack predecessor.
- The fixture configuration now distinguishes successful fixtures
  (`verify.command: "true"`) from the failure fixture
  (`verify.command: "false"`) and names the pre-merge hook phase.
- The command matrix no longer combines the incompatible `--tags` and `--name`
  filters.
- The rollback scenario is tagged `@e2e`, included in the user interaction
  inventory, and mapped in the matrix.
- Exact permanent and delta feature contents prove the single cross-capability
  shape match, one added operation, one removed operation, and no unrelated
  overlap finding.
- Coverage evidence now names the exact commands, reports, and production
  paths required for sign-off.
- The new architecture decision is ADR-040, following the existing ADR-038 and
  ADR-039 decisions in the current stack.

## Arc42 Assessment

| # | Chapter | Independent assessment | `arc42.md` | Content aligned? |
|---|---|---|---|---|
| 1 | Introduction and Goals | Yes | Yes | Yes |
| 2 | Architecture Constraints | Yes | Yes | Yes |
| 3 | Context and Scope | No | No | Yes |
| 4 | Solution Strategy | Yes | Yes | Yes |
| 5 | Building Block View | Yes | Yes | Yes |
| 6 | Runtime View | Yes | Yes | Yes |
| 7 | Deployment View | No | No | Yes |
| 8 | Crosscutting Concepts | Yes | Yes | Yes |
| 9 | Architecture Decisions | Yes | Yes | Yes |
| 10 | Quality Requirements | Yes | Yes | Yes |
| 11 | Risks and Technical Debt | Yes | Yes | Yes |
| 12 | Glossary | Yes | Yes | Yes |

All twelve durable chapter files exist and contain substantive content. The
chapter scan found no Unicode box-drawing diagrams and no non-table pseudo-
diagrams. ADR-040 consequences are represented by R-21/R-22 and D-10 in
chapter 11.

## Sign-Off

- [x] Scope matches the proposal and contains no Watn-specific fixture behavior.
- [x] The branch is based on the production overlap-disposition gate.
- [x] Exact fixture contents and disposition evidence are specified.
- [x] All three CLI interactions have one `@e2e` scenario and one matrix row.
- [x] Strict mode and the RED stub are explicit.
- [x] Fixture addon/configuration and failure phase are explicit.
- [x] Coverage measurement and child-process evidence are explicit.
- [x] Arc42 assessment is complete and durable chapters are aligned.
- [x] No open design question remains.

DESIGN-REVIEW: PASS
