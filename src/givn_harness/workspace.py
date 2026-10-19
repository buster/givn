"""Trial workspace creation and initialization."""

from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
from pathlib import Path

from givn_harness.benchmark import Benchmark, copy_workspace_inputs


class WorkspaceError(RuntimeError):
    """A trial workspace could not be initialized safely."""


def create_workspace(root: str | Path, label: str) -> Path:
    root_path = Path(root).resolve()
    root_path.mkdir(parents=True, exist_ok=True)
    prefix = "".join(char if char.isalnum() or char in "-_" else "-" for char in label)[:40]
    return Path(tempfile.mkdtemp(prefix=prefix + "-", dir=str(root_path)))


def init_git(workspace: Path, *, user_name: str = "givn harness", user_email: str = "harness@localhost") -> dict:
    completed = subprocess.run(
        ["git", "init"], cwd=workspace, capture_output=True, text=True
    )
    if completed.returncode != 0:
        raise WorkspaceError(f"git init failed: {completed.stderr.strip()}")
    for key, value in (("user.name", user_name), ("user.email", user_email)):
        subprocess.run(
            ["git", "config", key, value],
            cwd=workspace,
            capture_output=True,
            text=True,
        )
    return {"command": "git init", "exit_code": completed.returncode}


def init_givn(workspace: Path, binary: str | Path, agent: str = "opencode") -> dict:
    binary = str(binary)
    completed = subprocess.run(
        [binary, "init", "--agents", agent],
        cwd=workspace,
        capture_output=True,
        text=True,
        timeout=120,
    )
    if completed.returncode != 0:
        raise WorkspaceError(
            f"givn init failed ({completed.returncode}): "
            f"{(completed.stderr or completed.stdout).strip()[:1000]}"
        )
    config = workspace / "givn" / "config.yaml"
    if not config.is_file():
        raise WorkspaceError("givn init did not create givn/config.yaml")
    with config.open("a", encoding="utf-8") as handle:
        handle.write("\nrecord:\n  mode: full\n")
    return {
        "command": f"{binary} init --agents {agent}",
        "exit_code": completed.returncode,
        "stdout": completed.stdout.strip()[:2000],
        "stderr": completed.stderr.strip()[:2000],
    }


def initialize_workspace(
    benchmark: Benchmark,
    root: str | Path,
    *,
    label: str,
    givn_binary: str | Path | None,
    agent: str,
) -> tuple[Path, dict]:
    workspace = create_workspace(root, label)
    copied = copy_workspace_inputs(benchmark, workspace)
    git_info = init_git(workspace)
    givn_info = None
    if givn_binary is not None:
        givn_info = init_givn(workspace, givn_binary, agent=agent)
    return workspace, {
        "workspace": str(workspace),
        "copied_inputs": [str(path.relative_to(workspace)) for path in copied],
        "git": git_info,
        "givn": givn_info,
    }


def remove_workspace(workspace: Path) -> None:
    shutil.rmtree(workspace, ignore_errors=True)


def tail(path: Path, lines: int = 40) -> str:
    if not path.is_file():
        return ""
    content = path.read_text(encoding="utf-8", errors="replace").splitlines()
    return "\n".join(content[-lines:])
