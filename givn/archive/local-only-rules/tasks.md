# Tasks: local-only-rules

Owning contracts: fragment `support` capabilities `rules-policy`,
`local-only-transition`, `publication-cleanliness`, `attribute-commits`;
use case `initialize-project` capability `initialize-a-project`; use case
`upgrade-a-project` capability `apply-a-managed-upgrade`.
Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
`external-contributor--open-source-consumer`, `givn-maintainer--owner`.

Implementation note: the change was implemented as one unit and committed per
capability. The RED evidence is the first delta-suite run before the fixes
(`14 tests: 6 passed, 6 failed, 2 not run`); each task below records the
targeted GREEN after its capability landed, and the final delta suite is
15/15. The archive merges the delta and runs both scopes against the
candidate.

## Setup

- [x] Change scaffolded; proposal, specs, design, design-review, arc42
      complete.
- [x] Runner strict mode: `.fail_on_skipped()` in `tests/features_runner.rs`
      (existing); the first delta run exited non-zero on the new scenarios.
- [x] Planning artifacts committed. COMMIT: `55b3dcb1` -- `docs(local-only-rules): plan the change`

## Implementation

### rule policy (fragment `rules-policy`)

- [x] RED/GREEN: fresh init records `rules: local-only`; explicit
      `versioned` is preserved and reported; an invalid value warns and
      continues local-only; versioned sync clears the ignore block.
      Files: `src/config/overlay.rs`, `src/cli/init.rs`,
      `assets/config.yaml.tmpl`, `givn/config.yaml`,
      `tests/steps/local_only_rules_steps.rs`, `tests/steps/mod.rs`,
      `specs/fragments/fragment.md`, `specs/fragments/rules-policy.feature`.
      Targeted runs: 3 scenarios pass.
- [x] COMMIT: `73b5d226` -- `feat(rules-policy): resolve the generated-rules policy`

### local-only transition and setup (`initialize-project`, fragment `local-only-transition`)

- [x] RED/GREEN: a tracked rule becomes local-only with its file kept and
      the removal staged; the transition is idempotent; an orphan rule from
      a disabled addon is transitioned; init writes both ignore destinations
      and never stages `AGENTS.md`; the disabled-addon scenario is updated.
      Files: `src/skills/rules.rs`, `src/skills/mod.rs`,
      `src/skills/targets.rs`, `src/git.rs`, `src/fossil.rs`, `src/scm.rs`,
      `specs/fragments/local-only-transition.feature`,
      `specs/initialize-project/*`.
      Targeted runs: 5 scenarios pass.
- [x] COMMIT: `357c97b6` -- `feat(local-only-transition): keep tracked rules out of the repository`

### commit stage set (capability `attribute-commits`)

- [x] RED/GREEN: the no-path stage set excludes `AGENTS.md` and the strict
      rules under local-only and still excludes `AGENTS.md` under versioned.
      Files: `src/cli/commit.rs`, `specs/fragments/attribute-commits.feature`.
      Targeted runs: 3 scenarios pass.
- [x] COMMIT: `f94c26b8` -- `feat(attribute-commits): stage project content only under local-only`

### upgrade (`upgrade-a-project`)

- [x] RED/GREEN: the managed revision carries the scaffold and removals and
      never `AGENTS.md`; the upgrade reports reachable-history occurrences;
      the managed-revision e2e step requires removals under Git; the
      attribution fixtures use the moved hook home and opt in to versioned.
      Files: `src/cli/migration.rs`,
      `specs/upgrade-a-project/apply-a-managed-upgrade.feature`,
      `tests/steps/release_0_3_0_e2e_steps.rs`,
      `tests/steps/llm_attribution_installation_e2e_steps.rs`,
      `tests/steps/llm_attribution_e2e_steps.rs`.
      Targeted runs: 2 scenarios pass; e2e scope green.
- [x] COMMIT: `0644838b` -- `feat(apply-a-managed-upgrade): publish a policy-aware managed revision`

### dedicated check (fragment `publication-cleanliness`)

- [x] RED/GREEN: `givn check rules` names a tracked rule and exits
      non-zero; exits 0 on a clean project; names reachable-history
      occurrences. Files: `src/cli/check.rs`,
      `specs/fragments/publication-cleanliness.feature`.
      Targeted runs: 3 scenarios pass.
- [x] COMMIT: `ff7fb5cd` -- `feat(publication-cleanliness): report tracked and historical rules`

## Verification

- [x] Change delta suite — `GIVN_FEATURES=givn/changes/local-only-rules/specs ./run-tests.sh`
      — green. Evidence: `Summary [1.530s] 15 tests run: 15 passed,
      0 skipped`.
- [x] `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`
      — clean.
- [x] `givn lint --change local-only-rules` — clean (advisory overlap
      findings only).
- [x] Pre-merge permanent suite: one expected failure from the modified
      `enable-optional-addons` scenario; the archive merges the delta and
      verifies the candidate. Evidence: the archive verification receipt.
- [x] E2E pre-merge: `./run-tests.sh --e2e` — green.
- [x] Review artifact complete and signed off.
- [x] Archive the change.
