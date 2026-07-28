# Tasks: bubblewrap-sandbox

## Setup

- [x] **Setup: strict-mode proof and step scaffolding**
  - Created `tests/features/sandbox/` directory with symlink to delta spec
  - Created `tests/features/steps/sandbox/sandbox_steps.py` with `NotImplementedError` stubs for all steps
  - Proof: `uv run behave tests/features/ --no-capture --no-skipped -n "Bubblewrap sandbox restricts filesystem write access to configured writable paths"` — EXIT: 1, NotImplementedError raised from `step_then_agent_can_write_workspace`
  - Runner output:
    ```
    0 features passed, 0 failed, 1 error, 1 skipped
    0 scenarios passed, 0 failed, 1 error, 35 skipped
    2 steps passed, 0 failed, 1 error, 171 skipped
    EXIT: 1
    ```
  - Commit: `chore(sandbox): scaffold step stubs and prove strict mode`
  - Hash: `b949595`

## Modified scenarios (update existing `test_harness_steps.py`)

- [ ] **Scenario 1: The agent runs inside a sandlock sandbox** (`@givn.modified`)
  - RED: Remove `@wip` if present. Existing step defs at `tests/features/steps/harness/test_harness_steps.py`. Update the step that checks sandlock to check bubblewrap instead (`ralph executes inside a bubblewrap sandbox`, `sandbox workdir is the workspace directory`). Run `uv run behave tests/features/ --no-capture -n "The agent runs inside a sandlock sandbox"` — MUST FAIL.
  - Output:
    ```
    0 features passed, 2 failed, 0 skipped
    0 scenarios passed, 2 failed, 34 skipped
    4 steps passed, 2 failed, 168 skipped
    Took 0min 0.179s
    EXIT: 1
    ```
  - GREEN: Implement: modify `sandbox.py` (`check_bubblewrap`, `build_bwrap_cmd`), `runner.py` (`_run_with_bwrap`, modify `run_agent` dispatch), `cli.py` (replace `check_sandlock` with `check_bubblewrap`, remove `--profile`), `report.py` (change `sandboxed: bool` to `sandbox: str`, add `resource_limits: str`). Run single scenario — PASSES.
  - Files: `src/givn_harness/sandbox.py`, `src/givn_harness/runner.py`, `src/givn_harness/cli.py`, `src/givn_harness/report.py`
  - Output:
    ```
    1 feature passed, 0 failed, 1 skipped
    1 scenario passed, 0 failed, 35 skipped
    4 steps passed, 0 failed, 170 skipped
    Took 0min 0.141s
    EXIT: 0
    ```
  - REFACTOR: Removed unused imports (`pathlib.Path` from sandbox.py, `from pathlib import Path` from runner.py). Run single scenario — still PASSES.
  - Output:
    ```
    1 feature passed, 0 failed, 1 skipped
    1 scenario passed, 0 failed, 35 skipped
    4 steps passed, 0 failed, 170 skipped
    Took 0min 0.153s
    EXIT: 0
    ```
  - Commit: `feat(sandbox): replace sandlock with bubblewrap sandbox`
  - Hash: `fd041b6`

- [ ] **Scenario 2: Sandbox filesystem uses learn-then-confine mode** (`@givn.removed`) — No implementation. Placeholder scenario removed on archive.

- [ ] **Scenario 3: Sandlock is not available, harness falls back to --no-sandbox** (`@givn.modified`)
  - RED: Update existing step defs in `test_harness_steps.py`. The Given now checks bubblewrap not in PATH (was: sandlock not installed). Run `uv run behave tests/features/ --no-capture -n "Sandlock is not available, harness falls back to --no-sandbox"` — MUST FAIL.
  - No RED phase: scenario GREEN immediately (all steps reused or already implemented).
  - GREEN: `check_bubblewrap()` already returns False when bwrap not in PATH (implemented in Scenario 1). Step defs updated to assert `sandbox: "none"` in report. Run single scenario — PASSES.
  - Files: `tests/features/steps/harness/test_harness_steps.py`, `tests/features/steps/sandbox/sandbox_steps.py`
  - Output:
    ```
    1 feature passed, 0 failed, 1 skipped
    1 scenario passed, 0 failed, 35 skipped
    5 steps passed, 0 failed, 169 skipped
    Took 0min 0.082s
    EXIT: 0
    ```
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: (same as GREEN — step reuse, no refactor needed)
  - Commit: `feat(sandbox): fall back to unsandboxed when bubblewrap not available`
  - Hash: `9b1f53f`

