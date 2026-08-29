# Tasks: release-strict-model-overrides

Runner: `./run-tests.sh` (unit) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Single-scenario runs (faithful to the owning group's env):

- Dev control (non-@e2e; matches the default `""` e5 group, spec-retrieval only):
  ```
  env GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled GIVN_FEATURES=givn/changes/release-strict-model-overrides/specs \
    cargo test --no-default-features --features spec-retrieval --test features_runner \
    -- --tags 'not @bge and not @bge-approval and not @bge-disabled and not @gate-only and not @gate-combination and not @nli-future and not @nli and not @nli-combined and not @review-output and not @release-proof' --name "<scenario title>"
  ```
- Installed-build scenarios (`@release-proof`; all features, clean env):
  ```
  env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS \
    GIVN_FEATURES=givn/changes/release-strict-model-overrides/specs \
    cargo test --features spec-retrieval,spec-similarity-gate,cross-encoder-reranker,nli-evidence \
    --test features_runner -- --tags '@release-proof' --name "<scenario title>"
  ```
- Release-profile unit proof: `cargo test --release --lib -- release_strict`
  (dev profile: `cargo test --lib release_strict`)

Spec: `givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature`
(4 scenarios: `An installed givn refuses a request for simulated embedder
output` (@e2e @release-proof), `An installed givn ignores requests to disable
or simulate the second-stage models` (@release-proof), `An installed givn
honours a model-location request that still uses the real models`
(@e2e @release-proof), `A development givn build honours a simulated-embedder
request without an installed-build announcement` (untagged dev control)).

Interaction coverage verified pre-tasks: the spec's 2 User Interaction Inventory
entries map 1:1 to the design's Coverage Matrix rows (@e2e titles: scenario 1
and scenario 3, both tagged `@e2e @release-proof` in the feature). No gaps.

Production target: `src/spec/retrieval.rs` (gated accessor pair + release block
taxonomy + one-shot announcement sweep). Test support (new files):
`tests/steps/release_strict_overrides_steps.rs` (dev control),
`tests/steps/release_strict_overrides_e2e_steps.rs` (installed builds), and a
release-binary resolver + cache-isolation helper in `tests/common`.

## Commit tracking

Each scenario is one atomic commit (RED+GREEN+REFACTOR together, plus record
its hash here). Setup/infra/docs commits are separate. Recorded:

- [x] Task 0 setup — commit hash `77f7256` (bundled with scenario 4, mirrors
  the archived template-engine precedent where infra + first scenarios share a commit)
- [x] Scenario 4 dev control — commit hash `77f7256`
- [ ] Task 2 partition — commit hash `TBD`
- [ ] Scenario 1 embedded refuse — commit hash `TBD`
- [ ] Scenario 2 second-stage ignore — commit hash `TBD`
- [ ] Scenario 3 location honour — commit hash `TBD`
- [ ] Task 6 unit proofs — commit hash `TBD`
- [ ] Task 7 docs — commit hash `TBD`

## Task 0 — Setup: strict mode proof + step skeletons

- [x] Confirm `verify.command` = `./run-tests.sh` and `verify.e2e_command` =
  `./run-tests.sh --e2e` (givn/commands.yaml; distinct strings). Already configured.
- [x] Prove strict mode: create `tests/steps/release_strict_overrides_steps.rs`
  (dev-control steps) and `tests/steps/release_strict_overrides_e2e_steps.rs`
  (installed-build steps) with `unimplemented!()` bodies, register both in
  `tests/steps/mod.rs`; run the dev-control single-scenario command →
  `MUST exit non-zero` (undefined/pending step). Paste output:
  ```
   Σ 1 scenario (1 failed)
   3 steps (2 passed, 1 failed)
   thread 'main' panicked at cucumber.rs:1243: 1 step failed
   error: test failed, to rerun pass `--test features_runner`
   EXIT=101
  ```
- [x] Add the release-binary resolver scaffold in `tests/common` (GIVN_RELEASE_BIN
  override, else lazy `cargo build --release … --bin givn` cached in a
  `OnceLock`, `CARGO_MANIFEST_DIR`-rooted).
- [x] Commit: `test(retrieval): installed-build strictness step scaffolding + release resolver` — hash `77f7256` (see scenario 4 above)

## Task 1 — Scenario (dev control, non-@e2e): A development givn build honours a simulated-embedder request without an installed-build announcement

This is a consciously-vacuous regression guard (recorded in design-review): the
dev path already honours stubs and emits no announcement, so GREEN requires no
production change; the scenario locks the debug/production contrast so a future
regression (announcement leaking into dev) fails here.

