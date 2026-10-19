"""Task profiles and comparable-run metrics.

A profile is the task's acceptance contract: the smoke profile checks the
planning artifact set and the task's stop point; the full profile keeps the
archive/implementation gates. The completion oracle uses the same contract, so
an agent stops when the workspace satisfies the task, not when it claims so.
"""

import json
import re
from pathlib import Path

from givn_harness._utils import find_change_dir
from givn_harness.evaluator import (
    _check_addons_intact,
    _check_artifact_existence,
    _check_design_review_pass,
    _check_review_pass,
    run_evaluation,
)
from givn_harness.types import Check

PROFILES = ("smoke", "full")
EXPECTED_ARTIFACTS = (
    "proposal.md",
    "specs/*.feature",
    "design.md",
    "design-review.md",
    "tasks.md",
    "review.md",
)
ALLOWED_TOP_LEVEL = {
    ".git", ".githooks", ".gitignore", ".maki", ".omp", ".opencode",
    ".ralph", ".ralph-prompt-template.md", ".givn-harness-agent.json",
    ".givn-harness-run.json", "AGENTS.md", "BLOCKED.md", "README.md",
    "agent.log", "development-report.html", "docs", "givn", "report.md",
    "result.txt", "task.md",
}
CAPTURE_MIN_COVERAGE = 0.9
TURN_CONTENT_MIN_COVERAGE = 0.5


def _events(ws: Path) -> list[dict]:
    path = ws / "givn" / "history" / "events.jsonl"
    if not path.is_file():
        return []
    events: list[dict] = []
    for line in path.read_text(errors="replace").splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if isinstance(event, dict):
            events.append(event)
    return events


def capture_metrics(ws: Path) -> dict:
    events = _events(ws)
    counts: dict[str, int] = {}
    content = 0
    tool_turn = 0
    turn_events = 0
    turn_content = 0
    subagent_tools = 0
    subagent_names: dict[str, dict] = {}
    for event in events:
        kind = event.get("kind", "")
        counts[kind] = counts.get(kind, 0) + 1
        data = event.get("data")
        fields = data.get("content") if isinstance(data, dict) else None
        if kind in ("tool", "turn"):
            tool_turn += 1
            if isinstance(fields, dict):
                content += 1
        if kind == "turn":
            turn_events += 1
            if isinstance(fields, dict) and (
                isinstance(fields.get("assistant_text"), dict)
                or isinstance(fields.get("thinking_text"), dict)
            ):
                turn_content += 1
        if kind == "tool" and event.get("name") in ("subagent", "task"):
            subagent_tools += 1
        if kind == "subagent" and isinstance(data, dict):
            raw = event.get("name")
            title = data.get("title") if isinstance(data.get("title"), str) else None
            agent = data.get("agent") if isinstance(data.get("agent"), str) else None
            name = title or raw or agent or "subagent"
            if re.match(r"^(call_|ses_|01[0-9a-f])", str(name)):
                name = title or (f"{agent} subagent" if agent and agent != "subagent" else "subagent")
            identity = data.get("subagent") or event.get("name") or "subagent"
            key = str(identity)
            item = subagent_names.get(key)
            if item is None:
                item = {
                    "name": name,
                    "agent": agent,
                    "id": identity,
                    "session": event.get("session"),
                    "duration_ms": event.get("duration_ms"),
                    "status": data.get("status") or event.get("outcome"),
                }
                subagent_names[key] = item
            else:
                if title:
                    item["name"] = title
                if isinstance(event.get("duration_ms"), (int, float)):
                    item["duration_ms"] = event["duration_ms"]
                status = data.get("status") or event.get("outcome")
                if status and status not in ("unknown", "started"):
                    item["status"] = status
    longest = 0
    for event in events:
        duration = event.get("duration_ms")
        if isinstance(duration, int) and duration > longest:
            longest = duration
    return {
        "events": len(events),
        "content_events": content,
        "tool_turn_events": tool_turn,
        "coverage": round(content / tool_turn, 3) if tool_turn else 0.0,
        "turn_events": turn_events,
        "turn_content_events": turn_content,
        "turn_content_coverage": round(turn_content / turn_events, 3) if turn_events else 0.0,
        "counts": counts,
        "longest_ms": longest,
        "subagents": counts.get("subagent", 0),
        "subagent_tool_events": subagent_tools,
        "subagent_events": counts.get("subagent", 0),
        "subagent_starts": len(subagent_names),
        "subagent_names": list(subagent_names.values()),
        "questions": counts.get("question", 0),
        "tools": counts.get("tool", 0),
        "turns": turn_events,
    }


