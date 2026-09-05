# Design: coverage-merge-union

## Product change: the built-in legacy merge

`crate::cobertura` (src/cobertura.rs) is the module that parses Cobertura XML,
validates reports, and merges them for the legacy two-command coverage
configuration (src/cli/coverage_gate.rs:85-86 →
`write_legacy_merged_report` → `merge_legacy_reports`).

Current defect: `merge_legacy_reports` (src/cobertura.rs:103) adds both
reports' `lines_covered`/`lines_valid` counters. Both reports measure the
same source tree, so the denominator double-counts every line.

### Data model change

`CoberturaReport` gains one field:

```rust
/// Per-file hit counts keyed by line number: file → (line number → hits).
/// Required to compute a line union when merging two reports of the same
/// tree; aggregate counters alone cannot distinguish a shared line from
/// two distinct ones.
pub line_hits: BTreeMap<String, BTreeMap<u64, u64>>,
```

The XML deserializer already walks every `class/lines/line` element for
`file_lines()`; `CoberturaLine` additionally deserializes the required
`number` attribute (`@number`), and the map is assembled in the same pass.
A Cobertura `line` element's `number` attribute is mandatory per the
Cobertura DTD, so the field is non-optional; a report carrying line
elements without numbers fails parsing with the existing context wrapper
(generic "failed to parse" — observable, not this change's contract).
Reports whose `<packages />` tree carries no line elements deserialize
unchanged (`line_hits` empty; pinned by the existing
`parse_tolerates_packages_without_classes` test).

### Merge contract

```rust
pub(crate) fn merge_legacy_reports(a: &CoberturaReport, b: &CoberturaReport) -> CoberturaReport
```

- **Union when both reports carry line data** (both `line_hits` maps
  non-empty): for every (file, line number) present in either report, the
  merged map holds one entry whose hits are the sum of the reports' hits —
  intensity combines, identity is what matters for the counters. When
  several classes share a filename (multiple `class` elements with the same
  `filename`), hits for the same (file, line) sum as well.
- **Aggregate fallback**: if either report has no line data (empty map),
  fall back to the current aggregate summation for all four counters and
  leave `line_hits` empty. Rationale: aggregate-only source reports are the
  shape of every existing legacy fixture; a union would yield a 0/0 merged
  report that `validate` rejects, blocking archives for exactly the legacy
  consumers this change must keep working. Documented in the function doc
  comment.
- Under the union branch: `lines_valid` = number of entries in the merged
  map; `lines_covered` = entries with hits > 0; `line_rate` = covered /
  valid (recomputed, consistent with `validate`).
- Branch counters: unchanged summation in both branches. Branch identity
  (condition outcomes) is not retained by the parser, so a branch union is
  not computable without a new measurement contract; this limitation lives
  in the function's doc comment. `branch_rate` stays consistent with the
  summed branch counters.
- `file_lines` (the per-file aggregate used by `assert_distinct`) is
  recomputed from the merged line map so it stays consistent with
  `line_hits`.

`assert_distinct` keeps its current semantics (it compares the two SOURCE
reports before merging; unchanged). No permanent scenario pins the legacy
sum output (the two permanent extract-coverage-gate archive scenarios use
the script path), so the correction breaks no pinned expectation.

### Fixture reports (scenario-driven)

The scenarios write the two source reports through the legacy commands with
controlled line data; the commands are inline shell (`sh -c` heredoc `cat`
into the configured non-e2e/e2e paths, env vars
`GIVN_COVERAGE_NON_E2E_PATH`/`GIVN_COVERAGE_E2E_PATH`, src/subprocess.rs:50-54).
One class, one file (`src/lib.rs`), roots arithmetically consistent for
`validate` (line-rate within 0.01 of covered/valid; branches 0/0 skip the
branch check):

- Scenario 1 ("measured by both reports"): report A hits lines 1,2 and
  misses 3,4 (2/4, line-rate 0.5); report B hits all of 1,2,3,4 (4/4,
  line-rate 1.0). Both reports measure the same four lines — the crux —
  and are distinct (`assert_distinct` passes: 2 ≠ 4 covered). Union:
  4/4. The defective sum would report **6/8**.
- Scenario 2 ("only one report measures"): report A hits 1,2 and misses 3
  (2/3, line-rate 0.67); report B hits 2 and misses 4 (1/2, line-rate 0.5).
  Union: valid {1,2,3,4}, covered {1,2} → 2/4. The defective sum would
  report 3/5.

Both scenarios fail genuinely in RED against the current sum
implementation (asserting 4/4 and 2/4 while it produces 6/8 and 3/5).

## Repository script change: merge-coverages.sh

The repo-owned script merges per-class line nodes correctly (union by line
number, hits summed) and recomputes class/package rates — but the root
`<coverage>` totals are naive sums of the two reports' counters (the
`total()` helper). Fix, after `merge_packages()`:

