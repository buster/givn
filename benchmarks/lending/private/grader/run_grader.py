#!/usr/bin/env python3
"""Independent black-box grader for the lending benchmark.

The grader never imports the submission. It starts the documented command in a
copy of a frozen snapshot, speaks the public JSON-lines protocol, and records
one result per requirement. Hidden combinations live here; expectations do not.

Usage:
    run_grader.py --bundle BUNDLE --benchmark BENCHMARK --output RESULT.json
"""

from __future__ import annotations

import argparse
import json
import os
import selectors
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

GRADER_ID = "lending-register-grader"
GRADER_VERSION = "1.0.0"
RESPONSE_TIMEOUT_SECONDS = 5.0


class GraderError(Exception):
    """A requirement failed; the message is the evidence."""


class StartupError(GraderError):
    pass


class ProtocolError(GraderError):
    pass


def _app_sandbox_prefix(snapshot: Path, data_dir: Path) -> list[str]:
    """Network-free sandbox exposing only this snapshot and data directory.

    The trusted grader reads private material outside the sandbox. The
    submission process can read neither the benchmark package nor host state.
    """
    bwrap = shutil.which("bwrap")
    if bwrap is None:
        raise StartupError("bubblewrap is required for graded execution but is unavailable")
    home = str(Path.home())
    command = [
        bwrap,
        "--unshare-all",
        "--new-session",
        "--die-with-parent",
        "--as-pid-1",
        "--chdir",
        str(snapshot),
        "--ro-bind",
        "/usr",
        "/usr",
        "--ro-bind-try",
        "/bin",
        "/bin",
        "--ro-bind-try",
        "/sbin",
        "/sbin",
        "--ro-bind-try",
        "/lib",
        "/lib",
        "--ro-bind-try",
        "/lib64",
        "/lib64",
        "--tmpfs",
        "/etc",
    ]
    for relative in (
        "ssl",
        "ca-certificates",
        "resolv.conf",
        "hosts",
        "passwd",
        "group",
        "nsswitch.conf",
        "localtime",
        "ld.so.cache",
        "alternatives",
    ):
        source = Path("/etc") / relative
        if source.exists():
            command.extend(["--ro-bind-try", str(source), str(source)])
    command.extend(
        [
            "--proc",
            "/proc",
            "--dev",
            "/dev",
            "--tmpfs",
            "/run",
            "--tmpfs",
            "/tmp",
            "--dir",
            home,
            "--tmpfs",
            home,
            "--ro-bind",
            str(snapshot),
            str(snapshot),
            "--bind",
            str(data_dir),
            str(data_dir),
            "--clearenv",
            "--setenv",
            "PATH",
            "/usr/local/bin:/usr/bin:/bin",
            "--setenv",
            "HOME",
            home,
            "--setenv",
            "LANG",
            "C.UTF-8",
            "--setenv",
            "LC_ALL",
            "C.UTF-8",
            "--setenv",
            "PYTHONDONTWRITEBYTECODE",
            "1",
            "--setenv",
            "TMPDIR",
            "/tmp",
            "--",
            "/usr/bin/python3",
            "app.py",
            "--data-dir",
            str(data_dir),
        ]
    )
    return command


