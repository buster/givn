"""Command-line interface for the givn evaluation harness."""

from __future__ import annotations

import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

import click

from givn_harness.accounting import analyze_paths
from givn_harness.analysis_report import render_analysis
from givn_harness.redact import redact_analysis
from givn_harness.benchmark import BenchmarkError, load_benchmark
from givn_harness.compare import CompareError, build_comparison, render_comparison
from givn_harness.driver import DriverSpec, parse_script_command
from givn_harness.grader import GraderError, grade_bundle, verify_benchmark
from givn_harness.model import DEEPSEEK_FLASH_MAX, guard_record, require_model
from givn_harness.runner import RunConfig, RunError, execute, summarize_bundle
from givn_harness.sandbox import SandboxPolicy, SandboxUnavailable, build_command, probe


def _atomic_write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(text, encoding="utf-8")
    os.replace(temporary, path)


def _load_metadata(path: str | None) -> dict:
    if path is None:
        return {}
    loaded = json.loads(Path(path).read_text(encoding="utf-8"))
    if not isinstance(loaded, dict):
        raise click.ClickException("--metadata must contain a JSON object")
    return loaded


@click.group()
def main() -> None:
    """Evaluate givn workflows and analyze recorded sessions."""


@main.command()
@click.argument("events", nargs=-1, required=True, type=click.Path(exists=True))
@click.option("--output", required=True, type=click.Path(), help="Output directory")
@click.option("--session", "sessions", multiple=True, help="Only these session IDs (repeatable)")
@click.option("--scope", "scopes", multiple=True, help="Only these scope IDs (repeatable)")
@click.option("--since", default=None, help="ISO-8601 inclusive lower bound on event time")
@click.option("--until", default=None, help="ISO-8601 inclusive upper bound on event time")
@click.option("--kind", "kinds", multiple=True, help="Only these event kinds (repeatable)")
@click.option("--metadata", type=click.Path(exists=True), default=None, help="Declared metadata sidecar (JSON object)")
@click.option("--repo", "repository", type=click.Path(exists=True), default=None, help="Optional repository context (read-only)")
@click.option("--redact/--no-redact", default=True, show_default=True, help="Redact free text in exported reports")
@click.option("--quiet", is_flag=True, default=False, help="Do not print the summary")
def analyze(
    events: tuple[str, ...],
    output: str,
    sessions: tuple[str, ...],
    scopes: tuple[str, ...],
    since: str | None,
    until: str | None,
    kinds: tuple[str, ...],
    metadata: str | None,
    repository: str | None,
    redact: bool,
    quiet: bool,
) -> None:
    """Analyze existing events.jsonl records without running anything.

    The source files are read-only. No repository code, hook, plugin or test is
    executed, and no model request is made.
    """
    try:
        sidecar = _load_metadata(metadata)
        analysis = analyze_paths(
            list(events),
            sessions=sessions or None,
            scopes=scopes or None,
            since=since,
            until=until,
            kinds=kinds or None,
            metadata=sidecar,
            repository=repository,
        )
    except (ValueError, FileNotFoundError) as error:
        raise click.ClickException(str(error))

    output_dir = Path(output).resolve()
    created_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    if redact:
        analysis = redact_analysis(analysis)
    document = analysis.to_dict()
    document["created_at"] = created_at
    document["redacted"] = redact
    _atomic_write(output_dir / "analysis.json", json.dumps(document, indent=2) + "\n")
    _atomic_write(output_dir / "analysis.md", render_analysis(analysis))

    if not quiet:
        corpus = analysis.data["corpus"]
        quality = analysis.data["data_quality"]
        click.echo(
            f"Analyzed {corpus['analyzed_events']:,} of {corpus['events']:,} events "
            f"from {len(analysis.sources)} file(s); "
            f"{len(analysis.diagnostics)} diagnostic(s), "
            f"{len(analysis.findings)} finding(s)."
        )
        click.echo(f"Report: {output_dir / 'analysis.md'}")
        click.echo(f"Data:   {output_dir / 'analysis.json'}")
        if quality["diagnostic_codes"]:
            click.echo(
                "Data quality: "
                + ", ".join(
                    f"{code}={count}"
                    for code, count in list(quality["diagnostic_codes"].items())[:8]
                )
            )


