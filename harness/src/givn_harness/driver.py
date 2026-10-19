"""Neutral stage drivers.

The driver delivers a stage request, the permitted workspace, the answer policy
and the completion protocol. It does not repeat givn's lifecycle, testing or
commit rules; givn's installed guidance supplies those.

A stage gets a fresh process and therefore a fresh agent session. The follow-up
stage does not receive prior conversational context; it receives the repository
the first stage produced.
"""

from __future__ import annotations

import shutil
import json
import os
import shlex
import signal
import subprocess
import sys
import threading
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Callable

from givn_harness.sandbox import SandboxPolicy, build_command


@dataclass
class DriverSpec:
    kind: str  # opencode | script | none
    model: str
    command: list[str] | None = None
    show_output: bool = False
    answer_policy: str = "recommended"  # recommended | none
    extra_env: dict[str, str] = field(default_factory=dict)


@dataclass
class DriverResult:
    stage: str
    kind: str
    exit_code: int | None
    termination: str
    duration_seconds: float
    stdout_path: str
    stderr_path: str
    prompt_path: str
    usage: dict[str, Any] | None = None
    cost_usd: float | None = None
    questions: list[dict[str, Any]] = field(default_factory=list)
    protocol_failures: list[str] = field(default_factory=list)
    decisions_sha256: str | None = None
    note: str = ""

    def to_dict(self) -> dict[str, Any]:
        return {
            "stage": self.stage,
            "kind": self.kind,
            "exit_code": self.exit_code,
            "termination": self.termination,
            "duration_seconds": round(self.duration_seconds, 3),
            "stdout_path": self.stdout_path,
            "stderr_path": self.stderr_path,
            "prompt_path": self.prompt_path,
            "usage": self.usage,
            "cost_usd": self.cost_usd,
            "questions": self.questions,
            "protocol_failures": self.protocol_failures,
            "decisions_sha256": self.decisions_sha256,
            "note": self.note,
        }


class QuestionWatcher:
    """File-based question protocol with an unambiguous-recommended policy.

    The agent appends JSON lines to ``questions.jsonl``; the watcher appends one
    answer per valid question to ``answers.jsonl``. A question without exactly
    one recommended option is a protocol failure, never an arbitrary default.
    """

    def __init__(
        self,
        workspace: Path,
        *,
        policy: str,
        journal: Callable[..., None] | None = None,
        poll_seconds: float = 0.2,
    ) -> None:
        self.workspace = workspace
        self.questions_path = workspace / "questions.jsonl"
        self.answers_path = workspace / "answers.jsonl"
        self.policy = policy
        self.journal = journal
        self.poll_seconds = poll_seconds
        self._stop = threading.Event()
        self._thread: threading.Thread | None = None
        self._seen: set[str] = set()
        self.answers: list[dict[str, Any]] = []
        self.failures: list[str] = []

    def start(self) -> None:
        if self.policy == "none":
            return
        self.questions_path.touch(exist_ok=True)
        self._thread = threading.Thread(target=self._run, daemon=True)
        self._thread.start()

    def stop(self) -> None:
        self._stop.set()
        if self._thread is not None:
            self._thread.join(timeout=2)

    def _run(self) -> None:
        while not self._stop.is_set():
            self._poll()
            self._stop.wait(self.poll_seconds)

    def _poll(self) -> None:
        if not self.questions_path.is_file():
            return
        try:
            lines = self.questions_path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError:
            return
        for line in lines:
            line = line.strip()
            if not line:
                continue
            try:
                question = json.loads(line)
            except ValueError:
                message = f"question line is not JSON: {line[:160]!r}"
                self._record_failure(message)
                continue
            if not isinstance(question, dict):
                self._record_failure("question is not a JSON object")
                continue
            identifier = str(question.get("id") or question.get("question") or "")[:240]
            if not identifier:
                self._record_failure("question has no id or text")
                continue
            if identifier in self._seen:
                continue
            self._seen.add(identifier)
            self._answer(question, identifier)

    def _answer(self, question: dict[str, Any], identifier: str) -> None:
        text = question.get("question")
        options = question.get("options")
        recommended = question.get("recommended")
        if not isinstance(text, str) or not text.strip():
            self._record_failure(f"question {identifier!r} has no text")
            return
        if (
            not isinstance(options, list)
            or len(options) < 2
            or not all(isinstance(item, str) and item.strip() for item in options)
        ):
            self._record_failure(f"question {identifier!r} needs at least two string options")
            return
        selected: str | None = None
        if isinstance(recommended, int) and not isinstance(recommended, bool):
            if 0 <= recommended < len(options):
                selected = options[recommended]
        elif isinstance(recommended, str) and recommended in options:
            selected = recommended
        # An explicit recommended option is required exactly once. If several
        # options are starred, the question is genuinely ambiguous.
        starred = [item for item in options if item.strip().startswith("*")]
        if selected is None and len(starred) == 1:
            selected = starred[0].lstrip("* ").strip()
        if selected is None:
            self._record_failure(
                f"question {identifier!r} has no unambiguous recommended option"
            )
            return
        answer = {
            "id": identifier,
            "question": text,
            "options": options,
            "selected": selected,
            "policy": "recommended",
            "answered_at": _stamp(),
        }
        with self.answers_path.open("a", encoding="utf-8") as handle:
            handle.write(json.dumps(answer, ensure_ascii=False) + "\n")
            handle.flush()
            os.fsync(handle.fileno())
        self.answers.append(answer)
        if self.journal:
            self.journal(
                "question_answered",
                question=identifier,
                selected=selected,
                policy="recommended",
            )

    def _record_failure(self, message: str) -> None:
        if message in self.failures:
            return
        self.failures.append(message)
        with self.answers_path.open("a", encoding="utf-8") as handle:
            handle.write(
                json.dumps(
                    {
                        "status": "protocol_failure",
                        "reason": message,
                        "at": _stamp(),
                    },
                    ensure_ascii=False,
                )
                + "\n"
            )
        if self.journal:
            self.journal("question_protocol_failure", reason=message)


