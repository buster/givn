#!/usr/bin/env python3
"""Compare the embedding, BGE reranker, and NLI approaches on local corpora.

The tool deliberately lives outside the production retrieval implementation.
It scans the copied active specs under ``test-corpus/specs`` and writes all
generated indexes and reports under ``test-corpus/indexes``. The Watn checkout
is never modified.

Example:

    uv run --with 'fastembed==0.8.0' --with 'onnxruntime' \
      --with 'tokenizers' --with 'numpy' \
      tools/approach-comparison/compare.py \
      --nli-model /tmp/opencode/nli-model \
      --bge-model givn/.spec-reranker-cache/models--rozgo/bge-reranker-v2-m3 \
      run
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable

import numpy as np


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_CORPUS = ROOT / "test-corpus" / "specs"
DEFAULT_INDEX = ROOT / "test-corpus" / "indexes" / "combined-e5"
DEFAULT_EMBED_CACHE = ROOT / "givn" / ".spec-model-cache"
DEFAULT_BGE_CACHE = ROOT / "givn" / ".spec-reranker-cache"
DEFAULT_NLI_MODEL = Path("/tmp/opencode/nli-model")
EMBED_MODEL = "intfloat/multilingual-e5-large"
BGE_MODEL_ID = "rozgo/bge-reranker-v2-m3"
NLI_MODEL_ID = "MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7"
MAX_LENGTH = 512
DEFAULT_TOP_K = 5
DEFAULT_NLI_BATCH = 32
DEFAULT_BGE_BATCH = 32
STRICT_THRESHOLD = 0.95
RERANK_THRESHOLD = 3.0

SCENARIO_RE = re.compile(r"^\s*Scenario(?: Outline)?\s*:\s*(.*)$")
STEP_RE = re.compile(r"^\s*(?:Given|When|Then|And|But|\*)\s+(.*)$")


@dataclass(frozen=True)
class Scenario:
    source: str
    file: str
    title: str
    line: int
    text: str

    @property
    def identity(self) -> tuple[str, str, str]:
        return self.source, self.file, self.title

    @property
    def feature_identity(self) -> tuple[str, str]:
        return self.source, self.file


def parse_feature(path: Path, root: Path, source: str) -> list[Scenario]:
    lines = path.read_text(encoding="utf-8").splitlines()
    relative = path.relative_to(root).as_posix()
    scenarios: list[Scenario] = []
    title: str | None = None
    title_line = 0
    body: list[str] = []
    in_docstring = False

    def finish() -> None:
        nonlocal title, title_line, body
        if title is not None:
            text = " ".join([title, *body])
            normalized = " ".join(text.split())
            scenarios.append(Scenario(source, relative, title, title_line, normalized))
        title = None
        title_line = 0
        body = []

    for number, raw in enumerate(lines, start=1):
        stripped = raw.strip()

        if in_docstring:
            if stripped.startswith('"""') or stripped.startswith("```"):
                in_docstring = False
            elif title is not None:
                body.append(stripped)
            continue

        if stripped.startswith('"""') or stripped.startswith("```"):
            in_docstring = True
            continue

        match = SCENARIO_RE.match(raw)
        if match:
            finish()
            title = match.group(1).strip()
            title_line = number
            continue

        if stripped.startswith(("Feature:", "Rule:", "Background:")):
            finish()
            continue

        if title is None or not stripped or stripped.startswith("#"):
            continue
        if stripped.startswith("@") or stripped.startswith("Examples:") or stripped.startswith("|"):
            continue

        step = STEP_RE.match(raw)
        if step:
            body.append(step.group(1).strip())

    finish()
    return scenarios


def load_corpus(corpus_root: Path) -> list[Scenario]:
    scenarios: list[Scenario] = []
    for source_dir in sorted(path for path in corpus_root.iterdir() if path.is_dir()):
        for feature in sorted(source_dir.glob("**/*.feature")):
            scenarios.extend(parse_feature(feature, source_dir, source_dir.name))
    scenarios.sort(key=lambda item: item.identity)
    return scenarios


