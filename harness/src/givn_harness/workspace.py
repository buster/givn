import os
import shutil
import subprocess
import tempfile


def create_workspace(workspace_root: str) -> str:
    os.makedirs(workspace_root, exist_ok=True)
    workspace_path = tempfile.mkdtemp(dir=workspace_root)
    return workspace_path


def init_workspace(workspace_path: str) -> None:
    subprocess.run(
        ["git", "init"],
        cwd=workspace_path,
        capture_output=True,
        check=True,
    )
    givn_binary = os.environ.get("GIVN_BINARY", "givn")
    # `givn init` requires an explicit agent-target selection without a
    # terminal; the harness scaffolds for the opencode-based agent runs.
    subprocess.run(
        [givn_binary, "init", "--agents", "opencode"],
        cwd=workspace_path,
        capture_output=True,
        check=True,
    )


def copy_task(workspace_path: str, task_path: str) -> str:
    dest = os.path.join(workspace_path, "task.md")
    shutil.copy(task_path, dest)
    return dest