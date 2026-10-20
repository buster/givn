from __future__ import annotations

import json
import tempfile
import time
import unittest
from pathlib import Path

from givn_harness.driver import QuestionWatcher


def append_jsonl(path: Path, payload: dict) -> None:
    with path.open("a", encoding="utf-8") as handle:
        handle.write(json.dumps(payload) + "\n")


def wait_for(predicate, timeout: float = 3.0) -> bool:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return True
        time.sleep(0.05)
    return predicate()


class QuestionWatcherTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.workspace = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def read_answers(self) -> list[dict]:
        path = self.workspace / "answers.jsonl"
        if not path.is_file():
            return []
        return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]

    def test_unambiguous_recommended_option_is_answered(self) -> None:
        watcher = QuestionWatcher(self.workspace, policy="recommended", poll_seconds=0.05)
        watcher.start()
        try:
            append_jsonl(
                self.workspace / "questions.jsonl",
                {
                    "id": "q1",
                    "question": "Which storage?",
                    "options": ["JSON file", "SQLite", "Postgres"],
                    "recommended": 0,
                },
            )
            self.assertTrue(wait_for(lambda: len(self.read_answers()) == 1))
        finally:
            watcher.stop()
        answers = self.read_answers()
        self.assertEqual(answers[0]["selected"], "JSON file")
        self.assertEqual(answers[0]["policy"], "recommended")

    def test_starred_recommendation_is_accepted_once(self) -> None:
        watcher = QuestionWatcher(self.workspace, policy="recommended", poll_seconds=0.05)
        watcher.start()
        try:
            append_jsonl(
                self.workspace / "questions.jsonl",
                {
                    "id": "q2",
                    "question": "Which name?",
                    "options": ["alpha", "*beta", "gamma"],
                },
            )
            self.assertTrue(wait_for(lambda: len(self.read_answers()) == 1))
        finally:
            watcher.stop()
        self.assertEqual(self.read_answers()[0]["selected"], "beta")

    def test_ambiguous_question_is_a_protocol_failure_not_a_guess(self) -> None:
        watcher = QuestionWatcher(self.workspace, policy="recommended", poll_seconds=0.05)
        watcher.start()
        try:
            append_jsonl(
                self.workspace / "questions.jsonl",
                {
                    "id": "q3",
                    "question": "Which database?",
                    "options": ["SQLite", "Postgres"],
                },
            )
            self.assertTrue(wait_for(lambda: len(self.read_answers()) == 1))
        finally:
            watcher.stop()
        answers = self.read_answers()
        self.assertEqual(answers[0]["status"], "protocol_failure")
        self.assertTrue(any("unambiguous" in failure for failure in watcher.failures))

    def test_policy_none_does_not_answer(self) -> None:
        watcher = QuestionWatcher(self.workspace, policy="none", poll_seconds=0.05)
        watcher.start()
        try:
            append_jsonl(
                self.workspace / "questions.jsonl",
                {
                    "id": "q4",
                    "question": "Which?",
                    "options": ["a", "b"],
                    "recommended": 0,
                },
            )
            time.sleep(0.2)
        finally:
            watcher.stop()
        self.assertEqual(self.read_answers(), [])


if __name__ == "__main__":
    unittest.main()
