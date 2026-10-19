import json
import os
import shutil
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

from givn_harness.model import AGENT_MODELS, AGENTS, require_agent_model
from givn_harness.profile import task_complete, task_contract
from givn_harness.sandbox import build_bwrap_cmd, check_bubblewrap, _wrap_with_limits

LOG_FILENAME = "agent.log"
DEFAULT_NATIVE_ITERATIONS = 10
TASK_PROMISE = "READY_FOR_NEXT_TASK"
COMPLETION_PROMISE = "COMPLETE"
DEFAULT_PROMPT_TEMPLATE = Path(__file__).resolve().parents[2] / "assets" / "ralph-givn-prompt.md"


def _read_blocked(workspace_path: str) -> str:
    blocked_path = os.path.join(workspace_path, "BLOCKED.md")
    if not os.path.isfile(blocked_path):
        return ""
    return open(blocked_path).read()


def _prepare_template(
    template_path: str, dest_path: str, workspace_path: str, profile: str = "full",
) -> None:
    content = open(template_path).read()
    content = content.replace("{{blocked}}", _read_blocked(workspace_path))
    content = content.replace("{{task_contract}}", task_contract(profile))
    with open(dest_path, "w") as f:
        f.write(content)


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)


def check_agent_binary(
    agent: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    driver: str = "ralph",
) -> bool:
    if agent == "opencode" and driver == "ralph":
        return check_ralph_binary(ralph_bin)
    return shutil.which(agent) is not None


def run_agent(
    workspace_path: str,
    ralph_bin: str = "/usr/local/bin/ralph",
    timeout: int | None = None,
    max_iterations: int | None = None,
    profile_info: dict | None = None,
    show_output: bool = True,
    model: str | None = None,
    prompt_template: str | None = None,
    agent: str = "opencode",
    driver: str = "ralph",
    profile: str = "full",
) -> tuple[str, str]:
    agent = agent.lower()
    if agent not in AGENTS:
        raise ValueError(f"unknown agent {agent!r}; choose one of {', '.join(AGENTS)}")
    driver = driver.lower()
    if driver not in ("ralph", "native"):
        raise ValueError(f"unknown driver {driver!r}; choose ralph or native")
    model = require_agent_model(agent, model)
    if agent == "opencode" and driver == "ralph":
        return _run_ralph(
            workspace_path, ralph_bin, timeout, max_iterations, profile_info,
            show_output, model, prompt_template, profile,
        )
    return _run_native(
        workspace_path, agent, timeout, max_iterations, show_output, model,
        prompt_template, driver, profile,
    )


def _run_ralph(
    workspace_path: str,
    ralph_bin: str,
    timeout: int | None,
    max_iterations: int | None,
    profile_info: dict | None,
    show_output: bool,
    model: str,
    prompt_template: str | None,
    profile: str = "full",
) -> tuple[str, str]:
    task_path = os.path.join(workspace_path, "task.md")
    cmd = [ralph_bin, "--agent", "opencode", "--prompt-file", task_path]
    if max_iterations is not None:
        cmd.extend(["--max-iterations", str(max_iterations)])
    if show_output:
        cmd.append("--verbose-tools")
    cmd.extend(["--model", model])
    # A private opencode server loads the workspace's own .opencode plugins;
    # the shared background service keeps the plugin root of whichever project
    # started it, so the givn timeline reporter would never load.
    cmd.extend(["--", "--standalone"])
    if prompt_template is not None:
        tmpl_dest = os.path.join(workspace_path, ".ralph-prompt-template.md")
        _prepare_template(prompt_template, tmpl_dest, workspace_path, profile)
        cmd.extend(["--prompt-template", tmpl_dest])

    log_path = os.path.join(workspace_path, LOG_FILENAME)

    if show_output:
        sys.stderr.write(f"  ralph: {' '.join(cmd)}\n")
        sys.stderr.flush()

    if check_bubblewrap():
        return _run_with_bwrap(cmd, timeout, workspace_path, log_path, show_output)

    return _run_without_sandbox(cmd, timeout, workspace_path, log_path, show_output)


