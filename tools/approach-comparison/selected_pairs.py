#!/usr/bin/env python3
"""Print the ten manually classified Givn reference pairs from a report."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


PAIRS = [
    ("Uninitialized project errors", "Uninitialised project is rejected", "VALID-BOUNDARY"),
    (
        "Integrity check detects a placeholder COMMIT hash in tasks.md and fails",
        "Integrity hook fails on placeholder commit hash",
        "DUPLICATE",
    ),
    (
        "Detail mode — skipped artifacts shown with skip marker",
        "givn status still reports skipped artifacts with the skip marker after dead code removal",
        "DUPLICATE",
    ),
    (
        "A parse error is reported on stderr and exits 1",
        "The json output reports a parse error as a machine-readable finding",
        "VALID-BOUNDARY",
    ),
    (
        "Coverage-enabled project receives a complete README coverage example",
        "Coverage-enabled project is guided to place the coverage badge directly after the README title",
        "DUPLICATE",
    ),
    ("Id with a dot is rejected", "Path traversal id is rejected by status", "VARIANT"),
    ("Id with a dot is rejected", "Path traversal id is rejected by instructions", "VARIANT"),
    ("Id with a dot is rejected", "Path traversal id is rejected by check", "VARIANT"),
    ("Id with non-ASCII letters is rejected", "Path traversal id is rejected by status", "VARIANT"),
    ("Id with spaces is rejected", "Path traversal id is rejected by instructions", "VARIANT"),
]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "report",
        nargs="?",
        default="test-corpus/indexes/combined-approach-report-strict.json",
    )
    args = parser.parse_args()
    report = json.loads(Path(args.report).read_text(encoding="utf-8"))
    scenarios = report["scenarios"]
    by_title = {}
    for index, scenario in enumerate(scenarios):
        by_title.setdefault(scenario["title"], []).append((index, scenario))

    print("| # | Classification | Embedding | BGE avg | NLI E/N/C | Scenario pair |")
    print("|---:|---|---:|---:|---|---|")
    for number, (left_title, right_title, classification) in enumerate(PAIRS, start=1):
        left = by_title[left_title]
        right = by_title[right_title]
        matches = []
        for left_index, left_scenario in left:
            for right_index, right_scenario in right:
                key = {left_index, right_index}
                for pair in report["pairs"]:
                    if {pair["left"], pair["right"]} == key:
                        matches.append(pair)
        if len(matches) != 1:
            raise SystemExit(
                f"expected exactly one report pair for {left_title!r} / {right_title!r}, "
                f"found {len(matches)}"
            )
        pair = matches[0]
        nli = pair["nli"]["symmetric"]
        print(
            f"| {number} | {classification} | {pair['embedding']:.4f} | "
            f"{pair['bge'][2]:.4f} | {nli[0]:.3f}/{nli[1]:.3f}/{nli[2]:.3f} | "
            f"{left_title} / {right_title} |"
        )


if __name__ == "__main__":
    main()
