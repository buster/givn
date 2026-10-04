# Tasks: release-truth

RED→GREEN→REFACTOR→COMMIT per scenario. Non-`@e2e` scenarios first
(S1–S4), then the E2E setup task, then the `@e2e` scenario (S5); the
workflow, documentation, and review tasks close the change.

Legend: an unchecked box means pending; a box is checked only when its
evidence and revision identifier are filled in. Check each box immediately
after the step — tasks.md must always reflect true state; never batch-check
at the end.

**Release note (shared subject of every revision in this change):**

> One repository workflow prepares and publishes every Givn release, and its
> changelog lists each user-visible change exactly once, taken from the
> change's release note.

**Delta inventory**: 5 delta records = 5 added; 5 executable scenarios =
4 regular + 1 `@e2e`. All scenarios stay `@wip` until their own task clears
the tag.

**Work order**: setup → S1 → S2 → S3 → S4 → E2E setup → S5 → workflow and CI
→ durable docs → review → archive.

---

## Global constraints (apply to every task)

- **Use-case context**: the owning use case is `publish-a-release`
  (capability `describe-the-release`). Its rules relevant here: a release
  section lists only user-visible change with one entry per change taken from
  the release note; a release starts at a version tag matching the package
  version; a range without user-visible change produces no release. Its
  success guarantee is "the release describes itself truthfully"; the
  confirmed Persona is `givn-maintainer--owner`.
- **Runner**: `tests/features_runner.rs` (cucumber-rs 0.23,
  `.fail_on_skipped()`) is the only runner. `verify.command` is
  `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`
  (`givn/commands.yaml`).
- **Scoped commands**:
  - single scenario (cargo):
    `GIVN_FEATURES=givn/changes/release-truth/specs cargo test --locked --test features_runner -- --exact 'publish-a-release/describe-the-release.feature::<exact title>'`
  - single scenario (nextest):
    `GIVN_FEATURES=givn/changes/release-truth/specs cargo nextest run --test features_runner --exact 'publish-a-release/describe-the-release.feature::<exact title>'`
  - whole delta: `GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh`
    (`--non-e2e` for regular only, `--e2e` for the smoke scope)
  - Never combine a name filter with a tag expression.
- **Step files, one binding module per kind, exactly as design.md names
  them**: regular steps in `tests/steps/release_steps.rs`, `@e2e` steps in
  `tests/steps/release_e2e_steps.rs`, both registered in
  `tests/steps/mod.rs`. The e2e module reuses the generator helper from the
  regular module.
- **Step bodies are never empty.** An unimplemented step uses
  `unimplemented!()`; a body counts as implemented only with its real
  assertion or real setup action.
- **Behavioral RED discipline**: write the scenario's real assertions first
  and run them against the pre-change surface with the exact fixture. A
  missing binding, a compile error, or an `unimplemented!()` panic is not
  the RED for a scenario that has a behavioral assertion. Record the RED
  category honestly: `OLD_PRODUCT_RED` (the delivered behavior fails the new
  assertion) or `ORACLE_RED` (the old contract already passes; prove
  discrimination with a controlled omission and restore it immediately).
- **Production edit set** (design.md §Production edit set): `cliff.toml`,
  `scripts/release-guard.sh`, `.github/workflows/release.yml` (rewritten),
  `.github/workflows/prepare-release.yml` (deleted), `.github/workflows/ci.yml`,
  `tests/steps/{release_steps.rs,release_e2e_steps.rs,mod.rs}`,
  `givn/specs/publish-a-release/usecase.md` (via the delta narrative),
  `docs/arc42/{07-deployment-view.md,11-risks-and-technical-debt.md,12-glossary.md}`,
  `AGENTS.md`. Nothing else; `src/**` is untouched.
- **`givn commit` stages every pending change.** Only the intended files may
  be dirty when a scenario commit runs; otherwise split afterwards.
- **Fixture configuration**: the fixture sets `user.name`, `user.email`,
  `commit.gpgsign=false`, and `core.hooksPath` to an empty directory locally,
  so host git configuration cannot reach it.

---

## Task 0 — Setup: strictness proof, baseline, and command forms

