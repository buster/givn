import json
import os
import signal
import subprocess
import tempfile
from pathlib import Path

from givn_harness._utils import find_change_dir
from givn_harness.types import JudgeScore

ARTIFACT_TYPES = ["proposal", "specs", "design", "tasks", "review"]

JUDGE_PROMPT_TEMPLATE = """You are evaluating the quality of a givn artifact produced by an AI agent
working on a task. Rate the artifact on a 0-4 scale:

0 - Missing or empty (no meaningful content)
1 - Minimal (exists but lacks substance)
2 - Adequate (covers basics but has gaps)
3 - Good (substantive, follows givn conventions, minor issues only)
4 - Excellent (complete, well-structured, follows all givn artifact instructions)

Respond ONLY with a JSON object: {{"score": <int>, "explanation": "<text>"}}

Task: {task}
Artifact type: {artifact_type}
Artifact content:
{content}"""


def _get_artifact_content(change_dir: Path, artifact_type: str) -> str:
    if artifact_type == "specs":
        specs_dir = change_dir / "specs"
        if not specs_dir.is_dir():
            return ""
        feature_files = list(specs_dir.rglob("*.feature"))
        return "\n\n".join(f.read_text() for f in sorted(feature_files))
    else:
        file_path = change_dir / f"{artifact_type}.md"
        if file_path.is_file():
            return file_path.read_text()
        return ""


def run_judge(workspace_path: str) -> list[JudgeScore]:
    ws = Path(workspace_path).resolve()
    change_dir = find_change_dir(ws)

    if change_dir is None:
        return [
            JudgeScore(artifact=at, score=0, explanation="no change directory found")
            for at in ARTIFACT_TYPES
        ]

    task_path = ws / "task.md"
    task = task_path.read_text() if task_path.is_file() else "No task description"

    scores: list[JudgeScore] = []
    for artifact_type in ARTIFACT_TYPES:
        content = _get_artifact_content(change_dir, artifact_type)
        if not content:
            scores.append(JudgeScore(
                artifact=artifact_type,
                score=0,
                explanation="artifact content is empty or missing",
            ))
            continue

        prompt = JUDGE_PROMPT_TEMPLATE.format(
            task=task,
            artifact_type=artifact_type,
            content=content,
        )

        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as f:
            f.write(prompt)
            prompt_file = f.name

        proc: subprocess.Popen | None = None
        try:
            proc = subprocess.Popen(
                ["opencode", "--prompt-file", prompt_file],
                start_new_session=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            stdout, _stderr = proc.communicate(timeout=300)
            parsed = json.loads(stdout.strip())
            scores.append(JudgeScore(
                artifact=artifact_type,
                score=parsed["score"],
                explanation=parsed["explanation"],
            ))
        except subprocess.TimeoutExpired:
            if proc is not None:
                try:
                    os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
                except (ProcessLookupError, OSError):
                    pass
                proc.communicate()
            scores.append(JudgeScore(
                artifact=artifact_type,
                score=0,
                explanation="judge output unparseable",
            ))
        except (json.JSONDecodeError, KeyError):
            scores.append(JudgeScore(
                artifact=artifact_type,
                score=0,
                explanation="judge output unparseable",
            ))
        finally:
            Path(prompt_file).unlink(missing_ok=True)

    return scores