def corpus_fingerprint(scenarios: list[Scenario]) -> str:
    payload = json.dumps([asdict(item) for item in scenarios], ensure_ascii=False, sort_keys=True)
    return hashlib.sha256(payload.encode()).hexdigest()


def scenario_texts(scenarios: Iterable[Scenario]) -> list[str]:
    return [scenario.text for scenario in scenarios]


def build_embeddings(scenarios: list[Scenario], cache_dir: Path) -> np.ndarray:
    from fastembed import TextEmbedding

    model = TextEmbedding(EMBED_MODEL, cache_dir=str(cache_dir), threads=1)
    texts = [f"passage: {text}" for text in scenario_texts(scenarios)]
    return np.asarray(list(model.embed(texts, batch_size=32)), dtype=np.float32)


def save_index(index_path: Path, scenarios: list[Scenario], vectors: np.ndarray) -> None:
    index_path.parent.mkdir(parents=True, exist_ok=True)
    np.savez_compressed(index_path.with_suffix(".npz"), vectors=vectors)
    payload = {
        "model": EMBED_MODEL,
        "threshold": STRICT_THRESHOLD,
        "fingerprint": corpus_fingerprint(scenarios),
        "scenarios": [asdict(item) for item in scenarios],
    }
    index_path.with_suffix(".json").write_text(
        json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8"
    )


def load_or_build_index(
    scenarios: list[Scenario], index_path: Path, cache_dir: Path, rebuild: bool
) -> np.ndarray:
    json_path = index_path.with_suffix(".json")
    npz_path = index_path.with_suffix(".npz")
    fingerprint = corpus_fingerprint(scenarios)
    if not rebuild and json_path.exists() and npz_path.exists():
        payload = json.loads(json_path.read_text(encoding="utf-8"))
        if payload.get("fingerprint") == fingerprint and payload.get("model") == EMBED_MODEL:
            with np.load(npz_path) as data:
                vectors = np.asarray(data["vectors"], dtype=np.float32)
            if vectors.shape[0] == len(scenarios):
                print(f"reused embedding index: {vectors.shape[0]} scenarios")
                return vectors
    print(f"embedding {len(scenarios)} scenarios with {EMBED_MODEL}")
    vectors = build_embeddings(scenarios, cache_dir)
    save_index(index_path, scenarios, vectors)
    print(f"wrote embedding index: {index_path}.json/.npz")
    return vectors


def normalized_vectors(vectors: np.ndarray) -> np.ndarray:
    norms = np.linalg.norm(vectors, axis=1, keepdims=True)
    return vectors / np.maximum(norms, 1e-12)


def pair_scope(left: Scenario, right: Scenario) -> str:
    if left.source != right.source:
        return "givn-watn"
    return f"within-{left.source}"


def candidate_pairs(
    scenarios: list[Scenario], vectors: np.ndarray, top_k: int, scope: str
) -> tuple[set[tuple[int, int]], np.ndarray, dict[str, int]]:
    normalized = normalized_vectors(vectors)
    similarity = normalized @ normalized.T
    topk_pairs: set[tuple[int, int]] = set()
    topk_count = 0
    for left in range(len(scenarios)):
        order = np.argsort(-similarity[left])
        taken = 0
        for right in order:
            if right == left:
                continue
            pair = (min(left, int(right)), max(left, int(right)))
            if pair not in topk_pairs:
                topk_pairs.add(pair)
                topk_count += 1
            taken += 1
            if taken >= top_k:
                break

    strict_pairs_array = np.argwhere(np.triu(similarity >= STRICT_THRESHOLD, k=1))
    strict_pairs = {(int(left), int(right)) for left, right in strict_pairs_array}
    if scope == "strict":
        pair_set = strict_pairs
    elif scope == "topk":
        pair_set = topk_pairs
    elif scope == "union":
        pair_set = topk_pairs | strict_pairs
    elif scope == "cross-source":
        pair_set = {
            pair
            for pair in topk_pairs
            if scenarios[pair[0]].source != scenarios[pair[1]].source
        }
    else:
        raise ValueError(f"unknown candidate scope: {scope}")
    return pair_set, similarity, {
        "top_k_union": topk_count,
        "strict_pairs": len(strict_pairs),
        "candidate_pairs": len(pair_set),
    }