@dataclass
class AppClient:
    snapshot: Path
    data_dir: Path
    sandbox_mode: str = "required"
    proc: subprocess.Popen[str] | None = None
    transcript: list[str] = field(default_factory=list)
    started: bool = False

    def start(self) -> None:
        app = self.snapshot / "app.py"
        if not app.is_file():
            raise StartupError(f"missing entry point: {app}")
        if self.sandbox_mode == "required":
            command = _app_sandbox_prefix(self.snapshot, self.data_dir)
            env = {"PATH": "/usr/local/bin:/usr/bin:/bin"}
        else:
            command = [sys.executable, "app.py", "--data-dir", str(self.data_dir)]
            env = os.environ.copy()
        self.proc = subprocess.Popen(
            command,
            cwd=self.snapshot,
            env=env,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        self.started = True
        time.sleep(0.05)
        if self.proc.poll() is not None:
            detail = self._stderr() or f"exit {self.proc.returncode}"
            raise StartupError(f"application exited during startup: {detail}")

    def _stderr(self) -> str:
        if self.proc is None or self.proc.stderr is None:
            return ""
        try:
            return " ".join(self.proc.stderr.read().split())[:800]
        except (OSError, ValueError):
            return ""

    def request(self, payload: dict) -> dict:
        if self.proc is None or not self.started:
            raise ProtocolError("application is not running")
        if self.proc.stdin is None or self.proc.stdout is None:
            raise ProtocolError("application pipes are unavailable")
        if self.proc.poll() is not None:
            raise ProtocolError(f"application exited before the request: {self._stderr()}")
        line = json.dumps(payload)
        self.transcript.append(f"> {line}")
        try:
            self.proc.stdin.write(line + "\n")
            self.proc.stdin.flush()
        except (BrokenPipeError, OSError) as error:
            raise ProtocolError(f"could not write request: {error}") from error
        selector = selectors.DefaultSelector()
        selector.register(self.proc.stdout, selectors.EVENT_READ)
        try:
            if not selector.select(RESPONSE_TIMEOUT_SECONDS):
                raise ProtocolError(
                    f"no response within {RESPONSE_TIMEOUT_SECONDS:.0f}s to {line[:200]}"
                )
            response_line = self.proc.stdout.readline()
        finally:
            selector.close()
        if not response_line:
            raise ProtocolError(f"application closed stdout after {line[:200]}: {self._stderr()}")
        self.transcript.append(f"< {response_line.strip()[:400]}")
        try:
            response = json.loads(response_line)
        except ValueError as error:
            raise ProtocolError(
                f"response was not one JSON object: {response_line[:300]!r}"
            ) from error
        if not isinstance(response, dict):
            raise ProtocolError(f"response was not an object: {response_line[:300]!r}")
        return response

    def stop(self) -> None:
        if self.proc is None:
            return
        try:
            if self.proc.stdin:
                self.proc.stdin.close()
            self.proc.wait(timeout=5)
        except (OSError, subprocess.TimeoutExpired):
            self.proc.kill()
            try:
                self.proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                pass

    def restart(self) -> None:
        self.stop()
        self.start()


def expect_ok(client: AppClient, payload: dict) -> dict:
    response = client.request(payload)
    if response.get("ok") is not True:
        raise GraderError(f"expected success for {payload}, got {response}")
    return response


def expect_error(client: AppClient, payload: dict, code: str) -> dict:
    response = client.request(payload)
    if response != {"ok": False, "error": code}:
        raise GraderError(f"expected error {code!r} for {payload}, got {response}")
    return response


def seed(client: AppClient, members: int = 2, books: int = 7) -> None:
    for index in range(members):
        expect_ok(client, {"command": "add_member", "member_id": f"m{index}"})
    for index in range(books):
        expect_ok(
            client,
            {
                "command": "add_book",
                "book_id": f"b{index}",
                "title": f"Book {index}",
            },
        )


def assert_borrowed(client: AppClient, member: str, book: str) -> None:
    expect_ok(client, {"command": "borrow", "member_id": member, "book_id": book})


def assert_returned(client: AppClient, member: str, book: str) -> None:
    expect_ok(client, {"command": "return", "member_id": member, "book_id": book})


# ---------------------------------------------------------------------------
# Initial delivery requirements
# ---------------------------------------------------------------------------


def initial_basic(client: AppClient) -> None:
    seed(client)
    assert_borrowed(client, "m0", "b0")
    assert_returned(client, "m0", "b0")
    assert_borrowed(client, "m1", "b0")


def initial_book_exclusivity(client: AppClient) -> None:
    seed(client)
    assert_borrowed(client, "m0", "b1")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m1", "book_id": "b1"},
        "book_already_loaned",
    )
    assert_returned(client, "m0", "b1")
    assert_borrowed(client, "m1", "b1")


def initial_loan_limit(client: AppClient) -> None:
    seed(client)
    for index in range(5):
        assert_borrowed(client, "m0", f"b{index}")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "b5"},
        "loan_limit_reached",
    )
    # The limit belongs to the member, not the library.
    assert_borrowed(client, "m1", "b5")


def initial_return_releases_capacity(client: AppClient) -> None:
    seed(client)
    for index in range(5):
        assert_borrowed(client, "m0", f"b{index}")
    assert_returned(client, "m0", "b2")
    assert_borrowed(client, "m0", "b5")


def initial_persistence(client: AppClient) -> None:
    seed(client)
    assert_borrowed(client, "m0", "b3")
    client.restart()
    expect_error(
        client,
        {"command": "borrow", "member_id": "m1", "book_id": "b3"},
        "book_already_loaned",
    )
    assert_returned(client, "m0", "b3")


