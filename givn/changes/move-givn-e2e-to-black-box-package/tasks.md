# Tasks: move-givn-e2e-to-black-box-package

The change moves Givn's `@e2e` tier out of the product-linked test target and
into the standalone `e2e/` package, then points the E2E verification scope at
that package. Work is organised as behavior batches; each batch is one atomic
`givn commit` whose message carries the release note as its subject and names
the batch in the body.

Owning use cases: `verify-change` (`givn-workflow-enforcement`) and
`fragments` (`adr-qualification-gate`). Design: `design.md`. Confirmed Personas:
`agent--technical-role`, `givn-maintainer--owner`.

Runner contract after this change:

- regular scope: `./run-tests.sh --non-e2e` (in-process component tier,
  `not @e2e and not @givn.removed`)
- E2E scope: `./run-tests.sh --e2e --package e2e` (the package, `@e2e`)
- default: `./run-tests.sh` runs the regular scope, then the E2E scope

## Setup: strict-mode proof

- [x] **Strict mode**: the package runner calls `.fail_on_skipped()` and exits
      nonzero when `failed > 0`, `skipped > 0`, or no scenario ran.
      Proof (scratch feature with one undefined step, run through the package):
      `GIVN_FEATURES=/tmp/e2e-strict/undefined.feature ... cargo test --locked
      --manifest-path e2e/Cargo.toml --test e2e_runner` exits 1 and writes
      `{"scope":"e2e","total":1,"passed":0,"failed":1,"skipped":0}`. An
      undefined step is reported as one failed test, and `fail_on_skipped`
      converts any skipped step into a failure, so a skipped step cannot pass
      silently either. The unmodified package: 119 scenarios (119 passed),
      exit 0.

## Batch 1: the standalone black-box package

Release note: `Move the @e2e acceptance corpus into a standalone package that links no product crate`

- [x] **Package manifest and lockfile**: `e2e/Cargo.toml` declares the
      `givn-e2e` package, its own `[workspace]`, `publish = false`, explicit
      test-only dependency versions, and `[[test]] name = "e2e_runner"
      harness = false`; `e2e/Cargo.lock` is committed; the root `Cargo.toml`
      `exclude` list gains `e2e/**`.
- [x] **Runner preflight**: `e2e/tests/e2e_runner.rs` reads `e2e/Cargo.lock`
      and exits 2 when a `[[package]]` named `givn`, `givn-config`, or
      `givn-core` appears, then requires an executable `GIVN_BIN`; the scope
      filter selects `@e2e`, excludes `@givn.removed`, and writes the four-count
      result with `GIVN_RESULT_SCOPE`.
- [x] **Harness**: `e2e/tests/harness/**` resolves `GIVN_BIN` and
      `GIVN_READY_FIXTURE`, builds child environments with `env_clear()` plus
      `PATH`, `HOME`, `LLVM_PROFILE_FILE`, and `FOSSIL_USER`, mirrors the
      receipt and record file formats, and keeps a local `scenario_slug`.
- [x] **Step migration**: every step an `@e2e` scenario binds moved to
      `e2e/tests/steps/`; the 23 product-calling bindings were rewritten as the
      design's inventory says (local slug, literal narratives, local target
      lists, receipts through the binary or the recorded file format, local
      record type). 30 of the 31 `tests/steps/*_e2e_steps.rs` files were deleted
      from the component tree; `llm_attribution_e2e_steps.rs` stays because
      non-`@e2e` attribution scenarios bind its fixture builders, and its
      `@e2e` definitions are duplicated in the package.
- [x] **Observable check**: `./run-tests.sh --e2e --package e2e` →
      `119 scenarios (119 passed)`, exit 0. `cargo tree --locked
      --manifest-path e2e/Cargo.toml` names no `givn`, `givn-config`, or
      `givn-core` package.
- [x] **Seeded-defect discrimination**: applying
      `scripts/qualify/status-verification-current.patch` (makes `givn status`
      report `Verification: current` with no receipt) and rebuilding gives
      `119 scenarios (118 passed, 1 failed)`, exit 1; reverting it restores
      `119 scenarios (119 passed)`.