def find_model_snapshot(cache_root: Path, repo_fragment: str) -> Path:
    direct = cache_root / "model.onnx"
    if direct.exists():
        return direct
    matches = sorted(cache_root.glob(f"models--{repo_fragment.replace('/', '--')}/snapshots/*/model.onnx"))
    if not matches:
        raise FileNotFoundError(f"no ONNX model found under {cache_root}")
    return matches[-1]


def resolve_model_path(path: Path) -> Path:
    if path.is_dir():
        path = path / "model.onnx"
    if not path.exists():
        raise FileNotFoundError(f"model artifact does not exist: {path}")
    return path


class OnnxPairModel:
    def __init__(self, model_path: Path, max_length: int = MAX_LENGTH) -> None:
        import onnxruntime as ort
        from tokenizers import Tokenizer

        self.model_path = model_path
        self.tokenizer = Tokenizer.from_file(str(model_path.with_name("tokenizer.json")))
        config = json.loads(model_path.with_name("config.json").read_text(encoding="utf-8"))
        tokenizer_config = json.loads(
            model_path.with_name("tokenizer_config.json").read_text(encoding="utf-8")
        )
        self.tokenizer.enable_truncation(max_length=max_length)
        self.tokenizer.enable_padding(
            pad_id=int(config.get("pad_token_id", 0)),
            pad_token=tokenizer_config.get("pad_token", "[PAD]"),
        )
        options = ort.SessionOptions()
        options.intra_op_num_threads = 4
        options.inter_op_num_threads = 1
        self.session = ort.InferenceSession(
            str(model_path), sess_options=options, providers=["CPUExecutionProvider"]
        )
        self.input_names = {item.name for item in self.session.get_inputs()}

    def logits(self, pairs: list[tuple[str, str]], batch_size: int) -> np.ndarray:
        results: list[np.ndarray] = []
        for start in range(0, len(pairs), batch_size):
            batch = pairs[start : start + batch_size]
            encodings = self.tokenizer.encode_batch(batch, add_special_tokens=True)
            ids = np.asarray([encoding.ids for encoding in encodings], dtype=np.int64)
            masks = np.asarray([encoding.attention_mask for encoding in encodings], dtype=np.int64)
            inputs: dict[str, np.ndarray] = {
                "input_ids": ids,
                "attention_mask": masks,
            }
            if "token_type_ids" in self.input_names:
                inputs["token_type_ids"] = np.asarray(
                    [encoding.type_ids for encoding in encodings], dtype=np.int64
                )
            outputs = self.session.run(None, inputs)
            results.append(np.asarray(outputs[0], dtype=np.float32))
        return np.concatenate(results, axis=0)


def score_bge(
    candidates: list[tuple[int, int]], scenarios: list[Scenario], model_path: Path
) -> dict[str, list[float]]:
    print(f"scoring BGE: {len(candidates)} pairs in both directions", flush=True)
    model = OnnxPairModel(model_path)
    pairs: list[tuple[str, str]] = []
    for left, right in candidates:
        pairs.extend(
            [
                (scenarios[left].text, scenarios[right].text),
                (scenarios[right].text, scenarios[left].text),
            ]
        )
    scores = model.logits(pairs, DEFAULT_BGE_BATCH).reshape(-1)
    output: dict[str, list[float]] = {}
    for index, (left, right) in enumerate(candidates):
        forward = float(scores[index * 2])
        reverse = float(scores[index * 2 + 1])
        output[f"{left}:{right}"] = [forward, reverse, (forward + reverse) / 2.0]
    print("finished BGE scoring", flush=True)
    return output