def initial_rejection_consistency(client: AppClient) -> None:
    seed(client)
    assert_borrowed(client, "m0", "b4")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m1", "book_id": "b4"},
        "book_already_loaned",
    )
    # The failed borrow must not have created a loan for m1: the book is still
    # m0's, and m1's five-loan capacity is untouched.
    assert_returned(client, "m0", "b4")
    assert_borrowed(client, "m1", "b4")
    for index in range(5):
        if f"b{index}" != "b4":
            assert_borrowed(client, "m1", f"b{index}")


def initial_unknown_identifiers(client: AppClient) -> None:
    seed(client)
    expect_error(
        client,
        {"command": "borrow", "member_id": "nobody", "book_id": "b0"},
        "unknown_member",
    )
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "nothing"},
        "unknown_book",
    )
    expect_error(
        client,
        {"command": "return", "member_id": "m0", "book_id": "b0"},
        "loan_not_found",
    )
    assert_borrowed(client, "m0", "b0")


# ---------------------------------------------------------------------------
# Follow-up requirements
# ---------------------------------------------------------------------------


def followup_reference_only_blocks_borrow(client: AppClient) -> None:
    seed(client)
    expect_ok(client, {"command": "set_reference_only", "book_id": "b0", "reference_only": True})
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "b0"},
        "reference_only",
    )
    assert_borrowed(client, "m0", "b1")


def followup_existing_loan_can_return(client: AppClient) -> None:
    seed(client)
    assert_borrowed(client, "m0", "b0")
    expect_ok(client, {"command": "set_reference_only", "book_id": "b0", "reference_only": True})
    assert_returned(client, "m0", "b0")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m1", "book_id": "b0"},
        "reference_only",
    )


def followup_flag_can_be_cleared(client: AppClient) -> None:
    seed(client)
    expect_ok(client, {"command": "set_reference_only", "book_id": "b0", "reference_only": True})
    expect_ok(client, {"command": "set_reference_only", "book_id": "b0", "reference_only": False})
    assert_borrowed(client, "m0", "b0")


def followup_flag_persists_and_is_scoped(client: AppClient) -> None:
    seed(client)
    expect_ok(client, {"command": "set_reference_only", "book_id": "b1", "reference_only": True})
    client.restart()
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "b1"},
        "reference_only",
    )
    assert_borrowed(client, "m0", "b2")


def followup_preserves_lending_rules(client: AppClient) -> None:
    seed(client)
    expect_ok(client, {"command": "set_reference_only", "book_id": "b6", "reference_only": True})
    for index in range(5):
        assert_borrowed(client, "m0", f"b{index}")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "b5"},
        "loan_limit_reached",
    )
    assert_borrowed(client, "m1", "b5")
    expect_error(
        client,
        {"command": "borrow", "member_id": "m1", "book_id": "b0"},
        "book_already_loaned",
    )
    assert_returned(client, "m0", "b0")
    assert_borrowed(client, "m1", "b0")
    client.restart()
    expect_error(
        client,
        {"command": "borrow", "member_id": "m0", "book_id": "b0"},
        "book_already_loaned",
    )


