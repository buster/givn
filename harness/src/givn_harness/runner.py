import os
import signal
import subprocess
from pathlib import Path


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(
    workspace_path: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    timeout: int | None = None,
    max_iterations: int | None = None,
    profile_info: dict | None = None,
) -> tuple[str, str]:
    task_path = os.path.join(workspace_path, "task.md")
    cmd = [ralph_bin, "--prompt-file", task_path]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])

    if profile_info is not None:
        return _run_with_profile(cmd, timeout, workspace_path, profile_info)

    return _run_without_sandbox(cmd, timeout, workspace_path)


def _result_to_termination(result: object) -> tuple[str, str]:
    from sandlock import ExitReason

    stdout = result.stdout.decode("utf-8") if isinstance(result.stdout, bytes) else result.stdout
    if result.reason == ExitReason.TIMEOUT:
        return stdout, "timeout"
    if result.reason == ExitReason.EXITED:
        return stdout, "completed"
    if result.reason in (ExitReason.SIGNALED, ExitReason.KILLED):
        return stdout, "interrupted"
    return stdout, str(result.reason) if result.reason else "completed"


def _run_with_profile(cmd: list[str], timeout: int | None, workspace_path: str, profile_info: dict) -> tuple[str, str]:
    from sandlock import presets

    sandbox = profile_info["sandbox"]
    mode = profile_info["mode"]
    profile_path: Path | None = profile_info.get("profile_path")
    profile_name: str = profile_info["profile_name"]

    if mode == "learn":
        print(f"Learning profile '{profile_name}' ...")

        learned_paths: list[str] = []

        def _record(path: str | None, _ctx: object) -> None:
            if path is not None:
                learned_paths.append(path)

        audit = presets.AuditPathsHandler(callback=_record)
        handlers = [(sc, audit) for sc in presets.COMMON_PATH_SYSCALLS]

        result = sandbox.run_with_handlers(cmd, handlers)
        if profile_path is not None:
            _save_learned_profile(profile_path, learned_paths)

        return _result_to_termination(result)

    if mode == "enforce":
        print(f"Enforcing profile '{profile_name}' ...")

        result = sandbox.run(cmd, timeout=timeout)
        return _result_to_termination(result)

    print(f"Running agent inside sandlock sandbox (workdir: {workspace_path})")
    result = sandbox.run(cmd, timeout=timeout)
    return _result_to_termination(result)


def _save_learned_profile(profile_path: Path, paths: list[str]) -> None:
    profile_path.parent.mkdir(parents=True, exist_ok=True)

    dirs: set[str] = set()
    for p in paths:
        parent = os.path.dirname(p)
        if parent:
            dirs.add(parent)

    readable = sorted({"/usr", "/bin", "/lib", "/lib64", "/etc"} | dirs)
    writable = sorted({"/tmp"} | {d for d in dirs if d.startswith("/tmp")})

    content = f"""\
[filesystem]
read = {readable!r}
write = {writable!r}

[limits]
memory = "16G"
processes = 256
disk = "20G"
"""
    profile_path.write_text(content)


def _run_without_sandbox(cmd: list[str], timeout: int | None, workspace_path: str) -> tuple[str, str]:
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