from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.benchmark import load_benchmark
from givn_harness.driver import DriverSpec
from givn_harness.grader import grade_bundle
from givn_harness.runner import RunConfig, execute
from givn_harness.compare import build_comparison, render_comparison


class CompareTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.benchmark = load_benchmark(
            Path(__file__).resolve().parents[1] / "benchmarks" / "lending"
        )
        self.agent = Path(__file__).resolve().parent / "agents" / "copy_app.py"

    def tearDown(self) -> None:
        self.temp.cleanup()

    def make_bundle(self, condition: str) -> Path:
        config = RunConfig(
            benchmark=self.benchmark,
            output=self.root / condition,
            condition=condition,
            repeats=1,
            driver=DriverSpec(
                kind="script",
                model="deepseek/deepseek-flash#max",
                command=["/usr/bin/python3", str(self.agent)],
                extra_env={
                    "HARNESS_APP_SOURCE": str(self.benchmark.reference / "app.py"),
                },
            ),
            givn_binary=None,
            sandbox_mode="none",
            workspace_root=self.root / "workspaces",
        )
        bundle = execute(config)[0]
        grade_bundle(bundle, self.benchmark, sandbox_mode="none")
        return bundle

    def test_single_trial_comparison_is_descriptive_and_inconclusive(self) -> None:
        baseline = self.make_bundle("baseline")
        candidate = self.make_bundle("candidate")
        document = build_comparison([str(baseline)], [str(candidate)])
        self.assertTrue(document["inconclusive"])
        self.assertTrue(document["comparability"]["comparable"])
        rows = {row["id"]: row for row in document["requirements"]}
        self.assertEqual(rows["I1"]["baseline"]["pass"], 1)
        self.assertEqual(rows["I1"]["candidate"]["pass"], 1)
        markdown = render_comparison(document)
        self.assertIn("INCONCLUSIVE", markdown)
        self.assertIn("does not combine quality", markdown)


if __name__ == "__main__":
    unittest.main()