def _render_native_prompt(
    template: str, workspace_path: str, iteration: int, max_iterations: int,
    profile: str = "full",
) -> str:
    task_path = os.path.join(workspace_path, "task.md")
    task_text = Path(task_path).read_text() if os.path.isfile(task_path) else ""
    values = {
        "{{iteration}}": str(iteration),
        "{{max_iterations}}": str(max_iterations),
        "{{prompt}}": task_text,
        "{{task_contract}}": task_contract(profile),
        "{{task_promise}}": TASK_PROMISE,
        "{{completion_promise}}": COMPLETION_PROMISE,
        "{{blocked}}": _read_blocked(workspace_path),
    }
    for key, value in values.items():
        template = template.replace(key, value)
    return template


def _native_command(agent: str, model: str, prompt: str) -> list[str]:
    if agent == "opencode":
        return [
            "opencode", "run", "--standalone", "--auto", "--format", "default",
            "--model", model, prompt,
        ]
    if agent == "omp":
        return [
            "omp", "-p", "--mode", "text", "--model", model,
            "--thinking", "max", "--auto-approve", prompt,
        ]
    if agent == "maki":
        return ["maki", "-p", "--trust", "--yolo", "-m", model, prompt]
    raise ValueError(f"no native command for {agent!r}")


def _run_native(
    workspace_path: str,
    agent: str,
    timeout: int | None,
    max_iterations: int | None,
    show_output: bool,
    model: str,
    prompt_template: str | None,
    driver: str = "native",
    profile: str = "full",
) -> tuple[str, str]:
    iterations = max_iterations or DEFAULT_NATIVE_ITERATIONS
    template_path = prompt_template or str(DEFAULT_PROMPT_TEMPLATE)
    template = Path(template_path).read_text()
    log_path = os.path.join(workspace_path, LOG_FILENAME)
    deadline = None if timeout is None else time.monotonic() + timeout
    log_chunks: list[str] = []
    termination = "max_iterations"
    completed = False

    for iteration in range(1, iterations + 1):
        if deadline is not None and time.monotonic() >= deadline:
            termination = "timeout"
            break
        prompt = _render_native_prompt(template, workspace_path, iteration, iterations, profile)
        cmd = _native_command(agent, model, prompt)
        if show_output:
            sys.stderr.write(f"  {agent}: iteration {iteration}/{iterations}\n")
            sys.stderr.flush()
        remaining = None if deadline is None else max(1, int(deadline - time.monotonic()))
        if check_bubblewrap():
            stdout, step_termination = _run_with_bwrap(
                cmd, remaining, workspace_path, "", show_output, write_log=False,
            )
        else:
            stdout, step_termination = _run_without_sandbox(
                cmd, remaining, workspace_path, "", show_output, write_log=False,
            )
        log_chunks.append(f"== iteration {iteration}/{iterations} ==\n{stdout}\n")
        if step_termination != "completed":
            termination = step_termination
            break
        if task_complete(workspace_path, profile):
            termination = "completed"
            completed = True
            break

    log = "\n".join(log_chunks)
    _write_log(log_path, log)
    try:
        with open(os.path.join(workspace_path, ".givn-harness-agent.json"), "w") as f:
            json.dump({
                "agent": agent,
                "driver": driver,
                "profile": profile,
                "iterations": iteration,
                "termination": termination,
                "completed": completed,
            }, f, indent=2)
    except OSError:
        pass
    return log, termination


def _write_log(log_path: str, stdout: str) -> None:
    try:
        with open(log_path, "w") as f:
            f.write(stdout)
    except OSError:
        print(f"warning: could not write agent log to {log_path}")


def _run_with_bwrap(
    cmd: list[str], timeout: int | None, workspace_path: str, log_path: str,
    show_output: bool = True, write_log: bool = True,
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

    if write_log:
        _write_log(log_path, stdout)
    return stdout, termination


def _run_without_sandbox(
    cmd: list[str], timeout: int | None, workspace_path: str, log_path: str,
    show_output: bool = True, write_log: bool = True,
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

    if write_log:
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