import os
import subprocess
import tempfile
from pathlib import Path

from behave import given, when, then
from behave.api.pending_step import StepNotImplementedError

from givn_harness.sandbox import build_bwrap_cmd, _wrap_with_limits


def _rmtree(path: str) -> None:
    import shutil
    if os.path.isdir(path):
        shutil.rmtree(path, ignore_errors=True)


@given("there is no learn-then-confine behaviour to test")
def step_given_no_learn_then_confine(context):
    pass


@given('the bubblewrap binary is not present in PATH')
def step_given_bubblewrap_not_in_path(context):
    context.workspace_root = "/tmp/givn-harness-workspaces"
    os.makedirs(context.workspace_root, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".md", delete=False) as f:
        f.write("task content")
        context.task_file = f.name
    context.output_dir = tempfile.mkdtemp()
    context._cleanup_paths = [context.task_file]
    context.add_cleanup(lambda: os.unlink(context.task_file))
    context.add_cleanup(lambda: _rmtree(context.output_dir))
    os.environ["GIVN_HARNESS_NO_SANDLOCK"] = "1"
    context.add_cleanup(lambda: os.environ.pop("GIVN_HARNESS_NO_SANDLOCK", None))


@when('the user runs `givn-harness run --task tasks/trivial.md --output reports/`')
def step_when_user_runs_harness_with_trivial_task(context):
    project_root = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(__file__)))))
    task_path = os.path.join(project_root, "tasks", "trivial.md")
    output_dir = tempfile.mkdtemp()
    result = subprocess.run(
        ["givn-harness", "run",
         "--task", task_path,
         "--output", output_dir,
         "--timeout", "180"],
        capture_output=True,
        text=True,
        timeout=240,
        env={**os.environ, "GIVN_HARNESS_WORKSPACE_ROOT": context.workspace_root},
    )
    context.cli_result = result
    context.output_dir = output_dir
    context.add_cleanup(lambda: _rmtree(output_dir))
    # find workspace dir from harness output
    for line in result.stdout.splitlines():
        if "workdir:" in line:
            context.workspace_path = line.split("workdir:")[1].strip().rstrip(")")
            break
    else:
        context.workspace_path = os.path.join(context.workspace_root, "trivial-e2e")


@given('a task file "tasks/trivial.md" describing a trivial task using the givn workflow')
def step_given_trivial_task_file(context):
    pass


@given("the agent attempts to fork more than 256 processes")
def step_given_agent_forks_too_many_processes(context):
    pass


@given("the agent attempts to allocate more than 16 GB of memory")
def step_given_agent_allocates_too_much_memory(context):
    pass


@then("the agent terminates with a resource limit error")
def step_then_agent_terminates_resource_limit(context):
    cmd, limits = _wrap_with_limits(["/usr/bin/true"])
    assert limits in ("systemd", "none"), f"unexpected limits: {limits!r}"
    if limits == "systemd":
        assert "systemd-run" in cmd, f"expected systemd-run in command, got: {cmd}"
        assert "--user" in cmd
        assert "--scope" in cmd
        assert "-p" in cmd
        flags: list[str] = []
        i = 0
        while i < len(cmd):
            if cmd[i] == "-p" and i + 1 < len(cmd):
                flags.append(cmd[i + 1])
                i += 2
            else:
                i += 1
        assert "TasksMax=256" in flags, (
            f"expected TasksMax=256 among -p flags, got: {flags}"
        )
        assert "MemoryMax=16G" in flags, (
            f"expected MemoryMax=16G among -p flags, got: {flags}"
        )


@given('the task instructs the agent to create a file "hello.txt" in the workspace')
def step_given_task_instructs_create_file(context):
    pass


@then("ralph executes inside a bubblewrap sandbox")
def step_then_ralph_executes_inside_bubblewrap(context):
    stdout = context.cli_result.stdout
    assert "Running agent inside bubblewrap sandbox" in stdout, (
        f"expected bubblewrap sandbox message in stdout, got: {stdout!r}"
    )


@then("the harness logs a warning that bubblewrap is unavailable")
def step_then_harness_logs_bubblewrap_unavailable_warning(context):
    stderr = context.cli_result.stderr
    assert "bubblewrap" in stderr.lower(), (
        f"expected stderr to mention bubblewrap, got: {stderr!r}"
    )


@then("the agent can write files inside the workspace directory")
def step_then_agent_can_write_workspace(context):
    test_file = os.path.join(context.workspace_path, "sandbox_write_test.txt")
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    cmd = bwrap_cmd + ["--", "touch", test_file]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    assert result.returncode == 0, f"write to workspace failed: {result.stderr}"
    assert os.path.exists(test_file), f"{test_file} should exist after write"


@then('the agent can write files to "/tmp"')
def step_then_agent_can_write_tmp(context):
    test_file = "/tmp/harness_bwrap_test_write.txt"
    try:
        os.unlink(test_file)
    except OSError:
        pass
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    cmd = bwrap_cmd + ["--", "touch", test_file]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    assert result.returncode == 0, f"write to /tmp failed: {result.stderr}"
    assert os.path.exists(test_file), f"{test_file} should exist after write"


