import os


def check_sandlock() -> bool:
    if os.environ.get("GIVN_HARNESS_NO_SANDLOCK") == "1":
        return False
    try:
        import sandlock  # noqa: F401
        return True
    except ImportError:
        return False


def create_sandbox(workspace_path: str):
    from sandlock import BranchAction, Sandbox

    return Sandbox(
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
        on_exit=BranchAction.COMMIT,
    )