# Review — release-strict-model-overrides

An installed (release) givn build ignores every fixture request — simulated
embedder/BGE/NLI output, disabled evidence stages, test pools, canned scores,
token-budget caps, report chatter — and always evaluates the semantic gates
with the real models. Development builds stay unchanged. Model-location
requests keep working in both build kinds.

## Fabrication audit

### Files scanned

- Delta features: `givn/changes/release-strict-model-overrides/specs/
  release-strict-model-overrides/release-strict-model-overrides.feature`
  (4 scenarios).
- Step definitions: `tests/steps/release_strict_overrides_steps.rs`,
  `tests/steps/release_strict_overrides_e2e_steps.rs`, registration in
  `tests/steps/mod.rs`, release-binary resolver in `tests/common/mod.rs`.
- Runner: `tests/features_runner.rs` (the only cucumber runner).
- Production delta: `src/spec/retrieval.rs` (gate accessors, release-only
  taxonomy, one-shot announcement sweep, dual-profile unit module).
- Scripts: `run-tests.sh`, `measure-coverage.sh`.
- Docs: `docs/retrieval-models.md`, durable arc42 chapters 02/05/07/08/10/11/12.

### Findings and dispositions

| # | Check | Result |
|---|---|---|
| 1 | `@e2e` tag integrity (none silently removed) | Clean. `@givn.added @e2e` retained on scenarios 1 and 3; scenario 2 is deliberately `@release-proof`-only (documented in design.md §E2E infra: one value-aware taxonomy change, matrix proof left to the release unit matrix); scenario 4 is the dev-control regular scenario. |
| 2 | Step bodies non-empty (no `{}`/`pass`/no-assert stubs) | Clean. Every step asserts on real output (`command_output`, exit code, stderr/stdout text, `release_model_cache` existence). No assertion-less bodies. |
| 3 | Every checked task has a verified commit | Clean (with documented shared commits). Commits and what they touch: `77f7256` (steps + release resolver + dev-control scenario 4 — test-only, no src needed: scenario 4 characterizes pre-existing dev behaviour), `142fcfd` (runner partition), `d6a4996` (src/spec/retrieval.rs), `7cd305f` (src/spec/retrieval.rs, scenarios 2+3 one atomic value-aware change), `4164b02` (src/spec/retrieval.rs), `86213cb` (src/spec/retrieval.rs import-scope repair), `7cd379a` (docs). Scenarios 2+3 and Task 0/scenario 4 share one atomic commit each — recorded once in `- [x] COMMIT:` lines (integrity hook passes; no duplicate-hash claim). |
| 4 | Promised components exist | Clean. Both step files, release resolver (`release_givn_binary`/`run_release_givn`), `release_blocked`/`spec_env`/`spec_env_var`/`report_ignored_release_overrides`, `REQUEST_CHANNELS`, the `release_strict_tests` module, `@release-proof` partition in both scripts, `docs/retrieval-models.md` section. |
| 5 | Strict-mode proof (non-zero exit) | Present. Unimplemented step → `features_runner` exits 101 (recorded in tasks.md Task 0, EXIT=101); `fail_on_skipped()` in `tests/features_runner.rs:20`. |
| 6 | `@e2e` scenarios assert a real interface (not repo/db-only) | Clean. Scenarios 1 and 3 spawn the installed givn binary (`env_clear()` + pinned envs) and assert on its stderr/exit status — the named real interface is the release CLI. |
| 7 | No weaker parallel e2e implementation | Clean. `tests/features_runner.rs` is the sole cucumber runner; the `*_e2e_steps.rs` files are step definitions under it, not separate drivers. |
| 8 | Canonical E2E policy + inventory normalization | Clean. `givn instructions specs` inventory = 2 entries; each has exactly one `@e2e` scenario (scenarios 1 and 3). Scenario 2 (second-stage ignore) is a non-inventory wiring scenario per design; scenario 4 (dev control) asserts an invariant distinct from the e2e scenarios. No excess `@e2e`. |
| 9 | Local run command starts full stack / digital twin | Clean. `./run-tests.sh`/`--e2e` start `ArtifactFixtureServer` inside the `When` step (HF digital twin); release refusals never need a live third-party service. |
| 10 | `verify.command` ≠ `verify.e2e_command`; count isolation | Clean. Commands differ (`./run-tests.sh` vs `./run-tests.sh --e2e`). Permanent scope: e2e = 5 scenarios < non-e2e = 30. Delta scope: the `@release-proof` group (3 scenarios + release unit proofs) runs only under `--e2e`. |
| 11 | Implementation matches design.md | Clean. Release-proof group, clean-env `--tags '@release-proof'` invocation, `--release` unit proof under the same guard, GIVN_RELEASE_BIN/lazy-build resolver (OnceLock), `env_clear()`, per-scenario temp caches, `ArtifactFixtureServer` refusal path, docs section — all as design.md named. |
| 12 | Coverage measurement validity | Valid. `cargo llvm-cov --test features_runner` instruments the givn lib linked into the harness; spawned release binary is intentionally outside instrumentation (design.md §E2E infra, lines 248-254). Fresh merged report: non-e2e 2791/11441 lines, e2e 3328/11437 lines, `src/spec/retrieval.rs` non-zero (≈30%); both source reports passed to `merge-coverages.sh`; output `coverage/cobertura-coverage.xml` freshly regenerated (2026-08-29 11:12). `COVERAGE MEASUREMENT: VALID`. |

