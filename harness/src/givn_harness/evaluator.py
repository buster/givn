import os
import re
import subprocess
from pathlib import Path

from givn_harness._utils import find_change_dir
from givn_harness.types import Check

GIVN_BINARY = os.environ.get("GIVN_BINARY", "givn")

CONVENTIONAL_COMMIT_RE = re.compile(
    r"^(build|chore|ci|docs|feat|fix|perf|refactor|revert|style|test)"
    r"(\([a-zA-Z0-9_.-]+\))?!?: .+"
)


def _find_archive_dir(ws: Path) -> Path | None:
    archive_dir = ws / "givn" / "archive"
    if not archive_dir.is_dir():
        return None
    for entry in sorted(archive_dir.iterdir()):
        if entry.is_dir():
            return entry
    return None


def _check_artifact_existence(ws: Path) -> Check:
    change_dir = find_change_dir(ws)
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
    change_dir = find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="lint clean",
            passed=False,
            detail="no change directory found — nothing to lint",
        )
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
    change_dir = find_change_dir(ws)
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


def _extract_section(markdown: str, heading: str) -> str | None:
    """Return the content between a given ## heading and the next ## heading."""
    for line in markdown.splitlines():
        if line.strip() == f"## {heading}":
            start = markdown.index(line) + len(line)
            rest = markdown[start:]
            end = rest.find("\n## ")
            section = rest[:end] if end != -1 else rest
            return section.strip()
    return None


def _parse_step_table(design_md: str) -> list[str]:
    """Extract step definition file paths from design.md's Step Definitions table.
    
    Returns file paths (column 2 of each data row) from the markdown table
    under the `## Step Definitions` heading. Mirrors the logic in
    givn's integrity_hook.rs:check_promised_files_exist.
    """
    in_section = False
    in_table = False
    files: list[str] = []
    for line in design_md.splitlines():
        if line.startswith("## Step Definitions"):
            in_section = True
            in_table = False
            continue
        if in_section and line.startswith("## "):
            break
        if in_section and line.startswith("|"):
            cols = [c.strip() for c in line.split("|")[1:-1]]
            if len(cols) < 2:
                continue
            # Skip header row (contains "Capability") and separator row
            if "Capability" in cols[0] or set(cols[0]) <= set("-"):
                in_table = True
                continue
            if in_table:
                path = cols[1].strip()
                if path:
                    files.append(path)
    return files


def _check_step_files_nonempty(ws: Path) -> Check:
    change_dir = find_change_dir(ws)
    if change_dir is None:
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail="no change directory found",
        )
    design_path = change_dir / "design.md"
    if not design_path.is_file():
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail="design.md not found in change directory",
        )
    design_content = design_path.read_text()
    promised_files = _parse_step_table(design_content)
    if not promised_files:
        # Check if the Step Definitions section explicitly says N/A / none
        section = _extract_section(design_content, "Step Definitions")
        if section and any(kw in section.lower() for kw in ("n/a", "none", "not applicable")):
            return Check(
                name="step definition files are non-empty",
                passed=True,
                detail="Step Definitions explicitly marked as N/A in design.md",
            )
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail="no step definition file paths found in design.md Step Definitions table",
        )
    missing: list[str] = []
    empty: list[str] = []
    for rel_path in promised_files:
        candidate = (ws / rel_path).resolve()
        if not candidate.exists():
            missing.append(rel_path)
            continue
        if candidate.stat().st_size == 0:
            empty.append(rel_path)
            continue
    if missing:
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail=f"step file(s) missing: {', '.join(missing[:3])}",
        )
    if empty:
        return Check(
            name="step definition files are non-empty",
            passed=False,
            detail=f"step file(s) empty: {', '.join(empty[:3])}",
        )
    return Check(
        name="step definition files are non-empty",
        passed=True,
        detail=f"all {len(promised_files)} step definition file(s) exist and are non-empty",
    )


def _check_review_pass(ws: Path) -> Check:
    change_dir = find_change_dir(ws)
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
    change_dir = find_change_dir(ws)
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


def _check_addons_intact(ws: Path) -> Check:
    config_path = ws / "givn" / "config.yaml"
    if not config_path.exists():
        return Check(
            name="givn addons remain enabled",
            passed=False,
            detail="givn/config.yaml not found",
        )
    import re

    content = config_path.read_text()
    disabled: list[str] = []
    for addon in ("coverage", "arc42", "readme", "absolute_mode", "domain_modeling", "dev_principles"):
        if re.search(rf"^\s*{addon}\s*:\s*false", content, re.MULTILINE):
            disabled.append(addon)
    if disabled:
        return Check(
            name="givn addons remain enabled",
            passed=False,
            detail=f"addon(s) disabled by agent: {', '.join(disabled)}. Re-enable with: givn addons enable <name>",
        )
    return Check(name="givn addons remain enabled", passed=True, detail="all addons enabled")


def _check_readme_substance(ws: Path) -> Check:
    readme_path = ws / "README.md"
    if not readme_path.is_file():
        return Check(
            name="readme substance",
            passed=False,
            detail="README.md not found at workspace root",
        )
    content = readme_path.read_text()
    stripped = re.sub(
        r"<!--\s*givn:begin:.*?-->\n.*?<!--\s*givn:end:.*?-->\n?",
        "",
        content,
        flags=re.DOTALL,
    )
    # Strip all HTML comments (placeholder instructions) and markdown headers.
    stripped = re.sub(r"<!--.*?-->", "", stripped, flags=re.DOTALL)
    body = re.sub(r"^#+\s+.*$", "", stripped, flags=re.MULTILINE)
    meaningful = "".join(body.split())
    threshold = 200
    if len(meaningful) >= threshold:
        return Check(
            name="readme substance",
            passed=True,
            detail=f"README.md has {len(meaningful)} significant body chars (threshold: {threshold})",
        )
    return Check(
        name="readme substance",
        passed=False,
        detail=f"README.md has only {len(meaningful)} significant body chars (threshold: {threshold}); likely a placeholder",
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
    checks.append(_check_addons_intact(ws))
    checks.append(_check_readme_substance(ws))
    return checks
