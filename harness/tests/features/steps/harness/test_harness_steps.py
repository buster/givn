import os
import subprocess
import tempfile
from pathlib import Path

from behave import given, when, then


REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent.parent


@given('a workspace directory at "/tmp/givn-harness-workspaces"')
def step_workspace_directory(context):
    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write("task content")
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context._cleanup_paths = [context.task_file]
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))


@when('the user runs `givn-harness run --task task.md --output reports/`')
def step_user_runs_harness(context):
    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
    ]
    ralph_bin = getattr(context, "ralph_bin", None)
    if ralph_bin is not None:
        cmd.extend(["--ralph-bin", ralph_bin])
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip()


@then('the workspace is created in a temporary directory under the configured workspace root')
def step_workspace_created_in_temp_dir(context):
    assert os.path.isdir(context.workspace_path), (
        f"workspace path {context.workspace_path} does not exist"
    )
    assert context.workspace_path.startswith(context.workspace_root), (
        f"workspace {context.workspace_path} is not under {context.workspace_root}"
    )
    assert os.path.basename(context.workspace_path) != "", (
        "workspace path appears to be the root itself"
    )


@then('the workspace directory is not within the givn repository')
def step_workspace_not_within_repo(context):
    repo_resolved = REPO_ROOT.resolve()
    workspace_resolved = Path(context.workspace_path).resolve()
    try:
        workspace_resolved.relative_to(repo_resolved)
        assert False, (
            f"workspace {workspace_resolved} is inside the givn repository {repo_resolved}"
        )
    except ValueError:
        pass


@then('the workspace contains an initialized git repository')
def step_workspace_contains_git_repo(context):
    git_dir = os.path.join(context.workspace_path, ".git")
    assert os.path.isdir(git_dir), (
        f"expected .git directory at {git_dir}, not found"
    )


@then('the workspace contains a givn/ directory scaffolded by givn init')
def step_workspace_contains_givn_dir(context):
    givn_dir = os.path.join(context.workspace_path, "givn")
    assert os.path.isdir(givn_dir), (
        f"expected givn/ directory at {givn_dir}, not found"
    )


@then('the workspace contains givn/config.yaml')
def step_workspace_contains_givn_config_yaml(context):
    config_yaml = os.path.join(context.workspace_path, "givn", "config.yaml")
    assert os.path.isfile(config_yaml), (
        f"expected givn/config.yaml at {config_yaml}, not found"
    )


def _rmtree(path: str) -> None:
    import shutil
    if os.path.isdir(path):
        shutil.rmtree(path, ignore_errors=True)


@given('a task prompt file at "{path}" with a task description')
def step_task_prompt_file(context, path):
    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    context.task_content = "# Shopping List\n\nBuild a shopping list web application."
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write(context.task_content)
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))


@when('the user runs `givn-harness run --task nonexistent.md --output reports/`')
def step_user_runs_harness_nonexistent_task(context):
    result = subprocess.run(
        [
            "uv", "run", "givn-harness", "run",
            "--task", "nonexistent.md",
            "--output", os.path.join(context.output_dir, "reports"),
            "--workspace-root", context.workspace_root,
        ],
        capture_output=True,
        text=True,
    )
    context.cli_result = result


@then('the command exits non-zero')
def step_command_exits_non_zero(context):
    assert context.cli_result.returncode != 0, (
        f"expected non-zero exit, got {context.cli_result.returncode}"
    )


@then('stderr reports that the task file does not exist')
def step_stderr_reports_task_file_not_exist(context):
    stderr = context.cli_result.stderr
    assert "does not exist" in stderr, (
        f"expected stderr to report missing file, got: {stderr!r}"
    )
