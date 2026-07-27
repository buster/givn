# Tasks: givn-test-harness

- [x] Setup: configure test infrastructure, strict mode, and verify.command
      PROOF OF STRICTNESS: `uv run behave tests/features/ --no-capture -n "PROOF"` → exit 1 (NotImplementedError raised). Full suite without --tags=-wip exits 1 (all 25 scenarios error). Full suite with --tags=-wip exits 0 (25 skipped).
      verify.command: `uv run behave tests/features/ --no-capture --tags=-wip`
      Step file: `tests/features/steps/harness/test_harness_steps.py` (necessary deviation from design: behave discovers steps under `tests/features/steps/`, not `tests/features/harness/steps/`).
      Commits: none yet (setup only, no scenarios implemented)

## Phase 1 — Workspace infrastructure (no sandlock, no ralph)

- [x] test-harness: Harness creates an isolated workspace outside the repository
      Design constraints:
        - `workspace.py` creates a temp dir under `/tmp/givn-harness-workspaces/` (configurable via `--workspace-root`).
        - Workspace path must not be inside the givn repository tree.
        - CLI entry point via click: `givn-harness run --task <path> --output <dir>`.
        - `types.py` defines the `RunResult` dataclass.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Harness creates an isolated workspace outside the repository                       # tests/features/harness/test-harness.feature:13
            Given a workspace directory at "/tmp/givn-harness-workspaces"                              # tests/features/steps/harness/test_harness_steps.py:4
              Traceback (most recent call last):
                ...
              NotImplementedError: step: Given a workspace directory at "/tmp/givn-harness-workspaces"

            When the user runs `givn-harness run --task task.md --output reports/`                     # None
            Then the workspace is created in a temporary directory under the configured workspace root # None
            And the workspace directory is not within the givn repository                              # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:13  Harness creates an isolated workspace outside the repository

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        0 steps passed, 0 failed, 1 error, 126 skipped
        ```
      GREEN:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        4 steps passed, 0 failed, 123 skipped
        ```
        Files created/modified: `src/givn_harness/cli.py`, `src/givn_harness/workspace.py`, `src/givn_harness/types.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        4 steps passed, 0 failed, 123 skipped
        ```
      COMMIT: `3b3d643` — feat(test-harness): Harness creates an isolated workspace outside the repository

- [x] test-harness: Workspace is initialized with git init and givn init
      Design constraints:
        - `workspace.py` runs `git init` and `givn init` as subprocesses in the workspace dir.
        - `givn init` scaffolds `givn/` with `config.yaml`, `specs/`, `changes/`, `archive/`.
        - `GIVN_BINARY` env var for givn binary path (defaults to `givn` on PATH).
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Workspace is initialized with git init and givn init           # tests/features/harness/test-harness.feature:20
            Given a workspace directory at "/tmp/givn-harness-workspaces"          # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task task.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:25
            Then the workspace contains an initialized git repository              # tests/features/steps/harness/test_harness_steps.py:67
              Traceback (most recent call last):
                ...
              NotImplementedError: step: Then the workspace contains an initialized git repository

            And the workspace contains a givn/ directory scaffolded by givn init   # None
            And the workspace contains givn/config.yaml                            # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:20  Workspace is initialized with git init and givn init

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        2 steps passed, 0 failed, 1 error, 124 skipped
        ```
      GREEN:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        5 steps passed, 0 failed, 122 skipped
        Took 0min 0.068s
        ```
        Files modified: `src/givn_harness/workspace.py`, `src/givn_harness/cli.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        5 steps passed, 0 failed, 122 skipped
        Took 0min 0.063s
        ```
      COMMIT: `0c24647` — feat(test-harness): Workspace is initialized with git init and givn init