- [x] Run the delta's regular scope before any binding exists and paste the
      non-zero undefined-step failure (proof of strict mode):
      `GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --non-e2e`
  - OUTPUT: exit 100; `Summary: 4 tests run: 0 passed, 4 failed, 1 skipped`;
    sample: `Step doesn't match any function` at
    `publish-a-release/describe-the-release.feature:7:5` (full log
    `/tmp/setup-regular.log`).
- [x] Run the delta's e2e scope before any binding exists and paste the
      non-zero failure:
      `GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --e2e`
  - OUTPUT: exit 100; `Summary: 1 test run: 0 passed, 1 failed, 4 skipped`
    (the four regular scenarios are filtered out in this scope).
- [x] Run the permanent corpus baseline and paste the exit status:
      `./run-tests.sh`
  - OUTPUT: exit 0; `Summary: 578 tests run: 578 passed, 0 skipped`
    (45.9 s).
- [x] Prove the single-scenario command form on one delta scenario and paste
      the targeted failure:
      `GIVN_FEATURES=givn/changes/release-truth/specs cargo test --locked --test features_runner -- --exact 'publish-a-release/describe-the-release.feature::A release section lists only the user-visible change groups'`
  - OUTPUT: exit 101; `1 feature / 1 scenario (1 failed) / 1 step (1 failed)`;
    `Step doesn't match any function` at
    `publish-a-release/describe-the-release.feature:7:5`.
- [x] Confirm `verify.command` and `verify.e2e_command` from
      `givn/commands.yaml` (no change needed) and that `git-cliff 2.13.1`,
      `git`, `fossil`, `git-filter-repo`, `script`, and `jq` are on PATH.
  - OUTPUT: `verify.command: ./run-tests.sh`, `verify.e2e_command:
    ./run-tests.sh --e2e`; `git-cliff` `/home/buster/.cargo/bin/git-cliff`
    (version 2.13.1), `git` `/usr/bin/git`, `fossil`
    `/home/buster/bin/fossil`, `git-filter-repo` `/usr/bin/git-filter-repo`,
    `script` `/usr/bin/script`, `jq` on PATH; `cargo-nextest 0.9.143`.
- [x] COMMIT: `760a3872` -- Setup: strictness proof, baseline, and command forms

## Task S1 — A release section lists only the user-visible change groups

Owning rule: a release section lists only user-visible change. Production
change owned by this scenario: the internal-scope fix skip and the `.*` skip
fallback in `cliff.toml`.

- [x] RED: implement the fixture and the scenario's assertions in
      `tests/steps/release_steps.rs` against the current `cliff.toml`; run
      the single scenario; paste the failure
  - OUTPUT: exit 101; `1 scenario (1 failed)`, 6 steps with 1 failed:
    `And the changelog lists no "Other Changes" group` —
    `Step panicked. Captured output: the changelog lists an unexpected Other Changes group:`
    followed by the generated section showing `### Other Changes` and
    `- **harness:** Retarget the fixture scaffold` under `### Bug Fixes`
    (full log `/tmp/s1-red.log`).
- [x] GREEN: apply the parser change; rerun the scenario; paste the pass
  - OUTPUT: exit 0; `1 scenario (1 passed)`, `6 steps (6 passed)`; production
    files: `cliff.toml` (internal-scope fix skip, `.*` skip fallback),
    `tests/steps/release_steps.rs`, `tests/steps/mod.rs`
    (log `/tmp/s1-green.log`).
- [x] REFACTOR: tidy the helper without behavior change; rerun; paste the pass
  - OUTPUT: `cargo fmt --all` applied to the new module; exit 0;
    `1 scenario (1 passed)`, `6 steps (6 passed)` (log `/tmp/s1-refactor.log`).
- [x] COMMIT: `b5f91a1c` -- describe-the-release: A release section lists only the user-visible change groups

## Task S2 — A release range without user-visible change has no release section

Owning rule: a range without user-visible change produces no release section.
Production change owned by this scenario: the `^docs` and `^refactor` skip
rules that complete the allowlist in `cliff.toml`.

- [x] RED: add the scenario's steps and run it; paste the failure showing the
      remaining `Documentation`/`Refactoring` groups
  - OUTPUT: exit 101; `1 scenario (1 failed)`, `Then the release section lists
    no change group` — `Step panicked. Captured output: the release section
    lists change groups ["### Documentation", "### Refactoring"]:` followed by
    the generated section (log `/tmp/s2-red.log`).
