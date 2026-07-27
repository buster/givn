from pathlib import Path


def find_change_dir(ws: Path) -> Path | None:
    changes_dir = ws / "givn" / "changes"
    if not changes_dir.is_dir():
        return None
    for entry in sorted(changes_dir.iterdir()):
        if entry.is_dir():
            return entry
    return None