def _stamp() -> str:
    from datetime import datetime, timezone

    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def build_driver_prompt(
    stage_id: str,
    task_text: str,
    *,
    answer_policy: str,
    workspace: Path,
) -> str:
    policy = (
        "If you face a genuine open choice with one clearly recommended option, "
        "take it and append a short record to `decisions.md`. Do not wait for a "
        "human. If you use the question protocol, append a JSON line to "
        "`questions.jsonl` with `id`, `question`, `options` and `recommended` "
        "(an index), then read `answers.jsonl`; the harness answers only "
        "unambiguous recommended options. Do not invent approval."
        if answer_policy == "recommended"
        else "Do not wait for a human; make reasonable in-scope choices and record "
        "them in `decisions.md`."
    )
    return f"""# Harness stage request: {stage_id}

You are working in an isolated repository at `{workspace}`.

Read `task.md` and `contract.md` in the repository root. Carry out the request.
Use the repository as it stands. Work only inside this workspace; do not read
other harness workspaces or host state.

{policy}

The harness freezes the workspace when this process stops. It does not supply
hidden tests and it does not repeat givn's lifecycle rules: use the guidance
already installed in the repository. Do not modify `task.md` or `contract.md`.

When the request is complete, stop. If it cannot be completed, leave the
workspace in an inspectable state and stop.
"""


def _title(stage_id: str, condition: str) -> str:
    return f"harness:{condition}:{stage_id}"


def _command_for(
    spec: DriverSpec,
    *,
    workspace: Path,
    prompt_path: Path,
    stage_id: str,
    condition: str,
) -> list[str]:
    if spec.kind == "opencode":
        executable = shutil.which("opencode") or "opencode"
        return [
            executable,
            "run",
            "--standalone",
            "--auto",
            "--format",
            "json",
            "--model",
            spec.model,
            "--title",
            _title(stage_id, condition),
            "--file",
            str(prompt_path),
            "Follow the attached harness stage request exactly.",
        ]
    if spec.kind == "script":
        if not spec.command:
            raise ValueError("script driver needs a command")
        values = {
            "workspace": str(workspace),
            "prompt_file": str(prompt_path),
            "task": str(workspace / "task.md"),
            "stage": stage_id,
            "condition": condition,
        }
        command: list[str] = []
        for item in spec.command:
            for key, value in values.items():
                item = item.replace("{" + key + "}", value)
            command.append(item)
        return command
    if spec.kind == "none":
        return ["true"]
    raise ValueError(f"unknown driver kind: {spec.kind!r}")


