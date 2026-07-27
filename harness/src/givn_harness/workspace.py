import os
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
    subprocess.run(
        [givn_binary, "init"],
        cwd=workspace_path,
        capture_output=True,
        check=True,
    )