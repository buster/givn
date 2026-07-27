import os
import tempfile


def create_workspace(workspace_root: str) -> str:
    os.makedirs(workspace_root, exist_ok=True)
    workspace_path = tempfile.mkdtemp(dir=workspace_root)
    return workspace_path