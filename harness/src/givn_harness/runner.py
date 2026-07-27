import os
import signal
import subprocess


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(
    workspace_path: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    timeout: int | None = None,
    max_iterations: int | None = None,
    sandbox: object | None = None,
) -> tuple[str, str]:
    """Run the ralph agent. Returns ``(stdout, termination_reason)``.

    The agent is launched in its own process group so the entire tree of
    descendant processes can be killed atomically on timeout or SIGINT.
    """
    task_path = os.path.join(workspace_path, "task.md")
    cmd = [ralph_bin, "--prompt-file", task_path]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])

    if sandbox is not None:
        from sandlock import ExitReason
        print(f"Running agent inside sandlock sandbox (workdir: {workspace_path})")
        result = sandbox.run(cmd, timeout=timeout)
        stdout = result.stdout.decode("utf-8") if isinstance(result.stdout, bytes) else result.stdout
        if result.reason == ExitReason.TIMEOUT:
            return stdout, "timeout"
        if result.reason == ExitReason.EXITED:
            return stdout, "completed"
        if result.reason == ExitReason.SIGNALED or result.reason == ExitReason.KILLED:
            return stdout, "interrupted"
        return stdout, str(result.reason) if result.reason else "completed"

    proc = subprocess.Popen(
        cmd,
        start_new_session=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd=workspace_path,
    )

    termination: str = "completed"

    def _handle_sigint(*_: object) -> None:
        nonlocal termination
        termination = "interrupted"
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except (ProcessLookupError, OSError):
            pass

    original_handler = signal.signal(signal.SIGINT, _handle_sigint)
    try:
        stdout, _stderr = proc.communicate(timeout=timeout)
    except subprocess.TimeoutExpired:
        termination = "timeout"
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except (ProcessLookupError, OSError):
            pass
        stdout, _stderr = proc.communicate()
    finally:
        signal.signal(signal.SIGINT, original_handler)

    return stdout, termination
