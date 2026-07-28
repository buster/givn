import os
import signal
import subprocess
import sys
import threading
from pathlib import Path

LOG_FILENAME = "agent.log"


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(
    workspace_path: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    timeout: int | None = None,
    max_iterations: int | None = None,
    profile_info: dict | None = None,
    verbose: bool = False,
    verbose_tools: bool = False,
    model: str | None = None,
) -> tuple[str, str]:
    task_path = os.path.join(workspace_path, "task.md")
    cmd = [ralph_bin, "--prompt-file", task_path]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])
    if verbose_tools:
        cmd.append("--verbose-tools")

    if model is not None:
        cmd.extend(["--model", model])

    log_path = os.path.join(workspace_path, LOG_FILENAME)

    if verbose:
        sys.stderr.write(f"  ralph: {' '.join(cmd)}\n")
        sys.stderr.flush()

    if profile_info is not None:
        return _run_with_profile(cmd, timeout, workspace_path, profile_info, log_path, verbose)

    return _run_without_sandbox(cmd, timeout, workspace_path, log_path, verbose)


def _result_to_termination(result: object) -> tuple[str, str, int, str]:
    from sandlock import ExitReason

    stdout = result.stdout.decode("utf-8") if isinstance(result.stdout, bytes) else result.stdout
    stderr = result.stderr.decode("utf-8") if isinstance(result.stderr, bytes) else result.stderr
    exit_code = result.exit_code
    detail = ""

    if result.reason == ExitReason.TIMEOUT:
        detail = f"agent timed out (exit code {exit_code})"
        return stdout, "timeout", exit_code, detail
    if result.reason == ExitReason.EXITED:
        detail = f"agent completed (exit code {exit_code})"
        return stdout, "completed", exit_code, detail
    if result.reason in (ExitReason.SIGNALED, ExitReason.KILLED):
        error_msg = stderr.strip() if stderr else f"exit code {exit_code}"
        detail = f"agent killed by sandbox: {error_msg}"
        return stdout, "interrupted", exit_code, detail
    detail = str(result.reason) if result.reason else ""
    return stdout, "completed", exit_code, detail


def _write_log(log_path: str, stdout: str) -> None:
    try:
        with open(log_path, "w") as f:
            f.write(stdout)
    except OSError:
        print(f"warning: could not write agent log to {log_path}")


def _run_with_profile(
    cmd: list[str], timeout: int | None, workspace_path: str, profile_info: dict, log_path: str,
    verbose: bool = False,
) -> tuple[str, str]:
    from sandlock import presets

    sandbox = profile_info["sandbox"]
    mode = profile_info["mode"]
    profile_path: Path | None = profile_info.get("profile_path")
    profile_name: str = profile_info["profile_name"]

    if mode == "learn":
        print(f"Learning profile '{profile_name}' ...")
        print(f"  sandlock profiles dir: {_profiles_dir()}")

        learned_paths: list[str] = []

        def _record(path: str | None, _ctx: object) -> None:
            if path is not None:
                learned_paths.append(path)

        audit = presets.AuditPathsHandler(callback=_record)
        handlers = [
            (sc, audit) for sc in presets.COMMON_PATH_SYSCALLS
            if sc not in ("execve", "execveat")
        ]

        result = sandbox.run_with_handlers(cmd, handlers)
        if profile_path is not None:
            _save_learned_profile(profile_path, learned_paths)
            learned_count = len(set(os.path.dirname(p) for p in learned_paths if os.path.dirname(p)))
            print(f"  captured {len(learned_paths)} path accesses across ~{learned_count} directories")

        stdout, termination, exit_code, detail = _result_to_termination(result)
        _write_log(log_path, stdout)
        print(f"  learn phase {detail}")
        return stdout, termination

    if mode == "enforce":
        print(f"Enforcing profile '{profile_name}' ...")
        print(f"  profile: {profile_path}")

        result = sandbox.run(cmd, timeout=timeout)
        stdout, termination, exit_code, detail = _result_to_termination(result)
        _write_log(log_path, stdout)
        print(f"  enforce phase {detail}")
        return stdout, termination

    print(f"Running agent inside sandlock sandbox (workdir: {workspace_path})")
    result = sandbox.run(cmd, timeout=timeout)
    stdout, termination, exit_code, detail = _result_to_termination(result)
    _write_log(log_path, stdout)
    print(f"  agent {detail}")
    return stdout, termination