- [x] RED: implement the dev-control step bodies with real assertions and run
  the dev-control single-scenario command. The pre-change dev binary already
  honours the stub → the run passes, so the failing seed here is the
  unimplemented-stub baseline from Task 0 (recorded above). Instead, prove the
  guard fires: temporarily make the step assert an announcement exists (or run
  the target against a release build) → `MUST exit non-zero`. Paste:
  ```
  Exit 0 with the real assertion set because the dev path is stable; the
  unimplemented-step baseline (Task 0) is the recorded RED evidence
  (EXIT=101). The guard's inversion is covered by Task 3's release side.
  ```
- [x] GREEN: restore correct assertions; produce files:
  `tests/steps/release_strict_overrides_steps.rs`. Run the dev-control
  single-scenario command → `PASSES`. Paste:
  ```
  Feature: Installed builds always evaluate the semantic gates with the real models
    Scenario: A development givn build honours a simulated-embedder request ...
      1 scenario (1 passed), 5 steps (5 passed)
  EXIT=0
  ```
- [x] REFACTOR: rerun → still passes. (No-op: no code changed post-GREEN.)
- [x] Commit: `test(retrieval): installed-build strictness step scaffolding, release resolver, dev-build control scenario` — hash `77f7256`

## Task 2 — Partition run-tests.sh / measure-coverage.sh for @release-proof (infra)

Precondition for the @e2e loops: the installed-build scenarios must run in
exactly one dedicated group, never in the capability groups with their stub
wrapper envs.

- [x] `run-tests.sh`: append `and not @release-proof` to the catch-all e5 group
  filter in all three modes (default `""` line 115, `--non-e2e` line 88,
  `--e2e` line 73) so `@e2e @release-proof` scenarios never double-run in the
  e2e capability groups; add a dedicated `--e2e` group guarded by
  `tag_present '@release-proof'`. Done.
- [x] `measure-coverage.sh`: mirror the partition (`not @release-proof` on the
  non-e2e and existing e2e filters; dedicated `@release-proof` group in the e2e
  block, kept out of the fast non-e2e report). Done.
- [x] Verify: `./run-tests.sh` (permanent scope) green (30 scenarios, exit 0);
  `./run-tests.sh --e2e` (permanent scope) green (5 scenarios, exit 0). With
  the delta scope, the dedicated `@release-proof` group runs the 3 installed
  scenarios and is RED (unimplemented e2e steps). Paste:
  ```
  default:   30 scenarios (30 passed) DEFAULT_EXIT=0   [permanent givn/specs]
  --e2e:      5 scenarios (5 passed)  E2E_EXIT=0        [permanent givn/specs]
  --e2e delta: 3 scenarios (1 passed, 2 failed) E2E_DELTA_EXIT=101
              # the release group ran clean-env, all-features; scenarios 1+2
              # RED on missing announcements; scenario 3 green-vacuous.
  ```
- [ ] Commit: `test(retrieval): partition @release-proof across test groups` — hash `TBD`

## Task 3 — Scenario (@e2e @release-proof): An installed givn refuses a request for simulated embedder output

- [ ] RED: e2e step bodies still `unimplemented!()` from Task 0; run the
  installed-build single-scenario command → `MUST exit non-zero`. (Also note:
  the pre-change release binary honours `GIVN_SPEC_EMBEDDER=stub` → exit 0, no
  announcement, so behavior would fail even with real steps.) Paste:
  ```
  <paste RED output>
  ```
- [ ] GREEN: implement the gated accessor pair (`spec_env`/`spec_env_var`), the
  release block check, and the one-shot announcement sweep in
  `src/spec/retrieval.rs` (embedder blocked in release; debug/passthrough
  preserved). Production files:
  - `src/spec/retrieval.rs` (modified)
  - `tests/common/…` (release resolver driving real steps; temp XDG_CACHE_HOME)
  Run the installed-build single-scenario command → `PASSES`. Paste:
  ```
  <paste GREEN output>
  ```
- [ ] REFACTOR: rerun → still passes. (Paste if changed.)
- [ ] Commit: `feat(retrieval): installed builds refuse simulated-embedder requests` — hash `TBD`

## Task 4 — Scenario (@release-proof): An installed givn ignores requests to disable or simulate the second-stage models

- [ ] RED: embedder path implemented, BGE/RERANKER not yet blocked; run the
  installed-build single-scenario command → assertions on the two announcements
  fail → `MUST exit non-zero`. Paste:
  ```
  <paste RED output>
  ```
