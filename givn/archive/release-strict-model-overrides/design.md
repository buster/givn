# Design: release-strict-model-overrides

## Overview

The semantic gates (E5 similarity, BGE reranker evidence, NLI entailment
evidence) are the project's quality guardrails. The development fixture suite
drives them fast and offline through environment-driven **fixture requests**
(requests to substitute simulated model output or to disable an evidence
stage). Today every givn binary — including the one a user installs via
`cargo install` — honours those requests, so an installed givn can silently
run the gates against fake evidence.

This change makes the **installed build** strict: it ignores every fixture
request, announces that the request is ignored, and always runs the real
models. The **development build** keeps honouring fixture requests so the
fixture suite is unchanged. **Model-location requests** (real models from a
chosen local/offline/custom-cache location) are honoured by both builds and
are never treated as fixture requests.

The two build kinds are already distinguished by Cargo profiles: `cargo test`
and `cargo build` use the `dev` profile (`debug_assertions` on), while
`cargo install` and `cargo build --release` use the `release` profile
(`debug_assertions` off). That profile switch is the enforcement surface —
no new crate, no new flag, no new configuration.

**Universal invariant:** a release build reads none of the fixture-request
environment. The only givn model-channel channels that keep working in
release are the model-location requests.

## Technology decisions

- **Compile-time profile switch, no new dependency.** `cfg!(debug_assertions)`
  is a compile-time constant: `true` in dev/test builds, `false` in release
  builds. Baked into `const ALLOW_SPEC_OVERRIDES: bool = cfg!(debug_assertions)`,
  the optimizer folds both branches and a release binary genuinely has no
  code path that honours a fixture request. Because it is not a Cargo feature,
  it cannot be turned back on accidentally in a release build. The repo uses
  the locally available stable Rust toolchain (there is no
  `rust-toolchain.toml` pin in this checkout); cucumber-rs is the existing
  runner and is not changed.
- **One gated accessor pair replaces direct fixture-request reads.** In
  `src/spec/retrieval.rs` there are 44 `std::env::var[_os](...)` calls: 40 on
  `GIVN_SPEC_*` channels and 4 on non-`GIVN_SPEC_*` variables (`HF_HOME`,
  `XDG_CACHE_HOME`, `HOME`, `HF_ENDPOINT`), the last four being read by givn
  itself (cache roots and the download endpoint). Two wrappers with the same
  call signatures as the calls they replace are introduced and every
  `GIVN_SPEC_*` site is routed through them:
  - `spec_env(name) -> Option<OsString>` (replaces `std::env::var_os`)
  - `spec_env_var(name) -> Result<String, VarError>` (replaces
    `std::env::var`; preserves the existing `NotUnicode`/`NotPresent` error
    contract and messages)
  A blocked fixture request resolves to `Err(VarError::NotPresent)` /
  `None` — never an empty string — so the existing
  `match … .as_deref() { Ok("stub") … Err(VarError::NotPresent) => real-model }`
  arms fall through to the real-model branch. Two of the 40 sites use the
  const-aliased names `THREADS_ENV` (`resolve_thread_budget`) and
  `MAX_RERANK_PAIRS_ENV` (`max_rerank_pairs`), and one reads a runtime key
  (the token-budget helper at `retrieval.rs` ~2329); all three are routed
  through the wrappers too. The four non-`GIVN_SPEC_*` reads are **not** routed
  through the wrappers — they are the air-gap/mirror cache and endpoint
  channels, honoured identically in both build kinds by staying raw. In the
  dev build the wrappers pass every `GIVN_SPEC_*` value through unchanged
  (byte-identical behavior). In the release build a blocked request resolves
  to "unset".
- **Value-aware blocking for the three stage selectors.** `GIVN_SPEC_EMBEDDER`
  is blocked for any value (every value swaps the real embedder). For
  `GIVN_SPEC_BGE` only `stub`/`disabled`/`missing` and for
  `GIVN_SPEC_RERANKER` only `stub`/`disabled`/`missing`/`nli-stub`/`nli-missing`
  are blocked; the real-model selectors `auto`/`local` (BGE) and `nli`/`nli-auto`
  (NLI) stay honoured. `GIVN_SPEC_BGE_APPROVAL=failed` is blocked. All other
  `GIVN_SPEC_*` request channels are blocked by name.
