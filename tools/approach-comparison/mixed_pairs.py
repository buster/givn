#!/usr/bin/env python3
"""Print the mixed Givn/Watn reference classifications from a report."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


PAIRS = [
    (
        "givn addons disable errors when givn/ is not initialised",
        "givn addons enable errors when givn/ is not initialised",
        "VARIANTE",
    ),
    (
        "Integrity hook fails on unedited arc42 chapter",
        "Integrity hook passes on edited arc42 chapter",
        "VALID-BOUNDARY",
    ),
    (
        "Active and archived changes — single active auto-defaults to detail (text)",
        "Active and archived changes — single active auto-defaults to detail JSON",
        "VALID-BOUNDARY",
    ),
    (
        "Path traversal id is rejected by check",
        "Path traversal id is rejected by status",
        "VARIANTE",
    ),
    (
        "A modified scenario keeps its step docstring after archiving",
        "An added scenario keeps its step docstring after archiving",
        "VALID-BOUNDARY",
    ),
    (
        "Execute flag prompts for confirmation",
        "Execute flag with explicit \"y\" confirmation",
        "VARIANTE",
    ),
    (
        "The green border follows model focus",
        "The green border follows optional shortcut focus",
        "VALID-BOUNDARY",
    ),
    (
        "Bash completion exposes the authoritative command tree",
        "Zsh completion exposes the authoritative command tree",
        "VALID-BOUNDARY",
    ),
    (
        "Rerunning provider setup preserves unrelated configuration",
        "Provider setup preserves unrelated settings",
        "DUPLIKAT",
    ),
    (
        "Model picker shows metadata when available",
        "Model picker shows model IDs when no metadata available",
        "VALID-BOUNDARY",
    ),
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
    by_title: dict[str, list[int]] = {}
    for index, scenario in enumerate(report["scenarios"]):
        by_title.setdefault(scenario["title"], []).append(index)

    print("| # | Classification | E5 | BGE | NLI E/N/C | Pair |")
    print("|---:|---|---:|---:|---|---|")
    for number, (left_title, right_title, classification) in enumerate(PAIRS, start=1):
        matches = []
        for left in by_title[left_title]:
            for right in by_title[right_title]:
                for pair in report["pairs"]:
                    if {pair["left"], pair["right"]} == {left, right}:
                        matches.append(pair)
        if len(matches) != 1:
            raise SystemExit(
                f"expected one match for {left_title!r} / {right_title!r}; "
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