@main.command()
@click.option("--benchmark", required=True, type=click.Path(exists=True), help="benchmark.toml or directory")
@click.option("--output", default="runs/", type=click.Path(), help="Bundle output directory")
@click.option("--condition", default="current", help="Condition label, for example current or candidate")
@click.option("--repeats", default=1, type=int, help="Independent trials in this condition")
@click.option("--driver", "driver_kind", type=click.Choice(["opencode", "script", "none"]), default="opencode")
@click.option("--driver-command", default=None, help="Script driver command; may use {workspace}, {prompt_file}, {stage}")
@click.option("--driver-env", multiple=True, help="Script driver environment KEY=VALUE (repeatable)")
@click.option("--model", default=DEEPSEEK_FLASH_MAX, show_default=True, help="Pinned model; other values are refused")
@click.option("--givn-binary", default=None, type=click.Path(exists=True), help="Pinned givn binary under test")
@click.option("--sandbox", "sandbox_mode", type=click.Choice(["required", "none"]), default="required")
@click.option("--network", type=click.Choice(["none", "host"]), default="none")
@click.option("--workspace-root", default="/tmp/givn-harness-workspaces", type=click.Path())
@click.option("--budget-usd", type=float, default=None, help="Hard campaign cap; required for paid drivers")
@click.option("--price-table", type=click.Path(exists=True), default=None, help="Dated price table (TOML)")
@click.option("--max-output-tokens", type=int, default=8192)
@click.option("--answer-policy", type=click.Choice(["recommended", "none"]), default="recommended")
@click.option("--deepseek-api-key-file", type=click.Path(exists=True), default=None, help="Read DEEPSEEK_API_KEY from this file without logging it")
@click.option("--allow-unverified-network", is_flag=True, default=False, help="Permit explicitly exploratory host networking for opencode")
@click.option("--stop-after", default=None, help="Stop after this stage and record later stages as not reached")
@click.option("--resume", type=click.Path(exists=True), default=None, help="Resume a bundle's completed prefix")
@click.option("--no-analyze", is_flag=True, default=False, help="Do not run the offline analyzer on stage records")
@click.option("--show-agent-output", is_flag=True, default=False)
def run(
    benchmark: str,
    output: str,
    condition: str,
    repeats: int,
    driver_kind: str,
    driver_command: str | None,
    driver_env: tuple[str, ...],
    model: str,
    givn_binary: str | None,
    sandbox_mode: str,
    network: str,
    workspace_root: str,
    budget_usd: float | None,
    price_table: str | None,
    max_output_tokens: int,
    answer_policy: str,
    deepseek_api_key_file: str | None,
    allow_unverified_network: bool,
    stop_after: str | None,
    resume: str | None,
    no_analyze: bool,
    show_agent_output: bool,
) -> None:
    """Execute a staged benchmark and preserve a durable trial bundle."""
    try:
        selected_model = require_model(model)
    except ValueError as error:
        raise click.ClickException(str(error))
    if driver_kind == "script" and not driver_command:
        raise click.ClickException("--driver script requires --driver-command")
    command = parse_script_command(driver_command) if driver_command else None
    if command:
        command = [
            os.path.abspath(item) if not os.path.isabs(item) and os.path.exists(item) else item
            for item in command
        ]
    extra_env: dict[str, str] = {}
    for entry in driver_env:
        if "=" not in entry:
            raise click.ClickException(f"--driver-env expects KEY=VALUE, got {entry!r}")
        key, value = entry.split("=", 1)
        if not key:
            raise click.ClickException("--driver-env key cannot be empty")
        extra_env[key] = value
    secret_env: dict[str, str] = {}
    if deepseek_api_key_file:
        key = Path(deepseek_api_key_file).read_text(encoding="utf-8").strip()
        if not key:
            raise click.ClickException("--deepseek-api-key-file is empty")
        secret_env["DEEPSEEK_API_KEY"] = key
    elif extra_env.get("DEEPSEEK_API_KEY"):
        secret_env["DEEPSEEK_API_KEY"] = extra_env.pop("DEEPSEEK_API_KEY")
    if driver_kind == "opencode" and not secret_env.get("DEEPSEEK_API_KEY"):
        raise click.ClickException(
            "the opencode driver needs --deepseek-api-key-file or "
            "--driver-env DEEPSEEK_API_KEY=..."
        )
    try:
        loaded = load_benchmark(benchmark)
    except BenchmarkError as error:
        raise click.ClickException(str(error))
    config = RunConfig(
        benchmark=loaded,
        output=Path(output),
        condition=condition,
        repeats=repeats,
        driver=DriverSpec(
            kind=driver_kind,
            model=selected_model,
            command=command,
            show_output=show_agent_output,
            answer_policy=answer_policy,
            extra_env=extra_env,
        ),
        givn_binary=Path(givn_binary) if givn_binary else None,
        sandbox_mode=sandbox_mode,
        network=network,
        workspace_root=Path(workspace_root),
        budget_usd=budget_usd,
        price_table=Path(price_table) if price_table else None,
        max_output_tokens=max_output_tokens,
        analyze_stages=not no_analyze,
        answer_policy=answer_policy,
        resume=Path(resume) if resume else None,
        allow_unverified_network=allow_unverified_network,
        stop_after=stop_after,
        secret_env=secret_env,
    )
    click.echo(f"Model guard: {json.dumps(guard_record(selected_model, driver_kind))}")
    try:
        bundles = execute(config)
    except (RunError, SandboxUnavailable, BenchmarkError, OSError) as error:
        raise click.ClickException(str(error))
    for bundle in bundles:
        summary = summarize_bundle(bundle)
        click.echo(
            f"Bundle: {bundle}\n"
            f"  status: {summary['status']}; "
            f"stages: "
            + ", ".join(
                f"{stage['id']}={'completed' if stage['completed'] else ('not_reached' if stage['not_reached'] else 'incomplete')}"
                for stage in summary["stages"]
            )
        )


