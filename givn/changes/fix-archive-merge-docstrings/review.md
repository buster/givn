# Review: fix-archive-merge-docstrings

## Fabrication Audit (do this first)

**@e2e tag integrity** — grep of `givn/changes/fix-archive-merge-docstrings/specs/preserve-docstrings-on-archive/preserve-docstrings-on-archive.feature`:
scenario tags are `@givn.added` ×3 and `@e2e` ×1. The `@e2e` tag was never
removed (git history: `@e2e @wip` → `@e2e`; only `@wip` dropped on
implementation). NO finding.

**Step definition scan** — every step definition file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, no assertion reference):

```
grep -nE "\{\s*\}|unimplemented!|todo!|=> \{\}" tests/steps/*.rs
→ no empty bodies. 0 empty step bodies found across tests/steps/.
The two "{}" strings in preserve_docstrings_on_archive_steps.rs are
format-string braces inside assert!(), not bodies.
```

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Modified scenario keeps docstring | `0dd8bbc` | YES — src/gherkin/merge.rs (+ steps) |
| Added scenario keeps docstring | `b7d7b35` | Reuse — follows `0dd8bbc` (production fix), spec only |
| Modified scenario keeps data table | `9719526` | Reuse — follows `0dd8bbc`, spec only |
| @e2e Consumer archives… | `8c2c97c` | Reuse — follows `0dd8bbc`, spec only |

Reuse-only commits follow a prior commit carrying the production change
(standard TDD pattern, same as archived apply-review-fixes).

**Promised-architecture cross-check** — every component design.md said would
be created actually exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `render_scenario_stripped` docstring/table rendering | Yes | src/gherkin/merge.rs:222-247 |
| Capability step-definition file | Yes | tests/steps/preserve_docstrings_on_archive_steps.rs |
| Module registered | Yes | tests/steps/mod.rs (added `pub mod preserve_docstrings_on_archive_steps;`) |
| Merge unit tests (docstring added/modified, empty docstring, table, combined) | Yes | src/gherkin/merge.rs tests (13 pass) |

**Strict-mode proof** — present in tasks.md Setup: scenario 1 RED exited 101
("Step doesn't match any function") — undefined steps fail. `fail_on_skipped()`
at tests/features_runner.rs:20.

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Consumer archives a modified scenario and the permanent spec keeps the docstring | YES — asserts on the on-disk merged permanent spec produced by the real `givn archive` subprocess (CLI output) | N/A — CLI capability; driven via `common::run_givn` subprocess | No |

**E2e scope check** — one `@e2e` scenario per distinct user-facing action:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| preserve-docstrings-on-archive | 1 | Single action: archive a change | No |

**Duplicate-implementation check** — `verify.e2e_command` invokes
`cargo test --test features_runner -- --tags @e2e` → targets
tests/features_runner.rs (cucumber-rs). `git status` clean; no untracked
step files; the capability's e2e and non-e2e steps share the single
per-capability file (per design.md, no separate e2e file). No parallel
implementation found.

**Local environment check** — CLI-only capability; no server/browser/DB.
Local run: `cargo build && cargo test --test features_runner`. No digital
twins (no external dependencies). Clean startup;

```
cargo test --test features_runner → 125 scenarios (125 passed)
```

**E2e isolation check** — values read literally from `givn/config.yaml`:

| `verify.command` value | `verify.e2e_command` value | Identical? | Full-suite scenario count | E2e-only scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `cargo test --test features_runner` | `cargo test --test features_runner -- --tags @e2e` | No | 125 | 9 | Yes (9 < 125) |

**Design.md conformance check**:

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Merge fix location | `render_scenario_stripped` in src/gherkin/merge.rs | Same | Yes |
| Step-definition file | tests/steps/preserve_docstrings_on_archive_steps.rs, registered in tests/steps/mod.rs | Same | Yes |
| Fixture commands.yaml (verify true, e2e empty, coverage `true` + distinct dummy XMLs) | Full YAML pinned | Implemented in `write_command_overrides` | Yes |
| Fixture deltas without `@e2e` | Pinned | Fixture deltas are `@givn.modified`/`@givn.added` only | Yes |
| Single-scenario / e2e commands | `GIVN_FEATURES=…/specs cargo test --test features_runner -- --name/--tags` | Used as designed | Yes |

