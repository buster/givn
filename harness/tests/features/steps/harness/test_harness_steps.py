import json
import os
import subprocess
import tempfile
from pathlib import Path

from behave import given, when, then


REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent.parent


def _generate_e2e_report(report_path: Path, workspace_path: Path) -> None:
    from datetime import datetime, timezone

    from givn_harness.types import Check, JudgeScore

    checks = [
        Check(name=f"check_{i}", passed=i != 3, detail=f"check_{i} {'passed' if i != 3 else 'failed'}")
        for i in range(10)
    ]
    judge_scores = [
        JudgeScore(artifact=name, score=i, explanation=f"{name} scored {i}")
        for i, name in enumerate(["proposal", "design", "tasks", "review", "specs"])
    ]

    run_id = workspace_path.name
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    all_pass = all(c.passed for c in checks)

    lines: list[str] = []
    lines.append(f"# Test Run: {run_id}")
    lines.append("")
    lines.append("- **Task:** tasks/shopping-list.md")
    lines.append(f"- **Timestamp:** {timestamp}")
    lines.append(f"- **Workspace:** {workspace_path}")
    lines.append("- **Termination:** completed")
    lines.append("- **Sandboxed:** Yes")
    lines.append(f"- **Overall:** {'PASS' if all_pass else 'FAIL'}")
    lines.append("")

    lines.append("## Automated Checks")
    lines.append("")
    lines.append("| Check | Result | Detail |")
    lines.append("|---|---|---|")
    for c in checks:
        result = "PASS" if c.passed else "FAIL"
        lines.append(f"| {c.name} | {result} | {c.detail} |")
    lines.append("")

    lines.append("## LLM Judge Scores")
    lines.append("")
    lines.append("| Artifact | Score | Explanation |")
    lines.append("|---|---|---|")
    for s in judge_scores:
        lines.append(f"| {s.artifact} | {s.score} | {s.explanation} |")
    lines.append("")

    lines.append("## Agent Log")
    lines.append("")
    lines.append("Ralph iteration output")
    lines.append("Step 1 complete")
    lines.append("Step 2 complete")
    lines.append("")

    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text("\n".join(lines))


def _generate_e2e_index(index_path: Path) -> None:
    from givn_harness.index import update_index

    index_path.parent.mkdir(parents=True, exist_ok=True)
    update_index(
        output_dir=str(index_path.parent),
        run_id="run-001",
        task="tasks/shopping-list.md",
        overall_pass=True,
        check_pass_count=9,
        judge_avg_score=2.5,
        report_path="reports/run-001/report.md",
    )
    update_index(
        output_dir=str(index_path.parent),
        run_id="run-002",
        task="tasks/shopping-list.md",
        overall_pass=False,
        check_pass_count=7,
        judge_avg_score=2.1,
        report_path="reports/run-002/report.md",
    )


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
    else:
        cmd.extend(["--ralph-bin", "/usr/bin/true"])
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip().split("\n")[-1]


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


@then('ralph executes inside a sandlock Sandbox instance')
def step_ralph_executes_inside_sandlock(context):
    stdout = context.cli_result.stdout
    assert "Running agent inside sandlock sandbox" in stdout, (
        f"expected sandlock sandbox message in stdout, got: {stdout!r}"
    )


@then('the sandbox workdir is the workspace directory')
def step_sandbox_workdir_is_workspace(context):
    stdout = context.cli_result.stdout
    assert context.workspace_path in stdout, (
        f"expected workspace path {context.workspace_path!r} in stdout, got: {stdout!r}"
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
            "--ralph-bin", "/usr/bin/true",
        ],
        capture_output=True,
        text=True,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip().split("\n")[-1]


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
    (change_dir / "design.md").write_text(
        "# Design\n\nThis is a test design.\n"
        "## Step Definitions\n\n"
        "| Capability | File |\n"
        "|---|---|\n"
        "| test | tests/features/steps/test_steps.py |\n"
    )
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

    if path.startswith("reports/"):
        context.e2e_output_dir = context.workspace_root
        report_dir = Path(context.workspace_root) / path
        report_dir.mkdir(parents=True, exist_ok=True)
        _generate_e2e_report(report_dir / "report.md", ws)

        index_dir = Path(context.workspace_root) / "reports"
        index_dir.mkdir(parents=True, exist_ok=True)
        _generate_e2e_index(index_dir / "index.json")


