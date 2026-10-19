#!/usr/bin/env python3
"""Deterministic fake agent for runner tests.

It copies an application source supplied through HARNESS_APP_SOURCE into the
workspace. HARNESS_AGENT_MODE=noop leaves the workspace untouched, which is used
to test incomplete-stage handling and follow-up not-reached behavior.
"""

from __future__ import annotations

import os
import shutil
import sys
from pathlib import Path


def main() -> int:
    workspace = Path.cwd()
    mode = os.environ.get("HARNESS_AGENT_MODE", "copy")
    if mode == "noop":
        return 0
    if mode == "sleep":
        import time

        time.sleep(float(os.environ.get("HARNESS_SLEEP_SECONDS", "5")))
        return 0
    if mode == "fail":
        print("fake agent: deliberate failure", file=sys.stderr)
        return 7
    source = os.environ.get("HARNESS_APP_SOURCE")
    if not source:
        print("fake agent: HARNESS_APP_SOURCE is not set", file=sys.stderr)
        return 2
    shutil.copy2(source, workspace / "app.py")
    (workspace / "decisions.md").write_text(
        "- Used the documented JSON-lines interface from contract.md.\n",
        encoding="utf-8",
    )
    print("fake agent: copied application")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