**Interaction coverage verification**:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| Archive a change; merged permanent spec keeps step docstrings/tables | Yes | Yes — "Consumer archives a modified scenario and the permanent spec keeps the docstring" | CLI subprocess (`common::run_givn`) | e2e given runs `common::make_init_root()` + `complete_change_with_delta`, `the user invokes` reuses subprocess seam | Yes |

**Result:** CLEAN

---

## Coverage

**Coverage result:** instrumented measurement produced via the configured
`coverage.*` commands (`cargo llvm-cov --cobertura --test features_runner`).

```
non-e2e: GIVN_FEATURES=givn/specs/ cargo llvm-cov --cobertura
         --output-path coverage/non-e2e-cobertura.xml --test features_runner
         → 125 scenarios (125 passed), line-rate 0.677
e2e:     GIVN_FEATURES=givn/specs/ cargo llvm-cov --cobertura
         --output-path coverage/e2e-cobertura.xml --test features_runner -- --tags @e2e
         → 9 scenarios (9 passed), line-rate 0.440
unit:    cargo llvm-cov --lib --test features_runner → merge.rs line coverage
         shows the new render lines (228-238, 243-245) covered
```

The new render lines are covered when the unit tests are part of the
measurement (`cargo llvm-cov --lib --test features_runner`). Under the
permanent-spec-only run (`GIVN_FEATURES=givn/specs`) they appear uncovered
because no permanent spec can exercise an archive that carries step
docstrings until THIS change's delta is merged into `givn/specs/` — the
documented chicken-and-egg this change eliminates. Post-archive, the
preserve-docstrings-on-archive scenarios (added/modified docstring + table)
exercise exactly these lines through the Gherkin runner.

## Coverage gap classification

Every uncovered region introduced or modified by this change:

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| src/gherkin/merge.rs render_scenario_stripped docstring block | 227-239 | Covered | Covered by unit tests modified/added/empty-docstring docstring cases; exercised through the Gherkin runner by this change's own scenarios after archive. No gap. |
| src/gherkin/merge.rs render_scenario_stripped table block | 241-247 | Covered | Covered by unit test `modified_scenario_keeps_step_data_table` + `step_with_docstring_and_table_renders_docstring_first`; exercised by this change's table scenario after archive. No gap. |

Remaining uncovered lines in src/gherkin/merge.rs (error branches lines
44/48-52/57-61, merge_file error paths 107-111, header edge 196/202, tag-line
215 etc.) are pre-existing, unchanged by this change, and covered by the
module's existing unit tests (`add_duplicate_fails`, `modify_missing_fails`,
`remove_missing_fails`); they are outside this change's modified regions and
were not introduced or left uncovered by it. No new line added by this
change is left uncovered.

Redundant unit tests: kept — the merge.rs unit tests exercise boundary
conditions absent from the `.feature` scenarios (empty docstring,
docstring+table render order) and are the fast RED/GREEN seam before the
change is archived into givn/specs.

---

## Test suite result

```
cargo test --test features_runner        → 125 scenarios (125 passed), 663 steps
cargo test --test features_runner -- --tags @e2e → 9 scenarios (9 passed), 40 steps
cargo test --lib gherkin::merge          → 13 passed
cargo clippy --workspace --all-targets -- -D warnings → pass
cargo fmt -- --check                     → pass
```

**Result:** PASS

**Coverage measured:** YES

---

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] All scenarios pass (`cargo test --test features_runner` exits 0).
- [x] Every coverage gap is classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation (9 < 125).
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed; no deviation.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces
      (modified `render_scenario_stripped`; no new primitives) — none
      omitted.
- [x] arc42 slug-mapping verified: arc42.md rows map to existing
      `docs/arc42/*.md` files (11 → 11-risks-and-technical-debt.md).
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
