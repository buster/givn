from datetime import datetime, timezone
from pathlib import Path

from givn_harness.types import Check, JudgeScore


def generate_report_structure(
    checks: list[Check],
    workspace_path: str,
    judge_scores: list[JudgeScore] | None = None,
) -> dict:
    result: dict = {
        "workspace_path": workspace_path,
        "checks": checks,
        "all_pass": all(c.passed for c in checks),
    }
    if judge_scores is not None:
        result["judge_scores"] = judge_scores
    return result


def write_report(
    checks: list[Check],
    workspace_path: str,
    judge_scores: list[JudgeScore] | None = None,
    task: str = "",
    agent: str = "",
    agent_log: str = "",
    termination: str = "completed",
    sandbox: str = "none",
    resource_limits: str = "none",
    metadata: dict | None = None,
) -> Path:
    report_path = Path(workspace_path) / "report.md"
    run_id = Path(workspace_path).name
    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    all_pass = all(c.passed for c in checks) if checks else True

    lines: list[str] = []

    lines.append(f"# Test Run: {run_id}")
    lines.append("")
    if task:
        lines.append(f"- **Task:** {task}")
    else:
        lines.append("- **Task:** (no task)")
    lines.append(f"- **Timestamp:** {timestamp}")
    lines.append(f"- **Workspace:** {workspace_path}")
    lines.append(f"- **Agent:** {agent or 'unknown'}")
    lines.append(f"- **Termination:** {termination}")
    lines.append(f"- **Sandbox:** {sandbox}")
    lines.append(f"- **Resource Limits:** {resource_limits}")
    lines.append(f"- **Overall:** {'PASS' if all_pass else 'FAIL'}")
    lines.append("")

    if metadata:
        lines.append("## Run Metadata")
        lines.append("")
        lines.append("| Field | Value |")
        lines.append("|---|---|")
        for key, label in [
            ("profile", "Profile"), ("driver", "Driver"), ("model", "Model"),
            ("thinking", "Thinking"), ("judge_model", "Judge model"),
            ("agent_version", "Agent version"), ("givn_version", "Givn version"),
            ("harness_commit", "Harness commit"), ("task_sha256", "Task hash"),
            ("prompt_template_sha256", "Prompt hash"),
            ("iterations", "Iterations"), ("wall_seconds", "Wall seconds"),
            ("capture_mode", "Capture mode"),
            ("development_report", "Development report"),
        ]:
            value = metadata.get(key)
            if value not in (None, "", []):
                lines.append(f"| {label} | {value} |")
        capture = metadata.get("capture") or {}
        if capture:
            lines.append(
                f"| Capture coverage | {capture.get('content_events', 0)}/{capture.get('tool_turn_events', 0)} tool+turn events ({capture.get('coverage', 0):.0%}); "
                f"turn content {capture.get('turn_content_events', 0)}/{capture.get('turn_events', 0)} ({capture.get('turn_content_coverage', 0):.0%}) |"
            )
            names = capture.get("subagent_names") or []
            rendered = "; ".join(
                f"{item.get('name') or 'unnamed'}"
                + (f" ({item.get('agent')})" if item.get("agent") else "")
                + (f", {item.get('duration_ms') / 1000:.1f}s" if isinstance(item.get("duration_ms"), (int, float)) else "")
                for item in names
            )
            lines.append(
                f"| Subagents | {capture.get('subagent_starts', 0)} subagent(s), {capture.get('subagent_events', 0)} recorded event(s), {capture.get('subagent_tool_events', 0)} tool-level launch(es)"
                + (f": {rendered}" if rendered else "")
                + " |"
            )
            lines.append(
                f"| Process | {capture.get('questions', 0)} question, {capture.get('tools', 0)} tool, {capture.get('turns', 0)} turn events; longest {capture.get('longest_ms', 0)} ms |"
            )
        lines.append("")

    lines.append("## Automated Checks")
    lines.append("")
    lines.append("| Check | Result | Detail |")
    lines.append("|---|---|---|")
    for c in checks:
        result = "PASS" if c.passed else "FAIL"
        lines.append(f"| {c.name} | {result} | {c.detail} |")
    lines.append("")

    if judge_scores is not None:
        lines.append("## LLM Judge Scores")
        lines.append("")
        lines.append("| Artifact | Score | Explanation |")
        lines.append("|---|---|---|")
        for s in judge_scores:
            lines.append(f"| {s.artifact} | {s.score} | {s.explanation} |")
        lines.append("")

    lines.append("## Agent Log")
    lines.append("")
    if agent_log:
        lines.append(agent_log)
    else:
        lines.append("(no agent log)")
    lines.append("")

    report_path.write_text("\n".join(lines))
    return report_path