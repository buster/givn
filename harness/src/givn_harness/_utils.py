from pathlib import Path


def _first_dir(path: Path) -> Path | None:
    if not path.is_dir():
        return None
    for entry in sorted(path.iterdir()):
        if entry.is_dir():
            return entry
    return None


def find_change_dir(ws: Path) -> Path | None:
    active = _first_dir(ws / "givn" / "changes")
    if active is not None:
        return active
    return _first_dir(ws / "givn" / "archive")