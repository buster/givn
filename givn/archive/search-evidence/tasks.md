# Tasks: search-evidence

## Setup

- [x] Configure the strict Cucumber runner for the active change: register
  `tests/steps/search_evidence_steps.rs` and
  `tests/steps/search_evidence_e2e_steps.rs` in `tests/steps/mod.rs`,
  scaffold unimplemented step bodies with `unimplemented!()`, and run the
  proof-of-strictness check.
  - Runner command: `./run-tests.sh`
  - E2E command: `./run-tests.sh --e2e` (already configured in
    `givn/commands.yaml`; the e2e scope is strictly smaller via tag filters)
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()` in
    `tests/features_runner.rs` (already active; no config change)
  - Strict proof command:
    `GIVN_FEATURES=givn/changes/search-evidence/specs cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "Search without the evidence flag prints no evidence lines"`
  - Strict proof output: `1 feature`, `1 scenario (1 failed)` — the
    unimplemented `local NLI evidence is enabled` Given panicked
    `not implemented`; the process exited non-zero (`exit status: 101`).
    This proves `.fail_on_skipped()` rejects the unimplemented step.
- [x] COMMIT: register the two step modules + the strict-mode proof. Hash: `9e5dbab`

## Scenario: Search without the evidence flag prints no evidence lines

- [x] RED: remove this scenario's `@wip`; the unimplemented
  `local NLI evidence is enabled` Given fails the strict run.
  Evidence: `1 scenario (1 failed)`, `Step panicked. Captured output: not implemented: local NLI evidence is enabled`.
- [x] GREEN: implement `local NLI evidence is enabled`
  (`world.retrieval_reranker = Some("nli-stub".to_string())`); no production
  change needed — this guard keeps the flag-off path byte-identical.
  Evidence: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] COMMIT: search-without-evidence keeps the plain ranking. Hash: `529c6cf`

## Scenario: Evidence search reports a BGE verdict for every listed candidate

- [x] RED: remove `@wip`; run the `@bge` single-scenario command
  (`GIVN_FEATURES=givn/changes/search-evidence/specs cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --name "Evidence search reports a BGE verdict for every listed candidate"`).
  Evidence: the unimplemented `BGE evidence recommends all listed candidates`
  Given panicked `not implemented`; `1 scenario (1 failed)`.
- [x] GREEN: implement the `--evidence` CLI flag (`SpecCommands::Search` in
  `src/main.rs`, `cli::spec::run_search`), the evidence branch in
  `search_enabled` (synthesize `ExploreQuery`, top-5 `ExploreCandidate`s
  from `collect_scenarios`, `bge_explore_run`), the BGE per-candidate
  rendering (`BGE_RECOMMENDED`/`BGE_FILTERED` verdict lines with raw
  scores), the step `BGE evidence recommends all listed candidates`
  (`GIVN_SPEC_BGE_DIRECTIONAL_SCORES`, `3.1000` both directions on all three
  fixture pairs), and `the user searches with evidence for "<query>"`
  (full `retrieval_*` environment assembly like `user_runs` in
  `tests/steps/mod.rs`, NOT like `search_retrieval` which forwards only the
  embedder).
  Production files: `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs`.
  Test files: `tests/steps/search_evidence_steps.rs`,
  `tests/steps/spec_retrieval_steps.rs`, `tests/steps/mod.rs`.
  Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: evidence search reports a BGE verdict per candidate. Hash: `454e532`

## Scenario: Evidence search reports an NLI verdict for every candidate

- [x] RED: remove `@wip`; run the `@nli` single-scenario command
  (`--features spec-retrieval,nli-evidence`, `--name "Evidence search reports an NLI verdict for every candidate"`).
  Evidence: the unimplemented `NLI evidence recommends all listed candidates`
  Given panicked `not implemented`; `1 scenario (1 failed)`.
- [x] GREEN: implement the NLI path: `nli_explore_run` branch in
  `search_enabled`, the NLI per-candidate rendering
  (`NLI_RECOMMENDED`/`NLI_FILTERED` probability lines), and the step
  `NLI evidence recommends all listed candidates`
  (`GIVN_SPEC_NLI_DIRECTIONAL_SCORES`, logits `4.0,0.0,0.0` both directions;
  fixture helper `nli_recommend_fixture`).
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/search_evidence_steps.rs`.
  Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: evidence search reports an NLI verdict per candidate. Hash: `a54f4d1`

## Scenario: Absent BGE evidence is reported but never presented as a verdict

- [x] RED: remove `@wip`; run the `@nli` single-scenario command. The BGE
  summary line is not implemented yet, so the `stdout contains "BGE evidence: unavailable"`
  assertion fails. Evidence: `1 scenario (1 failed)` with
  `stdout missing "BGE evidence: unavailable"`.
- [x] GREEN: emit the BGE layer-without-run summary line (`BGE evidence:
  unavailable`, no per-candidate BGE lines) when the BGE run is `None` or a
  run with status `BGE_UNAVAILABLE`, and the NLI counterpart in NLI builds.
  Production files: `src/spec/retrieval.rs`.
  Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: absent BGE evidence is reported without a verdict. Hash: `c49c049`

## Scenario: Evidence search without any cross-encoder answers with the plain ranking

- [x] RED: remove `@wip`; run the e5 single-scenario command
  (`--features spec-retrieval`, no tags). The `NLI evidence: unavailable`
  summary is absent, so the `stdout contains "NLI evidence: unavailable"`
  assertion fails. Evidence: `1 scenario (1 failed)`.
