import json
import os
from datetime import datetime, timezone
from pathlib import Path


def update_index(
    output_dir: str,
    run_id: str,
    task: str,
    overall_pass: bool = False,
    check_pass_count: int = 0,
    judge_avg_score: float = 0.0,
    report_path: str = "",
    agent: str = "",
    metadata: dict | None = None,
) -> Path:
    os.makedirs(output_dir, exist_ok=True)
    index_path = Path(output_dir) / "index.json"

    existing: list = []
    if index_path.is_file():
        try:
            existing = json.loads(index_path.read_text())
        except (json.JSONDecodeError, ValueError):
            existing = []

    if not isinstance(existing, list):
        existing = []

    timestamp = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    entry = {
        "run_id": run_id,
        "task": task,
        "agent": agent,
        "timestamp": timestamp,
        "overall_pass": overall_pass,
        "check_pass_count": check_pass_count,
        "judge_avg_score": judge_avg_score,
        "report_path": report_path,
        "metadata": metadata or {},
    }

    existing.append(entry)
    index_path.write_text(json.dumps(existing, indent=2) + "\n")
    return index_path