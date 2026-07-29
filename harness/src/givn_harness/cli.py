import os

import click

from givn_harness.evaluator import run_evaluation
from givn_harness.index import update_index
from givn_harness.judge import run_judge
from givn_harness.report import write_report
from givn_harness.runner import check_ralph_binary, run_agent
from givn_harness.sandbox import check_bubblewrap
from givn_harness.types import Check
from givn_harness.workspace import configure_commands, copy_task, create_workspace, init_workspace


@click.group()
def main() -> None:
    pass


@main.command()
@click.option("--task", required=True, type=click.Path(exists=True))
@click.option("--output", required=True, type=click.Path())
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--ralph-bin", default="/usr/local/bin/ralph", type=click.Path())
@click.option("--fail", is_flag=True, default=False, help="Simulate agent failure")
@click.option("--timeout", type=int, default=None, help="Agent execution timeout in seconds")
@click.option("--max-iterations", type=int, default=None, help="Maximum agent iterations")
@click.option("--no-agent-output", is_flag=True, default=False, help="Suppress streaming agent output")
@click.option("--model", type=str, default=None, help="OpenCode model (e.g. openai/gpt-4o)")
def run(task: str, output: str, workspace_root: str, ralph_bin: str, fail: bool, timeout: int | None, max_iterations: int | None, no_agent_output: bool, model: str | None) -> None:
    if not check_ralph_binary(ralph_bin):
        click.echo(f"Error: ralph binary not found at {ralph_bin}", err=True)
        raise SystemExit(1)

    click.echo(f"Creating workspace at {workspace_root} ...")
    workspace_path = create_workspace(workspace_root)

    click.echo("Running git init ...")
    click.echo("Running givn init ...")
    init_workspace(workspace_path)
    configure_commands(workspace_path)
    copy_task(workspace_path, task)

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
            show_output=show_output, model=model,
        )

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