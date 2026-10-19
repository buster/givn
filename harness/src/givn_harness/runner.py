"""Staged benchmark execution with durable bundles and explicit budgets."""

from __future__ import annotations

import json
import os
import shlex
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from givn_harness import __version__
from givn_harness.accounting import analyze_paths
from givn_harness.analysis_report import render_analysis
from givn_harness.benchmark import Benchmark, dump_json, load_benchmark, sha256_file, write_stage_task
from givn_harness.bundle import (
    append_journal,
    create_bundle,
    freeze_snapshot,
    load_manifest,
    update_manifest,
)
from givn_harness.budget import (
    BudgetError,
    MissingPrice,
    SpendLedger,
    estimate_request_usd,
    load_prices,
)
from givn_harness.driver import DriverResult, DriverSpec, build_driver_prompt, run_driver
from givn_harness.sandbox import SandboxPolicy, SandboxUnavailable, probe
from givn_harness.workspace import WorkspaceError, initialize_workspace


class RunError(RuntimeError):
    """A trial cannot be executed under the requested policy."""


@dataclass
class RunConfig:
    benchmark: Benchmark
    output: Path
    condition: str = "current"
    repeats: int = 1
    driver: DriverSpec = field(
        default_factory=lambda: DriverSpec(kind="opencode", model="deepseek/deepseek-flash#max")
    )
    givn_binary: Path | None = None
    sandbox_mode: str = "required"
    network: str = "none"
    workspace_root: Path = Path("/tmp/givn-harness-workspaces")
    budget_usd: float | None = None
    price_table: Path | None = None
    max_output_tokens: int = 8192
    analyze_stages: bool = True
    answer_policy: str = "recommended"
    sandbox_extra_ro_binds: list[Path] = field(default_factory=list)
    sandbox_credentials: list[Path] = field(default_factory=list)
    resume: Path | None = None
    allow_unverified_network: bool = False
    stop_after: str | None = None
    stage_timeout_override: int | None = None
    secret_env: dict[str, str] = field(default_factory=dict)

    def driver_extra_sandbox(self) -> dict[str, Any]:
        return {
            "network_allowlist": (
                "unverified" if self.network == "host" else "none"
            ),
            "paid_execution_requires_reconciled_usage": True,
            "execution_class": (
                "exploratory"
                if self.network == "host" and self.allow_unverified_network
                else "controlled"
            ),
        }


def _stamp() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def _command_version(command: str | Path) -> str:
    try:
        completed = subprocess.run(
            [str(command), "--version"], capture_output=True, text=True, timeout=20
        )
    except (OSError, subprocess.TimeoutExpired):
        return "unknown"
    text = (completed.stdout or completed.stderr).strip().splitlines()
    return text[0].strip() if text else "unknown"


def _git_revision() -> dict[str, Any]:
    root = Path(__file__).resolve().parents[2]
    try:
        commit = subprocess.run(
            ["git", "-C", str(root), "rev-parse", "--short", "HEAD"],
            capture_output=True,
            text=True,
            timeout=10,
        ).stdout.strip()
        status = subprocess.run(
            ["git", "-C", str(root), "status", "--porcelain"],
            capture_output=True,
            text=True,
            timeout=10,
        ).stdout
    except (OSError, subprocess.TimeoutExpired):
        return {"revision": "unknown", "dirty": None}
    return {
        "revision": commit or "unknown",
        "dirty": bool(status.strip()),
    }


def _sandbox_policy(
    config: RunConfig,
    workspace: Path,
    *,
    env_override: dict[str, str] | None = None,
) -> SandboxPolicy:
    env = {
        "PYTHONUNBUFFERED": "1",
        **(env_override if env_override is not None else config.driver.extra_env),
    }
    extra_ro_binds = list(config.sandbox_extra_ro_binds)
    credential_binds = list(config.sandbox_credentials)
    if config.givn_binary is not None:
        binary = Path(config.givn_binary).resolve()
        extra_ro_binds.append(binary)
        env["PATH"] = f"{binary.parent}:" + env.get(
            "PATH", "/usr/local/bin:/usr/bin:/bin"
        )
    if config.driver.kind == "opencode":
        home = Path.home()
        # Only the runtime. Config, cache, data and state are private per trial,
        # so host plugins, sessions and the credentials database are not read.
        runtime = home / ".opencode"
        if runtime.exists():
            extra_ro_binds.append(runtime)
        env["PATH"] = (
            f"{home / '.opencode' / 'bin'}:"
            + env.get("PATH", "/usr/local/bin:/usr/bin:/bin")
        )
    return SandboxPolicy(
        mode=config.sandbox_mode,
        network=config.network,
        workspace=workspace,
        extra_ro_binds=extra_ro_binds,
        credential_binds=credential_binds,
        env=env,
    )


