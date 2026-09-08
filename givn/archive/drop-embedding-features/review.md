# Review: drop-embedding-features

## Fabrication Audit (do this first)

**@e2e tag integrity** — the change's one e2e scenario ("A consumer invoking
a retired engine command is told it does not exist") carries `@e2e` in the
delta file; tasks.md records it and the verify-e2e gate ran it (1 scenario,
5 steps, GREEN). No tag removal.

**Step definition scan** — all new/changed step bodies contain real
assertions or real setup: `no_optional_model_features_declared` parses
`cargo metadata` and asserts the optional-feature set is empty;
`synchronized_specs_guidance` loads three asset files; the three
guidance-assertion steps each assert concrete content; the retired-command
steps spawn the real binary and assert exit code, stderr wording, and
error-output wording. The only trivially-shaped body is
`world_envs` → `Vec::new()`, a shared helper (documented), not a step.

**Commit cross-check** —

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Cargo installation enables every declared feature by default | `8c78642` | yes — Cargo.toml, src/spec, src/cli, src/main.rs, run-tests.sh, measure-coverage.sh, tests |
| Retrieval guidance distinguishes build capabilities | `3b4bf6b` | yes — assets/skills/givn-spec/SKILL.md.tmpl, assets/commands/givn-spec.md, tests/steps/instruction_integrity_steps.rs |
| A consumer invoking a retired engine command is told it does not exist | `42e90da` | yes — tests/steps/release_0_3_0_steps.rs, givn/specs/maintain-project/usecase.md |

**Promised-architecture cross-check** — design.md promises no new
components; the removal inventory is verifiably gone: `src/spec/retrieval.rs`
and `src/spec/remediation_log.rs` deleted, `SpecCommands::{Index, Search,
Explore, Review, BgeParity}` gone, `run_retrieval_gate`/
`run_similarity_gate`/`refresh_similarity_index` gone, `[features]` reduced
to an explicit empty `default`, optional model dependencies removed from
Cargo.toml and Cargo.lock.

**Strict-mode proof** — recorded in tasks.md: each scenario's RED run exits
101 with undefined/failed steps (`fail_on_skipped()` in
tests/features_runner.rs:20).

**Downgraded e2e scenario scan** — the single e2e scenario asserts on the
real CLI's exit code, stderr, and output wording; no repository-only
assertions.

**E2e isolation check** — delta scope: full run 3 scenarios, e2e run 1
scenario (strictly fewer). `verify.e2e_command` = `./run-tests.sh --e2e`
(`--tags '@e2e'`), not a copy of `verify.command`.

**Design.md conformance check** —

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Engine + features + deps deleted | delete retrieval.rs, remediation_log.rs, features, optional deps | done | yes |
| Engine-backed commands removed | Index/Search/Explore/Review/BgeParity | done | yes |
| Gates removed | check retrieval gate, archive similarity gates | done | yes |
| Scripts collapsed | single pass, two coverage reports | done | yes |
| Test plumbing removed | retired steps, release helpers, world fields, env residue | done | yes |
| Guidance de-scoped | assets free of engine wording | done | yes |
| Fragment routing in the gate | `NEW in fragments` validated against fragment.md, no interactions | done (src/spec/routing_table.rs) | yes — discovered during review, recorded in design.md |

## Coverage classification

| Scenario (delta) | Status | Resolution |
|---|---|---|
| Cargo installation enables every declared feature by default | GREEN | pinned by the modified permanent scenario |
| Retrieval guidance distinguishes build capabilities | GREEN | pinned by the modified permanent scenario |
| A consumer invoking a retired engine command is told it does not exist | GREEN (@e2e) | new e2e evidence |
| release-strict-model-overrides (tombstone) | resolved | retired below |

Known deliberate gaps (accepted, out of scope): `GIVN_SPEC_*` env inertness
is not spec'd (the knobs no longer exist); two permanent scenario *titles*
retain historical engine wording (`spec-progress-output` feature title,
`skill-rewrites` scenario title) — renaming them would churn stable
identifiers without observable behaviour change.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|

## Retirements

RETIRE: release-strict-model-overrides — specifies installed-build strictness
of the semantic engine that `retire-semantic-capabilities` retired and this
change deletes; the strictness gate it specified executed zero checks.

## Review disposition

REVIEW: PASS
README-IMPACT: none

Full delta scope: 3 scenarios / 14 steps, exit 0. e2e scope: 1 scenario /
5 steps, exit 0. `cargo check --workspace --all-targets` and
`cargo clippy --workspace --all-targets -- -D warnings` clean.
