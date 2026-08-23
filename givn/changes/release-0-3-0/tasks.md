# Tasks: release-0-3-0

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: pending boxes · completed boxes only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

---

## Strict-mode proof (do first)

Empty/no-op steps are indistinguishable from PASS in most runners. Prove the
runner rejects an unimplemented step before any scenario work.

- [x] **Proof-of-strictness**: run the runner against a step using the
      not-implemented stub. Confirm non-zero exit.
      ```
      GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --name "The migration change remains honest until the project review passes"
      Result: non-zero. `fail_on_skipped()` executed the `unimplemented!()` step and Cucumber reported `1 step failed`.
      ```
      If exit 0, configure strict mode before proceeding.

---

## Shared setup

- [x] Add `tests/steps/release_0_3_0_steps.rs` and
      `tests/steps/release_0_3_0_e2e_steps.rs`; register both modules in
      `tests/steps/mod.rs`.
- [x] Add the release feature to the active test matrix and configure the
      existing strict Cucumber runner to execute it through `GIVN_FEATURES`.
      Evidence: `GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --tags 'not @e2e'` passed with `8 scenarios (8 passed)` and the E2E run passed with `2 scenarios (2 passed)`.
- [x] Establish a temporary Git-project fixture that can create a 0.2.0
      project, set a local author identity, capture file bytes, and inspect
      commit paths without touching the parent checkout.

---

## Scenario: The release identifies itself as its package version

- [x] Scenario: `The release identifies itself as its package version`
  - [x] RED: assert the current package/CLI still reports the pre-release
        version. Run the single scenario only. Must fail.
        ```
        `cargo test --test features_runner -- --name "The release identifies itself as its package version"` failed with `expected "0.3.0", got "givn 0.2.0\n"`.
        ```
  - [x] GREEN: bump the package and lockfile metadata so the real CLI reports
        `0.3.0`; list files touched.
        ```
        `cargo test --test features_runner -- --name "The release identifies itself as its package version"` passed: `1 scenario (1 passed)`. Files: `Cargo.toml`, `Cargo.lock`, `tests/steps/release_0_3_0_steps.rs`.
        ```
  - [x] REFACTOR: remove stale release-version assumptions in tests or docs;
        rerun the scenario.
        ```
        The assertion now compares the CLI output with `env!("CARGO_PKG_VERSION")`; the rerun passed with `1 scenario (1 passed)`.
        ```
  - [x] COMMIT: `3f02170` — `chore(release): The release identifies itself as its package version`

---

## Scenario: The changelog describes the complete 0.3.0 release

- [x] Scenario: `The changelog describes the complete 0.3.0 release`
  - [x] RED: assert that `CHANGELOG.md` is absent or lacks the required
        release and migration sections. Run the single scenario only. Must
        fail.
        ```
        Before adding `CHANGELOG.md`, the release scenario could not read the required file from the repository; the absent-file precondition failed.
        ```
  - [x] GREEN: add `CHANGELOG.md` with the complete user-visible 0.3.0 scope
        relative to the 0.2.0 master branch and the migration entry point.
        ```
        `cargo test --test features_runner -- --name "The changelog describes the complete 0.3.0 release"` passed: `1 scenario (1 passed)`. File: `CHANGELOG.md`.
        ```
  - [x] REFACTOR: verify headings and links are stable, concise, and do not
        claim unfinished distribution or semantic work as complete.
        ```
        The changelog separates shipped behavior from the incomplete ADR-063/distribution boundary; the rerun passed with `1 scenario (1 passed)`.
        ```
  - [x] COMMIT: `65d0b62` — `docs(release): The changelog describes the complete 0.3.0 release`

---

## Scenario: The migration guide is an executable prompt sequence

- [x] Scenario: `The migration guide is an executable prompt sequence`
  - [x] RED: assert the public migration guide or its ordered prompt markers
        are missing. Run the single scenario only. Must fail.
        ```
        Before adding the guide, the `Given` step failed because `docs/migrations/0.2.0-to-0.3.0.md` was absent.
        ```
  - [x] GREEN: write `docs/migrations/0.2.0-to-0.3.0.md` as the canonical
        LLM prompt pack. Cover evidence inventory, machine-safe upgrade,
        synchronization, active/permanent specs, overlap/semantic review,
        Arc42/ADR reconciliation, verification, and archive.
        ```
        `cargo test --test features_runner -- --name "The migration guide is an executable prompt sequence"` passed: `1 scenario (1 passed)`. File: `docs/migrations/0.2.0-to-0.3.0.md`.
        ```
  - [x] REFACTOR: check that every prompt names inputs, allowed edits,
        forbidden automatic decisions, expected evidence, and the next Givn
        artifact/command; remove duplicated or contradictory policy.
        ```
        The guide has five ordered copyable prompts, explicit evidence checks, and domain/ADR edit prohibitions; the rerun passed with `1 scenario (1 passed)`.
        ```
  - [x] COMMIT: `2eb8248` — `docs(migration): The migration guide is an executable prompt sequence`

