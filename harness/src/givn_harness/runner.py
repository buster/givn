import os
import subprocess


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(workspace_path: str, timeout: int | None = None) -> None:
    subprocess.run(
        ["sleep", "86400"],
        timeout=timeout,
        capture_output=True,
        text=True,
        cwd=workspace_path,
    )