- **One eager report of every ignored request.** When a release build first
  consults any spec channel (the first `spec_env`/`spec_env_var` call, so
  only a semantic command triggers it), a one-shot sweep scans the process
  environment for present-and-blocked request variables and prints one line
  per variable to stderr, deduplicated per process. This makes the refusal
  observable even for stage selectors a given command does not itself consult
  (e.g. `givn spec index` announces a blocked BGE request even though index
  only consults the embedder channel).
- **Testing proof via a real release binary plus seam-level unit tests.**
  See "E2E infrastructure" and "Unit proofs" below.

## Environment request taxonomy

| Channel (name, code constant) | Dev build | Installed build | Reason |
|---|---|---|---|
| `GIVN_SPEC_EMBEDDER` (any value) | honoured | **ignored** | substitutes simulated embedder output |
| `GIVN_SPEC_BGE` = `stub` \| `disabled` \| `missing` | honoured | **ignored** | swaps in fake output or disables the BGE stage |
| `GIVN_SPEC_BGE` = `auto` \| `local` | honoured | honoured | real model, chosen location |
| `GIVN_SPEC_RERANKER` = `stub` \| `disabled` \| `missing` \| `nli-stub` \| `nli-missing` | honoured | **ignored** | swaps in fake output or disables the NLI stage |
| `GIVN_SPEC_RERANKER` = `nli` \| `nli-auto` | honoured | honoured | real model, chosen location |
| `GIVN_SPEC_BGE_APPROVAL` = `failed` | honoured | **ignored** | forces the BGE stage unavailable |
| `GIVN_SPEC_*_LOCAL_STUB`, `*_TEST_POOL`, `*_DIRECTIONAL_SCORES`, `STUB_PAIR_SCORES`, `DUPLICATES_TEST_SCORES`, `*_TOKEN_BUDGET`, `NLI_REPORT_LIMIT`, `MAX_RERANK_PAIRS`, `RESOLUTION_NOTICES`, `DOWNLOAD_EVENTS` | honoured | **ignored** | test-pool sizing, canned scores, token-cap fixtures, report chatter |
| `GIVN_SPEC_MODEL_CACHE`, `GIVN_SPEC_E5_TOKENIZER`, `GIVN_SPEC_BGE_CACHE`, `GIVN_SPEC_BGE_MODEL_DIR`, `GIVN_SPEC_NLI_CACHE`, `GIVN_SPEC_NLI_MODEL_DIR` | honoured | honoured | model-location requests (real artifacts) |
| `GIVN_SPEC_THREADS` | honoured | honoured | pure compute budget, no model substitution |
| `GIVN_FEATURES` (external) | honoured | honoured | suite feature scope, not a model channel |
| `GIVN_COVERAGE_*` (internal, gate plumbing) | honoured | honoured | givn-set values for the coverage gate |

`HF_HOME`, `HF_ENDPOINT`, `XDG_CACHE_HOME`, and `HOME` are cache-root and
download-endpoint channels that givn reads directly (cache resolution and the
Hub endpoint) and that `hf-hub`/`fastembed` also consume. None of them is a
fixture request; all four stay honoured identically in both build kinds. That
is exactly why air-gapped/mirror deployments (which use `HF_ENDPOINT` plus the
model-location channels) keep working in an installed build.

## Architecture impact

`src/spec/retrieval.rs` is the only production file that reads `GIVN_SPEC_*`
(verified by search; 44 `std::env::var[_os]` calls in total, 40 on
`GIVN_SPEC_*` channels). Changes:

1. `const ALLOW_SPEC_OVERRIDES: bool = cfg!(debug_assertions);`
2. `spec_env(name) -> Option<OsString>` and
   `spec_env_var(name) -> Result<String, VarError>` wrappers (dev =
   transparent passthrough; release = block + announce). All 40 `GIVN_SPEC_*`
   sites are routed through them — including the const-aliased reads
   (`THREADS_ENV`, `MAX_RERANK_PAIRS_ENV`) and the runtime-key budget read;
   the four non-`GIVN_SPEC_*` reads (`HF_HOME`, `XDG_CACHE_HOME`, `HOME`,
   `HF_ENDPOINT`) stay raw.
3. `release_blocks(name, raw) -> bool` classification (the taxonomy table).
4. `report_ignored_release_overrides()` one-shot stderr sweep using the
   existing `notice()` helper (`eprintln!` at `retrieval.rs:1093`), guarded by
   a `OnceLock` so each variable is announced once per process.
5. A `#[cfg(all(test, not(debug_assertions)))]` unit-test module exercising
   `release_blocks`/blocking for the release reality and a
   `#[cfg(all(test, debug_assertions))]` module proving dev passthrough; the
   release half runs only under `cargo test --release --lib` (see Unit proofs).

No CLI, no config, no manifest, no data-model changes. The dev behavior is
unchanged, so `run-tests.sh`, `measure-coverage.sh`, and all existing step
definitions keep their current env values verbatim.

## Unit proofs (which case each covers that E2E does not)

- The **block/allow classification** is a pure function over
  `(name, value, allow)`; unit tests enumerate every var × value from the
  taxonomy in both modes. The E2E scenarios cannot enumerate the full matrix
  cheaply (each requires a release build), so this is the exhaustive
  matrix proof; E2E proves the wiring and the real announcement end-to-end.
  The release-mode half of these tests is compiled under
  `#[cfg(all(test, not(debug_assertions)))]`, so it executes only under the
  release profile. `run-tests.sh --e2e` therefore runs an explicit guarded
  `cargo test --release --lib -- <release_strict filter>` so the installed
  reality is proven by a configured runner, not by an orphaned module.

## Step definition locations (one file per capability)

- `tests/steps/release_strict_overrides_steps.rs` — shared project
  preparation and the development-build scenario (runs in the normal suite,
  `debug` harness):
  - `Given a project prepared for semantic indexing`
  - `Given a request for simulated embedder output` (sets `GIVN_SPEC_EMBEDDER=stub`)
  - `And a request for simulated embedder output` (same)
  - `When a consumer runs \`givn spec index\` with a development givn build`
  - `Then the index is built`
  - `Then no installed-build announcement is emitted`
- `tests/steps/release_strict_overrides_e2e_steps.rs` — the installed-build
  steps (used by the `@release-proof` scenarios), including the release
  binary machinery:
  - `Given a request to disable the reranker evidence stage`
    (`GIVN_SPEC_BGE=disabled`)
  - `Given a request for simulated entailment evidence`
    (`GIVN_SPEC_RERANKER=nli-stub`)
  - `Given a request for the real models from a chosen cache location`
    (`GIVN_SPEC_BGE=auto` + `GIVN_SPEC_MODEL_CACHE=<temp path>`)
  - `When a consumer runs \`givn spec index\` with an installed givn build`
  - `Then the installed givn announces that the simulated-embedder request is ignored`
  - `Then the installed givn refuses to proceed without the real embedder model`
  - `Then the installed givn announces that the disable-reranker request is ignored`
  - `Then the installed givn announces that the simulated-entailment request is ignored`
  - `Then the installed givn refuses to proceed without the real model evidence`
  - `Then the installed givn does not announce that any model request is ignored`

`tests/steps/mod.rs` registers both modules (alphabetical order, matching the
existing list).

## Runner and strict mode

- `verify.command` = `./run-tests.sh` (from `givn/commands.yaml`), which
  executes the `.feature` files through `cucumber-rs` via `cargo test --test
  features_runner`. Step bodies bind the `.feature` contract; there is no
  parallel hand-maintained test file.
- **Strict mode (already proven):** `tests/features_runner.rs:20` calls
  `.fail_on_skipped()` on the `Cucumber` builder, so an undefined/pending
  step is a hard failure. New step bodies must never be left empty.
