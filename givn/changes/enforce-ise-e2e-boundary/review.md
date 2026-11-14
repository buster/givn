# Review: enforce-ise-e2e-boundary

Review kind: independent
Author: enforce-ise-e2e-boundary implementation author (omp, db765d4c)
Reviewer context: 9111d088-7854-4cfa-8bdc-08976f308d35 (fresh read-only grilling context; produced the branch coverage, the four questions, and the blocker set)

README-IMPACT: none
PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner
VISUAL-REVIEW: PASS
ARC42 CONFORMANCE: CLEAN

## Review context and scope

This is the independent review recorded by the proposal (`Review kind:
independent`). The reviewer did not author the change and received only the
artifacts, the repository state, and the finding brief. The recorded
design-review context (`9111d088-7854-4cfa-8bdc-08976f308d35`) is the one
review context the change records; this final review continues that independent
context and is not the authoring context (`db765d4c`).

Read: `proposal.md`, `design.md`, `design-review.md`, `arc42.md`, `tasks.md`,
both delta features, both delta use-case documents, `git log --name-only
1016f3bc..HEAD`, every changed production file (`src/e2e_boundary/**`,
`src/proof.rs`, `src/cli/check.rs`, `src/cli/lint/{mod,specification}.rs`,
`src/config/lock.rs`, `src/lib.rs`), the changed step files, the delta and
permanent guidance (`assets/instructions/{specs,ise}.md`), `docs/reference.md`,
`docs/glossary.md`, the affected arc42 chapters, and ADR-070.

Ran, read-only, in the first pass and again against `9b86ebe5` on 2026-11-14:

- `givn status --change enforce-ise-e2e-boundary` (88/89 boxes; the last box
  is this review);
- `givn check review --change enforce-ise-e2e-boundary` (gate output in
  "Verification" below);
- `givn lint --change enforce-ise-e2e-boundary` (clean, exit 0);
- the delta suite, `GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs
  cargo test --locked --test features_runner`: 31 scenarios (31 passed), 166
  steps (166 passed), 7.27 s;
- the permanent `check-specification-health` feature in a scratch scope
  (`GIVN_FEATURES=/tmp/perm-lint2`): 57 scenarios (57 passed), 345 steps (345
  passed), 9.16 s, proving the moved resolver did not change lint output;
- `cargo test --locked --lib e2e_boundary`: 30 passed, 0 failed, including the
  target-table, anchor, source-root, import-form, and escape tests;
- `cargo clippy --locked --all-targets -- -D warnings` (clean) and
  `cargo run -- instructions` (exit 0);
- `givn instructions specs` and `givn instructions ise` on this repository
  (`ise` off).

The re-audit probes were rerun against the current binary in fresh `/tmp`
fixtures. The first-pass run used the pre-closure binary; the closure results
below are from the current tree. It edited no production code, step
definition, spec, design, task, glossary, or README file.

## Tasks preflight

`givn status --change enforce-ise-e2e-boundary` reports **88/89** boxes. Every
Setup, scenario, and Closeout box is checked; the only unchecked box is
`givn check review --change enforce-ise-e2e-boundary passes`, which this review
completes.

The Closeout section (`tasks.md` `## Closeout`) records each command with its
real output: `cargo fmt --all -- --check` exit 0, clippy exit 0, `cargo nextest
run` 1622 passed, permanent `./run-tests.sh --non-e2e` 963 passed with 111
skipped, permanent `./run-tests.sh --e2e` 111 passed, the delta scopes 31
passed and 0 end-to-end, `givn lint` clean, and `givn check tasks` 31 scenarios
at `32256f5f`. The receipt in `verification/tasks-verification.json` is that
run (`head: 32256f5f`, 31 passed, 0 failed, 0 skipped). F0 is closed. My
independent reruns of the delta scope (31/31) and the permanent
`check-specification-health` feature (57/57) agree.

## Implementation evidence check

### 1. `@e2e` tag integrity