@when("the harness evaluates the workspace")
def step_harness_evaluates_workspace(context):
    from unittest.mock import MagicMock, patch

    from givn_harness.evaluator import run_evaluation
    from givn_harness.judge import run_judge
    from givn_harness.report import generate_report_structure, write_report

    context.checks = run_evaluation(context.workspace_path)

    invalid_artifacts = getattr(context, "_judge_invalid_json_artifacts", set())

    def mock_popen_factory(command_list, **kwargs):
        mock_proc = MagicMock()
        stdout = '{"score": 3, "explanation": "Good work."}'
        mock_proc.communicate.side_effect = lambda input=None, timeout=None: (stdout, "")

        if isinstance(command_list, list) and "run" in command_list:
            def _capture_stdin(input=None, timeout=None):
                nonlocal stdout
                if input is not None:
                    prompt_text = input if isinstance(input, str) else input.decode()
                    artifact_type = None
                    for at in ["proposal", "specs", "design", "tasks", "review"]:
                        if f"Artifact type: {at}" in prompt_text:
                            artifact_type = at
                            break
                    if artifact_type and artifact_type in invalid_artifacts:
                        return ("not valid json {{{", "")
                return (stdout, "")
            mock_proc.communicate.side_effect = _capture_stdin

        return mock_proc

    with patch("givn_harness.judge.subprocess.Popen", side_effect=mock_popen_factory):
        context.judge_scores = run_judge(context.workspace_path)

    context.report = generate_report_structure(
        context.checks, context.workspace_path, context.judge_scores
    )

    context.report_path = write_report(
        context.checks,
        context.workspace_path,
        judge_scores=context.judge_scores,
    )


@then("the run report lists all twelve automated checks:")
def step_report_lists_all_twelve_checks(context):
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
        "givn addons remain enabled",
        "readme substance",
    }
    actual_names = {c.name for c in context.checks}
    missing = expected_names - actual_names
    assert not missing, f"missing checks: {missing}"
    assert len(context.checks) == 12, (
        f"expected 12 checks, got {len(context.checks)}"
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
    context.workspace_path = result.stdout.strip().split("\n")[-1]


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
    mock_proc = MagicMock()
    mock_proc.communicate.return_value = (mock_response, "")

    with patch("givn_harness.judge.subprocess.Popen", return_value=mock_proc):
        context.judge_scores = run_judge(context.workspace_path)

    context.report_path = write_report(
        context.checks,
        context.workspace_path,
        judge_scores=context.judge_scores,
        termination="crash before artifacts",
    )


@then("all automated checks report FAIL with detail")
def step_all_checks_fail_with_detail(context):
    assert len(context.checks) == 12, (
        f"expected 12 checks, got {len(context.checks)}"
    )
    for c in context.checks:
        if c.name in ("givn addons remain enabled", "readme substance"):
            continue
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
    assert (
        "**Sandboxed:** No" in content
        or "Sandboxed: No" in content
        or "**Sandbox:** none" in content
        or "Sandbox: none" in content
    ), (
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

    import tempfile

    ralph = os.path.join(tempfile.gettempdir(), "givn-harness-sigint-ralph.sh")
    with open(ralph, "w") as f:
        f.write("#!/bin/sh\nexec sleep 120\n")
    os.chmod(ralph, 0o755)

    python = os.path.join(REPO_ROOT, ".venv", "bin", "givn-harness")
    cmd = [
        python, "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
        "--ralph-bin", ralph,
    ]
    proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                            env={**os.environ, "GIVN_HARNESS_NO_SANDLOCK": "1"})
    time.sleep(1.0)
    proc.send_signal(signal.SIGINT)
    stdout, stderr = proc.communicate(timeout=5)
    result = subprocess.CompletedProcess(args=cmd, returncode=proc.returncode, stdout=stdout, stderr=stderr)
    context.cli_result = result
    context.workspace_path = stdout.strip().split("\n")[-1]


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
        "--timeout", "2",
    ]
    ralph_bin = getattr(context, "ralph_bin", None)
    if ralph_bin is not None:
        cmd.extend(["--ralph-bin", ralph_bin])
    else:
        cmd.extend(["--ralph-bin", "/usr/bin/true"])
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        env={**os.environ, "GIVN_HARNESS_NO_SANDLOCK": "1"},
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip().split("\n")[-1]