- [x] GREEN: complete the allowlist; rerun; paste the pass
  - OUTPUT: exit 0; `3 steps (3 passed)`; production file: `cliff.toml`
    (`^docs` and `^refactor` skip; the narrower `docs(givn)`/`docs(plan)`
    rules became redundant); S1 regression rerun also exit 0
    (log `/tmp/s2-green.log`).
- [x] REFACTOR: rerun; paste the pass
  - OUTPUT: `cargo fmt --all` applied to the long Given attribute; exit 0;
    `3 steps (3 passed)` (log `/tmp/s2-refactor.log`).
- [x] COMMIT: `4071f575` -- describe-the-release: A release range without user-visible change has no release section

## Task S3 — A release cannot be prepared without a tag for the current version

Owning rule: a release starts at a version tag matching the package version.
Production change owned by this scenario: `scripts/release-guard.sh` with the
`base` check.

- [x] RED: add the scenario's steps, run it against the absent script, and
      paste the literal failure and the failing named-tag assertion
  - OUTPUT: exit 101; 5 steps with 1 failed: `Then the check fails naming the
    missing tag "v0.8.0"` — `Step panicked. Captured output: the failure does
    not name the missing tag "v0.8.0":` followed by `sh: 0: cannot open
    /home/buster/projects/givn/scripts/release-guard.sh: No such file`
    (behavior absent, recorded as the RED; no stub was introduced in its
    place; log `/tmp/s3-red.log`).
- [x] GREEN: add `scripts/release-guard.sh` with the `base` subcommand; make
      it executable; rerun; paste the pass
  - OUTPUT: first iteration failed on the message wording (it named `0.8.0`
    and `v0.4.0` but not the expected tag); the message now names the
    expected tag, exit 0, `5 steps (5 passed)`; production file:
    `scripts/release-guard.sh` (`base`; log `/tmp/s3-green.log`).
- [x] REFACTOR: rerun; paste the pass
  - OUTPUT: `cargo fmt --all` applied to the new steps; exit 0; `5 steps
    (5 passed)`; the delta's regular scope is green for S1–S3 with only S4
    still unbound (log `/tmp/s3-refactor.log`).
- [x] COMMIT: `5ddbad58` -- describe-the-release: A release cannot be prepared without a tag for the current version

## Task S4 — A release section without user-visible change stops the release

Owning rule: a range without user-visible change stops the release before the
version revision and the tag. Production change owned by this scenario: the
`section` subcommand of `scripts/release-guard.sh`.

- [x] RED: add the scenario's steps and run it against the script without the
      `section` subcommand; paste the failure
  - OUTPUT: exit 101; 4 steps with 1 failed: `Then the check fails naming the
    release version "0.9.0"` — `Step panicked. Captured output: the failure
    does not name the release version "0.9.0":` followed by
    `usage: .../scripts/release-guard.sh base <expected-version>` (behavior
    absent; log `/tmp/s4-red.log`).
- [x] GREEN: add the `section` subcommand; rerun; paste the pass
  - OUTPUT: exit 0; `4 steps (4 passed)`; the delta's regular scope is fully
    green (`4 tests run: 4 passed, 1 skipped`); production file:
    `scripts/release-guard.sh` (`section`; log `/tmp/s4-green.log`).
- [x] REFACTOR: rerun; paste the pass
  - OUTPUT: formatting already clean; `sh -n scripts/release-guard.sh` ok;
    exit 0; `4 steps (4 passed)` (log `/tmp/s4-refactor.log`).
- [ ] COMMIT: `<hash>` -- describe-the-release: A release section without user-visible change stops the release

## Task E2E setup — e2e scope, skeleton, and scenario-count proof

- [x] Create `tests/steps/release_e2e_steps.rs` with the `@e2e` bindings
      (shared generator helper imported from the regular module) and register
      it in `tests/steps/mod.rs`.
  - OUTPUT: the module binds the four `@e2e`-specific steps with
    `unimplemented!()` bodies; the shared generator `When` stays in the
    regular module.
- [x] Prove the e2e scope is a strict subset: run the delta's regular scope
      and the e2e scope, paste both scenario counts, and state the relation.
  - OUTPUT: regular scope `4 tests run: 4 passed, 1 skipped` (exit 0); e2e
    scope `1 test run: 0 passed, 1 failed, 4 skipped` (exit 100). e2e count 1
    is strictly smaller than the regular count 4.