REQUIREMENTS: list[dict[str, Any]] = [
    {
        "id": "I1",
        "stage": "initial",
        "description": "Add members and books, borrow an available book, and return it",
        "critical": True,
        "test": initial_basic,
    },
    {
        "id": "I2",
        "stage": "initial",
        "description": "Reject borrowing a book already loaned to someone else",
        "critical": True,
        "test": initial_book_exclusivity,
    },
    {
        "id": "I3",
        "stage": "initial",
        "description": "Enforce at most five active loans per member, independently per member",
        "critical": True,
        "test": initial_loan_limit,
    },
    {
        "id": "I4",
        "stage": "initial",
        "description": "Returning a book releases the member's loan capacity",
        "critical": True,
        "test": initial_return_releases_capacity,
    },
    {
        "id": "I5",
        "stage": "initial",
        "description": "Lending state survives a normal restart",
        "critical": True,
        "test": initial_persistence,
    },
    {
        "id": "I6",
        "stage": "initial",
        "description": "A rejected borrow leaves book ownership and member capacity consistent",
        "critical": True,
        "test": initial_rejection_consistency,
    },
    {
        "id": "I7",
        "stage": "initial",
        "description": "Unknown members, books and loans are rejected without corrupting state",
        "critical": False,
        "test": initial_unknown_identifiers,
    },
    {
        "id": "F1",
        "stage": "followup",
        "description": "Reference-only books cannot be borrowed",
        "critical": True,
        "test": followup_reference_only_blocks_borrow,
    },
    {
        "id": "F2",
        "stage": "followup",
        "description": "An existing loan of a now reference-only book can still be returned",
        "critical": True,
        "test": followup_existing_loan_can_return,
    },
    {
        "id": "F3",
        "stage": "followup",
        "description": "Clearing the reference-only flag restores borrowing",
        "critical": False,
        "test": followup_flag_can_be_cleared,
    },
    {
        "id": "F4",
        "stage": "followup",
        "description": "The reference-only flag persists and does not affect other books",
        "critical": True,
        "test": followup_flag_persists_and_is_scoped,
    },
    {
        "id": "F5",
        "stage": "followup",
        "description": "Loan limit, book exclusivity and persistence still work after the change",
        "critical": True,
        "test": followup_preserves_lending_rules,
    },
]


def run_case(
    requirement: dict[str, Any],
    snapshot: Path,
    *,
    data_dir: Path | None = None,
    sandbox_mode: str = "required",
) -> dict[str, Any]:
    result = {
        "id": requirement["id"],
        "description": requirement["description"],
        "stage": requirement["stage"],
        "critical": bool(requirement.get("critical")),
        "status": "ungraded",
        "evidence": "",
    }
    if not snapshot.is_dir():
        result["evidence"] = f"snapshot is missing: {snapshot}"
        return result
    temporary = None
    snapshot_work = None
    if data_dir is None:
        temporary = tempfile.TemporaryDirectory(prefix=f"grade-{requirement['id'].lower()}-")
        data_dir = Path(temporary.name)
    # Never let submission code write into the authoritative snapshot.
    snapshot_work = tempfile.TemporaryDirectory(prefix=f"grade-copy-{requirement['id'].lower()}-")
    working_snapshot = Path(snapshot_work.name) / "snapshot"
    import shutil

    shutil.copytree(snapshot, working_snapshot, symlinks=True)
    client = AppClient(working_snapshot, data_dir, sandbox_mode=sandbox_mode)
    try:
        client.start()
        requirement["test"](client)
        result["status"] = "pass"
        result["evidence"] = "observed expected responses; see transcript"
    except StartupError as error:
        result["status"] = "fail"
        result["evidence"] = f"startup failure: {error}"
    except (GraderError, AssertionError) as error:
        result["status"] = "fail"
        result["evidence"] = str(error)
    except Exception as error:  # noqa: BLE001 - a grader must not claim pass
        result["status"] = "fail"
        result["evidence"] = f"unexpected grader-observed failure: {type(error).__name__}: {error}"
    finally:
        client.stop()
        if temporary is not None:
            temporary.cleanup()
        if snapshot_work is not None:
            snapshot_work.cleanup()
    result["transcript"] = client.transcript[-40:]
    return result


def run_cross_version(
    initial_snapshot: Path, followup_snapshot: Path, *, sandbox_mode: str = "required"
) -> dict[str, Any]:
    identifier = "X1"
    description = (
        "A loan created by the initial snapshot is visible to and returnable by "
        "the follow-up snapshot on the same data directory"
    )
    result = {
        "id": identifier,
        "description": description,
        "stage": "followup",
        "critical": True,
        "status": "ungraded",
        "evidence": "",
    }
    if not initial_snapshot.is_dir() or not followup_snapshot.is_dir():
        result["evidence"] = "both stage snapshots are required"
        return result
    with tempfile.TemporaryDirectory(prefix="grade-x1-") as root:
        data_dir = Path(root) / "data"
        data_dir.mkdir(parents=True, exist_ok=True)
        initial_copy = Path(root) / "initial"
        followup_copy = Path(root) / "followup"
        import shutil

        shutil.copytree(initial_snapshot, initial_copy, symlinks=True)
        shutil.copytree(followup_snapshot, followup_copy, symlinks=True)
        first = AppClient(initial_copy, data_dir, sandbox_mode=sandbox_mode)
        second = AppClient(followup_copy, data_dir, sandbox_mode=sandbox_mode)
        try:
            first.start()
            seed(first)
            assert_borrowed(first, "m0", "b0")
            first.stop()

            second.start()
            expect_error(
                second,
                {"command": "borrow", "member_id": "m1", "book_id": "b0"},
                "book_already_loaned",
            )
            assert_returned(second, "m0", "b0")
            assert_borrowed(second, "m1", "b0")
            result["status"] = "pass"
            result["evidence"] = "initial-created loan survived and was returnable after the change"
            result["transcript"] = (first.transcript + second.transcript)[-40:]
        except StartupError as error:
            result["status"] = "fail"
            result["evidence"] = f"startup failure: {error}"
        except (GraderError, AssertionError) as error:
            result["status"] = "fail"
            result["evidence"] = str(error)
        except Exception as error:  # noqa: BLE001
            result["status"] = "fail"
            result["evidence"] = f"unexpected grader-observed failure: {type(error).__name__}: {error}"
        finally:
            first.stop()
            second.stop()
    return result


