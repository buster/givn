import os
import subprocess


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(workspace_path: str, timeout: int | None = None, max_iterations: int | None = None) -> None:
    cmd = ["sleep", "86400"]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])
    subprocess.run(
        cmd,
        timeout=timeout,
        capture_output=True,
        text=True,
        cwd=workspace_path,
    )