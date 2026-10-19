from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

from givn_harness.benchmark import load_benchmark
from givn_harness.bundle import load_manifest, read_journal, verify_snapshot
from givn_harness.driver import DriverSpec
from givn_harness.grader import grade_bundle
from givn_harness.runner import RunConfig, execute


class RunnerTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.benchmark = load_benchmark(
            Path(__file__).resolve().parents[1] / "benchmarks" / "lending"
        )
        self.reference = self.benchmark.reference
        self.agent = Path(__file__).resolve().parent / "agents" / "copy_app.py"

    def tearDown(self) -> None:
        self.temp.cleanup()

    def config(self, *, mode: str = "copy", output: str = "runs") -> RunConfig:
        return RunConfig(
            benchmark=self.benchmark,
            output=self.root / output,
            condition="fake",
            repeats=1,
            driver=DriverSpec(
                kind="script",
                model="deepseek/deepseek-flash#max",
                command=[sys.executable, str(self.agent)],
                extra_env={
                    "HARNESS_APP_SOURCE": str(self.reference / "app.py"),
                    "HARNESS_AGENT_MODE": mode,
                },
            ),
            givn_binary=None,
            sandbox_mode="none",
            network="none",
            workspace_root=self.root / "workspaces",
            max_output_tokens=1024,
        )

    def test_complete_two_stage_run_is_bundled_and_grades_green(self) -> None:
        config = self.config()
        bundles = execute(config)
        self.assertEqual(len(bundles), 1)
        bundle = bundles[0]
        manifest = load_manifest(bundle)
        self.assertEqual(manifest["status"], "completed")
        self.assertEqual([stage["id"] for stage in manifest["stages"]], ["initial", "followup"])
        self.assertTrue(all(stage["completed"] for stage in manifest["stages"]))
        for stage in ("initial", "followup"):
            stage_dir = bundle / "stages" / stage
            self.assertTrue((stage_dir / "snapshot" / "app.py").is_file())
            self.assertEqual(verify_snapshot(stage_dir), [])
            self.assertTrue((stage_dir / "probe.json").is_file())
            self.assertTrue((stage_dir / "stage.json").is_file())
        events = [record["event"] for record in read_journal(bundle)]
        for expected in ("bundle_created", "stage_started", "driver_finished", "snapshot_frozen", "run_finished"):
            self.assertIn(expected, events)
        grade = grade_bundle(bundle, self.benchmark, sandbox_mode="none")
        self.assertTrue(grade["summary"]["all_required_passed"])
        self.assertEqual(grade["summary"]["failed"], 0)
        self.assertTrue((bundle / "grades" / grade["grade_id"] / "grade.md").is_file())
        for stage in ("initial", "followup"):
            self.assertEqual(verify_snapshot(bundle / "stages" / stage), [])

    def test_incomplete_first_stage_keeps_evidence_and_does_not_reach_followup(self) -> None:
        config = self.config(mode="noop")
        bundle = execute(config)[0]
        manifest = load_manifest(bundle)
        self.assertEqual(manifest["status"], "incomplete")
        self.assertFalse(manifest["stages"][0]["completed"])
        self.assertTrue(manifest["stages"][1]["not_reached"])
        self.assertTrue((bundle / "stages" / "initial" / "snapshot").is_dir())
        self.assertTrue((bundle / "stages" / "followup" / "not-reached.json").is_file())
        grade = grade_bundle(bundle, self.benchmark, sandbox_mode="none")
        statuses = {item["id"]: item["status"] for item in grade["requirements"]}
        self.assertEqual(statuses["I1"], "fail")
        self.assertEqual(statuses["F1"], "ungraded")

    def test_driver_failure_is_recorded_not_hidden(self) -> None:
        config = self.config(mode="fail")
        bundle = execute(config)[0]
        manifest = load_manifest(bundle)
        self.assertEqual(manifest["status"], "incomplete")
        stage = manifest["stages"][0]
        self.assertEqual(stage["driver"]["termination"], "driver_error")
        self.assertEqual(stage["driver"]["exit_code"], 7)
        self.assertTrue((bundle / "stages" / "initial" / "agent.stderr.log").is_file())

    def test_snapshot_tamper_is_detected(self) -> None:
        bundle = execute(self.config())[0]
        stage_dir = bundle / "stages" / "initial"
        target = stage_dir / "snapshot" / "app.py"
        target.write_text(target.read_text() + "\n# tampered\n")
        problems = verify_snapshot(stage_dir)
        self.assertTrue(any("changed: app.py" in item for item in problems))

    def test_grading_refuses_a_tampered_snapshot(self) -> None:
        from givn_harness.grader import GraderError

        bundle = execute(self.config())[0]
        target = bundle / "stages" / "initial" / "snapshot" / "app.py"
        target.write_text(target.read_text() + "\n# tampered\n")
        with self.assertRaises(GraderError):
            grade_bundle(bundle, self.benchmark, sandbox_mode="none")

    def test_grading_sandbox_hides_private_benchmark_material(self) -> None:
        from givn_harness.sandbox import bubblewrap_available

        if not bubblewrap_available():
            self.skipTest("bubblewrap is unavailable")
        bundle = execute(self.config())[0]
        grade = grade_bundle(bundle, self.benchmark, sandbox_mode="required")
        self.assertTrue(grade["summary"]["all_required_passed"])

    def test_resume_of_completed_bundle_returns_without_rework(self) -> None:
        bundle = execute(self.config())[0]
        before = load_manifest(bundle)
        resumed = execute(
            RunConfig(
                benchmark=self.benchmark,
                output=self.root / "runs",
                driver=self.config().driver,
                givn_binary=None,
                sandbox_mode="none",
                network="none",
                workspace_root=self.root / "workspaces",
                resume=bundle,
            )
        )[0]
        after = load_manifest(resumed)
        self.assertEqual(before["status"], after["status"])
        self.assertEqual(len(after["stages"]), 2)


if __name__ == "__main__":
    unittest.main()