No delta scenario carries `@e2e`; the design says the delta adds no end-to-end
scenario. `grep -rn '@e2e' givn/changes/enforce-ise-e2e-boundary/specs/`
returns only narrative references in the two use-case documents. No permanent
`@e2e` tag was removed: the delta's two `@givn.modified` scenarios keep their
tags, and no delta file edits a permanent feature. 0 tag removals.

### 2. Empty or trivial step bodies

0 empty, whitespace-only, `todo!()`, `unimplemented!()`, or assertion-free step
bodies found across the 92 files under `tests/steps/`. The single `todo!()` hit
(`tests/steps/outline_report_support.rs:581`) is a Rust source fixture string
passed to the outline analyzer, not a step body. The new bindings are in
`tests/steps/workflow_enforcement_steps.rs:3530-4050` and
`tests/steps/prose_consolidation_steps.rs:480-720`; two delta phrases reuse
existing bindings (`specification_health_steps.rs:288` for the dependency-edge
Then, `archive_proof_steps.rs:742` for `stderr names "..."`), so no phrase is
registered twice.

### 2a. Characterization

`CHARACTERIZATION` is not used. This is a normal behavior change.

### 2b. Discrimination

`tasks.md` records observed RED for `receipt-identity` and `receipt-freshness`,
and revert-based RED (uncommitted production reverted with `git stash`) for the
three `e2e-scope` scenarios. The five `boundary-resolution` scenarios record
"RED admitted (steps TESTS.5)" because the resolver was implemented before the
bindings; no historical RED is claimed for them. I did not mutate the reviewed
tree. I did confirm the steps are not no-ops: the probes below produced the
exact refusal lines and receipt fields the steps assert, so each step fails
when that production path is absent. The three `served-policy` scenarios and
the two `pinned-policy-replacements` scenarios assert committed prose through
the served instruction; they pass on their first run by design (the design
records this drift-protection role), and they are not RED evidence.

### 3. Promised components

| Component (design) | As-built | Check |
|---|---|---|
| `E2eBoundary` (`resolve`, `conditions`, `identity`) | `src/e2e_boundary/{mod,conditions,manifest,imports,paths,hash}.rs` | present; `E2eBoundary::resolve` at `mod.rs:34`, `conditions` at `mod.rs:296`, `identity` at `mod.rs:246` |
| `ChangeReceipt` (`write`, `read`, `freshness`) with `E2eIdentity`, `ScopeOutcome` | `src/proof.rs:542-586`, `write_change_receipt` at `proof.rs:1204`, `freshness` at `proof.rs:880` | present |
| Blocking check integration | `src/cli/check.rs:469`, `581-687` | present |
| Read-only effectiveness helper | `src/config/lock.rs:126` | present; no schema change |
| Shared resolver consumed by lint | `src/cli/lint/specification.rs:165` | present; `src/cli/lint/manifest.rs` deleted and the hash moved to `src/e2e_boundary/hash.rs` |

Consumers import the two contracts only. `cli::lint`, `cli::check`, and
`proof` import `e2e_boundary`; no `e2e_boundary` module imports `proof` or a
`cli` module, so the dependency is one-way. No dead code: clippy with
`-D warnings` is clean and the `pub(crate)` surface is consumed.

### 4. Strict-mode proof

`tasks.md` Setup records the real-runner probe: a scratch `@e2e` scenario with
one unbound step exits 1 and writes `{"failed":1,"passed":0,"scope":"e2e",
"skipped":0,"total":1}`. Strict mode is present and exercised. The delta also
asserts distinct counts for the two runtime cases (`1 failed test` and
`1 skipped test`), so one generic message cannot satisfy both.

### 5. `@e2e` Then assertions

The delta contains no `@e2e` scenario (0 of 19). The permanent `@e2e` scenarios
that share these actions are outside the delta and unchanged; no downgraded
Then assertion exists in the delta. Not applicable, no finding.

### 6. Browser-UI driving

Not a browser capability. The product is a CLI, every delta scenario drives
the built binary as a subprocess, and no step sends HTTP or calls `fetch()`.