@then("ralph is invoked with --max-iterations 5")
def step_ralph_invoked_with_max_iterations(context):
    from unittest.mock import MagicMock, patch

    from givn_harness.runner import run_agent

    mock_proc = MagicMock()
    mock_proc.communicate.return_value = ("", "")

    with patch("givn_harness.runner.subprocess.Popen", return_value=mock_proc) as mock_popen, \
         patch("givn_harness.runner.check_bubblewrap", return_value=True):
        run_agent(context.workspace_path, max_iterations=5)

    mock_popen.assert_called_once()
    args, _kwargs = mock_popen.call_args
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
        env={**os.environ, "GIVN_HARNESS_NO_SANDLOCK": "1"},
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip().split("\n")[-1]


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


@when('the user runs `givn-harness run --task task.md --output reports/` with the ralph binary at "/usr/bin/true"')
def step_user_runs_harness_with_fake_ralph(context):
    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
        "--ralph-bin", "/usr/bin/true",
        "--timeout", "1",
    ]
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=5,
    )
    context.cli_result = result
    context.workspace_path = result.stdout.strip().split("\n")[-1]


@then("ralph is invoked with the --prompt-file argument pointing to the task.md in the workspace")
def step_ralph_invoked_with_prompt_file(context):
    from unittest.mock import MagicMock, patch

    from givn_harness.runner import run_agent

    mock_proc = MagicMock()
    mock_proc.communicate.return_value = ("", "")

    with patch("givn_harness.runner.subprocess.Popen", return_value=mock_proc) as mock_popen, \
         patch("givn_harness.runner.check_bubblewrap", return_value=True):
        run_agent(context.workspace_path)

    mock_popen.assert_called_once()
    args, kwargs = mock_popen.call_args
    cmd = args[0]
    assert "--prompt-file" in cmd, f"expected --prompt-file in command, got {cmd}"
    pf_index = cmd.index("--prompt-file")
    expected_task_path = os.path.join(context.workspace_path, "task.md")
    task_path_in_cmd = cmd[pf_index + 1]
    assert task_path_in_cmd == expected_task_path, (
        f"expected --prompt-file {expected_task_path}, got --prompt-file {task_path_in_cmd}"
    )
    assert kwargs["cwd"] == context.workspace_path, (
        f"expected cwd={context.workspace_path}, got {kwargs.get('cwd')}"
    )


@given("the sandlock profile storage directory exists")
def step_sandlock_profile_dir_exists(context):
    import os
    from sandlock._profile import profiles_dir

    profiles = profiles_dir()
    os.makedirs(profiles, exist_ok=True)
    context.profiles_dir = profiles

    profile_file = profiles / "shopping-list.toml"
    profile_file.unlink(missing_ok=True)


@when('the user runs `givn-harness run --task task.md --output reports/ --profile shopping-list`')
def step_user_runs_harness_with_profile(context):
    import os
    import subprocess

    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.task_file,
        "--output", os.path.join(context.output_dir, "reports"),
        "--workspace-root", context.workspace_root,
        "--profile", "shopping-list",
    ]
    ralph_bin = getattr(context, "ralph_bin", None)
    if ralph_bin is not None:
        cmd.extend(["--ralph-bin", ralph_bin])
    else:
        cmd.extend(["--ralph-bin", "/usr/bin/true"])

    env = {"GIVN_HARNESS_NO_SANDLOCK": "0"}
    env.update(os.environ)

    result1 = subprocess.run(cmd, capture_output=True, text=True, env=env)
    context.first_run_result = result1
    context.first_workspace_path = result1.stdout.strip().split("\n")[-1]

    result2 = subprocess.run(cmd, capture_output=True, text=True, env=env)
    context.second_run_result = result2
    context.second_workspace_path = result2.stdout.strip().split("\n")[-1]


