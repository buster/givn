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
        ],
        fs_readable=[
            "/usr",
            "/bin",
            "/lib",
            "/lib64",
            "/etc",
            os.path.expanduser("~/.config/opencode"),
            os.path.expanduser("~/.cargo"),
            os.path.expanduser("~/.local"),
        ],
        clean_env=False,
        max_processes=256,
        max_memory="16G",
        max_disk="20G",
        port_remap=True,
        on_exit=None,
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