- [x] test-harness: verify.command is at the sentinel default after initialization
      Design constraints:
        - After `givn init`, `givn/config.yaml` contains `command: "givn missing-testrunner"` (sentinel).
        - This is a givn behavior assertion, NOT a harness behavior assertion — the harness calls `givn init` and the sentinel is givn's default.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: verify.command is at the sentinel default after initialization                               # tests/features/harness/test-harness.feature:28
            Given a workspace directory at "/tmp/givn-harness-workspaces"                                        # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task task.md --output reports/`                               # tests/features/steps/harness/test_harness_steps.py:25
            Then the workspace's givn/config.yaml contains the verify.command sentinel "givn missing-testrunner" # tests/features/steps/harness/test_harness_steps.py:97
              Traceback (most recent call last):
                ...
              NotImplementedError: step: the workspace's givn/config.yaml contains the verify.command sentinel "givn missing-testrunner"

        Errored scenarios:
          tests/features/harness/test-harness.feature:28  verify.command is at the sentinel default after initialization

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        2 steps passed, 0 failed, 1 error, 124 skipped
        Took 0min 0.062s
        ```
      GREEN:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: verify.command is at the sentinel default after initialization                               # tests/features/harness/test-harness.feature:28
            Given a workspace directory at "/tmp/givn-harness-workspaces"                                        # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task task.md --output reports/`                               # tests/features/steps/harness/test_harness_steps.py:25
            Then the workspace's givn/config.yaml contains the verify.command sentinel "givn missing-testrunner" # tests/features/steps/harness/test_harness_steps.py:97

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        3 steps passed, 0 failed, 124 skipped
        Took 0min 0.074s
        ```
        Files modified: `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: verify.command is at the sentinel default after initialization                               # tests/features/harness/test-harness.feature:28
            Given a workspace directory at "/tmp/givn-harness-workspaces"                                        # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task task.md --output reports/`                               # tests/features/steps/harness/test_harness_steps.py:25
            Then the workspace's givn/config.yaml contains the verify.command sentinel "givn missing-testrunner" # tests/features/steps/harness/test_harness_steps.py:97

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        3 steps passed, 0 failed, 124 skipped
        Took 0min 0.061s
        ```
      COMMIT: `0a6b4e1` — feat(test-harness): verify.command is at the sentinel default after initialization

- [x] test-harness: The task prompt is delivered to the workspace as a file
      Design constraints:
        - Task file read from `--task` argument and copied to `<workspace>/task.md`.
        - File content is preserved byte-for-byte.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt is delivered to the workspace as a file                       # tests/features/harness/test-harness.feature:34
            Given a task prompt file at "tasks/shopping-list.md" with a task description          # tests/features/steps/harness/test_harness_steps.py:97
              Traceback (most recent call last):
                ...
              NotImplementedError: step: a task prompt file at 'tasks/shopping-list.md' with a task description

            When the user runs `givn-harness run --task tasks/shopping-list.md --output reports/` # None
            Then the workspace contains a file "task.md" with the task description content        # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:34  The task prompt is delivered to the workspace as a file

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        0 steps passed, 0 failed, 1 error, 126 skipped
        Took 0min 0.001s
        ```
      GREEN:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt is delivered to the workspace as a file                       # tests/features/harness/test-harness.feature:34
            Given a task prompt file at "tasks/shopping-list.md" with a task description          # tests/features/steps/harness/test_harness_steps.py:97
            When the user runs `givn-harness run --task tasks/shopping-list.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:110
            Then the workspace contains a file "task.md" with the task description content        # tests/features/steps/harness/test_harness_steps.py:126

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        3 steps passed, 0 failed, 124 skipped
        Took 0min 0.067s
        ```
        Files modified: `src/givn_harness/workspace.py`, `src/givn_harness/cli.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt is delivered to the workspace as a file                       # tests/features/harness/test-harness.feature:34
            Given a task prompt file at "tasks/shopping-list.md" with a task description          # tests/features/steps/harness/test_harness_steps.py:97
            When the user runs `givn-harness run --task tasks/shopping-list.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:110
            Then the workspace contains a file "task.md" with the task description content        # tests/features/steps/harness/test_harness_steps.py:126

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        3 steps passed, 0 failed, 124 skipped
        Took 0min 0.277s
        ```
      COMMIT: `760207c` — feat(test-harness): The task prompt is delivered to the workspace as a file

- [x] test-harness: The task prompt file path that does not exist is rejected
      Design constraints:
        - Missing file → non-zero exit, error message on stderr.
        - click handles argument validation; error message produced via click.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt file path that does not exist is rejected             # tests/features/harness/test-harness.feature:40
            Given a workspace directory at "/tmp/givn-harness-workspaces"                 # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task nonexistent.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:110
              Traceback (most recent call last):
                ...
              NotImplementedError: TODO

            Then the command exits non-zero                                               # None
            And stderr reports that the task file does not exist                          # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:40  The task prompt file path that does not exist is rejected

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        1 step passed, 0 failed, 1 error, 125 skipped
        Took 0min 0.001s
        ```
      GREEN:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt file path that does not exist is rejected             # tests/features/harness/test-harness.feature:40
            Given a workspace directory at "/tmp/givn-harness-workspaces"                 # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task nonexistent.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:110
            Then the command exits non-zero                                               # tests/features/steps/harness/test_harness_steps.py:125
            And stderr reports that the task file does not exist                          # tests/features/steps/harness/test_harness_steps.py:132

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        4 steps passed, 0 failed, 123 skipped
        Took 0min 0.065s
        ```
        No new production code — click's `exists=True` on `--task` already handles file existence validation.
        Files modified: `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: The task prompt file path that does not exist is rejected             # tests/features/harness/test-harness.feature:40
            Given a workspace directory at "/tmp/givn-harness-workspaces"                 # tests/features/steps/harness/test_harness_steps.py:12
            When the user runs `givn-harness run --task nonexistent.md --output reports/` # tests/features/steps/harness/test_harness_steps.py:110
            Then the command exits non-zero                                               # tests/features/steps/harness/test_harness_steps.py:125
            And stderr reports that the task file does not exist                          # tests/features/steps/harness/test_harness_steps.py:132

        1 feature passed, 0 failed, 0 skipped
        5 scenarios passed, 0 failed, 20 skipped
        19 steps passed, 0 failed, 108 skipped
        Took 0min 0.326s
        ```
      COMMIT: `c43748f` — feat(test-harness): The task prompt file path that does not exist is rejected

## Phase 2 — Report and evaluation (no sandlock, no ralph; needs workspace)

- [x] test-harness: Report includes all ten automated checks
      Design constraints:
        - `evaluator.py` runs 10 checks (artifact_existence, lint_clean, tasks_all_done, valid_commits, archive_succeeded, specs_clean, step_files_nonempty, review_pass, design_review_pass, verify_green).
        - Each check produces a `Check` (name, passed, detail).
        - Evaluation reads from the workspace filesystem — this scenario uses pre-existing workspace content, not a live agent run.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Report includes all ten automated checks    # tests/features/harness/test-harness.feature:89
            Given a completed test run at "reports/run-001/"    # tests/features/steps/harness/test_harness_steps.py:175
              Traceback (most recent call last):
                ...
                File "tests/features/steps/harness/test_harness_steps.py", line 177, in step_completed_test_run
                  raise NotImplementedError("TODO")
              NotImplementedError: TODO

            When the harness evaluates the workspace            # None
            Then the run report lists all ten automated checks: # None
              | check                                   |
              | artifact existence                      |
              ...
            And each check has a pass or fail result            # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:89  Report includes all ten automated checks

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        0 steps passed, 0 failed, 1 error, 126 skipped
        Took 0min 0.001s
        ```
      GREEN:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Report includes all ten automated checks    # tests/features/harness/test-harness.feature:89
            Given a completed test run at "reports/run-001/"    # tests/features/steps/harness/test_harness_steps.py:175
            When the harness evaluates the workspace            # tests/features/steps/harness/test_harness_steps.py:267
            Then the run report lists all ten automated checks: # tests/features/steps/harness/test_harness_steps.py:276
              | check                                   |
              | artifact existence                      |
              | lint clean                              |
              | tasks all done                          |
              | valid commits present                   |
              | archive succeeded                       |
              | permanent specs are clean Gherkin       |
              | step definition files are non-empty     |
              | REVIEW: PASS marker present             |
              | DESIGN-REVIEW: PASS marker present      |
              | verify.command exits zero after archive |
            And each check has a pass or fail result            # tests/features/steps/harness/test_harness_steps.py:298

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        4 steps passed, 0 failed, 123 skipped
        Took 0min 0.024s
        ```
        Files modified: `src/givn_harness/evaluator.py`, `src/givn_harness/types.py`, `src/givn_harness/report.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Report includes all ten automated checks    # tests/features/harness/test-harness.feature:89
            Given a completed test run at "reports/run-001/"    # tests/features/steps/harness/test_harness_steps.py:175
            When the harness evaluates the workspace            # tests/features/steps/harness/test_harness_steps.py:267
            Then the run report lists all ten automated checks: # tests/features/steps/harness/test_harness_steps.py:276
              | check                                   |
              | artifact existence                      |
              ...
            And each check has a pass or fail result            # tests/features/steps/harness/test_harness_steps.py:298

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        4 steps passed, 0 failed, 123 skipped
        Took 0min 0.023s
        ```
      COMMIT: `0f8e425` — feat(test-harness): Report includes all ten automated checks

- [x] test-harness: LLM-as-judge scores each artifact on a 0-4 rubric
      Design constraints:
        - `judge.py` scores 5 artifacts (proposal, specs, design, tasks, review) on 0-4 scale.
        - Judge prompt delivered as a file, passed to opencode (single-turn, not ralph loop).
        - Each score has a written explanation.
        - `JudgeScore` dataclass in `types.py`.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: LLM-as-judge scores each artifact on a 0-4 rubric     # tests/features/harness/test-harness.feature:107
            Given a completed test run at "reports/run-001/"              # tests/features/steps/harness/test_harness_steps.py:175
            When the harness evaluates the workspace                      # tests/features/steps/harness/test_harness_steps.py:267
            Then an LLM judge scores the proposal artifact on a 0-4 scale # tests/features/steps/harness/test_harness_steps.py:306
              Traceback (most recent call last):
                ...
              NotImplementedError: TODO

            And an LLM judge scores the specs artifact on a 0-4 scale     # None
            ...

        Errored scenarios:
          tests/features/harness/test-harness.feature:107  LLM-as-judge scores each artifact on a 0-4 rubric

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        2 steps passed, 0 failed, 1 error, 124 skipped
        Took 0min 0.022s
        ```
      GREEN:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        8 steps passed, 0 failed, 119 skipped
        Took 0min 0.044s
        ```
        Files modified: `src/givn_harness/judge.py`, `src/givn_harness/types.py`, `src/givn_harness/report.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        8 steps passed, 0 failed, 119 skipped
        Took 0min 0.032s
        ```
        Files created: `src/givn_harness/_utils.py` (extracted `find_change_dir` shared between evaluator and judge)
        Files modified: `src/givn_harness/evaluator.py` (use shared `find_change_dir`)
      COMMIT: `1f90036` — feat(test-harness): LLM-as-judge scores each artifact on a 0-4 rubric

- [x] test-harness: Report is written as a markdown file
      Design constraints:
        - `report.py` writes `reports/<run-id>/report.md`.
        - Markdown structure: Run metadata, Automated Checks table, LLM Judge Scores table, Agent Log section, Workspace path.
        - Report format from design.md entry 392-427.
      RED:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Report is written as a markdown file           # tests/features/harness/test-harness.feature:118
            Given a completed test run at "reports/run-001/"       # tests/features/steps/harness/test_harness_steps.py:175
            When the harness evaluates the workspace               # tests/features/steps/harness/test_harness_steps.py:267
            Then a report file "reports/run-001/report.md" exists  # tests/features/steps/harness/test_harness_steps.py:357
              Traceback (most recent call last):
                ...
              NotImplementedError: TODO

            And the report contains a section for automated checks # None
            And the report contains a section for LLM judge scores # None
            And the report contains the workspace path             # None

        Errored scenarios:
          tests/features/harness/test-harness.feature:118  Report is written as a markdown file

        0 features passed, 0 failed, 1 error, 0 skipped
        0 scenarios passed, 0 failed, 1 error, 24 skipped
        2 steps passed, 0 failed, 1 error, 124 skipped
        Took 0min 0.029s
        ```
      GREEN:
        ```
        USING RUNNER: behave.runner:Runner
        USE_NESTED_STEP_MODULES: yes
        @givn.delta @test-harness
        Feature: test-harness # tests/features/harness/test-harness.feature:10

          @givn.added
          Scenario: Report is written as a markdown file           # tests/features/harness/test-harness.feature:118
            Given a completed test run at "reports/run-001/"       # tests/features/steps/harness/test_harness_steps.py:175
            When the harness evaluates the workspace               # tests/features/steps/harness/test_harness_steps.py:267
            Then a report file "reports/run-001/report.md" exists  # tests/features/steps/harness/test_harness_steps.py:363
            And the report contains a section for automated checks # tests/features/steps/harness/test_harness_steps.py:371
            And the report contains a section for LLM judge scores # tests/features/steps/harness/test_harness_steps.py:379
            And the report contains the workspace path             # tests/features/steps/harness/test_harness_steps.py:387

        1 feature passed, 0 failed, 0 skipped
        1 scenario passed, 0 failed, 24 skipped
        6 steps passed, 0 failed, 121 skipped
        Took 0min 0.032s
        ```
        Files modified: `src/givn_harness/report.py`, `tests/features/steps/harness/test_harness_steps.py`, `tests/features/harness/test-harness.feature`, `givn/changes/givn-test-harness/specs/test-harness/test-harness.feature`
      REFACTOR:
        ```
        1 feature passed, 0 failed, 0 skipped
        8 scenarios passed, 0 failed, 17 skipped
        37 steps passed, 0 failed, 90 skipped
        Took 0min 0.417s
        ```
      COMMIT: `751ed40` — feat(test-harness): Report is written as a markdown file