- [x] RED: the moved corpus had no home until the package existed: before the
      move the `@e2e` definitions were bound only by the product-linked
      `features_runner`, and an `e2e/Cargo.toml` did not exist. The first
      package run before the step migration failed with unbound steps; the
      migrated run is the GREEN above.
- [x] COMMIT: `132d9ea3` -- Batch: standalone black-box package
- [x] Scenario: A change whose declared E2E package is a standalone workspace passes the tasks check and binds the E2E scope
- [x] COMMIT: `132d9ea3` -- Batch: standalone black-box package

## Batch 2: the typed package runner name

Release note: `Name the E2E package through --package in the declared runner command`

- [x] **Runner-target rule**: no product change. The predecessor's rule in
      `src/e2e_boundary/mod.rs` accepts any whole token equal to the declared
      root or carrying `<root>/`; the `--package` value `e2e` is such a token,
      so `./run-tests.sh --e2e --package e2e` satisfies it. A bare `--e2e` tier
      flag names no root. Review finding F1 proved that a `--package` window
      branch changed no result; it and its three unit tests are deleted, and
      the accepted command stays covered by the delta scenario and the outline
      row.
- [x] **Command**: `givn/commands.yaml` `verify.e2e_command` becomes
      `./run-tests.sh --e2e --package e2e`.
- [x] **Project runner**: `run-tests.sh` builds `givn` and `ready_fixture`
      unless `GIVN_BIN` is already an executable, canonicalizes
      `GIVN_FEATURES`, `GIVN_RESULT_FILE`, and the binary paths, exports
      `GIVN_BIN`/`GIVN_READY_FIXTURE`, and runs the package with `e2e/` as the
      working directory. `--e2e` without `--package e2e` is refused.
- [x] **Delta Given**: `tests/steps/e2e_boundary_support.rs` gains
      `ise_change_with_standalone_workspace` (root manifest excluding the
      package, `[workspace]` package manifest, a fixture runner, the delta
      `@e2e` scenario), bound by the new Given in
      `tests/steps/workflow_enforcement_steps.rs`.
- [x] **Observable check**: the delta features run:
      `GIVN_FEATURES=givn/changes/move-givn-e2e-to-black-box-package/specs
      cargo test --locked --test features_runner -- --tags
      "@givn-workflow-enforcement or @adr-qualification-gate"` →
      `6 scenarios (6 passed); 37 steps (37 passed)`.
- [x] GREEN with no product change: the accepted-command scenario passes
      through the pre-existing whole-token rule, so no RED is claimed for it.
      It still discriminates: the review's mutation (`CargoPackage::read`
      refuses a `[workspace]` manifest) makes it fail at `Then the command
      exits 0`. The outline's `./run-tests.sh --e2e` row discriminates too:
      accepting the bare flag makes exactly that row fail.
- [x] COMMIT: `1282b9cd` -- Batch: typed package runner name
- [x] Scenario: An ise-effective change whose E2E runner command names no declared e2e root fails the tasks check
- [x] COMMIT: `1282b9cd` -- Batch: typed package runner name

## Batch 3: the project declaration

Release note: `Declare the project's retained roles and its e2e acceptance package`

- [x] **Declaration**: `givn/specification.yaml` writes the six-key schema with
      `behavior: ["givn/specs/**"]`, `structure`, `contracts`, `limits`,
      `e2e: ["e2e"]`, and `source: ["src/**", "tests/**"]`. Inert until the
      `ise` addon is effective.
- [x] **Observable check**: `givn lint` exits 0 with the declaration present
      and the `ise` addon off (`givn lint: 2 file(s) checked, 3 finding(s)` are
      the three `@wip` delta scenarios).
- [x] COMMIT: `9b63979c` -- Batch: project declaration

## Batch 4: coverage and browser assets

Release note: `Measure the E2E scope through the instrumented product binary`