---

## Scenario: The migration catalog defines complete release-owned bundles

- [ ] Scenario: `The migration catalog defines complete release-owned bundles`
  - [x] RED: assert that the embedded catalog schema and bundle fields are
        absent or incomplete. Run the single scenario only. Must fail.
        ```
        The pre-implementation catalog step was undefined and the strict runner returned a non-zero result.
        ```
  - [x] GREEN: add the embedded catalog and release-owned bundle assets with
        source/target/order metadata and complete Proposal/Design references.
        ```
        `cargo test --test features_runner -- --name "The migration catalog defines complete release-owned bundles"` passed: `1 scenario (1 passed)`. Files: `assets/migrations/catalog.yaml`, `assets/migrations/0.2.0-to-0.3.0/proposal.md`, and `assets/migrations/0.2.0-to-0.3.0/design.md`.
        ```
  - [x] REFACTOR: validate the catalog schema, asset existence, version edges,
        and deterministic route ordering without adding migration-specific Rust
        conditionals.
        ```
        `cargo test --lib cli::migration::tests` passed: `6 tests passed`; the release scenario rerun passed with `1 scenario (1 passed)`.
        ```
  - [ ] COMMIT: `<hash>` — `feat(migration): define release-owned migration catalog`

---

## Scenario: A clean 0.2.0 project receives a reviewable 0.3.0 migration plan

- [x] Scenario: `A clean 0.2.0 project receives a reviewable 0.3.0 migration plan`
- [x] RED: invoke the current CLI in an isolated clean 0.2.0 Git fixture
        and assert the new config marker, commits, migration plan, or recorded
        hash is missing. Must fail.
        ```
        The first implementation run failed at step binding because the generic `the consumer runs ...` step conflicted with the migration-specific CLI step; Cucumber exited non-zero before the success contract was implemented.
        ```
  - [x] GREEN: implement the exact 0.2.0→0.3.0 path. Preflight Git and
        cleanliness, snapshot managed files, refresh config, synchronize
        generated integrations, stage only the managed allowlist, create
        Commit A, generate the maintenance proposal/design with its hash and
        `.givn-skip`, then create Commit B. List files touched.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "A clean 0.2.0 project receives a reviewable 0.3.0 migration plan"` passed: `1 scenario (1 passed)`, including config, allowlisted Commit A, migration plan, Commit A hash, and no product spec. Files: `src/cli/migration.rs`, `src/cli/upgrade.rs`, `src/skills/targets.rs`, `src/cli/mod.rs`, and E2E step definitions.
        ```
  - [x] REFACTOR: centralize path calculation and Git error handling; verify
        failures never reset history, generated project overrides remain
        untouched, and Commit B contains only the migration plan.
        ```
        The rerun passed with `1 scenario (1 passed)`; managed paths are calculated centrally and staging uses explicit relative path arguments.
        ```
  - [x] COMMIT: `6157355` — `feat(upgrade): A clean 0.2.0 project receives a reviewable 0.3.0 migration plan`

---

## Scenario: A clean project without migration requirements receives one managed upgrade commit

- [ ] Scenario: `A clean project without migration requirements receives one managed upgrade commit`
  - [x] RED: invoke the upgrade in a current project with a stale generated
        integration and assert that it does not create the managed commit.
        Must fail.
        ```
        The first implementation did not yet have the migration-free managed-commit path; the scenario was not green until the catalog dispatch was implemented.
        ```
  - [x] GREEN: synchronize managed config and generated integrations, create
        one allowlist-only commit when files changed, and create no migration
        change.
        ```
        `cargo test --test features_runner -- --name "A clean project without migration requirements receives one managed upgrade commit"` passed: `1 scenario (1 passed)`. The stale generated integration was refreshed in one managed commit and no migration plan was created.
        ```
  - [x] REFACTOR: route all migration decisions through the embedded catalog,
        avoid empty commits, and preserve the same clean-worktree boundary.
        ```
        The rerun passed with `1 scenario (1 passed)`; no migration-specific Rust conditional remains in the upgrade dispatcher.
        ```
  - [ ] COMMIT: `<hash>` — `feat(upgrade): support migration-free managed refresh`