## New scenarios (new `sandbox_steps.py`)

- [ ] **Scenario 4: Bubblewrap sandbox restricts filesystem write access** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Bubblewrap sandbox restricts filesystem write access to configured writable paths"` — MUST FAIL.
  - Output: `________`
  - GREEN: Implement step defs: execute harness with an agent that writes to workspace (succeeds), `/tmp` (succeeds), `/home` (fails with bwrap access error), `/var` (fails), `/opt` (fails). Verify via bwrap stderr for access-denied on disallowed paths.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify filesystem write restrictions`
  - Hash: `________`

- [ ] **Scenario 5: Bubblewrap sandbox restricts filesystem read access** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Bubblewrap sandbox restricts filesystem read access to configured readable paths"` — MUST FAIL.
  - Output: `________`
  - GREEN: Implement step defs: execute harness with agent that reads from `/usr`, `/bin`, `/lib`, `/lib64`, `/etc` (succeeds), reads from outside paths (fails with bwrap access error).
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify filesystem read restrictions`
  - Hash: `________`

- [ ] **Scenario 6: Bubblewrap sandbox enforces process count limit** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Bubblewrap sandbox enforces process count limit"` — MUST FAIL.
  - Output: `________`
  - GREEN: Step defs verify `_wrap_with_limits()` produces `systemd-run --user --scope -p TasksMax=256 -- <cmd>`. No orchestration of actual agent forking — spec documents expected behavior, steps verify command construction. The Given "agent attempts to fork more than 256 processes" is accepted as a documented precondition.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`, `src/givn_harness/sandbox.py` (if `_wrap_with_limits` not yet exposed for testing)
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify process count limit configuration`
  - Hash: `________`

- [ ] **Scenario 7: Bubblewrap sandbox enforces memory limit** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Bubblewrap sandbox enforces memory limit"` — MUST FAIL.
  - Output: `________`
  - GREEN: Step defs verify `_wrap_with_limits()` produces `systemd-run --user --scope -p MemoryMax=16G -- <cmd>`.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify memory limit configuration`
  - Hash: `________`

- [ ] **Scenario 8: Agent writes persist after sandbox exits** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Agent writes to workspace persist after sandbox exits"` — MUST FAIL.
  - Output: `________`
  - GREEN: Implement step defs: execute harness with a task asking agent to write `hello.txt` in workspace. After sandbox exits, verify file exists in workspace directory with expected content.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify workspace writes persist after sandbox exit`
  - Hash: `________`

- [ ] **Scenario 9: Agent stdout and logs are captured** (`@givn.added`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture -n "Agent stdout and logs are captured from the sandbox"` — MUST FAIL.
  - Output: `________`
  - GREEN: Implement step defs: execute harness with agent that writes to stdout. Verify agent stdout is captured and written to `<workspace>/agent.log`, and the report contains the log output.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): verify stdout capture from bubblewrap sandbox`
  - Hash: `________`

## E2E smoke tests

- [ ] **Scenario 10: E2E smoke test for bubblewrap sandbox with real agent** (`@givn.added @e2e`)
  - RED: Remove `@wip`. Run `uv run behave tests/features/ --no-capture --tags=@e2e -n "E2E smoke test for bubblewrap sandbox with real agent"` — MUST FAIL.
  - Output: `________`
  - GREEN: Implement e2e step defs: create `tasks/trivial.md`, run full harness with the sandbox, verify agent completes task, verify report `sandbox: "bwrap"`, verify workspace changes visible to evaluator.
  - Files: `tests/features/sandbox/steps/sandbox_steps.py`, `tasks/trivial.md`
  - Output: `________`
  - REFACTOR: Clean up. Run single scenario — still PASSES.
  - Output: `________`
  - Commit: `test(sandbox): e2e smoke test with real agent in bubblewrap sandbox`
  - Hash: `________`

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run the harness with a task description | User runs the harness end-to-end and observes a complete autoresearch loop (test-harness capability) | CLI | `subprocess.run(["givn-harness", "run", "--task", "tasks/shopping-list.md", "--output", "reports/"])` |
| E2E sandbox smoke test | E2E smoke test for bubblewrap sandbox with real agent | CLI | `subprocess.run(["givn-harness", "run", "--task", "tasks/trivial.md", "--output", "reports/"])` |