# Tasks: maintain-project

Owning behavior: the existing `maintain-project` use case, capability
`release-0-3-0`, plus the retirement of a `scm-backend-selection` fragment
scenario.

Domain constraints copied from the use-case contract
(`givn/changes/maintain-project/specs/maintain-project/usecase.md`):

- The managed upgrade and migration bundles run on both backends with the same guarantees (Q8): clean preflight, calculated allowlist, reviewable revisions.
- A migration-free managed upgrade publishes exactly one revision and no plan revision.
- The plan revision records the upgrade revision identifier.
- Ignored managed paths are force-staged only through the calculated allowlist, on both backends.
- A dirty checkout or a missing backend is refused by name before any write.
- Confirmed Personas: developer--primary-user, givn-maintainer--owner (review lenses).

## Setup

- [x] Runner contract confirmed: `./run-tests.sh` plus `GIVN_FEATURES` for the
      change scope; `./run-tests.sh --e2e` strictly narrower.
- [x] Strict mode proven with a targeted RED run: with the production changes
      stashed (the temporary Fossil upgrade refusal restored), the new Fossil
      scenarios exited non-zero:

      ```text
      ✘ Then the command exits successfully
        Step panicked. Captured output: upgrade failed: ... the Fossil backend is not available for the managed upgrade in this version
      1 scenario (1 failed), 3 steps (2 passed, 1 failed)  /  exit 1
      2 scenarios (2 failed), 4 steps (2 passed, 2 failed) /  exit 1
      ```

- [x] E2E count proof for the change scope:

      ```text
      GIVN_FEATURES=givn/changes/maintain-project/specs ./run-tests.sh
      Summary: 4 tests run: 4 passed, 0 skipped
      GIVN_FEATURES=givn/changes/maintain-project/specs ./run-tests.sh --e2e
      Summary: 2 tests run: 2 passed, 2 skipped
      ```

## Scenario: A clean 0.2.0 project receives a reviewable aggregate migration plan

Domain constraint: the managed revision contains only the allowlisted managed scaffold and generated integration changes; the plan records the upgrade revision.

- [x] RED: with the production changes stashed the outline's Fossil example
      exited non-zero on the restored refusal (setup evidence above).
- [x] GREEN: `legacy_fossil_project` sets the ignore-glob before the initial
      check-in so the generated agent files stay unmanaged; the managed
      revision's paths come from `fossil diff --brief --checkin <uuid>` (the
      first child check-in at `plink` offset 1, since `fossil init` seeds an
      initial empty check-in); the plan design records that identifier.
      Targeted run:

      ```text
      1 feature, 2 scenarios (2 passed), 18 steps (18 passed)  /  exit 0
      ```

- [x] REFACTOR: rustfmt on the touched files; run still green.
- [x] COMMIT: 7f31901e -- the outline scenario's refactor commit; its RED+GREEN landed in a224e40f.
      fragment retirement.

## Scenario: A clean Fossil project receives the same managed upgrade revision shape

Domain constraint: a migration-free upgrade publishes exactly one revision and no plan revision, on both backends.

- [x] RED: shared with the setup stash run (the migration-free scenario
      failed on the restored refusal).
- [x] GREEN: `current_fossil_project_with_stale_integration` commits the whole
      scaffold; the upgrade publishes exactly one check-in (count 3 including
      Fossil's initial empty check-in) whose subject contains
      `update managed files`; no migration change exists. Targeted run:

      ```text
      1 feature, 1 scenario (1 passed), 6 steps (6 passed)  /  exit 0
      ```

- [x] REFACTOR: shared.
- [x] COMMIT: d14dba32 -- the migration-free scenario's refactor commit; RED+GREEN in a224e40f.

## Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits

Domain constraint: the dirty preflight refuses before any write, on both backends.

- [x] RED: shared with the setup stash run (the dirty scenario failed before
      the fix only on the missing config snapshot; the refusal wording was
      already accepted).
- [x] GREEN: the Fossil dirty fixture captures the config snapshot; the
      refusal names `clean checkout`; `no revision is created` pins the
      check-in count at 2 (Fossil's initial check-in plus the fixture); the
      byte-for-byte step branches on the backend and reads
      `fossil changes --differ`. Targeted run:

      ```text
      1 feature, 1 scenario (1 passed), 5 steps (5 passed)  /  exit 0
      ```

- [x] REFACTOR: shared.
- [x] COMMIT: b6ee8145 -- the dirty-protection scenario's refactor commit; RED+GREEN in a224e40f.

## Scenario: A Fossil checkout is refused by name until the Fossil backend is available (retired)

Domain constraint: the refusal no longer exists; the permanent fragment
scenario is deleted in the same commit that implements the Fossil upgrade.

- [x] GREEN: the scenario and its `then_fossil_refusal` binding are deleted;
      the fragments suite stays green (18 features, 116 scenarios, 548 steps).
- [x] COMMIT: a224e40f -- the implementation slice that deleted the permanent scenario and its binding.

## Finalize

- [x] Full suite green: `./run-tests.sh` 477/477 and `./run-tests.sh --e2e`
      84/84.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean;
      `cargo fmt --all -- --check` clean.
- [x] `givn lint --change maintain-project` exits 0.
- [x] All tasks above checked with evidence and commit hashes. The three
      scenarios plus the retirement share one implementation commit because
      the seam operations, fixtures, and permanent alignment land together;
      the commit is `a224e40f feat(release-0-3-0): the managed upgrade publishes allowlist-only revisions on both backends` (plan artifacts in `176cc8e4`).
