import os
import shutil
import subprocess


def _deduplicate_paths(paths: list[str]) -> list[str]:
    if not paths:
        return []
    normalized = [p.rstrip("/") for p in paths if p and p != "/"]
    if not normalized:
        return []
    normalized.sort(key=lambda p: p.count("/"))
    result: list[str] = []
    for p in normalized:
        if any(p == kept or p.startswith(kept + "/") for kept in result):
            continue
        result.append(p)
    return sorted(result)


def check_bubblewrap() -> bool:
    if (
        os.environ.get("GIVN_HARNESS_NO_SANDBOX") == "1"
        or os.environ.get("GIVN_HARNESS_NO_SANDLOCK") == "1"
    ):
        return False
    if shutil.which("bwrap") is None:
        return False
    try:
        subprocess.run(
            [
                "bwrap", "--unshare-user",
                "--ro-bind", "/usr", "/usr",
                "--ro-bind-try", "/lib64", "/lib64",
                "--ro-bind-try", "/lib", "/lib",
                "--", "true",
            ],
            capture_output=True, timeout=5, check=True,
        )
        return True
    except Exception:
        return False


def _resolved_binds(home: str) -> list[str]:
    """Bind resolved agent binaries/configs whose symlink targets live outside home.

    maki is commonly installed as a symlink into a dotfiles checkout; bwrap
    binds ``~/.local`` but not that target, so execvp fails with ENOENT until the
    resolved path is mounted too.
    """
    args: list[str] = []
    candidates = [
        shutil.which("omp"),
        shutil.which("maki"),
        os.path.join(home, ".config", "maki"),
        os.path.join(home, ".local", "share", "maki"),
    ]
    seen: set[str] = set()
    for candidate in candidates:
        if not candidate or not os.path.exists(candidate):
            continue
        real = os.path.realpath(candidate)
        if real == candidate or real in seen or not os.path.exists(real):
            continue
        seen.add(real)
        args.extend(["--ro-bind-try", real, real])
    return args


def build_bwrap_cmd(workspace_path: str) -> list[str]:
    workspace_path = os.path.abspath(workspace_path)
    home = os.path.expanduser("~")
    args = [
        "bwrap",
        "--unshare-all",
        "--share-net",
        "--new-session",
        "--die-with-parent",
        "--as-pid-1",
        "--chdir", workspace_path,
        "--ro-bind", "/usr", "/usr",
        "--ro-bind", "/bin", "/bin",
        "--ro-bind-try", "/lib", "/lib",
        "--ro-bind-try", "/lib64", "/lib64",
        "--ro-bind", "/etc", "/etc",
        "--ro-bind-try", os.path.join(home, "bin"), os.path.join(home, "bin"),
        "--ro-bind-try", os.path.join(home, ".cargo"), os.path.join(home, ".cargo"),
        "--ro-bind-try", os.path.join(home, ".opencode"), os.path.join(home, ".opencode"),
        "--ro-bind-try", os.path.join(home, ".gitconfig"), os.path.join(home, ".gitconfig"),
        "--ro-bind-try", os.path.join(home, ".config", "opencode"), os.path.join(home, ".config", "opencode"),
        "--bind-try", os.path.join(home, ".omp"), os.path.join(home, ".omp"),
        "--ro-bind-try", os.path.join(home, ".config", "maki"), os.path.join(home, ".config", "maki"),
        # A private /tmp: only this run's workspace is visible. Without this,
        # an agent can read previous runs and unrelated harness fixtures under
        # the shared /tmp (both opencode and omp did in the first campaign).
        "--tmpfs", "/tmp",
        "--dir", os.path.dirname(workspace_path),
        "--bind", workspace_path, workspace_path,
        "--ro-bind-try", os.path.join(workspace_path, "givn", "config.yaml"),
        os.path.join(workspace_path, "givn", "config.yaml"),
        "--bind-try", os.path.join(home, ".npm"), os.path.join(home, ".npm"),
        "--bind-try", os.path.join(home, ".cache"), os.path.join(home, ".cache"),
        "--bind-try", os.path.join(home, ".bun"), os.path.join(home, ".bun"),
        "--bind-try", os.path.join(home, ".local"), os.path.join(home, ".local"),
        "--proc", "/proc",
        "--dev", "/dev",
        "--tmpfs", "/run",
        "--ro-bind-try", "/run/systemd/resolve", "/run/systemd/resolve",
    ]
    args.extend(_resolved_binds(home))
    return args


def _wrap_with_limits(cmd: list[str]) -> tuple[list[str], str]:
    if shutil.which("systemd-run") is None:
        return (cmd, "none")
    return (
        [
            "systemd-run", "--user", "--scope",
            "-p", "MemoryMax=16G",
            "-p", "TasksMax=256",
            "--",
        ] + cmd,
        "systemd",
    )