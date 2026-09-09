# Tasks: fix-opencode-llm-attribution

The owning behavior is the reusable `support` fragment. Its existing
commit-attribution guarantees remain in force: attribution is best effort,
missing metadata never blocks a commit, and the environment protocol remains
process-scoped. Attributed messages also carry the managed co-author line
`<model>/<reasoning>@<harness> with Givn <version>`.
No Persona is confirmed for this change.

## Setup

- [x] Runner contract confirmed: `./run-tests.sh` executes the repository
      feature suite plus the active change when `GIVN_FEATURES` is set;
      `./run-tests.sh --e2e` executes the E2E-filtered subset.
- [x] Strict mode confirmed by `GivnWorld::cucumber().fail_on_skipped()` in
      `tests/features_runner.rs`. The initial targeted run failed on the
      undefined first step with a non-zero exit before step bindings existed.
- [x] The scenario uses the existing capability-specific E2E file
      `tests/steps/llm_attribution_e2e_steps.rs`; no new runner or service is
      required. Node.js latest LTS executes the generated plugin module.
- [x] E2E count proof: the change scope contains three scenarios and all are
      tagged `@e2e`; full and E2E runs each execute three scenarios, so the
      explicit all-scenarios-are-E2E exception applies.
- [x] No local service, isolated network, database, provider credential, or
      digital twin is required.

## E2E Scenario

### OpenCode prepares attribution using its selected model

Domain constraint: the OpenCode harness supplies the active model selection;
Givn exposes it through the existing attribution environment without changing
the Git-side protocol or blocking shell execution.

- [x] RED: the original targeted run executed the generated plugin and failed
      on the LLM assertion while the asset still read obsolete top-level model
      fields:

      ```text
      GIVN_FEATURES=givn/changes/fix-opencode-llm-attribution/specs cargo test --test features_runner -- --name "OpenCode prepares attribution using its selected model"
      1 scenario (1 failed)
      4 steps (3 passed, 1 failed)
      assertion `left == right` failed
        left: None
        right: Some("openai/gpt-5.6-luna")
      exit 101
      ```

- [x] GREEN: `assets/agent-integrations/opencode/givn-attribution.js` and the
      checked-in `.opencode/plugins/givn-attribution.js` now read
      `input.model.providerID` and `input.model.modelID`. The E2E steps invoke
      the generated module with the current payload shape and assert LLM,
      reasoning mode, harness, and committer environment values. The plugin
      initializes missing LLM/reasoning values to `unavailable`, and the shared
      Git path stamps only the requested managed co-author line. The source and
      checked-in generated assets are also asserted byte-for-byte equal.
      Targeted result:

      ```text
      GIVN_FEATURES=givn/changes/fix-opencode-llm-attribution/specs cargo test --test features_runner -- --name "OpenCode prepares attribution using its selected model"
      1 feature, 1 scenario, 7 steps passed
      exit 0
      ```

- [x] REFACTOR: retained the existing session cache and environment protocol;
      added no compatibility alias or new abstraction. `cargo fmt --all
      -- --check`, the targeted scenario, the change-scoped full run, and the
      change-scoped E2E run pass.

- [x] COMMIT: `95f4b3e9` -- `fix(llm-attribution): keep only co-author trailer`

### Claude attributes a commit using its current model

Domain constraint: the Claude harness supplies the current model in its session
      transcript; Givn exposes that model through the existing committer identity and
      managed co-author message line without blocking the Bash command.

- [x] RED: before transcript lookup was implemented, the Claude path used the
      literal `claude-code` fallback and could not satisfy the model assertion.
      The scenario was marked `@wip` while its real hook binding was introduced;
      it failed until transcript extraction was implemented.
- [x] GREEN: `claude_model` reads the latest assistant model from
      `transcript_path`, while retaining `CLAUDE_MODEL` and `claude-code`
      fallbacks. The generated hook exports context for every Bash command,
      including an RTK-wrapped command, and the returned command creates a real
      Git commit with only the managed co-author line.

      ```text
      GIVN_FEATURES=givn/changes/fix-opencode-llm-attribution/specs cargo test --test features_runner -- --name "Claude attributes a commit using its current model"
      1 feature, 1 scenario, 7 steps passed
      exit 0
      ```

- [x] REFACTOR: kept the shell hook as a thin launcher, reused the existing
       command rewrite path, exported context across RTK-wrapped Bash commands,
       and normalized the managed co-author line while removing stale legacy
       Givn trailers, preserving ordinary human co-author trailers and fallback
       semantics.
- [x] COMMIT: `95f4b3e9` -- `fix(llm-attribution): keep only co-author trailer`

## Verification

- [x] `givn lint --change fix-opencode-llm-attribution` reports one file
      checked and clean.
- [x] `givn check review --change fix-opencode-llm-attribution` passes verify,
      verify-e2e, run declaration, and overlap disposition gates. Integrity
      reaches only the repository-required scenario commit-hash check because
      no commit was authorized in this session.
- [x] Existing scenario `An attributed commit exposes the current LLM and
      harness` passes through the real Git commit path.
- [x] `cargo check --all-targets` passes.
- [x] Formatting audit completed. The repository-wide `cargo fmt --all --
      --check` still reports pre-existing formatting drift in unrelated
      attribution-installation scaffolding and module ordering; no unrelated
      formatting churn was retained.
- [x] Final verification: `./run-tests.sh` reports 390/390 passed;
      `./run-tests.sh --e2e` reports 75/75 passed; `cargo check --all-targets`,
      `cargo clippy --workspace --all-targets -- -D warnings`,
      `givn lint --change fix-opencode-llm-attribution`, and `git diff --check`
      pass.