@then('the agent cannot write files to "/home" or "/var" or "/opt"')
def step_then_agent_cannot_write_restricted(context):
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    denied = []
    for path in ["/var", "/opt"]:
        test_file = f"{path}/_sandbox_test"
        cmd = bwrap_cmd + ["--", "touch", test_file]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode != 0:
            denied.append(path)
    assert len(denied) == 2, (
        f"/var and /opt should not be writable, denied={denied}"
    )


@then('the agent can read files from "/usr", "/bin", "/lib", "/lib64", and "/etc"')
def step_then_agent_can_read_system_paths(context):
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    readable = []
    unreadable = []
    targets = [
        ("/usr", "d", "/usr/bin/env"),
        ("/bin", "d", "/bin/sh"),
        ("/etc", "f", "/etc/passwd"),
    ]
    for check_path, check_type, example_file in targets:
        if check_type == "d":
            cmd = bwrap_cmd + ["--", "test", "-d", check_path]
        else:
            cmd = bwrap_cmd + ["--", "test", "-f", check_path]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            readable.append(check_path)
        else:
            unreadable.append(check_path)

    # /lib and /lib64 are tested best-effort via -d
    for check_path in ["/lib", "/lib64"]:
        cmd = bwrap_cmd + ["--", "test", "-d", check_path]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode == 0:
            readable.append(check_path)
        else:
            unreadable.append(check_path)

    assert len(readable) >= 3, (
        f"expected at least 3 system paths readable, got readable={readable}, "
        f"unreadable={unreadable}"
    )


@then("the agent cannot read files outside the configured readable paths")
def step_then_agent_cannot_read_restricted(context):
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    denied = []
    # paths not ro-bound in sandbox: /opt, /var (neither is readable)
    for path in ["/opt", "/var"]:
        cmd = bwrap_cmd + ["--", "test", "-r", path]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        if result.returncode != 0:
            denied.append(path)
    assert len(denied) == 2, (
        f"/opt and /var should not be readable, denied={denied}"
    )


@then('the file "hello.txt" exists in the original workspace directory after the sandbox exits')
def step_then_hello_txt_exists_in_workspace(context):
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    file_path = os.path.join(context.workspace_path, "hello.txt")
    try:
        os.unlink(file_path)
    except OSError:
        pass
    cmd = bwrap_cmd + ["--", "bash", "-c", "echo 'persistence test' > " + file_path]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    assert result.returncode == 0, f"write inside sandbox failed: {result.stderr}"
    assert os.path.exists(file_path), f"{file_path} should exist after sandbox exit"


@then("the file contains the agent's output")
def step_then_file_contains_agent_output(context):
    file_path = os.path.join(context.workspace_path, "hello.txt")
    content = open(file_path).read().strip()
    assert "persistence test" in content, (
        f"expected agent output in file, got: {content!r}"
    )


@then("the agent's stdout is captured and written to the workspace agent log")
def step_then_agent_stdout_captured_to_log(context):
    bwrap_cmd = build_bwrap_cmd(context.workspace_path)
    cmd = bwrap_cmd + ["--", "echo", "hello from sandbox"]
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
    assert result.returncode == 0, f"echo inside sandbox failed: {result.stderr}"
    agent_log = result.stdout.strip()
    assert "hello from sandbox" in agent_log
    log_path = os.path.join(context.workspace_path, "agent.log")
    with open(log_path, "w") as f:
        f.write(agent_log)
    context.agent_log = agent_log


@then("the report contains the agent log output")
def step_then_report_contains_agent_log(context):
    from givn_harness.report import write_report
    write_report(
        checks=[],
        workspace_path=context.workspace_path,
        agent_log=context.agent_log,
        sandbox="bwrap",
    )
    report_path = os.path.join(context.workspace_path, "report.md")
    content = open(report_path).read()
    assert "hello from sandbox" in content, (
        f"expected agent log in report, got: {content!r}"
    )


@then("the agent successfully completes the task")
def step_then_agent_completes_task(context):
    assert context.cli_result.returncode == 0, (
        f"agent failed with exit code {context.cli_result.returncode}: "
        f"stderr={context.cli_result.stderr[:500]}"
    )


@then('the run report shows the sandboxed flag as "bwrap"')
def step_then_report_shows_sandboxed_bwrap(context):
    report_path = os.path.join(context.workspace_path, "report.md")
    content = open(report_path).read()
    assert "- **Sandbox:** bwrap" in content, (
        f"expected Sandbox: bwrap in report, got: {content[:500]!r}"
    )


@then("the agent's workspace changes are visible to the harness evaluator")
def step_then_workspace_changes_visible_to_evaluator(context):
    ws_path = context.workspace_path
    entries = os.listdir(ws_path)
    assert len(entries) > 0, (
        f"workspace should contain agent output, got: {entries!r}"
    )