### Interaction coverage cross-reference

| Inventory entry (specs comment block) | Matrix row (design.md) | `@e2e` scenario (delta feature) | Driving mechanism (step file) |
|---|---|---|---|
| run a semantic command with an installed givn while a fixture request asks for simulated or disabled model output | An installed givn refuses a request for simulated embedder output | `An installed givn refuses a request for simulated embedder output` | Release binary subprocess (`run_release_givn`), `givn spec index` with `GIVN_SPEC_EMBEDDER=stub` and `HF_ENDPOINT` at `ArtifactFixtureServer`; asserts stderr announcement + non-zero exit |
| run a semantic command with an installed givn while a fixture request keeps the real models from a chosen location | An installed givn honours a model-location request that still uses the real models | `An installed givn honours a model-location request that still uses the real models` | Same release-binary driver with `GIVN_SPEC_BGE=auto` + `GIVN_SPEC_MODEL_CACHE=<temp>`; asserts no announcement + exit 0 |

Every inventory entry maps to a matrix row, an `@e2e` scenario, and the
promised real-CLI driver. No unmapped entry, no unmatched scenario.

## Arc42 implementation conformance

| Chapter | Arc42 claim | Implementation evidence | Match |
|---|---|---|---|
| 02 architecture constraints | Installed builds ignore fixture requests and always require real models; split fixed by Cargo profile, not config | `ALLOW_SPEC_OVERRIDES: bool = cfg!(debug_assertions)` — profile-fixed; all read paths go through `spec_env`/`spec_env_var` | Match |
| 05 building-block view | Every model-behaviour env channel read through one gated accessor; installed build refuses fixture requests and announces | `spec_env`/`spec_env_var` + `report_ignored_release_overrides` sweep at every `*_enabled` entry; `REQUEST_CHANNELS` list | Match |
| 07 deployment view | Installed builds never substitute simulated output; model-location requests honoured; deterministic-stub behaviour dev-only | `release_blocked` value-aware allowlist (`BGE=auto|local`, `RERANKER=nli|nli-auto`) honours location requests; dev passthrough | Match |
| 08 crosscutting concepts | Two channel classes: model-location (honoured, all builds) vs fixture request (installed ignores, announces) | glossary-aligned; announcement text pinned in e2e steps and asserted | Match |
| 10 quality requirements | Release-profile strictness verified by unit matrix + e2e release scenarios | 9 release unit proofs + 3 @release-proof e2e scenarios + parity guard `sweep_list_fully_gated` | Match |
| 11 risks | Release-only else branches uncovered in debug-instrumented coverage — accepted | Confirmed in Coverage gaps table below; release unit matrix + e2e assertions cover behaviour | Match |
| 12 glossary | Installed build / Development build / Fixture request / Model-location request | Terms used identically in specs, design, docs/retrieval-models.md; glossary updated in `e16aef2` | Match |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Durable glossary (`docs/arc42/12-glossary.md`) holds all four terms: Installed
build, Development build, Fixture request, Model-location request. The specs,
design.md, docs/retrieval-models.md, and code comments reuse them consistently;
the announcement copy mirrors the glossary wording ("ignored in installed
builds"). No term the glossary does not hold is introduced; no inconsistent
reuse found.

UBIQUITOUS LANGUAGE CONFORMANCE: CLEAN

## Deterministic gate dispositions

`givn check review` ran the deterministic overlap phases (verify, verify-e2e,
integrity, overlap dispositions). No overlap/shape-match findings were emitted
for the delta; no duplicate titles; no long-scenario split decisions required.
The delta scenarios are distinguished semantically (see semantic-review.md:
dev-vs-installed pair scores contradiction ≈0.98, all pairs NLI_FILTERED /
BGE_FILTERED). Nothing to dispose.

## Coverage gaps (completeness, not percentage)

Scope: lines added by this change in `src/spec/retrieval.rs` (gate accessors
~139-233, sweep wiring, plus the release/development unit module which is test
code, not production).

| Uncovered delta code | Bucket | Resolution |
|---|---|---|
| `release_blocked` release-only match body (142-143, 146, 150-154, 157, 160-176) | 3 — legitimately hard to test | Only compiled under `!debug_assertions`; the debug-instrumented cucumber harness cannot execute it. Exhaustively asserted by `cargo test --release --lib -- release_strict` (9 tests: `gates_every_fixture_request`, `gates_only_disqualifying_selector_values`, `location_channels_are_honoured`, `sweep_list_fully_gated`, ...) and behaviourally by the @release-proof e2e scenarios (release binary anniversary/refusal asserted). The release subprocess is outside llvm-cov instrumentation by design (design.md §E2E infra). |
| `spec_env` block-branch `return None` (190) and `spec_env_var` block-arm `Err(NotPresent)` (203) | 3 — legitimately hard to test | Same release-only profile gate; unit-proofed (`spec_env_maps_blocked_request_to_none`, `spec_env_var_maps_blocked_request_to_not_present`, `spec_env_var_maps_location_request_to_value`) and e2e-asserted. |
| `report_ignored_release_overrides` notice branch (222-231) | 3 — legitimately hard to test | Release-only announcement emission; asserted by @release-proof scenarios 1 and 2 (`Then the installed givn announces ...`) spawning the release binary; not reachable from a debug-instrumented process. Dev-reachable early-return path (220-221) is covered. |
| Pre-existing error branches in `max_rerank_pairs`/`resolve_thread_budget` (malformed env value returns) | Pre-existing, not introduced by this change | Malformed-value rejections predate the change; not a delta finding. |

Explanations use only the three buckets (dead code / missing test /
legitimately hard to test). No invented exceptions. No dead code introduced.

## Sign-off checklist

- Fabrication audit: clean (all findings remediated/none).
- Every checked task has a verified commit; scenario commits touch
  `src/spec/retrieval.rs`; shared atomic commits recorded once (documented above).
- Every promised component exists.
- Strict-mode proof present and passing (EXIT=101).
- `verify.command` and `verify.e2e_command` both exit 0 (permanent 30, e2e 5;
  delta scope: release-proof 3 + release unit 9).
- Coverage measured across all test binaries, including the Gherkin runner;
  valid (2800+ lines in each report, retrieval.rs ~30% non-zero).
- Every coverage gap classified and resolved (bucket 3 with technical
  justification; pre-existing gaps noted as not-delta).
- Dead code deleted: none introduced. Missing tests added: none — release unit
  matrix + e2e scenarios cover the release paths. Hard-to-test gaps justified.
- Redundant unit tests removed: none; the `development`/`release` split is
  cfg-exclusive and non-redundant.
- No `@wip` tags remain. No implementation-layer detail in the spec.
- Canonical E2E policy read from the resolved specs instruction: 2 inventory
  entries, one `@e2e` scenario each (scenarios 1, 3); variants are regular
  scenarios.
- Every E2E scenario has a real-interface primary assertion (release CLI
  stderr/exit) using the reviewed driver; no browser scenario here.
- Local run command (`./run-tests.sh`/`--e2e`) starts the full stack including
  the `ArtifactFixtureServer` digital twin, verified during this review.
- `verify.e2e_command` (`./run-tests.sh --e2e`) identified, read, distinct from
  `verify.command`; its scenario count (5 permanent) proven strictly smaller
  than the non-e2e count (30).
- Implementation (commands, file layout, driver) matches design.md exactly;
  deviations none.
- Interaction coverage verified: 2 inventory entries → 2 matrix rows → 2 `@e2e`
  scenarios → release-binary driver; no mismatches.
- No finding excused outside the three buckets.

Semantic review: `semantic-review.md` Status PASS, candidate count 0; all
delta-internal pairs filtered by NLI/BGE.

REVIEW: PASS