- [x] Confirm strict mode for the e2e runner by running the `@e2e` scenario
      before its binding exists and pasting the non-zero failure.
  - OUTPUT: exit 100; `Step panicked. Captured output: not implemented:
    implemented by the dedupe scenario task` — the stub fails instead of
    silently passing (log `/tmp/e2e-setup.log`).
- [ ] COMMIT: `<hash>` -- E2E setup: e2e scope, skeleton, and scenario-count proof

## Task S5 — Repeated revisions of one change yield one changelog entry (`@e2e`)

Owning rule: one entry per change, taken from the release note. Production
change owned by this scenario: the `unique(attribute="message",
case_sensitive=true)` dedupe in the `cliff.toml` body template.

- [ ] RED: add the scenario's bindings and run it; paste the failure showing
      the same release note three times
  - OUTPUT: _pending_
- [ ] GREEN: add the dedupe filter; rerun; paste the pass
  - OUTPUT: _pending_
- [ ] REFACTOR: rerun; paste the pass
  - OUTPUT: _pending_
- [ ] Capture the terminal transcript to
      `givn/changes/release-truth/evidence/visual/repeated-revisions-of-one-change-yield-one-changelog-entry/transcript.txt`.
- [ ] COMMIT: `<hash>` -- test(e2e): Repeated revisions of one change yield one changelog entry

## Task W — Release workflow and CI

Production change: the single dispatch release workflow, the deletion of the
split workflow, and the CI install of the pinned changelog tool. Verification
bound (design.md A2): the guard predicates are scenarios S3/S4; the remaining
wiring is checked structurally here and reviewed in the archive.

- [ ] Rewrite `.github/workflows/release.yml` (dispatch with `version`/`tag`;
      prepare job: branch/clean check, version checks, pinned tools, base
      guard call, version bump, Cargo verification, changelog generation with
      the section guard, acceptance prerequisites, nextest, suites, security
      checks, release build and `--version`, release commit, annotated tag,
      identity record, token push; release job: tag resolution, identity
      validation including the section guard, idempotent GitHub release).
- [ ] Delete `.github/workflows/prepare-release.yml`.
- [ ] Add the pinned `git-cliff 2.13.1` cache and install step to
      `.github/workflows/ci.yml`.
- [ ] Structural smoke: parse every workflow as YAML; run the shell syntax
      check on every extracted `run:` block; confirm the trigger is
      `workflow_dispatch` only; confirm the three version checks, both guard
      calls, the idempotence guard, and the `RELEASE_PUSH_TOKEN` push are
      present; confirm no crates.io publication step exists; run
      `git grep 'prepare-release'` excluding `givn/archive/**` and
      `assets/migrations/**` and paste the result.
  - OUTPUT: _pending_
- [ ] COMMIT: `<hash>` -- chore(release): one dispatch workflow for prepare, tag, and GitHub release

## Task D — Durable documentation

- [ ] Update `AGENTS.md` (acceptance prerequisites name `git-cliff`) and
      confirm the chapter updates for `docs/arc42/07-deployment-view.md`,
      `docs/arc42/11-risks-and-technical-debt.md`, and
      `docs/arc42/12-glossary.md` match the implementation.
  - OUTPUT: _pending_
- [ ] COMMIT: `<hash>` -- docs(release-truth): record the pipeline, risks, and vocabulary

## Task R — Review

- [ ] Write `review.md` with every template section (fabrication audit,
      commit cross-check, promised-architecture cross-check, strict-mode
      proof, e2e scope and isolation checks, use-case/persona conformance,
      design conformance, interaction coverage, visual review with the
      committed transcript, retirements, arc42 conformance, coverage
      classification, execution evidence, overlap dispositions, sign-off).
- [ ] Run `givn check review --change release-truth` until it passes; paste
      the final invocation result.
  - OUTPUT: _pending_
- [ ] COMMIT: `<hash>` -- review(release-truth): coverage sign-off

## Definition of done

All boxes checked with evidence and unique commit hashes; both
`./run-tests.sh` and `./run-tests.sh --e2e` exit 0 on the merged corpus; the
e2e setup task records a strictly smaller e2e scenario count; then
`givn archive --change release-truth` merges the delta, publishes the
candidate, verifies both scopes, and finalizes.