@when('the user runs `givn-harness run --task tasks/shopping-list.md --output reports/`')
def step_user_runs_harness_shopping_list(context):
    result = subprocess.run(
        [
            "uv", "run", "givn-harness", "run",
            "--task", context.task_file,
            "--output", os.path.join(context.output_dir, "reports"),
            "--workspace-root", context.workspace_root,
        ],
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip()


@then('the workspace contains a file "{path}" with the task description content')
def step_workspace_contains_task_file(context, path):
    file_path = os.path.join(context.workspace_path, path)
    assert os.path.isfile(file_path), f"expected file {file_path} not found"
    with open(file_path) as f:
        content = f.read()
    assert content == context.task_content, (
        f"task.md content mismatch:\nexpected: {context.task_content!r}\nactual:   {content!r}"
    )


@then('the workspace\'s givn/config.yaml contains the verify.command sentinel "givn missing-testrunner"')
def step_config_yaml_contains_sentinel(context):
    config_path = os.path.join(context.workspace_path, "givn", "config.yaml")
    with open(config_path) as f:
        content = f.read()
    assert '"givn missing-testrunner"' in content, (
        f'config.yaml does not contain the verify.command sentinel "givn missing-testrunner"'
    )


@given('a completed test run at "{path}"')
def step_completed_test_run(context, path):
    import shutil
    from pathlib import Path

    context.workspace_root = tempfile.mkdtemp(prefix="givn-harness-test-")
    context.add_cleanup(lambda: shutil.rmtree(context.workspace_root, ignore_errors=True))

    ws = Path(context.workspace_root)
    change_dir = ws / "givn" / "changes" / "test-change"
    specs_cd = change_dir / "specs" / "test"
    specs_cd.mkdir(parents=True)
    archive_dir = ws / "givn" / "archive" / "test-change"
    archive_specs = archive_dir / "specs" / "test"
    archive_specs.mkdir(parents=True)
    permanent_specs = ws / "givn" / "specs" / "test"
    permanent_specs.mkdir(parents=True)
    steps_dir = ws / "tests" / "features" / "steps"
    steps_dir.mkdir(parents=True)

    (change_dir / "proposal.md").write_text("# Proposal\n\nThis is a test proposal.\n")
    (change_dir / "design.md").write_text("# Design\n\nThis is a test design.\n")
    (change_dir / "design-review.md").write_text("# Design Review\n\nDESIGN-REVIEW: PASS\n")
    (change_dir / "tasks.md").write_text("# Tasks\n\n- [x] task one\n- [x] task two\n")
    (change_dir / "review.md").write_text("# Review\n\nREVIEW: PASS\n")

    feature_content = (
        "Feature: Test\n"
        "  Scenario: A test\n"
        "    Given something\n"
        "    When something\n"
        "    Then something\n"
    )
    (specs_cd / "test.feature").write_text(feature_content)
    (archive_specs / "archived.feature").write_text(feature_content)
    (permanent_specs / "permanent.feature").write_text(feature_content)

    (archive_dir / "proposal.md").write_text("# Proposal\n\nArchived proposal.\n")
    (archive_dir / "design.md").write_text("# Design\n\nArchived design.\n")

    (steps_dir / "test_steps.py").write_text(
        "from behave import given, when, then\n"
        "\n"
        "@given('something')\n"
        "def step_given_something(context):\n"
        '    context.x = 1\n'
        "\n"
        "@when('something')\n"
        "def step_when_something(context):\n"
        '    context.x += 1\n'
        "\n"
        "@then('something')\n"
        "def step_then_something(context):\n"
        "    assert context.x == 2\n"
    )

    config_dir = ws / "givn"
    config_dir.mkdir(parents=True, exist_ok=True)
    (config_dir / "config.yaml").write_text(
        'verify:\n  command: "true"\n'
        'givn_config_version: "0.2.0"\n'
    )

    subprocess.run(
        ["git", "init"],
        cwd=str(ws),
        capture_output=True,
    )
    subprocess.run(
        ["git", "config", "user.email", "test@example.com"],
        cwd=str(ws),
        capture_output=True,
    )
    subprocess.run(
        ["git", "config", "user.name", "Test User"],
        cwd=str(ws),
        capture_output=True,
    )
    subprocess.run(
        ["git", "add", "-A"],
        cwd=str(ws),
        capture_output=True,
    )
    subprocess.run(
        ["git", "commit", "-m", "feat(test): initial scaffold"],
        cwd=str(ws),
        capture_output=True,
    )

    context.workspace_path = str(ws)


@when("the harness evaluates the workspace")
def step_harness_evaluates_workspace(context):
    from unittest.mock import MagicMock, patch

    from givn_harness.evaluator import run_evaluation
    from givn_harness.judge import run_judge
    from givn_harness.report import generate_report_structure, write_report

    context.checks = run_evaluation(context.workspace_path)

    invalid_artifacts = getattr(context, "_judge_invalid_json_artifacts", set())

    def mock_run(args, **kwargs):
        prompt_file = args[2] if len(args) > 2 else None
        if prompt_file and Path(prompt_file).is_file():
            prompt_text = Path(prompt_file).read_text()
            artifact_type = None
            for at in ["proposal", "specs", "design", "tasks", "review"]:
                if f"Artifact type: {at}" in prompt_text:
                    artifact_type = at
                    break
            if artifact_type and artifact_type in invalid_artifacts:
                mock_result = MagicMock()
                mock_result.stdout = "not valid json {{{"
                mock_result.stderr = ""
                mock_result.returncode = 0
                return mock_result
        mock_result = MagicMock()
        mock_result.stdout = '{"score": 3, "explanation": "Good work."}'
        mock_result.stderr = ""
        mock_result.returncode = 0
        return mock_result

    with patch("givn_harness.judge.subprocess.run", side_effect=mock_run):
        context.judge_scores = run_judge(context.workspace_path)

    context.report = generate_report_structure(
        context.checks, context.workspace_path, context.judge_scores
    )

    context.report_path = write_report(
        context.checks,
        context.workspace_path,
        judge_scores=context.judge_scores,
    )


@then("the run report lists all ten automated checks:")
def step_report_lists_all_ten_checks(context):
    expected_names = {
        "artifact existence",
        "lint clean",
        "tasks all done",
        "valid commits present",
        "archive succeeded",
        "permanent specs are clean Gherkin",
        "step definition files are non-empty",
        "REVIEW: PASS marker present",
        "DESIGN-REVIEW: PASS marker present",
        "verify.command exits zero after archive",
    }
    actual_names = {c.name for c in context.checks}
    missing = expected_names - actual_names
    assert not missing, f"missing checks: {missing}"
    assert len(context.checks) == 10, (
        f"expected 10 checks, got {len(context.checks)}"
    )


@then("each check has a pass or fail result")
def step_each_check_has_pass_or_fail(context):
    for c in context.checks:
        assert isinstance(c.passed, bool), (
            f"check '{c.name}' passed is not bool: {c.passed!r}"
        )


@then("an LLM judge scores the proposal artifact on a 0-4 scale")
def step_judge_scores_proposal(context):
    _check_judge_score(context, "proposal")


@then("an LLM judge scores the specs artifact on a 0-4 scale")
def step_judge_scores_specs(context):
    _check_judge_score(context, "specs")


@then("an LLM judge scores the design artifact on a 0-4 scale")
def step_judge_scores_design(context):
    _check_judge_score(context, "design")


@then("an LLM judge scores the tasks artifact on a 0-4 scale")
def step_judge_scores_tasks(context):
    _check_judge_score(context, "tasks")


@then("an LLM judge scores the review artifact on a 0-4 scale")
def step_judge_scores_review(context):
    _check_judge_score(context, "review")


@then("each score is accompanied by a written explanation")
def step_each_score_has_explanation(context):
    for score in context.judge_scores:
        assert score.explanation, (
            f"missing explanation for {score.artifact}"
        )
        assert isinstance(score.explanation, str), (
            f"explanation is not a string for {score.artifact}"
        )


@then('a report file "{path}" exists')
def step_report_file_exists(context, path):
    report_path = context.report_path
    assert report_path.is_file(), (
        f"report file {report_path} does not exist"
    )


@then("the report contains a section for automated checks")
def step_report_contains_automated_checks_section(context):
    content = context.report_path.read_text()
    assert "## Automated Checks" in content, (
        "report does not contain '## Automated Checks' section"
    )


@then("the report contains a section for LLM judge scores")
def step_report_contains_judge_scores_section(context):
    content = context.report_path.read_text()
    assert "## LLM Judge Scores" in content, (
        "report does not contain '## LLM Judge Scores' section"
    )


@then("the report contains the workspace path")
def step_report_contains_workspace_path(context):
    content = context.report_path.read_text()
    assert context.workspace_path in content, (
        f"report does not contain workspace path {context.workspace_path}"
    )


@when("the agent fails to archive the change")
def step_agent_fails_to_archive(context):
    result = subprocess.run(
        [
            "uv", "run", "givn-harness", "run",
            "--task", context.task_file,
            "--output", os.path.join(context.output_dir, "reports"),
            "--workspace-root", context.workspace_root,
            "--fail",
        ],
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip()


@then("the workspace directory is not deleted")
def step_workspace_not_deleted(context):
    assert os.path.isdir(context.workspace_path), (
        f"workspace {context.workspace_path} was deleted"
    )


@then("the report includes the path to the preserved workspace")
def step_report_includes_preserved_workspace(context):
    report_path = os.path.join(context.workspace_path, "report.md")
    assert os.path.isfile(report_path), (
        f"report file {report_path} does not exist"
    )
    content = Path(report_path).read_text()
    assert context.workspace_path in content, (
        f"report does not contain workspace path {context.workspace_path}"
    )


@when("the agent crashes before producing any artifacts")
def step_agent_crashes_before_artifacts(context):
    from unittest.mock import MagicMock, patch

    from givn_harness.evaluator import run_evaluation
    from givn_harness.judge import run_judge
    from givn_harness.report import write_report

    context.checks = run_evaluation(context.workspace_path)

    mock_response = '{"score": 3, "explanation": "Good work."}'
    mock_result = MagicMock()
    mock_result.stdout = mock_response
    mock_result.stderr = ""
    mock_result.returncode = 0

    with patch("givn_harness.judge.subprocess.run", return_value=mock_result):
        context.judge_scores = run_judge(context.workspace_path)

    context.report_path = write_report(
        context.checks,
        context.workspace_path,
        judge_scores=context.judge_scores,
        termination="crash before artifacts",
    )


@then("all ten automated checks report FAIL with detail")
def step_all_checks_fail_with_detail(context):
    assert len(context.checks) == 10, (
        f"expected 10 checks, got {len(context.checks)}"
    )
    for c in context.checks:
        assert not c.passed, (
            f"check '{c.name}' should have failed but passed"
        )
        assert c.detail, (
            f"check '{c.name}' has no detail"
        )


@then("the LLM judge scores default to 0")
def step_judge_scores_default_to_zero(context):
    assert len(context.judge_scores) == 5, (
        f"expected 5 judge scores, got {len(context.judge_scores)}"
    )
    for s in context.judge_scores:
        assert s.score == 0, (
            f"judge score for {s.artifact} is {s.score}, expected 0"
        )


@then("the termination reason is recorded in the report")
def step_termination_reason_in_report(context):
    content = context.report_path.read_text()
    assert "crash before artifacts" in content, (
        "report does not contain termination reason 'crash before artifacts'"
    )


@given('an existing cross-run index at "{path}" with one prior entry')
def step_existing_cross_run_index(context, path):
    import json

    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write("task content")
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))

    reports_dir = os.path.join(context.output_dir, "reports")
    os.makedirs(reports_dir, exist_ok=True)
    index_path = os.path.join(reports_dir, "index.json")
    existing_entry = {
        "run_id": "prior-run-001",
        "task": "prior-task.md",
        "timestamp": "2026-07-27T00:00:00Z",
        "overall_pass": True,
        "check_pass_count": 10,
        "judge_avg_score": 3.0,
        "report_path": "reports/prior-run-001/report.md",
    }
    with open(index_path, "w") as f:
        json.dump([existing_entry], f)
    context.index_path = index_path


