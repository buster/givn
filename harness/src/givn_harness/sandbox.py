import os
from pathlib import Path


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
        fs_writable=[
            workspace_path,
            "/tmp",
            os.path.expanduser("~/.npm"),
            os.path.expanduser("~/.cache"),
            os.path.expanduser("~/.bun"),
            os.path.expanduser("~/.config"),
            os.path.expanduser("~/.local"),
        ],
        fs_readable=[
            "/usr",
            "/bin",
            "/lib",
            "/lib64",
            "/etc",
            os.path.expanduser("~"),
            os.path.expanduser("~/.config"),
            os.path.expanduser("~/.config/opencode"),
            os.path.expanduser("~/.local"),
            os.path.expanduser("~/.cargo"),
            os.path.expanduser("~/.npm"),
            os.path.expanduser("~/.bun"),
            os.path.expanduser("~/.cache"),
            os.path.expanduser("~/bin"),
        ],
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