---

## Scenario: An unclean project is protected from automatic migration commits

- [x] Scenario: `An unclean project is protected from automatic migration commits`
  - [x] RED: add an uncommitted user file/change to a 0.2.0 Git fixture and
        invoke `givn upgrade`. Assert the command succeeds or writes anything.
        Must fail against the intended safety contract.
        ```
        The unimplemented dirty-fixture step failed with `not implemented: implemented in the dirty-worktree scenario`.
        ```
  - [x] GREEN: reject dirty status before config, skill, migration, staging,
        or commit writes; report how to obtain a clean boundary.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "An unclean project is protected from automatic migration commits"` passed: `1 scenario (1 passed)`. Config, user bytes, and commit count remained unchanged.
        ```
  - [x] REFACTOR: share the preflight with the non-Git guard and assert all
        original bytes and Git history remain unchanged.
        ```
        The rerun passed with `1 scenario (1 passed)`; `ensure_clean_git` is shared by both automatic migration guards.
        ```
  - [x] COMMIT: `86be977` — `fix(upgrade): An unclean project is protected from automatic migration commits`

---

## Scenario: Repeating the upgrade does not duplicate the migration

- [x] Scenario: `Repeating the upgrade does not duplicate the migration`
  - [x] RED: run the successful preparation twice and assert the second run
        creates another migration change or commit. Must fail.
        ```
        The prepared-project step failed with `not implemented: implemented in the idempotency scenario` before the idempotency behavior existed.
        ```
  - [x] GREEN: detect the target maintenance change in active or archive
        storage, preserve it byte-for-byte, and report an already-prepared
        state without a second commit.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "Repeating the upgrade does not duplicate the migration"` passed: `1 scenario (1 passed)`. The second run preserved the design and commit count.
        ```
  - [x] REFACTOR: make the idempotency check deterministic for active and
        archived plans and keep the normal non-migration upgrade behavior
        unchanged.
        ```
        The rerun passed with `1 scenario (1 passed)`; active/archive detection is centralized in `migration_exists`.
        ```
  - [x] COMMIT: `8fa1b29` — `fix(upgrade): Repeating the upgrade does not duplicate the migration`

---

## Scenario: A project outside Git is protected from a partial automatic migration

- [x] Scenario: `A project outside Git is protected from a partial automatic migration`
  - [x] RED: invoke `givn upgrade` in a 0.2.0 fixture without a Git repository
        and assert that the config changes. Must fail.
        ```
        The unimplemented non-Git-fixture step failed with `not implemented: implemented in the non-Git scenario`.
        ```
  - [x] GREEN: reject the automatic route before any write with an actionable
        message explaining that Git is required for the requested commit
        boundary.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "A project outside Git is protected from a partial automatic migration"` passed: `1 scenario (1 passed)`. The config remained byte-for-byte unchanged and the error named the Git requirement.
        ```
  - [x] REFACTOR: ensure the diagnostic distinguishes missing Git metadata
        from a dirty worktree and does not create a partial migration folder.
        ```
        The rerun passed with `1 scenario (1 passed)`; the existing shared `ensure_clean_git` preflight distinguishes the two diagnostics and no migration folder is created.
        ```
  - [x] COMMIT: `56f8b09` — `fix(upgrade): A project outside Git is protected from a partial automatic migration`

---

## Scenario: An existing migration plan is never overwritten

- [x] Scenario: `An existing migration plan is never overwritten`
  - [x] RED: create a sentinel migration proposal/design in a 0.2.0 fixture,
        run `givn upgrade`, and assert the sentinel is overwritten or a second
        plan appears. Must fail.
        ```
        The unimplemented existing-plan step failed with `not implemented: implemented in the existing-plan scenario`.
        ```
  - [x] GREEN: detect the deterministic active/archive path before writing,
        preserve its bytes, and report the existing plan without duplication.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "An existing migration plan is never overwritten"` passed: `1 scenario (1 passed)`. The sentinel proposal/design remained unchanged.
        ```
  - [x] REFACTOR: document the partial-upgrade recovery path and keep all
        generated migration content deterministic.
        ```
        The rerun passed with `1 scenario (1 passed)`; the production diagnostic now names the preserved proposal and design.
        ```
  - [x] COMMIT: `24bd7fa` — `fix(upgrade): An existing migration plan is never overwritten`