@then('the index file at "{path}" contains two entries')
def step_index_file_contains_two_entries(context, path):
    import json

    index_path = os.path.join(context.output_dir, "reports", "index.json")
    assert os.path.isfile(index_path), f"index file {index_path} does not exist"
    with open(index_path) as f:
        data = json.load(f)
    assert isinstance(data, list), "index is not a JSON array"
    assert len(data) == 2, f"expected 2 entries, got {len(data)}"
    context.index_data = data


@then("the new entry includes the run identifier, task name, timestamp, and overall pass/fail")
def step_new_entry_has_fields(context):
    new_entry = context.index_data[-1]
    assert "run_id" in new_entry, "new entry missing run_id"
    assert "task" in new_entry, "new entry missing task"
    assert "timestamp" in new_entry, "new entry missing timestamp"
    assert "overall_pass" in new_entry, "new entry missing overall_pass"
    assert isinstance(new_entry["overall_pass"], bool), "overall_pass is not a bool"


@given("the sandlock SDK is not importable")
def step_sandlock_not_importable(context):
    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write("task content")
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context._cleanup_paths = [context.task_file]
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))
    os.environ["GIVN_HARNESS_NO_SANDLOCK"] = "1"
    context.add_cleanup(lambda: os.environ.pop("GIVN_HARNESS_NO_SANDLOCK", None))


