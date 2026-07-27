import os


def check_ralph_binary(ralph_bin: str) -> bool:
    return os.path.isfile(ralph_bin)