import os

import click

from givn_harness.index import update_index
from givn_harness.report import write_report
from givn_harness.types import Check
from givn_harness.workspace import copy_task, create_workspace, init_workspace


@click.group()
def main():
    pass


@main.command()
@click.option("--task", required=True, type=click.Path(exists=True))
@click.option("--output", required=True, type=click.Path())
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--fail", is_flag=True, default=False, help="Simulate agent failure")
def run(task: str, output: str, workspace_root: str, fail: bool) -> None:
    workspace_path = create_workspace(workspace_root)
    init_workspace(workspace_path)
    copy_task(workspace_path, task)

    run_id = os.path.basename(workspace_path)
    task_name = os.path.basename(task)
    report_filename = os.path.join(output, run_id, "report.md")

    if fail:
        checks = [
            Check(name="archive succeeded", passed=False, detail="agent failed to archive"),
        ]
        write_report(
            checks=checks,
            workspace_path=workspace_path,
            termination="agent failure",
        )
        update_index(
            output_dir=output,
            run_id=run_id,
            task=task_name,
            overall_pass=False,
            report_path=report_filename,
        )
        click.echo(workspace_path)
    else:
        update_index(
            output_dir=output,
            run_id=run_id,
            task=task_name,
            report_path=report_filename,
        )
        click.echo(workspace_path)