### 7. The exact E2E implementation

`verify.e2e_command` is `./run-tests.sh --e2e`. `run-tests.sh:78` runs
`GIVN_FEATURES="$FEATURES_PATH" run_cucumber '@e2e and not @givn.removed'`,
which invokes `cargo test --locked --test features_runner -- --tags ...`; the
only E2E harness is `tests/features_runner.rs`. I searched the tree for a
second implementation of the boundary's E2E steps: the delta adds none, and
`tests/steps/{workflow_enforcement_e2e_steps,prose_consolidation_e2e_steps}.rs`
exist with their existing content. No parallel, stronger implementation was
ignored.

### 8. E2E scope and the interaction inventory

The delta adds no inventory entry. The two owning documents hold 13
(`verify-change`) and 9 (`create-change`) interaction rows, the design matrix
has the same 22 rows, and each matrix target title resolves to a permanent
scenario carrying `@e2e` (verified in the owning files; the tasks pre-flight
records the same cross-check). The delta's boundary refusals, receipt
inspections, and guidance cases are variants of those actions, so 0 new
actions and 0 excess E2E scenarios.

### 9. Explicit scope selection

The check passes `GIVN_RESULT_SCOPE=change` for the regular run
(`check.rs:502-507`) and `GIVN_RESULT_SCOPE=e2e` for the E2E run
(`check.rs:626-631`); `run-tests.sh:20-40` honors it, and the receipt records
`result.scope: "change"` plus the `e2e_scope` outcome. The receipt read is
fail-closed on failed, skipped, zero, malformed, unreconciled, and wrong-scope
results.

### 10. Design against the as-built tree

| Design statement | As built | Verdict |
|---|---|---|
| Receipt fields `e2e_identity` and `e2e_scope` are optional and not serialized when absent (`design.md:288`) | `proof.rs:578-584` with `#[serde(default, skip_serializing_if = "Option::is_none")]`; field order otherwise unchanged | matches |
| `not_applicable` only without a delta `@e2e`, green scope otherwise (`design.md:211`, `:430`) | `check.rs:523-538`; `proof.rs:980-1010` | matches |
| The runner rule is textual and proves naming, not execution (`design.md:214`) | `e2e_boundary/mod.rs:369-381`; refusal text names the command and root | matches |
| The boundary preflight runs before the no-behavior-change return (`design.md:399-403`) | `check.rs:459-460` calls `preflight_e2e_boundary` before `change_verification_not_applicable`; the no-receipt branch of `check_change_receipt` (`check.rs:700-703`) calls it too | matches (closed in `3b2c293d`) |
| The dependency graph covers direct, transitive, and workspace-inherited edges (`design.md:358`, `docs/reference.md:146-160`) | `manifest.rs:128-145` reads the top-level, dev, build, and every `[target.<cfg>.*]` table; `mod.rs:292-315` refuses a directory that is a source root, lies inside its literal anchor, or contains a path the pattern names | matches (closed in `a072aa99`) |
| Step Definitions: every delta phrase is bound in `workflow_enforcement_steps.rs` / `prose_consolidation_steps.rs` (`design.md` Step Definitions) | two phrases reuse permanent bindings in `specification_health_steps.rs` and `archive_proof_steps.rs` | wording drift only (O3), no behavior gap |

### 11. Interaction coverage cross-reference

Every inventory entry has a design matrix row, an existing `@e2e` scenario, and
a real CLI driving mechanism (a `givn` subprocess). The full mapping is the 22
rows of `design.md`'s Interaction Coverage Matrix; the tasks pre-flight table
reproduces it, and I re-verified that all 22 titles exist and carry `@e2e` in
`givn/specs/**`. No gap, no extra row, no changed driver.

## Arc42 implementation conformance

