import os

import click

from givn_harness.index import update_index
from givn_harness.workspace import copy_task, create_workspace, init_workspace


@click.group()
def main():
    pass


@main.command()
@click.option("--task", required=True, type=click.Path(exists=True))
@click.option("--output", required=True, type=click.Path())
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
def run(task: str, output: str, workspace_root: str) -> None:
    workspace_path = create_workspace(workspace_root)
    init_workspace(workspace_path)
    copy_task(workspace_path, task)

    run_id = os.path.basename(workspace_path)
    task_name = os.path.basename(task)
    report_filename = os.path.join(output, run_id, "report.md")
    update_index(
        output_dir=output,
        run_id=run_id,
        task=task_name,
        report_path=report_filename,
    )

    click.echo(workspace_path)