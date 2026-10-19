from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.accounting import analyze_paths
from tests.support import event, write_events


class AccountingTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)

    def tearDown(self) -> None:
        self.temp.cleanup()

    def analyze(self, events: list[dict], **kwargs):
        path = write_events(self.root / "events.jsonl", events)
        return analyze_paths([str(path)], **kwargs)

    def test_overlaps_are_unioned_but_per_kind_sums_are_separate(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:10Z", kind="phase", name="phase", duration_ms=10_000),
                event(2, at="2026-01-01T00:00:08Z", kind="tool", name="tool", duration_ms=8_000),
                event(3, at="2026-01-01T00:00:06Z", kind="turn", name="turn", duration_ms=6_000),
            ]
        )
        time_data = analysis.data["time"]
        self.assertAlmostEqual(time_data["recorded_interval_union_seconds"], 10.0)
        self.assertAlmostEqual(time_data["duration_sum_by_kind_seconds"]["phase"], 10.0)
        self.assertAlmostEqual(time_data["duration_sum_by_kind_seconds"]["tool"], 8.0)
        self.assertAlmostEqual(time_data["duration_sum_by_kind_seconds"]["turn"], 6.0)
        self.assertEqual(time_data["envelope_seconds"], 4.0)
        self.assertAlmostEqual(time_data["unattributed_envelope_seconds"], 0.0)
        self.assertTrue(time_data["overlap_examples"])

    def test_at_is_interval_end(self) -> None:
        analysis = self.analyze(
            [event(1, at="2026-01-01T00:01:00Z", kind="tool", name="x", duration_ms=5_000)]
            + [event(2, at="2026-01-01T00:01:01Z", kind="tool", name="y", duration_ms=1_000)]
        )
        self.assertAlmostEqual(analysis.data["time"]["recorded_interval_union_seconds"], 6.0)
        self.assertAlmostEqual(analysis.data["time"]["envelope_seconds"], 1.0)

    def test_inline_usage_wins_over_linked_usage(self) -> None:
        analysis = self.analyze(
            [
                event(
                    1,
                    at="2026-01-01T00:00:01Z",
                    kind="turn",
                    name="turn 0",
                    session="s",
                    data={
                        "usage": {
                            "input": 100,
                            "output": 10,
                            "cache_read": 5,
                            "reasoning": 3,
                            "total_tokens": 115,
                        },
                        "cost": {"input": 0.1, "output": 0.2, "total": 0.3},
                    },
                ),
                event(
                    2,
                    at="2026-01-01T00:00:01Z",
                    kind="usage",
                    name="usage",
                    session="s",
                    parent=1,
                    data={"input": 100, "output": 10, "cache_read": 5, "reasoning": 3, "total_tokens": 115},
                ),
            ]
        )
        usage = analysis.data["usage"]
        self.assertEqual(usage["input_tokens"], 100)
        self.assertEqual(usage["output_tokens"], 10)
        self.assertEqual(usage["reasoning_tokens"], 3)
        self.assertEqual(usage["duplicate_usage_not_counted"], 1)
        self.assertEqual(analysis.data["cost"]["observed_total_usd"], 0.3)
        codes = {item.code for item in analysis.diagnostics}
        self.assertIn("duplicate_usage", codes)

    def test_linked_usage_is_used_when_turn_has_no_inline_usage(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="turn", name="turn 0", session="s"),
                event(
                    2,
                    at="2026-01-01T00:00:01Z",
                    kind="usage",
                    name="usage",
                    session="s",
                    parent=1,
                    data={"input": 7, "output": 2},
                ),
            ]
        )
        usage = analysis.data["usage"]
        self.assertEqual(usage["input_tokens"], 7)
        self.assertEqual(usage["output_tokens"], 2)
        self.assertEqual(usage["turns_without_usage"], 0)

    def test_orphan_usage_is_counted_once_and_reported(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="usage", name="usage", data={"input": 4, "output": 1}),
            ]
        )
        self.assertEqual(analysis.data["usage"]["input_tokens"], 4)
        self.assertEqual(analysis.data["usage"]["turns_without_usage"], 0)
        self.assertIn("orphan_usage", {item.code for item in analysis.diagnostics})

    def test_missing_usage_is_partial_not_zero(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="turn", name="turn 0"),
            ]
        )
        self.assertEqual(analysis.data["usage"]["turns_without_usage"], 1)
        measurements = {item.id: item for item in analysis.measurements}
        self.assertEqual(measurements["usage.input_tokens"].status, "partial")
        self.assertIn("cost-missing", {item.id for item in analysis.findings})

    def test_cost_components_are_summed_and_numeric_is_used(self) -> None:
        analysis = self.analyze(
            [
                event(
                    1,
                    at="2026-01-01T00:00:01Z",
                    kind="turn",
                    name="turn 0",
                    data={"usage": {"input": 1, "output": 1}, "cost": {"input": 0.1, "output": 0.2}},
                ),
                event(
                    2,
                    at="2026-01-01T00:00:02Z",
                    kind="turn",
                    name="turn 1",
                    data={"usage": {"input": 1, "output": 1}, "cost": "bogus"},
                ),
            ]
        )
        cost = analysis.data["cost"]
        self.assertAlmostEqual(cost["observed_total_usd"], 0.3)
        self.assertEqual(cost["ambiguous_cost_requests"], 1)
        self.assertEqual(cost["status"], "partial")

    def test_cumulative_cost_is_not_summed(self) -> None:
        analysis = self.analyze(
            [
                event(
                    1,
                    at="2026-01-01T00:00:01Z",
                    kind="turn",
                    name="turn 0",
                    data={"usage": {"input": 1}, "cost": {"total": 1.0, "cumulative": True}},
                ),
                event(
                    2,
                    at="2026-01-01T00:00:02Z",
                    kind="turn",
                    name="turn 1",
                    data={"usage": {"input": 1}, "cost": {"total": 2.0, "cumulative": True}},
                ),
            ]
        )
        cost = analysis.data["cost"]
        self.assertIsNone(cost["observed_total_usd"])
        self.assertAlmostEqual(cost["cumulative_rollup_usd"], 2.0)
        self.assertIn("cumulative_cost", {item.code for item in analysis.diagnostics})

    def test_recorder_plumbing_is_excluded_from_command_friction(self) -> None:
        analysis = self.analyze(
            [
                event(
                    1,
                    at="2026-01-01T00:00:01Z",
                    kind="command",
                    name="history",
                    label="history ingest",
                    outcome="failed",
                    duration_ms=10,
                ),
                event(
                    2,
                    at="2026-01-01T00:00:02Z",
                    kind="command",
                    name="check",
                    label="check review",
                    outcome="failed",
                    duration_ms=10,
                ),
                event(
                    3,
                    at="2026-01-01T00:00:03Z",
                    kind="command",
                    name="check",
                    label="check review",
                    outcome="ok",
                    duration_ms=10,
                ),
            ]
        )
        repeated = " ".join(row["normalized"] for row in analysis.data["commands"]["repeated"])
        self.assertNotIn("history", repeated)
        self.assertIn("check", repeated)

    def test_recovery_requires_a_later_success(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="command", name="check", label="check review", outcome="failed"),
                event(2, at="2026-01-01T00:00:02Z", kind="command", name="check", label="check review", outcome="ok"),
                event(3, at="2026-01-01T00:00:03Z", kind="command", name="lint", label="lint", outcome="failed"),
            ]
        )
        failures = analysis.data["failures"]
        self.assertEqual(failures["total"], 2)
        self.assertEqual(failures["recovered"], 1)
        self.assertEqual(failures["unrecovered"], 1)
        self.assertEqual(failures["unrecovered_event_ids"], ["0:3"])

    def test_questions_do_not_gain_an_answer_from_silence(self) -> None:
        analysis = self.analyze(
            [
                event(
                    1,
                    at="2026-01-01T00:00:10Z",
                    kind="question",
                    name="question",
                    label="Which option?",
                    duration_ms=9_000,
                    data={"question": {"text": "Which option?\nA\nB", "truncated": False}},
                )
            ]
        )
        question = analysis.data["questions"]["rows"][0]
        self.assertIsNone(question["answer"])
        self.assertEqual(question["answer_status"], "not_recorded")
        self.assertEqual(analysis.data["questions"]["total_wait_ms"], 9_000)

    def test_phase_markers_are_not_intervals(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="phase", name="proposal.md", outcome="ok"),
            ]
        )
        self.assertAlmostEqual(analysis.data["time"]["recorded_interval_union_seconds"], 0.0)
        self.assertEqual(len(analysis.data["phases"]["markers"]), 1)
        self.assertIn("not proof", analysis.data["phases"]["note"])

    def test_filters_apply_to_analysis(self) -> None:
        analysis = self.analyze(
            [
                event(1, at="2026-01-01T00:00:01Z", kind="tool", name="a", session="one"),
                event(2, at="2026-01-01T00:00:02Z", kind="tool", name="b", session="two"),
            ],
            sessions=["two"],
        )
        self.assertEqual(analysis.data["corpus"]["events"], 2)
        self.assertEqual(analysis.data["corpus"]["analyzed_events"], 1)
        self.assertEqual(analysis.data["corpus"]["filtered_out"], 1)
        self.assertEqual(analysis.data["corpus"]["kinds"], {"tool": 1})


if __name__ == "__main__":
    unittest.main()