@then("the harness logs a warning that sandlock is unavailable")
def step_harness_sandlock_warning(context):
    stderr = context.cli_result.stderr
    assert "sandlock" in stderr.lower(), (
        f"expected stderr to mention sandlock, got: {stderr!r}"
    )


@then("the harness runs the agent without sandbox confinement")
def step_harness_runs_without_sandbox(context):
    assert os.path.isdir(context.workspace_path), (
        f"workspace {context.workspace_path} does not exist"
    )
    assert context.cli_result.returncode == 0, (
        f"expected exit 0, got {context.cli_result.returncode}"
    )


@then("the report notes that the run was unsandboxed")
def step_report_notes_unsandboxed(context):
    report_path = os.path.join(context.workspace_path, "report.md")
    assert os.path.isfile(report_path), (
        f"report file {report_path} does not exist"
    )
    content = Path(report_path).read_text()
    assert "**Sandboxed:** No" in content or "Sandboxed: No" in content, (
        f"report does not indicate unsandboxed run: {content!r}"
    )


def _check_judge_score(context, artifact_name):
    scores = context.judge_scores
    matching = [s for s in scores if s.artifact == artifact_name]
    assert len(matching) == 1, (
        f"expected 1 score for {artifact_name}, got {len(matching)}"
    )
    score = matching[0]
    assert 0 <= score.score <= 4, (
        f"{artifact_name} score {score.score} not in 0-4"
    )