def softmax(values: np.ndarray) -> np.ndarray:
    shifted = values - values.max(axis=1, keepdims=True)
    exponentials = np.exp(shifted)
    return exponentials / exponentials.sum(axis=1, keepdims=True)


def score_nli(
    candidates: list[tuple[int, int]], scenarios: list[Scenario], model_path: Path
) -> dict[str, dict[str, list[float]]]:
    print(f"scoring NLI: {len(candidates)} pairs in both directions", flush=True)
    model = OnnxPairModel(model_path)
    config = json.loads(model_path.with_name("config.json").read_text(encoding="utf-8"))
    labels = {int(key): value.lower() for key, value in config["id2label"].items()}
    indices = [
        next(index for index, label in labels.items() if "entail" in label),
        next(index for index, label in labels.items() if "neutral" in label),
        next(index for index, label in labels.items() if "contrad" in label),
    ]
    pairs: list[tuple[str, str]] = []
    for left, right in candidates:
        pairs.extend(
            [
                (scenarios[left].text, scenarios[right].text),
                (scenarios[right].text, scenarios[left].text),
            ]
        )
    raw = model.logits(pairs, DEFAULT_NLI_BATCH)
    raw = raw[:, indices]
    probabilities = softmax(raw)
    output: dict[str, dict[str, list[float]]] = {}
    for index, (left, right) in enumerate(candidates):
        forward_logits = raw[index * 2]
        reverse_logits = raw[index * 2 + 1]
        symmetric = softmax(((forward_logits + reverse_logits) / 2.0)[None, :])[0]
        output[f"{left}:{right}"] = {
            "forward": probabilities[index * 2].tolist(),
            "reverse": probabilities[index * 2 + 1].tolist(),
            "symmetric": symmetric.tolist(),
        }
    print("finished NLI scoring", flush=True)
    return output


def print_summary(
    scenarios: list[Scenario],
    candidates: list[tuple[int, int]],
    similarity: np.ndarray,
    stats: dict[str, int],
    bge: dict[str, list[float]],
    nli: dict[str, dict[str, list[float]]],
    report_limit: int,
) -> None:
    scope_counts: dict[str, int] = {}
    for left, right in candidates:
        scope = pair_scope(scenarios[left], scenarios[right])
        scope_counts[scope] = scope_counts.get(scope, 0) + 1
    bge_hits = sum(value[2] >= RERANK_THRESHOLD for value in bge.values())
    nli_labels = ["entailment", "neutral", "contradiction"]
    nli_counts = {label: 0 for label in nli_labels}
    for value in nli.values():
        nli_counts[nli_labels[int(np.argmax(value["symmetric"]))]] += 1

    print(f"corpus scenarios: {len(scenarios)}")
    print(f"corpus sources: {', '.join(sorted({item.source for item in scenarios}))}")
    print(f"strict embedding pairs >= {STRICT_THRESHOLD:.2f}: {stats['strict_pairs']}")
    print(f"top-{DEFAULT_TOP_K} union pairs: {stats['top_k_union']}")
    print(f"common candidate pairs: {stats['candidate_pairs']}")
    print(f"candidate scopes: {json.dumps(scope_counts, sort_keys=True)}")
    print(f"BGE pairs >= {RERANK_THRESHOLD:.1f}: {bge_hits}")
    print(f"NLI label distribution: {json.dumps(nli_counts, sort_keys=True)}")
    print("top common candidates by embedding score:")
    ordered = sorted(
        candidates,
        key=lambda pair: similarity[pair[0], pair[1]],
        reverse=True,
    )
    for left, right in ordered[:report_limit]:
        key = f"{left}:{right}"
        nli_value = nli[key]["symmetric"]
        bge_value = bge[key][2]
        print(
            f"  {similarity[left, right]:.4f}  bge={bge_value:.4f}  "
            f"nli={nli_value[0]:.3f}/{nli_value[1]:.3f}/{nli_value[2]:.3f}  "
            f"{scenarios[left].source}/{scenarios[left].title}  <->  "
            f"{scenarios[right].source}/{scenarios[right].title}"
        )