def _save_learned_profile(profile_path: Path, paths: list[str]) -> None:
    import os as _os

    from givn_harness.sandbox import _deduplicate_paths

    profile_path.parent.mkdir(parents=True, exist_ok=True)

    dirs: set[str] = set()
    for p in paths:
        parent = _os.path.dirname(p)
        if parent and parent != "/":
            dirs.add(parent)

    home = _os.path.expanduser("~")

    readable = _deduplicate_paths(
        [
            "/dev/null",
            "/dev/zero",
            "/dev/urandom",
            "/dev/random",
            "/dev/fd",
            "/usr",
            "/usr/bin",
            "/usr/local",
            "/usr/local/bin",
            "/usr/local/lib",
            "/bin",
            "/lib",
            "/lib64",
            "/etc",
            home,
            _os.path.join(home, ".config"),
            _os.path.join(home, ".config/opencode"),
            _os.path.join(home, ".local"),
            _os.path.join(home, ".cargo"),
            _os.path.join(home, ".npm"),
            _os.path.join(home, ".cache"),
            _os.path.join(home, ".bun"),
            _os.path.join(home, "bin"),
        ]
        + list(dirs)
    )
    writable = _deduplicate_paths(
        [
            "/tmp",
            _os.path.join(home, ".npm"),
            _os.path.join(home, ".cache"),
            _os.path.join(home, ".bun"),
            _os.path.join(home, ".config"),
            _os.path.join(home, ".local"),
        ]
        + [d for d in dirs if d.startswith("/tmp")]
    )

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


def _profiles_dir() -> Path:
    import sandlock._profile

    return sandlock._profile.profiles_dir()


def _run_without_sandbox(
    cmd: list[str], timeout: int | None, workspace_path: str, log_path: str,
    verbose: bool = False,
) -> tuple[str, str]:
    env = os.environ.copy()
    env["PWD"] = workspace_path
    proc = subprocess.Popen(
        cmd,
        start_new_session=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd=workspace_path,
        env=env,
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

    if verbose:
        stdout = _stream_output(proc, timeout)
    else:
        try:
            stdout, stderr_data = proc.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            termination = "timeout"
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except (ProcessLookupError, OSError):
                pass
            stdout, stderr_data = proc.communicate()
        finally:
            signal.signal(signal.SIGINT, original_handler)

    _write_log(log_path, stdout)
    exit_code = proc.returncode
    print(f"  agent {termination} (exit code {exit_code})")
    return stdout, termination


def _stream_output(proc: subprocess.Popen[str], timeout: int | None) -> str:
    lines: list[str] = []
    deadline = None if timeout is None else (__import__("time").monotonic() + timeout)

    def _reader(pipe, label: str) -> None:
        for line in iter(pipe.readline, ""):
            sys.stderr.write(f"  {label} {line}")
            sys.stderr.flush()
            lines.append(line)

    stdout_thread = threading.Thread(target=_reader, args=(proc.stdout, "|"), daemon=True)
    stderr_thread = threading.Thread(target=_reader, args=(proc.stderr, "E"), daemon=True)
    stdout_thread.start()
    stderr_thread.start()

    while stdout_thread.is_alive() or stderr_thread.is_alive():
        stdout_thread.join(timeout=0.25)
        stderr_thread.join(timeout=0.25)
        if deadline is not None and __import__("time").monotonic() > deadline:
            break

    try:
        os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
    except (ProcessLookupError, OSError):
        pass
    proc.wait()

    return "".join(lines)