@given('the ralph binary is not present at "{path}"')
def step_ralph_binary_not_present(context, path):
    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write("task content")
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context._cleanup_paths = [context.task_file]
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))
    context.ralph_bin = "/tmp/nonexistent-ralph-xyz"


@then("the harness exits with a non-zero exit code")
def step_harness_exits_non_zero(context):
    assert context.cli_result.returncode != 0, (
        f"expected non-zero exit, got {context.cli_result.returncode}"
    )


@then("stderr reports that the ralph binary was not found")
def step_stderr_reports_ralph_not_found(context):
    stderr = context.cli_result.stderr
    assert "ralph binary not found" in stderr, (
        f"expected stderr to report ralph binary not found, got: {stderr!r}"
    )


@given("the LLM judge returns invalid JSON for the proposal artifact")
def step_judge_returns_invalid_json(context):
    context._judge_invalid_json_artifacts = {"proposal"}


@then("the proposal artifact judge score is 0")
def step_proposal_score_is_zero(context):
    scores = context.judge_scores
    matching = [s for s in scores if s.artifact == "proposal"]
    assert len(matching) == 1, (
        f"expected 1 score for proposal, got {len(matching)}"
    )
    assert matching[0].score == 0, (
        f"expected proposal score 0, got {matching[0].score}"
    )