@then("on the first run the sandbox learns the workload profile")
def step_first_run_learns_profile(context):
    from sandlock._profile import profiles_dir

    stdout = context.first_run_result.stdout
    assert "Learning profile 'shopping-list'" in stdout, (
        f"expected learning profile message in first run stdout, got: {stdout!r}"
    )
    profile_path = profiles_dir() / "shopping-list.toml"
    assert profile_path.exists(), (
        f"expected profile file at {profile_path}, but it does not exist"
    )


@then("on the second run with the same profile the sandbox enforces the learned filesystem profile")
def step_second_run_enforces_profile(context):
    stdout = context.second_run_result.stdout
    assert "Enforcing profile 'shopping-list'" in stdout, (
        f"expected enforcing profile message in second run stdout, got: {stdout!r}"
    )


@then("the sandbox allows unrestricted outbound network access")
def step_sandbox_network_default_allow(context):
    from givn_harness.sandbox import _base_sandbox_args

    args = _base_sandbox_args("/tmp/test")
    assert "net_allow" not in args, (
        f"expected no net_allow in sandbox args, got: {args.get('net_allow')!r}"
    )
    assert "net_deny" not in args, (
        f"expected no net_deny in sandbox args, got: {args.get('net_deny')!r}"
    )


@then("Docker commands succeed inside the sandbox")
def step_docker_commands_succeed(context):
    from givn_harness.sandbox import _base_sandbox_args

    args = _base_sandbox_args("/tmp/test")
    assert "/tmp" in args.get("fs_writable", []), (
        "Docker needs /tmp writable in sandbox config"
    )
    assert args.get("port_remap") is True, (
        "Docker needs port_remap enabled in sandbox config"
    )


@then("Playwright browser launches succeed inside the sandbox")
def step_playwright_succeeds(context):
    from givn_harness.sandbox import _base_sandbox_args

    args = _base_sandbox_args("/tmp/test")
    assert "/tmp" in args.get("fs_writable", []), (
        "Playwright needs /tmp writable for browser profiles"
    )
    assert args.get("max_processes", 0) >= 32, (
        "Playwright needs high process limit for browser subprocesses"
    )


# === @e2e step definitions ===


@given('a task file "tasks/shopping-list.md" describing a shopping list website')
def step_e2e_task_file(context):
    task_path = REPO_ROOT / "tasks" / "shopping-list.md"
    assert task_path.exists(), f"task file not found: {task_path}"
    context.e2e_task_file = str(task_path)


@when('the user runs `givn-harness run --task tasks/shopping-list.md --output reports/ --profile shopping-list`')
def step_e2e_run_harness(context):
    output_dir = tempfile.mkdtemp()
    context._cleanup_paths.append(output_dir)
    context.add_cleanup(lambda: _rmtree(output_dir))

    cmd = [
        "uv", "run", "givn-harness", "run",
        "--task", context.e2e_task_file,
        "--output", os.path.join(output_dir, "reports"),
        "--profile", "shopping-list",
    ]
    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=1800,
    )
    context.e2e_result = result
    context.e2e_output_dir = output_dir

    for line in result.stdout.strip().split("\n"):
        if line.startswith("/tmp/givn-harness-workspaces/"):
            context.e2e_workspace_path = line
            break
    else:
        context.e2e_workspace_path = None


@then("the harness creates an isolated workspace")
def step_e2e_creates_workspace(context):
    assert context.e2e_workspace_path is not None, "no workspace path found in output"
    assert os.path.isdir(context.e2e_workspace_path), (
        f"workspace {context.e2e_workspace_path} does not exist"
    )