I re-derived the affected chapters from the implementation before reading
`arc42.md`: 02 (constraints), 04 (strategy), 05 (building blocks), 06 (runtime),
08 (cross-cutting), 09 (decisions), 11 (risks), and 12 (glossary link) are
affected; 01, 03, 07, and 10 are not.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Receipt binds the E2E identity and scope | `docs/arc42/02-architecture-constraints.md:53` (TC-39) | Yes | `design.md:76`, `:280-310` | batch `receipt-identity`, `e2e-scope` | `proof.rs:542-586`, `check.rs:523-573` | yes |
| Blocking boundary and advisory counterpart | `02:54` (TC-45) | Yes | `design.md:165-250` | batch `boundary-resolution`, `runner-target` | `check.rs:650-687`, `lint/specification.rs:165-177` | yes |
| Change-verification strategy row | `docs/arc42/04-solution-strategy.md:23` | Yes | `design.md:20-30` | all batches | `check.rs:457-573` | yes |
| Addons strategy row | `04:30` | Yes | `design.md:14`, `:170-190` | `config/lock.rs:126` | `ise_effective_for_change` | yes |
| `e2e_boundary` building block | `docs/arc42/05-building-block-view.md:83` | Yes | `design.md:96-150` | batch `boundary-resolution` | `src/e2e_boundary/**` | yes |
| Check flow: preflight, regular scope, E2E scope, receipt | `docs/arc42/06-runtime-view.md:263-280` | Yes | `design.md:350-390` | batches `boundary-resolution` to `e2e-scope` | `check.rs:469-573`; B2 is the no-behavior exception | B2 |
| Review reads the receipt and runs no suite | `06:328-331` | Yes | `design.md:418-443` | batch `receipt-freshness` | `check.rs:692-744` | yes |
| Receipt concept includes the E2E identity | `docs/arc42/08-crosscutting-concepts.md:666-670` | Yes | `design.md:280-310` | `receipt-identity`, `receipt-freshness` | `proof.rs:542-586`, `:960-1085` | yes |
| ADR-070 amended under `change-proof/v1` | `docs/arc42/09-architecture-decisions.md:25`; `adr/ADR-070-forward-only-archive-proof.md:318-379`; `adr/README.md:71` | Yes | `design.md:165-250` | `receipt-identity` | `proof.rs:556-584`, `:1215-1245` | yes |
| R-138 runner-target limit | `docs/arc42/11-risks-and-technical-debt.md:125` | Yes | `design.md:214`, `:678` | `runner-target` | `e2e_boundary/mod.rs:369-381` refusal names the root | yes |
| R-139 import-reader depth | `11:126` | Yes | `design.md:696` | `boundary-resolution` | `imports.rs:1-9`, refusal names the target | yes |
| R-140 in-flight rerun, R-141 broad entry | `11:127-128` | Yes | `design.md:690-706` | `receipt-freshness` | `proof.rs:972`, `:1011-1023` | yes |
| Chapter 12 stays a link | `docs/arc42/12-glossary.md` | Yes | n/a | n/a | link to `docs/glossary.md`, no table | yes |

ARC42 CONFORMANCE: CLEAN

The chapter files exist and carry decision-specific content at the cited lines;
no box-drawing or pseudo-diagram characters were found in the changed
chapters. The one non-matching row is the implementation deviation B2, which is
recorded as a finding rather than silently accepted.

## Ubiquitous language conformance

The glossary already holds the terms this change uses: `End-to-end (E2E) test`
(`docs/glossary.md:72`), `E2E identity` (`:73`), `Change verification receipt`
(`:50`), `Verification scope` (`:54`), `Tested content` (`:53`), and
`Source code` (entry above the E2E group). The delta specs, design, code
symbols (`E2eBoundary`, `E2eIdentity`, `ChangeReceipt`, `ScopeOutcome`), and
served guidance use those terms; the rejected "unit" noun from the seed is
absent, and the design-review records why `package` was chosen. No unrecorded
term is introduced and no old term is left with a second definition.

`docs/domain-model.md` is unchanged (`git diff 1016f3bc..HEAD -- 
docs/domain-model.md` is empty). The change adds an evidence field and an
internal resolver; it adds no bounded context, aggregate, or domain
relationship, so the structural impact is none and the model correctly needs no
edit.