def run(args: argparse.Namespace) -> None:
    corpus_root = Path(args.corpus).resolve()
    index_path = Path(args.index).resolve()
    scenarios = load_corpus(corpus_root)
    if not scenarios:
        raise SystemExit(f"no feature scenarios found under {corpus_root}")
    vectors = load_or_build_index(scenarios, index_path, Path(args.embed_cache), args.rebuild)
    pair_set, similarity, stats = candidate_pairs(
        scenarios, vectors, args.top_k, args.candidate_scope
    )
    candidates = sorted(pair_set)
    bge_model = (
        resolve_model_path(Path(args.bge_model).resolve())
        if args.bge_model
        else find_model_snapshot(Path(args.bge_cache), "rozgo/bge-reranker-v2-m3")
    )
    nli_model = resolve_model_path(Path(args.nli_model).resolve())
    bge_started = time.perf_counter()
    bge = score_bge(candidates, scenarios, bge_model)
    bge_seconds = time.perf_counter() - bge_started
    nli_started = time.perf_counter()
    nli = score_nli(candidates, scenarios, nli_model)
    nli_seconds = time.perf_counter() - nli_started

    report = {
        "corpus": str(corpus_root),
        "embedding_model": EMBED_MODEL,
        "bge_model": BGE_MODEL_ID,
        "nli_model": NLI_MODEL_ID,
        "strict_threshold": STRICT_THRESHOLD,
        "bge_threshold": RERANK_THRESHOLD,
        "top_k": args.top_k,
        "candidate_scope": args.candidate_scope,
        "timing_seconds": {
            "bge": bge_seconds,
            "nli": nli_seconds,
            "total_pair_models": bge_seconds + nli_seconds,
        },
        "stats": stats,
        "scenarios": [asdict(item) for item in scenarios],
        "pairs": [
            {
                "left": left,
                "right": right,
                "scope": pair_scope(scenarios[left], scenarios[right]),
                "embedding": float(similarity[left, right]),
                "bge": bge[f"{left}:{right}"],
                "nli": nli[f"{left}:{right}"],
            }
            for left, right in candidates
        ],
    }
    report_path = index_path.with_name("combined-approach-report.json")
    if args.report_name:
        report_path = Path(args.report_name).resolve()
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding="utf-8")
    print_summary(scenarios, candidates, similarity, stats, bge, nli, args.report_limit)
    print(f"wrote report: {report_path}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["run"])
    parser.add_argument("--corpus", default=str(DEFAULT_CORPUS))
    parser.add_argument("--index", default=str(DEFAULT_INDEX))
    parser.add_argument("--embed-cache", default=str(DEFAULT_EMBED_CACHE))
    parser.add_argument("--bge-cache", default=str(DEFAULT_BGE_CACHE))
    parser.add_argument("--bge-model")
    parser.add_argument("--nli-model", default=str(DEFAULT_NLI_MODEL))
    parser.add_argument("--top-k", type=int, default=DEFAULT_TOP_K)
    parser.add_argument(
        "--candidate-scope",
        choices=["strict", "topk", "union", "cross-source"],
        default="strict",
        help="common pair set to score; strict is tractable and directly compares gate conflicts",
    )
    parser.add_argument("--report-name")
    parser.add_argument("--report-limit", type=int, default=20)
    parser.add_argument("--rebuild", action="store_true")
    return parser.parse_args()


if __name__ == "__main__":
    arguments = parse_args()
    if arguments.command == "run":
        run(arguments)