@main.command()
@click.argument("bundles", nargs=-1, required=True, type=click.Path(exists=True))
@click.option("--benchmark", required=True, type=click.Path(exists=True), help="benchmark.toml or directory")
@click.option("--sandbox", "sandbox_mode", type=click.Choice(["required", "none"]), default="required")
@click.option("--quiet", is_flag=True, default=False)
def grade(bundles: tuple[str, ...], benchmark: str, sandbox_mode: str, quiet: bool) -> None:
    """Grade frozen stage snapshots independently; never repairs submissions."""
    try:
        loaded = load_benchmark(benchmark)
    except BenchmarkError as error:
        raise click.ClickException(str(error))
    for raw in bundles:
        bundle = Path(raw).resolve()
        try:
            document = grade_bundle(
                bundle, loaded, sandbox_mode=sandbox_mode
            )
        except GraderError as error:
            raise click.ClickException(f"{bundle}: {error}")
        if not quiet:
            summary = document["summary"]
            click.echo(
                f"{bundle.name}: {summary['passed']} passed, {summary['failed']} failed, "
                f"{summary['ungraded']} ungraded; all required passed="
                f"{summary['all_required_passed']}"
            )
            if summary.get("critical_failures"):
                click.echo("  critical: " + ", ".join(summary["critical_failures"]))


@main.command()
@click.option("--baseline", "baseline_paths", multiple=True, required=True, type=click.Path(exists=True))
@click.option("--candidate", "candidate_paths", multiple=True, required=True, type=click.Path(exists=True))
@click.option("--output", default="comparison", type=click.Path())
def compare(
    baseline_paths: tuple[str, ...],
    candidate_paths: tuple[str, ...],
    output: str,
) -> None:
    """Compare saved baseline and candidate bundles descriptively."""
    try:
        document = build_comparison(list(baseline_paths), list(candidate_paths))
    except (CompareError, OSError, ValueError) as error:
        raise click.ClickException(str(error))
    output_dir = Path(output).resolve()
    _atomic_write(output_dir / "compare.json", json.dumps(document, indent=2) + "\n")
    _atomic_write(output_dir / "compare.md", render_comparison(document))
    click.echo(
        f"Comparison: {'INCONCLUSIVE' if document['inconclusive'] else 'DESCRIPTIVE'}\n"
        f"Report: {output_dir / 'compare.md'}\n"
        f"Data:   {output_dir / 'compare.json'}"
    )
    if not document["comparability"]["comparable"]:
        click.echo("Warning: execution conditions differ; see the comparability section.")


@main.group("benchmark")
def benchmark_group() -> None:
    """Benchmark package operations."""


@benchmark_group.command("list")
@click.option("--root", default="benchmarks", type=click.Path(exists=True), help="Directory containing benchmark packages")
def benchmark_list(root: str) -> None:
    """List packaged benchmarks and their stages."""
    root_path = Path(root)
    found = 0
    for manifest in sorted(root_path.glob("*/benchmark.toml")):
        try:
            loaded = load_benchmark(manifest)
        except BenchmarkError as error:
            click.echo(f"{manifest.parent.name}: INVALID ({error})")
            continue
        found += 1
        stages = ", ".join(stage.id for stage in loaded.stages)
        click.echo(
            f"{loaded.id} {loaded.version} — {loaded.title} "
            f"[stages: {stages}] ({manifest.parent})"
        )
    if not found:
        raise click.ClickException(f"no benchmarks found under {root_path}")


@benchmark_group.command("verify")
@click.argument("benchmark", type=click.Path(exists=True))
@click.option("--output", default="benchmark-verification", type=click.Path())
def benchmark_verify(benchmark: str, output: str) -> None:
    """Prove the reference passes, the scaffold fails, and seeded defects are caught."""
    try:
        loaded = load_benchmark(benchmark)
    except BenchmarkError as error:
        raise click.ClickException(str(error))
    try:
        document = verify_benchmark(loaded, Path(output))
    except GraderError as error:
        raise click.ClickException(str(error))
    click.echo(f"Benchmark verification: {'PASS' if document['passed'] else 'FAIL'}")
    for case in document["cases"]:
        click.echo(
            f"  {case['id']}: {'PASS' if case['passed'] else 'FAIL'}"
            + (f" missing={case['missing_expected_failures']}" if case.get("missing_expected_failures") else "")
        )
    if not document["passed"]:
        raise click.ClickException("benchmark verification failed")


@main.command("sandbox-probe")
@click.option("--network", type=click.Choice(["none", "host"]), default="none")
def sandbox_probe(network: str) -> None:
    """Prove bubblewrap workspace visibility, host-state denial and network denial."""
    import tempfile

    with tempfile.TemporaryDirectory(prefix="givn-sandbox-probe-") as temporary:
        workspace = Path(temporary) / "workspace"
        workspace.mkdir()
        result = probe(SandboxPolicy(mode="required", network=network, workspace=workspace))
    click.echo(json.dumps(result, indent=2))
    if not result["ok"]:
        raise click.ClickException("sandbox probe failed")
