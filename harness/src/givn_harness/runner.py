import os
import signal
import subprocess
import sys
import threading

from givn_harness.sandbox import build_bwrap_cmd, check_bubblewrap, _wrap_with_limits

LOG_FILENAME = "agent.log"


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def run_agent(
    workspace_path: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    timeout: int | None = None,
    max_iterations: int | None = None,
    profile_info: dict | None = None,
    show_output: bool = True,
    model: str | None = None,
    prompt_template: str | None = None,
) -> tuple[str, str]:
    task_path = os.path.join(workspace_path, "task.md")
    cmd = [ralph_bin, "--prompt-file", task_path]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])
    if show_output:
        cmd.append("--verbose-tools")
    if model is not None:
        cmd.extend(["--model", model])
    if prompt_template is not None:
        cmd.extend(["--prompt-template", prompt_template])

    log_path = os.path.join(workspace_path, LOG_FILENAME)

    if show_output:
        sys.stderr.write(f"  ralph: {' '.join(cmd)}\n")
        sys.stderr.flush()

    if check_bubblewrap():
        return _run_with_bwrap(cmd, timeout, workspace_path, log_path, show_output)

    return _run_without_sandbox(cmd, timeout, workspace_path, log_path, show_output)


def _write_log(log_path: str, stdout: str) -> None:
    try:
        with open(log_path, "w") as f:
            f.write(stdout)
    except OSError:
        print(f"warning: could not write agent log to {log_path}")


def _run_with_bwrap(
    cmd: list[str], timeout: int | None, workspace_path: str, log_path: str,
    show_output: bool = True,
) -> tuple[str, str]:
    bwrap_cmd = build_bwrap_cmd(workspace_path)
    full_cmd, resource_limits = _wrap_with_limits(bwrap_cmd + ["--"] + cmd)

    print(f"Running agent inside bubblewrap sandbox (workdir: {workspace_path})")

    env = os.environ.copy()
    env["PWD"] = workspace_path
    proc = subprocess.Popen(
        full_cmd,
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

    if show_output:
        stream_stdout, stream_termination = _stream_output(proc, timeout)
        stdout = stream_stdout
        stderr_data = ""
        if termination != "interrupted":
            termination = stream_termination
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

    exit_code = proc.returncode
    if exit_code != 0:
        bwrap_error = stderr_data.strip() if stderr_data else ""
        if "bwrap:" in bwrap_error:
            print(f"  sandbox failure: {bwrap_error}")
            termination = "interrupted"
        else:
            print(f"  agent {termination} (exit code {exit_code})")
    else:
        print(f"  agent {termination} (exit code {exit_code})")

    _write_log(log_path, stdout)
    return stdout, termination


def _run_without_sandbox(
    cmd: list[str], timeout: int | None, workspace_path: str, log_path: str,
    show_output: bool = True,
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

    if show_output:
        stream_stdout, stream_termination = _stream_output(proc, timeout)
        stdout = stream_stdout
        if termination != "interrupted":
            termination = stream_termination
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


def _stream_output(proc: subprocess.Popen[str], timeout: int | None) -> tuple[str, str]:
    lines: list[str] = []
    deadline = None if timeout is None else (__import__("time").monotonic() + timeout)
    termination: str = "completed"

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
            termination = "timeout"
            break

    if termination == "timeout":
        try:
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        except (ProcessLookupError, OSError):
            pass
        proc.wait()

    return "".join(lines), termination