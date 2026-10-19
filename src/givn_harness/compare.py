"""Descriptive comparison of saved trial bundles.

No single quality score is produced. Quality, workflow completion and
measurement validity stay separate, and a comparison can be reported as
inconclusive.
"""

from __future__ import annotations

import json
import statistics
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from givn_harness.bundle import load_manifest, read_journal


class CompareError(RuntimeError):
    """The requested bundles cannot be compared as stated."""


@dataclass
class Trial:
    bundle: Path
    manifest: dict[str, Any]
    grade: dict[str, Any] | None
    journal: list[dict[str, Any]] = field(default_factory=list)

    @property
    def condition(self) -> str:
        return str(self.manifest.get("condition") or "unknown")

    @property
    def status(self) -> str:
        return str(self.manifest.get("status") or "unknown")

    @property
    def benchmark(self) -> dict[str, Any]:
        return self.manifest.get("benchmark") or {}

    @property
    def requirements(self) -> dict[str, dict[str, Any]]:
        if not self.grade:
            return {}
        return {item["id"]: item for item in self.grade.get("requirements", [])}

    @property
    def stage_records(self) -> list[dict[str, Any]]:
        return self.manifest.get("stages") or []

    def stage(self, stage_id: str) -> dict[str, Any] | None:
        for stage in self.stage_records:
            if stage.get("id") == stage_id:
                return stage
        return None

    def observed_cost(self) -> tuple[float | None, bool]:
        """Return (sum of stage costs, complete?) from event analysis."""
        total = 0.0
        complete = True
        found = False
        for stage in self.stage_records:
            analysis = stage.get("analysis") or {}
            cost = analysis.get("cost") or {}
            value = cost.get("observed_total_usd")
            if isinstance(value, (int, float)):
                total += float(value)
                found = True
            else:
                complete = False
        return (total if found else None), complete

    def driver_seconds(self) -> float | None:
        values = [
            float(stage["driver"]["duration_seconds"])
            for stage in self.stage_records
            if isinstance((stage.get("driver") or {}).get("duration_seconds"), (int, float))
        ]
        return sum(values) if values else None

    def givn_identity(self) -> dict[str, Any]:
        return self.manifest.get("givn") or {}

    def agent_identity(self) -> dict[str, Any]:
        return self.manifest.get("agent") or {}

    def sandbox_identity(self) -> dict[str, Any]:
        return self.manifest.get("sandbox") or {}

    def benchmark_identity(self) -> dict[str, Any]:
        benchmark = self.benchmark
        return {
            "id": benchmark.get("id"),
            "version": benchmark.get("version"),
            "manifest_sha256": benchmark.get("manifest_sha256"),
        }


def discover_bundles(paths: list[str]) -> list[Path]:
    bundles: list[Path] = []
    for raw in paths:
        path = Path(raw).resolve()
        if (path / "manifest.json").is_file():
            bundles.append(path)
            continue
        if not path.is_dir():
            raise CompareError(f"not a bundle or directory: {path}")
        for child in sorted(path.iterdir()):
            if (child / "manifest.json").is_file():
                bundles.append(child)
    if not bundles:
        raise CompareError("no bundles found")
    return sorted(set(bundles))


def _latest_grade(bundle: Path) -> dict[str, Any] | None:
    grades = bundle / "grades"
    if not grades.is_dir():
        return None
    candidates = sorted(
        entry / "result.json"
        for entry in grades.iterdir()
        if entry.is_dir() and (entry / "result.json").is_file()
    )
    if not candidates:
        return None
    try:
        return json.loads(candidates[-1].read_text(encoding="utf-8"))
    except ValueError:
        return None


def load_trials(paths: list[str]) -> list[Trial]:
    return [
        Trial(bundle=bundle, manifest=load_manifest(bundle), grade=_latest_grade(bundle), journal=read_journal(bundle))
        for bundle in discover_bundles(paths)
    ]


def _distribution(values: list[float]) -> dict[str, Any]:
    if not values:
        return {"count": 0, "median": None, "min": None, "max": None}
    return {
        "count": len(values),
        "median": round(statistics.median(values), 6),
        "min": round(min(values), 6),
        "max": round(max(values), 6),
    }


