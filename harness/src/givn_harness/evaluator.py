import os
import re
import subprocess
from pathlib import Path

from givn_harness.types import Check

GIVN_BINARY = os.environ.get("GIVN_BINARY", "givn")

CONVENTIONAL_COMMIT_RE = re.compile(
    r"^(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)"
    r"(\([a-zA-Z0-9_.-]+\))?!?: .+"
)


def _find_change_dir(ws: Path) -> Path | None:
    changes_dir = ws / "givn" / "changes"
    if not changes_dir.is_dir():
        return None
    for entry in sorted(changes_dir.iterdir()):
        if entry.is_dir():
            return entry
    return None


def _find_archive_dir(ws: Path) -> Path | None:
    archive_dir = ws / "givn" / "archive"
    if not archive_dir.is_dir():
        return None
    for entry in sorted(archive_dir.iterdir()):
        if entry.is_dir():
            return entry
    return None


def _check_artifact_existence(ws: Path) -> Check:
    change_dir = _find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="artifact existence",
            passed=False,
            detail="no change directory found in givn/changes/",
        )
    expected = ["proposal.md", "design.md", "design-review.md", "tasks.md", "review.md"]
    missing = [f for f in expected if not (change_dir / f).is_file()]
    specs_dir = change_dir / "specs"
    feature_files = list(specs_dir.rglob("*.feature")) if specs_dir.is_dir() else []
    if missing:
        detail = f"missing: {', '.join(missing)}"
        if not feature_files:
            detail += "; no .feature files found"
        return Check(name="artifact existence", passed=False, detail=detail)
    if not feature_files:
        return Check(
            name="artifact existence",
            passed=False,
            detail="no .feature files in change specs",
        )
    return Check(
        name="artifact existence",
        passed=True,
        detail=f"all {len(expected)} artifacts present, {len(feature_files)} .feature file(s)",
    )


def _check_lint_clean(ws: Path) -> Check:
    try:
        result = subprocess.run(
            [GIVN_BINARY, "lint"],
            cwd=str(ws),
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            return Check(name="lint clean", passed=True, detail="givn lint passed")
        return Check(
            name="lint clean",
            passed=False,
            detail=f"givn lint failed: {result.stderr.strip() or result.stdout.strip()}",
        )
    except FileNotFoundError:
        return Check(
            name="lint clean",
            passed=False,
            detail="givn binary not found",
        )


def _check_tasks_all_done(ws: Path) -> Check:
    change_dir = _find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="tasks all done",
            passed=False,
            detail="no change directory found",
        )
    tasks_path = change_dir / "tasks.md"
    if not tasks_path.is_file():
        return Check(
            name="tasks all done",
            passed=False,
            detail="tasks.md not found",
        )
    content = tasks_path.read_text()
    unchecked = [line.strip() for line in content.splitlines() if line.strip().startswith("- [ ]")]
    if unchecked:
        return Check(
            name="tasks all done",
            passed=False,
            detail=f"{len(unchecked)} unchecked task(s): {unchecked[0][:80]}",
        )
    checked = [line.strip() for line in content.splitlines() if line.strip().startswith("- [x]")]
    if not checked:
        return Check(
            name="tasks all done",
            passed=False,
            detail="no checked tasks found in tasks.md",
        )
    return Check(
        name="tasks all done",
        passed=True,
        detail=f"all {len(checked)} task(s) checked",
    )


def _check_valid_commits(ws: Path) -> Check:
    try:
        result = subprocess.run(
            ["git", "log", "--oneline"],
            cwd=str(ws),
            capture_output=True,
            text=True,
        )
        if result.returncode != 0:
            return Check(
                name="valid commits present",
                passed=False,
                detail="git log failed — no repository or not a git repo",
            )
        lines = [l.strip() for l in result.stdout.splitlines() if l.strip()]
        if not lines:
            return Check(
                name="valid commits present",
                passed=False,
                detail="no commits found",
            )
        invalid = []
        for line in lines:
            # Format: <hash> <message>
            parts = line.split(" ", 1)
            if len(parts) < 2:
                invalid.append(line)
                continue
            msg = parts[1]
            if not CONVENTIONAL_COMMIT_RE.match(msg):
                invalid.append(line)
        if invalid:
            return Check(
                name="valid commits present",
                passed=False,
                detail=f"{len(invalid)} non-conventional commit(s): {invalid[0][:80]}",
            )
        return Check(
            name="valid commits present",
            passed=True,
            detail=f"all {len(lines)} commit(s) follow Conventional Commits",
        )
    except FileNotFoundError:
        return Check(
            name="valid commits present",
            passed=False,
            detail="git binary not found",
        )


def _check_archive_succeeded(ws: Path) -> Check:
    archive_dir = _find_archive_dir(ws)
    if archive_dir is None:
        return Check(
            name="archive succeeded",
            passed=False,
            detail="no archive directory found in givn/archive/",
        )
    expected = ["proposal.md", "design.md"]
    missing = [f for f in expected if not (archive_dir / f).is_file()]
    specs_dir = archive_dir / "specs"
    feature_files = list(specs_dir.rglob("*.feature")) if specs_dir.is_dir() else []
    permanent_specs = list((ws / "givn" / "specs").rglob("*.feature"))
    if missing:
        return Check(
            name="archive succeeded",
            passed=False,
            detail=f"archive missing: {', '.join(missing)}",
        )
    if not feature_files and not permanent_specs:
        return Check(
            name="archive succeeded",
            passed=False,
            detail="no .feature files in archive or permanent specs",
        )
    return Check(
        name="archive succeeded",
        passed=True,
        detail="change archived with specs",
    )


