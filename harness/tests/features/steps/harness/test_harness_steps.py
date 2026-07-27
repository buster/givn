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