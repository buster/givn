# Tasks: llm-attribution

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT per
`specs/support/llm-attribution.feature`. Non-E2E scenarios are implemented
before the two `@e2e` scenarios.

The support use case guarantees remain in force for every scenario: attribution
is best effort, missing context never blocks a commit, human message editing is
preserved, the checked-in hook is shared, and an existing `commit-msg` hook is
preserved and chained before Givn attribution.

## Setup: runner, strict mode, step skeletons, e2e filter proof

- [x] Use the existing cucumber-rs runner in `tests/features_runner.rs`, which
      already calls `.fail_on_skipped()`. Register the one-file-per-capability
      step modules required by design:
      `tests/steps/llm_attribution_steps.rs` and
      `tests/steps/llm_attribution_e2e_steps.rs`. New RED bodies use
      `unimplemented!()`.
- [x] Strictness proof: run one change scenario with an unimplemented step and
      paste the non-zero command/output here:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "A human commit remains usable without attribution context"
  output: 1 feature, 1 scenario, 1 failed; the first step panicked with "not implemented" from llm_attribution_steps.rs
  exit: 101
  ```

- [x] `verify.command` is `./run-tests.sh`; `verify.e2e_command` is
      `./run-tests.sh --e2e`. Run both and paste the full-suite and E2E-only
      scenario counts here, proving the E2E count is smaller:

  ```text
  full command/output:
  e2e command/output:
  count comparison:
  4 non-E2E scenarios passed; 2 E2E scenarios passed; 2 < 6 total change scenarios.
  ```

- [x] No services or digital twins are required. Confirm isolated temporary
      project directories and local Git repositories are sufficient.

## Scenario 1: A human commit remains usable without attribution context

- [x] RED — remove `@wip` from this scenario only, implement the named steps
      with `unimplemented!()`, and run:
      `GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "A human commit remains usable without attribution context"`

  ```text
  output: 1 feature, 1 scenario, 1 failed; the first step panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — create a real temporary Git repository, activate the checked-in
      hook, run `git commit` without attribution variables, and assert success,
      unchanged message, and unchanged human committer.

  Production/test files:
  - `src/cli/attribution.rs`
  - `src/cli/git.rs`
  - `src/main.rs`
  - `src/cli/mod.rs`
  - `assets/hooks/commit-msg`
  - `tests/steps/llm_attribution_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 5 passed; real Git commit and unchanged message asserted
  exit: 0
  ```

- [x] REFACTOR — rerun the same named scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 5 passed
  exit: 0
  ```

- [x] COMMIT: 7445dad -- A human commit remains usable without attribution context

  ```text
  commit hash: 7445dad
  ```

## Scenario 2: A partial attribution context does not block a commit

- [x] RED — remove `@wip` from this scenario only, run the named scenario, and
      paste the non-zero output:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "A partial attribution context does not block a commit"
  output: 1 feature, 1 scenario, 2 steps passed, 1 failed; the partial-context step panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — stamp a partial context with explicit unavailable values and
      `partial` status while allowing the real commit to complete.

  Production/test files:
  - `src/cli/attribution.rs`
  - `assets/hooks/commit-msg`
  - `tests/steps/llm_attribution_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 6 passed; real Git commit contains harness and unavailable trailers
  exit: 0
  ```

- [x] REFACTOR — rerun the same named scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 6 passed
  exit: 0
  ```

- [x] COMMIT: f65d0c2 -- A partial attribution context does not block a commit

  ```text
  commit hash: f65d0c2
  ```

## Scenario 3: An existing commit hook runs before Givn attribution

- [x] RED — remove `@wip` from this scenario only, run the named scenario, and
      paste the non-zero output:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "An existing commit hook runs before Givn attribution"
  output: 1 feature, 1 scenario, 1 failed; the existing-hook setup step panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — install an idempotent local dispatcher that preserves an unknown
      existing hook as `.givn-original`, runs it first, preserves its failure
      result, and invokes the checked-in hook only after success.

  Production/test files:
  - `src/cli/git.rs`
  - `src/cli/init.rs`
  - `src/skills/targets.rs`
  - `assets/hooks/git-commit-msg-launcher`
  - `tests/steps/llm_attribution_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 7 passed; existing marker and attribution trailers asserted
  exit: 0
  ```