- [ ] test-harness: Cross-run index accumulates results
      Design constraints:
        - `index.py` reads/writes `reports/index.json` as a JSON array of summaries.
        - Each entry: run_id, task, timestamp, overall_pass, check_pass_count, judge_avg_score, report_path.
        - New run appends entry; existing entries preserved.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement `index.py`. Files: `index.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Cross-run index accumulates results". Record hash.

- [ ] test-harness: Report handles a run where the agent produced no artifacts
      Design constraints:
        - When agent produces zero artifacts (crash before givn init), all 10 checks show FAIL with detail explaining what was expected vs. found.
        - Judge scores default to 0 for all artifacts (not missing — explicit 0).
        - Termination reason recorded as the actual cause (timeout/error/crash).
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Extend `evaluator.py` and `judge.py` for zero-artifact resilience. Files: `evaluator.py`, `judge.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Report handles a run where the agent produced no artifacts". Record hash.

## Phase 3 — Error handling and resilience

- [ ] test-harness: Workspace is preserved when a run fails
      Design constraints:
        - On agent failure (archive never completed), workspace dir is not deleted.
        - Report includes the path to the preserved workspace.
        - Cleanup only on successful runs; preserve on any non-zero exit or exception.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Modify workspace lifecycle: defer cleanup to explicit success path. Files: `workspace.py`, `cli.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Workspace is preserved when a run fails". Record hash.

- [ ] test-harness: Sandlock is not available, harness falls back to --no-sandbox
      Design constraints:
        - `sandbox.py` attempts `import sandlock`. If ImportError: log warning, set `--no-sandbox` mode.
        - Agent runs without sandbox confinement (straight subprocess, no sandlock).
        - Report notes that the run was unsandboxed.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement `sandbox.py` with import-time fallback. Files: `sandbox.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Sandlock is not available, harness falls back to --no-sandbox". Record hash.

