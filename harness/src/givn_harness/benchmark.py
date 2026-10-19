"""Benchmark manifests and public/private workspace separation."""

from __future__ import annotations

import hashlib
import json
import shutil
import tomllib
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


class BenchmarkError(ValueError):
    """The benchmark manifest or package is invalid."""


@dataclass(frozen=True)
class Stage:
    id: str
    task: Path
    timeout_seconds: int
    require_archive: bool = True
    require_probe: bool = True
    requires: str | None = None
    probe: Path | None = None

    def probe_command(self) -> list[str]:
        if self.probe is not None:
            return ["{python}", self.probe.name, "--workspace", "."]
        return []


@dataclass
class Benchmark:
    root: Path
    manifest_path: Path
    data: dict[str, Any]
    stages: list[Stage]
    public_files: list[Path]
    scaffold: Path
    probe_command: list[str]
    probe_timeout: int
    grader_manifest: Path
    reference: Path | None
    defects: list[dict[str, Any]] = field(default_factory=list)

    @property
    def id(self) -> str:
        return str(self.data["id"])

    @property
    def version(self) -> str:
        return str(self.data["version"])

    @property
    def title(self) -> str:
        return str(self.data.get("title") or self.id)

    def stage(self, stage_id: str) -> Stage:
        for stage in self.stages:
            if stage.id == stage_id:
                return stage
        raise BenchmarkError(f"unknown stage {stage_id!r}")

    def stage_probe_files(self) -> set[Path]:
        return {stage.probe for stage in self.stages if stage.probe is not None}

    def hashes(self) -> dict[str, str]:
        hashes = {"manifest": sha256_file(self.manifest_path)}
        for path in self.public_files:
            hashes[str(path.relative_to(self.root))] = sha256_file(path)
        for stage in self.stages:
            hashes[str(stage.task.relative_to(self.root))] = sha256_file(stage.task)
        hashes[str(self.grader_manifest.relative_to(self.root))] = sha256_file(
            self.grader_manifest
        )
        return hashes

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "version": self.version,
            "title": self.title,
            "root": str(self.root),
            "manifest_sha256": sha256_file(self.manifest_path),
            "stages": [
                {
                    "id": stage.id,
                    "task": str(stage.task.relative_to(self.root)),
                    "timeout_seconds": stage.timeout_seconds,
                    "require_archive": stage.require_archive,
                    "require_probe": stage.require_probe,
                    "probe": str(stage.probe.relative_to(self.root)) if stage.probe else None,
                    "requires": stage.requires,
                }
                for stage in self.stages
            ],
            "hashes": self.hashes(),
        }


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _inside(root: Path, path: Path) -> bool:
    try:
        path.resolve().relative_to(root.resolve())
        return True
    except ValueError:
        return False


def _resolve_public(root: Path, relative: str) -> Path:
    candidate = (root / relative).resolve()
    if not _inside(root, candidate):
        raise BenchmarkError(f"public path escapes the benchmark root: {relative}")
    if not candidate.is_file():
        raise BenchmarkError(f"public file does not exist: {relative}")
    relative_path = candidate.relative_to(root.resolve())
    if relative_path.parts and relative_path.parts[0] == "private":
        raise BenchmarkError(f"public file is under private/: {relative}")
    return candidate


