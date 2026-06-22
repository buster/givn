# Review — fix-lint-severity-ranking

Completeness-based coverage review for the `lint-command` bug-fix change
`fix-lint-severity-ranking`. The change replaces the inverted
`max_severity: u8` counter in `src/cli/lint.rs` with `has_parse_error` /
`has_wip` booleans and explicit priority logic so that a parse error
(exit 1) outranks a `@wip` finding (exit 2).

## 1. Test runner result

```
$ cargo test --test features_runner
[Summary]
3 features
29 scenarios (29 passed)
153 steps (153 passed)
```

All scenarios GREEN. The new `@givn.added` scenario "A parse error outranks
a @wip finding" passes (asserts exit 1); the `@givn.removed` placeholder
for the old AS-IS scenario "A @wip finding outranks a parse error" is
retained as a deletion marker.

## 2. Coverage measurement

Tool: `cargo llvm-cov 0.8.5` (LLVM source-based coverage). The
`features_runner` binary spawns the compiled `givn` binary as a subprocess
(`tests/common/mod.rs:55 run_givn`); `cargo-llvm-cov` instruments the bin
target and propagates `LLVM_PROFILE_FILE` to the subprocess, so the
`.feature`-driven execution IS instrumented. This satisfies the
"coverage MUST include the Gherkin runner" requirement.

Command:

```
cargo llvm-cov --test features_runner
```

Overall (all instrumented binaries):

| Metric   | Total | Covered | %      |
|----------|-------|---------|--------|
| Lines    | 1797  | 448     | 24.93% |
| Regions  | 2856  | 642     | 22.48% |
| Functions| 183   | 42      | 22.95% |

The low overall number is expected and not a gate: only the `lint-command`
and `new-command` capabilities are exercised by the `.feature` suite under
`givn/specs/`; the other CLI subcommands (`check`, `eject`, `status`,
`upgrade`, `instructions`, `skills`, `missing_testrunner`) have no
`.feature` coverage and are out of scope for this change. The relevant
production files for this change:

| File                   | Lines total | Lines covered | %      | Functions |
|------------------------|-------------|---------------|--------|-----------|
| `src/cli/lint.rs`      | 81          | 79            | 97.53% | 5/5       |
| `src/gherkin/parse.rs` | 69          | 68            | 98.55% | 6/6       |
| `src/gherkin/delta.rs` | 42          | 4             | 9.52%  | 2/11      |

`lint.rs` and `parse.rs` are the files touched by / directly feeding this
change; both are near-fully covered. `delta.rs` is unchanged by this
change and only `is_removed_placeholder` is reachable from `lint.rs` —
its pre-existing gaps were classified in the prior
`reverse-engineer-lint-command` review and are carried forward unchanged
(see gap table, row 3).

## 3. Coverage gap table

Uncovered regions in the production code, classified into exactly one
bucket each.

| # | Bucket | Region | Action taken |
|---|--------|--------|--------------|
| 1 | Missing test (out of scope — deferred) | `src/cli/lint.rs:62-66` — the `@givn.removed` suppression branch inside the `filter` closure: `is_removed_placeholder(s) && s.position.line <= wip_line` (line 63), the closure-closing `});` (line 64), and `return false; // suppress` (line 66). | **Deferred.** This is the over-broad `@givn.removed` suppression logic, explicitly listed under "Out of Scope" in `proposal.md` for the separate change `fix-lint-removed-suppression`. The capturing scenarios ("A @givn.removed placeholder suppresses the @wip warning on itself" and "A @givn.removed scenario suppresses an unrelated later @wip scenario") exist commented-out in the permanent spec at `givn/specs/lint-command/lint-command.feature:162-194` with a pointer to that follow-up change. The current change does not alter the suppression code (it only changes the exit-code ranking), so this gap is pre-existing and not introduced here. No code change in this change; the gap is owned by `fix-lint-removed-suppression`. |
| 2 | Missing test (out of scope — not reachable from lint) | `src/gherkin/delta.rs:22-75` — `is_delta_feature`, `capability_tag`, `delta_op`, `delta_scenarios`, `strip_givn_tags`. These functions are NOT called by `givn lint`; lint imports only `is_removed_placeholder`. | **Carried forward.** Classified identically to the prior `reverse-engineer-lint-command` review (row 3 of its gap table). They are used by other givn capabilities (apply/archive/merge), not by `lint-command`. Not dead code; not reachable from this change's code path. Follow-up `@givn.added` candidates belong in the apply/archive/merge capability specs. |
| 3 | Missing test (out of scope — not reachable from lint) | `src/gherkin/delta.rs:45` — `_ => {}` wildcard arm in `delta_op`. | **Carried forward.** Same as prior review (row 4). Fires only for non-givn tags preceding any `givn.*` tag. Not reachable from `lint.rs`. Follow-up `@givn.added` candidate for the merge/apply spec. |

