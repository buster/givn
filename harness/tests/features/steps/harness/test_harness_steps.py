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


def _rmtree(path: str) -> None:
    import shutil
    if os.path.isdir(path):
        shutil.rmtree(path, ignore_errors=True)