## Coverage

No measured coverage evidence exists yet for this change: `verification/`
holds only the runner result and the change receipt, and the archive receipt
will carry the Cobertura report. The classification below is from inspection;
the archive measurement will confirm or reopen it.

Per-scenario classification (all 19 executed green in the delta scope):

| # | Delta scenario | Exercised production path | Classification |
|---|---|---|---|
| 1 | declared `e2e` role resolves no readable package (3 rows, incl. `e2e: []`) | `E2eBoundary::resolve`, `MissingPackage`, `MalformedManifest`, the empty-role guard | covered |
| 2 | package reaches a source package (4 rows, incl. target-specific) | `dependency_tables`, `dependency_edges`, BFS, `source_root_reached_by`, `DependencyEdge` | covered |
| 2a | no behavior change still needs a readable declaration | preflight before the not-applicable result in check and review | covered |
| 3 | `e2e` path under a source root | `PackageUnderSource` | covered |
| 4 | package contains a source root | `SourceInsidePackage` | covered |
| 5 | path resolves no package and imports source | `import_edges`, `ImportEdge` | covered (the syntax branches are unit-tested) |
| 6 | runner command names no root (3 rows) | `names_root`, `RunnerNamesNoPackage` | covered |
| 7 | unbound E2E step | `run_change_e2e_scope`, result read, count wording | covered |
| 8 | skipped E2E step | same path, skipped count | covered |
| 9 | receipt binds two packages, runner, version | `E2eBoundary::identity`, `E2eIdentity`, `ScopeOutcome::passed` | covered |
| 10 | delta without `@e2e` records not applicable | delta re-read, `ScopeOutcome::not_applicable` | covered |
| 11 | identity no longer matches (5 rows) | `e2e_stale_reasons` (5 variants) | covered |
| 12 | scope no longer matches (2 rows) | delta-aware scope rules | covered |
| 13 | `ise` off keeps today's receipt | `preflight_e2e_boundary` `None`, `skip_serializing_if` | covered |
| 14-15 | two modified pinned policy scenarios | served `specs` render | covered (drift protection) |
| 16-17 | declared package policy; served `ise` guidance | served `specs`/`ise` render | covered (drift protection) |
| 18 | `ise` off allowance | `{{else}}` render | covered |

Gaps, by bucket, as of 2026-11-14:

- **Bucket 2 (missing test coverage):** none open. B1's target-table and
  anchor rules, F1's import branches, and F2's empty-role guard now have
  committed tests (see "Findings and closures"). The `cargo test --locked
  --lib e2e_boundary` run is 30 passed, 0 failed.
- **Bucket 1 (dead code):** none found; clippy with `-D warnings` is clean and
  the resolver surface is consumed by lint, check, and proof.
- **Bucket 3 (legitimately hard to test):** no bucket-3 claim is used. The SCM
  error branches of `identity()` are fail-closed through `?` and are exercised
  by the existing SCM failure tests, not counted as gaps.


## Visual review

`design.md` declares `Interface: terminal`, no viewports, no states, and a
product with no rendered surface. The delta adds no `@e2e` scenario, so the
transcript rule collects nothing and no evidence file is expected. The terminal
baseline is the served refusal text and the receipt JSON, both asserted by the
regular delta scenarios, and the baseline holds: every refusal names its
condition and subject, no Rust identifier appears in user-facing output, and
the receipt uses the domain field names. `VISUAL-REVIEW: PASS`.

## Persona disposition

| Persona | Disposition | Evidence |
|---|---|---|
| `developer--primary-user` | confirmed; reads the refusals and the receipt | proposal `Confirmed Personas`; design Persona impact table; boundary and receipt scenarios |
| `agent--technical-role` | confirmed; reads the served policy and stale reasons | same; guidance and freshness scenarios |
| `givn-maintainer--owner` | confirmed; owns the declaration and runner command | same; runner-target and identity scenarios |

