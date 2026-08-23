# Tasks: release-0-3-0

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

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
- [ ] Add the release feature to the active test matrix and configure the
      existing strict Cucumber runner to execute it through `GIVN_FEATURES`.
- [ ] Establish a temporary Git-project fixture that can create a 0.2.0
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

- [ ] Scenario: `The changelog describes the complete 0.3.0 release`
  - [ ] RED: assert that `CHANGELOG.md` is absent or lacks the required
        release and migration sections. Run the single scenario only. Must
        fail.
        ```
        Before adding `CHANGELOG.md`, the release scenario could not read the required file from the repository; the absent-file precondition failed.
        ```
  - [ ] GREEN: add `CHANGELOG.md` with the complete user-visible 0.3.0 scope
        relative to the 0.2.0 master branch and the migration entry point.
        ```
        `cargo test --test features_runner -- --name "The changelog describes the complete 0.3.0 release"` passed: `1 scenario (1 passed)`. File: `CHANGELOG.md`.
        ```
  - [ ] REFACTOR: verify headings and links are stable, concise, and do not
        claim unfinished distribution or semantic work as complete.
        ```
        The changelog separates shipped behavior from the incomplete ADR-063/distribution boundary; the rerun passed with `1 scenario (1 passed)`.
        ```
  - [ ] COMMIT: `<hash>` — `docs(release): add 0.3.0 changelog`

---

## Scenario: The migration guide is an executable prompt sequence

- [ ] Scenario: `The migration guide is an executable prompt sequence`
  - [ ] RED: assert the public migration guide or its ordered prompt markers
        are missing. Run the single scenario only. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: write `docs/migrations/0.2.0-to-0.3.0.md` as the canonical
        LLM prompt pack. Cover evidence inventory, machine-safe upgrade,
        synchronization, active/permanent specs, overlap/semantic review,
        Arc42/ADR reconciliation, verification, and archive.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: check that every prompt names inputs, allowed edits,
        forbidden automatic decisions, expected evidence, and the next Givn
        artifact/command; remove duplicated or contradictory policy.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `docs(migration): add 0.2.0 to 0.3.0 LLM guide`

---

## Scenario: A clean 0.2.0 project receives a reviewable 0.3.0 migration plan

- [ ] Scenario: `A clean 0.2.0 project receives a reviewable 0.3.0 migration plan`
  - [ ] RED: invoke the current CLI in an isolated clean 0.2.0 Git fixture
        and assert the new config marker, commits, migration plan, or recorded
        hash is missing. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: implement the exact 0.2.0→0.3.0 path. Preflight Git and
        cleanliness, snapshot managed files, refresh config, synchronize
        generated integrations, stage only the managed allowlist, create
        Commit A, generate the maintenance proposal/design with its hash and
        `.givn-skip`, then create Commit B. List files touched.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: centralize path calculation and Git error handling; verify
        failures never reset history, generated project overrides remain
        untouched, and Commit B contains only the migration plan.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `feat(upgrade): prepare automatic 0.2.0 migration`

---

## Scenario: An unclean project is protected from automatic migration commits

- [ ] Scenario: `An unclean project is protected from automatic migration commits`
  - [ ] RED: add an uncommitted user file/change to a 0.2.0 Git fixture and
        invoke `givn upgrade`. Assert the command succeeds or writes anything.
        Must fail against the intended safety contract.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: reject dirty status before config, skill, migration, staging,
        or commit writes; report how to obtain a clean boundary.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: share the preflight with the non-Git guard and assert all
        original bytes and Git history remain unchanged.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `fix(upgrade): refuse dirty migration worktrees`

---

## Scenario: Repeating the upgrade does not duplicate the migration

- [ ] Scenario: `Repeating the upgrade does not duplicate the migration`
  - [ ] RED: run the successful preparation twice and assert the second run
        creates another migration change or commit. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: detect the target maintenance change in active or archive
        storage, preserve it byte-for-byte, and report an already-prepared
        state without a second commit.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: make the idempotency check deterministic for active and
        archived plans and keep the normal non-migration upgrade behavior
        unchanged.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `fix(upgrade): make migration preparation idempotent`

---

## Scenario: A project outside Git is protected from a partial automatic migration

- [ ] Scenario: `A project outside Git is protected from a partial automatic migration`
  - [ ] RED: invoke `givn upgrade` in a 0.2.0 fixture without a Git repository
        and assert that the config changes. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: reject the automatic route before any write with an actionable
        message explaining that Git is required for the requested commit
        boundary.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: ensure the diagnostic distinguishes missing Git metadata
        from a dirty worktree and does not create a partial migration folder.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `fix(upgrade): require git for automatic migrations`

---

## Scenario: An existing migration plan is never overwritten

- [ ] Scenario: `An existing migration plan is never overwritten`
  - [ ] RED: create a sentinel migration proposal/design in a 0.2.0 fixture,
        run `givn upgrade`, and assert the sentinel is overwritten or a second
        plan appears. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: detect the deterministic active/archive path before writing,
        preserve its bytes, and report the existing plan without duplication.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: document the partial-upgrade recovery path and keep all
        generated migration content deterministic.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `fix(upgrade): preserve existing migration plans`

---

## Scenario: The migration change remains honest until the project review passes

- [ ] Scenario: `The migration change remains honest until the project review passes`
  - [ ] RED: inspect the generated plan and assert it already contains a
        fabricated pass marker or implementation commit. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: generate only proposal, detailed design, and `.givn-skip`;
        include project-specific task prompts and no fabricated design-review,
        task, implementation, or review evidence.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: keep the plan aligned with the public guide and assert that
        normal Givn review/archive gates remain the completion authority.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `docs(upgrade): keep generated migration evidence honest`

---

## Arc42 and ADR documentation

- [ ] Update the affected durable Arc42 chapters and the change-level
      `arc42.md` with a full 12-row assessment.
- [ ] Amend ADR-023 with the automatic migration-plan and two-commit protocol;
      update both ADR indexes if its metadata or relationship changes.
- [ ] Record `ARC42 CONFORMANCE: CLEAN` evidence in `review.md` only after
      comparing implementation, design, tasks, and durable chapters.

---

## Full verification

- [ ] Run `givn lint --change release-0-3-0` and resolve all findings.
- [ ] Run the non-E2E change suite through `GIVN_FEATURES`; paste the output.
- [ ] Run the E2E change suite through `GIVN_FEATURES`; paste the output.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Run repository coverage and classify every uncovered region.
- [ ] Complete `review.md` with `REVIEW: PASS` only after all gates pass.

---

**Done when:** every box is checked with evidence and commit hash, both
`./run-tests.sh` and `./run-tests.sh --e2e` exit 0 for the change, the release
metadata is 0.3.0, and the automatic migration path is verified in an isolated
Git fixture.
