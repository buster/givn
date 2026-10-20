#!/usr/bin/env python3
"""Public follow-up smoke probe: reference-only behavior.

Visible to the agent and intentionally narrow. Hidden grading still exercises
combinations and boundaries independently.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path


class Client:
    def __init__(self, workspace: Path, data_dir: Path) -> None:
        self.proc = subprocess.Popen(
            [sys.executable, "app.py", "--data-dir", str(data_dir)],
            cwd=workspace,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )

    def request(self, payload: dict) -> dict:
        assert self.proc.stdin is not None
        assert self.proc.stdout is not None
        self.proc.stdin.write(json.dumps(payload) + "\n")
        self.proc.stdin.flush()
        line = self.proc.stdout.readline()
        if not line:
            error = self.proc.stderr.read() if self.proc.stderr else ""
            raise RuntimeError(f"application stopped: {error.strip()[:500]}")
        return json.loads(line)

    def close(self) -> None:
        if self.proc.stdin:
            self.proc.stdin.close()
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()
            self.proc.wait()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--workspace", default=".")
    args = parser.parse_args()
    workspace = Path(args.workspace).resolve()
    if not (workspace / "app.py").is_file():
        print("follow-up probe: app.py is missing", file=sys.stderr)
        return 2
    with tempfile.TemporaryDirectory(prefix="lending-followup-probe-") as data_dir:
        client = Client(workspace, Path(data_dir))
        try:
            for request in (
                {"command": "add_member", "member_id": "probe-m0"},
                {"command": "add_member", "member_id": "probe-m1"},
                {"command": "add_book", "book_id": "probe-b", "title": "Probe"},
                {"command": "borrow", "member_id": "probe-m0", "book_id": "probe-b"},
            ):
                response = client.request(request)
                if response.get("ok") is not True:
                    print(f"follow-up probe: unexpected {response} for {request}", file=sys.stderr)
                    return 1
            response = client.request(
                {"command": "set_reference_only", "book_id": "probe-b", "reference_only": True}
            )
            if response.get("ok") is not True:
                print(f"follow-up probe: set_reference_only failed: {response}", file=sys.stderr)
                return 1
            response = client.request(
                {"command": "return", "member_id": "probe-m0", "book_id": "probe-b"}
            )
            if response.get("ok") is not True:
                print(f"follow-up probe: returning an existing loan failed: {response}", file=sys.stderr)
                return 1
            response = client.request(
                {"command": "borrow", "member_id": "probe-m1", "book_id": "probe-b"}
            )
            if response != {"ok": False, "error": "reference_only"}:
                print(f"follow-up probe: reference-only borrow not rejected: {response}", file=sys.stderr)
                return 1
        except Exception as error:  # noqa: BLE001
            print(f"follow-up probe: {error}", file=sys.stderr)
            return 1
        finally:
            client.close()
    print("follow-up probe: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