No Persona is invented. Gherkin names `the developer` and `the author`, which
are interaction roles, never Personas. A user-visible terminal surface
therefore has confirmed Personas.

## README impact

The decision is `README-IMPACT: none`. The README's only statement about this
surface is the generic gate bullet (`README.md:27`): `givn check tasks`, `givn check review`,
and `givn archive` verify receipts and sign-off before a change lands. That
bullet remains true. The README documents neither the `ise` addon nor the
change-verification receipt fields, and the detailed conditions, receipt
fields, and limits are documented in `docs/reference.md:146-160` and `:277-310`,
which this change updated. No named README section is falsified.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `An ise-effective change receipt whose E2E identity no longer matches the project is stale for review` | `An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review` | boundary |
| `Canonical policy accepts the library public API as its real interface` | `Canonical policy keeps the in-process allowance while ise is off` | variant |

Rationale: the two stale Outlines share a shape but assert different rules
(identity-value comparison versus delta-aware scope applicability), so merging
them would lose a distinct failure. The permanent library scenario and the
delta ise-off scenario are the two render branches of the same action and are
each other's negative control, so neither supersedes the other. Net delta: 17
added, 2 modified, 0 removed. The two `@givn.modified` scenarios supersede the
permanent titles they replace; no removed+added pair exists.

## Split-or-keep

No delta scenario exceeds the length limit; `givn lint --change` reports 0
`LONG` findings. No split-or-keep row is required.

## Deferrals

None. No mandatory check is deferred and no operator decision is recorded.

## Findings and closures

Each finding below was opened in the first review pass (`REVIEW: FAIL` at
`8dbb80ab`) and re-audited against HEAD `9b86ebe5` on 2026-11-14. Behavioral
closures were probed with fresh `/tmp` fixtures against the current binary;
the code closures were read at the cited lines.

### B1 (closed 2026-11-14, `a072aa99`): target-specific Cargo dependency edges

Original finding: the resolver read only the three top-level tables, so a
direct `[target.'cfg(unix)'.dependencies]` path edge into `source` was
accepted.

Closure: `manifest::dependency_tables` (`manifest.rs:128-145`) reads the three
top-level tables and every `[target.<cfg>.*]` variant. `source_root_reached_by`
(`mod.rs:292-315`) refuses a target that is a `source:` root, lies inside the
root's literal anchor (`Glob::anchor`), or contains a path the pattern names.
`docs/reference.md:146-160` states both rules. Unit tests:
`a_target_specific_dependency_table_is_read`,
`a_target_specific_dependency_into_source_is_refused`,
`a_glob_shaped_source_root_reaches_the_directory_it_anchors_in`,
`a_dependency_that_contains_a_source_root_is_refused`,
`a_dependency_outside_every_source_root_is_admitted`,
`the_anchor_is_the_literal_directory_a_pattern_starts_in`; the refusal Outline
gained the `target-specific` row.

Independent probe against the current binary (fresh fixture:
`tests/e2e/Cargo.toml` with
`[target.'cfg(unix)'.dependencies] product = { path = "../../src" }`,
`source: ["src/**"]`):

```text
$ givn check tasks --change ise-boundary
error: the declared e2e boundary cannot be accepted:
  - tests/e2e: e2e package 'e2e-tests' has a dependency edge into source (root 'src/**') through package 'product'
EXIT=1
```

The top-level control is refused identically. The glob-shaped source variant
(`source: ["src/**/*.rs"]`, top-level path edge) is also refused, now with
`root 'src/**/*.rs'`. Closed.

### B2 (closed 2026-11-14, `3b2c293d`): no-behavior-change bypass

Original finding: `run_change_verification` returned on
`change_verification_not_applicable` before the boundary preflight, and
`check_change_receipt` did the same, so an `ise`-effective change could skip
the boundary by declaring no behavior change.

Closure: `check.rs:459-460` now calls `preflight_e2e_boundary` before
`change_verification_not_applicable`; the not-applicable branch of
`check_change_receipt` (`check.rs:700-703`) calls the preflight because it
writes no receipt to age. A delta scenario drives both commands.

