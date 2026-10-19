from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.accounting import analyze_paths
from givn_harness.events import freeze_source, iter_events, read_sources
from givn_harness.types import Diagnostic
from tests.support import event, write_events


class ReaderTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_namespaces_ids_per_file(self) -> None:
        first = write_events(self.root / "a.jsonl", [event(1, at="2026-01-01T00:00:00Z")])
        second = write_events(self.root / "b.jsonl", [event(1, at="2026-01-01T00:00:01Z")])
        sources = read_sources([first, second])
        diagnostics: list[Diagnostic] = []
        ids = [item.id for source in sources for item in iter_events(source, diagnostics)]
        self.assertEqual(ids, ["0:1", "1:1"])
        self.assertEqual(diagnostics, [])

    def test_malformed_and_truncated_lines_are_reported(self) -> None:
        path = self.root / "bad.jsonl"
        path.write_text(
            '{"schema":1,"id":1,"at":"2026-01-01T00:00:00Z","kind":"tool","name":"x"}\n'
            "not json\n"
            '{"schema":1,"id":2,"at":',  # incomplete final line, no newline
            encoding="utf-8",
        )
        source = freeze_source(path, 0)
        diagnostics: list[Diagnostic] = []
        events = list(iter_events(source, diagnostics))
        self.assertEqual(len(events), 1)
        codes = [item.code for item in diagnostics]
        self.assertIn("malformed_json", codes)
        self.assertIn("truncated_final_line", codes)

    def test_unknown_schema_and_kind_are_preserved_with_diagnostics(self) -> None:
        path = write_events(
            self.root / "future.jsonl",
            [
                event(1, at="2026-01-01T00:00:00Z", kind="future-kind", schema=99),
            ],
        )
        source = freeze_source(path, 0)
        diagnostics: list[Diagnostic] = []
        events = list(iter_events(source, diagnostics))
        self.assertEqual(events[0].kind, "future-kind")
        self.assertEqual(events[0].schema, 99)
        codes = {item.code for item in diagnostics}
        self.assertEqual(codes, {"unsupported_schema", "unknown_kind"})

    def test_invalid_timestamp_and_negative_duration_are_diagnosed(self) -> None:
        path = self.root / "invalid.jsonl"
        path.write_text(
            '{"schema":1,"id":1,"at":"not-a-date","kind":"tool","name":"x","duration_ms":-5}\n',
            encoding="utf-8",
        )
        source = freeze_source(path, 0)
        diagnostics: list[Diagnostic] = []
        events = list(iter_events(source, diagnostics))
        self.assertIsNone(events[0].at)
        self.assertIsNone(events[0].duration_ms)
        codes = {item.code for item in diagnostics}
        self.assertEqual(codes, {"invalid_timestamp", "negative_duration"})

    def test_byte_boundary_excludes_later_appends(self) -> None:
        path = write_events(self.root / "growing.jsonl", [event(1, at="2026-01-01T00:00:00Z")])
        source = freeze_source(path, 0)
        with path.open("a", encoding="utf-8") as handle:
            handle.write(
                '{"schema":1,"id":2,"at":"2026-01-01T00:00:01Z","kind":"tool","name":"late"}\n'
            )
        diagnostics: list[Diagnostic] = []
        events = list(iter_events(source, diagnostics))
        self.assertEqual([item.id for item in events], ["0:1"])
        self.assertTrue(source.appended)
        self.assertGreater(source.appended_bytes, 0)

    def test_duplicate_numeric_id_is_namespaced_apart(self) -> None:
        path = write_events(
            self.root / "duplicate.jsonl",
            [
                event(1, at="2026-01-01T00:00:00Z", name="first"),
                event(1, at="2026-01-01T00:00:01Z", name="second"),
            ],
        )
        source = freeze_source(path, 0)
        diagnostics: list[Diagnostic] = []
        events = list(iter_events(source, diagnostics))
        self.assertNotEqual(events[0].id, events[1].id)
        self.assertIn("duplicate_event_id", {item.code for item in diagnostics})


if __name__ == "__main__":
    unittest.main()