def _extract_usage(stdout_path: Path) -> tuple[dict[str, Any] | None, float | None]:
    """Best-effort extraction from JSON-formatted agent output."""
    if not stdout_path.is_file():
        return None, None
    usage_totals: dict[str, float] = {}
    cost: float | None = None
    for line in stdout_path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or not line.startswith("{"):
            continue
        try:
            item = json.loads(line)
        except ValueError:
            continue
        candidates: list[dict[str, Any]] = []
        if isinstance(item, dict):
            candidates.append(item)
            if isinstance(item.get("data"), dict):
                candidates.append(item["data"])
            if isinstance(item.get("usage"), dict):
                candidates.append(item["usage"])
            if isinstance(item.get("part"), dict):
                candidates.append(item["part"])
        for candidate in candidates:
            usage = candidate.get("usage") if isinstance(candidate.get("usage"), dict) else candidate
            if isinstance(usage, dict):
                for key in ("input", "output", "cache_read", "cache_write", "total_tokens", "reasoning"):
                    value = usage.get(key)
                    if isinstance(value, (int, float)) and not isinstance(value, bool):
                        usage_totals[key] = usage_totals.get(key, 0.0) + float(value)
            cost_value = candidate.get("cost")
            if isinstance(cost_value, (int, float)) and not isinstance(cost_value, bool):
                cost = (cost or 0.0) + float(cost_value)
            elif isinstance(cost_value, dict):
                total = cost_value.get("total")
                if isinstance(total, (int, float)) and not isinstance(total, bool):
                    cost = (cost or 0.0) + float(total)
    return (usage_totals or None), cost


def _stage_script_inputs(
    workspace: Path,
    spec: DriverSpec,
) -> tuple[DriverSpec, dict[str, str]]:
    """Copy external script-driver inputs into the workspace.

    A sandboxed script driver cannot read the host repository. Copy only the
    explicit command file and absolute env values that exist, and rewrite them
    to workspace-local paths. The source files are not modified.
    """
    import dataclasses
    import shutil

    command = list(spec.command or [])
    env = dict(spec.extra_env)
    driver_dir = workspace / ".harness" / "driver"
    input_dir = workspace / ".harness" / "inputs"
    driver_dir.mkdir(parents=True, exist_ok=True)
    input_dir.mkdir(parents=True, exist_ok=True)
    for index, item in enumerate(command):
        candidate = Path(item)
        if candidate.is_file() and not str(candidate.resolve()).startswith(str(workspace.resolve())):
            destination = driver_dir / f"{index}-{candidate.name}"
            shutil.copy2(candidate, destination)
            command[index] = str(destination)
    for key, value in list(env.items()):
        candidate = Path(value)
        if candidate.is_file() and not str(candidate.resolve()).startswith(str(workspace.resolve())):
            destination = input_dir / f"{key}-{candidate.name}"
            shutil.copy2(candidate, destination)
            env[key] = str(destination)
    return (
        dataclasses.replace(spec, command=command, extra_env=env),
        {"command": command, "env": env},
    )


