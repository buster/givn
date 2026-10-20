from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.benchmark import BenchmarkError, load_benchmark
from givn_harness.grader import verify_benchmark


class BenchmarkVerificationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.benchmark = load_benchmark(
            Path(__file__).resolve().parents[1] / "benchmarks" / "lending"
        )

    def test_reference_scaffold_and_defects(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            document = verify_benchmark(self.benchmark, Path(temporary) / "verification")
        self.assertTrue(document["passed"], document)
        cases = {case["id"]: case for case in document["cases"]}
        self.assertTrue(cases["reference"]["all_required_passed"])
        self.assertGreaterEqual(cases["scaffold"]["failed"], 1)
        for defect in self.benchmark.defects:
            self.assertTrue(cases[defect["id"]]["passed"], cases[defect["id"]])
            self.assertFalse(cases[defect["id"]]["missing_expected_failures"])

    def test_private_path_in_public_files_is_rejected(self) -> None:
        import tomllib

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = Path(__file__).resolve().parents[1] / "benchmarks" / "lending"
            for path in source.rglob("*"):
                if path.is_file() and "defects" not in path.parts and "reference" not in path.parts and "grader" not in path.parts:
                    destination = root / path.relative_to(source)
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    destination.write_bytes(path.read_bytes())
            manifest = root / "benchmark.toml"
            text = manifest.read_text()
            text = text.replace(
                'public_files = [\n  "public/contract.md",',
                'public_files = [\n  "private/grader/run_grader.py",\n  "public/contract.md",',
            )
            manifest.write_text(text)
            with self.assertRaises(BenchmarkError):
                load_benchmark(manifest)


if __name__ == "__main__":
    unittest.main()