def _check_task_state(ws: Path) -> Check:
    change = find_change_dir(ws)
    if change is None:
        return Check("one unchecked task", False, "no change directory found")
    tasks = change / "tasks.md"
    if not tasks.is_file():
        return Check("one unchecked task", False, "tasks.md not found")
    text = tasks.read_text(errors="replace")
    unchecked = len(re.findall(r"^\s*- \[ \]", text, re.MULTILINE))
    checked = len(re.findall(r"^\s*- \[[xX]\]", text, re.MULTILINE))
    if unchecked == 1 and checked == 0:
        return Check("one unchecked task", True, "exactly one unchecked task, none checked")
    return Check(
        "one unchecked task", False,
        f"unchecked={unchecked}, checked={checked}; the smoke task expects one unchecked task and no implementation",
    )


def _check_no_production_code(ws: Path) -> Check:
    unexpected = sorted(
        entry.name for entry in ws.iterdir() if entry.name not in ALLOWED_TOP_LEVEL
    )
    if unexpected:
        return Check(
            "no production code", False,
            f"unexpected workspace entries outside the Givn artifacts: {', '.join(unexpected[:8])}",
        )
    return Check("no production code", True, "no production code created")


def _check_capture(ws: Path) -> Check:
    metrics = capture_metrics(ws)
    coverage = metrics["coverage"]
    turn_coverage = metrics["turn_content_coverage"]
    passed = coverage >= CAPTURE_MIN_COVERAGE and turn_coverage >= TURN_CONTENT_MIN_COVERAGE
    return Check(
        "full capture",
        passed,
        f"tool+turn content {metrics['content_events']}/{metrics['tool_turn_events']} ({coverage:.0%}); "
        f"turn content {metrics['turn_content_events']}/{metrics['turn_events']} ({turn_coverage:.0%}); "
        f"subagents {metrics['subagent_starts']} recorded + {metrics['subagent_tool_events']} tool calls",
    )


def smoke_checks(ws: Path) -> list[Check]:
    return [
        _check_artifact_existence(ws),
        _check_design_review_pass(ws),
        _check_review_pass(ws),
        _check_addons_intact(ws),
        _check_task_state(ws),
        _check_no_production_code(ws),
        _check_capture(ws),
    ]


def evaluate(workspace_path: str, profile: str = "full") -> list[Check]:
    if profile == "smoke":
        return smoke_checks(Path(workspace_path).resolve())
    return run_evaluation(workspace_path)


def _markers_present(change: Path) -> bool:
    design_review = change / "design-review.md"
    review = change / "review.md"
    return (
        design_review.is_file()
        and "DESIGN-REVIEW: PASS" in design_review.read_text(errors="replace")
        and review.is_file()
        and "REVIEW: PASS" in review.read_text(errors="replace")
    )


def task_contract(profile: str) -> str:
    if profile == "smoke":
        return (
            "The task's stop point is the artifact set: `proposal.md`, one "
            "`specs/*.feature`, `design.md`, `design-review.md`, `tasks.md`, and "
            "`review.md`. `tasks.md` must contain exactly one unchecked task and "
            "no checked tasks. Create **no production code**. Do NOT run "
            "`givn implement`, `givn archive`, or any implementation/verification "
            "step. If `givn instructions` selects implement or archive, ignore "
            "that next step: write the review artifact and stop. The only "
            "permitted subagent is one design-review grilling subagent; do not "
            "spawn architect, exploration, or verification subagents. Give that "
            "subagent a short descriptive name/title in the tool description "
            "(for example `DesignGrill`); the harness reports the name. Use only "
            "this workspace: do not read other workspaces or search /tmp for "
            "artifacts, templates, or pass markers. The task is complete when "
            "the artifacts and the two PASS markers exist; the harness then "
            "stops on its own."
        )
    return (
        "Continue the Givn lifecycle until the change is archived. Implementation, "
        "verification, review, and archive are allowed; the stop point is a "
        "successful archive with every task checked, tests green, and the review "
        "and design-review markers present."
    )


def task_complete(workspace_path: str, profile: str = "full") -> bool:
    """The task's stop point: artifacts and markers, or an archived change."""
    ws = Path(workspace_path).resolve()
    change = find_change_dir(ws)
    if change is None:
        return False
    if profile == "smoke":
        expected = ["proposal.md", "design.md", "design-review.md", "tasks.md", "review.md"]
        if any(not (change / name).is_file() for name in expected):
            return False
        specs = change / "specs"
        if not specs.is_dir() or not any(specs.rglob("*.feature")):
            return False
        return _markers_present(change)
    archive = ws / "givn" / "archive"
    return archive.is_dir() and any(archive.iterdir())