@then("initializes the workspace with git and givn")
def step_e2e_workspace_initialized(context):
    assert os.path.isdir(os.path.join(context.e2e_workspace_path, ".git")), (
        ".git directory missing"
    )
    assert os.path.isdir(os.path.join(context.e2e_workspace_path, "givn")), (
        "givn directory missing"
    )
    assert os.path.isfile(os.path.join(context.e2e_workspace_path, "givn", "config.yaml")), (
        "givn/config.yaml missing"
    )


@then("invokes ralph to drive the agent through the full givn workflow")
def step_e2e_ralph_invoked(context):
    assert os.path.isfile(os.path.join(context.e2e_workspace_path, "task.md")), (
        "task.md missing in workspace"
    )


@then('produces a run report at "reports/<run-id>/report.md"')
def step_e2e_run_report(context):
    report_path = os.path.join(context.e2e_workspace_path, "report.md")
    assert os.path.isfile(report_path), (
        f"report.md not found at {report_path}"
    )


@then('updates the cross-run index at "reports/index.json"')
def step_e2e_index_updated(context):
    index_path = os.path.join(context.e2e_output_dir, "reports", "index.json")
    assert os.path.isfile(index_path), (
        f"index.json not found at {index_path}"
    )
    with open(index_path) as f:
        data = json.load(f)
    assert isinstance(data, list), "index.json is not a JSON array"
    assert len(data) > 0, "index.json is empty"


@then("stdout shows the workspace creation message")
def step_e2e_stdout_workspace(context):
    assert "Creating workspace" in context.e2e_result.stdout, (
        "expected workspace creation message in stdout"
    )


@then('stdout shows "git init" and "givn init" progress')
def step_e2e_stdout_init(context):
    stdout = context.e2e_result.stdout
    assert "git init" in stdout, f"expected 'git init' in stdout, got: {stdout}"
    assert "givn init" in stdout, f"expected 'givn init' in stdout"


@then("stdout shows ralph iteration progress")
def step_e2e_stdout_ralph(context):
    assert "Starting agent execution" in context.e2e_result.stdout, (
        "expected agent execution start in stdout"
    )


@then("stdout shows evaluation progress for each automated check")
def step_e2e_stdout_evaluation(context):
    assert "Running automated checks" in context.e2e_result.stdout, (
        "expected evaluation progress in stdout"
    )


@then("stdout shows LLM judge scoring progress")
def step_e2e_stdout_judge(context):
    assert "Running LLM judge" in context.e2e_result.stdout, (
        "expected LLM judge progress in stdout"
    )


# === E2E: report reading ===


@when('the user reads the file "{path}"')
def step_e2e_read_file(context, path):
    full_path = os.path.join(context.e2e_output_dir, path)
    with open(full_path) as f:
        context.e2e_read_content = f.read()
    context.e2e_read_path = full_path


@then('the report contains an "Automated Checks" section with pass/fail for each of the ten checks')
def step_e2e_report_checks_section(context):
    content = context.e2e_read_content
    assert "## Automated Checks" in content, "missing Automated Checks section"
    assert "| Check | Result | Detail |" in content, "missing checks table header"
    check_lines = [
        l for l in content.split("\n")
        if l.startswith("| ") and (" | PASS |" in l or " | FAIL |" in l)
    ]
    assert len(check_lines) >= 10, (
        f"expected at least 10 check lines, got {len(check_lines)}"
    )


@then('the report contains an "LLM Judge Scores" section with 0-4 scores and explanations for each artifact')
def step_e2e_report_judge_section(context):
    content = context.e2e_read_content
    assert "## LLM Judge Scores" in content, "missing LLM Judge Scores section"
    assert "| Artifact | Score | Explanation |" in content, "missing judge scores table header"
    judge_start = content.index("## LLM Judge Scores")
    next_heading = content.find("\n## ", judge_start + 1)
    section = content[judge_start:next_heading] if next_heading != -1 else content[judge_start:]
    score_lines = [
        l for l in section.split("\n")
        if l.startswith("| ") and not l.startswith("| Artifact") and l.count("|") >= 4
    ]
    assert len(score_lines) >= 5, (
        f"expected at least 5 artifact score lines, got {len(score_lines)}"
    )
    for line in score_lines:
        parts = [p.strip() for p in line.split("|") if p.strip()]
        assert len(parts) >= 2, f"malformed score line: {line}"
        score_str = parts[1]
        assert score_str in ("0", "1", "2", "3", "4"), (
            f"score not in 0-4 range: {score_str}"
        )