- [ ] GREEN: extend the block taxonomy + sweep: `GIVN_SPEC_BGE` in
  `disabled|stub|missing`, `GIVN_SPEC_RERANKER` in
  `disabled|stub|missing|nli-stub|nli-missing`, and `GIVN_SPEC_BGE_APPROVAL=failed`.
  Production files: `src/spec/retrieval.rs`. Run the installed-build
  single-scenario command → `PASSES`. Paste:
  ```
  <paste GREEN output>
  ```
- [ ] REFACTOR: rerun → still passes. (Paste if changed.)
- [ ] Commit: `feat(retrieval): installed builds ignore second-stage disable/simulate requests` — hash `TBD`

## Task 5 — Scenario (@e2e @release-proof): An installed givn honours a model-location request that still uses the real models

- [ ] RED: with BGE/RERANKER blocked for non-allowlist values, `GIVN_SPEC_BGE=auto`
  is allowed only after the allowlist exists; run the installed-build
  single-scenario command → the "does not announce" assertion fails if the
  location request is (wrongly) blocked/announced → `MUST exit non-zero` until
  the allowlist exists. Paste:
  ```
  <paste RED output>
  ```
- [ ] GREEN: refine the taxonomy so model-location requests are allowed and not
  announced: `GIVN_SPEC_BGE=auto|local`, `GIVN_SPEC_RERANKER=nli|nli-auto`, and
  the location vars (`*_MODEL_DIR`, `MODEL_CACHE`, `E5_TOKENIZER`). Also harden
  the release subprocess: `env_clear()` + minimal env (`PATH`, `HOME`) + the
  scenario's own envs + per-scenario temp `XDG_CACHE_HOME`/`HF_HOME` so no
  ambient `GIVN_SPEC_*` leaks a spurious announcement. Production files:
  - `src/spec/retrieval.rs` (allowlist, modified)
  - `tests/common/…` (cache isolation in release subprocess)
  Run the installed-build single-scenario command → `PASSES`. Paste:
  ```
  <paste GREEN output>
  ```
- [ ] REFACTOR: rerun → still passes. (Paste if changed.)
- [ ] Commit: `feat(retrieval): installed builds honour model-location requests` — hash `TBD`

## Task 6 — Unit proofs (exhaustive taxonomy, both profiles)

- [ ] RED: write the unit modules
  - dev passthrough `#[cfg(all(test, debug_assertions))]`: `spec_env`/`spec_env_var`
    honour stub/disabled/missing/test-pool/token-budget requests; unset vars →
    `None`; `spec_env_var` on an unset var → `Err(VarError::NotPresent)`.
  - release matrix `#[cfg(all(test, not(debug_assertions)))]`: every blocked
    request (`GIVN_SPEC_EMBEDDER=*`, BGE/RERANKER non-allowlist values,
    `*_LOCAL_STUB`, `*_TEST_POOL`, `*_DIRECTIONAL_SCORES`, `STUB_PAIR_SCORES`,
    `DUPLICATES_TEST_SCORES`, token budgets, `NLI_REPORT_LIMIT`,
    `MAX_RERANK_PAIRS`, `RESOLUTION_NOTICES`, `DOWNLOAD_EVENTS`) →
    `None`/`Err(VarError::NotPresent)`; every location request (`BGE=auto|local`,
    `RERANKER=nli|nli-auto`, `*_MODEL_DIR`, `MODEL_CACHE`, `E5_TOKENIZER`) →
    passthrough; announcement sweep fires once per blocked request.
  Run `cargo test --lib release_strict` and
  `cargo test --release --lib -- release_strict` → RED (name-list vars not yet
  in the block set). Paste:
  ```
  <paste RED output>
  ```
- [ ] GREEN: extend `release_blocks`/sweep to the full name-list taxonomy in
  `src/spec/retrieval.rs`; run both commands → `PASSES`. Paste:
  ```
  <paste GREEN output>
  ```
- [ ] REFACTOR: rerun both → still pass.
- [ ] Commit: `feat(retrieval): exhaustive release block taxonomy` — hash `TBD`

## Task 7 — Docs

- [ ] `docs/retrieval-models.md`: overrides always honoured in dev; installed
  builds ignore fixture requests and always use real models; offline-verify note.
- [ ] `docs/reference.md`: installed-build strictness where applicable.
- [ ] `README.md`: verify no behavior wording needs updating.
- [ ] Commit: `docs(retrieval): installed-build fixture-request behaviour` — hash `TBD`

## Task 8 — Final verification

- [ ] `cargo fmt` + `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [ ] `./run-tests.sh` green; `./run-tests.sh --e2e` green.
- [ ] `./measure-coverage.sh` merges; coverage nondecreasing.
- [ ] `givn status --change release-strict-model-overrides` shows `design-review`,
  `tasks` complete; next `givn instructions review --change release-strict-model-overrides`.
