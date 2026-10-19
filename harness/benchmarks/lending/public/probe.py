#!/usr/bin/env python3
"""Public smoke probe for the lending benchmark.

This probe is visible to the agent and is not a hidden grading case. It checks
only that the documented startup and a basic lending sequence work.
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
        print("probe: app.py is missing", file=sys.stderr)
        return 2
    with tempfile.TemporaryDirectory(prefix="lending-probe-") as data_dir:
        client = Client(workspace, Path(data_dir))
        try:
            checks = [
                ({"command": "ping"}, lambda r: r == {"ok": True}),
                ({"command": "add_member", "member_id": "probe-m"}, lambda r: r.get("ok") is True),
                ({"command": "add_book", "book_id": "probe-b", "title": "Probe"}, lambda r: r.get("ok") is True),
                ({"command": "borrow", "member_id": "probe-m", "book_id": "probe-b"}, lambda r: r.get("ok") is True),
                ({"command": "return", "member_id": "probe-m", "book_id": "probe-b"}, lambda r: r.get("ok") is True),
            ]
            for request, predicate in checks:
                response = client.request(request)
                if not predicate(response):
                    print(f"probe: unexpected response to {request}: {response}", file=sys.stderr)
                    return 1
        except Exception as error:  # noqa: BLE001 - probe reports any failure
            print(f"probe: {error}", file=sys.stderr)
            return 1
        finally:
            client.close()
    print("probe: ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