- [ ] test-harness: Ralph binary is not found at the configured path
      Design constraints:
        - `runner.py` checks for ralph binary at `--ralph-bin` (default `/usr/local/bin/ralph`). If not found: non-zero exit, stderr message.
        - Check happens before any workspace creation.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement `runner.py` binary check. Files: `runner.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Ralph binary is not found at the configured path". Record hash.

- [ ] test-harness: LLM judge returns unparseable JSON, score defaults to zero
      Design constraints:
        - `judge.py` parses LLM output as JSON. On parse failure: score=0, explanation="judge output unparseable".
        - Does not crash the harness on unparseable output.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Add JSON parse error handling in `judge.py`. Files: `judge.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): LLM judge returns unparseable JSON, score defaults to zero". Record hash.

- [ ] test-harness: Interrupted run preserves workspace on SIGINT
      Design constraints:
        - SIGINT handler catches KeyboardInterrupt, does NOT delete workspace.
        - Report records the interruption as termination reason.
        - Workspace preservation logic from prior scenario applies.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Add SIGINT handler in `cli.py`. Files: `cli.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Interrupted run preserves workspace on SIGINT". Record hash.

## Phase 4 — Agent execution and sandbox integration

- [ ] test-harness: Agent execution respects time limit
      Design constraints:
        - `--timeout <seconds>` is passed to the ralph subprocess or monitored via a background thread.
        - Report records termination reason as "timeout" when hit.
        - `runner.py` is responsible for subprocess lifecycle.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement timeout enforcement in `runner.py`. Files: `runner.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Agent execution respects time limit". Record hash.

