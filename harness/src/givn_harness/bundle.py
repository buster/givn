"""Durable trial bundles: manifests, journals, snapshots and checksums."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import threading
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

BUNDLE_SCHEMA = "givn-harness/bundle/v1"
JOURNAL_SCHEMA = "givn-harness/journal/v1"

IGNORED_DIRECTORIES = {
    ".venv",
    "venv",
    "node_modules",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "target",
    ".cache",
    "coverage",
    ".harness",
}

_APPEND_LOCKS: dict[str, threading.Lock] = {}
_LOCKS_GUARD = threading.Lock()


def _lock_for(path: Path) -> threading.Lock:
    key = str(path.resolve())
    with _LOCKS_GUARD:
        lock = _APPEND_LOCKS.get(key)
        if lock is None:
            lock = threading.Lock()
            _APPEND_LOCKS[key] = lock
        return lock


def _stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _atomic_write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(text, encoding="utf-8")
    os.replace(temporary, path)


def create_bundle(
    output_dir: str | Path,
    *,
    condition: str,
    benchmark: dict[str, Any],
    repeat: int = 1,
    extra_manifest: dict[str, Any] | None = None,
) -> Path:
    output = Path(output_dir).resolve()
    output.mkdir(parents=True, exist_ok=True)
    base = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    bundle_id = f"{base}-{condition}-r{repeat}"
    candidate = output / bundle_id
    suffix = 2
    while candidate.exists():
        candidate = output / f"{bundle_id}-{suffix}"
        suffix += 1
    candidate.mkdir()
    manifest: dict[str, Any] = {
        "schema": BUNDLE_SCHEMA,
        "bundle_id": candidate.name,
        "created_at": _stamp(),
        "condition": condition,
        "repeat": repeat,
        "status": "initialized",
        "benchmark": benchmark,
        "stages": [],
        "budget": {"max_usd": None, "reserved_usd": 0.0, "committed_usd": 0.0},
    }
    if extra_manifest:
        manifest.update(extra_manifest)
    _atomic_write(candidate / "manifest.json", json.dumps(manifest, indent=2) + "\n")
    (candidate / "journal.jsonl").write_text("", encoding="utf-8")
    append_journal(candidate, "bundle_created", condition=condition, repeat=repeat)
    return candidate


def load_manifest(bundle: str | Path) -> dict[str, Any]:
    path = Path(bundle) / "manifest.json"
    if not path.is_file():
        raise FileNotFoundError(f"bundle manifest not found: {path}")
    loaded = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(loaded, dict):
        raise ValueError("bundle manifest is not an object")
    if loaded.get("schema") != BUNDLE_SCHEMA:
        raise ValueError(f"unsupported bundle schema: {loaded.get('schema')!r}")
    return loaded


def update_manifest(bundle: str | Path, updates: dict[str, Any]) -> dict[str, Any]:
    path = Path(bundle) / "manifest.json"
    manifest = load_manifest(bundle)
    manifest.update(updates)
    manifest["updated_at"] = _stamp()
    _atomic_write(path, json.dumps(manifest, indent=2) + "\n")
    return manifest


def append_journal(bundle: str | Path, event: str, **payload: Any) -> None:
    path = Path(bundle) / "journal.jsonl"
    path.parent.mkdir(parents=True, exist_ok=True)
    record = {
        "schema": JOURNAL_SCHEMA,
        "at": _stamp(),
        "event": event,
        **payload,
    }
    line = json.dumps(record, ensure_ascii=False, default=str) + "\n"
    with _lock_for(path):
        with path.open("a", encoding="utf-8") as handle:
            handle.write(line)
            handle.flush()
            os.fsync(handle.fileno())


def read_journal(bundle: str | Path) -> list[dict[str, Any]]:
    path = Path(bundle) / "journal.jsonl"
    if not path.is_file():
        return []
    records: list[dict[str, Any]] = []
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except ValueError:
            continue
        if isinstance(record, dict):
            records.append(record)
    return records


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def tree_checksums(root: Path) -> list[dict[str, Any]]:
    files: list[dict[str, Any]] = []
    if not root.is_dir():
        return files
    for path in sorted(root.rglob("*")):
        relative = path.relative_to(root)
        if any(part in IGNORED_DIRECTORIES for part in relative.parts):
            continue
        if path.is_symlink():
            files.append(
                {
                    "path": str(relative),
                    "kind": "symlink",
                    "target": os.readlink(path),
                    "sha256": hashlib.sha256(
                        os.readlink(path).encode("utf-8", "replace")
                    ).hexdigest(),
                }
            )
        elif path.is_file():
            files.append(
                {
                    "path": str(relative),
                    "kind": "file",
                    "bytes": path.stat().st_size,
                    "mode": oct(path.stat().st_mode & 0o777),
                    "sha256": sha256_file(path),
                }
            )
    return files


def _git(workspace: Path, *args: str) -> str:
    try:
        completed = subprocess.run(
            ["git", "-C", str(workspace), *args],
            capture_output=True,
            text=True,
            timeout=30,
        )
    except (OSError, subprocess.TimeoutExpired):
        return ""
    if completed.returncode != 0:
        return ""
    return completed.stdout


def git_metadata(workspace: Path) -> dict[str, Any]:
    status = _git(workspace, "status", "--porcelain=v1", "-uall")
    log = _git(workspace, "log", "--oneline", "--decorate", "-30")
    head = _git(workspace, "rev-parse", "HEAD").strip()
    branch = _git(workspace, "rev-parse", "--abbrev-ref", "HEAD").strip()
    untracked = _git(workspace, "ls-files", "--others", "--exclude-standard")
    return {
        "is_repository": bool(head),
        "head": head,
        "branch": branch,
        "status_clean": not status.strip(),
        "status_porcelain": status.splitlines()[:200],
        "log": log.splitlines()[:50],
        "untracked": untracked.splitlines()[:200],
        "note": (
            "Git history omits uncommitted and untracked files; the snapshot "
            "checksums cover the whole working tree."
        ),
    }


def givn_metadata(workspace: Path) -> dict[str, Any]:
    givn_dir = workspace / "givn"
    archive = givn_dir / "archive"
    changes = givn_dir / "changes"
    archive_entries = sorted(
        entry.name for entry in archive.iterdir()
    ) if archive.is_dir() else []
    change_entries = sorted(
        entry.name for entry in changes.iterdir() if entry.is_dir()
    ) if changes.is_dir() else []
    events = givn_dir / "history" / "events.jsonl"
    return {
        "archive_entries": archive_entries,
        "active_changes": change_entries,
        "events": {
            "path": str(events.relative_to(workspace)) if events.is_file() else None,
            "sha256": sha256_file(events) if events.is_file() else None,
            "bytes": events.stat().st_size if events.is_file() else None,
        },
    }


def _copy_workspace(workspace: Path, destination: Path) -> list[str]:
    ignored: list[str] = []

    def ignore(directory: str, names: list[str]) -> set[str]:
        skip: set[str] = set()
        for name in names:
            if name in IGNORED_DIRECTORIES:
                skip.add(name)
                ignored.append(str(Path(directory, name).relative_to(workspace)))
        return skip

    shutil.copytree(workspace, destination, symlinks=True, ignore=ignore)
    return sorted(ignored)


def freeze_snapshot(
    workspace: str | Path,
    stage_dir: str | Path,
    *,
    stage_id: str,
) -> dict[str, Any]:
    workspace = Path(workspace).resolve()
    stage_dir = Path(stage_dir)
    snapshot_dir = stage_dir / "snapshot"
    if snapshot_dir.exists():
        raise FileExistsError(f"stage snapshot already exists: {snapshot_dir}")
    stage_dir.mkdir(parents=True, exist_ok=True)
    ignored = _copy_workspace(workspace, snapshot_dir)
    files = tree_checksums(snapshot_dir)
    document = {
        "schema": "givn-harness/snapshot/v1",
        "stage": stage_id,
        "frozen_at": _stamp(),
        "source_workspace": str(workspace),
        "file_count": sum(1 for item in files if item["kind"] == "file"),
        "bytes": sum(item.get("bytes", 0) for item in files),
        "files": files,
        "ignored": ignored,
        "git": git_metadata(snapshot_dir),
        "givn": givn_metadata(snapshot_dir),
        "note": (
            "This directory is the authoritative frozen output for the stage. "
            "Grading must not modify it; regrading reads a fresh copy."
        ),
    }
    _atomic_write(stage_dir / "snapshot.json", json.dumps(document, indent=2) + "\n")
    append_journal(
        stage_dir.parent.parent if stage_dir.parent.name == "stages" else stage_dir,
        "snapshot_frozen",
        stage=stage_id,
        path=str(stage_dir / "snapshot"),
        file_count=document["file_count"],
        bytes=document["bytes"],
        snapshot_sha256=hashlib.sha256(
            json.dumps(files, sort_keys=True).encode("utf-8")
        ).hexdigest(),
    )
    return document


def verify_snapshot(stage_dir: str | Path) -> list[str]:
    """Return mismatches between a frozen snapshot and its recorded checksums."""
    stage_dir = Path(stage_dir)
    snapshot_dir = stage_dir / "snapshot"
    record_path = stage_dir / "snapshot.json"
    if not record_path.is_file():
        return ["snapshot.json is missing"]
    document = json.loads(record_path.read_text(encoding="utf-8"))
    recorded = {item["path"]: item for item in document.get("files", [])}
    current = {item["path"]: item for item in tree_checksums(snapshot_dir)}
    problems: list[str] = []
    for path, item in recorded.items():
        other = current.get(path)
        if other is None:
            problems.append(f"missing: {path}")
        elif item.get("sha256") != other.get("sha256"):
            problems.append(f"changed: {path}")
    for path in current:
        if path not in recorded:
            problems.append(f"added: {path}")
    return problems
