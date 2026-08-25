#!/usr/bin/env python3
"""Evaluate bounded BGE admission against labeled scenario pairs."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
from collections import Counter
from pathlib import Path


POSITIVE_LABELS = {"DUPLICATE", "VARIANT"}
NEGATIVE_LABELS = {"VALID-BOUNDARY", "UNRELATED"}
MIN_RECALL = 0.9
MIN_FALSE_POSITIVE_REDUCTION = 0.25
MIN_PRECISION = 0.8
MAX_FALSE_NEGATIVE_DELTA = 0.10
MODEL_ID = "rozgo/bge-reranker-v2-m3"
REQUIRED_ARTIFACTS = {
    "model.onnx",
    "tokenizer.json",
    "config.json",
    "special_tokens_map.json",
    "tokenizer_config.json",
}
APPROVAL_ACCEPTANCE_KEYS = {
    "min_recall",
    "min_false_positive_reduction",
    "min_precision",
    "max_false_negative_delta",
    "holdout_recall_pass",
    "false_positive_reduction_pass",
    "precision_pass",
    "false_negative_delta_pass",
    "unscored_positive_pass",
    "candidate_pipeline_pass",
    "pass",
}


def endpoint_ids(report: dict, label: dict) -> tuple[set[int], set[int]]:
    scenarios = report["scenarios"]
    left_ids = {
        index
        for index, item in enumerate(scenarios)
        if (item["source"], item["file"], item["title"])
        == (label["left_source"], label["left_file"], label["left_title"])
    }
    right_ids = {
        index
        for index, item in enumerate(scenarios)
        if (item["source"], item["file"], item["title"])
        == (label["right_source"], label["right_file"], label["right_title"])
    }
    if len(left_ids) != 1 or len(right_ids) != 1:
        raise ValueError(
            "label endpoint did not resolve uniquely: "
            f"{label['left_source']}/{label['left_file']}|{label['left_title']} :: "
            f"{label['right_source']}/{label['right_file']}|{label['right_title']}"
        )
    return left_ids, right_ids


def artifact_metadata(model_path: Path) -> dict[str, dict[str, int | str]]:
    model_path = model_path / "model.onnx" if model_path.is_dir() else model_path
    if not model_path.exists():
        raise ValueError(f"BGE model artifact does not exist: {model_path}")
    required_paths = {name: model_path.with_name(name) for name in REQUIRED_ARTIFACTS}
    model_bytes = model_path.read_bytes()
    external_path = model_path.with_name("model.onnx.data")
    if b"model.onnx.data" in model_bytes:
        required_paths["model.onnx.data"] = external_path
    elif external_path.exists():
        required_paths["model.onnx.data"] = external_path
    missing = [name for name, path in required_paths.items() if not path.is_file()]
    if missing:
        raise ValueError(f"BGE model artifact set is missing: {', '.join(sorted(missing))}")
    for name in ("config.json", "special_tokens_map.json", "tokenizer_config.json"):
        try:
            json.loads(required_paths[name].read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            raise ValueError(f"invalid BGE artifact {name}: {error}") from error
    if not required_paths["tokenizer.json"].read_bytes():
        raise ValueError("BGE tokenizer.json is empty")
    return {
        name: {
            "bytes": path.stat().st_size,
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        }
        for name, path in sorted(required_paths.items())
    }


def validate_report_artifacts(report: dict) -> dict[str, dict[str, int | str]]:
    artifacts = report.get("bge_artifacts")
    if not isinstance(artifacts, dict):
        raise ValueError("benchmark report has no bge_artifacts object")
    names = set(artifacts)
    if not REQUIRED_ARTIFACTS.issubset(names) or names - REQUIRED_ARTIFACTS - {"model.onnx.data"}:
        raise ValueError("benchmark report has an incomplete BGE artifact set")
    digest_pattern = re.compile(r"^[0-9a-f]{64}$")
    for name, metadata in artifacts.items():
        if not isinstance(metadata, dict) or set(metadata) != {"bytes", "sha256"}:
            raise ValueError(f"invalid BGE artifact metadata for {name}")
        if not isinstance(metadata["bytes"], int) or metadata["bytes"] <= 0:
            raise ValueError(f"invalid BGE artifact size for {name}")
        if not isinstance(metadata["sha256"], str) or not digest_pattern.fullmatch(metadata["sha256"]):
            raise ValueError(f"invalid BGE artifact hash for {name}")
    return artifacts


def validate_rust_parity(parity_path: Path, binary: Path | None) -> dict:
    try:
        parity = json.loads(parity_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"invalid Rust production attestation: {error}") from error
    if binary is not None:
        try:
            output = subprocess.run(
                [str(binary), "spec", "bge-parity"],
                check=True,
                capture_output=True,
                text=True,
            )
            generated = json.loads(output.stdout)
        except (OSError, subprocess.CalledProcessError, json.JSONDecodeError) as error:
            raise ValueError(f"could not verify Rust production attestation: {error}") from error
        if generated != parity:
            raise ValueError(
                "Rust parity file does not match the compiled binary output from "
                "`givn spec bge-parity`"
            )
    required = {
        "schema_version",
        "source",
        "candidate_pipeline",
        "pipeline_fingerprint",
        "descriptor",
        "top_k",
        "threshold",
        "max_unique_pairs",
    }
    if set(parity) != required:
        raise ValueError("Rust production attestation has an unexpected schema")
    if (
        parity["schema_version"] != 1
        or parity["source"] != "givn spec bge-parity"
        or parity["candidate_pipeline"] != "rust-production-v2"
        or not isinstance(parity["pipeline_fingerprint"], str)
        or not re.fullmatch(r"[0-9a-f]{16}", parity["pipeline_fingerprint"])
        or parity["top_k"] != 5
        or parity["threshold"] != 3.0
        or not isinstance(parity["max_unique_pairs"], int)
        or parity["max_unique_pairs"] <= 0
    ):
        raise ValueError("invalid Rust production candidate-pipeline attestation")
    return parity


def pair_in_set(report: dict, label: dict, stat_name: str) -> bool:
    left_ids, right_ids = endpoint_ids(report, label)
    for left, right in report.get("stats", {}).get(stat_name, []):
        if (left in left_ids and right in right_ids) or (
            right in left_ids and left in right_ids
        ):
            return True
    return False


def pair_matches(report: dict, label: dict) -> dict | None:
    left_ids, right_ids = endpoint_ids(report, label)
    matches = [
        pair
        for pair in report["pairs"]
        if (pair["left"] in left_ids and pair["right"] in right_ids)
        or (pair["right"] in left_ids and pair["left"] in right_ids)
    ]
    if len(matches) > 1:
        raise ValueError(
            f"label pair matched multiple selected report pairs: {label['left_title']} :: "
            f"{label['right_title']}"
        )
    return matches[0] if matches else None


def bge_score(pair: dict | None) -> float | None:
    if pair is None:
        return None
    evidence = pair.get("bge_evidence")
    if evidence is not None:
        return evidence["symmetric_raw"]
    legacy = pair.get("bge")
    return legacy[2] if isinstance(legacy, list) else None


def evaluate(report: dict, labels: dict) -> dict:
    counts = Counter()
    per_pair = []
    for label in labels["pairs"]:
        e5_raw = pair_in_set(report, label, "strict_pair_ids")
        in_top_k = pair_in_set(report, label, "top_k_pairs")
        in_bounded = pair_in_set(report, label, "bounded_pair_ids")
        deterministic = label.get("deterministic", False) or pair_in_set(
            report, label, "deterministic_pair_ids"
        )
        pair = pair_matches(report, label)
        score = bge_score(pair)
        scored = e5_raw and in_bounded and score is not None
        recommended = bool(scored and score is not None and score >= 3.0)
        label_name = label["label"]
        split = label.get("split", "reference")
        counts[(split, "labeled")] += 1
        counts[(split, "deterministic")] += int(deterministic)
        if deterministic:
            per_pair.append(
                {
                    "left_title": label["left_title"],
                    "right_title": label["right_title"],
                    "label": label_name,
                    "split": split,
                    "disposition": "deterministic",
                    "in_e5_raw": False,
                    "in_top_k": False,
                    "bge_scored": False,
                    "bge_score": None,
                    "recommended": False,
                }
            )
            continue
        counts[(split, "eligible_labeled")] += 1
        counts[(split, "e5_raw")] += int(e5_raw)
        counts[(split, "top_k")] += int(in_top_k)
        counts[(split, "scored")] += int(scored)
        counts[(split, "unscored")] += int(e5_raw and not scored)
        counts[(split, "baseline_recommended")] += int(in_top_k)
        counts[(split, "e5_baseline_recommended")] += int(e5_raw)
        counts[(split, "recommended")] += int(recommended)
        if label_name in POSITIVE_LABELS:
            counts[(split, "positive")] += 1
            counts[(split, "e5_positive")] += int(e5_raw)
            counts[(split, "top_k_positive")] += int(in_top_k)
            counts[(split, "scored_positive")] += int(scored)
            counts[(split, "unscored_positive")] += int(e5_raw and not scored)
            counts[(split, "true_positive")] += int(recommended)
            counts[(split, "false_negative_scored")] += int(scored and not recommended)
        elif label_name in NEGATIVE_LABELS:
            counts[(split, "negative")] += 1
            counts[(split, "e5_negative")] += int(e5_raw)
            counts[(split, "top_k_negative")] += int(in_top_k)
            counts[(split, "scored_negative")] += int(scored)
            counts[(split, "false_positive")] += int(recommended)
            counts[(split, "true_negative")] += int(scored and not recommended)
        else:
            raise ValueError(f"unknown label {label_name!r}")
        per_pair.append(
            {
                "left_title": label["left_title"],
                "right_title": label["right_title"],
                "label": label_name,
                "split": split,
                "disposition": "semantic",
                "in_e5_raw": e5_raw,
                "in_top_k": in_top_k,
                "in_bounded": in_bounded,
                "bge_scored": scored,
                "bge_score": score,
                "embedding_score": pair.get("embedding") if pair else None,
                "bge_forward_raw": (
                    pair.get("bge_evidence", {}).get("forward_raw")
                    if pair and pair.get("bge_evidence")
                    else None
                ),
                "bge_reverse_raw": (
                    pair.get("bge_evidence", {}).get("reverse_raw")
                    if pair and pair.get("bge_evidence")
                    else None
                ),
                "recommended": recommended,
            }
        )

    metrics = {}
    for split in sorted({label.get("split", "reference") for label in labels["pairs"]}):
        positive = counts[(split, "positive")]
        negative = counts[(split, "negative")]
        e5_positive = counts[(split, "e5_positive")]
        e5_negative = counts[(split, "e5_negative")]
        scored_positive = counts[(split, "scored_positive")]
        scored_negative = counts[(split, "scored_negative")]
        true_positive = counts[(split, "true_positive")]
        false_positive = counts[(split, "false_positive")]
        e5_false_negative = positive - e5_positive
        total_false_negative = positive - true_positive
        unscored_negative = e5_negative - scored_negative
        filtered = counts[(split, "scored")] - counts[(split, "recommended")]
        metrics[split] = {
            "labeled": counts[(split, "labeled")],
            "eligible_labeled": counts[(split, "eligible_labeled")],
            "deterministic": counts[(split, "deterministic")],
            "e5_raw": counts[(split, "e5_raw")],
            "top_k": counts[(split, "top_k")],
            "bge_scored": counts[(split, "scored")],
            "bge_unscored": counts[(split, "unscored")],
            "bge_recommended": counts[(split, "recommended")],
            "bge_filtered": filtered,
            "bge_unscored_positive": counts[(split, "unscored_positive")],
            "baseline_recommended": counts[(split, "baseline_recommended")],
            "e5_baseline_recommended": counts[(split, "e5_baseline_recommended")],
            "top_k_recall": counts[(split, "top_k_positive")] / positive
            if positive
            else None,
            "e5_recall": e5_positive / positive if positive else None,
            "admission_recall": true_positive / positive if positive else None,
            "admission_recall_on_top_k": true_positive / counts[(split, "top_k_positive")]
            if counts[(split, "top_k_positive")]
            else None,
            "admission_recall_on_e5": true_positive / e5_positive
            if e5_positive
            else None,
            "admission_recall_on_scored": true_positive / scored_positive
            if scored_positive
            else None,
            "positive": positive,
            "negative": negative,
            "true_positive": true_positive,
            "false_positive": false_positive,
            "false_negative": total_false_negative,
            "false_negative_scored": counts[(split, "false_negative_scored")],
            "true_negative": counts[(split, "true_negative")],
            "baseline_false_positive": e5_negative,
            "baseline_false_negative": e5_false_negative,
            "precision": true_positive / (true_positive + false_positive)
            if true_positive + false_positive
            else None,
            "recall": true_positive / e5_positive if e5_positive else None,
            "false_positive_rate": false_positive / scored_negative
            if scored_negative
            else None,
            "false_positive_rate_overall": false_positive / negative if negative else None,
            "false_negative_rate": total_false_negative / positive if positive else None,
            "false_negative_rate_on_scored": counts[(split, "false_negative_scored")]
            / scored_positive
            if scored_positive
            else None,
            "false_positive_reduction": (
                e5_negative - (false_positive + unscored_negative)
            )
            / e5_negative
            if e5_negative
            else None,
            "false_positive_reduction_on_scored": (
                scored_negative - false_positive
            )
            / scored_negative
            if scored_negative
            else None,
            "baseline_false_negative_rate": e5_false_negative / positive
            if positive
            else None,
            "false_negative_delta": (total_false_negative - e5_false_negative) / positive
            if positive
            else None,
        }
    aggregate = Counter()
    for (split, name), value in counts.items():
        aggregate[name] += value
    aggregate_positive = aggregate["positive"]
    aggregate_negative = aggregate["negative"]
    aggregate_e5_positive = aggregate["e5_positive"]
    aggregate_e5_negative = aggregate["e5_negative"]
    aggregate_true_positive = aggregate["true_positive"]
    aggregate_false_positive = aggregate["false_positive"]
    aggregate_e5_false_negative = aggregate_positive - aggregate_e5_positive
    aggregate_total_false_negative = aggregate_positive - aggregate_true_positive
    aggregate_unscored_negative = aggregate_e5_negative - aggregate["scored_negative"]
    metrics["aggregate"] = {
        "labeled": aggregate["labeled"],
        "eligible_labeled": aggregate["eligible_labeled"],
        "deterministic": aggregate["deterministic"],
        "e5_raw": aggregate["e5_raw"],
        "top_k": aggregate["top_k"],
        "bge_scored": aggregate["scored"],
        "bge_unscored": aggregate["unscored"],
        "bge_recommended": aggregate["recommended"],
        "bge_filtered": aggregate["scored"] - aggregate["recommended"],
        "bge_unscored_positive": aggregate["unscored_positive"],
        "positive": aggregate_positive,
        "negative": aggregate_negative,
        "true_positive": aggregate_true_positive,
        "false_positive": aggregate_false_positive,
        "false_negative": aggregate_total_false_negative,
        "baseline_false_positive": aggregate_e5_negative,
        "baseline_false_negative": aggregate_e5_false_negative,
        "precision": aggregate_true_positive / (aggregate_true_positive + aggregate_false_positive)
        if aggregate_true_positive + aggregate_false_positive
        else None,
        "recall": aggregate_true_positive / aggregate_e5_positive
        if aggregate_e5_positive
        else None,
        "top_k_recall": aggregate["top_k_positive"] / aggregate_positive
        if aggregate_positive
        else None,
        "e5_recall": aggregate_e5_positive / aggregate_positive
        if aggregate_positive
        else None,
        "admission_recall": aggregate_true_positive / aggregate_positive
        if aggregate_positive
        else None,
        "admission_recall_on_top_k": aggregate_true_positive / aggregate["top_k_positive"]
        if aggregate["top_k_positive"]
        else None,
        "false_positive_rate": aggregate_false_positive / aggregate["scored_negative"]
        if aggregate["scored_negative"]
        else None,
        "false_positive_rate_overall": aggregate_false_positive / aggregate_negative
        if aggregate_negative
        else None,
        "false_positive_reduction": (
            aggregate_e5_negative
            - (aggregate_false_positive + aggregate_unscored_negative)
        )
        / aggregate_e5_negative
        if aggregate_e5_negative
        else None,
        "false_negative_rate": aggregate_total_false_negative / aggregate_positive
        if aggregate_positive
        else None,
        "false_negative_delta": (
            aggregate_total_false_negative - aggregate_e5_false_negative
        )
        / aggregate_positive
        if aggregate_positive
        else None,
    }
    return {
        "schema_version": 1,
        "bge_threshold": 3.0,
        "bge_runtime_seconds": report.get("timing_seconds", {}).get("bge"),
        "metrics": metrics,
        "scenario_metrics": [
            {
                "source": item["source"],
                "file": item["file"],
                "title": item["title"],
                "character_count": item["character_count"],
                "step_count": item["step_count"],
                "assertion_count": item["assertion_count"],
            }
            for item in report["scenarios"]
        ],
        "pairs": per_pair,
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--report", required=True)
    parser.add_argument("--labels", required=True)
    parser.add_argument("--enforce", action="store_true")
    parser.add_argument("--approval-file")
    parser.add_argument(
        "--rust-parity",
        help="JSON emitted by `givn spec bge-parity` from the production binary",
    )
    parser.add_argument("--givn-binary")
    parser.add_argument("--bge-model")
    args = parser.parse_args()
    if args.approval_file and not args.enforce:
        raise SystemExit("--approval-file requires --enforce")
    report = json.loads(Path(args.report).read_text(encoding="utf-8"))
    labels = json.loads(Path(args.labels).read_text(encoding="utf-8"))
    parity = None
    if args.rust_parity:
        try:
            parity = validate_rust_parity(
                Path(args.rust_parity), Path(args.givn_binary) if args.givn_binary else None
            )
        except ValueError as error:
            raise SystemExit(str(error)) from error
    if args.approval_file and args.enforce and parity is None:
        raise SystemExit("Rust parity is required for an enforced approval artifact")
    if args.enforce and (parity is None or args.givn_binary is None or args.bge_model is None):
        raise SystemExit("--enforce requires --rust-parity, --givn-binary, and --bge-model")
    if report.get("candidate_scope") != "bounded":
        raise SystemExit(
            "BGE admission evaluation requires candidate_scope=bounded "
            "(top-K union intersected with eligible E5 pairs)"
        )
    if report.get("top_k") != 5:
        raise SystemExit("BGE admission evaluation requires top_k=5")
    stats = report.get("stats")
    if not isinstance(stats, dict):
        raise SystemExit("BGE admission report has no stats object")
    for field in ("top_k_pairs", "strict_pair_ids", "bounded_pair_ids", "deterministic_pair_ids"):
        if not isinstance(stats.get(field), list):
            raise SystemExit(f"BGE admission report is missing stats.{field}")
    if report.get("candidate_pipeline") != "python-reference-v1":
        raise SystemExit("BGE admission evaluation requires a python-reference-v1 report")
    if args.enforce:
        assert parity is not None
        try:
            actual_artifacts = artifact_metadata(Path(args.bge_model))
            reported_artifacts = validate_report_artifacts(report)
        except ValueError as error:
            raise SystemExit(str(error)) from error
        if actual_artifacts != reported_artifacts:
            raise SystemExit("benchmark artifact hashes do not match the local BGE artifact set")
        runtime = report.get("bge_runtime_config")
        if not isinstance(runtime, dict) or runtime != {
            "max_length_tokens": 512,
            "intra_op_threads": 4,
            "inter_op_threads": 1,
            "batch_size": 32,
            "max_unique_pairs": parity["max_unique_pairs"],
        }:
            raise SystemExit("benchmark BGE runtime configuration does not match production")
        if report.get("bge_model") != MODEL_ID or report.get("bge_threshold") != 3.0:
            raise SystemExit("benchmark BGE model or threshold does not match production")
    result = evaluate(report, labels)
    result["candidate_pipeline"] = parity.get("candidate_pipeline") if parity else None
    result["rust_parity"] = parity
    holdout = result["metrics"].get("holdout", {})
    result["acceptance"] = {
        "min_recall": MIN_RECALL,
        "min_false_positive_reduction": MIN_FALSE_POSITIVE_REDUCTION,
        "min_precision": MIN_PRECISION,
        "max_false_negative_delta": MAX_FALSE_NEGATIVE_DELTA,
        "holdout_recall_pass": (holdout.get("admission_recall_on_top_k") or 0) >= MIN_RECALL,
        "false_positive_reduction_pass": (holdout.get("false_positive_reduction") or 0)
        >= MIN_FALSE_POSITIVE_REDUCTION,
        "precision_pass": (holdout.get("precision") or 0) >= MIN_PRECISION,
        "false_negative_delta_pass": (
            holdout.get("false_negative_delta") is not None
            and holdout["false_negative_delta"] <= MAX_FALSE_NEGATIVE_DELTA
        ),
        "unscored_positive_pass": holdout.get("bge_unscored_positive", 0) == 0,
        "candidate_pipeline_pass": parity is not None,
    }
    result["acceptance"]["pass"] = all(
        value
        for key, value in result["acceptance"].items()
        if key.endswith("_pass")
    )
    if set(result["acceptance"]) != APPROVAL_ACCEPTANCE_KEYS:
        raise SystemExit("internal acceptance schema does not match approval contract")
    print(json.dumps(result, ensure_ascii=False, indent=2))
    if args.approval_file and result["acceptance"]["pass"]:
        if parity is None:
            raise SystemExit("passing approval requires Rust production attestation")
        approval = {
            "schema_version": 1,
            "model": MODEL_ID,
            "threshold": 3.0,
            "top_k": 5,
            "max_unique_pairs": parity["max_unique_pairs"],
            "candidate_pipeline": parity["candidate_pipeline"],
            "pipeline_fingerprint": parity["pipeline_fingerprint"],
            "rust_attestation": parity,
            "acceptance": result["acceptance"],
            "artifact_hashes": validate_report_artifacts(report),
        }
        approval_path = Path(args.approval_file)
        approval_path.parent.mkdir(parents=True, exist_ok=True)
        approval_path.write_text(json.dumps(approval, indent=2) + "\n", encoding="utf-8")
    if args.enforce and not result["acceptance"]["pass"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
