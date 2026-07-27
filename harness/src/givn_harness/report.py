from givn_harness.types import Check


def generate_report_structure(checks: list[Check], workspace_path: str) -> dict:
    return {
        "workspace_path": workspace_path,
        "checks": checks,
        "all_pass": all(c.passed for c in checks),
    }
