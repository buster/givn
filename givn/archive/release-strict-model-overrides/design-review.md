# Design Review: release-strict-model-overrides

## Grilling (fresh-context subagent)

The plan was grilled by an independent fresh-context subagent that read the
proposal, spec, design, arc42 assessment, the normative design-review/specs/
arc42-docs instructions, and the relevant code (`src/spec/retrieval.rs`,
`run-tests.sh`, `measure-coverage.sh`, `tests/common/mod.rs`,
`tests/features_runner.rs`, `givn/commands.yaml`, glossary, all 12 arc42
chapter files, the ADR register).

## Findings and resolutions

| Severity | Finding | Resolution |
|---|---|---|
| RED-1 | Scenarios asserting "still requires the real reranker/entailment evidence" are unobservable: `givn spec index` consults only E5, and no offline command reaches the BGE/NLI selectors. | Reframed. The consolidated second-stage scenario asserts the two announcements plus the deterministic E5-level refusal; staged BGE/NLI reality is proven by the exhaustive unit matrix (see design "Unit proofs"). Spec updated. |
| RED-2 | `@release-proof` scenarios also carry `@e2e` (scenarios 1/3) and would double-run in the `--e2e` capability groups, re-triggering a release build and leaking that group's stub envs. | Every existing group filter in all three `run-tests.sh` modes now excludes `@release-proof`; the dedicated group runs with a clean env. `measure-coverage.sh` mirrors the exclusion and the group. design.md and spec updated. |
| AMBER-3 | The release-mode unit-proof module (`#[cfg(all(test, not(debug_assertions)))]`) would never run under the configured runners. | `run-tests.sh --e2e` now runs the guarded `cargo test --release --lib -- release_strict` so the exhaustive matrix is proven by a configured runner. |
| AMBER-4 | Env-read inventory was wrong (~30 vs actually 44 reads) and falsely claimed HF_HOME/XDG_CACHE_HOME/HF_ENDPOINT are read only by external crates. | Corrected: 44 reads, 40 on `GIVN_SPEC_*`, 4 non-`GIVN_SPEC_*` (`HF_HOME`, `XDG_CACHE_HOME`, `HOME`, `HF_ENDPOINT`) read by givn and left raw/honoured. design.md corrected. |
| AMBER-5 | Scenario 4 ("honours a model-location request") lacked the `@e2e` tag, violating one-E2E-per-inventory-entry and the design matrix. | Scenario 4 tagged `@e2e`; the RED-2 exclusion now makes this safe. |
| GREEN-6 | Release subprocess could write garbage artifacts into `~/.cache`. | Release subprocess is spawned with `env_clear()` + minimal env + per-scenario temp `XDG_CACHE_HOME`/`HF_HOME`. |
| GREEN-7 | `rust-toolchain.toml` does not exist; the pinned failure wording must be ~8891, not the dev-only selector branch. | design.md now states the locally available stable toolchain and pins the `retrieval model is unavailable; check the local E5 model cache, network, and GIVN_SPEC_MODEL_CACHE` wording. |
| GREEN-8 | "Unset" sentinel semantics must be `Err(VarError::NotPresent)`/`None`, never an empty string, for the `.as_deref() == Ok("stub")` arms to fall through. | Documented in design.md Technology decisions. |

## Branch verdicts

- **Scope** — spec matches the proposal exactly; "done" is unambiguous after the
  RED-1 reframe.
- **Tech choices** — `cfg!(debug_assertions)` + gated accessor pair + one-shot
  sweep, no new dependency; correct fit.
- **Missing scenarios** — value-aware selector blocking delegated to the unit
  matrix; no proposal behaviour left without a test surface.
- **Testability** — every `@release-proof` scenario can fail RED with zero
  production code (the pre-change release binary honours the stub and succeeds);
  scenarios 4 and 5 are consciously-vacuous regression guards (negative/absence
  assertions) and are documented as such.
- **E2E fidelity / interaction coverage** — inventory (2 entries) maps 1:1 to
  the design matrix (scenarios 1 and 4 as `@e2e`); regular scenarios are
  legitimate variants; driving mechanism is a real CLI subprocess.
- **Risk** — most likely failure is the offline real-model-refusal wording;
  mitigated by pinned wording (verified in GREEN) + `GIVN_RELEASE_BIN` + cache
  isolation.
- **ADR qualification** — the "installed builds ignore fixture requests"
  decision fails `durable_consequence` (reversal is a one-line `cfg!` flip, no
  migration/surface consequence) and `lower_level_artifact` (the rationale is
  fully owned by the spec + design.md + arc42 02/07/08 + glossary). Route:
  `CANONICAL_ARTIFACT`; no new ADR and no amendment (the change strengthens
  ADR-028's non-deactivatable gate without conflicting). arc42.md's route is
  consistent.
- **arc42 chapters** — independent re-derivation of all 12 rows matches
  `arc42.md` (no Yes/No or omission diff); every Yes-row chapter contains real
  matching content; all 12 chapter files exist, non-empty, and contain no ASCII
  diagrams.
- **Ubiquitous language** — `Installed build`, `Development build`,
  `Fixture request`, `Model-location request` recorded in
  `docs/arc42/12-glossary.md` and used consistently in design and arc42
  chapters.

## Hardening applied

`specs/` reframed (consolidated second-stage scenario, scenario 4 `@e2e`),
`design.md` corrected (env inventory, run-tests/measure-coverage group
partition, release-profile unit runner, cache isolation, pinned wording,
sentinel semantics, removal of the false toolchain-pin claim). `givn lint
--change` clean after edits.

DESIGN-REVIEW: PASS
