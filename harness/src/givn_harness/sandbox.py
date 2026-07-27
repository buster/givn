import os


def check_sandlock() -> bool:
    if os.environ.get("GIVN_HARNESS_NO_SANDLOCK") == "1":
        return False
    try:
        import sandlock  # noqa: F401
        return True
    except ImportError:
        return False