# Tasks: enforce-ise-e2e-boundary

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`). Strict mode: `.fail_on_skipped()` on both cucumber-rs
builders in `tests/features_runner.rs` (lines 55 and 155); an undefined or
skipped step fails the run. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (design.md, "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs \
  cargo test --locked --test features_runner -- --exact "<usecase>/<capability>.feature::<title>"
```

`<usecase>/<capability>.feature` is the path under this change's `specs/`.
`<title>` is the scenario title without its `@wip` tag; for an Outline, use the
Outline title, and the runner executes every Examples row.

Release note (every batch commit subject, verbatim from `proposal.md`, without
backticks):

```text
With ise effective, a change check refuses a declared E2E package that is missing, links production source, or is not named by the E2E runner command, and binds that package, the runner command, and the running Givn version into the change verification receipt; the canonical @e2e policy now states the same package boundary.
```

Batch commit rule: one atomic `givn commit` per batch with explicit paths. The
subject is the release note; the body is `Batch: <batch-name>`, plus the
scenario title for a single-scenario batch. Every scenario of the batch records
the same revision as `COMMIT: <hash> -- Batch: <batch-name>`. A commit contains
the batch's implementation or its regression test. `@wip` is removed from a
scenario when its batch starts, never in bulk; `givn check tasks` refuses the
change while any selected delta scenario still carries `@wip`.

**Evidence fields.** Per the served `tasks` instruction, no separate
TEST-FIRST, RED, or TEST-ORDER field is authored here. Each implementation
checkbox carries the slot the implementer fills with the observed failure and
the GREEN run (`givn instructions steps`, TESTS and IMPLEMENTATION). A scenario
that passes on its first run is recorded as existing behavior protected; no
production edit is manufactured for it.

**Verification per batch.** The single-scenario command above, once per batch
scenario. Whole-change checks are in `## Closeout`.

## Batch map

| Batch | Scenarios | Owning capability |
|---|---|---|
| `boundary-resolution` | 5 | `gvn-workflow-enforcement` |
| `runner-target` | 1 | `gvn-workflow-enforcement` |
| `receipt-identity` | 2 | `gvn-workflow-enforcement` |
| `receipt-freshness` | 2 | `gvn-workflow-enforcement` |
| `e2e-scope` | 3 | `gvn-workflow-enforcement` |
| `served-policy` | 3 | `prose-consolidation` |
| `pinned-policy-replacements` | 2 | `prose-consolidation` |
| `review-closure` | 1 | `gvn-workflow-enforcement` |
| **Total** | **19** | 14 `gvn-workflow-enforcement` + 5 `prose-consolidation` |

Every scenario of `specs/verify-change/givn-workflow-enforcement.feature` (14)
and `specs/create-change/prose-consolidation.feature` (5) appears in exactly one
batch by exact title. The delta adds no scenario the features do not contain and
adds no `@e2e` scenario at all.

---

## Interaction coverage pre-flight