- **Single-scenario run command** (used by every RED/GREEN check):
  `cargo test --test features_runner -- --name "<Scenario title>"`
  (cucumber-rs `--name` filter), optionally prefixed by
  `GIVN_FEATURES=givn/specs`. For a `@release-proof` scenario the release
  binary is built lazily by the step (see E2E infrastructure).

## E2E infrastructure

- **E2E runner command:** `verify.e2e_command` = `./run-tests.sh --e2e`
  (from `givn/commands.yaml`), filtered to `@e2e` groups.
- **Release-proof group and tag.** The installed-build scenarios are tagged
  `@release-proof` (scenarios 1 and 3 also carry `@e2e`). `run-tests.sh`:
  - every existing group filter in all three modes (default `""`,
    `--non-e2e`, and `--e2e`) gains `and not @release-proof`, so the
    `@release-proof` scenarios run in exactly one dedicated group and never
    also match the capability groups (scenario 1/3's `@e2e` tag would
    otherwise double-run them in the `run_e5` group and leak that group's
    stub envs into the release subprocess);
  - the `--e2e` branch gains the dedicated group:
    `cargo test --features spec-retrieval,spec-similarity-gate,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@release-proof'`
    guarded by `tag_present '@release-proof'`, run with a clean environment
    and `GIVN_FEATURES` only (no stub wrapper functions — the scenarios set
    their own request envs inside the steps);
  - the `--e2e` branch also runs the release-profile unit proof:
    `cargo test --release --lib -- release_strict` under the same guard.
  `measure-coverage.sh` mirrors the group partition exactly: `not
  @release-proof` on every non-e2e filter (so the fast non-e2e report never
  builds release) and on every existing e2e filter, plus the dedicated
  `@release-proof` group in the e2e block, so the e2e coverage report
  exercises the same scenarios as `run-tests.sh --e2e`.
- **Release binary source.** `tests/common` gains a release-binary resolver
  used by the e2e steps:
  - If `GIVN_RELEASE_BIN` is set, use that path (operators/CI can point at a
    pre-built binary to skip the build).
  - Otherwise build once into `target/release/givn` with `cargo build
    --release --features spec-retrieval,spec-similarity-gate,cross-encoder-reranker,nli-evidence
    --bin givn` from `CARGO_MANIFEST_DIR`, cached in a `OnceLock`. The repo's
    dogfood loop already performs a full release (`cargo install --path .`),
    so this does not introduce a new kind of build, only a second occurrence
    of it inside the e2e suite.
  - The release subprocess is spawned with `env_clear()` plus a minimal set
    (`PATH`, `HOME`, `CARGO_*` as needed for the lazy build) and then exactly
    the scenario's own request/location envs and `HF_ENDPOINT`/`XDG_CACHE_HOME`
    — so no ambient shell `GIVN_SPEC_*` can leak a spurious announcement.
  - `XDG_CACHE_HOME`/`HF_HOME` are pointed at a per-scenario temp dir so the
    release binary never writes model artifacts into the developer's real
    `~/.cache` (mirrors the existing `semantic_model_download` pattern).
- **Deterministic real-model refusal (offline).** The `@release-proof`
  scenarios must prove "the real model is required" without downloading a
  real model. The step runs `givn spec index` against a fixture project that
  has at least one permanent scenario (so embedding is actually attempted)
  with `HF_ENDPOINT` pointed at the existing `ArtifactFixtureServer`
  (`tests/common/mod.rs:42`); the release build therefore attempts real E5
  acquisition, which cannot produce a valid snapshot. `fastembed_model`
  (`retrieval.rs` ~8882) fails on the invalid ONNX with the stable diagnostic
  `retrieval model is unavailable; check the local E5 model cache, network,
  and GIVN_SPEC_MODEL_CACHE` (~8891) — that exact wording is pinned in the
  step during GREEN. The step asserts a non-zero exit, the relevant
  announcement, and the absence of any `Indexed …` success line. No live
  third-party service is involved.
