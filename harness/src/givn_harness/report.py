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
    agent_log: str = "",
    termination: str = "completed",
    sandbox: str = "none",
    resource_limits: str = "none",
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
    lines.append(f"- **Termination:** {termination}")
    lines.append(f"- **Sandbox:** {sandbox}")
    lines.append(f"- **Resource Limits:** {resource_limits}")
    lines.append(f"- **Overall:** {'PASS' if all_pass else 'FAIL'}")
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