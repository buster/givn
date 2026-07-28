import os
from pathlib import Path


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


def check_sandlock() -> bool:
    if os.environ.get("GIVN_HARNESS_NO_SANDLOCK") == "1":
        return False
    try:
        import sandlock  # noqa: F401
        return True
    except ImportError:
        return False


def _base_sandbox_args(workspace_path: str) -> dict:
    return dict(
        name="givn-harness",
        workdir=workspace_path,
        cwd=workspace_path,
        fs_writable=_deduplicate_paths([
            workspace_path,
            "/tmp",
            os.path.expanduser("~/.npm"),
            os.path.expanduser("~/.cache"),
            os.path.expanduser("~/.bun"),
            os.path.expanduser("~/.config"),
            os.path.expanduser("~/.local"),
        ]),
        fs_readable=_deduplicate_paths([
            "/dev/null",
            "/dev/zero",
            "/dev/urandom",
            "/dev/random",
            "/dev/fd",
            "/usr",
            "/bin",
            "/lib",
            "/lib64",
            "/etc",
            os.path.expanduser("~"),
        ]),
        clean_env=False,
        max_processes=256,
        max_memory="16G",
        max_disk="20G",
        port_remap=True,
        on_exit=None,
        allow_degraded=[4, 5, 3],  # SIGNAL_SCOPE, ABSTRACT_UNIX_SOCKET_SCOPE, FS_IOCTL_DEV
    )


def _profiles_dir() -> Path:
    import sandlock._profile

    return sandlock._profile.profiles_dir()


def create_sandbox(workspace_path: str, profile_name: str | None = None) -> dict:
    from sandlock import BranchAction, Sandbox

    args = _base_sandbox_args(workspace_path)
    args["on_exit"] = BranchAction.COMMIT

    if profile_name is None:
        return {
            "sandbox": Sandbox(**args),
            "mode": "basic",
            "profile_path": None,
            "profile_name": None,
        }

    profile_path = _profiles_dir() / f"{profile_name}.toml"

    if profile_path.exists():
        import sandlock._profile as sp

        sandbox = sp.load_profile_path(profile_path)
        sandbox.name = "givn-harness"
        sandbox.workdir = workspace_path
        sandbox.cwd = workspace_path
        sandbox.fs_readable = _deduplicate_paths(
            list(sandbox.fs_readable) + args["fs_readable"]
        )
        sandbox.fs_writable = _deduplicate_paths(
            list(sandbox.fs_writable) + args["fs_writable"]
        )
        if workspace_path not in sandbox.fs_readable:
            sandbox.fs_readable.append(workspace_path)
        if workspace_path not in sandbox.fs_writable:
            sandbox.fs_writable.append(workspace_path)
        sandbox.clean_env = args["clean_env"]
        sandbox.port_remap = args["port_remap"]
        sandbox.on_exit = args["on_exit"]
        sandbox.allow_degraded = args["allow_degraded"]
        return {
            "sandbox": sandbox,
            "mode": "enforce",
            "profile_path": profile_path,
            "profile_name": profile_name,
        }

    return {
        "sandbox": Sandbox(**args),
        "mode": "learn",
        "profile_path": profile_path,
        "profile_name": profile_name,
    }