- **E2E strict-mode proof:** the same `fail_on_skipped()` builder runs the
  `@release-proof` group; an undefined e2e step aborts with a non-zero exit.
- **No new running process to instrument:** the spawned release binary is a
  black-box subprocess; coverage instruments the `features_runner` harness
  (debug), not the release subprocess. The release-only `else` branches in
  `spec_env`/`release_blocks` are executed only by the release binary, so they
  show as uncovered lines in the debug-instrumented coverage reports — a
  noted, intentional outcome that does not move the overall percentage
  materially.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run a semantic command with an installed givn while a fixture request asks for simulated or disabled model output | An installed givn refuses a request for simulated embedder output | CLI (release givn binary) | Spawn the release binary (`GIVN_RELEASE_BIN` or lazy `cargo build --release`), run `givn spec index` with `GIVN_SPEC_EMBEDDER=stub` and `HF_ENDPOINT` at the fixture server; assert the announcement and the real-model requirement on stderr |
| Run a semantic command with an installed givn while a fixture request keeps the real models from a chosen location | An installed givn honours a model-location request that still uses the real models | CLI (release givn binary) | Same release-binary driver with `GIVN_SPEC_BGE=auto` and `GIVN_SPEC_MODEL_CACHE=<temp dir>`; assert no announcement and no blocked-request behavior |

The second-stage consolidated scenario (BGE disable + NLI simulation) and the
development-build scenario are regular scenarios covering distinct invariants:
the multi-stage refusal announcements and the dev-build contrast. They reuse
the same CLI driving mechanism; per the canonical policy they are variants
determined by which request is set, not new inventory actions. The
"still requires the real reranker/entailment evidence" staging is proven by the
unit matrix, not by these scenarios — no offline command reaches the BGE/NLI
selectors (they all require a real E5 index first), so the release-proof
scenarios assert the deterministic announcement and the E5-level refusal.

## Coverage process boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `givn` (dev binary) | `cargo llvm-cov` in `measure-coverage.sh` | `givn` lib/bin via `--instrument-coverage` | profraw accumulated per group, Cobertura exported | `merge-coverages.sh` | `spec_env`/`spec_env_var` passthrough in `givn spec index` (dev branch) |
| release givn subprocess | `release_strict_overrides_e2e_steps.rs` | none (black-box) | n/a | n/a | n/a (asserted on subprocess stderr/exit) |

## Local runnability and digital twins

- **Local run command:** `./run-tests.sh` (development fixtures) and
  `./run-tests.sh --e2e` (including the `@release-proof` group). No service
  stack exists — givn is a single-binary CLI.
- **Digital twins:** the only external dependency is Hugging Face model
  acquisition (`hf-hub`/`fastembed`). Its digital twin is the existing
  `ArtifactFixtureServer` (`tests/common/mod.rs`) reached via `HF_ENDPOINT`;
  no scenario depends on the live Hub. Model-location requests already point
  at local directories. The real-model-refusal assertions (`retrieval model
  is unavailable`) intentionally need no digital twin because they prove the
  real acquisition path fails cleanly.
- **Anticipated interface obstacle:** building a release binary inside a
  cucumber step is slow and may be impossible in sandboxed CI that forbids
  nested cargo builds. Named fix: `GIVN_RELEASE_BIN` lets the runner use a
  pre-built binary; `run-tests.sh --e2e` documents it, and AGENTS.md's
  existing `cargo install --path .` produces exactly such a binary.

## Docs updates

- `docs/retrieval-models.md` — Overrides section: state that fixture requests
  are honoured only by development builds; an installed build ignores them,
  announces the refusal, and always runs the real models; model-location
  requests continue to work in installed builds. Offline section: note the
  same split.
- `docs/reference.md` — install/environment section: `cargo install` yields a
  strict binary; fixture requests require a development build
  (`cargo build` / `cargo test`).
- `README.md` — Install section: one sentence pointing at the strict-install /
  dev-fixtures split.
- `docs/arc42/12-glossary.md` — new terms (already recorded):
  Installed build, Development build, Fixture request, Model-location request.
