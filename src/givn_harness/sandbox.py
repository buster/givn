"""Execution isolation policy and bubblewrap command construction.

The harness never silently falls back to unsandboxed graded execution. A policy
is either enforceable (``mode="required"`` and every probe passes) or the run
stops with a reported blocker. Host networking is never described as a verified
egress allowlist; that distinction is recorded in every manifest.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import tempfile
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


class SandboxUnavailable(RuntimeError):
    """The requested isolation cannot be established."""


@dataclass
class SandboxPolicy:
    mode: str = "required"  # required | none
    network: str = "none"  # none | host
    workspace: Path | None = None
    extra_ro_binds: list[Path] = field(default_factory=list)
    credential_binds: list[Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    minimal_etc: bool = True

    def describe(self) -> dict[str, Any]:
        return {
            "mode": self.mode,
            "runtime": "bubblewrap" if self.mode == "required" else "none",
            "network": self.network,
            "network_allowlist": "unavailable" if self.network == "host" else "none",
            "private_home": self.mode == "required",
            "private_tmp": self.mode == "required",
            "minimal_etc": self.minimal_etc,
            "notes": (
                "Host networking is not an outbound allowlist. Graded paid runs "
                "require a verified egress mechanism or must stop."
                if self.network == "host"
                else "No network namespace route is shared with the host."
            ),
        }


def bubblewrap_available() -> bool:
    if shutil.which("bwrap") is None:
        return False
    try:
        completed = subprocess.run(
            [
                "bwrap",
                "--unshare-all",
                "--ro-bind",
                "/usr",
                "/usr",
                "--ro-bind-try",
                "/bin",
                "/bin",
                "--ro-bind-try",
                "/lib",
                "/lib",
                "--ro-bind-try",
                "/lib64",
                "/lib64",
                "--proc",
                "/proc",
                "--dev",
                "/dev",
                "--tmpfs",
                "/tmp",
                "--",
                "true",
            ],
            capture_output=True,
            text=True,
            timeout=10,
        )
    except (OSError, subprocess.TimeoutExpired):
        return False
    return completed.returncode == 0


def _parent_dir_args(path: Path) -> list[str]:
    """Create missing parent directories inside the sandbox for a bind target."""
    args: list[str] = []
    parents = [parent for parent in path.parents if parent != Path("/")]
    for parent in reversed(parents):
        text = str(parent)
        if text.startswith(("/home/", "/tmp/", "/run/")):
            args.extend(["--dir", text])
    return args


def _binds(policy: SandboxPolicy, home: Path) -> list[str]:
    args: list[str] = []
    for path in policy.extra_ro_binds:
        candidate = Path(path)
        if candidate.exists():
            source = candidate.resolve() if candidate.is_symlink() else candidate
            args.extend(_parent_dir_args(candidate))
            args.extend(["--ro-bind-try", str(source), str(candidate)])
    # Credentials are opt-in and mounted read-only. The entire agent state
    # directory must never be mounted implicitly.
    for path in policy.credential_binds:
        candidate = Path(path)
        if candidate.is_file():
            args.extend(_parent_dir_args(candidate))
            args.extend(["--ro-bind", str(candidate), str(candidate)])
    return args


def build_command(policy: SandboxPolicy, command: list[str]) -> list[str]:
    """Return the full command prefix for a sandboxed process."""
    if policy.mode == "none":
        return list(command)
    if policy.mode != "required":
        raise ValueError(f"unknown sandbox mode: {policy.mode!r}")
    if policy.workspace is None:
        raise SandboxUnavailable("sandbox workspace is not set")
    workspace = policy.workspace.resolve()
    home = Path(os.path.expanduser("~"))
    args = [
        "bwrap",
        "--unshare-all",
        "--new-session",
        "--die-with-parent",
        "--as-pid-1",
        "--chdir",
        str(workspace),
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
        "--ro-bind-try",
        "/lib32",
        "/lib32",
        "--ro-bind-try",
        "/etc/ld.so.cache",
        "/etc/ld.so.cache",
    ]
    if policy.minimal_etc:
        args.extend(["--tmpfs", "/etc"])
        for relative in (
            "ssl",
            "ca-certificates",
            "pki",
            "resolv.conf",
            "hosts",
            "passwd",
            "group",
            "nsswitch.conf",
            "localtime",
            "ld.so.cache",
            "terminfo",
            "alternatives",
        ):
            source = Path("/etc") / relative
            if source.exists():
                args.extend(["--ro-bind-try", str(source), str(source)])
    else:
        args.extend(["--ro-bind-try", "/etc", "/etc"])
    args.extend(
        [
            "--proc",
            "/proc",
            "--dev",
            "/dev",
            "--tmpfs",
            "/run",
            "--tmpfs",
            "/tmp",
            # Private HOME and XDG state. Nothing from the host home is visible
            # unless explicitly allowlisted below.
            "--dir",
            str(home),
            "--tmpfs",
            str(home),
            # Workspace is the only writable project state.
            "--bind",
            str(workspace),
            str(workspace),
        ]
    )
    args.extend(_binds(policy, home))
    if policy.network == "host":
        args.append("--share-net")
    # Environment: clear first, then set only the allowlist plus caller values.
    args.append("--clearenv")
    env = {
        "PATH": "/usr/local/bin:/usr/bin:/bin",
        "HOME": str(home),
        "LANG": "C.UTF-8",
        "LC_ALL": "C.UTF-8",
        "TERM": "dumb",
        "TMPDIR": "/tmp",
    }
    env.update(policy.env)
    for key, value in sorted(env.items()):
        if (
            key in {"HOME", "PATH", "LANG", "LC_ALL", "TERM", "TMPDIR", "PYTHONPATH", "PYTHONUNBUFFERED", "PYTHONDONTWRITEBYTECODE"}
            or key.startswith("GIVN_")
            or key.startswith("HARNESS_")
        ):
            args.extend(["--setenv", key, value])
        elif key in {"OPENCODE_CONFIG", "OPENCODE_CONFIG_DIR", "XDG_CONFIG_HOME", "XDG_DATA_HOME", "XDG_CACHE_HOME", "DEEPSEEK_API_KEY"}:
            # Credential-bearing values are injected only when explicitly
            # supplied by the caller; they are not copied from the host env.
            args.extend(["--setenv", key, value])
    args.append("--")
    args.extend(command)
    return args


def probe(policy: SandboxPolicy) -> dict[str, Any]:
    """Prove workspace visibility, host-state denial, and network denial."""
    checks: list[dict[str, Any]] = []
    if policy.mode == "none":
        return {
            "ok": True,
            "mode": "none",
            "checks": [
                {
                    "name": "sandbox-disabled",
                    "ok": True,
                    "detail": "explicitly unconfined; not a graded default",
                }
            ],
        }
    if not bubblewrap_available():
        return {
            "ok": False,
            "mode": "required",
            "checks": [
                {"name": "bubblewrap", "ok": False, "detail": "bwrap is unavailable or cannot unshare"}
            ],
        }
    if policy.workspace is None:
        return {
            "ok": False,
            "mode": "required",
            "checks": [{"name": "workspace", "ok": False, "detail": "workspace is not set"}],
        }
    with tempfile.TemporaryDirectory(prefix="givn-sandbox-probe-") as temporary:
        root = Path(temporary)
        workspace = root / "workspace"
        workspace.mkdir()
        sentinel_dir = root / "host-only"
        sentinel_dir.mkdir()
        sentinel = sentinel_dir / "sentinel"
        sentinel.write_text("host only", encoding="utf-8")
        inside = workspace / "visible"
        inside.write_text("visible", encoding="utf-8")
        test_policy = SandboxPolicy(
            mode=policy.mode,
            network=policy.network,
            workspace=workspace,
            extra_ro_binds=policy.extra_ro_binds,
            credential_binds=policy.credential_binds,
            env=policy.env,
            minimal_etc=policy.minimal_etc,
        )
        # Workspace is visible and writable.
        command = build_command(test_policy, ["/bin/sh", "-c", "printf ok > probe-write"])
        completed = subprocess.run(command, capture_output=True, text=True, timeout=20)
        checks.append(
            {
                "name": "workspace-writable",
                "ok": completed.returncode == 0 and (workspace / "probe-write").is_file(),
                "detail": (completed.stderr or completed.stdout).strip()[:300],
            }
        )
        # Host sentinel is not visible under the private /tmp or home.
        command = build_command(
            test_policy,
            [
                "/bin/sh",
                "-c",
                f"test ! -e {sentinel} && test ! -e {sentinel_dir}",
            ],
        )
        completed = subprocess.run(command, capture_output=True, text=True, timeout=20)
        checks.append(
            {
                "name": "host-state-hidden",
                "ok": completed.returncode == 0,
                "detail": "sentinel and its directory are not visible"
                if completed.returncode == 0
                else (completed.stderr or completed.stdout).strip()[:300],
            }
        )
        # Network denial, when requested.
        if policy.network == "none":
            command = build_command(
                test_policy,
                [
                    "/bin/sh",
                    "-c",
                    "python3 - <<'PY'\nimport socket\ns=socket.socket()\ntry:\n s.settimeout(1)\n s.connect(('1.1.1.1',443))\n print('connected')\nexcept OSError:\n print('denied')\nPY",
                ],
            )
            completed = subprocess.run(command, capture_output=True, text=True, timeout=20)
            checks.append(
                {
                    "name": "network-denied",
                    "ok": "denied" in completed.stdout,
                    "detail": (completed.stdout or completed.stderr).strip()[:300],
                }
            )
    return {
        "ok": all(check["ok"] for check in checks),
        "mode": policy.mode,
        "checks": checks,
    }


def require_isolated(policy: SandboxPolicy) -> dict[str, Any]:
    if policy.mode == "none":
        raise SandboxUnavailable(
            "graded execution requires isolation; explicit --sandbox none is not a graded fallback"
        )
    result = probe(policy)
    if not result["ok"]:
        failed = ", ".join(
            check["name"] for check in result["checks"] if not check["ok"]
        )
        raise SandboxUnavailable(f"bubblewrap isolation probe failed: {failed}")
    return result
