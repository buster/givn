import json
import os
import shutil
import statistics
import time
from pathlib import Path

import click

from givn_harness.index import update_index
from givn_harness.judge import run_judge
from givn_harness.model import AGENTS, DEEPSEEK_FLASH_MAX, require_agent_model
from givn_harness.profile import PROFILES, evaluate
from givn_harness.report import write_report
from givn_harness.runmeta import collect as collect_metadata
from givn_harness.runner import (
    DEFAULT_PROMPT_TEMPLATE,
    _read_blocked,
    check_agent_binary,
    run_agent,
)
from givn_harness.sandbox import check_bubblewrap
from givn_harness.types import Check
from givn_harness.workspace import copy_task, create_workspace, init_workspace

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
@click.option("--agent", type=click.Choice(AGENTS, case_sensitive=False), default=None, help="Agent under test; default opencode")
@click.option("--driver", type=click.Choice(["ralph", "native"]), default="ralph", help="Driver: ralph (opencode only) or the harness's native one-step loop; use native for comparisons")
@click.option("--profile", type=click.Choice(PROFILES), default="full", help="Task acceptance profile: full (archive gates) or smoke (planning artifacts)")
@click.option("--task", default=None, type=click.Path(exists=True), help="Task prompt file")
@click.option("--output", default="reports/", type=click.Path(), help="Report output directory")
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--workspace", default=None, type=click.Path(exists=True), help="Resume an existing workspace")
@click.option("--continue", "continue_", is_flag=True, default=False, help="Resume the last run from .givn-harness-run.json")
@click.option("--ralph-bin", default="/usr/local/bin/ralph", type=click.Path())
@click.option("--fail", is_flag=True, default=False, help="Simulate agent failure")
@click.option("--timeout", type=int, default=None, help="Agent execution timeout in seconds")
@click.option("--max-iterations", type=int, default=None, help="Maximum one-step iterations")
@click.option("--no-agent-output", is_flag=True, default=False, help="Suppress streaming agent output")
@click.option("--model", type=str, default=None, help="Pinned model for the selected agent; other values are refused")
@click.option("--prompt-template", type=click.Path(exists=True), default=None, help="One-step prompt template")
def run(
    task: str | None,
    output: str,
    workspace_root: str,
    workspace: str | None,
    continue_: bool,
    ralph_bin: str,
    fail: bool,
    timeout: int | None,
    max_iterations: int | None,
    no_agent_output: bool,
    agent: str | None,
    driver: str,
    profile: str,
    model: str | None,
    prompt_template: str | None,
) -> None:
    started = time.monotonic()
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
        if agent is None:
            agent = state.get("agent", "opencode")
        if model is None:
            model = state.get("model")
        if prompt_template is None:
            prompt_template = state.get("prompt_template")
        driver = state.get("driver", driver)
        profile = state.get("profile", profile)
        click.echo(f"Resuming run from state file ({RUN_STATE_FILE})")

    agent = (agent or "opencode").lower()
    try:
        model = require_agent_model(agent, model)
    except ValueError as error:
        raise click.ClickException(str(error))

    if not check_agent_binary(agent, ralph_bin, driver):
        if agent == "opencode" and driver == "ralph":
            click.echo(f"Error: ralph binary not found at {ralph_bin}", err=True)
        else:
            click.echo(f"Error: {agent} binary not found", err=True)
        raise SystemExit(1)

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
        init_workspace(workspace_path, agent)
        copy_task(workspace_path, task)

    _save_run_state({
        "workspace": workspace_path,
        "task": task,
        "output": output,
        "agent": agent,
        "driver": driver,
        "profile": profile,
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
            workspace_path, ralph_bin=ralph_bin, timeout=timeout,
            max_iterations=max_iterations, show_output=show_output, model=model,
            prompt_template=prompt_template, agent=agent, driver=driver,
            profile=profile,
        )

        if _check_blocked(workspace_path):
            raise SystemExit(0)

        click.echo("Running automated checks ...")
        checks = evaluate(workspace_path, profile)
        for c in checks:
            result = "PASS" if c.passed else "FAIL"
            click.echo(f"  {c.name}: {result}")

        click.echo("Running LLM judge ...")
        judge_scores = run_judge(workspace_path, model=DEEPSEEK_FLASH_MAX)
        for s in judge_scores:
            click.echo(f"  {s.artifact}: {s.score}/4 - {s.explanation[:80] if s.explanation else ''}")

    prompt_hash_path = prompt_template
    if prompt_hash_path is None and driver == "native":
        prompt_hash_path = str(DEFAULT_PROMPT_TEMPLATE)
    metadata = collect_metadata(
        agent=agent,
        driver=driver,
        profile=profile,
        model=model,
        workspace_path=workspace_path,
        task_path=task,
        prompt_template=prompt_hash_path,
        termination=termination,
        wall_seconds=time.monotonic() - started,
        output_dir=output,
        run_id=run_id,
    )

    written_report = write_report(
        checks=checks,
        workspace_path=workspace_path,
        judge_scores=judge_scores,
        task=task_name,
        agent=agent,
        sandbox="bwrap" if sandboxed else "none",
        termination=termination,
        agent_log=agent_log,
        metadata=metadata,
    )
    # The index names the report under the output directory; keep the workspace
    # copy for debugging and publish the indexed copy.
    Path(report_filename).parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(written_report, report_filename)
    update_index(
        output_dir=output,
        run_id=run_id,
        task=task_name,
        report_path=report_filename,
        overall_pass=all(c.passed for c in checks) if checks else not fail,
        check_pass_count=sum(1 for c in checks if c.passed),
        judge_avg_score=(
            sum(s.score for s in judge_scores) / len(judge_scores)
            if judge_scores else 0.0
        ),
        agent=agent,
        metadata=metadata,
    )
    click.echo(workspace_path)


def _campaign_rows(output: str, agents: list[str], repeats: int) -> list[dict]:
    rows: list[dict] = []
    for agent in agents:
        for repeat in range(1, repeats + 1):
            run_output = Path(output) / f"{agent}-r{repeat}"
            index_path = run_output / "index.json"
            if not index_path.is_file():
                rows.append({"agent": agent, "repeat": repeat, "error": "missing index"})
                continue
            entries = json.loads(index_path.read_text())
            if not entries:
                rows.append({"agent": agent, "repeat": repeat, "error": "empty index"})
                continue
            entry = entries[-1]
            meta = entry.get("metadata", {})
            rows.append({
                "agent": agent,
                "repeat": repeat,
                "run_id": entry.get("run_id"),
                "judge": entry.get("judge_avg_score"),
                "checks": entry.get("check_pass_count"),
                "overall_pass": entry.get("overall_pass"),
                "report": entry.get("report_path"),
                "development_report": meta.get("development_report"),
                "termination": meta.get("termination"),
                "iterations": meta.get("iterations"),
                "wall_seconds": meta.get("wall_seconds"),
                "capture": (meta.get("capture") or {}).get("coverage"),
                "content_events": (meta.get("capture") or {}).get("content_events"),
                "tool_turn_events": (meta.get("capture") or {}).get("tool_turn_events"),
                "turn_content_events": (meta.get("capture") or {}).get("turn_content_events"),
                "turn_events": (meta.get("capture") or {}).get("turn_events"),
                "turn_content": (meta.get("capture") or {}).get("turn_content_coverage"),
                "subagent_starts": (meta.get("capture") or {}).get("subagent_starts"),
                "subagent_tool_events": (meta.get("capture") or {}).get("subagent_tool_events"),
                "subagent_names": (meta.get("capture") or {}).get("subagent_names") or [],
                "model": meta.get("model"),
                "thinking": meta.get("thinking"),
                "profile": meta.get("profile"),
                "driver": meta.get("driver"),
                "task_sha256": meta.get("task_sha256"),
                "prompt_template_sha256": meta.get("prompt_template_sha256"),
                "agent_version": meta.get("agent_version"),
                "givn_version": meta.get("givn_version"),
                "harness_commit": meta.get("harness_commit"),
            })
    return rows


def _campaign_markdown(task: str, profile: str, driver: str, repeats: int, max_iterations: int, timeout: int, rows: list[dict]) -> str:
    lines = [
        f"# Campaign: {Path(task).name} — {profile} profile",
        "",
        f"- Task: `{Path(task).name}`",
        f"- Profile: `{profile}`",
        f"- Driver: `{driver}`",
        f"- Judge: `{DEEPSEEK_FLASH_MAX}`",
        f"- Repeats per agent: {repeats}",
        f"- Budget: {max_iterations} iterations / {timeout}s",
        "",
        "## Runs",
        "",
        "| Agent | Repeat | Judge avg | Checks | Capture | Turn content | Subagents | Iterations | Wall s | Termination | Report | Development |",
        "|---|---|---|---|---|---|---|---|---|---|---|---|",
    ]
    for row in rows:
        if row.get("error"):
            lines.append(f"| {row['agent']} | {row['repeat']} | — | — | — | — | — | — | — | {row['error']} | — | — |")
            continue
        report = Path(row["report"]).name if row.get("report") else ""
        report_link = f"[{report}]({row['agent']}-r{row['repeat']}/{row.get('run_id')}/report.md)"
        dev = row.get("development_report")
        dev_link = f"[timeline]({row['agent']}-r{row['repeat']}/{row.get('run_id')}/development-report.html)" if dev else ""
        capture = row.get("capture")
        capture_text = f"{row.get('content_events', 0)}/{row.get('tool_turn_events', 0)} ({capture:.0%})" if isinstance(capture, (int, float)) else ""
        turn = row.get("turn_content")
        turn_text = f"{row.get('turn_content_events', 0)}/{row.get('turn_events', 0)} ({turn:.0%})" if isinstance(turn, (int, float)) else ""
        sub_names = [item.get("name") for item in (row.get("subagent_names") or []) if item.get("name")]
        subs = f"{row.get('subagent_starts', 0)} rec/{row.get('subagent_tool_events', 0)} tool"
        if sub_names:
            subs += ": " + ", ".join(sub_names[:4])
        judge = row.get('judge') or 0
        lines.append(
            f"| {row['agent']} | {row['repeat']} | {judge:.2f} | "
            f"{row.get('checks', 0)} | {capture_text} | {turn_text} | {subs} | {row.get('iterations') or ''} | "
            f"{row.get('wall_seconds') or ''} | {row.get('termination') or ''} | {report_link} | {dev_link} |"
        )
    lines += ["", "## Aggregate", "", "| Agent | Runs | Judge mean | Judge median | Checks mean | Capture mean |", "|---|---|---|---|---|---|"]
    for agent in sorted({row["agent"] for row in rows}):
        agent_rows = [row for row in rows if row["agent"] == agent and not row.get("error")]
        if not agent_rows:
            lines.append(f"| {agent} | 0 | — | — | — | — |")
            continue
        judges = [row["judge"] for row in agent_rows if isinstance(row.get("judge"), (int, float))]
        checks = [row["checks"] for row in agent_rows if isinstance(row.get("checks"), (int, float))]
        captures = [row["capture"] for row in agent_rows if isinstance(row.get("capture"), (int, float))]
        mean = lambda values: statistics.mean(values) if values else 0.0
        median = lambda values: statistics.median(values) if values else 0.0
        capture_mean = mean(captures)
        lines.append(
            f"| {agent} | {len(agent_rows)} | {mean(judges):.2f} | {median(judges):.2f} | "
            f"{mean(checks):.1f} | {capture_mean:.0%} |"
        )
    lines += [
        "",
        "## Read this carefully",
        "",
        "- The same task, prompt template, profile, judge, budget, and DeepSeek Flash model are used for every agent.",
        "- `opencode` uses the native one-step loop and a private server (`--standalone`); `omp` and `maki` use their native print modes.",
        "- `capture` is the share of tool+turn events with full content. The full-capture check also requires at least 50% of turns to carry assistant or thinking text; below either threshold fails.",
        "- `Subagents` is `recorded subagent spans / subagent-tool calls`; the canonical record query only sees the first number, so both reporters now map `subagent`/`task` tools and the query must count both.",
        "- The private `/tmp` mount exposes only the current workspace; the first campaign's opencode read prior runs and omp read a stray `/tmp` pass marker, so its results were contaminated.",
        "- The task contract in the prompt forbids implementation/archive for the smoke and allows at most one design-review grilling subagent.",
        "- The `thinking` metadata records the reasoning setting: opencode and omp are max; maki exposes no max flag and runs the provider default.",
    ]
    return "\n".join(lines) + "\n"


@main.command()
@click.option("--task", required=True, type=click.Path(exists=True), help="Task prompt file")
@click.option("--output", default="campaigns/", type=click.Path(), help="Campaign output directory")
@click.option("--agents", default="opencode,omp,maki", help="Comma-separated agents")
@click.option("--repeats", type=int, default=1, help="Runs per agent (use 3 for a stable comparison)")
@click.option("--driver", type=click.Choice(["native", "ralph"]), default="native")
@click.option("--profile", type=click.Choice(PROFILES), default="smoke")
@click.option("--max-iterations", type=int, default=10)
@click.option("--timeout", type=int, default=1200)
@click.option("--no-agent-output", is_flag=True, default=False)
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--ralph-bin", default="/usr/local/bin/ralph", type=click.Path())
@click.option("--model", default=None, help="Pinned model for the selected agent; other values are refused")
@click.option("--prompt-template", default=None, type=click.Path(exists=True))
@click.pass_context
def compare(
    ctx: click.Context,
    task: str,
    output: str,
    agents: str,
    repeats: int,
    driver: str,
    profile: str,
    max_iterations: int,
    timeout: int,
    no_agent_output: bool,
    workspace_root: str,
    ralph_bin: str,
    model: str | None,
    prompt_template: str | None,
) -> None:
    os.makedirs(output, exist_ok=True)
    agent_list = [agent.strip().lower() for agent in agents.split(",") if agent.strip()]
    if not agent_list or any(agent not in AGENTS for agent in agent_list):
        raise click.ClickException(f"agents must be drawn from {', '.join(AGENTS)}")
    for agent in agent_list:
        for repeat in range(1, repeats + 1):
            run_output = os.path.join(output, f"{agent}-r{repeat}")
            click.echo(f"\n=== {agent} / repeat {repeat} / {profile} ===")
            try:
                ctx.invoke(
                    run,
                    task=task,
                    output=run_output,
                    workspace_root=workspace_root,
                    workspace=None,
                    continue_=False,
                    ralph_bin=ralph_bin,
                    fail=False,
                    timeout=timeout,
                    max_iterations=max_iterations,
                    no_agent_output=no_agent_output,
                    agent=agent,
                    driver=driver,
                    profile=profile,
                    model=model,
                    prompt_template=prompt_template,
                )
            except SystemExit:
                pass
    rows = _campaign_rows(output, agent_list, repeats)
    markdown = _campaign_markdown(task, profile, driver, repeats, max_iterations, timeout, rows)
    (Path(output) / "campaign.md").write_text(markdown)
    (Path(output) / "campaign.json").write_text(json.dumps({
        "task": task,
        "profile": profile,
        "driver": driver,
        "repeats": repeats,
        "rows": rows,
    }, indent=2) + "\n")
    click.echo(f"\nCampaign report: {Path(output) / 'campaign.md'}")