def comparability(baseline: list[Trial], candidate: list[Trial]) -> dict[str, Any]:
    labels = {
        "benchmark": lambda trial: trial.benchmark_identity(),
        "agent": lambda trial: {
            "driver": trial.agent_identity().get("driver"),
            "model": trial.agent_identity().get("model"),
            "variant": trial.agent_identity().get("variant"),
        },
        "givn": lambda trial: {
            "sha256": trial.givn_identity().get("sha256"),
            "version": trial.givn_identity().get("version"),
        },
        "sandbox": lambda trial: {
            "mode": trial.sandbox_identity().get("mode"),
            "network": trial.sandbox_identity().get("network"),
        },
    }
    differences: list[dict[str, Any]] = []
    for label, accessor in labels.items():
        base_values = {json.dumps(accessor(trial), sort_keys=True) for trial in baseline}
        candidate_values = {json.dumps(accessor(trial), sort_keys=True) for trial in candidate}
        if base_values != candidate_values:
            differences.append(
                {
                    "field": label,
                    "baseline": sorted(base_values),
                    "candidate": sorted(candidate_values),
                    "note": "A difference in an execution condition confounds comparison.",
                }
            )
    return {
        "comparable": not differences,
        "differences": differences,
        "note": (
            "Comparability covers benchmark identity, driver/model, givn identity and "
            "sandbox/network policy. Other environment differences may remain."
        ),
    }


def _condition_summary(trials: list[Trial]) -> dict[str, Any]:
    statuses: dict[str, int] = {}
    for trial in trials:
        statuses[trial.status] = statuses.get(trial.status, 0) + 1
    stage_completion: dict[str, dict[str, int]] = {}
    stage_ids = sorted(
        {stage.get("id") for trial in trials for stage in trial.stage_records if stage.get("id")}
    )
    for stage_id in stage_ids:
        completed = 0
        reached = 0
        for trial in trials:
            stage = trial.stage(stage_id) or {}
            if stage.get("not_reached"):
                continue
            reached += 1
            if stage.get("completed"):
                completed += 1
        stage_completion[stage_id] = {"reached": reached, "completed": completed}
    costs = [trial.observed_cost() for trial in trials]
    cost_values = [value for value, _ in costs if value is not None]
    cost_complete = sum(1 for _, complete in costs if complete)
    seconds = [trial.driver_seconds() for trial in trials if trial.driver_seconds() is not None]
    return {
        "trials": len(trials),
        "statuses": statuses,
        "stage_completion": stage_completion,
        "observed_cost_usd": _distribution(cost_values),
        "cost_complete_trials": cost_complete,
        "driver_seconds": _distribution([float(value) for value in seconds]),
        "fully_graded_trials": sum(
            1 for trial in trials if trial.grade and trial.grade.get("summary", {}).get("all_required_passed")
        ),
        "graded_trials": sum(1 for trial in trials if trial.grade),
    }


def _requirement_table(baseline: list[Trial], candidate: list[Trial]) -> list[dict[str, Any]]:
    ids: dict[str, dict[str, Any]] = {}
    for trial in baseline + candidate:
        for identifier, item in trial.requirements.items():
            ids.setdefault(identifier, item)
    rows: list[dict[str, Any]] = []
    for identifier in sorted(ids):
        row: dict[str, Any] = {
            "id": identifier,
            "description": ids[identifier].get("description", ""),
            "critical": bool(ids[identifier].get("critical")),
        }
        for label, trials in (("baseline", baseline), ("candidate", candidate)):
            counts = {"pass": 0, "fail": 0, "ungraded": 0, "missing": 0}
            for trial in trials:
                item = trial.requirements.get(identifier)
                if item is None:
                    counts["missing"] += 1
                else:
                    counts[item["status"]] = counts.get(item["status"], 0) + 1
            row[label] = counts
        rows.append(row)
    return rows


def build_comparison(baseline_paths: list[str], candidate_paths: list[str]) -> dict[str, Any]:
    baseline = load_trials(baseline_paths)
    candidate = load_trials(candidate_paths)
    if not baseline or not candidate:
        raise CompareError("both baseline and candidate need at least one bundle")
    comparability_result = comparability(baseline, candidate)
    baseline_summary = _condition_summary(baseline)
    candidate_summary = _condition_summary(candidate)
    requirements = _requirement_table(baseline, candidate)
    insufficient = baseline_summary["trials"] < 2 or candidate_summary["trials"] < 2
    inconclusive = (
        insufficient
        or not comparability_result["comparable"]
        or baseline_summary["graded_trials"] == 0
        or candidate_summary["graded_trials"] == 0
    )
    return {
        "schema": "givn-harness/comparison/v1",
        "baseline": {
            "paths": baseline_paths,
            "summary": baseline_summary,
            "trials": [_trial_row(trial) for trial in baseline],
        },
        "candidate": {
            "paths": candidate_paths,
            "summary": candidate_summary,
            "trials": [_trial_row(trial) for trial in candidate],
        },
        "requirements": requirements,
        "comparability": comparability_result,
        "inconclusive": inconclusive,
        "note": (
            "This report is descriptive. It does not collapse quality and resource "
            "use into one score. Fewer than two trials per condition, differing "
            "conditions, missing grades, or ungraded requirements make the result "
            "inconclusive rather than evidence of improvement."
        ),
    }


