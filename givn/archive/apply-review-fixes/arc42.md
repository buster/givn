# arc42 Documentation Update — apply-review-fixes

## Impact Assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
| --- | --- | --- | --- | --- |
| 01 | introduction-and-goals | Yes | License choice (#8) is project identity | Added license section under 1.1 |
| 02 | architecture-constraints | Yes | License is an organisational constraint | Noted MIT OR Apache-2.0 dual license |
| 03 | context-and-scope | No | Scope is unchanged | — |
| 04 | solution-strategy | No | No strategic changes | — |
| 05 | building-block-view | Yes | New module coverage_gate.rs; shared functions moved | Added coverage_gate.rs to module tree; noted validate_id/resolve_change_dir/find_change_dir shared locations |
| 06 | runtime-view | Yes | Archive gate logic changed for post-merge e2e | Updated 6.6 archive scenario to describe the post-merge e2e gate behaviour |
| 07 | deployment-view | No | No deployment changes | — |
| 08 | crosscutting-concepts | Yes | Checkbox parsing, e2e gate behaviour, input validation | Updated 8.6 (filesystem state), 8.12 (hooks), noted shared validation |
| 09 | architecture-decisions | Yes | License decision | Added ADR-034: MIT OR Apache-2.0 dual license |
| 10 | quality-requirements | Yes | Path traversal fix improves security | Noted input validation under correctness quality subtree |
| 11 | risks-and-technical-debt | Yes | Dead code removed; path traversal risk mitigated | Removed dead code debt entry; added path traversal risk as resolved |
| 12 | glossary | No | No new terms | — |

## Chapter Updates

### 01-introduction-and-goals.md
Added: "This project is dual-licensed under MIT OR Apache-2.0 (see ADR-034)."

### 02-architecture-constraints.md
Added: License constraint — MIT OR Apache-2.0 dual license.

### 05-building-block-view.md
Added: `coverage_gate.rs` under `src/cli/`. Noted shared functions
`validate_id`, `resolve_change_dir`, `find_change_dir` in `src/state.rs`.

### 06-runtime-view.md
Updated: Archive scenario (6.6) — post-merge verify-e2e gate now hard-fails
when e2e_command is empty and merged permanent specs contain @e2e scenarios,
mirroring the per-change hook behaviour.

### 08-crosscutting-concepts.md
Updated: 8.6 filesystem-derived state — checkbox parsing now counts both
`[x]` and `[X]`. 8.12 hooks — noted post-merge e2e gate asymmetry resolved.
Added note about shared validate_id pattern for all `--change` parameters.

### 09-architecture-decisions.md
Added: ADR-034 — MIT OR Apache-2.0 dual license. Rationale: Rust ecosystem
standard, unblocks cargo publish, unambiguous contributor signal.

### 10-quality-requirements.md
Updated: Correctness subtree — added input validation (change-id kebab-case
enforcement, artifact-id manifest validation).

### 11-risks-and-technical-debt.md
Updated: Removed path traversal risk (addressed by #3 validate-change-id-input).
Added resolved risk entry. Removed dead-code debt note (#7 deleted).

## Status

STATUS: DONE
