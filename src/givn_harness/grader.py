"""Independent grading of frozen trial bundles."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from givn_harness.benchmark import Benchmark, dump_json, load_benchmark, sha256_file
from givn_harness.bundle import append_journal, verify_snapshot

VALID_STATUSES = {"pass", "fail", "ungraded"}


class GraderError(RuntimeError):
    """The grader could not be run or returned an invalid document."""


def _stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")


def _clean_env(extra: dict[str, str] | None = None) -> dict[str, str]:
    allowed = {
        "PATH",
        "LANG",
        "LC_ALL",
        "LC_CTYPE",
        "TZ",
        "TMPDIR",
        "SYSTEMROOT",
    }
    env = {key: value for key, value in os.environ.items() if key in allowed}
    env.setdefault("PATH", "/usr/local/bin:/usr/bin:/bin")
    env["PYTHONHASHSEED"] = "0"
    env["PYTHONDONTWRITEBYTECODE"] = "1"
    env["PYTHONNOUSERSITE"] = "1"
    if extra:
        env.update(extra)
    return env


def _resolve_command(
    command: list[str], benchmark: Benchmark, bundle: Path, output: Path
) -> list[str]:
    sandbox_python = "/usr/bin/python3"
    interpreter = sys.executable if sys.executable.startswith("/usr/") else (
        sandbox_python if Path(sandbox_python).exists() else os.path.realpath(sys.executable)
    )
    replacements = {
        "{python}": interpreter,
        "{benchmark}": str(benchmark.root),
        "{bundle}": str(bundle),
        "{output}": str(output),
    }
    resolved: list[str] = []
    for item in command:
        value = item
        for key, replacement in replacements.items():
            value = value.replace(key, replacement)
        if "{" in value and "}" in value:
            raise GraderError(f"unresolved placeholder in grader command: {value!r}")
        if not os.path.isabs(value):
            candidate = (benchmark.root / value).resolve()
            if candidate.is_file():
                value = str(candidate)
        resolved.append(value)
    return resolved


def load_grader_manifest(benchmark: Benchmark) -> dict[str, Any]:
    import tomllib

    try:
        manifest = tomllib.loads(benchmark.grader_manifest.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        raise GraderError(f"cannot parse grader manifest: {error}") from error
    if manifest.get("schema") != 1:
        raise GraderError("grader manifest schema must be 1")
    command = manifest.get("command")
    if not isinstance(command, list) or not all(isinstance(item, str) for item in command):
        raise GraderError("grader command must be a list of strings")
    return manifest


def validate_grade(document: Any) -> dict[str, Any]:
    if not isinstance(document, dict):
        raise GraderError("grader output is not a JSON object")
    if document.get("schema") != "givn-harness/grade/v1":
        raise GraderError("grader output schema must be givn-harness/grade/v1")
    requirements = document.get("requirements")
    if not isinstance(requirements, list) or not requirements:
        raise GraderError("grader output has no requirements")
    seen: set[str] = set()
    for item in requirements:
        if not isinstance(item, dict):
            raise GraderError("requirement is not an object")
        identifier = item.get("id")
        status = item.get("status")
        if not isinstance(identifier, str) or not identifier:
            raise GraderError("requirement id is missing")
        if identifier in seen:
            raise GraderError(f"duplicate requirement id: {identifier}")
        seen.add(identifier)
        if status not in VALID_STATUSES:
            raise GraderError(f"requirement {identifier} has invalid status {status!r}")
        if not isinstance(item.get("description"), str) or not item["description"]:
            raise GraderError(f"requirement {identifier} has no description")
    summary = document.get("summary")
    if not isinstance(summary, dict):
        raise GraderError("grader output has no summary")
    return document


def invoke_grader(
    bundle: Path,
    benchmark: Benchmark,
    output: Path,
    *,
    sandbox: "SandboxPolicy | None" = None,
    timeout: int | None = None,
) -> tuple[dict[str, Any], str]:
    manifest = load_grader_manifest(benchmark)
    command = _resolve_command(list(manifest["command"]), benchmark, bundle, output)
    command.extend(
        [
            "--bundle",
            str(bundle),
            "--benchmark",
            str(benchmark.root),
            "--output",
            str(output),
        ]
    )
    if sandbox is not None:
        from givn_harness.sandbox import build_command

        command = build_command(sandbox, command)
    if timeout is None:
        timeout = int(manifest.get("timeout_seconds") or 900)
    started = time.monotonic()
    try:
        completed = subprocess.run(
            command,
            cwd=benchmark.root,
            env=_clean_env(),
            capture_output=True,
            text=True,
            timeout=timeout,
        )
    except subprocess.TimeoutExpired as error:
        raise GraderError(f"grader timed out after {timeout}s") from error
    duration = time.monotonic() - started
    log = (
        f"command: {command}\n"
        f"exit_code: {completed.returncode}\n"
        f"duration_seconds: {duration:.3f}\n"
        f"--- stdout ---\n{completed.stdout}\n"
        f"--- stderr ---\n{completed.stderr}\n"
    )
    if completed.returncode != 0:
        raise GraderError(
            f"grader exited {completed.returncode}: "
            f"{(completed.stderr or completed.stdout).strip()[:1200]}"
        )
    if not output.is_file():
        raise GraderError("grader did not write its output document")
    try:
        document = json.loads(output.read_text(encoding="utf-8"))
    except ValueError as error:
        raise GraderError(f"grader output is not JSON: {error}") from error
    return validate_grade(document), log


def grading_sandbox(bundle: Path, benchmark: Benchmark) -> "SandboxPolicy":
    """Build a verified grading sandbox for one bundle.

    The bundle is the only writable mount; the benchmark package is read-only.
    A path-only or unverified fallback is never returned.
    """
    import tempfile

    from givn_harness.sandbox import SandboxPolicy, SandboxUnavailable, probe

    with tempfile.TemporaryDirectory(prefix="givn-grade-capability-") as temporary:
        workspace = Path(temporary) / "workspace"
        workspace.mkdir()
        capability = probe(
            SandboxPolicy(mode="required", network="none", workspace=workspace)
        )
    if not capability.get("ok"):
        failed = ", ".join(
            check["name"] for check in capability.get("checks", []) if not check.get("ok")
        )
        raise SandboxUnavailable(f"grading isolation probe failed: {failed}")
    return SandboxPolicy(
        mode="required",
        network="none",
        workspace=bundle,
        extra_ro_binds=[benchmark.root],
    )


def render_grade_markdown(document: dict[str, Any], benchmark: Benchmark, grade_id: str) -> str:
    summary = document["summary"]
    lines = [
        f"# Grade: {benchmark.title}",
        "",
        f"- Grader: `{document['grader']['id']}` {document['grader']['version']}",
        f"- Grade ID: `{grade_id}`",
        f"- Requirements graded: {summary['graded']}/{summary['required']}",
        f"- Passed: {summary['passed']}; failed: {summary['failed']}; ungraded: {summary['ungraded']}",
        f"- All required passed: **{summary.get('all_required_passed')}**",
    ]
    if summary.get("critical_failures"):
        lines.append(
            "- Critical failures: "
            + ", ".join(f"`{item}`" for item in summary["critical_failures"])
        )
    lines += [
        "",
        "Independent behavioral tests are the primary quality evidence. This "
        "grade does not collapse quality and resource use into one score.",
        "",
        "## Requirements",
        "",
        "| Stage | Requirement | Critical | Result | Evidence |",
        "|---|---|---|---|---|",
    ]
    for item in document["requirements"]:
        evidence = str(item.get("evidence") or "").replace("|", "\\|")[:300]
        lines.append(
            f"| {item.get('stage', '')} | `{item['id']}` {item['description']} | "
            f"{'yes' if item.get('critical') else 'no'} | {item['status']} | {evidence} |"
        )
    lines += [
        "",
        "## Limits",
        "",
        "- The grader sees frozen snapshots, not the agent's development process.",
        "- Requirements are counted once; many assertions for one rule are not many "
        "independent requirements.",
        "- A requirement that was not reached is `ungraded`, not passed and not failed.",
    ]
    if document["requirements"] and any(
        item["status"] == "ungraded" for item in document["requirements"]
    ):
        lines.append(
            "- Ungraded requirements mean the trial is incomplete; they do not prove "
            "that the delivered software would fail them."
        )
    lines.append("")
    return "\n".join(lines)


def grade_bundle(
    bundle: Path,
    benchmark: Benchmark | str | Path,
    *,
    sandbox: "SandboxPolicy | None" = None,
) -> dict[str, Any]:
    if not isinstance(benchmark, Benchmark):
        benchmark = load_benchmark(benchmark)
    bundle = bundle.resolve()
    if not bundle.is_dir():
        raise GraderError(f"bundle not found: {bundle}")
    grades_dir = bundle / "grades"
    grades_dir.mkdir(exist_ok=True)
    grade_id = _stamp()
    grade_dir = grades_dir / grade_id
    grade_dir.mkdir()
    result_path = grade_dir / "result.json"
    verified: dict[str, Any] = {}
    stages_dir = bundle / "stages"
    if stages_dir.is_dir():
        for entry in sorted(stages_dir.iterdir()):
            if not entry.is_dir() or not (entry / "snapshot").is_dir():
                continue
            problems = verify_snapshot(entry)
            verified[entry.name] = {
                "ok": not problems,
                "problems": problems[:50],
            }
            if problems:
                append_journal(
                    bundle,
                    "grading_refused_snapshot_tamper",
                    stage=entry.name,
                    problems=problems[:50],
                )
                raise GraderError(
                    f"snapshot {entry.name!r} does not match its recorded checksums: "
                    f"{problems[:3]}"
                )
    try:
        document, log = invoke_grader(bundle, benchmark, result_path, sandbox=sandbox)
    except GraderError:
        if result_path.exists():
            result_path.unlink()
        raise
    (grade_dir / "grader.log").write_text(log, encoding="utf-8")
    (grade_dir / "grade.md").write_text(
        render_grade_markdown(document, benchmark, grade_id), encoding="utf-8"
    )
    document["grade_id"] = grade_id
    document["graded_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    document["benchmark"] = {
        "id": benchmark.id,
        "version": benchmark.version,
        "manifest_sha256": sha256_file(benchmark.manifest_path),
    }
    document["grader_manifest_sha256"] = sha256_file(benchmark.grader_manifest)
    document["snapshot_verification"] = verified
    dump_json(result_path, document)
    append_journal(
        bundle,
        "grade_finished",
        grade_id=grade_id,
        summary=document["summary"],
        result="grades/" + grade_id + "/result.json",
    )
    return document


# ---------------------------------------------------------------------------
# Benchmark verification: reference, scaffold, and seeded defects
# ---------------------------------------------------------------------------


def _synthetic_bundle(root: Path, sources: dict[str, Path]) -> Path:
    bundle = root / "bundle"
    for stage_id, source in sources.items():
        destination = bundle / "stages" / stage_id / "snapshot"
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copytree(source, destination, symlinks=True)
    bundle.mkdir(parents=True, exist_ok=True)
    dump_json(
        bundle / "manifest.json",
        {
            "schema": "givn-harness/bundle/v1",
            "bundle_id": "verification-" + root.name,
            "synthetic": True,
            "stages": [{"id": stage_id} for stage_id in sources],
        },
    )
    return bundle


def _grade_case(
    benchmark: Benchmark, work_dir: Path, sources: dict[str, Path]
) -> dict[str, Any]:
    bundle = _synthetic_bundle(work_dir, sources)
    output = work_dir / "grade.json"
    document, log = invoke_grader(bundle, benchmark, output)
    (work_dir / "grader.log").write_text(log, encoding="utf-8")
    return document


def _apply_patch(reference: Path, patch: Path, destination: Path) -> None:
    shutil.copytree(reference, destination, symlinks=True)
    if shutil.which("patch") is None:
        raise GraderError("the 'patch' utility is required to verify seeded defects")
    completed = subprocess.run(
        ["patch", "-p1", "--batch", "--forward", "--input", str(patch)],
        cwd=destination,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise GraderError(
            f"defect patch {patch.name} did not apply: "
            f"{(completed.stderr or completed.stdout).strip()[:800]}"
        )


def _case_summary(document: dict[str, Any]) -> dict[str, Any]:
    summary = document["summary"]
    failed = {
        item["id"] for item in document["requirements"] if item["status"] == "fail"
    }
    ungraded = {
        item["id"] for item in document["requirements"] if item["status"] == "ungraded"
    }
    return {
        "graded": summary["graded"],
        "required": summary["required"],
        "passed": summary["passed"],
        "failed": summary["failed"],
        "ungraded": summary["ungraded"],
        "critical_failures": summary.get("critical_failures", []),
        "all_required_passed": summary.get("all_required_passed", False),
        "failed_ids": sorted(failed),
        "ungraded_ids": sorted(ungraded),
    }


def verify_benchmark(benchmark: Benchmark | str | Path, output: Path) -> dict[str, Any]:
    if not isinstance(benchmark, Benchmark):
        benchmark = load_benchmark(benchmark)
    if benchmark.reference is None:
        raise GraderError("benchmark has no [verification] reference")
    output = output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    cases: list[dict[str, Any]] = []
    with tempfile.TemporaryDirectory(prefix="givn-benchmark-verify-") as temporary:
        work = Path(temporary)
        reference_sources = {
            stage.id: benchmark.reference for stage in benchmark.stages
        }
        reference_doc = _grade_case(benchmark, work / "reference", reference_sources)
        reference_summary = _case_summary(reference_doc)
        cases.append(
            {
                "id": "reference",
                "kind": "trusted reference",
                "expectation": "all required requirements pass",
                "expected_failures": [],
                **reference_summary,
                "passed": reference_summary["all_required_passed"],
            }
        )

        scaffold_sources = {stage.id: benchmark.scaffold for stage in benchmark.stages}
        scaffold_doc = _grade_case(benchmark, work / "scaffold", scaffold_sources)
        scaffold_summary = _case_summary(scaffold_doc)
        cases.append(
            {
                "id": "scaffold",
                "kind": "public scaffold",
                "expectation": "substantive requirements fail",
                "expected_failures": ["at least one required requirement"],
                **scaffold_summary,
                "passed": (
                    not scaffold_summary["all_required_passed"]
                    and scaffold_summary["failed"] >= 1
                ),
            }
        )

        for defect in benchmark.defects:
            destination = work / "defects" / defect["id"] / "snapshot"
            _apply_patch(benchmark.reference, defect["patch"], destination)
            sources = {stage.id: destination for stage in benchmark.stages}
            document = _grade_case(
                benchmark, work / "defects" / defect["id"], sources
            )
            summary = _case_summary(document)
            expected = set(defect["expected_failures"])
            actual = set(summary["failed_ids"])
            missing = sorted(expected - actual)
            unexpected = sorted(actual - expected)
            cases.append(
                {
                    "id": defect["id"],
                    "kind": "seeded defect",
                    "expectation": "named requirements fail",
                    "expected_failures": sorted(expected),
                    "missing_expected_failures": missing,
                    "unexpected_failures": unexpected,
                    **summary,
                    "passed": not missing,
                }
            )

    document = {
        "schema": "givn-harness/benchmark-verification/v1",
        "benchmark": {"id": benchmark.id, "version": benchmark.version},
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "cases": cases,
        "passed": all(case["passed"] for case in cases),
        "note": (
            "The reference must pass all required checks; the public scaffold must "
            "fail substantive checks; each seeded defect must trip its named checks."
        ),
    }
    dump_json(output / "verification.json", document)
    (output / "verification.md").write_text(
        render_verification_markdown(document), encoding="utf-8"
    )
    return document


def render_verification_markdown(document: dict[str, Any]) -> str:
    lines = [
        f"# Benchmark verification: {document['benchmark']['id']}",
        "",
        f"Overall: **{'PASS' if document['passed'] else 'FAIL'}**",
        "",
        "| Case | Kind | Expected | Failed | Missing expected | Unexpected | Result |",
        "|---|---|---|---|---|---|---|",
    ]
    for case in document["cases"]:
        lines.append(
            f"| `{case['id']}` | {case['kind']} | {case['expectation']} | "
            f"{', '.join(case.get('failed_ids') or []) or '—'} | "
            f"{', '.join(case.get('missing_expected_failures') or []) or '—'} | "
            f"{', '.join(case.get('unexpected_failures') or []) or '—'} | "
            f"{'PASS' if case['passed'] else 'FAIL'} |"
        )
    lines += [
        "",
        "The reference passing and every seeded defect being caught together show "
        "that the grader measures delivered behavior rather than artifact presence.",
        "",
    ]
    return "\n".join(lines)