def _trial_row(trial: Trial) -> dict[str, Any]:
    cost, cost_complete = trial.observed_cost()
    grade_summary = (trial.grade or {}).get("summary") or {}
    return {
        "bundle": str(trial.bundle),
        "condition": trial.condition,
        "status": trial.status,
        "benchmark": trial.benchmark_identity(),
        "givn": trial.givn_identity(),
        "agent": trial.agent_identity(),
        "stages": [
            {
                "id": stage.get("id"),
                "completed": stage.get("completed"),
                "not_reached": stage.get("not_reached", False),
                "driver_termination": (stage.get("driver") or {}).get("termination"),
                "probe_ok": (stage.get("probe") or {}).get("ok"),
                "archive_ok": (stage.get("archive") or {}).get("ok"),
                "observed_cost_usd": ((stage.get("analysis") or {}).get("cost") or {}).get("observed_total_usd"),
            }
            for stage in trial.stage_records
        ],
        "grade": grade_summary,
        "observed_cost_usd": cost,
        "cost_complete": cost_complete,
        "driver_seconds": trial.driver_seconds(),
    }


def render_comparison(document: dict[str, Any]) -> str:
    base = document["baseline"]["summary"]
    cand = document["candidate"]["summary"]
    lines = [
        "# Trial comparison",
        "",
        f"Result: **{'INCONCLUSIVE' if document['inconclusive'] else 'DESCRIPTIVE'}**",
        "",
        "This report describes observed trials. It does not prove causation and does "
        "not combine quality and resource use into one score.",
        "",
        "## Comparability",
        "",
    ]
    if document["comparability"]["comparable"]:
        lines.append("The recorded execution conditions match on the checked fields.")
    else:
        lines.append("The conditions differ; treat the comparison as confounded:")
        lines.append("")
        for difference in document["comparability"]["differences"]:
            lines.append(
                f"- `{difference['field']}`: baseline {difference['baseline']} vs "
                f"candidate {difference['candidate']}"
            )
    lines += [
        "",
        "## Conditions",
        "",
        "| Measure | Baseline | Candidate |",
        "|---|---|---|",
        f"| Trials | {base['trials']} | {cand['trials']} |",
        f"| Statuses | {base['statuses']} | {cand['statuses']} |",
        f"| Trials passing all required | {base['fully_graded_trials']} | {cand['fully_graded_trials']} |",
        f"| Observed cost, median USD | {base['observed_cost_usd']['median']} | {cand['observed_cost_usd']['median']} |",
        f"| Cost complete trials | {base['cost_complete_trials']} | {cand['cost_complete_trials']} |",
        f"| Driver seconds, median | {base['driver_seconds']['median']} | {cand['driver_seconds']['median']} |",
        "",
        "## Stage completion",
        "",
        "| Stage | Baseline reached/completed | Candidate reached/completed |",
        "|---|---|---|",
    ]
    stage_ids = sorted(set(base["stage_completion"]) | set(cand["stage_completion"]))
    for stage_id in stage_ids:
        b = base["stage_completion"].get(stage_id, {"reached": 0, "completed": 0})
        c = cand["stage_completion"].get(stage_id, {"reached": 0, "completed": 0})
        lines.append(
            f"| `{stage_id}` | {b['reached']}/{b['completed']} | {c['reached']}/{c['completed']} |"
        )
    lines += [
        "",
        "## Requirements",
        "",
        "| Requirement | Critical | Baseline pass/fail/ungraded/missing | Candidate pass/fail/ungraded/missing |",
        "|---|---|---|---|",
    ]
    for row in document["requirements"]:
        b = row["baseline"]
        c = row["candidate"]
        lines.append(
            f"| `{row['id']}` {row['description']} | {'yes' if row['critical'] else 'no'} | "
            f"{b['pass']}/{b['fail']}/{b['ungraded']}/{b['missing']} | "
            f"{c['pass']}/{c['fail']}/{c['ungraded']}/{c['missing']} |"
        )
    lines += ["", "## Trials", "", "| Bundle | Condition | Status | Grade | Cost USD | Driver s |", "|---|---|---|---|---|---|"]
    for trial in document["baseline"]["trials"] + document["candidate"]["trials"]:
        grade = trial["grade"]
        grade_text = (
            f"{grade.get('passed', 0)}/{grade.get('graded', 0)} graded"
            if grade
            else "missing"
        )
        lines.append(
            f"| `{Path(trial['bundle']).name}` | {trial['condition']} | {trial['status']} | "
            f"{grade_text} | {trial['observed_cost_usd']} | {trial['driver_seconds']} |"
        )
    lines += [
        "",
        "## Interpretation",
        "",
        "- Fewer than two trials per condition is not a significance test.",
        "- Ungraded requirements mean the trial stopped; they are not passes and not failures.",
        "- External outages, agent failures, budget exhaustion and capture failures stay "
        "visible rather than being collapsed into one exclusion.",
        "- A faster candidate that breaks a requirement is not an unqualified improvement.",
        "",
    ]
    return "\n".join(lines)