def load_benchmark(path: str | Path) -> Benchmark:
    manifest_path = Path(path)
    if manifest_path.is_dir():
        manifest_path = manifest_path / "benchmark.toml"
    if not manifest_path.is_file():
        raise BenchmarkError(f"benchmark manifest not found: {manifest_path}")
    root = manifest_path.parent.resolve()
    try:
        data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        raise BenchmarkError(f"cannot parse {manifest_path}: {error}") from error
    if data.get("schema") != 1:
        raise BenchmarkError("benchmark schema must be 1")
    if not isinstance(data.get("id"), str) or not data["id"]:
        raise BenchmarkError("benchmark id must be a non-empty string")
    if not isinstance(data.get("version"), str) or not data["version"]:
        raise BenchmarkError("benchmark version must be a non-empty string")

    workspace = data.get("workspace")
    if not isinstance(workspace, dict):
        raise BenchmarkError("benchmark needs a [workspace] table")
    public_entries = workspace.get("public_files")
    if not isinstance(public_entries, list) or not public_entries:
        raise BenchmarkError("workspace.public_files must be a non-empty list")
    public_files = [_resolve_public(root, str(entry)) for entry in public_entries]
    if len({path.name for path in public_files}) != len(public_files):
        raise BenchmarkError("public files must not share a basename")
    scaffold_relative = workspace.get("scaffold")
    if not isinstance(scaffold_relative, str):
        raise BenchmarkError("workspace.scaffold must be a directory path")
    scaffold = (root / scaffold_relative).resolve()
    if not scaffold.is_dir() or not _inside(root, scaffold):
        raise BenchmarkError("workspace.scaffold must be an existing directory under the root")
    if scaffold.relative_to(root).parts[0] == "private":
        raise BenchmarkError("the scaffold must not be under private/")

    probe = workspace.get("probe")
    if not isinstance(probe, dict):
        raise BenchmarkError("workspace.probe must be a table")
    probe_command = probe.get("command")
    if not isinstance(probe_command, list) or not all(
        isinstance(item, str) for item in probe_command
    ):
        raise BenchmarkError("workspace.probe.command must be a list of strings")
    probe_timeout = int(probe.get("timeout_seconds") or 60)

    raw_stages = data.get("stages")
    if not isinstance(raw_stages, list) or not raw_stages:
        raise BenchmarkError("benchmark needs at least one [[stages]] entry")
    stages: list[Stage] = []
    seen: set[str] = set()
    previous: str | None = None
    for entry in raw_stages:
        if not isinstance(entry, dict):
            raise BenchmarkError("each stage must be a table")
        stage_id = entry.get("id")
        if not isinstance(stage_id, str) or not stage_id:
            raise BenchmarkError("stage id must be a non-empty string")
        if stage_id in seen:
            raise BenchmarkError(f"duplicate stage id: {stage_id}")
        seen.add(stage_id)
        task_relative = entry.get("task")
        if not isinstance(task_relative, str):
            raise BenchmarkError(f"stage {stage_id} needs a task path")
        task = _resolve_public(root, task_relative)
        requires = entry.get("requires")
        if requires is not None and requires != previous:
            raise BenchmarkError(
                f"stage {stage_id} requires {requires!r}, but the previous stage is {previous!r}"
            )
        stage_probe = None
        probe_relative = entry.get("probe")
        if probe_relative is not None:
            if not isinstance(probe_relative, str):
                raise BenchmarkError(f"stage {stage_id} probe must be a path")
            stage_probe = _resolve_public(root, probe_relative)
            if stage_probe.name not in {path.name for path in public_files}:
                raise BenchmarkError(
                    f"stage {stage_id} probe must be listed in workspace.public_files"
                )
        stages.append(
            Stage(
                id=stage_id,
                task=task,
                timeout_seconds=int(entry.get("timeout_seconds") or 1200),
                require_archive=bool(entry.get("require_archive", True)),
                require_probe=bool(entry.get("require_probe", True)),
                requires=str(requires) if requires else None,
                probe=stage_probe,
            )
        )
        previous = stage_id

    grader = data.get("grader")
    if not isinstance(grader, dict) or not isinstance(grader.get("manifest"), str):
        raise BenchmarkError("benchmark needs [grader] manifest")
    grader_manifest = (root / grader["manifest"]).resolve()
    if not grader_manifest.is_file() or not _inside(root, grader_manifest):
        raise BenchmarkError(f"grader manifest does not exist: {grader['manifest']}")
    if grader_manifest.relative_to(root).parts[0] != "private":
        raise BenchmarkError("the grader manifest must be under private/")

    verification = data.get("verification") or {}
    reference: Path | None = None
    defects: list[dict[str, Any]] = []
    if verification:
        reference_relative = verification.get("reference")
        if not isinstance(reference_relative, str):
            raise BenchmarkError("verification.reference must be a directory")
        reference = (root / reference_relative).resolve()
        if not reference.is_dir() or not _inside(root, reference):
            raise BenchmarkError("verification.reference must be an existing directory")
        if reference.relative_to(root).parts[0] != "private":
            raise BenchmarkError("verification.reference must be under private/")
        raw_defects = verification.get("defects") or []
        if not isinstance(raw_defects, list):
            raise BenchmarkError("verification.defects must be a list")
        for entry in raw_defects:
            if not isinstance(entry, dict):
                raise BenchmarkError("each defect must be a table")
            defect_id = entry.get("id")
            patch_relative = entry.get("patch")
            expected = entry.get("expected_failures")
            if not isinstance(defect_id, str) or not isinstance(patch_relative, str):
                raise BenchmarkError("defect needs id and patch")
            patch = (root / patch_relative).resolve()
            if not patch.is_file() or not _inside(root, patch):
                raise BenchmarkError(f"defect patch does not exist: {patch_relative}")
            if not isinstance(expected, list) or not expected:
                raise BenchmarkError(f"defect {defect_id} needs expected_failures")
            defects.append(
                {
                    "id": defect_id,
                    "patch": patch,
                    "expected_failures": [str(item) for item in expected],
                }
            )

    return Benchmark(
        root=root,
        manifest_path=manifest_path.resolve(),
        data=data,
        stages=stages,
        public_files=public_files,
        scaffold=scaffold,
        probe_command=probe_command,
        probe_timeout=probe_timeout,
        grader_manifest=grader_manifest,
        reference=reference,
        defects=defects,
    )


def copy_workspace_inputs(benchmark: Benchmark, workspace: Path) -> list[Path]:
    """Copy only explicitly allowlisted public inputs; never private siblings.

    Stage-specific public probes are deliberately withheld until their stage
    ends, so the initial agent cannot read the follow-up check in advance.
    """
    workspace.mkdir(parents=True, exist_ok=True)
    copied: list[Path] = []
    withheld = benchmark.stage_probe_files()
    for source in benchmark.public_files:
        if source in withheld:
            continue
        destination = workspace / source.name
        shutil.copy2(source, destination)
        copied.append(destination)
    for source in sorted(benchmark.scaffold.rglob("*")):
        if not source.is_file():
            continue
        relative = source.relative_to(benchmark.scaffold)
        if any(part in {".git", "__pycache__", ".pytest_cache"} for part in relative.parts):
            continue
        if source.name == ".gitkeep":
            continue
        destination = workspace / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source, destination)
        copied.append(destination)
    return copied


def write_stage_task(benchmark: Benchmark, stage: Stage, workspace: Path) -> Path:
    destination = workspace / "task.md"
    shutil.copy2(stage.task, destination)
    return destination


def public_manifest(benchmark: Benchmark) -> dict[str, Any]:
    return {
        "benchmark": benchmark.to_dict(),
        "public_files": [
            {
                "path": path.name,
                "sha256": sha256_file(path),
                "bytes": path.stat().st_size,
            }
            for path in benchmark.public_files
        ],
        "scaffold_files": [
            str(path.relative_to(benchmark.scaffold))
            for path in sorted(benchmark.scaffold.rglob("*"))
            if path.is_file()
        ],
    }


def dump_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