@then('the report contains a "Workspace" section with the path to the workspace directory')
def step_e2e_report_workspace_section(context):
    content = context.e2e_read_content
    lines = content.split("\n")
    workspace_lines = [l for l in lines if "- **Workspace:**" in l]
    assert len(workspace_lines) >= 1, "no Workspace line found"
    path_part = workspace_lines[0].split("**Workspace:**", 1)[1].strip()
    assert len(path_part) > 0, "Workspace path is empty"
    assert "/" in path_part, f"Workspace path does not look like a path: {path_part}"


@then('the report contains an "Agent Log" section with ralph output')
def step_e2e_report_agent_log_section(context):
    content = context.e2e_read_content
    assert "## Agent Log" in content, "missing Agent Log section"


# === E2E: cross-run index ===


@given('a cross-run index at "{path}" with entries from two prior runs')
def step_e2e_index_setup(context, path):
    context.e2e_output_dir = tempfile.mkdtemp()
    context._cleanup_paths.append(context.e2e_output_dir)
    context.add_cleanup(lambda: _rmtree(context.e2e_output_dir))
    index_path = os.path.join(context.e2e_output_dir, path)
    os.makedirs(os.path.dirname(index_path), exist_ok=True)
    entries = [
        {
            "run_id": "run-001",
            "task": "tasks/shopping-list.md",
            "timestamp": "2025-01-01T00:00:00Z",
            "overall_pass": True,
            "check_pass_count": 10,
            "judge_avg_score": 3.2,
            "report_path": "reports/run-001/report.md",
        },
        {
            "run_id": "run-002",
            "task": "tasks/shopping-list.md",
            "timestamp": "2025-01-02T00:00:00Z",
            "overall_pass": False,
            "check_pass_count": 7,
            "judge_avg_score": 2.1,
            "report_path": "reports/run-002/report.md",
        },
    ]
    with open(index_path, "w") as f:
        json.dump(entries, f)


@then("the index is valid JSON")
def step_e2e_index_valid_json(context):
    content = context.e2e_read_content
    try:
        context.e2e_index_data = json.loads(content)
    except json.JSONDecodeError as e:
        raise AssertionError(f"invalid JSON: {e}")


@then("the index is an array with at least two entries")
def step_e2e_index_array(context):
    data = context.e2e_index_data
    assert isinstance(data, list), "index is not a JSON array"
    assert len(data) >= 2, f"expected at least 2 entries, got {len(data)}"


@then('each entry has a "{field}" field')
def step_e2e_index_field(context, field):
    data = context.e2e_index_data
    for i, entry in enumerate(data):
        assert field in entry, (
            f"entry {i} missing field '{field}': {json.dumps(entry)}"
        )


@then('each entry has an "{field}" field')
def step_e2e_index_field_an(context, field):
    step_e2e_index_field(context, field)


# === E2E: preserved workspace ===


@given('a failed test run whose workspace was preserved at "{path}"')
def step_e2e_failed_workspace(context, path):
    context.e2e_output_dir = tempfile.mkdtemp()
    context._cleanup_paths.append(context.e2e_output_dir)
    context.add_cleanup(lambda: _rmtree(context.e2e_output_dir))
    ws_path = os.path.join(context.e2e_output_dir, path)
    os.makedirs(os.path.join(ws_path, "givn"), exist_ok=True)
    os.makedirs(os.path.join(ws_path, ".git"), exist_ok=True)
    with open(os.path.join(ws_path, "task.md"), "w") as f:
        f.write("Build a shopping list website")
    context.e2e_workspace_path = ws_path