def prepare_opencode_workspace(
    workspace: Path,
    stage_dir: Path,
    model: str,
) -> dict[str, str]:
    """Create a private XDG tree and a provider/model guard for opencode.

    The host OpenCode config, plugins, cache and credential database are not
    mounted. The sandbox receives only this config and the API key from its
    environment.
    """
    base = workspace / ".harness" / "opencode"
    config_dir = base / "config" / "opencode"
    data_dir = base / "data"
    cache_dir = base / "cache"
    state_dir = base / "state"
    for path in (config_dir, data_dir, cache_dir, state_dir):
        path.mkdir(parents=True, exist_ok=True)
    config = {
        "$schema": "https://opencode.ai/config.json",
        "model": model,
        "snapshots": False,
        "policies": [
            {"action": "provider.use", "resource": "*", "effect": "deny"},
            {"action": "provider.use", "resource": "deepseek", "effect": "allow"},
        ],
    }
    config_path = config_dir / "opencode.json"
    config_path.write_text(json.dumps(config, indent=2) + "\n", encoding="utf-8")
    shutil.copy2(config_path, stage_dir / "opencode.json")
    return {
        "XDG_CONFIG_HOME": str(config_dir.parent),
        "XDG_DATA_HOME": str(data_dir),
        "XDG_CACHE_HOME": str(cache_dir),
        "XDG_STATE_HOME": str(state_dir),
    }


def prepare_secret_env(workspace: Path, env: dict[str, str]) -> dict[str, str]:
    """Move credential values out of argv into a mode-600 workspace file."""
    secret_keys = {"DEEPSEEK_API_KEY"}
    secrets = {key: value for key, value in env.items() if key in secret_keys}
    if not secrets:
        return dict(env)
    secret_path = workspace / ".harness" / "secrets.env"
    secret_path.parent.mkdir(parents=True, exist_ok=True)
    lines = [f"export {key}={shlex.quote(value)}" for key, value in secrets.items()]
    secret_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    secret_path.chmod(0o600)
    cleaned = {key: value for key, value in env.items() if key not in secret_keys}
    cleaned["HARNESS_SECRETS_FILE"] = str(secret_path)
    return cleaned


def remove_secret_env(workspace: Path) -> None:
    path = workspace / ".harness" / "secrets.env"
    try:
        path.unlink()
    except FileNotFoundError:
        pass


def _initial_probe(config: RunConfig) -> dict[str, Any]:
    import tempfile

    with tempfile.TemporaryDirectory(prefix="givn-sandbox-capability-") as temporary:
        workspace = Path(temporary) / "workspace"
        workspace.mkdir()
        policy = SandboxPolicy(
            mode=config.sandbox_mode,
            network=config.network,
            workspace=workspace,
            extra_ro_binds=config.sandbox_extra_ro_binds,
            credential_binds=config.sandbox_credentials,
            env={"PYTHONUNBUFFERED": "1", **config.driver.extra_env},
        )
        return probe(policy)


def _manifest_details(config: RunConfig, sandbox_result: dict[str, Any]) -> dict[str, Any]:
    givn_info: dict[str, Any] = {
        "binary": str(config.givn_binary) if config.givn_binary else None,
        "sha256": sha256_file(config.givn_binary) if config.givn_binary else None,
        "version": _command_version(config.givn_binary) if config.givn_binary else None,
    }
    price_info = None
    if config.price_table is not None:
        try:
            prices = load_prices(config.price_table)
            price_info = {"path": str(config.price_table), **prices.to_dict()}
        except MissingPrice as error:
            price_info = {"path": str(config.price_table), "error": str(error)}
    return {
        "harness": {
            "version": __version__,
            **_git_revision(),
            "pyproject_sha256": sha256_file(Path(__file__).resolve().parents[2] / "pyproject.toml"),
        },
        "condition": config.condition,
        "benchmark": config.benchmark.to_dict(),
        "givn": givn_info,
        "agent": {
            "driver": config.driver.kind,
            "model": config.driver.model,
            "variant": "max" if config.driver.model.endswith("#max") else "default",
            "command": config.driver.command,
        },
        "sandbox": {**sandbox_result, **config.driver_extra_sandbox()},
        "budget": {
            "max_usd": config.budget_usd,
            "price_table": price_info,
            "ledger": "spend.json",
        },
        "workspace_root": str(config.workspace_root),
        "answer_policy": config.answer_policy,
        "network": config.network,
        "stop_after": config.stop_after,
        "stage_timeout_override": config.stage_timeout_override,
    }