Cross-checked every owning `## Interactions` inventory against design.md's
Interaction Coverage Matrix (`## Interaction Coverage Matrix`).

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/verify-change/usecase.md` | 13 | 13 | yes |
| `specs/create-change/usecase.md` | 9 | 9 | yes |
| **Total** | **22** | **22** | **all 22 tagged `@e2e`** |

No gap: every matrix target title resolves to a scenario in the permanent tree
and every target carries `@e2e` (checked against `git ls-files givn/specs`). The
delta adds no inventory entry: its boundary refusals, receipt inspections, and
served-policy cases are variants of the `gvn-workflow-enforcement` change-check
action and the `prose-consolidation` canonical-guidance action, which already
own their `@e2e` scenarios. No task is written for an `@e2e` scenario that does
not exist.

---

## Domain Constraints

Copied from the owning use-case documents before any scenario task. Each
scenario's Design constraints block repeats the rules its behavior exercises.

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `gvn-workflow-enforcement` | With `ise` effective, the change check refuses a declared `e2e` role that resolves no package, a resolved package whose manifest cannot be read, a resolved path under a `source:` root, a resolved package whose tree contains a `source:` root, a direct, transitive, or workspace-inherited dependency edge into `source`, a resolved path resolving no package whose reference resolves into a `source:` root, and an E2E runner command naming no resolved root. Each refusal names the condition and its subject and writes no receipt. | `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` |
| `gvn-workflow-enforcement` | The declared `e2e` role resolves a package set: each matched path resolves its nearest `Cargo.toml` at or above it, every distinct package directory is one resolved package checked on its own manifest, and a matched path that resolves no manifest is checked by its path-resolving references. Empty resolution refuses. `e2e` roots and `source` roots are disjoint both ways. | `developer--primary-user`, `givn-maintainer--owner` |
| `gvn-workflow-enforcement` | The runner-target rule is textual: `verify.e2e_command` must name every resolved `e2e` root by whole token or by a token containing `<directory>/`. It proves naming, not execution; a run that reported the regular scope is refused by the result scope field. | `givn-maintainer--owner`, `agent--technical-role` |
| `gvn-workflow-enforcement` | An unbound step is reported as a failed test and a reported skip is refused; neither yields a passed receipt and each refusal states its own count. The result read refuses a nonzero exit, a failed count, a skipped count, a zero count, a malformed result, an unreconciled result, and a wrong scope. | `developer--primary-user`, `agent--technical-role` |
| `gvn-workflow-enforcement` | The receipt binds the digests of the resolved packages, the E2E runner command digest, the running Givn version, and the E2E scope outcome. The scope outcome is delta-aware: an `@e2e` scenario in the delta requires a green scope, and a delta without one records `not_applicable`. Either mismatch is stale. | `developer--primary-user`, `agent--technical-role` |
| `gvn-workflow-enforcement` | Review and archive reuse the receipt only while the tested revision ancestry and the E2E identity still match. Every stale reason is collected, printed, and routed to `givn check tasks`; review and archive run no suite. | `developer--primary-user`, `givn-maintainer--owner` |
| `gvn-workflow-enforcement` | A change whose locked add-on set has `ise` off reads no declaration, runs one scope, gains no new refusal, and writes a receipt with neither E2E key (`skip_serializing_if`), so its bytes match the pre-existing shape. | `developer--primary-user`, `givn-maintainer--owner` |
| `gvn-workflow-enforcement` | One boundary resolver owns the condition text. `givn lint` renders the placement conditions (`MissingPackage`, `NoPackageManifest`, `PackageUnderSource`, `DependencyEdge`) as advisory findings with exit 0; `givn check tasks` refuses the acceptance conditions. The malformed-declaration error stays the declaration's own blocking error. | `agent--technical-role`, `givn-maintainer--owner` |
| `prose-consolidation` | The `specs` instruction owns the canonical `@e2e` policy; design, steps, and review instructions point at it. With `ise` effective the policy states the declared `e2e` package holds the runner and every `@e2e` step definition, links no production source, is run by `verify.e2e_command`, fails on an unbound or skipped step, and keeps feature files shared. A call that needs production source becomes a component scenario; the `ise`-off branch keeps today's in-process allowance. | `agent--technical-role`, `developer--primary-user` |

Guarantees carried by every batch: a refused request leaves project state
unchanged and names the condition; no declared path, role, package, or file is
invented; user-facing output names the package, path, edge, root, command, or
step it is about and names no Rust module, struct, or field.

---

## Contradictions

Recorded, not fixed. Each names the authoritative artifact; `design.md`, the
delta features, and the durable docs are unchanged.

1. **`design.md` "Guidance Amendments" names the permanent files as the edit
   target.** The rows name
   `givn/specs/create-change/prose-consolidation.feature` ("modify the two pinned
   scenarios by exact title") and `givn/specs/create-change/usecase.md` ("add the
   boundary rules"). This change carries both as delta content: the two pinned
   scenarios are `@givn.modified` entries in
   `givn/changes/enforce-ise-e2e-boundary/specs/create-change/prose-consolidation.feature`,
   and the rules live in the delta `usecase.md`. The delta feature is the
   executable authority (`.feature` file over prose), and archive performs the
   replacement. Batches `served-policy` and `pinned-policy-replacements` bind and
   run the delta scenarios; no permanent spec or use-case is edited here.

2. **The seed's landing rule is already satisfied by the authoring commit.**
   `arc42.md` "Notes" and the seed's landing rule place the two
   `assets/instructions/specs.md` amendments, the `assets/instructions/ise.md`
   correction, and the pinned scenarios "in the implementation batch that
   enables the blocking boundary". `1016f3bc` already committed
   `assets/instructions/specs.md`, `assets/instructions/ise.md`,
   `docs/reference.md`, `docs/glossary.md`, and chapters 02, 04, 05, 06, 08, 09,
   11 together with the authoring artifacts. Resolution: batches
   `served-policy` and `pinned-policy-replacements` verify the committed prose
   against the delta scenarios instead of re-committing it, and the placement
   table below ties every statement to the batch whose code makes it true.

3. **`design.md` "Strict Mode" proof step 2 cannot run in Setup.** Step 2 asks
   for "the absence of `verification/tasks-verification.json` for the same delta
   through `givn check tasks`", but the change check has no E2E scope until
   batch `e2e-scope`. Resolution: Setup proves the runner half (step 1: nonzero
   exit and the four counts from the real runner) and the writer's counts; the
   receipt half is asserted, with its own count, by `An ise-effective change
   whose E2E run leaves a step unbound fails the tasks check` and `An
   ise-effective change whose E2E run reports a skipped step fails the tasks
   check` in batch `e2e-scope`. The design's claim stays true; only its timing
   moves.

4. **The declared-package paragraph's runner sentence has no assertion in its
   own scenario.** `design.md`'s exact wording for the conditional paragraph
   includes "`verify.e2e_command` runs it", while
   `Canonical policy names the declared e2e package as the E2E target` asserts
   only the package-holds-runner-and-steps, no-production-source,
   unbound-or-skipped, and shared-feature-files sentences. The runner claim is
   asserted by the modified pinned scenario `Canonical policy selects E2E scope
   explicitly rather than by command shape` ("the E2E execution runs the
   declared e2e package which links no production source"). Reported, not
   fixed: adding a step would change a reviewed feature file, and the claim is
   covered by the second scenario in its own batch.

No design-named scenario is missing from the delta features: every title named
in `design.md` "Use-Case Traceability" and "Scenario set" resolves to a
scenario in the two delta features (checked by exact title), and the
permanent `Archive refuses a stale change verification receipt before merging`
referenced there exists at
`givn/specs/archive-change/archive-proof.feature:327`.

---

## Durable documentation placement

Each statement is verified in the batch whose code makes it true. No batch
re-commits it: the text landed in `1016f3bc`, and the artifact is not re-edited
here.

| Statement | Committed location | Verified in batch | What makes it true |
|---|---|---|---|
| `@e2e` declared-package paragraph | `assets/instructions/specs.md:153-160` | `served-policy` | the `{{#if addons.ise}}` block renders when `addons.ise` is effective |
| library bullet `ise` branch | `assets/instructions/specs.md:138-147` | `pinned-policy-replacements` (modified scenario 1), `served-policy` (ise-off scenario) | the rendered branch selected by `addons.ise` |
| shared-runner `ise` sentence | `assets/instructions/specs.md:185-189` | `pinned-policy-replacements` (modified scenario 2) | the same conditional render |
| `ise.md` two-layer boundary bullet | `assets/instructions/ise.md:20-23` | `served-policy` | `givn instructions ise` serves the corrected bullet |
| boundary, import fallback, textual runner-target, receipt fields, stale reasons | `docs/reference.md:146-160`, `:283-310` | `boundary-resolution`, `runner-target`, `receipt-identity`, `receipt-freshness`, `e2e-scope` | the implemented refusals and receipt fields |
| `Change verification receipt`, `E2E identity` rows | `docs/glossary.md:50`, `:73` | `receipt-identity` | the serialized receipt fields |
| `TC-39`, `TC-45`, chapter 04 strategy rows, chapter 05 `e2e_boundary` map, chapter 06 boundary flow | `docs/arc42/{02,04,05,06}` | `boundary-resolution` | the shared resolver and the blocking preflight |
| chapter 06 E2E scope and receipt flow, chapter 08 receipt concept, chapter 09 register plus ADR-070 amendment, chapter 11 `R-140`, `R-141` | `docs/arc42/{06,08,09,11}`, `docs/arc42/adr/ADR-070-forward-only-archive-proof.md`, `docs/arc42/adr/README.md` | `receipt-identity`, `receipt-freshness`, `e2e-scope` | the receipt fields, the freshness gate, and the second scope |
| chapter 11 `R-138` | `docs/arc42/11-risks-and-technical-debt.md:125` | `runner-target` | the textual runner-target rule |
| chapter 11 `R-139` | `docs/arc42/11-risks-and-technical-debt.md:126` | `boundary-resolution` | the import fallback's depth |

---

## Setup

- [x] Read the owning use-case documents and this change's seed before writing
      any step: `specs/verify-change/usecase.md` and `specs/create-change/usecase.md`
      (guarantees, rules, Personas) and
      `givn/ideation/ise-rebuildability/changes/enforce-ise-e2e-boundary.md`
      (Q5, Q11, the deferred policy amendments and their landing rule). Domain
      constraints are in `## Domain Constraints` above.
      Evidence: every scenario's Design constraints carry its owning rule, and
      the interaction pre-flight maps all 22 inventory entries to matrix rows and
      existing `@e2e` scenarios.

- [x] Confirm the runner and both commands. No new runner is installed:
      `tests/features_runner.rs` uses cucumber-rs 0.23 with `.fail_on_skipped()`
      at both registration points, `verify.command` is
      `./run-tests.sh --non-e2e`, and `verify.e2e_command` is
      `./run-tests.sh --e2e`. The runner reads `givn/specs/**` by default and the
      change's `specs/**` under `GIVN_FEATURES`; `run-tests.sh` forces the
      single-process `cargo` path when `GIVN_RESULT_FILE` is set.
      Evidence: `givn/commands.yaml` carries both commands unchanged.

- [x] Create the step-definition skeletons named by design.md "Step Definitions",
      one file per capability, and register them in `tests/steps/mod.rs` with
      `unimplemented!()` bodies:
      `tests/steps/workflow_enforcement_steps.rs` (extend),
      `tests/steps/prose_consolidation_steps.rs` (extend), and the new support
      module `tests/steps/e2e_boundary_support.rs` (fixture builders only, no
      `#[given]`/`#[when]`/`#[then]`).
      Reuse, never re-bind: `a project whose product is a library without a
      service or browser`, `the author reads the canonical E2E guidance`,
      `the author reads the canonical verification guidance` and the other
      bindings in `prose_consolidation_steps.rs`; `the user runs `cmd``,
      `the command exits 0` / `non-zero`, `stderr contains "…"`,
      `the file "…" exists` / `does not exist`; `a clean committed givn project
      with a change whose delta specs pass`, `the change has a current
      verification receipt`, `stderr says the verification receipt is stale`,
      `stderr routes to `givn check tasks --change <id>`` in
      `workflow_enforcement_steps.rs`; `a Givn project` and
      `served_instruction(env, id)` in `serve_extended_rules_steps.rs`.
      A phrase is bound exactly once across all files; a duplicate registration
      panics the runner.
      ```
$ cargo check --locked --tests
    Checking givn v0.9.20 (/home/buster/.paseo/worktrees/008cxm4u/modern-mule)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 14.55s
      ```
      The support module and its fixture builders landed with the boundary
      batches: `1a4d6bd2` registers `tests/steps/e2e_boundary_support.rs` in
      `tests/steps/mod.rs`; every delta phrase bound so far sits in
      `tests/steps/workflow_enforcement_steps.rs`.

- [x] Proof-of-strictness. Write one scratch delta feature outside the delta
      scope (a scratch `GIVN_FEATURES` directory) containing one scenario tagged
      `@e2e` whose single step has no binding, and run it through the real runner
      with a scratch result file:
      ```
      GIVN_FEATURES=<scratch dir> GIVN_CUCUMBER_TAGS='@e2e' \
        GIVN_RESULT_FILE=<scratch dir>/probe-result.json \
        cargo test --locked --test features_runner -- --exact \
        "strict-mode-probe.feature::An undefined step fails the run"
      ```
      Confirm a nonzero exit and record the four counts the writer emits
      (`failed = 1`, `skipped = 0`). If it exits 0, stop: `.fail_on_skipped()` is
      not in effect on the builder the run used.
      ```
$ cat /tmp/ise-strict-probe/strict-mode-probe.feature
@e2e
Feature: Strict mode probe

  @e2e
  Scenario: An undefined step fails the run
    Given a probe step with no binding

$ GIVN_FEATURES=/tmp/ise-strict-probe \
    GIVN_RESULT_FILE=/tmp/ise-strict-probe/probe-result.json \
    GIVN_RESULT_SCOPE=e2e cargo test --locked --test features_runner
Feature: Strict mode probe
  Scenario: An undefined step fails the run
   ✘  Given a probe step with no binding
      Step failed:
      Defined: tmp/ise-strict-probe/strict-mode-probe.feature:6:5
      Step doesn't match any function
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)
error: test failed, to rerun pass `--test features_runner`
Caused by: process didn't exit successfully: ... (exit status: 1)
exit 1

$ cat /tmp/ise-strict-probe/probe-result.json
{"failed":1,"passed":0,"scope":"e2e","skipped":0,"total":1}
      ```
      Counts: total 1, passed 0, failed 1, skipped 0 — an unbound step reaches the
      writer as a failed test, not a skip, so `.fail_on_skipped()` is in effect.
      Deviation from the recipe above: the `--exact` path (`nextest_run`) writes
      no `GIVN_RESULT_FILE`, so the probe ran through the plain
      `cargo test --locked --test features_runner` path, the only path that emits
      the counts. `GIVN_CUCUMBER_TAGS` was dropped with it — the scratch
      directory holds only the probe scenario.

- [x] Restore a passing baseline: delete the scratch feature and result file, run
      `./run-tests.sh --non-e2e`, and confirm exit 0. The delta scenarios stay
      `@wip`; the permanent tree uses none of the phrases above.
      ```
$ rm -rf /tmp/ise-strict-probe
$ ./run-tests.sh --non-e2e
────────────
     Summary [ 160.112s] 963 tests run: 963 passed, 111 skipped
exit 0
      ```
      The scratch directory is gone; the permanent tree binds none of the probe
      phrases. The later batches (`served-policy`, `pinned-policy-replacements`)
      keep `@wip`.

- [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `setup`
      The skeletons and any fixture helper land in this commit, so the later
      batches start from a clean tree. Setup was folded into the first boundary
      batch: `1a4d6bd2` carries the skeleton, the support module, and its
      registration.

---

## Behavior batches

Each batch names the complete file set for its scenarios; its scenarios share
one test phase, one implementation phase, one refactor phase, and one commit. A
scenario's Design constraints name the step file that binds it and the
production symbols it exercises, so no per-scenario file list repeats the batch
header.

### Batch 1: `boundary-resolution`

Five refusal scenarios of the declared boundary, all before any runner executes:
the package-resolution Outline, the dependency-edge Outline, the two source-root
scenarios, and the import-mode scenario.

**Files created:** `src/e2e_boundary/mod.rs`, `src/e2e_boundary/manifest.rs`
(moved from `src/cli/lint/manifest.rs`), `src/e2e_boundary/hash.rs` (the
path/executable/bytes hash rule moved out of `src/proof.rs`),
`tests/steps/e2e_boundary_support.rs`.
**Files deleted:** `src/cli/lint/manifest.rs`.
**Files modified:** `src/lib.rs`, `src/proof.rs` (imports `hash_entry`; no
behavior change), `src/cli/lint/mod.rs`, `src/cli/lint/specification.rs`,
`src/cli/check.rs`, `src/config/lock.rs`, `tests/steps/mod.rs`,
`tests/steps/workflow_enforcement_steps.rs`.

**Design constraints (batch-wide):**
- `E2eBoundary::resolve(project_root, &SpecificationDeclaration) -> E2eBoundary`
  performs the design's nine-step pass. Walk with the existing exclusions
  (`.git`, `.hg`, `.svn`, `.fossil`, `target`, `givn/archive`, `givn/changes`),
  compile each declared `e2e` entry to a `Glob`, and collect the union of
  matched files and directories. Empty match yields one `MissingPackage` per
  declared entry.
- Resolve each matched path to the nearest `Cargo.toml` at or above it, bounded
  at the project root, with `nearest_manifest` moved into
  `src/e2e_boundary/manifest.rs`. Distinct package directories are the resolved
  packages; a matched path with no manifest is an import-mode path.
- A package directory matched by a `source:` glob yields `PackageUnderSource`
  naming the entry and the root. A package directory whose tree contains a path
  matching a `source:` glob yields `SourceInsidePackage` naming the package and
  the root. A project-root package fails the second rule, because every declared
  source root lies inside it.
- Read every `Cargo.toml` under the walk once. A resolved package whose manifest
  exists but does not parse, or declares neither `[package]` nor `[workspace]`,
  yields `MalformedManifest { package, reason }`; a malformed manifest elsewhere
  is ignored as today.
- Build direct edges from path dependencies and `workspace = true` dependencies
  resolved through the nearest workspace root. Breadth-first traversal from each
  package reports every reachable package under a `source:` root as
  `DependencyEdge { transitive }`, once per starting package.
- For each import-mode path, read every tracked file under it. A `mod`, `use`,
  `require`, or `import` reference that reduces to path segments naming a file or
  directory under a `source:` root, or a relative reference that escapes into a
  `source:` root, yields `ImportEdge { path, target }`. External package names,
  standard-library names, and references resolving to no project path are
  ignored.
- `BoundaryCondition` carries `kind`, `subject`, `message`, and `placement`.
  `BoundaryCondition::message` is the one wording: `cli::lint` renders the
  placement subset (`MissingPackage`, `NoPackageManifest`,
  `PackageUnderSource`, `DependencyEdge`) as `Severity::Advisory` findings with
  exit 0, and `cli::check` renders the acceptance set (`MissingPackage`,
  `PackageUnderSource`, `SourceInsidePackage`, `DependencyEdge`, `ImportEdge`,
  `MalformedManifest`) as refusals. No condition text is written twice.
- `config::lock::ise_effective_for_change(change_dir) -> Result<bool>` reads the
  change's `config-lock.yaml` through `config::lock::read` and resolves
  `addons.ise`; an absent lock is false (D3). It adds no schema and reads no
  project manifest.
- `run_change_verification` inserts the boundary preflight after
  `preflight_readme_impact` and `change_verification_not_applicable`, and before
  the scenario-commit and clean-tree checks, so an `ise`-effective change always
  needs a readable declaration and a non-empty resolved set. Every condition is
  printed; the function returns nonzero and writes no receipt.
- `cli::lint` keeps its four advisory kinds, messages, severity, and exit code,
  and its output must stay byte-identical for the existing fixtures. The
  existing glob matcher and bounded walk move into `e2e_boundary` and are
  re-exported to `cli::lint`; they are not duplicated.
- `hash_entry` moves into `src/e2e_boundary/hash.rs`; `src/proof.rs` imports it
  and no `e2e_boundary` module imports `proof`.
- Refusal text names the package, path, root, or imported package and names no
  Rust identifier. Exit code is nonzero; `givn/specification.yaml` and the change
  tree stay unchanged.

**Batch verification:** the five single-scenario commands below, plus
`./run-tests.sh --non-e2e` to prove `givn lint` output is unchanged for the
permanent `check-specification-health` scenarios and the receipt scenarios the
hash move touches.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: boundary-resolution'`.

- [x] Scenario: `An ise-effective change whose declared e2e role resolves no readable package fails the tasks check`
  - Design constraints:
    - Examples rows: `matches no project path` yields `MissingPackage` whose
      message contains `matches no path`; `resolves a package with a malformed
      manifest` yields `MalformedManifest` whose message contains
      `cannot be read`; `lists no path` declares `e2e: []` and yields the empty
      resolution refusal, `the 'e2e' role lists no path`.
    - Fixture (`e2e_boundary_support.rs`): a temp project, `git init`, one
      committed change directory with `config-lock.yaml` recording `ise: true`,
      a `givn/specification.yaml` whose `e2e:` entry matches no path (row 1), and
      a `source:` list naming `src`. Row 2 declares `e2e: ["tests/e2e"]` and
      writes a `tests/e2e/Cargo.toml` that is not valid TOML. Row 3 declares an
      empty `e2e:` list.
    - The Given step per row lives in
      `tests/steps/workflow_enforcement_steps.rs`; it drives
      `common::run_givn` and writes files only. It imports no givn module.
    - Then `stderr names the declared e2e package and "<condition>"` asserts the
      declared entry text and the row's condition substring on stderr; Then
      `no change verification receipt is written` asserts
      `verification/tasks-verification.json` is absent.
  - [x] Implement the scenario and its tests (test-first per `design.md`).
        Files: `src/e2e_boundary/{mod,manifest,hash}.rs`,
        `src/cli/check.rs`, `src/config/lock.rs`; step:
        `tests/steps/workflow_enforcement_steps.rs`. Run both Examples rows.
        Evidence (observed failure, then GREEN):
        ```
RED observed (row "matches no project path", first run before the guard was ordered):
  step panicked: stderr does not name the declared e2e entry "tests/e2e": error: the declared
  e2e boundary cannot be accepted: the 'e2e' role lists no path; the change check requires a
  resolved E2E package
The empty-resolution guard masked the MissingPackage condition; it now runs only when no
condition exists.
RED admitted for row "resolves a package with a malformed manifest": the resolver and preflight
were implemented before the binding, so no historical RED is claimed (steps TESTS.5).
GREEN: GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
  features_runner -- --exact "verify-change/givn-workflow-enforcement.feature::An ise-effective
  change whose declared e2e role resolves no readable package fails the tasks check"
  -> 2 scenarios (2 passed), 10 steps (10 passed).
  Row 1 fires MissingPackage ("matches no path"); row 2 fires MalformedManifest ("cannot be read").
Discrimination: with the preflight call removed, both rows fail at the same Then.
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run the scenario and
        `./run-tests.sh --non-e2e`.
        ```
REFACTOR: extracted the condition vocabulary into src/e2e_boundary/conditions.rs (mod.rs 506 ->
355 lines; conditions.rs 172) and re-ran the scenario: 2 scenarios (2 passed), 10 steps (10 passed).
./run-tests.sh --non-e2e on the batch tree: 963 tests run: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `boundary-resolution`

- [x] Scenario: `An ise-effective change whose e2e package reaches a source package fails the tasks check`
  - Design constraints:
    - Examples rows: `direct`, `transitive`, `workspace-inherited`,
      `target-specific`.
    - Fixture: `tests/e2e` is a Cargo package; `src` is a package under the
      declared `source:` root. Direct: `tests/e2e/Cargo.toml` names
      `givn_product = { path = "../../src" }`. Transitive: `tests/e2e` depends on
      `tests/fixtures/helper`, which depends on `../../src`. Workspace-inherited:
      a workspace root `Cargo.toml` declares
      `[workspace.dependencies] givn_product = { path = "src" }` and
      `tests/e2e/Cargo.toml` names `givn_product.workspace = true`.
      Target-specific: `tests/e2e/Cargo.toml` declares the edge under
      `[target.'cfg(unix)'.dependencies]`.
    - Then `stderr names the e2e package and the dependency edge into "source"`
      asserts the resolved package directory and a dependency-edge phrase naming
      `source` on one stderr line; `DependencyEdge` renders as
      `dependency edge into 'source'` with the edge kind.
    - Then `no change verification receipt is written` as above.
  - [x] Implement the scenario and its tests. Run all four Examples rows.
        Files: `src/e2e_boundary/mod.rs` (edge traversal),
        `src/e2e_boundary/manifest.rs` (workspace-inherited resolution).
        Evidence:
        ```
RED admitted (steps TESTS.5): the resolver and preflight were implemented before the bindings;
no historical RED is claimed.
GREEN: the same single-scenario command on "...whose e2e package reaches a source package fails
the tasks check" -> 3 scenarios (3 passed), 15 steps (15 passed) for direct, transitive, and
workspace-inherited. Direct renders "has a dependency edge into source (root 'src/**') through
package 'givn_product'"; transitive renders "has a transitive dependency edge ...".
Discrimination: with the preflight call removed, all three rows fail at the shared Then.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-ran the scenario after the conditions.rs split and the committed-fixture change:
3 scenarios (3 passed), 15 steps (15 passed). Batch suite: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `boundary-resolution`

- [x] Scenario: `An ise-effective change whose e2e path sits under a source root fails the tasks check`
  - Design constraints:
    - Fixture: the declared `source:` list contains `src`, and the `e2e` entry
      matches `src/e2e-tests`, a directory with no manifest.
    - `PackageUnderSource` renders the matched path and the root it sits under.
    - Then `stderr names the e2e path and the source root`; no receipt is
      written.
  - [x] Implement the scenario and its tests. Files: `src/e2e_boundary/mod.rs`
        (disjointness rule). Evidence:
        ```
RED admitted (steps TESTS.5): no historical RED is claimed.
GREEN: "...whose e2e path sits under a source root fails the tasks check" -> 1 scenario
(1 passed), 5 steps (5 passed). The refusal names the matched path 'src/e2e-tests' and the root
'src/**' (PackageUnderSource); the advisory NoPackageManifest line prints on stdout and vetoes
nothing.
Discrimination: preflight removed -> fails at "stderr does not name the source root condition".
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-ran the scenario after the conditions.rs split and the committed-fixture change:
1 scenario (1 passed), 5 steps (5 passed). Batch suite: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `boundary-resolution`

- [x] Scenario: `An ise-effective change whose e2e package contains a source root fails the tasks check`
  - Design constraints:
    - Fixture: the declared `e2e` entry matches the project root (`.`), whose
      tree contains `src/`; the resolved package is the project-root package.
    - `SourceInsidePackage` renders the package and the contained root, and it
      is what a project-root package produces instead of a separate
      project-root condition.
    - Then `stderr names the e2e package and the source root`; no receipt is
      written.
  - [x] Implement the scenario and its tests. Files: `src/e2e_boundary/mod.rs`.
        Evidence:
        ```
RED admitted (steps TESTS.5): no historical RED is claimed.
GREEN: "...whose e2e package contains a source root fails the tasks check" -> 1 scenario
(1 passed), 5 steps (5 passed). The project-root package '.' resolves and SourceInsidePackage
names it with the root 'src/**'; no separate project-root condition exists.
Discrimination: preflight removed -> fails at "stderr does not name the e2e package".
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-ran the scenario after the conditions.rs split and the committed-fixture change:
1 scenario (1 passed), 5 steps (5 passed). Batch suite: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `boundary-resolution`

- [x] Scenario: `An ise-effective change whose e2e path resolves no package and imports a source package fails the tasks check`
  - Design constraints:
    - Fixture: `tests/e2e` carries no `Cargo.toml` and one tracked step file
      whose reference is `use givn::proof::ChangeReceipt;` (a qualified
      reference whose first segment names no project path) plus
      `mod ../../src/helpers;` (a relative reference escaping into `source`).
      The refusal names the escaping reference, which is the one that resolves.
    - `ImportEdge { path, target }` renders the import-mode path and the
      imported source package or file; external package names and
      standard-library names are admitted, so the fixture must contain a
      reachable source target.
    - Then `stderr names the e2e path and the imported source package`; no
      receipt is written.
  - [x] Implement the scenario and its tests. Files: `src/e2e_boundary/mod.rs`
        (`import_references`), `src/e2e_boundary/manifest.rs`.
        Evidence:
        ```
RED admitted (steps TESTS.5): no historical RED is claimed.
GREEN: "...whose e2e path resolves no package and imports a source package fails the tasks check"
-> 1 scenario (1 passed), 5 steps (5 passed). tests/e2e resolves no manifest, so the import reader
refuses only "mod ../../src/helpers" -> target 'src/helpers'; the qualified
"use givn::proof::ChangeReceipt" names no project path under source and is admitted.
Discrimination: preflight removed -> fails at "stderr does not name the import edge from
\"tests/e2e\" to \"src/helpers\"".
        ```
  - [x] REFACTOR: re-run the scenario and the permanent
        `check-specification-health` scope; must still pass.
        ```
REFACTOR: re-ran the scenario after the conditions.rs split and the committed-fixture change:
1 scenario (1 passed), 5 steps (5 passed). Batch suite: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `1a4d6bd25f907956cfc8e3be1813ba8d104ac3a9` -- Batch: `boundary-resolution`

---

### Batch 2: `runner-target`

One Outline over the three runner-command rows.

**Files modified:** `src/e2e_boundary/mod.rs` (`RunnerNamesNoPackage` and the
token rule), `src/cli/check.rs` (render the condition),
`tests/steps/workflow_enforcement_steps.rs`,
`tests/steps/e2e_boundary_support.rs`.

**Design constraints (batch-wide):**
- `conditions(command: &str)` adds `RunnerNamesNoPackage { root }` naming the
  first resolved `e2e` root the command does not name. Matching is textual: the
  command must contain the project-relative root directory as a whole token, or
  a token containing `<directory>/`. Quoting and whitespace around a token are
  allowed.
- The empty command and the `givn missing-e2e-runner` sentinel name no root and
  refuse, naming the first root the command must name. The refusal shows the
  command as it stands, so an empty value renders as an explicit empty value,
  not as a missing line.
- The rule proves naming, not execution. It does not refuse a command string
  identical to `verify.command` (design D6, review deviation B): a shared runner
  distinguishes the executions through `GIVN_RESULT_SCOPE`, and a run that
  executed the regular scope is refused by the result's `scope` field in batch
  `e2e-scope`.
- Refusal text names the declared root and the E2E runner command; exit is
  nonzero; no receipt is written. No Rust identifier appears.

**Batch verification:** the single-scenario command below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: runner-target'`.

- [x] Scenario: `An ise-effective change whose E2E runner command names no declared e2e root fails the tasks check`
  - Design constraints:
    - Examples rows: `./run-tests.sh --bin other-target` (names a foreign
      target), `(empty)` (the fixture writes `verify.e2e_command: ""`), and
      `givn missing-e2e-runner` (the sentinel).
    - Fixture: one resolved `e2e` package at `tests/e2e` with no dependency edge,
      so only the runner condition fires. The step maps the literal `(empty)`
      row to an empty command value.
    - Then `stderr names the declared e2e root and the E2E runner command`
      asserts `tests/e2e` and the command text (or its explicit empty rendering)
      on stderr; `no change verification receipt is written` as before.
    - A command naming the root (`./run-ise-tests.sh --package tests/e2e`) is
      accepted, which the next batch's fixtures exercise.
  - [x] Implement the scenario and its tests. Run all three Examples rows.
        Evidence:
        ```
RED admitted (steps TESTS.5): E2eBoundary::conditions already carried the runner rule when batch
boundary-resolution landed; this batch adds the regression test for it.
GREEN: the single-scenario command on "...whose E2E runner command names no declared e2e root
fails the tasks check" -> 3 scenarios (3 passed), 15 steps (15 passed). Rows:
"./run-tests.sh --bin other-target", "(empty)" (the refusal renders the command as ''), and
"givn missing-e2e-runner"; every refusal names 'tests/e2e' and the command.
Discrimination: with the preflight call removed, all three rows fail at "stderr does not name the
declared e2e root \"tests/e2e\"".
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-ran the scenario on the final tree: 3 scenarios (3 passed), 15 steps (15 passed).
cargo fmt --all --check clean; cargo clippy --workspace --all-targets -- -D warnings clean;
./run-tests.sh --non-e2e: 963 tests run: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `78ca50390dec435a406801b90be52b5874e5f556` -- Batch: `runner-target`

---

### Batch 3: `receipt-identity`

The receipt shape: the two optional fields, their serialization rule, and the
E2E scope outcome for a delta without an `@e2e` scenario.

**Files modified:** `src/proof.rs` (`E2eIdentity`, the two `Option` fields,
`CHANGE_E2E_RESULT_FILE`, the `ChangeVerification` input struct),
`src/e2e_boundary/mod.rs` (`E2eBoundary::identity`),
`src/cli/check.rs` (capture the identity and record the scope outcome),
`tests/steps/workflow_enforcement_steps.rs`,
`tests/steps/e2e_boundary_support.rs`.

**Design constraints (batch-wide):**
- `pub const CHANGE_E2E_RESULT_FILE: &str = "verification/e2e-result.json";`
- `pub struct E2eIdentity { package_digests: BTreeMap<String, String>,
  runner_command_digest: String, givn_version: String }` lives in `src/proof.rs`
  and derives `Debug, Clone, PartialEq, Eq, Serialize, Deserialize`.
- `ChangeReceipt` keeps `schema: 2` and `content_scope: "change-proof/v1"` and
  gains `#[serde(default, skip_serializing_if = "Option::is_none")] pub
  e2e_identity: Option<E2eIdentity>` and the same attribute on `pub e2e_scope:
  Option<ScopeOutcome>`. Neither key is serialized when absent: no `null`, so an
  `ise`-off receipt keeps its previous bytes and an older schema-2 receipt still
  parses.
- `E2eBoundary::identity(project_root) -> Result<E2eIdentity>` hashes each
  resolved package's tracked tree with `hash_entry`, keyed by the
  project-relative package directory; in import mode the key is the resolved
  path. It hashes no declaration pattern and no file outside the resolved set.
  `runner_command_digest = digest_text(&manifest.verify.e2e_command)`;
  `givn_version = env!("CARGO_PKG_VERSION")`.
- `write_change_receipt` takes a `ChangeVerification` input struct carrying the
  regular `RunnerResult`, the optional `E2eIdentity`, and the optional
  `ScopeOutcome`, instead of positional arguments for those values. Every
  existing caller is migrated; no shim remains.
- `ScopeOutcome::not_applicable()` (existing) records
  `{ "status": "not_applicable" }`. `gherkin::parse::has_e2e_scenarios` decides
  applicability at check time; the E2E command is not run when the delta has no
  `@e2e` scenario, and the boundary preflight still runs.
- Receipt JSON uses domain names only (`e2e_identity`, `e2e_scope`,
  `package_digests`, `runner_command_digest`, `givn_version`).

**Batch verification:** the two single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: receipt-identity'`.

- [x] Scenario: `An ise-effective change whose delta has no E2E scenario records the E2E scope as not applicable`
  - Design constraints:
    - Fixture: an `ise`-effective change whose delta has one regular scenario,
      one committed scenario revision, a clean tree, a scoped fake runner
      (`verify.command`) writing `total/passed = 1`, and a resolved `e2e`
      package whose runner command names it. `GIVN_RESULT_SCOPE` handling stays
      in the fixture runner.
    - `givn check tasks` exits 0; the receipt at
      `verification/tasks-verification.json` carries `e2e_identity` with one
      `package_digests` entry keyed `tests/e2e`, and `e2e_scope.status` equal to
      `not_applicable`. The `verification/e2e-result.json` path is never
      required.
    - Then `the change verification receipt records a digest of every resolved
      e2e package` and `the change verification receipt records the E2E scope as
      not applicable` parse the JSON; they do not compare bytes.
  - [x] Implement the scenario and its tests. Evidence:
        ```
RED: GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
features_runner -- --exact
"verify-change/givn-workflow-enforcement.feature::An ise-effective change whose delta has no E2E
scenario records the E2E scope as not applicable" -> 1 scenario (1 failed), 4 steps (3 passed, 1
failed). Observed: "the receipt records no resolved e2e package digest: {"change":"ise-boundary",
"command":"./emit-change-result.sh",..."content_scope":"change-proof/v1",...,"result":{"failed":0,
"passed":1,"scope":"change","skipped":0,"total":1},"runner_exit":0,"schema":2,"status":"passed",
...}" — the check exited 0 from a green one-scenario receipt that carried no e2e_identity.
Production: src/proof.rs (E2eIdentity, ChangeVerification, the two optional receipt fields),
src/e2e_boundary/mod.rs (E2eBoundary::identity), src/cli/check.rs (the preflight returns the
accepted boundary; the check records the identity and the not_applicable scope).
GREEN: same command -> 1 scenario (1 passed), 5 steps (5 passed).
        ```
  - [x] REFACTOR: re-run the scenario and the permanent receipt scenarios in
        `workflow_enforcement_steps.rs`; must still pass.
        ```
REFACTOR: both batch scenarios re-run on the final tree: 1 scenario (1 passed) each (5 and 4
steps). cargo fmt --all -- --check clean; cargo clippy --locked --all-targets -- -D warnings clean.
Permanent receipt scenarios (GIVN_FEATURES=givn/specs, same single-scenario command): "Consumer
runs check tasks and receives a change verification receipt", "Review refuses a change without a
verification receipt", "Review refuses a receipt after the delta specs changed", "Review accepts a
current verification receipt" -> each 1 scenario (1 passed). The shared shape also holds for
"Archive refuses a stale change verification receipt before merging", "Archive reuses the change
receipt and runs only the merged suite", "Archive reuses a proof committed with its receipt" ->
each 1 scenario (1 passed).
        ```
  - [x] COMMIT: `a95b6f020bf04c8ddb3f5cf9abdbcd6c4a40cc60` -- Batch: `receipt-identity`

- [x] Scenario: `A change without ise keeps today's receipt without an E2E identity`
  - Design constraints:
    - Fixture: the same shape with `config-lock.yaml` recording `ise: false`.
      No declaration is read, no boundary preflight runs, and the fixture needs
      no `givn/specification.yaml`.
    - `givn check tasks` exits 0 and the receipt JSON contains neither
      `"e2e_identity"` nor `"e2e_scope"` (key absence, not a null value).
    - Then `the change verification receipt contains no E2E identity and no E2E
      scope` asserts key absence and that the remaining schema-2 keys are
      unchanged.
  - [x] Implement the scenario and its tests. Evidence:
        ```
PROTECTED: the requirement is the pre-existing receipt shape; the first run passed before any
production edit -> "A change without ise keeps today's receipt without an E2E identity":
1 scenario (1 passed), 4 steps (4 passed). No behavior was manufactured for it.
Production edit in the batch: write_change_receipt takes the ChangeVerification input struct
(every caller migrated); the ise-off path binds no identity and no scope.
GREEN: same command after the migration -> 1 scenario (1 passed), 4 steps (4 passed); the receipt
holds exactly the 13 schema-2 keys and neither "e2e_identity" nor "e2e_scope".
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree -> 1 scenario (1 passed), 4 steps (4 passed).
        ```
  - [x] COMMIT: `a95b6f020bf04c8ddb3f5cf9abdbcd6c4a40cc60` -- Batch: `receipt-identity`

---

### Batch 4: `receipt-freshness`

The seven new stale reasons, the delta-aware scope rule, and the shared gate
review and archive already call.

**Files modified:** `src/proof.rs` (`StaleReason` variants, `render`,
`ChangeReceipt::freshness`), `src/cli/check.rs` (the routing message unchanged
unless the E2E reason needs its own repair sentence),
`tests/steps/workflow_enforcement_steps.rs`,
`tests/steps/e2e_boundary_support.rs`.

**Design constraints (batch-wide):**
- `StaleReason` gains `MissingE2eIdentity`, `E2eIdentityUnavailable { reason }`,
  `E2ePackageChanged { package, receipt, current }`,
  `E2eRunnerCommandChanged { receipt, current }`,
  `GivnVersionChanged { receipt, current }`, `E2eScopeMissing`,
  `E2eScopeUnexpected`. `render()` produces one line each, containing these
  contiguous substrings: `receipt lacks the E2E identity`; `the E2E identity
  cannot be read` plus the declaration error; `the e2e package digest` plus the
  package path and both digests plus `changed`; `E2E runner command changed`;
  `Givn version changed`; `the E2E scope is missing`; `the E2E scope is not
  applicable to this delta`.
- `ChangeReceipt::freshness` keeps its existing checks and adds the E2E rules
  only while `config::lock::ise_effective_for_change(change_dir)` is true:

  | Condition | Stale reason |
  |---|---|
  | `e2e_identity` absent | `MissingE2eIdentity` |
  | declaration absent or unreadable | `E2eIdentityUnavailable` |
  | a recomputed package digest differs | `E2ePackageChanged` |
  | `digest_text(manifest.verify.e2e_command)` differs | `E2eRunnerCommandChanged` |
  | `env!("CARGO_PKG_VERSION")` differs | `GivnVersionChanged` |
  | delta has an `@e2e` scenario and the scope is absent, `not_applicable`, or not green | `E2eScopeMissing` |
  | `e2e_scope` is `passed` while the delta has no `@e2e` scenario | `E2eScopeUnexpected` |

- Scope applicability is re-read with `gherkin::parse::has_e2e_scenarios` at
  freshness time, so a delta edited after `givn check tasks` reports a scope
  reason. Every reason is collected, not short-circuited; review and archive
  print all of them and route to `givn check tasks`.
- A change with `ise` off ignores both fields.
- `src/cli/archive.rs` needs no new mechanism: it already calls the shared gate.
  Confirm that in the batch and record it; do not add a second archive check.
- Fixture: `e2e_boundary_support.rs` builds "a current receipt" by running
  `givn check tasks` once on an `ise`-effective fixture whose delta has no
  `@e2e` scenario (scope `not_applicable`, identity recorded, no E2E run), then
  rewrites only the field a row mutates and commits. For rows needing an `@e2e`
  delta, the fixture writes a delta feature carrying an `@e2e` scenario and
  rewrites `e2e_scope` to the row's value; no E2E command executes in this
  batch.

**Batch verification:** the two single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: receipt-freshness'`.

- [x] Scenario: `An ise-effective change receipt whose E2E identity no longer matches the project is stale for review`
  - Design constraints:
    - Examples rows and mutations: `the e2e package changed` (add a tracked file
      under `tests/e2e` and commit), `the E2E runner command changed` (edit
      `verify.e2e_command` in `givn/commands.yaml`), `the Givn version changed`
      (rewrite `givn_version` in the receipt), `the receipt lacks the identity`
      (delete the `e2e_identity` key), `the declaration cannot be read` (write a
      malformed `givn/specification.yaml`).
    - Each mutation lands in a commit, so the clean-tree requirement of review
      holds; the receipt itself may be committed as evidence.
    - Then `stderr says the change verification receipt is stale`; Then `stderr
      names "<reason>"` asserts every word of the row's reason appears in order
      on one stderr line, case-insensitive, so an inserted package path between
      the words does not fail the row while another reason does.
    - `givn check review` exits nonzero, runs no suite, and the reason text
      routes to `givn check tasks`.
  - [x] Implement the scenario and its tests. Run all five Examples rows.
        Evidence:
        ```
RED (5 rows, before the freshness rule):
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
features_runner -- --exact
"verify-change/givn-workflow-enforcement.feature::An ise-effective change receipt whose E2E
identity no longer matches the project is stale for review"
-> 5 scenarios (5 failed), 28 steps (23 passed, 5 failed).
  - "the e2e package changed" and "the E2E runner command changed" failed at "stderr names <reason>";
    stderr reported only "tested content changed: receipt ..., current ...".
  - "the Givn version changed" and "the receipt lacks the identity" failed at "stderr says the change
    verification receipt is stale": a receipt-only rewrite left the receipt Current, so review
    reported no staleness.
  - "the declaration cannot be read" failed at "stderr names <reason>"; stderr reported only
    "tested content changed".
Production: src/proof.rs (the seven StaleReason variants and their render lines, ScopeOutcome::is_green,
ChangeReceipt::e2e_stale_reasons called from the shared freshness gate); the shared step
`stderr names "..."` now matches the reason's words in order on one line, so a package path inserted
between the words does not fail the row.
GREEN: same command -> 5 scenarios (5 passed), 30 steps (30 passed); each row reports its own reason.
        ```
  - [x] REFACTOR: re-run the scenario and the permanent
        `Review refuses a receipt after the delta specs changed` scenario; must
        still pass.
        ```
REFACTOR on the committed tree: identity Outline -> 5 scenarios (5 passed), 30 steps (30 passed).
Permanent scenarios (GIVN_FEATURES=givn/specs): "Review refuses a receipt after the delta specs
changed" and "Review accepts a current verification receipt" -> each 1 scenario (1 passed);
"Archive refuses a stale change verification receipt before merging" and "Archive reuses a proof
committed with its receipt" -> each 1 scenario (1 passed). src/cli/archive.rs:1363 calls the same
ChangeReceipt::freshness gate; no second archive check was added.
cargo fmt --all -- --check clean; cargo clippy --locked --all-targets -- -D warnings clean;
./run-tests.sh --non-e2e: 963 tests run: 963 passed, 111 skipped, exit 0.
        ```
  - [x] COMMIT: `61c381427f5c9efdbf2eb18925c41a211e151a37` -- Batch: `receipt-freshness`

- [x] Scenario: `An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review`
  - Design constraints:
    - Examples rows: a delta that `has an E2E scenario` with the receipt
      recording `not_applicable` yields `the E2E scope is missing`; a delta that
      `has no E2E scenario` with the receipt recording `passed` yields `the E2E
      scope is not applicable to this delta`.
    - The fixture writes the delta feature (with or without an `@e2e` scenario)
      and the receipt's `e2e_scope`; freshness re-reads the delta through
      `has_e2e_scenarios`.
    - Then the same two Then steps as the previous scenario, with the row's
      reason; the refusal names the delta-aware rule, not a generic staleness.
  - [x] Implement the scenario and its tests. Run both Examples rows.
        Evidence:
        ```
RED (2 rows, before the scope rule): same single-scenario command on
"An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review"
-> 2 scenarios (2 failed).
  - "has an E2E scenario" / not_applicable failed at "stderr names 'the E2E scope is missing'";
    stderr reported only "delta features changed" and "tested content changed".
  - "has no E2E scenario" / passed failed at "stderr says the change verification receipt is stale":
    the receipt-only scope rewrite left freshness Current.
Production: the delta-aware rule re-reads the delta with gherkin::parse::has_e2e_scenarios at
freshness time: an @e2e delta needs a green scope, and a passed scope with no @e2e scenario is
E2eScopeUnexpected.
GREEN: same command -> 2 scenarios (2 passed), 12 steps (12 passed).
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR on the committed tree: 2 scenarios (2 passed), 12 steps (12 passed).
        ```
  - [x] COMMIT: `61c381427f5c9efdbf2eb18925c41a211e151a37` -- Batch: `receipt-freshness`

---

### Batch 5: `e2e-scope`

The change-scoped E2E execution, its two failure refusals, and the receipt that
binds a passed E2E run over two packages.

**Ordering.** `design.md` "Change Check Flow" runs the E2E command before the
identity is recaptured and the receipt written, so the receipt-binding scenario
belongs to this batch: it needs a green E2E run and cannot pass earlier. The
receipt shape it uses lands in batch `receipt-identity`.

**Files modified:** `src/cli/check.rs` (E2E scope: clear the result file,
capture the identity, run, read, recapture), `src/proof.rs` (the
`ChangeVerification` E2E member, `ScopeOutcome::passed` reuse, and a
`RunnerResult::read(&path, "e2e")` call site),
`tests/steps/workflow_enforcement_steps.rs`,
`tests/steps/e2e_boundary_support.rs`.

**Design constraints (batch-wide):**
- When the delta has an `@e2e` scenario, the check clears
  `verification/e2e-result.json`, captures the identity, runs
  `manifest.verify.e2e_command` with `GIVN_FEATURES` pointing at the change's
  `specs/`, `GIVN_RESULT_FILE` pointing at the E2E result path, and
  `GIVN_RESULT_SCOPE=e2e`. Then it reads the result with
  `RunnerResult::read(_, "e2e")` and recaptures the identity; a difference
  refuses with no receipt.
- A nonzero exit, an unbound step reported as a failed count, a skipped count, a
  zero count, a malformed result, an unreconciled result, or a wrong scope each
  fails the check and leaves no receipt. The refusal names the failing E2E run
  and the count the result reported.
- The refusal text states the count: `1 failed test` for the unbound row and
  `1 skipped test` for the skip row, so one generic failure message cannot
  satisfy both.
- The receipt records `e2e_scope` with `status: "passed"` and the counts, and
  `e2e_identity.package_digests` carries one entry per resolved package
  directory, keyed `tests/e2e` and `tests/e2e-support` in the two-package
  fixture.
- Fixture runner script (in `e2e_boundary_support.rs`): a small `sh` script
  branching on `GIVN_RESULT_SCOPE`; for `e2e` it writes the counts the row
  selects (`failed = 1`, `skipped = 1`, or all green) and exits nonzero for the
  two failure rows. It names each resolved `e2e` root in its argument text, so
  the batch `runner-target` rule holds.

**Batch verification:** the three single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: e2e-scope'`.

- [x] Scenario: `An ise-effective change whose E2E run leaves a step unbound fails the tasks check`
  - Design constraints:
    - Fixture: delta with one regular scenario and one `@e2e` scenario whose
      step the fixture runner reports as `failed = 1`, exiting nonzero. The
      regular scope is green.
    - Then `stderr names the failing E2E run and reports 1 failed test`; Then
      `no change verification receipt is written`. The regular scope ran before
      the E2E scope, so the absence proves the second scope blocks the receipt.
  - [x] Implement the scenario and its tests. Evidence:
        ```
RED: the test and its bindings were written first, but the production edit had already been
written; the observed failure below was measured by reverting the uncommitted production files
(`git stash push -- src/cli/check.rs src/proof.rs`), so the scenario ran against the base tree. No
history was rewritten and no behaviour was broken on purpose.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
features_runner -- --exact "verify-change/givn-workflow-enforcement.feature::An ise-effective
change whose E2E run leaves a step unbound fails the tasks check"
  ✘ Then the command exits non-zero
    Step panicked. Captured output: expected failure, got exit Some(0): stdout=Mandatory checks
    for 'tasks': ⚠ task preflight: review.md is missing ✓ change verification receipt:
    1 scenario(s), 0 failed, 0 skipped at e141d80dbb878b2eb29c8a57f6a1c1d1c8dc6706 ✓ verify passed
1 scenario (1 failed), 3 steps (2 passed, 1 failed), runner exit 101. The base check ran only the
regular scope, so the unbound E2E step never reached it and the receipt was written anyway.
Production: src/cli/check.rs (run_change_e2e_scope: clear verification/e2e-result.json, capture the
identity, run verify.e2e_command with GIVN_FEATURES / GIVN_RESULT_FILE / GIVN_RESULT_SCOPE=e2e, read
the result before the exit status, recapture the identity; capture_e2e_identity), src/proof.rs
(CHANGE_E2E_RESULT_FILE, read_change_e2e_result, ScopeOutcome::passed reuse) and the count wording
in RunnerResult::read_counts.
GREEN: same command -> 1 scenario (1 passed), 5 steps (5 passed), exit 0.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after the fixture-runner deduplication and cargo fmt ->
1 scenario (1 passed), 5 steps (5 passed), exit 0.
        ```
  - [x] COMMIT: `070b59233a80f66d8bc903bce1773ee47e81e9a6` -- Batch: `e2e-scope`

- [x] Scenario: `An ise-effective change whose E2E run reports a skipped step fails the tasks check`
  - Design constraints:
    - Fixture: the same shape with the runner writing `skipped = 1` and exiting
      nonzero. `RunnerResult::read(_, "e2e")` refuses a nonzero skipped count
      even when the runner exits 0, so the fixture's exit code is not the only
      guard.
    - Then `stderr names the failing E2E run and reports 1 skipped test`; no
      receipt is written. The count differs from the unbound row, so the two
      scenarios discriminate.
  - [x] Implement the scenario and its tests. Evidence:
        ```
RED: same method as the unbound row — the scenario ran against the base tree with the uncommitted
production files reverted.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
features_runner -- --exact "verify-change/givn-workflow-enforcement.feature::An ise-effective
change whose E2E run reports a skipped step fails the tasks check"
  ✘ Then the command exits non-zero
    Step panicked. Captured output: expected failure, got exit Some(0): stdout=Mandatory checks
    for 'tasks': ⚠ task preflight: review.md is missing ✓ change verification receipt:
    1 scenario(s), 0 failed, 0 skipped at 471863bac104294e10fd4b05618ec5f8605f410c ✓ verify passed
1 scenario (1 failed), 3 steps (2 passed, 1 failed), runner exit 101.
Production: the same run_change_e2e_scope path; the fixture runner writes failed=0/skipped=1 for
GIVN_RESULT_SCOPE=e2e, and read_change_e2e_result refuses it with "1 skipped test" — a different
count from the unbound row, so one generic message cannot satisfy both.
GREEN: same command -> 1 scenario (1 passed), 5 steps (5 passed), exit 0.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree -> 1 scenario (1 passed), 5 steps (5 passed), exit 0.
        ```
  - [x] COMMIT: `070b59233a80f66d8bc903bce1773ee47e81e9a6` -- Batch: `e2e-scope`

- [x] Scenario: `An ise-effective change receipt binds the declared e2e package, the E2E runner command, and the Givn version`
  - Design constraints:
    - Fixture: two resolved packages, `tests/e2e` and `tests/e2e-support`, both
      named by the E2E runner command, neither with a dependency edge into
      `source`; the delta has one `@e2e` scenario; the fixture runner writes
      `total/passed = 1` for `GIVN_RESULT_SCOPE=e2e` and a green regular scope.
    - `givn check tasks` exits 0; the receipt carries `e2e_identity` with two
      `package_digests` keys, `runner_command_digest` equal to
      `digest_text(verify.e2e_command)`, `givn_version` equal to the built
      binary's version, and `e2e_scope.status` equal to `passed` with the four
      counts.
    - The four Then steps parse the receipt JSON and compare domain values; none
      compares a raw byte string.
  - [x] Implement the scenario and its tests. Evidence:
        ```
RED: same method — the scenario ran against the base tree with the uncommitted production files
reverted.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test
features_runner -- --exact "verify-change/givn-workflow-enforcement.feature::An ise-effective
change receipt binds the declared e2e package, the E2E runner command, and the Givn version"
  ✔ Then the command exits 0
  ✔ And the change verification receipt records a digest of every resolved e2e package
  ✔ And the change verification receipt records the E2E runner command digest
  ✔ And the change verification receipt records the running Givn version
  ✘ And the change verification receipt records a passed E2E scope
    assertion `left == right` failed: the receipt does not record a passed E2E scope: {"change":
    "ise-boundary","command":"./emit-two-package-result.sh",... ,"e2e_identity":{"givn_version":
    "0.9.20","package_digests":{"tests/e2e":"58765007…","tests/e2e-support":"4f54c43f…"},
    "runner_command_digest":"802e0b2c…"},...} left: None right: Some("passed")
1 scenario (1 failed), 7 steps (6 passed, 1 failed), runner exit 101. The identity half of the
receipt already held both packages; only the E2E scope was missing.
Production: src/cli/check.rs records ScopeOutcome::passed with the four counts and recaptures the
identity after the run; src/proof.rs serialize_skips nothing — both optional fields already existed.
GREEN: same command -> 1 scenario (1 passed), 7 steps (7 passed), exit 0; the receipt carries
package_digests keys tests/e2e and tests/e2e-support, runner_command_digest equal to
digest_text("./emit-two-package-result.sh tests/e2e tests/e2e-support"), givn_version 0.9.20, and
e2e_scope {status: passed, total: 1, passed: 1, failed: 0, skipped: 0}.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree -> 1 scenario (1 passed), 7 steps (7 passed), exit 0.
        ```
  - [x] COMMIT: `070b59233a80f66d8bc903bce1773ee47e81e9a6` -- Batch: `e2e-scope`

---

### Batch 6: `served-policy`

The three added policy scenarios. The `specs.md` conditional paragraph and the
corrected `ise.md` bullet are already committed (`1016f3bc`); this batch binds
and proves them from the served text, so each new binding is a regression test
against prose drift. No production edit is expected; if an assertion fails, the
served text is missing a reviewed decision and the batch reports it instead of
weakening the step.

**Files modified:** `tests/steps/prose_consolidation_steps.rs` (new ise-effective
fixture, served-text reads, and the assertions). Reuse
`serve_extended_rules_steps::served_instruction` and
`request_change_guidance`; bind no new phrase in another file.

**Design constraints (batch-wide):**
- Fixture: `common::make_init_root()` (a Git-backed project, `ise` off), then
  `givn addons enable ise`, which materializes `givn/specification.yaml`; a
  change exists when `givn instructions specs` needs `--change`
  (`request_change_guidance` creates `guidance-amendments` when the fixture has
  none).
- The ise-off fixture uses `make_init_root()` unchanged, so the `{{else}}`
  branch renders.
- Assertions compare normalized served text (`split_whitespace().join(" ")`) so
  line wrapping does not decide a result.
- The scenario asserts the policy sentences, not the template markers; a served
  `{{#if` proves the render failed.
- No `@e2e` tag is added or removed; these are regular scenarios.

**Batch verification:** the three single-scenario commands below, plus
`givn instructions specs --change enforce-ise-e2e-boundary` and
`givn instructions ise` on this repository as a served-text spot check (this
repository has `ise` off, so the paragraph must be absent here and the `ise`
request must refuse by name).
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: served-policy'`.

- [x] Scenario: `Canonical policy names the declared e2e package as the E2E target`
  - Design constraints:
    - The served `specs` instruction with `addons.ise` effective contains: the
      declared package holds the E2E runner and every `@e2e` step definition;
      the package links no production source; `verify.e2e_command` runs it; an
      unbound or skipped step fails the E2E run; feature files stay shared.
    - Then steps assert the four sentences the feature names, each as a
      normalized substring of the served content. The runner sentence belongs to
      the `pinned-policy-replacements` batch (see `## Contradictions` 4).
  - [x] Implement the scenario and its tests. Evidence:
        ```
Existing behavior protected: the four sentences were committed in `1016f3bc`; the new
bindings read them from the served instruction, never from the raw asset.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test features_runner
  -- --exact "create-change/prose-consolidation.feature::Canonical policy names the declared e2e
  package as the E2E target"
  ✔ Given a Givn project with ise effective
  ✔ When the author reads the canonical E2E guidance
  ✔ Then the policy states that the declared e2e package holds the E2E runner and every E2E step definition
  ✔ And the policy states that the package links no production source
  ✔ And the policy states that an unbound or skipped step fails the E2E run
  ✔ And the policy states that feature files stay shared
1 scenario (1 passed), 6 steps (6 passed), exit 0.
Not tautological: the same request on a fresh `ise`-off fixture serves neither the
`Declared E2E package` paragraph nor any of the four sentences, so the ise-off render cannot
satisfy this scenario (`givn instructions specs --change guidance-amendments` in a `/tmp`
fixture; the served text carries no `{{#if`). No production edit.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after `cargo fmt` -> 1 scenario (1 passed), 6 steps (6 passed),
exit 0.
        ```
  - [x] COMMIT: `1fdb7e8e828bbf3699b771ee60f5fac2e3554430` -- Batch: `served-policy`

- [x] Scenario: `Served ise guidance keeps component scenarios in the Specification`
  - Design constraints:
    - `givn instructions ise` with `addons.ise` effective serves the
      `two-layer test boundary` bullet; the three Then steps assert that
      component scenarios stay in the Specification unchanged, that only their
      step definitions and fakes are implementation, and that the phrase
      `two-layer test boundary` is present.
    - The scenario states no gate: the bullet is an obligation, not a check.
  - [x] Implement the scenario and its tests. Evidence:
        ```
Existing behavior protected: the corrected bullet was committed in `1016f3bc`; the new
bindings serve `givn instructions ise` in an ise-effective fixture.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test features_runner
  -- --exact "create-change/prose-consolidation.feature::Served ise guidance keeps component
  scenarios in the Specification"
  ✔ Given a Givn project with ise effective
  ✔ When the author reads the served ise guidance
  ✔ Then the guidance states that component scenarios stay in the Specification unchanged
  ✔ And the guidance states that only their step definitions and fakes are implementation
  ✔ And the guidance keeps the two-layer test boundary
1 scenario (1 passed), 5 steps (5 passed), exit 0.
Not tautological: the ise-off fixture refuses the same request by name ("instruction 'ise'
requires the 'ise' addon; enable it with givn addons enable ise"), so the wording is reachable
only through the addon. No production edit.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after `cargo fmt` -> 1 scenario (1 passed), 5 steps (5 passed),
exit 0.
        ```
  - [x] COMMIT: `1fdb7e8e828bbf3699b771ee60f5fac2e3554430` -- Batch: `served-policy`

- [x] Scenario: `Canonical policy keeps the in-process allowance while ise is off`
  - Design constraints:
    - The fixture has `ise` off (the default init posture). The served `specs`
      instruction contains the unconditional allowance ("may run in process")
      and does not require artificial services, containers, or browser
      infrastructure.
    - Then steps assert the allowance and the absence of the ise-effective
      restriction: the served text must not contain `only from the declared`, so
      an ise-off project reads no false package requirement.
  - [x] Implement the scenario and its tests. Evidence:
        ```
Existing behavior protected: the `{{else}}` branch committed in `1016f3bc` already serves the
unconditional allowance; the Given asserts the served text carries no `only from the declared`,
so the two ise renders discriminate each other.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test features_runner
  -- --exact "create-change/prose-consolidation.feature::Canonical policy keeps the in-process
  allowance while ise is off"
  ✔ Given a Givn project with ise off whose product is a library without a service or browser
  ✔ When the author reads the canonical E2E guidance
  ✔ Then consumer calls to the public library API may run in process as real-interface evidence
  ✔ And the policy does not require artificial services containers or browser infrastructure
1 scenario (1 passed), 4 steps (4 passed), exit 0.
The absence assertion fails if the ise branch renders while the addon is off; the batch
`served-policy` scenario above fails if it does not render while the addon is on. No production
edit.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after `cargo fmt` -> 1 scenario (1 passed), 4 steps (4 passed),
exit 0.
        ```
  - [x] COMMIT: `1fdb7e8e828bbf3699b771ee60f5fac2e3554430` -- Batch: `served-policy`

---

### Batch 7: `pinned-policy-replacements`

The two `@givn.modified` scenarios that replace pinned permanent titles. Their
step text changes (an `ise`-effective Given and one added Then each), so each
needs new bindings; the added assertions are the same reviewed sentences batch
`served-policy` proves, scoped to the library and shared-runner cases.

**Files modified:** `tests/steps/prose_consolidation_steps.rs`.

**Design constraints (batch-wide):**
- New Given `a Givn project with ise effective whose product is a library
  without a service or browser`: `make_init_root()`, `givn addons enable ise`,
  and a normalized read of the served `specs` instruction. It does not reuse
  `a project whose product is a library without a service or browser`, which
  reads the raw asset and describes the ise-off posture.
- New Then `consumer calls to the public library API may run in process only
  from the declared e2e package as real-interface evidence` asserts both the
  in-process allowance and the `only from the declared` restriction adjacent to
  the declared package, so the ise-off sentence cannot satisfy it.
- New Then `a call that needs production source becomes a component scenario`
  asserts the `component scenario` sentence.
- New Then `the E2E execution runs the declared e2e package which links no
  production source` asserts the shared-runner paragraph's conditional sentence.
- The permanent titles are unchanged in `givn/specs/`; archive merges the
  `@givn.modified` replacements. `givn lint` reports the two modified titles as
  replacements, not duplicates.
- The scenarios keep their existing When and unchanged Then steps verbatim, so
  every retained assertion still passes.

**Batch verification:** the two single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: pinned-policy-replacements'`.

- [x] Scenario: `Canonical policy accepts the library public API as its real interface`
  - Design constraints:
    - Replaces the permanent
      `givn/specs/create-change/prose-consolidation.feature:123-128` scenario by
      exact title.
    - Given and Then bindings as above; the two unchanged Then steps keep their
      existing bindings (`consumer calls to the public library API may run in
      process as real-interface evidence` is replaced by the new phrase, and
      `the policy does not require artificial services containers or browser
      infrastructure` is reused).
    - A project with `ise` off must not satisfy the new Then, so the new
      assertion cannot degrade into the old allowance.
  - [x] Implement the scenario and its tests. Evidence:
        ```
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test features_runner
  -- --exact "create-change/prose-consolidation.feature::Canonical policy accepts the library public
  API as its real interface"
First run failed on a retained Then, not on the added one:
  ✔ Given a Givn project with ise effective whose product is a library without a service or browser
  ✔ When the author reads the canonical E2E guidance
  ✔ Then consumer calls to the public library API may run in process only from the declared e2e package as real-interface evidence
  ✔ And a call that needs production source becomes a component scenario
  ✘ And the policy does not require artificial services containers or browser infrastructure
    Step panicked. Captured output: E2E guidance demands infrastructure the product does not have
1 scenario (1 failed), 5 steps (4 passed, 1 failed), runner exit 101.
Cause: the `ise` branch opens a sentence ("Do not require artificial services, ...") while the
`ise`-off branch continues the bullet ("do not require ..."); the retained step matched only the
lowercase form. Fix in the glue only (the decision and the committed prose are unchanged): the
negation is matched on lowercased served text, so the same step still reads the ise-off branch.
GREEN: same command -> 1 scenario (1 passed), 5 steps (5 passed), exit 0.
The added Then asserts the contiguous "may run in process only from the declared `e2e` package",
so the ise-off sentence ("may run in process; do not require ...") cannot satisfy it; the ise-off
served text carries no `only from the declared` (asserted by the `served-policy` batch).
No production or prose edit.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after `cargo fmt` -> 1 scenario (1 passed), 5 steps (5 passed),
exit 0. The permanent `Canonical policy accepts the library public API as its real interface`
(raw-asset fixture) and the `served-policy` ise-off scenario also re-ran green, so the
case-insensitive match changed no retained reading.
        ```
  - [x] COMMIT: `137bdebdd4c81b16269fcea275194f5a4a6b7fe3` -- Batch: `pinned-policy-replacements`

- [x] Scenario: `Canonical policy selects E2E scope explicitly rather than by command shape`
  - Design constraints:
    - Replaces the permanent
      `givn/specs/create-change/prose-consolidation.feature:130-135` scenario by
      exact title.
    - Keeps the explicit-scope, equal-count, and zero-failure Then steps
      verbatim; adds the Then asserting the E2E execution runs the declared
      package, which links no production source.
    - The existing `regular and E2E verification share one runner command` Given
      is reused together with the ise-effective fixture, so the added sentence
      is read from the rendered instruction.
  - [x] Implement the scenario and its tests. Evidence:
        ```
Existing behavior protected: the five retained Then sentences were committed in `1016f3bc`; the
Given now serves `specs`, `design`, `tasks`, and `review` from an ise-effective fixture instead of
reading the raw assets, so the added sentence is read from the render.
GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs cargo test --locked --test features_runner
  -- --exact "create-change/prose-consolidation.feature::Canonical policy selects E2E scope
  explicitly rather than by command shape"
  ✔ Given regular and E2E verification share one runner command
  ✔ When the author reads the canonical verification guidance
  ✔ Then explicit scope selection such as GIVN_RESULT_SCOPE distinguishes the executions
  ✔ And equal test counts or command strings do not invalidate honestly isolated scopes
  ✔ And the E2E execution runs the declared e2e package which links no production source
  ✔ And required executions and zero failed or skipped tests remain required without an extra isolation run
1 scenario (1 passed), 6 steps (6 passed), exit 0.
Not tautological: the raw asset would satisfy the added sentence from the `{{#if addons.ise}}`
branch whatever the addon state; the served ise-off text contains no "The E2E execution runs the
declared `e2e` package". The permanent pinned scenario re-ran green on the same Given, so no
retained assertion depended on the raw asset. No production or prose edit.
        ```
  - [x] REFACTOR: re-run the scenario; must still pass.
        ```
REFACTOR: re-run on the final tree after `cargo fmt` -> 1 scenario (1 passed), 6 steps (6 passed),
exit 0.
        ```
  - [x] COMMIT: `137bdebdd4c81b16269fcea275194f5a4a6b7fe3` -- Batch: `pinned-policy-replacements`

---

### Batch 8: `review-closure`

The review found that the boundary preflight ran after the no-behavior-change
return, so an `ise`-effective change could skip the acceptance boundary by
declaring no behavior change. The preflight now runs before that return in both
the tasks check and the review receipt check. This batch adds the one delta
scenario that pins the order; the fix itself also restores the shared
`stderr names` assertion and adds the unit tests recorded in `review.md`
`## Finding closure`.

**Files modified:** `src/cli/check.rs` (preflight order in
`run_change_verification`; preflight in the not-applicable branch of
`check_change_receipt`), `tests/steps/e2e_boundary_support.rs`,
`tests/steps/workflow_enforcement_steps.rs`,
`givn/changes/enforce-ise-e2e-boundary/specs/verify-change/givn-workflow-enforcement.feature`,
`docs/reference.md`.

**Design constraints (batch-wide):**
- `design.md:399-403`: the boundary preflight runs before the
  no-behavior-change return, so an `ise`-effective change always needs a
  readable declaration and a non-empty resolved set.
- The not-applicable branch writes no receipt, so the preflight is the only
  boundary evidence review can require there; an applicable change keeps its
  receipt freshness reasons, including `the E2E identity cannot be read`.
- The refusal names the declaration path and the reason; no receipt is written.

- [x] Scenario: `An ise-effective change with no behavior change still needs a readable e2e declaration`
  - Design constraints:
    - Fixture: the `ise-boundary` change with `ise: true` in its lock, a
      proposal whose `Behavior change:` entry is `none`, `.givn-skip` naming
      `specs`, the delta feature removed, and no `givn/specification.yaml`.
    - Then `stderr names the absent e2e declaration` asserts the boundary
      refusal, the declaration path, and `is absent`.
    - Both commands are driven: `givn check tasks` and `givn check review`;
      each must exit nonzero for the same absent declaration.
  - [x] Implement the scenario and its tests (test-first per `design.md`).
        Files: `src/cli/check.rs`, `tests/steps/e2e_boundary_support.rs`,
        `tests/steps/workflow_enforcement_steps.rs`. Evidence:
        ```
RED (reverted preflight order: the two check calls moved back below the
not-applicable return, `cargo test --locked --test features_runner -- --exact
"verify-change/givn-workflow-enforcement.feature::An ise-effective change with no
behavior change still needs a readable e2e declaration"`):
  ✔  Given an ise-effective change with no behavior change and no readable e2e declaration
  ✔  When the developer runs `givn check tasks --change ise-boundary`
  ✘  Then the command exits non-zero
     Step panicked. Captured output: expected failure, got exit Some(0): stdout=Mandatory checks for 'tasks':
       changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)
       ✓ verify passed
  -> 1 scenario (1 failed), 3 steps (2 passed, 1 failed)
GREEN (preflight restored):
  ✔  Given / When `givn check tasks` / Then exits non-zero / And stderr names the absent e2e declaration
  ✔  When `givn check review` / Then exits non-zero / And stderr names the absent e2e declaration
  -> 1 scenario (1 passed), 7 steps (7 passed), exit 0
        ```
  - [x] REFACTOR: clean up, no behaviour change. Re-run the scenario.
        ```
REFACTOR: `cargo fmt --all` and the recorded clippy run changed no behaviour;
the scenario re-ran green in the whole-delta run below and the Closeout runs.
        ```
  - [x] COMMIT: `3b2c293d81b97048488ceeb71d12c702f2dd01df` -- Batch: `review-closure`

---

## Delta end-to-end scope

The delta adds no end-to-end scenario: 0 of the 19 delta scenarios carry the
`@e2e` tag, and the boundary refusals, receipt inspections, and served-policy
cases are variants of actions whose end-to-end scenarios already exist
permanently (see `## Interaction coverage pre-flight`). Consequences, recorded
once here:

- **No new step file.** design.md's `*_e2e_steps.rs` rows keep their existing
  content; no new binding is added to `workflow_enforcement_e2e_steps.rs` or
  `prose_consolidation_e2e_steps.rs`.
- **No evidence capture.** The terminal contract collects no transcript, because
  no delta end-to-end scenario runs. The terminal baseline is the refusal text
  the regular scenarios assert.
- **Both executions still run.** `./run-tests.sh --e2e` must exit 0 on the
  permanent scope and on the delta scope (`GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs`),
  proving the change removed no end-to-end coverage; the delta end-to-end count
  is 0 of 19.
- **Strict mode is unchanged.** The `.fail_on_skipped()` proof from `## Setup`
  covers both scopes; the delta needs no second probe.

---

## Closeout

- [x] Every delta phrase is bound in a permanent per-capability step file and no
      delta step copy exists under this change's `specs/`:
      `find givn/changes/enforce-ise-e2e-boundary/specs -name '*steps*' -print`
      prints nothing, and every new binding lives in
      `tests/steps/workflow_enforcement_steps.rs`,
      `tests/steps/prose_consolidation_steps.rs`, or the helper-only
      `tests/steps/e2e_boundary_support.rs` (the archive gate re-checks the same
      condition before mutation).
      ```
$ find givn/changes/enforce-ise-e2e-boundary/specs -name '*steps*' -print
(no output, exit 0)
$ for f in tests/steps/workflow_enforcement_steps.rs tests/steps/prose_consolidation_steps.rs; do
    printf '%s ' "$f"; grep -c '^#\[\(given\|when\|then\)' "$f"; done
tests/steps/workflow_enforcement_steps.rs 226
tests/steps/prose_consolidation_steps.rs 67
      ```
- [x] No empty or no-op stub remains: `grep -rn 'unimplemented!\|todo!()'
      tests/steps/` prints no step body (a Rust source fixture string is not a
      step body; name it if it matches).
      ```
$ grep -rn 'unimplemented!\|todo!()' tests/steps/
tests/steps/outline_report_support.rs:581: "... pub fn shapes(... todo!() } }"
The only hit is the Rust source fixture string passed to the outline analyzer,
not a step body. No empty or assertion-free step body exists.
      ```
- [x] No delta scenario still carries `@wip`
      (`givn check tasks --change enforce-ise-e2e-boundary` refuses otherwise).
      ```
$ grep -rn '@wip' givn/changes/enforce-ise-e2e-boundary/specs/
(no output, exit 1)
      ```
- [x] `givn lint --change enforce-ise-e2e-boundary` reports no end-to-end intent
      finding: the delta carries no end-to-end scenario, and this file uses no
      end-to-end section heading or per-scenario end-to-end task line, so
      `has_e2e_intent_in_tasks` stays false.
      ```
$ givn lint --change enforce-ise-e2e-boundary; echo $?
2 SHAPE advisory (the two pairs dispositioned in review.md "Overlap dispositions")
0 LONG
117 SUBST advisory
givn lint: 2 file(s) checked — clean
0
      ```
- [x] Whole-change verification, in this order:
      ```
      cargo fmt --all -- --check
      cargo clippy --workspace --all-targets -- -D warnings
      cargo nextest run
      ./run-tests.sh --non-e2e
      ./run-tests.sh --e2e
      GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs ./run-tests.sh --non-e2e
      GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs ./run-tests.sh --e2e
      givn lint --change enforce-ise-e2e-boundary
      ```
      Every executed scenario passes with zero failures and zero in-scope skips.
      Record the counts; the delta holds 19 scenario names (26 executed cases
      from the `gvn-workflow-enforcement` Outlines and scenarios, 5 from
      `prose-consolidation`) and 0 end-to-end scenarios.
      ```
$ cargo fmt --all -- --check            -> exit 0
$ cargo clippy --locked --workspace --all-targets -- -D warnings
    Finished `dev` profile ... in 0.06s -> exit 0
$ cargo nextest run
    Summary [ 181.859s] 1622 tests run: 1622 passed, 0 skipped
$ ./run-tests.sh --non-e2e
    Summary [ 160.791s] 963 tests run: 963 passed, 111 skipped
$ ./run-tests.sh --e2e
    Summary [  38.990s] 111 tests run: 111 passed, 963 skipped
$ GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs ./run-tests.sh --non-e2e
    Summary [   3.302s] 19 tests run: 19 passed, 0 skipped
    (the raw cucumber run of the same delta reports 31 scenarios, 31 passed,
    166 steps, 166 passed)
$ GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs ./run-tests.sh --e2e
    Summary [   0.000s] 0 tests run: 0 passed, 19 skipped
    warning: no tests to run -> exit 0 (the delta carries no @e2e scenario)
$ givn lint --change enforce-ise-e2e-boundary
    givn lint: 2 file(s) checked — clean -> exit 0
      ```
- [x] `givn check tasks --change enforce-ise-e2e-boundary` exits 0 and records
      the change verification receipt for the delta.
      ```
$ givn check tasks --change enforce-ise-e2e-boundary; echo $?
Mandatory checks for 'tasks':
  ✓ scenario commits passed (19 scenario(s))
  ...
  2 features
  31 scenarios (31 passed)
  166 steps (166 passed)
  ✓ change verification receipt: 31 scenario(s), 0 failed, 0 skipped at 32256f5f0b728e16b81e9b4f6f97731338f090f1
  ✓ verify passed
0
Receipt (verification/tasks-verification.json): schema 2, head 32256f5f, result
{"scope":"change","total":31,"passed":31,"failed":0,"skipped":0}, runner_exit 0.
It is ise-off shaped: this change's own config-lock.yaml does not have ise
effective, so it carries no e2e_identity and no e2e_scope key.
      ```
- [x] `givn check review --change enforce-ise-e2e-boundary` passes the
      scenario-commit, integrity, routing, and add-on lock checks. The review
      verdict is the reviewer's, not the author's.
      ```
$ givn check review --change enforce-ise-e2e-boundary; echo $?
Mandatory checks for 'review':
  ✓ integrity checks passed
  ✓ run declaration passed
  [SHAPE] / [SUBST] rows are advisory only
  givn lint: 2 file(s) checked — clean
  net delta: 17 added, 2 modified, 0 removed
  ✓ overlap dispositions passed
  capability 'givn-workflow-enforcement' already owned by givn/specs/verify-change
  capability 'prose-consolidation' already owned by givn/specs/create-change
  ✓ scenario commits passed (19 scenario(s))
  ✓ change verification receipt reused across documentation-only revision
    65696b7f3ef232e533c2ff7ae5b1a26c18845644
    (tested at 32256f5f0b728e16b81e9b4f6f97731338f090f1, 31 scenario(s))
  ✓ scenario commits passed (19 scenario(s))
0
The reviewer flipped the verdict after closing every finding; the gate now
reads `REVIEW: PASS`.
      ```

**Done when:** every box is checked with its batch `COMMIT:` revision,
`./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the permanent
scope and on the delta scope, `givn check tasks --change
enforce-ise-e2e-boundary` exits 0, and the two drifted-prose contradictions
above are reported rather than silently repaired.