- [ ] test-harness: Agent execution respects maximum iteration limit
      Design constraints:
        - `--max-iterations <N>` is passed to ralph as `--max-iterations N`.
        - ralph enforces the iteration cap internally; harness passes the flag.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Pass `--max-iterations` flag to ralph in `runner.py`. Files: `runner.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Agent execution respects maximum iteration limit". Record hash.

- [ ] test-harness: ralph is invoked with --prompt-file pointing to the workspace task file
      Design constraints:
        - `runner.py` constructs ralph command with `--prompt-file <workspace>/task.md`.
        - ralph is invoked as a subprocess; stdout/stderr captured for the report.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement full ralph invocation in `runner.py`. Files: `runner.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): ralph is invoked with --prompt-file pointing to the workspace task file". Record hash.

- [ ] test-harness: The agent runs inside a sandlock sandbox
      Design constraints:
        - `sandbox.py` constructs a `sandlock.Sandbox` with: workdir=workspace, cwd=workspace, fs_writable=[workspace, /tmp, ~/.npm, ~/.cache], fs_readable=[/usr, /bin, /lib, ...], clean_env=False, max_processes=256, max_memory=16G, max_disk=20G, port_remap=True, on_exit=COMMIT.
        - Ralph subprocess runs inside the sandbox.
        - Full sandlock config from design.md entry 309-333.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement Sandbox construction in `sandbox.py`. Integrate with `runner.py`. Files: `sandbox.py`, `runner.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): The agent runs inside a sandlock sandbox". Record hash.

- [ ] test-harness: Sandbox filesystem uses learn-then-confine mode
      Design constraints:
        - First run with `--profile <name>`: sandlock runs in audit mode, records all path accesses into `profiles/<name>.toml`.
        - Second run with same profile: sandlock enforces learned profile with PathAllowHandler.
        - Harness detects whether profile file exists and switches mode.
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Implement profile detection and mode switching in `sandbox.py`. Files: `sandbox.py`. Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Sandbox filesystem uses learn-then-confine mode". Record hash.

- [ ] test-harness: Sandbox network is default-allow with no outbound restrictions
      Design constraints:
        - Sandbox does not specify `net_allow` or `net_deny` — default-allow networking.
        - Docker commands and Playwright browser launches must succeed inside sandbox.
        - This scenario validates sandlock default-allow behavior (not harness logic).
      RED:      Remove @wip. Stubs. Run targeting this scenario → MUST FAIL. Paste output.
      GREEN:    Assert no network restrictions in sandbox config. Files: `sandbox.py` or `test_harness_steps.py` (step only). Run → PASSES. Paste output.
      REFACTOR: Clean up. Re-run → PASSES. Paste output.
      COMMIT:   One commit. Message: "feat(test-harness): Sandbox network is default-allow with no outbound restrictions". Record hash.

## Phase 5 — E2E setup and scenarios

- [ ] Setup: configure e2e test infrastructure, strict mode, and verify.e2e_command
      - E2e step definitions share `tests/features/harness/steps/test_harness_steps.py` (same file as non-@e2e — the harness CLI IS the real interface).
      - PROVE strict mode for e2e runner: run `uv run behave tests/features/ --no-capture --tags=@e2e -n "PROOF"` with a stub step. Confirm NON-ZERO exit.
      - Set `verify.e2e_command` in `harness/givn/config.yaml` to `uv run behave tests/features/ --no-capture --tags=@e2e`.
      - PROVE ISOLATION: run `uv run behave tests/features/ --no-capture` → record scenario count. Run `uv run behave tests/features/ --no-capture --tags=@e2e` → record scenario count. E2E count MUST be strictly less than full count. Paste both counts.
      - Run: `uv run behave tests/features/ --no-capture --tags=@e2e` → must exit 0.

- [ ] @e2e test-harness: User runs the harness end-to-end and observes a complete autoresearch loop
      Design constraints:
        - Invokes `givn-harness run --task tasks/shopping-list.md --output reports/ --profile shopping-list`.
        - Real ralph drives opencode with LLM through full givn lifecycle (propose → archive).
        - Verifies: workspace created, git/givn init, ralph invocations, report.md created, index.json updated.
        - stdout contains workspace creation message, git/init progress, ralph iteration progress, evaluation progress, judge progress.
      RED:      Remove @wip. Stubs. Run `uv run behave tests/features/ --no-capture --tags=@e2e -n "User runs the harness end-to-end and observes a complete autoresearch loop"` → MUST FAIL. Paste evidence.
      GREEN:    Set up e2e infrastructure. Replace stubs with real assertions driving the CLI. List files touched. Run → PASSES. Paste evidence.
      REFACTOR: Clean up. Re-run → PASSES. Paste evidence.
      COMMIT:   One commit. Message: "test(e2e): User runs the harness end-to-end and observes a complete autoresearch loop". Record hash.

- [ ] @e2e test-harness: User reads a per-run report after a completed run
      Design constraints:
        - Reads `reports/run-001/report.md`.
        - Verifies sections: Automated Checks (10 checks with pass/fail), LLM Judge Scores (5 artifacts with 0-4 scores + explanations), Workspace path, Agent Log.
        - Asserts through file content, not live run (the report already exists from a prior run).
      RED:      Remove @wip. Stubs. Run `uv run behave tests/features/ --no-capture --tags=@e2e -n "User reads a per-run report after a completed run"` → MUST FAIL. Paste evidence.
      GREEN:    Assert on report file content. List files touched. Run → PASSES. Paste evidence.
      REFACTOR: Clean up. Re-run → PASSES. Paste evidence.
      COMMIT:   One commit. Message: "test(e2e): User reads a per-run report after a completed run". Record hash.

- [ ] @e2e test-harness: User inspects the cross-run index after multiple runs
      Design constraints:
        - Reads `reports/index.json`.
        - Verifies: valid JSON array, at least 2 entries, each has run_id, task, timestamp, overall_pass fields.
      RED:      Remove @wip. Stubs. Run `uv run behave tests/features/ --no-capture --tags=@e2e -n "User inspects the cross-run index after multiple runs"` → MUST FAIL. Paste evidence.
      GREEN:    Assert on index file content. List files touched. Run → PASSES. Paste evidence.
      REFACTOR: Clean up. Re-run → PASSES. Paste evidence.
      COMMIT:   One commit. Message: "test(e2e): User inspects the cross-run index after multiple runs". Record hash.

- [ ] @e2e test-harness: User inspects a workspace from a failed run
      Design constraints:
        - Workspace preserved at `/tmp/givn-harness-workspaces/run-002`.
        - Verifies: task.md exists, givn/ directory with partial agent progress, .git directory with agent commits.
        - Asserts through filesystem inspection (os.listdir), not live run.
      RED:      Remove @wip. Stubs. Run `uv run behave tests/features/ --no-capture --tags=@e2e -n "User inspects a workspace from a failed run"` → MUST FAIL. Paste evidence.
      GREEN:    Assert on workspace directory content. List files touched. Run → PASSES. Paste evidence.
      REFACTOR: Clean up. Re-run → PASSES. Paste evidence.
      COMMIT:   One commit. Message: "test(e2e): User inspects a workspace from a failed run". Record hash.