- [x] GREEN: emit the NLI layer-without-run summary (`NLI evidence:
  unavailable`; e5 binaries print both summaries while rank lines stay
  byte-identical). Production files: `src/spec/retrieval.rs`.
  Evidence: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `11 steps (11 passed)`.
- [x] COMMIT: search without cross-encoders reports unavailable evidence. Hash: `e6562d1`

## Scenario: Candidates beyond the evidence pair budget stay unscored

- [x] RED: remove `@wip`; run the `@bge` single-scenario command (reused
  Given `the second-stage pair budget is 2`). The `BGE_UNSCORED` per-candidate
  line is not implemented yet, so `stdout contains "BGE_UNSCORED"` fails.
  Evidence: `1 scenario (1 failed)` with `stdout missing "BGE_UNSCORED"`.
- [x] GREEN: implement the `BGE_UNSCORED` per-candidate else-branch for
  candidates without a pair (beyond `max_unique_pairs`); never
  `BGE_FILTERED` for them. Production files: `src/spec/retrieval.rs`.
  Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `10 steps (10 passed)`.
- [x] COMMIT: candidates beyond the pair budget stay unscored. Hash: `e56d1ba`

## Scenario: An over-limit search pair receives BGE token-cap evidence

- [x] RED: remove `@wip`; run the `@bge` single-scenario command. The
  unimplemented `every BGE pair exceeds the token limit` Given panicked
  `not implemented`. Evidence: `1 scenario (1 failed)`.
- [x] GREEN: implement the step (`retrieval_bge_token_budget =
  Some("all-513")`) and the `BGE_TOKEN_CAP` per-candidate line
  (`pair <p> tokens, left <l>, right <r>, limit <m>`).
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/search_evidence_steps.rs`.
  Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: over-limit search pairs receive BGE token-cap evidence. Hash: `97514a7`

## Scenario: An over-limit search pair receives NLI token-cap evidence

- [x] RED: remove `@wip`; run the `@nli` single-scenario command. The
  unimplemented `every NLI pair exceeds the token limit` Given panicked
  `not implemented`. Evidence: `1 scenario (1 failed)`.
- [x] GREEN: implement the step (`retrieval_nli_token_budget =
  Some("all-513")`) and the `NLI_TOKEN_CAP` per-candidate line.
  Production files: `src/spec/retrieval.rs`. Test files:
  `tests/steps/search_evidence_steps.rs`.
  Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: rerun the named scenario. Evidence: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] COMMIT: over-limit search pairs receive NLI token-cap evidence. Hash: `04a3cef`

## Scenario: Search reports BGE and NLI evidence through the givn CLI (@e2e)

Run after all non-e2e scenarios are green. `verify.e2e_command` is
`./run-tests.sh --e2e` (distinct from `verify.command`; the e2e scope is
strictly smaller — established by the runner tag filters).

- [x] RED: remove `@wip`; `tests/steps/search_evidence_e2e_steps.rs` holds
  e2e step text with `unimplemented!()` bodies; run the combined
  single-scenario command:
  `GIVN_FEATURES=givn/changes/search-evidence/specs cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --name "Search reports BGE and NLI evidence through the givn CLI"`.
  Evidence: `1 scenario (1 failed)`, `Step panicked. Captured output: not implemented: end-to-end evidence search project`.
- [x] GREEN: implement the e2e steps as real subprocess spawns of
  `CARGO_BIN_EXE_givn` (`givn spec search --evidence "<query>"`) against the
  stub-indexed fixture project; assert `rank 1`, `BGE_RECOMMENDED`,
  `NLI_RECOMMENDED` in stdout via the shared generic output steps.
  Test files: `tests/steps/search_evidence_e2e_steps.rs`,
  `tests/steps/spec_retrieval_steps.rs` (shared fixture helpers),
  `tests/steps/mod.rs` (`world_envs`).
  Evidence: `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: rerun the named e2e scenario; then run the full
  `./run-tests.sh` and `./run-tests.sh --e2e` (see Final evidence).
  Evidence: e2e targeted rerun `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] COMMIT: end-to-end evidence search through the real CLI (also carries
  the arc42/ADR durable updates). Hash: `10cd8c5`

## Final evidence

- `./run-tests.sh` full suite: exit 0 — the four feature-set runs all
  report zero failed scenarios (tails: `9 scenarios (9 passed)`,
  `57 steps (57 passed)` / `8 scenarios (8 passed)` / `6 scenarios (6 passed)`;
  no `(failed)` line anywhere in the log).
- `./run-tests.sh --e2e` full suite: exit 0 — `53 scenarios (53 passed)`
  plus the `@bge`/`@nli`/`@nli-combined` e2e subsets all passed; zero failed.
- `givn lint --change search-evidence`: clean, exit 0.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo fmt --all` applied; formatting committed as
  `df0ec6f refactor(retrieval): cargo fmt search-evidence`; the evidence
  candidates binding was later scoped to the opt-in branch
  (`9ea94ee refactor(retrieval): scope evidence candidates to the opt-in branch`)
  so e5-only builds emit no warnings.
- `givn check tasks --change search-evidence`: exit 0 (verify hook green),
  and `givn status` reports all `38/38` task boxes checked.
- Coverage: the cucumber-rs runner used by this repository does not
  instrument coverage (no cobertura output in `verify.command`); execution
  coverage of the new code is therefore an unmeasured gap to be classified
  manually in review. The four compiled feature-set runs above execute every
  new branch (BGE/NLI lines, summaries, token caps, unscored tail, typing
  cast flag-off path).