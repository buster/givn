import json
import os

import click

from givn_harness.evaluator import run_evaluation
from givn_harness.index import update_index
from givn_harness.judge import run_judge
from givn_harness.report import write_report
from givn_harness.runner import _read_blocked, check_ralph_binary, run_agent
from givn_harness.sandbox import check_bubblewrap
from givn_harness.types import Check
from givn_harness.workspace import configure_commands, copy_task, create_workspace, init_workspace

RUN_STATE_FILE = ".givn-harness-run.json"


def _save_run_state(state: dict) -> None:
    with open(RUN_STATE_FILE, "w") as f:
        json.dump(state, f, indent=2)


def _load_run_state() -> dict | None:
    if not os.path.isfile(RUN_STATE_FILE):
        return None
    with open(RUN_STATE_FILE) as f:
        return json.load(f)


def _check_blocked(workspace_path: str) -> bool:
    blocked_path = os.path.join(workspace_path, "BLOCKED.md")
    if not os.path.isfile(blocked_path):
        return False
    content = open(blocked_path).read()
    if "## ANSWERED" in content:
        return False
    click.echo("")
    click.echo("=== BLOCKED — agent needs human input ===")
    click.echo("")
    click.echo(content)
    click.echo("")
    click.echo("Edit BLOCKED.md with your answers, add '## ANSWERED' at the top,")
    click.echo("then re-run with:")
    click.echo("")
    click.echo("  uv run givn-harness run --continue")
    return True


@click.group()
def main() -> None:
    pass


@main.command()
@click.option("--task", default=None, type=click.Path(exists=True), help="Task prompt file")
@click.option("--output", default="reports/", type=click.Path(), help="Report output directory")
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--workspace", default=None, type=click.Path(exists=True), help="Resume an existing workspace")
@click.option("--continue", "continue_", is_flag=True, default=False, help="Resume the last run from .givn-harness-run.json")
@click.option("--ralph-bin", default="/usr/local/bin/ralph", type=click.Path())
@click.option("--fail", is_flag=True, default=False, help="Simulate agent failure")
@click.option("--timeout", type=int, default=None, help="Agent execution timeout in seconds")
@click.option("--max-iterations", type=int, default=None, help="Maximum agent iterations")
@click.option("--no-agent-output", is_flag=True, default=False, help="Suppress streaming agent output")
@click.option("--model", type=str, default=None, help="OpenCode model (e.g. openai/gpt-4o)")
@click.option("--prompt-template", type=click.Path(exists=True), default=None, help="Ralph prompt template file")
def run(task: str | None, output: str, workspace_root: str, workspace: str | None, continue_: bool, ralph_bin: str, fail: bool, timeout: int | None, max_iterations: int | None, no_agent_output: bool, model: str | None, prompt_template: str | None) -> None:
    if not check_ralph_binary(ralph_bin):
        click.echo(f"Error: ralph binary not found at {ralph_bin}", err=True)
        raise SystemExit(1)

    if continue_:
        state = _load_run_state()
        if state is None:
            click.echo("Error: no .givn-harness-run.json found. Run without --continue first.", err=True)
            raise SystemExit(1)
        workspace = state.get("workspace")
        if not workspace or not os.path.isdir(workspace):
            click.echo(f"Error: saved workspace {workspace} no longer exists.", err=True)
            raise SystemExit(1)
        task = task or state.get("task")
        output = state.get("output", output)
        if max_iterations is None:
            max_iterations = state.get("max_iterations")
        if model is None:
            model = state.get("model")
        if prompt_template is None:
            prompt_template = state.get("prompt_template")
        click.echo(f"Resuming run from state file ({RUN_STATE_FILE})")

    if task is None:
        click.echo("Error: --task is required for new runs.", err=True)
        raise SystemExit(1)

    if workspace:
        workspace_path = workspace
        click.echo(f"Resuming workspace at {workspace_path}")
    else:
        click.echo(f"Creating workspace at {workspace_root} ...")
        workspace_path = create_workspace(workspace_root)

        click.echo("Running git init ...")
        click.echo("Running givn init ...")
        init_workspace(workspace_path)
        configure_commands(workspace_path)
        copy_task(workspace_path, task)

    _save_run_state({
        "workspace": workspace_path,
        "task": task,
        "output": output,
        "model": model,
        "max_iterations": max_iterations,
        "prompt_template": prompt_template,
    })

    run_id = os.path.basename(workspace_path)
    task_name = os.path.basename(task)
    report_filename = os.path.join(output, run_id, "report.md")

    sandboxed = check_bubblewrap()
    if not sandboxed:
        click.echo("WARNING: bubblewrap is not available, running without sandbox", err=True)

    checks: list[Check] = []
    judge_scores = None
    agent_log = ""

    if fail:
        checks = [
            Check(name="archive succeeded", passed=False, detail="agent failed to archive"),
        ]
        termination = "agent failure"
    else:
        click.echo("Starting agent execution ...")
        show_output = not no_agent_output
        agent_log, termination = run_agent(
            workspace_path, ralph_bin=ralph_bin, timeout=timeout, max_iterations=max_iterations,
            show_output=show_output, model=model, prompt_template=prompt_template,
        )

        if _check_blocked(workspace_path):
            raise SystemExit(0)

        click.echo("Running automated checks ...")
        checks = run_evaluation(workspace_path)
        for c in checks:
            result = "PASS" if c.passed else "FAIL"
            click.echo(f"  {c.name}: {result}")

        click.echo("Running LLM judge ...")
        judge_scores = run_judge(workspace_path, model=model)
        for s in judge_scores:
            click.echo(f"  {s.artifact}: {s.score}/4 - {s.explanation[:80] if s.explanation else ''}")

    write_report(
        checks=checks,
        workspace_path=workspace_path,
        judge_scores=judge_scores,
        sandbox="bwrap" if sandboxed else "none",
        termination=termination,
        agent_log=agent_log,
    )
    update_index(
        output_dir=output,
        run_id=run_id,
        task=task_name,
        report_path=report_filename,
        overall_pass=not fail,
    )
    click.echo(workspace_path)