@then('the explanation is "judge output unparseable"')
def step_explanation_is_unparseable(context):
    scores = context.judge_scores
    matching = [s for s in scores if s.artifact == "proposal"]
    assert len(matching) == 1
    assert matching[0].explanation == "judge output unparseable", (
        f"expected explanation 'judge output unparseable', got {matching[0].explanation!r}"
    )


@when("the process receives SIGINT")
def step_process_receives_sigint(context):
    import signal
    import time

    python = os.path.join(REPO_ROOT, ".venv", "bin", "givn-harness")
    cmd = [
        python, "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
    ]
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    time.sleep(1.0)
    proc.send_signal(signal.SIGINT)
    stdout, stderr = proc.communicate(timeout=5)
    result = subprocess.CompletedProcess(args=cmd, returncode=proc.returncode, stdout=stdout, stderr=stderr)
    context.cli_result = result
    context.workspace_path = stdout.strip()


@then("the report records the interruption as the termination reason")
def step_report_records_interruption_termination(context):
    report_path = Path(context.workspace_path) / "report.md"
    assert report_path.is_file(), f"report file {report_path} does not exist"
    content = report_path.read_text()
    assert "**Termination:** interrupted" in content or "Termination: interrupted" in content, (
        f"report does not record interruption as termination reason: {content!r}"
    )


@when('the user runs `givn-harness run --task task.md --output reports/ --max-iterations 5`')
def step_user_runs_harness_with_max_iterations(context):
    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
        "--max-iterations", "5",
    ]
    ralph_bin = getattr(context, "ralph_bin", None)
    if ralph_bin is not None:
        cmd.extend(["--ralph-bin", ralph_bin])
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip()


@then("ralph is invoked with --max-iterations 5")
def step_ralph_invoked_with_max_iterations(context):
    from unittest.mock import patch

    from givn_harness.runner import run_agent

    with patch("givn_harness.runner.subprocess.run") as mock_run:
        run_agent(context.workspace_path, max_iterations=5)

    mock_run.assert_called_once()
    args, _kwargs = mock_run.call_args
    cmd = args[0]
    assert "--max-iterations" in cmd, f"expected --max-iterations in command, got {cmd}"
    max_iter_index = cmd.index("--max-iterations")
    value_index = cmd.index("5")
    assert value_index == max_iter_index + 1, (
        f"expected --max-iterations followed by 5, got {cmd}"
    )


@when('the user runs `givn-harness run --task task.md --output reports/ --timeout 60`')
def step_user_runs_harness_with_timeout(context):
    timeout = getattr(context, "timeout", 2)
    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
        "--timeout", str(timeout),
    ]
    ralph_bin = getattr(context, "ralph_bin", None)
    if ralph_bin is not None:
        cmd.extend(["--ralph-bin", ralph_bin])
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=timeout + 5,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip()


@then("the harness stops agent execution after 60 seconds")
def step_harness_stops_agent_after_timeout(context):
    assert context.cli_result.returncode == 0, (
        f"expected exit 0, got {context.cli_result.returncode}"
    )


@then("the report records the timeout as the termination reason")
def step_report_records_timeout_termination(context):
    report_path = Path(context.workspace_path) / "report.md"
    assert report_path.is_file(), f"report file {report_path} does not exist"
    content = report_path.read_text()
    assert "**Termination:** timeout" in content or "Termination: timeout" in content, (
        f"report does not record timeout as termination reason: {content!r}"
    )