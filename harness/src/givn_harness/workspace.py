import os
import shutil
import subprocess
import tempfile


def create_workspace(workspace_root: str) -> str:
    os.makedirs(workspace_root, exist_ok=True)
    workspace_path = tempfile.mkdtemp(dir=workspace_root)
    return workspace_path


def init_workspace(workspace_path: str, agent: str = "opencode") -> None:
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
        [givn_binary, "init", "--agents", agent],
        cwd=workspace_path,
        capture_output=True,
        check=True,
    )
    # Capture complete LLM-facing content for every harness run: the smoke's
    # development report must show the actual prompts, tool input, and output.
    config_path = os.path.join(workspace_path, "givn", "config.yaml")
    with open(config_path, "a") as config:
        config.write("\nrecord:\n  mode: full\n")


def copy_task(workspace_path: str, task_path: str) -> str:
    dest = os.path.join(workspace_path, "task.md")
    shutil.copy(task_path, dest)
    # A task may reference sibling inputs by bare filename (for example the
    # shopping-list design zip and screenshot). Those files must be in the
    # workspace: the sandbox has a private /tmp and cannot see the task source
    # directory.
    task_dir = os.path.dirname(os.path.abspath(task_path))
    task_text = open(task_path, encoding="utf-8", errors="replace").read()
    for entry in os.listdir(task_dir):
        source = os.path.join(task_dir, entry)
        if entry == "task.md" or not os.path.isfile(source):
            continue
        if entry in task_text:
            shutil.copy2(source, os.path.join(workspace_path, entry))
    return dest