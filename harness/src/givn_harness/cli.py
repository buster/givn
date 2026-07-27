import os

import click

from givn_harness.index import update_index
from givn_harness.report import write_report
from givn_harness.runner import check_ralph_binary
from givn_harness.sandbox import check_sandlock
from givn_harness.types import Check
from givn_harness.workspace import copy_task, create_workspace, init_workspace


@click.group()
def main():
    pass


@main.command()
@click.option("--task", required=True, type=click.Path(exists=True))
@click.option("--output", required=True, type=click.Path())
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--ralph-bin", default="/usr/local/bin/ralph", type=click.Path())
@click.option("--fail", is_flag=True, default=False, help="Simulate agent failure")
def run(task: str, output: str, workspace_root: str, ralph_bin: str, fail: bool) -> None:
    if not check_ralph_binary(ralph_bin):
        click.echo(f"Error: ralph binary not found at {ralph_bin}", err=True)
        raise SystemExit(1)

    workspace_path = create_workspace(workspace_root)
    init_workspace(workspace_path)
    copy_task(workspace_path, task)

    run_id = os.path.basename(workspace_path)
    task_name = os.path.basename(task)
    report_filename = os.path.join(output, run_id, "report.md")

    sandboxed = check_sandlock()
    if not sandboxed:
        click.echo("WARNING: sandlock is not available, running without sandbox", err=True)

    checks: list[Check] = []
    termination = "completed"

    if fail:
        checks = [
            Check(name="archive succeeded", passed=False, detail="agent failed to archive"),
        ]
        termination = "agent failure"

    write_report(
        checks=checks,
        workspace_path=workspace_path,
        sandboxed=sandboxed,
        termination=termination,
    )
    update_index(
        output_dir=output,
        run_id=run_id,
        task=task_name,
        report_path=report_filename,
        overall_pass=not fail,
    )
    click.echo(workspace_path)