Independent probe (fresh fixture: `ise: true` lock, `Behavior change: none`,
`.givn-skip` with `specs`, no `givn/specification.yaml`):

```text
$ givn check tasks --change ise-none
error: the declared e2e boundary cannot be resolved: 'givn/specification.yaml' is absent
EXIT=1

$ givn check review --change ise-none
error: review checks failed:
  the declared e2e boundary cannot be resolved: 'givn/specification.yaml' is absent
EXIT=1
```

The design-review task's recorded revert RED (`expected failure, got exit
Some(0)`) matches the code path. Closed.

### F0 (closed 2026-11-14, `7788a061` and `384f0e61`): Closeout records

Original finding: 7 Closeout boxes were unchecked and the commands were not
recorded. The current `givn status` reports 88/89; the only unchecked box is
this review. `tasks.md` `## Closeout` records fmt (exit 0), clippy (exit 0),
nextest (1622 passed), the permanent non-e2e scope (963 passed, 111 skipped),
the permanent e2e scope (111 passed), the delta scopes (31 passed and 0
end-to-end), lint (clean), and `givn check tasks` (31 scenarios at
`32256f5f`). The receipt carries that run (`head: 32256f5f`, 31 passed, 0
failed, 0 skipped). Closed.

### F1 (closed 2026-11-14, `a072aa99`): import-reader branches

Original finding: the `from ... import`, quoted `require`/`import`,
`self`/`super`, and escape-above-root branches had no committed test.

Closure: `imports.rs` tests
`a_python_from_import_names_its_module_and_is_refused`,
`a_quoted_require_or_import_path_is_refused`,
`a_self_import_resolves_from_the_file_directory`,
`a_super_import_walks_up_from_the_file_directory`, and
`an_attributed_module_declaration_is_admitted` (the admitted `#[path]` form,
recorded as the R-139 limit). `manifest.rs`
`a_path_dependency_may_not_escape_the_project_root` covers the shared escape
guard. `cargo test --locked --lib e2e_boundary` is 30 passed, 0 failed. Closed.

### F2 (closed 2026-11-14, `3b2c293d`): empty-role refusal test

Original finding: the `e2e: []` guard had no test. Closure: the first refusal
Outline gained the `lists no path` row (`e2e: []`), and the Then asserts the
empty-resolution sentence. Independent probe with `e2e: []`:

```text
$ givn check tasks --change ise-boundary
error: the declared e2e boundary cannot be accepted: the 'e2e' role lists no path; the change check requires a resolved E2E package
EXIT=1
```

Closed.

### F3 (closed 2026-11-14; restore `0415ffa3`, delta step `3b2c293d`): shared step strictness

Original finding: the shared `stderr names "..."` step had been loosened from
a contiguous substring to words in order on one line, weakening 25 permanent
assertions.

Closure: the shared body at `tests/steps/archive_proof_steps.rs:742` is again
exactly `assert!(stderr.contains(&needle), ...)`. The delta binds its own
`stderr reports the stale reason "..."` step at
`tests/steps/workflow_enforcement_steps.rs:4089`, and the two stale Outlines
use it (delta feature lines 123 and 140). I read both bodies, so the
restoration is strict by construction, not by the reported count. Closed.

### Minor observations (no action requested)

- `RunnerResult::read_counts` (`src/proof.rs:511-524`) changed its shared
  failure wording from `declares N failed scenario(s)` to `reports N failed
  test(s)`. This alters the archive refusal text, is not named in `design.md`
  or `docs/reference.md`, and no permanent test pins the old wording. The
  delta's count assertions need the new wording. This is a documentation nit
  routed to the next change; no behavior claim in this change depends on it.