- Recount the root line totals from the merged tree with the same walk
  `refresh_rate` uses: `package/class/lines/line` only — never
  all-descendants, because class elements also carry `<methods>` blocks
  with their own `<lines>` (the real report holds 42817 line nodes but
  only 16598 class-level ones).
- Key the recount by distinct (filename, line number); currently zero
  duplicate filenames exist across classes, but the invariant is stated.
- Fallback: when either input report lacks `<packages>` (the
  aggregate-only shape), keep the current summed totals — a recount would
  write 0/0 and fail the gate's `validate`.
- Branch totals keep the current summation (same documented limitation as
  the product merge; both current reports carry 0/0 branches).

## Step definitions

New and reused steps live in the existing capability file
`tests/steps/extract_coverage_gate_steps.rs` (one file per capability; the
capability is `extract-coverage-gate`); no new GivnWorld fields are needed
(new givens reuse `world.env`):

- New givens: `a givn project with legacy coverage commands whose reports
  measure the same lines` / `... whose second report measures lines the
  first does not` — one parameterised helper writes commands.yaml with the
  legacy command schema (`measure_command`/`merge_command` at the
  `givn missing-coverage` sentinels plus `non_e2e_command`/`e2e_command`,
  accepted by `Coverage` deserialization, src/config/manifest.rs:232-239)
  and the two inline shell commands emitting the fixture XMLs.
- Reused: `a completed change "<id>" with all required artifacts`
  (extract_coverage_gate_steps.rs:150), `the project declares a complete run
  declaration` (run_declaration_fixtures_support.rs:47), `the user runs
  \`givn archive --change <id>\`` (mod.rs:411), `the command exits 0`
  (mod.rs:434).
- New then: `the merged Cobertura report contains line coverage `<c>/<v>``
  (regex), asserting the merged report's root attributes
  (`lines-covered="4" lines-valid="4"`; attribute order fixed at
  coverage_gate.rs:235).

## Test runner

```sh
./run-tests.sh
```

The suite is scoped with `GIVN_FEATURES` (default `givn/specs`); the change
delta runs scoped:

```sh
GIVN_FEATURES=givn/changes/coverage-merge-union/specs ./run-tests.sh
```

## Single-scenario run command

```sh
GIVN_FEATURES=givn/changes/coverage-merge-union/specs \
  cargo test --no-default-features --features spec-retrieval \
  --test features_runner -- --name "A line measured by both reports is counted once in the merged report"
```

(The runner is `harness = false`, so the trailing args reach cucumber;
`.fail_on_skipped()` enforces strict mode.)

## Strict mode

`cucumber-rs` `.fail_on_skipped()` (tests/features_runner.rs) — existing
proof, unchanged; the delta reuses existing steps plus fully implemented new
ones (no empty bodies).

## Version freshness

No new runtime, framework, or library. `quick-xml`/`serde` are already
pinned by the repository lockfile; no version decision is made here.

## E2E smoke test infrastructure

No new `@e2e` scenarios. The action "archive a change with coverage
configured" already has its @e2e scenario in the permanent spec
(extract-coverage-gate.feature: "Consumer archives a change with coverage
enabled and the coverage summary appears in README"); the delta scenarios
exercise the same CLI action as regular scenarios (inventory comment in the
delta file records this zero-new-interactions decision). The interaction
gate scans only the permanent tree's group.md against @e2e counts; zero new
@e2e scenarios and zero new declared interactions leaves parity untouched.

## Local runnability

Unchanged: `./run-tests.sh` runs the full stack. The scenario fixtures are
temp projects; no containers, services, or external dependencies.

## Repository-script verification (no scenario possible)

merge-coverages.sh is project-owned and runs only in this repository's own
archive gate; fixture projects use their own scripts, so no scenario can
drive this repo's copy. Verification is the regenerated repository report:
after the fix, the merged report's root line-rate must equal the class-level
union rate computed from the same file — 6946/16598 ≈ 41.8%, closing the
37.6% (sum) vs 41.8% (union) gap recorded in
givn/archive/harness-binary-stability/review.md.