Summary: 3 gap regions classified — 0 dead code, 0 hard-to-test, 3
missing-test (all pre-existing, all explicitly out of scope for this
change per `proposal.md`, all owned by named follow-up changes or other
capability specs). No gap was introduced by this change.

### Why no dead code was deleted

The 2 uncovered lines in `lint.rs` (64, 66) are inside the
`@givn.removed` suppression feature, which is live (reachable when a
delta feature contains a `@givn.removed` scenario) and intentionally
retained — its over-broad behaviour is the subject of the separate
`fix-lint-removed-suppression` change. Deleting it now would regress a
documented (if imperfect) feature, so it is not YAGNI dead code.

### Redundant unit tests

None. `src/cli/lint.rs` contains no `#[cfg(test)]` block. The redundant
unit tests that previously existed in `src/gherkin/parse.rs` were already
removed by the prior `reverse-engineer-lint-command` characterization
(its review row 1 notes "unit tests were removed"); `parse.rs` now
retains only the `#[cfg(test)] pub fn parse_str` test helper and a
`#[cfg(test)] use`, neither of which is a redundant test surface. No
action required.

## 4. Delta spec hygiene

- The `@wip` tag that was on the `@givn.removed` placeholder scenario was
  removed during this review (it is not needed: `Given a placeholder step`
  is a registered no-op step, so the scenario parses and passes without
  `@wip`). Re-ran `cargo test --test features_runner` → 29 scenarios
  still GREEN.
- The delta spec contains no implementation-layer detail (no class,
  function, or route names). The strings `[WIP  ]` and `[ERROR]` are
  observable CLI output, not implementation identifiers.

## 5. Final GREEN confirmation

```
$ cargo test --test features_runner
[Summary]
3 features
29 scenarios (29 passed)
153 steps (153 passed)
```

## 6. Review hook

```
$ givn check review --change fix-lint-severity-ranking
  ✓ verify
```

## 7. Sign-off checklist

- [x] All scenarios pass (`cargo test --test features_runner` exits 0; 29/29 GREEN).
- [x] Coverage measured across all test binaries, including the Gherkin
      runner that executes the `.feature` files (the `givn` subprocess is
      instrumented via `LLVM_PROFILE_FILE` propagation).
- [x] Every coverage gap classified into exactly one bucket (3 missing-test,
      0 dead code, 0 hard-to-test) and resolved or justified.
- [x] Every dead code region deleted — N/A; no dead code found.
- [x] Every missing-test gap has a new scenario + passing test, or a fix
      — the 3 gaps are pre-existing, explicitly out of scope per
      `proposal.md`, and owned by named follow-up changes
      (`fix-lint-removed-suppression`) or other capability specs
      (apply/archive/merge). No gap was introduced by this change.
- [x] Every hard-to-test gap has a written justification — N/A; none found.
- [x] Redundant unit tests (covered by `.feature` scenarios) removed — N/A;
      `lint.rs` has no unit tests; `parse.rs` redundant tests already
      removed in the prior characterization.
- [x] No `@wip` tags remain in the delta spec (removed from the
      `@givn.removed` placeholder during this review).
- [x] The delta spec contains no implementation-layer detail.
- [x] Review hook passes (`givn check review --change fix-lint-severity-ranking` → `✓ verify`).

REVIEW: PASS