- [x] REFACTOR — rerun the same named scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 7 passed
  exit: 0
  ```

- [x] COMMIT: 483c56d -- An existing commit hook runs before Givn attribution
- [x] COMMIT: 7c663c3e -- Centralize existing hook backup naming

  ```text
  commit hash: 483c56d, 7c663c3e
  ```

## Scenario 4: Attribution follows a model change between commits

- [x] RED — remove `@wip` from this scenario only, run the named scenario, and
      paste the non-zero output:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --tags 'not @e2e'
  output: 1 feature, 4 scenarios, 3 passed, 1 failed; the context-change step panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — ensure each real Git commit reads only the current process-scoped
      attribution environment and replaces managed trailers without carrying
      stale values from the previous commit.

  Production/test files:
  - `src/cli/attribution.rs`
  - `assets/hooks/commit-msg`
  - `tests/steps/llm_attribution_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 7 passed; both commit messages carry their own context
  exit: 0
  ```

- [x] REFACTOR — rerun the same named scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 7 passed
  exit: 0
  ```

- [x] COMMIT: f1bcbb6 -- Attribution follows a model change between commits
- [x] COMMIT: 62ea6540 -- Centralize unavailable attribution values

  ```text
  commit hash: f1bcbb6, 62ea6540
  ```

## Scenario 5: Consumer initializes a project and receives shared attribution plumbing

- [x] E2E setup — read the canonical specs policy, create
      `tests/steps/llm_attribution_e2e_steps.rs`, and confirm the existing
      `./run-tests.sh --e2e` filter selects only `@e2e` scenarios. No local
      service startup is required.

  ```text
  GIVN_FEATURES=givn/changes/llm-attribution/specs ./run-tests.sh --non-e2e -> 4 scenarios, 4 passed
  GIVN_FEATURES=givn/changes/llm-attribution/specs ./run-tests.sh --e2e -> 2 scenarios, 2 passed
  2 < 6 total change scenarios; the E2E filter is a strict subset.
  ```

- [x] RED — remove `@wip` from this E2E scenario only and run the exact E2E
      scenario command; paste the non-zero output:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "Consumer initializes a project and receives shared attribution plumbing"
  output: 1 feature, 1 scenario, 1 failed; the first E2E step panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — invoke the compiled CLI through a real subprocess with
      `givn init --agents opencode,claude` in a temporary Git repository and
      assert the checked-in hook, local activation, OpenCode plugin, Claude
      hook, and Claude settings entry.

  Production/test files:
  - `src/skills/targets.rs`
  - `src/cli/git.rs`
  - `assets/agent-integrations/opencode/givn-attribution.js`
  - `assets/agent-integrations/claude/givn-attribution.sh`
  - `tests/steps/llm_attribution_e2e_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 7 passed; real CLI and project filesystem assertions passed
  exit: 0
  ```

- [x] REFACTOR — rerun the exact E2E scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 7 passed
  exit: 0
  ```

- [x] COMMIT: 3962dbc -- Consumer initializes a project and receives shared attribution plumbing

  ```text
  commit hash: 3962dbc
  ```

## Scenario 6: An attributed commit exposes the current LLM and harness

- [x] RED — remove `@wip` from this E2E scenario only and run the exact E2E
      scenario command; paste the non-zero output:

  ```text
  command: GIVN_FEATURES=givn/changes/llm-attribution/specs cargo test --test features_runner -- --name "An attributed commit exposes the current LLM and harness"
  output: 1 feature, 1 scenario, 4 passed, 1 failed; the committer assertion panicked with "not implemented"
  exit: 101
  ```

- [x] GREEN — invoke real Git with complete runtime attribution and committer
      environment, then assert through `git show` that the LLM/harness are in
      the committer identity and that the final message ends with normalized
      Givn version, LLM, reasoning, harness, and status trailers.

  Production/test files:
  - `src/cli/attribution.rs`
  - `assets/hooks/commit-msg`
  - `tests/steps/llm_attribution_e2e_steps.rs`

  ```text
  output: 1 feature, 1 scenario, 5 passed; dedicated E2E Git steps asserted the committer and final message trailers
  exit: 0
  ```

- [x] REFACTOR — rerun the exact E2E scenario and paste the passing output:

  ```text
  output: 1 feature, 1 scenario, 5 passed
  exit: 0
  ```

- [x] COMMIT: 981d4b9 -- An attributed commit exposes the current LLM and harness
- [x] COMMIT: 6f6052a -- Keep attributed commits on dedicated E2E steps

  ```text
  commit hash: 981d4b9, 6f6052a
  ```