def run_driver(
    spec: DriverSpec,
    *,
    workspace: Path,
    stage_id: str,
    condition: str,
    stage_dir: Path,
    timeout_seconds: int,
    sandbox: SandboxPolicy,
    journal: Callable[..., None] | None = None,
) -> DriverResult:
    workspace = workspace.resolve()
    stage_dir.mkdir(parents=True, exist_ok=True)
    prompt_path = stage_dir / "prompt.md"
    prompt_text = build_driver_prompt(
        stage_id,
        (workspace / "task.md").read_text(encoding="utf-8", errors="replace")
        if (workspace / "task.md").is_file()
        else "",
        answer_policy=spec.answer_policy,
        workspace=workspace,
    )
    prompt_path.write_text(prompt_text, encoding="utf-8")
    visible_prompt_dir = workspace / ".harness"
    visible_prompt_dir.mkdir(exist_ok=True)
    visible_prompt = visible_prompt_dir / "prompt.md"
    visible_prompt.write_text(prompt_text, encoding="utf-8")

    if sandbox.mode == "required" and spec.kind == "script":
        spec, staged = _stage_script_inputs(workspace, spec)
        sandbox.env.update(spec.extra_env)
        if journal:
            journal("script_inputs_staged", stage=stage_id, **staged)

    watcher = QuestionWatcher(
        workspace,
        policy=spec.answer_policy,
        journal=journal,
    )
    watcher.start()
    stdout_path = stage_dir / "agent.stdout.log"
    stderr_path = stage_dir / "agent.stderr.log"
    command = _command_for(
        spec,
        workspace=workspace,
        prompt_path=visible_prompt,
        stage_id=stage_id,
        condition=condition,
    )
    full_command = build_command(sandbox, command)
    env = {"PYTHONUNBUFFERED": "1", **spec.extra_env}
    if sandbox.mode == "required":
        # Environment is set through bwrap; Popen still needs a minimal env.
        popen_env = {"PATH": "/usr/local/bin:/usr/bin:/bin"}
    else:
        popen_env = os.environ.copy()
        popen_env.update(env)
        popen_env["PYTHONUNBUFFERED"] = "1"
    started = time.monotonic()
    termination = "completed"
    exit_code: int | None = None
    note = ""
    if journal:
        journal(
            "driver_started",
            stage=stage_id,
            driver=spec.kind,
            command=command,
            sandbox=sandbox.describe(),
        )
    try:
        with stdout_path.open("wb") as stdout_handle, stderr_path.open("wb") as stderr_handle:
            try:
                proc = subprocess.Popen(
                    full_command,
                    cwd=str(workspace),
                    env=popen_env,
                    stdin=subprocess.DEVNULL,
                    stdout=stdout_handle,
                    stderr=stderr_handle,
                    start_new_session=True,
                )
            except OSError as error:
                note = f"could not start driver: {error}"
                termination = "startup_error"
                result = DriverResult(
                    stage=stage_id,
                    kind=spec.kind,
                    exit_code=None,
                    termination=termination,
                    duration_seconds=time.monotonic() - started,
                    stdout_path=str(stdout_path),
                    stderr_path=str(stderr_path),
                    prompt_path=str(prompt_path),
                    note=note,
                )
                if journal:
                    journal("driver_finished", **result.to_dict())
                return result
            try:
                exit_code = proc.wait(timeout=timeout_seconds)
                if exit_code != 0:
                    termination = "driver_error"
            except subprocess.TimeoutExpired:
                termination = "timeout"
                try:
                    os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
                except (ProcessLookupError, OSError):
                    pass
                try:
                    exit_code = proc.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    exit_code = None
            except KeyboardInterrupt:
                termination = "interrupted"
                try:
                    os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
                except (ProcessLookupError, OSError):
                    pass
                exit_code = proc.wait()
    finally:
        watcher.stop()

    usage, cost = _extract_usage(stdout_path)
    decisions = workspace / "decisions.md"
    result = DriverResult(
        stage=stage_id,
        kind=spec.kind,
        exit_code=exit_code,
        termination=termination,
        duration_seconds=time.monotonic() - started,
        stdout_path=str(stdout_path),
        stderr_path=str(stderr_path),
        prompt_path=str(prompt_path),
        usage=usage,
        cost_usd=cost,
        questions=watcher.answers,
        protocol_failures=watcher.failures,
        decisions_sha256=_sha256(decisions) if decisions.is_file() else None,
        note=note,
    )
    (stage_dir / "driver.json").write_text(
        json.dumps(result.to_dict(), indent=2) + "\n", encoding="utf-8"
    )
    if journal:
        journal("driver_finished", **result.to_dict())
    if spec.show_output:
        _print_tail(stdout_path, stderr_path)
    return result


def _sha256(path: Path) -> str:
    import hashlib

    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def _print_tail(stdout_path: Path, stderr_path: Path, lines: int = 60) -> None:
    for label, path in (("out", stdout_path), ("err", stderr_path)):
        if not path.is_file():
            continue
        content = path.read_text(encoding="utf-8", errors="replace").splitlines()
        if not content:
            continue
        sys.stderr.write(f"--- agent {label} (last {min(lines, len(content))} lines) ---\n")
        sys.stderr.write("\n".join(content[-lines:]) + "\n")
        sys.stderr.flush()


def parse_script_command(value: str) -> list[str]:
    return shlex.split(value)