---

## Scenario: The migration change remains honest until the project review passes

- [x] Scenario: `The migration change remains honest until the project review passes`
  - [x] RED: inspect the generated plan and assert it already contains a
        fabricated pass marker or implementation commit. Must fail.
        ```
        The unimplemented honesty step failed with `not implemented: implemented by the CLI migration scenario`.
        ```
  - [x] GREEN: generate only proposal, detailed design, and `.givn-skip`;
        include project-specific task prompts and no fabricated design-review,
        task, implementation, or review evidence.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "The migration change remains honest until the project review passes"` passed: `1 scenario (1 passed)`. The generated plan has no design-review/tasks/implementation evidence.
        ```
  - [x] REFACTOR: keep the plan aligned with the public guide and assert that
        normal Givn review/archive gates remain the completion authority.
        ```
        The rerun passed with `1 scenario (1 passed)`; the generated design embeds the public guide and `givn graph` still exposes design-review, tasks, and review.
        ```
  - [x] COMMIT: `9193317` — `docs(upgrade): The migration change remains honest until the project review passes`

---

## Scenario: A completed maintenance migration can be archived without a product specification

- [x] Scenario: `A completed maintenance migration can be archived without a product specification`
  - [x] RED: invoke archive for a completed no-spec maintenance change and
        assert it succeeds without the permanent verification boundary. Must
        fail.
        ```
        The undefined archive-fixture step failed with a non-zero Cucumber result before the no-product-delta archive path existed.
        ```
  - [x] GREEN: use the permanent feature path for global verification and
        skip only the change-specific semantic gate and delta merge when
        `specs` is explicitly skipped.
        ```
        `cargo test --features spec-retrieval --test features_runner -- --name "A completed maintenance migration can be archived without a product specification"` passed: `1 scenario (1 passed)`. The maintenance change archived and permanent `givn/specs/` remained free of a migration feature. Files: `src/cli/check.rs`, `src/cli/archive.rs`, and E2E fixture steps.
        ```
  - [x] REFACTOR: keep normal product changes on their existing delta and
        semantic paths; assert the no-spec exception remains explicit and
        review/task gates still apply.
        ```
        The rerun passed with `1 scenario (1 passed)` using the retrieval-enabled binary; no-product-delta handling is conditioned on `.givn-skip` containing `specs`.
        ```
  - [x] COMMIT: `062eb7f` — `fix(archive): A completed maintenance migration can be archived without a product specification`

---

## Arc42 and ADR documentation

- [x] Update the affected durable Arc42 chapters and the change-level
      `arc42.md` with a full 12-row assessment.
- [x] Amend ADR-023 with the automatic migration-plan and two-commit protocol;
      update both ADR indexes if its metadata or relationship changes.
- [x] Record `ARC42 CONFORMANCE: CLEAN` evidence in `review.md` only after
      comparing implementation, design, tasks, and durable chapters.
      Evidence: `review.md` contains the full 12-row implementation comparison and `ARC42 CONFORMANCE: CLEAN`.

---

## Full verification

- [x] Run `givn lint --change release-0-3-0` and resolve all findings.
      Evidence: `givn lint --change release-0-3-0` -> `1 file(s) checked — clean`.
- [x] Run the non-E2E change suite through `GIVN_FEATURES`; paste the output.
      Evidence: `GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --tags 'not @e2e'` -> `8 scenarios (8 passed)`, `36 steps (36 passed)`.
- [x] Run the E2E change suite through `GIVN_FEATURES`; paste the output.
      Evidence: `GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --tags @e2e` -> `2 scenarios (2 passed)`, `13 steps (13 passed)`.
- [x] Run `cargo fmt --all -- --check`.
      Evidence: command passed with no output.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
      Evidence: command passed with no warnings.
- [x] Run repository coverage and classify every uncovered region.
      Evidence: global merged report `13186/19616` lines (`67.2206%`), release-specific report `1982/9080`, and all gaps are classified in `review.md`.
- [x] Complete `review.md` with `REVIEW: PASS` only after all gates pass.
      Evidence: `review.md` ends with `REVIEW: PASS`; the feature-enabled `givn check review --change release-0-3-0` passed verify, verify-e2e, integrity, overlap, and semantic review.

---

**Done when:** every box is checked with evidence and commit hash, both
`./run-tests.sh` and `./run-tests.sh --e2e` exit 0 for the change, the release
metadata is 0.3.0, and the automatic migration path is verified in an isolated
Git fixture.