def _probe_stage(
    benchmark: Benchmark, stage, workspace: Path, stage_dir: Path
) -> dict[str, Any]:
    command_template = stage.probe_command() or benchmark.probe_command
    if stage.probe is not None:
        destination = workspace / stage.probe.name
        shutil.copy2(stage.probe, destination)
    command = [
        item.replace("{python}", sys.executable).replace("{workspace}", str(workspace))
        for item in command_template
    ]
    if any("{" in item and "}" in item for item in command):
        return {
            "ok": False,
            "command": command,
            "error": "unresolved placeholder in probe command",
        }
    started = time.monotonic()
    try:
        completed = subprocess.run(
            command,
            cwd=workspace,
            capture_output=True,
            text=True,
            timeout=benchmark.probe_timeout,
            env={
                "PATH": "/usr/local/bin:/usr/bin:/bin",
                "HOME": os.environ.get("HOME", "/tmp"),
                "LANG": "C.UTF-8",
                "LC_ALL": "C.UTF-8",
                "PYTHONDONTWRITEBYTECODE": "1",
            },
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        result = {
            "ok": False,
            "command": command,
            "error": f"{type(error).__name__}: {error}",
            "duration_seconds": round(time.monotonic() - started, 3),
        }
    else:
        result = {
            "ok": completed.returncode == 0,
            "command": command,
            "exit_code": completed.returncode,
            "duration_seconds": round(time.monotonic() - started, 3),
            "stdout": completed.stdout[-4000:],
            "stderr": completed.stderr[-4000:],
        }
    dump_json(stage_dir / "probe.json", result)
    return result


def _archive_entries(workspace: Path) -> set[str]:
    archive = workspace / "givn" / "archive"
    if not archive.is_dir():
        return set()
    return {entry.name for entry in archive.iterdir() if entry.is_dir()}


def _archive_check(before: set[str], workspace: Path, required: bool) -> dict[str, Any]:
    after = _archive_entries(workspace)
    new_entries = sorted(after - before)
    if not required:
        return {
            "required": False,
            "ok": True,
            "new_entries": new_entries,
            "note": "archive completion is not required for this run",
        }
    return {
        "required": True,
        "ok": bool(new_entries),
        "new_entries": new_entries,
        "before": sorted(before),
        "after": sorted(after),
        "note": (
            "Only an archive entry created during this stage counts; a pre-existing "
            "archive is not completion evidence."
        ),
    }


def _analyze_stage(workspace: Path, stage_dir: Path) -> dict[str, Any] | None:
    source = workspace / "givn" / "history" / "events.jsonl"
    if not source.is_file():
        return None
    copied = stage_dir / "events.jsonl"
    shutil.copy2(source, copied)
    try:
        analysis = analyze_paths([str(copied)])
    except Exception as error:  # noqa: BLE001 - analysis must not erase the trial
        dump_json(
            stage_dir / "analysis-error.json",
            {"error": f"{type(error).__name__}: {error}"},
        )
        return None
    document = analysis.to_dict()
    document["created_at"] = _stamp()
    dump_json(stage_dir / "analysis.json", document)
    (stage_dir / "analysis.md").write_text(render_analysis(analysis), encoding="utf-8")
    return {
        "events": analysis.data["corpus"]["events"],
        "diagnostics": len(analysis.diagnostics),
        "findings": len(analysis.findings),
        "cost": analysis.data["cost"],
        "usage": analysis.data["usage"],
    }


def _prompt_for_estimate(
    config: RunConfig, workspace: Path, stage_id: str
) -> str:
    task_path = workspace / "task.md"
    task_text = task_path.read_text(encoding="utf-8", errors="replace") if task_path.is_file() else ""
    return build_driver_prompt(
        stage_id,
        task_text,
        answer_policy=config.answer_policy,
        workspace=workspace,
    )


def _paid_driver(config: RunConfig) -> bool:
    return config.driver.kind == "opencode"


def _validate_config(config: RunConfig) -> None:
    if config.repeats < 1:
        raise RunError("repeats must be at least 1")
    if config.driver.kind == "opencode":
        if config.givn_binary is None:
            raise RunError("the opencode driver requires --givn-binary for a givn trial")
        if config.budget_usd is None:
            raise RunError(
                "paid runs require --budget-usd; no request is dispatched without a cap"
            )
        if config.price_table is None:
            raise RunError(
                "paid runs require --price-table with a dated price source; "
                "no price is invented"
            )
        if config.network == "none":
            raise RunError(
                "the opencode driver needs provider network access; use "
                "--network host with --allow-unverified-network for an "
                "exploratory run"
            )
        if config.network == "host" and not config.allow_unverified_network:
            raise RunError(
                "host networking is not a verified egress allowlist; pass "
                "--allow-unverified-network to run an explicitly exploratory trial"
            )
        if not config.secret_env.get("DEEPSEEK_API_KEY"):
            raise RunError(
                "the opencode driver requires DEEPSEEK_API_KEY in the sandbox "
                "environment; use --deepseek-api-key-file"
            )
        if config.sandbox_mode != "required":
            raise RunError(
                "the opencode driver requires sandbox mode 'required'; "
                "an unconfined paid run is not a graded fallback"
            )
    if config.stop_after is not None and config.stop_after not in {
        stage.id for stage in config.benchmark.stages
    }:
        raise RunError(f"unknown --stop-after stage: {config.stop_after!r}")
    if config.stage_timeout_override is not None and config.stage_timeout_override <= 0:
        raise RunError("--stage-timeout must be positive")
    if config.givn_binary is not None and not Path(config.givn_binary).is_file():
        raise RunError(f"givn binary not found: {config.givn_binary}")


def execute(config: RunConfig) -> list[Path]:
    _validate_config(config)
    benchmark = config.benchmark
    sandbox_result = _initial_probe(config)
    if config.sandbox_mode == "required" and not sandbox_result.get("ok"):
        failed = ", ".join(
            check["name"] for check in sandbox_result.get("checks", []) if not check.get("ok")
        )
        raise SandboxUnavailable(f"isolation probe failed: {failed}")
    if config.resume is not None:
        bundle = Path(config.resume).resolve()
        if not bundle.is_dir():
            raise RunError(f"bundle to resume does not exist: {bundle}")
        append_journal(bundle, "resume_requested")
        run_bundle(bundle, config, sandbox_result)
        return [bundle]

    bundles: list[Path] = []
    for repeat in range(1, config.repeats + 1):
        bundle = create_bundle(
            config.output,
            condition=config.condition,
            benchmark=benchmark.to_dict(),
            repeat=repeat,
            extra_manifest=_manifest_details(config, sandbox_result),
        )
        run_bundle(bundle, config, sandbox_result)
        bundles.append(bundle)
    return bundles


def _stage_workflow_complete(stage: dict[str, Any]) -> bool:
    """A stage is workflow-complete when its public gate and archive passed.

    The driver exit code is recorded separately. A nonzero exit after a
    completed delivery must not erase the archive and probe evidence.
    """
    if stage.get("completed"):
        return True
    probe_ok = bool((stage.get("probe") or {}).get("ok"))
    archive_ok = bool((stage.get("archive") or {}).get("ok"))
    protocol_ok = not (stage.get("driver") or {}).get("protocol_failures")
    return probe_ok and archive_ok and protocol_ok


def run_bundle(
    bundle: Path, config: RunConfig, sandbox_result: dict[str, Any] | None = None
) -> dict[str, Any]:
    manifest = load_manifest(bundle)
    if manifest.get("status") == "completed":
        return manifest
    if sandbox_result is None:
        sandbox_result = _initial_probe(config)
    update_manifest(bundle, {"status": "running"})
    append_journal(bundle, "run_started", condition=config.condition)

    workspace_value = manifest.get("workspace")
    init_info: dict[str, Any] | None = None
    if workspace_value and Path(workspace_value).is_dir():
        workspace = Path(workspace_value)
        append_journal(bundle, "workspace_reused", workspace=str(workspace))
    else:
        try:
            workspace, init_info = initialize_workspace(
                config.benchmark,
                config.workspace_root,
                label=f"{config.condition}-{config.benchmark.id}",
                givn_binary=config.givn_binary,
                agent="opencode",
            )
        except (WorkspaceError, OSError) as error:
            update_manifest(bundle, {"status": "failed", "blocker": str(error)})
            append_journal(bundle, "workspace_failed", error=str(error))
            raise RunError(f"workspace initialization failed: {error}") from error
        manifest = update_manifest(
            bundle,
            {
                "workspace": str(workspace),
                "workspace_initialized": init_info,
                "status": "running",
            },
        )
        append_journal(bundle, "workspace_initialized", **init_info)

    stages_done = {entry["id"] for entry in manifest.get("stages", []) if entry.get("completed")}
    if config.resume is not None:
        normalized: list[str] = []
        normalized_stages = list(manifest.get("stages", []))
        for entry in normalized_stages:
            if (
                not entry.get("completed")
                and not entry.get("not_reached")
                and _stage_workflow_complete(entry)
            ):
                entry["completed"] = True
                entry["completion_basis"] = (
                    "normalized on resume: public probe and workflow archive passed; "
                    "driver exit is reported separately"
                )
                entry["normalized_after_run"] = True
                normalized.append(str(entry.get("id")))
        if normalized:
            manifest = update_manifest(bundle, {"stages": normalized_stages})
            append_journal(
                bundle,
                "stage_completion_normalized",
                stages=normalized,
                basis="probe+archive",
            )
            stages_done = {
                entry["id"] for entry in normalized_stages if entry.get("completed")
            }
        blocking = [
            entry
            for entry in normalized_stages
            if not entry.get("not_reached") and not entry.get("completed")
        ]
        if blocking:
            raise RunError(
                "cannot resume an incomplete stage; inspect the preserved snapshot and "
                "start a new trial instead"
            )

    ledger: SpendLedger | None = None
    budget = manifest.get("budget") or {}
    if budget.get("max_usd") and _paid_driver(config):
        ledger = SpendLedger(bundle / "spend.json", float(budget["max_usd"]))

    completed_stages: list[dict[str, Any]] = [
        entry for entry in manifest.get("stages", []) if not entry.get("not_reached")
    ]
    all_completed = True
    previous_completed = True
    for stage in config.benchmark.stages:
        if stage.id in stages_done:
            append_journal(bundle, "stage_skipped", stage=stage.id, reason="already completed")
            continue
        stage_dir = bundle / "stages" / stage.id
        if stage_dir.exists():
            not_reached_path = stage_dir / "not-reached.json"
            if (
                config.resume is not None
                and not_reached_path.is_file()
                and not (stage_dir / "snapshot").exists()
            ):
                not_reached_path.rename(stage_dir / "not-reached-before-resume.json")
                append_journal(
                    bundle,
                    "stage_resumed",
                    stage=stage.id,
                    note="replaced a previous not-reached marker",
                )
            else:
                raise RunError(
                    f"stage directory exists without a completed record: {stage_dir}; "
                    "refusing to overwrite evidence"
                )
        if stage.requires and not previous_completed:
            stage_dir.mkdir(parents=True, exist_ok=True)
            dump_json(
                stage_dir / "not-reached.json",
                {
                    "stage": stage.id,
                    "reason": f"required stage {stage.requires!r} did not complete",
                    "at": _stamp(),
                },
            )
            append_journal(bundle, "stage_not_reached", stage=stage.id, requires=stage.requires)
            completed_stages.append(
                {"id": stage.id, "completed": False, "not_reached": True}
            )
            all_completed = False
            continue

        stage_dir.mkdir(parents=True, exist_ok=True)
        archive_before = _archive_entries(workspace)
        task_path = write_stage_task(config.benchmark, stage, workspace)
        shutil.copy2(task_path, stage_dir / "request.md")
        stage_timeout = config.stage_timeout_override or stage.timeout_seconds
        request = {
            "stage": stage.id,
            "task": str(stage.task),
            "task_sha256": sha256_file(stage.task),
            "timeout_seconds": stage_timeout,
            "benchmark_timeout_seconds": stage.timeout_seconds,
            "timeout_override": config.stage_timeout_override,
            "require_archive": stage.require_archive,
            "require_probe": stage.require_probe,
            "started_at": _stamp(),
        }
        dump_json(stage_dir / "request.json", request)
        append_journal(bundle, "stage_started", stage=stage.id, request=request)

        if config.driver.kind == "opencode":
            xdg_env = prepare_opencode_workspace(
                workspace, stage_dir, config.driver.model
            )
            stage_env = {
                **config.driver.extra_env,
                **xdg_env,
                **config.secret_env,
            }
            stage_env = prepare_secret_env(workspace, stage_env)
            config.driver.extra_env.update(xdg_env)
            append_journal(
                bundle,
                "opencode_config_prepared",
                stage=stage.id,
                config=str(stage_dir / "opencode.json"),
                xdg_roots=xdg_env,
            )
        else:
            stage_env = dict(config.driver.extra_env)

        reservation_id = f"stage:{stage.id}"
        if ledger is not None:
            try:
                ledger.assert_reconciled()
                if config.price_table is None:
                    raise MissingPrice("price table is required for a budgeted run")
                prices = load_prices(config.price_table)
                estimate = estimate_request_usd(
                    _prompt_for_estimate(config, workspace, stage.id),
                    config.max_output_tokens,
                    prices,
                )
                ledger.reserve(
                    reservation_id,
                    estimate,
                    note=f"{stage.id} conservative admission estimate",
                )
                ledger.mark_dispatched(reservation_id)
                append_journal(
                    bundle,
                    "budget_reserved",
                    stage=stage.id,
                    amount_usd=estimate,
                    estimate="worst-case prompt tokens + max output + safety factor",
                    price_table=prices.to_dict(),
                )
            except (BudgetError, MissingPrice) as error:
                append_journal(bundle, "budget_stopped", stage=stage.id, reason=str(error))
                update_manifest(bundle, {"status": "blocked", "blocker": str(error)})
                raise RunError(f"budget stopped before dispatch: {error}") from error

        policy = _sandbox_policy(config, workspace, env_override=stage_env)
        driver_result = run_driver(
            config.driver,
            workspace=workspace,
            stage_id=stage.id,
            condition=config.condition,
            stage_dir=stage_dir,
            timeout_seconds=stage_timeout,
            sandbox=policy,
            journal=lambda event, **payload: append_journal(bundle, event, **payload),
        )
        remove_secret_env(workspace)

        probe_result = (
            _probe_stage(config.benchmark, stage, workspace, stage_dir)
            if stage.require_probe
            else {"ok": True, "required": False}
        )
        archive_required = bool(stage.require_archive and config.givn_binary is not None)
        archive_result = _archive_check(archive_before, workspace, archive_required)

        analysis_summary = None
        snapshot_document = None
        try:
            snapshot_document = freeze_snapshot(workspace, stage_dir, stage_id=stage.id)
        except Exception as error:  # noqa: BLE001
            append_journal(
                bundle,
                "snapshot_failed",
                stage=stage.id,
                error=f"{type(error).__name__}: {error}",
            )
            update_manifest(bundle, {"status": "failed", "blocker": f"snapshot failed: {error}"})
            raise
        if config.analyze_stages:
            analysis_summary = _analyze_stage(workspace, stage_dir)

        if ledger is not None:
            actual = driver_result.cost_usd
            if analysis_summary:
                cost = analysis_summary.get("cost") or {}
                observed = cost.get("observed_total_usd")
                if (
                    cost.get("status") == "observed"
                    and isinstance(observed, (int, float))
                    and not cost.get("missing_cost_requests")
                ):
                    if actual is None or float(observed) > float(actual):
                        actual = float(observed)
                        append_journal(
                            bundle,
                            "budget_reconciled_from_events",
                            stage=stage.id,
                            amount_usd=actual,
                            driver_reported_usd=driver_result.cost_usd,
                            reason="event record is complete and exceeds driver-reported cost",
                        )
            ledger.commit(
                reservation_id,
                actual,
                note="committed from the larger of driver usage and complete event-record cost" if actual is not None else "usage missing; remains uncertain",
            )
            append_journal(
                bundle,
                "budget_committed",
                stage=stage.id,
                actual_usd=actual,
                totals=ledger.totals(),
            )
            update_manifest(bundle, {"budget": {**budget, **ledger.totals()}})

        protocol_ok = not driver_result.protocol_failures
        driver_clean_exit = (
            driver_result.termination == "completed" and driver_result.exit_code == 0
        )
        workflow_ok = (
            probe_result.get("ok", False)
            and archive_result.get("ok", False)
            and protocol_ok
        )
        completed = workflow_ok
        stage_record = {
            "id": stage.id,
            "completed": completed,
            "completion_basis": (
                "public probe and workflow archive; driver exit code is reported separately"
                if workflow_ok
                else "workflow gate failed"
            ),
            "driver_clean_exit": driver_clean_exit,
            "driver": driver_result.to_dict(),
            "probe": probe_result,
            "archive": archive_result,
            "analysis": analysis_summary,
            "snapshot": {
                "path": str(stage_dir / "snapshot"),
                "file_count": snapshot_document["file_count"],
                "bytes": snapshot_document["bytes"],
            },
            "task_sha256": request["task_sha256"],
            "finished_at": _stamp(),
        }
        dump_json(stage_dir / "stage.json", stage_record)
        completed_stages.append(stage_record)
        manifest = update_manifest(bundle, {"stages": completed_stages})
        append_journal(
            bundle,
            "stage_finished",
            stage=stage.id,
            completed=completed,
            driver_termination=driver_result.termination,
            driver_clean_exit=driver_clean_exit,
            protocol_ok=protocol_ok,
            probe_ok=probe_result.get("ok"),
            archive_ok=archive_result.get("ok"),
        )
        if not completed:
            all_completed = False
            previous_completed = False
            append_journal(
                bundle,
                "stage_incomplete",
                stage=stage.id,
                reason=(
                    f"driver={driver_result.termination}, probe={probe_result.get('ok')}, "
                    f"archive={archive_result.get('ok')}, protocol={protocol_ok}"
                ),
            )
            break
        previous_completed = True
        if config.stop_after == stage.id:
            append_journal(
                bundle,
                "stop_after_reached",
                stage=stage.id,
                reason="explicit --stop-after",
            )
            break

    recorded_ids = {entry.get("id") for entry in completed_stages}
    for stage in config.benchmark.stages:
        if stage.id in recorded_ids:
            continue
        stage_dir = bundle / "stages" / stage.id
        stage_dir.mkdir(parents=True, exist_ok=True)
        reason = (
            f"stopped after {config.stop_after!r} by explicit --stop-after"
            if config.stop_after and stage.requires == config.stop_after
            else (
                f"required stage {stage.requires!r} did not complete"
                if stage.requires
                else "an earlier stage did not complete"
            )
        )
        dump_json(
            stage_dir / "not-reached.json",
            {"stage": stage.id, "reason": reason, "at": _stamp()},
        )
        append_journal(bundle, "stage_not_reached", stage=stage.id, reason=reason)
        completed_stages.append({"id": stage.id, "completed": False, "not_reached": True})
        all_completed = False

    status = "completed" if all_completed and len(completed_stages) == len(config.benchmark.stages) else "incomplete"
    manifest = update_manifest(
        bundle,
        {"status": status, "finished_at": _stamp(), "stages": completed_stages},
    )
    append_journal(bundle, "run_finished", status=status)
    if ledger is not None:
        totals = ledger.totals()
        append_journal(bundle, "budget_summary", **totals)
        manifest = update_manifest(
            bundle,
            {"budget": {**(manifest.get("budget") or {}), **totals}},
        )
    return manifest


def summarize_bundle(bundle: Path) -> dict[str, Any]:
    manifest = load_manifest(bundle)
    stages = manifest.get("stages", [])
    return {
        "bundle": str(bundle),
        "condition": manifest.get("condition"),
        "status": manifest.get("status"),
        "benchmark": (manifest.get("benchmark") or {}).get("id"),
        "stages": [
            {
                "id": stage.get("id"),
                "completed": stage.get("completed"),
                "not_reached": stage.get("not_reached", False),
                "driver_termination": (stage.get("driver") or {}).get("termination"),
                "probe_ok": (stage.get("probe") or {}).get("ok"),
                "archive_ok": (stage.get("archive") or {}).get("ok"),
            }
            for stage in stages
        ],
    }
