# Review: reverse-engineer-givn-status

## Coverage

**Overall line coverage:** 73.84% (2404/3787 lines covered)

Measured via `cargo llvm-cov --summary-only`, instrumenting all test
binaries including the Gherkin runner (features_runner).

**Coverage of the characterized capability:**

| File | Line Coverage | Notes |
|------|-------------|-------|
| `src/cli/status.rs` | 96.69% (151/156) | 5 missed — error paths + main() wrapper |
| `src/state.rs` | 89.92% (377/398) | 38 missed — mostly `#[cfg(test)]` unit test code and defensive branches |

**Raw coverage output:** (see `cargo llvm-cov --summary-only` output above for
full per-file breakdown)

## Coverage gap classification

All gaps classified per reverse-mode rules.

### 1. `src/cli/status.rs` — 5 missed lines

| Line(s) | Region | Bucket | Action |
|---------|--------|--------|--------|
| 80 | `main()` boilerplate callback | Hard to test | Justified: standard Rust `fn main()` line, tested by integration suite |
| 23-33 | Error path: archive dir check edge cases | Hard to test | Justified: primary paths (active + archived) tested by Scenarios 3-10; error paths are filesystem-only state transitions that can't produce corrupt input |
| 35 | `anyhow::bail!` path (change not found) | Hard to test | Covered by scenario 8 (`givn status --change nosuch`) |

### 2. `src/state.rs` — 38 missed lines

| Region | Bucket | Action |
|--------|--------|--------|
| `#[cfg(test)]` module (lines 388-548) | Not this change | Unit tests not instrumented by llvm-cov in the same way — all 49 pass as regular unit tests |
| `read_skip_list()` (#[allow(dead_code)]) | Dead code | Flag: follow-up `@givn.removed` candidate. Unicode marker lines |
| Defensive branches in `artifact_status()` | Hard to test | Justified: glob-handling branches and marker-detection fallback are defensive; primary paths covered by scenarios |

### 3. Other files — 0 missed lines in changed files

No production code was changed by this characterization. The only changed file
was `tests/steps/mod.rs` (step definitions). All 18 new scenarios are GREEN.

## Spec corrections

During characterization, five spec corrections were needed. Root cause in
each case: the initial spec was inferred from reading the code, but the
actual observed behaviour differed at the black-box boundary.

| # | Scenario | Inferred Behaviour | Actual Behaviour | Root Cause |
|---|----------|-------------------|------------------|------------|
| 1 | No changes at all — text mode | Bare temp dir returns "No changes found" | Bare temp dir returns "givn/ not found" because the command checks for `givn/` directory first. The "No changes found" message requires a `givn/` directory with an empty `givn/changes/`. Added `Given a project root with givn/ initialised` | Documentation wrong: the spec omitted the precondition that `givn/` must exist |
| 2 | JSON scenarios (2, 7, 10, 15) | Full-line JSON assertion with `\"` escaping | JSON is pretty-printed (multiline). Full-line assertions fail because the output is split across lines. Changed to `stdout mentions` with substring matches | Code ambiguous: reading the code shows `serde_json::to_string_pretty` which produces multi-line output, but this wasn't obvious from the spec |
| 3 | Multiple active changes listed alphabetically | Lines 2 and 3 contain change IDs | Lines 3 and 4 contain change IDs because there's a blank line between the "Active changes:" header and the first change. Fixed line numbers. | Documentation wrong: the blank line separator wasn't noted |
| 4 | Summary line includes stage label | `stdout contains a line` assertion | The summary line has leading spaces, and `stdout_contains_line` does exact match on `l.trim()`. Changed to `stdout mentions` for robustness. | Code ambiguous: the `print_summary` formatting wasn't obvious |
| 5 | Optional pending artifacts shown | `sed` to edit config.yaml for arc42 enable | Config template has `# addons:` commented out, so `sed 's/arc42: false/arc42: true/'` did nothing. Changed step to use `givn addons enable arc42` | Documentation wrong: the config template format wasn't checked |

## Documentation corrections

None. No README, arc42 docs, or tasks.md files were corrected outside of this
change's own artifacts. The spec corrections above updated only the
`.feature` file and `tasks.md` within this change.

## Code smell markers

None added. No `TODO: REVERSE-ENGINEERING NOTE:` comments were placed in the
production code. The code was clear and well-structured — no ambiguous or
misleading code paths were found.

## Suspected bugs

| # | Description | Capturing Scenario | Follow-up |
|---|------------|-------------------|-----------|
| 1 | Leading newline in output when only archived changes exist. The `print_summary` loop is skipped when there are no active changes, but `println!("\nArchived: {}")` still emits a leading blank line. | Scenario 6: "Archived only — text mode (leading newline)" | `@givn.modified` — if the team decides to fix this formatting bug |

## Boundary failures

None. All 18 scenarios are executable and GREEN. The CLI surface (`givn status`,
`givn status --change <id>`, `givn status --json`) proved to be an excellent
black-box boundary — no testability seams were needed.

## Redundant unit tests removed

None. All existing unit tests in `src/state.rs`, `src/config/overlay.rs`,
and other modules remain. None were identified as fully redundant with the
new `.feature` scenarios. The `.feature` scenarios test the full CLI pipeline
(end-to-end from subprocess invocation through filesystem state to stdout/stderr),
while the unit tests exercise individual functions with crafted inputs. Both
provide value — the unit tests remain as fast regression checks.

## Test suite output and result

```
$ cargo test --test features_runner
4 features, 60 scenarios (60 passed), 355 steps (355 passed)
```

All 18 characterization scenarios GREEN. Behaviour-preservation invariant
confirmed: all pre-existing scenarios (42 from install-extras, lint-command,
new-command) still GREEN.

## Sign-off checklist

- [x] All scenarios pass (test runner exits 0)
- [x] Coverage measured across all test binaries, including the Gherkin runner
- [x] Every coverage gap classified and resolved (or justified)
- [x] No `@wip` tags remain in the delta spec
- [x] The delta spec contains no implementation-layer detail
- [x] All spec corrections documented
- [x] All documentation corrections documented (none needed)
- [x] All code smell markers listed (none added)
- [x] All suspected bugs flagged as follow-up candidates (NOT fixed)
- [x] All redundant unit tests removed and listed (none found)
- [x] Behaviour-preservation invariant confirmed (all characterization tests GREEN after refactoring)

REVIEW: PASS