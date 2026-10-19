"""Per-run metadata for comparable reports: versions, hashes, capture, time."""

import hashlib
import json
import os
import subprocess
import time
from pathlib import Path

from givn_harness._utils import find_change_dir
from givn_harness.model import AGENT_MODELS, DEEPSEEK_FLASH_MAX
from givn_harness.profile import capture_metrics

GIVN_BINARY = os.environ.get("GIVN_BINARY", "givn")
HARNESS_ROOT = Path(__file__).resolve().parents[2]
THINKING = {
    "opencode": "max (`#max` variant)",
    "omp": "max (`--thinking max`)",
    "maki": "provider default (no max flag exposed)",
}


def command_version(command: str) -> str:
    try:
        result = subprocess.run(
            [command, "--version"], capture_output=True, text=True, timeout=15,
        )
        text = (result.stdout or result.stderr).strip().splitlines()
        return text[0].strip() if text else "unknown"
    except (OSError, subprocess.TimeoutExpired):
        return "unknown"


def harness_commit() -> str:
    try:
        result = subprocess.run(
            ["git", "-C", str(HARNESS_ROOT), "rev-parse", "--short", "HEAD"],
            capture_output=True, text=True, timeout=10,
        )
        commit = result.stdout.strip()
        if not commit:
            return "unknown"
        status = subprocess.run(
            ["git", "-C", str(HARNESS_ROOT), "status", "--porcelain", "--", "harness"],
            capture_output=True, text=True, timeout=10,
        )
        return commit + ("-dirty" if status.stdout.strip() else "")
    except (OSError, subprocess.TimeoutExpired):
        return "unknown"


def sha256_file(path: str | Path | None) -> str:
    if not path:
        return ""
    try:
        data = Path(path).read_bytes()
    except OSError:
        return ""
    return hashlib.sha256(data).hexdigest()[:16]


def read_agent_meta(workspace_path: str) -> dict:
    path = Path(workspace_path) / ".givn-harness-agent.json"
    try:
        return json.loads(path.read_text())
    except (OSError, ValueError):
        return {}


def record_mode(workspace_path: str) -> str:
    try:
        result = subprocess.run(
            [GIVN_BINARY, "history", "settings", "--json"],
            cwd=workspace_path, capture_output=True, text=True, timeout=15,
        )
        return json.loads(result.stdout).get("mode", "unknown")
    except (OSError, ValueError, subprocess.TimeoutExpired):
        return "unknown"


def render_timeline(workspace_path: str, output_dir: str, run_id: str) -> str:
    change = find_change_dir(Path(workspace_path))
    if change is None:
        return ""
    destination = Path(output_dir).resolve() / run_id / "development-report.html"
    destination.parent.mkdir(parents=True, exist_ok=True)
    try:
        subprocess.run(
            [GIVN_BINARY, "timeline", "--change", change.name, "--out", str(destination)],
            cwd=workspace_path, capture_output=True, text=True, timeout=120,
        )
    except (OSError, subprocess.TimeoutExpired):
        return ""
    return str(destination) if destination.is_file() else ""


def collect(
    *,
    agent: str,
    driver: str,
    profile: str,
    model: str,
    workspace_path: str,
    task_path: str,
    prompt_template: str | None,
    termination: str,
    wall_seconds: float,
    output_dir: str,
    run_id: str,
) -> dict:
    metrics = capture_metrics(Path(workspace_path))
    agent_meta = read_agent_meta(workspace_path)
    timeline = render_timeline(workspace_path, output_dir, run_id)
    return {
        "agent": agent,
        "agent_version": command_version(agent),
        "driver": driver,
        "profile": profile,
        "model": model,
        "thinking": THINKING.get(agent, "unknown"),
        "judge_model": DEEPSEEK_FLASH_MAX,
        "givn_version": command_version(GIVN_BINARY),
        "harness_commit": harness_commit(),
        "task": os.path.basename(task_path),
        "task_sha256": sha256_file(task_path),
        "prompt_template_sha256": sha256_file(prompt_template),
        "termination": termination,
        "wall_seconds": round(wall_seconds, 1),
        "iterations": agent_meta.get("iterations"),
        "capture_mode": record_mode(workspace_path),
        "capture": metrics,
        "development_report": timeline,
    }