def find_stage_snapshots(bundle: Path) -> dict[str, Path]:
    stages: dict[str, Path] = {}
    stages_dir = bundle / "stages"
    if not stages_dir.is_dir():
        return stages
    for entry in sorted(stages_dir.iterdir()):
        snapshot = entry / "snapshot"
        if entry.is_dir() and snapshot.is_dir():
            stages[entry.name] = snapshot
    return stages


def summarize(requirements: list[dict[str, Any]]) -> dict[str, Any]:
    expected_total = len(REQUIREMENTS) + 1  # X1 is appended after the stage cases.
    required = [item for item in requirements if item["status"] != "ungraded"]
    failed = [item for item in required if item["status"] == "fail"]
    critical = [item for item in failed if item.get("critical")]
    return {
        "required": expected_total,
        "graded": len(required),
        "passed": sum(1 for item in required if item["status"] == "pass"),
        "failed": len(failed),
        "ungraded": len(requirements) - len(required),
        "critical_failures": [item["id"] for item in critical],
        "all_required_passed": len(required) == expected_total and not failed,
        "note": (
            "Requirements are counted once. Many assertions for one rule are not "
            "many independent requirements."
        ),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", required=True)
    parser.add_argument("--benchmark", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument(
        "--sandbox", choices=["required", "none"], default="required"
    )
    args = parser.parse_args(argv)

    bundle = Path(args.bundle).resolve()
    snapshots = find_stage_snapshots(bundle)
    results_by_stage: dict[str, list[dict[str, Any]]] = {"initial": [], "followup": []}
    all_results: list[dict[str, Any]] = []

    for requirement in REQUIREMENTS:
        snapshot = snapshots.get(requirement["stage"])
        if snapshot is None:
            result = {
                "id": requirement["id"],
                "description": requirement["description"],
                "stage": requirement["stage"],
                "critical": bool(requirement.get("critical")),
                "status": "ungraded",
                "evidence": f"stage snapshot {requirement['stage']!r} was not reached",
            }
        else:
            result = run_case(
                requirement, snapshot, sandbox_mode=args.sandbox
            )
        results_by_stage.setdefault(requirement["stage"], []).append(result)
        all_results.append(result)

    if "initial" in snapshots and "followup" in snapshots:
        cross = run_cross_version(
            snapshots["initial"], snapshots["followup"], sandbox_mode=args.sandbox
        )
    else:
        cross = {
            "id": "X1",
            "description": "Initial-created state survives the follow-up change",
            "stage": "followup",
            "critical": True,
            "status": "ungraded",
            "evidence": "both stage snapshots are required",
        }
    results_by_stage["followup"].append(cross)
    all_results.append(cross)

    document = {
        "schema": "givn-harness/grade/v1",
        "grader": {
            "id": GRADER_ID,
            "version": GRADER_VERSION,
            "kind": "independent black-box behavioral test",
        },
        "bundle": str(bundle),
        "stages": {
            stage: {
                "snapshot_present": stage in snapshots,
                "results": results,
            }
            for stage, results in results_by_stage.items()
        },
        "requirements": all_results,
        "summary": summarize(all_results),
    }
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(document, indent=2) + "\n", encoding="utf-8")
    print(
        f"graded {document['summary']['graded']}/{document['summary']['required']} requirements: "
        f"{document['summary']['passed']} passed, {document['summary']['failed']} failed, "
        f"{document['summary']['ungraded']} ungraded"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
