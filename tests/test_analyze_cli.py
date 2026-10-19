from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from click.testing import CliRunner

from givn_harness.cli import main
from tests.support import event, write_events


class AnalyzeCliTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_analyze_writes_markdown_and_json_without_executing_sources(self) -> None:
        events = write_events(
            self.root / "events.jsonl",
            [
                event(1, at="2026-01-01T00:00:01Z", kind="tool", name="read", duration_ms=100),
                event(2, at="2026-01-01T00:00:02Z", kind="turn", name="turn 0", data={"usage": {"input": 5}}),
            ],
        )
        output = self.root / "out"
        result = CliRunner().invoke(
            main, ["analyze", str(events), "--output", str(output)]
        )
        self.assertEqual(result.exit_code, 0, result.output)
        self.assertTrue((output / "analysis.md").is_file())
        document = json.loads((output / "analysis.json").read_text())
        self.assertEqual(document["schema"], "givn-harness/analysis/v1")
        self.assertEqual(document["data"]["corpus"]["events"], 2)
        self.assertIn("analysis.md", result.output)

    def test_invalid_timestamp_filter_is_a_clean_error(self) -> None:
        events = write_events(self.root / "events.jsonl", [event(1, at="2026-01-01T00:00:01Z")])
        result = CliRunner().invoke(
            main,
            ["analyze", str(events), "--output", str(self.root / "out"), "--since", "yesterday"],
        )
        self.assertNotEqual(result.exit_code, 0)
        self.assertIn("ISO-8601", result.output)

    def test_export_is_redacted_by_default(self) -> None:
        events = write_events(
            self.root / "events.jsonl",
            [
                event(
                    1,
                    at="2026-01-01T00:00:10Z",
                    kind="question",
                    name="question",
                    label="Secret question",
                    duration_ms=1000,
                    data={"question": {"text": "Secret question text", "truncated": False}},
                )
            ],
        )
        output = self.root / "out"
        result = CliRunner().invoke(main, ["analyze", str(events), "--output", str(output)])
        self.assertEqual(result.exit_code, 0, result.output)
        report = (output / "analysis.md").read_text()
        self.assertNotIn("Secret question text", report)
        self.assertIn("redacted", report)

    def test_no_redact_keeps_local_detail(self) -> None:
        events = write_events(
            self.root / "events.jsonl",
            [
                event(
                    1,
                    at="2026-01-01T00:00:10Z",
                    kind="question",
                    name="question",
                    label="Visible question",
                    duration_ms=1000,
                    data={"question": {"text": "Visible question text", "truncated": False}},
                )
            ],
        )
        output = self.root / "out"
        result = CliRunner().invoke(
            main, ["analyze", str(events), "--output", str(output), "--no-redact"]
        )
        self.assertEqual(result.exit_code, 0, result.output)
        self.assertIn("Visible question text", (output / "analysis.md").read_text())


if __name__ == "__main__":
    unittest.main()