- [x] **Coverage**: `measure-coverage.sh` builds the instrumented `givn` with
      `cargo llvm-cov --no-run --no-report --locked --bin givn`, exports
      `GIVN_BIN` and `LLVM_PROFILE_FILE`, and calls
      `./run-tests.sh --e2e --package e2e`; the runner never rebuilds an
      exported binary. The guard names the artifact: with a no-op `cargo` in
      `PATH` the script exits 2 with `the instrumented givn binary is missing at
      <root>/target/llvm-cov-target/debug/givn`. The reports under `coverage/`
      predate the source edit, so they are not evidence for this change; archive
      regenerates `coverage/non-e2e-cobertura.xml` and
      `coverage/e2e-cobertura.xml` from the merged candidate instead.
- [x] **Browser assets**: `browser-driver.mjs`, `package.json`,
      `package-lock.json`, and `.gitignore` moved to `e2e/browser/`;
      `setup-e2e-browser.sh` installs from there; a missing driver fails the
      browser scenario by name (`the browser driver exited before answering
      'goto'`), never a skip. The component-tier `tests/common/browser.rs` is
      deleted: it spawned the moved driver at `tests/e2e/browser-driver.mjs`
      and no component scenario binds a browser step.
- [x] **Observable check**: the browser scenarios in
      `./run-tests.sh --e2e --package e2e` pass
      (`development-report.feature` browser scenarios included in the 119).
- [x] COMMIT: `9b63979c` -- Batch: coverage and browser assets
      (the coverage script, the browser setup, and the qualification script
      landed in the declaration commit)

## Batch 5: qualification

Release note: `Prove the acceptance package fails on a seeded product defect`

- [x] **Script**: `scripts/qualify-givn-e2e.sh` copies the checkout to a
      scratch directory, requires `cargo tree --manifest-path
      e2e/Cargo.toml` to name no product crate, runs the suite green, applies
      `scripts/qualify/status-verification-current.patch`, requires the suite
      to fail, reverts the patch, and requires the suite to pass again.
- [x] **Observable check**: with the patch applied and the product rebuilt the
      package reports `119 scenarios (118 passed, 1 failed)`, exit 1; after the
      revert `119 scenarios (119 passed)`, exit 0.
- [x] COMMIT: `9b63979c` -- Batch: qualification

## Batch 6: the architecture record

Release note: `Record the standalone acceptance package as ADR-079`

- [x] **ADR and register**: `docs/arc42/adr/ADR-079-standalone-acceptance-package.md`
      accepted, register carries 31 accepted records and no proposed record,
      the permanent scenario asserts the same count.
- [x] **Chapters and docs**: `docs/arc42/{02,04,05,06,07,08,11}`,
      `docs/glossary.md` (`E2E package`), and `docs/reference.md` record the
      package, the declared runner command name, the lockfile guard, and the
      coverage path.
- [x] **Observable check**: two scopes after the move. Component:
      `GIVN_FEATURES=givn/specs/fragments cargo test --locked --test
      features_runner -- --tags "@adr-qualification-gate and not @e2e"` →
      `17 scenarios (17 passed); 91 steps (91 passed)`. Package:
      `GIVN_CUCUMBER_TAGS='@e2e and @adr-qualification-gate' ./run-tests.sh
      --e2e --package e2e` → `1 scenario (1 passed); 7 steps (7 passed)`. The
      18th `@adr-qualification-gate` scenario is the permanent `@e2e` one, bound
      only in the package after the move.
- [x] COMMIT: `cec46e95` -- Batch: architecture record
- [x] Scenario: The migration preserves independent architecture history across three proposed decisions
- [x] COMMIT: `cec46e95` -- Batch: architecture record

## Final review gate

- [x] `givn check review --change move-givn-e2e-to-black-box-package` exits 0 at
      the review PASS revision `d271dc64`, with the receipt reused across the
      documentation-only revision.
      ```
      $ givn check review --change move-givn-e2e-to-black-box-package
        ✓ integrity checks passed
        ✓ run declaration passed
        givn lint: 2 file(s) checked — clean
        net delta: 1 added, 2 modified, 0 removed
        ✓ overlap dispositions passed
        ✓ scenario commits passed (3 scenario(s))
        ✓ change verification receipt reused across documentation-only revision
          d271dc6451bb416fe7418e67868474c76bad2995 (tested at
          67ad51bb1466b746879684fbe4501a2f9b2f0e70, 6 scenario(s))
      exit 0
      ```