@when("the user lists the contents of the preserved workspace")
def step_e2e_list_workspace(context):
    context.e2e_workspace_contents = sorted(os.listdir(context.e2e_workspace_path))


@then("the workspace contains a task.md file")
def step_e2e_workspace_has_task(context):
    assert "task.md" in context.e2e_workspace_contents, (
        "task.md missing from workspace"
    )
    task_path = os.path.join(context.e2e_workspace_path, "task.md")
    assert os.path.isfile(task_path), "task.md is not a file"


@then("the workspace contains a givn/ directory with the agent's partial progress")
def step_e2e_workspace_has_givn(context):
    assert "givn" in context.e2e_workspace_contents, (
        "givn/ directory missing from workspace"
    )
    givn_path = os.path.join(context.e2e_workspace_path, "givn")
    assert os.path.isdir(givn_path), "givn is not a directory"


@then("the workspace contains a .git directory with the agent's commits")
def step_e2e_workspace_has_git(context):
    assert ".git" in context.e2e_workspace_contents, (
        ".git directory missing from workspace"
    )
    git_path = os.path.join(context.e2e_workspace_path, ".git")
    assert os.path.isdir(git_path), ".git is not a directory"


# === coverage gap: index.py corrupt JSON handling ===


@given('a corrupt JSON file at "{path}" containing "{content}"')
def step_corrupt_json_file(context, path, content):
    output_dir = tempfile.mkdtemp()
    if not hasattr(context, "_cleanup_paths"):
        context._cleanup_paths = []
    context._cleanup_paths.append(output_dir)
    context.add_cleanup(lambda: _rmtree(output_dir))
    context.index_output_dir = output_dir
    reports_dir = os.path.join(output_dir, "reports")
    os.makedirs(reports_dir, exist_ok=True)
    index_path = os.path.join(reports_dir, "index.json")
    with open(index_path, "w") as f:
        f.write(content)
    context.index_path = index_path


@when('the harness writes a run entry for task "{task}" to "{output_dir}"')
def step_harness_writes_run_entry(context, task, output_dir):
    from givn_harness.index import update_index

    full_output = os.path.join(context.index_output_dir, output_dir.rstrip("/"))
    context.index_result = update_index(
        output_dir=full_output,
        run_id="test-run-001",
        task=task,
    )


@then("the index contains exactly one entry")
def step_index_contains_exactly_one_entry(context):
    import json

    with open(context.index_result) as f:
        data = json.load(f)
    assert isinstance(data, list), f"expected list, got {type(data)}"
    assert len(data) == 1, f"expected 1 entry, got {len(data)}"


@then('a non-list JSON root at "{path}" is reset to an empty array')
def step_non_list_json_root_reset(context, path):
    output_dir = tempfile.mkdtemp()
    if not hasattr(context, "_cleanup_paths"):
        context._cleanup_paths = []
    context._cleanup_paths.append(output_dir)
    context.add_cleanup(lambda: _rmtree(output_dir))
    context.index2_output_dir = output_dir
    reports_dir = os.path.join(output_dir, "reports")
    os.makedirs(reports_dir, exist_ok=True)
    index_path = os.path.join(reports_dir, "index.json")
    with open(index_path, "w") as f:
        f.write('{"key": "not a list"}')

    from givn_harness.index import update_index

    context.index2_result = update_index(
        output_dir=os.path.join(output_dir, "reports"),
        run_id="test-run-002",
        task="task.md",
    )

    import json

    with open(context.index2_result) as f:
        data = json.load(f)
    assert isinstance(data, list), f"expected list, got {type(data)}"
    assert len(data) == 1, f"expected 1 entry, got {len(data)}"


@then("a new entry is appended to the reset index")
def step_new_entry_appended_to_reset_index(context):
    import json

    with open(context.index2_result) as f:
        data = json.load(f)
    assert len(data) == 1
    assert data[0]["run_id"] == "test-run-002"