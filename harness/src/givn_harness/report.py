from givn_harness.types import Check, JudgeScore


def generate_report_structure(
    checks: list[Check],
    workspace_path: str,
    judge_scores: list[JudgeScore] | None = None,
) -> dict:
    result: dict = {
        "workspace_path": workspace_path,
        "checks": checks,
        "all_pass": all(c.passed for c in checks),
    }
    if judge_scores is not None:
        result["judge_scores"] = judge_scores
    return result