- `ScopeOutcome::not_applicable()` serializes the zero count fields
  (`total`, `passed`, `failed`, `skipped`, `seconds`) as well as `status`
  (`proof.rs:408-413`, `:381-394`). The tasks' wording `{ "status":
  "not_applicable" }` is a shorthand, and the shape is pre-existing, so no
  action is needed.
- `E2eBoundary::identity` iterates `backend.content_paths()` without an
  explicit sort (`mod.rs:246-262`). Git and Fossil both return sorted entries,
  so the digest is stable today; an explicit sort would make it independent of
  the backend.
- `names_root` (`mod.rs:369-381`) accepts a token containing `<root>/`
  anywhere, so `mytests/e2e/` names `tests/e2e`. R-138 discloses the class
  (naming is not execution); this is a concrete instance to keep in mind when a
  target marker replaces the textual rule.

## Verification

`givn check review --change enforce-ise-e2e-boundary` ran against this
revision before the review commit and exits 0. The receipt is the closed
candidate at `32256f5f` (31 scenarios), reused across the documentation-only
review revision. Review-relevant output:

```text
givn lint: 2 file(s) checked — clean
net delta: 17 added, 2 modified, 0 removed
  ✓ overlap dispositions passed
  ✓ scenario commits passed (19 scenario(s))
  ✓ change verification receipt reused across documentation-only revision ... (tested at 32256f5f..., 31 scenario(s))
  ✓ scenario commits passed (19 scenario(s))
```

Exit code 0. Every review check passes: lint, overlap dispositions,
capability ownership, scenario commits, receipt freshness, integrity (including
the verdict, README impact, persona disposition, and visual markers), and the
change-scoped lock check.

Delta scope: 31 scenarios (31 passed), 166 steps (166 passed). Permanent
`check-specification-health`: 57 scenarios (57 passed), 345 steps (345
passed). `cargo test --locked --lib e2e_boundary`: 30 passed. Clippy clean.
Run gate: `cargo run -- instructions` exit 0.

## Sign-off checklist

- Fabrication audit: clean for the steps' production dependency; 0 empty step
  bodies; the served-policy scenarios are disclosed drift protection.
- Every checked task has a verified commit touching production code (the review
  gate reports 19 scenarios with revisions).
- Every promised component exists: `E2eBoundary`, `ChangeReceipt`, the check
  integration, the lock helper, and the lint consumer.
- Every consumer uses only its declared contract; the dependency is one-way.
- Strict-mode proof present and passing.
- Scenario execution evidence: the change receipt records 31 change-scope
  scenarios; the E2E scope is `not_applicable` because the delta has no `@e2e`
  scenario. The archive receipt will carry the measured coverage.
- Coverage classification uses inspection (no measured report yet); every gap
  is in bucket 1, 2, or 3 and none is open.
- Dead code deleted (none found); the missing tests are added (B1, F1, F2); no
  hard-to-test bucket-3 claim is used.
- No redundant unit test was added for behavior already covered by a scenario.
- No `@wip` tags remain in the delta and no implementation-layer detail is in
  the spec.
- Canonical E2E policy: the delta adds no `@e2e` scenario and no normalized
  action; every permanent action has one `@e2e` scenario.
- Browser/HTTP shortcuts: not applicable (CLI).
- The run gate starts the declared CLI cleanly.
- The exact `verify.e2e_command` files were identified and read; no parallel
  implementation exists.
- The E2E scope is selected explicitly; the receipt records the regular scope
  and the delta-aware E2E outcome.
- Implementation matches design: the boundary resolves before the
  no-behavior-change result (`3b2c293d`), the dependency read covers every
  table and the source-root comparison covers the tree (`a072aa99`), and the
  shared step is strict again (`0415ffa3`).
- Interaction coverage verified: 22 inventory entries, 22 matrix rows, 22
  permanent `@e2e` titles.
- No finding is excused with an invented fourth bucket.

Every finding from the first review pass is closed and verified above. The two
new behavioral probes (B1 target table, B2 bypass) are refused by the current
binary, the closeout evidence is recorded, and the one remaining minor
observation (O1, the archive failure wording) is a documentation nit routed to
the next change. The verdict is PASS.

REVIEW: PASS