def _check_specs_clean(ws: Path) -> Check:
    specs_dir = ws / "givn" / "specs"
    if not specs_dir.is_dir():
        return Check(
            name="permanent specs are clean Gherkin",
            passed=False,
            detail="givn/specs/ directory not found",
        )
    try:
        result = subprocess.run(
            [GIVN_BINARY, "lint", str(specs_dir)],
            cwd=str(ws),
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            return Check(name="permanent specs are clean Gherkin", passed=True, detail="permanent specs are clean Gherkin")
        return Check(
            name="permanent specs are clean Gherkin",
            passed=False,
            detail=f"specs lint failed: {result.stderr.strip() or result.stdout.strip()}",
        )
    except FileNotFoundError:
        return Check(
            name="permanent specs are clean Gherkin",
            passed=False,
            detail="givn binary not found",
        )


def _check_step_files_nonempty(ws: Path) -> Check:
    steps_dir = ws / "tests" / "features" / "steps"
    if not steps_dir.is_dir():
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail="tests/features/steps/ directory not found",
        )
    step_files = list(steps_dir.rglob("*_steps.py"))
    if not step_files:
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail="no *_steps.py files found",
        )
    empty_steps: list[str] = []
    for sf in step_files:
        content = sf.read_text()
        lines = content.splitlines()
        i = 0
        while i < len(lines):
            if "def step_" in lines[i]:
                j = i + 1
                # Skip past the def line
                body_lines = []
                current_indent = None
                while j < len(lines):
                    line = lines[j]
                    if line.strip() and not line[0].isspace():
                        # Next top-level construct — stop
                        break
                    if current_indent is None and line.strip():
                        current_indent = len(line) - len(line.lstrip())
                    if line.strip() and current_indent is not None:
                        body_lines.append(line.strip())
                    j += 1
                # Filter out pass lines
                meaningful = [bl for bl in body_lines if bl != "pass"]
                if not meaningful:
                    func_name = lines[i].strip().split("(")[0].replace("def ", "")
                    empty_steps.append(f"{sf.name}:{func_name}")
                i = j
            else:
                i += 1
    if empty_steps:
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail=f"{len(empty_steps)} empty step(s): {', '.join(empty_steps[:3])}",
        )
    return Check(
        name="step definition files are non-empty",
        passed=True,
        detail=f"all step bodies non-empty in {len(step_files)} file(s)",
    )


def _check_review_pass(ws: Path) -> Check:
    change_dir = _find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="REVIEW: PASS marker present",
            passed=False,
            detail="no change directory found",
        )
    review_path = change_dir / "review.md"
    if not review_path.is_file():
        return Check(
            name="REVIEW: PASS marker present",
            passed=False,
            detail="review.md not found",
        )
    content = review_path.read_text()
    if "REVIEW: PASS" in content:
        return Check(name="REVIEW: PASS marker present", passed=True, detail="REVIEW: PASS marker present")
    return Check(
        name="REVIEW: PASS marker present",
        passed=False,
        detail="REVIEW: PASS marker not found in review.md",
    )


def _check_design_review_pass(ws: Path) -> Check:
    change_dir = _find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="DESIGN-REVIEW: PASS marker present",
            passed=False,
            detail="no change directory found",
        )
    dr_path = change_dir / "design-review.md"
    if not dr_path.is_file():
        return Check(
            name="DESIGN-REVIEW: PASS marker present",
            passed=False,
            detail="design-review.md not found",
        )
    content = dr_path.read_text()
    if "DESIGN-REVIEW: PASS" in content:
        return Check(name="DESIGN-REVIEW: PASS marker present", passed=True, detail="DESIGN-REVIEW: PASS marker present")
    return Check(
        name="DESIGN-REVIEW: PASS marker present",
        passed=False,
        detail="DESIGN-REVIEW: PASS marker not found in design-review.md",
    )


def _check_verify_green(ws: Path) -> Check:
    try:
        result = subprocess.run(
            [GIVN_BINARY, "check", "review"],
            cwd=str(ws),
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            return Check(name="verify.command exits zero after archive", passed=True, detail="verify.command exits zero after archive")
        return Check(
            name="verify.command exits zero after archive",
            passed=False,
            detail=f"verify.command failed: {result.stderr.strip() or result.stdout.strip()}",
        )
    except FileNotFoundError:
        return Check(
            name="verify.command exits zero after archive",
            passed=False,
            detail="givn binary not found",
        )


def run_evaluation(workspace_path: str) -> list[Check]:
    ws = Path(workspace_path).resolve()
    checks: list[Check] = []
    checks.append(_check_artifact_existence(ws))
    checks.append(_check_lint_clean(ws))
    checks.append(_check_tasks_all_done(ws))
    checks.append(_check_valid_commits(ws))
    checks.append(_check_archive_succeeded(ws))
    checks.append(_check_specs_clean(ws))
    checks.append(_check_step_files_nonempty(ws))
    checks.append(_check_review_pass(ws))
    checks.append(_check_design_review_pass(ws))
    checks.append(_check_verify_green(ws))
    return checks
