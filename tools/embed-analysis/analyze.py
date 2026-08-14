#!/usr/bin/env python3
"""givn scenario embedding tool: index the corpus, then query it.

Subcommands
-----------
index    --watn <watn>            parse + embed all active scenarios, write index
query    [--watn <watn>]          load the index, embed new scenario text, show top-k
                                  matches (interactive REPL when no --text given)
evaluate [--watn <watn>]          run the labeled-pairs + hold-one-out retrieval
                                  experiment from docs/feature-step-overlap-report.md

Everything runs on CPU via fastembed (ONNX). Model default BAAI/bge-base-en-v1.5
(768-dim, 0.21 GB). The model cache lives in this directory (.embed-cache, gitignored)
so the watn checkout is never modified.

Examples
--------
    uv run python analyze.py index --watn $HOME/projects/watn
    uv run python analyze.py query --watn $HOME/projects/watn --topk 10
    echo 'Scenario: the newest search result stays visible
    When an older result arrives later
    Then the newest result remains visible' | \
        uv run python analyze.py query --watn $HOME/projects/watn

The query input is the same representation that was indexed: a scenario title plus
its Given/When/Then steps. An embedded new scenario that ranks a known duplicate
scenario at the top is exactly the retrieval the authoring flow needs.
"""

import argparse
import itertools
import json
import math
import os
import re
import sys

import numpy as np
from fastembed import TextEmbedding

DEFAULT_MODEL = "BAAI/bge-base-en-v1.5"
DEFAULT_INDEX = "index"
CACHE_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".embed-cache")

STEP_RE = re.compile(r"^\s*(Given|When|Then|And|But|\*)\s+(.*)$")
SCENARIO_RE = re.compile(r"^\s*(Scenario|Scenario Outline)\s*:\s*(.*)$")


# --------------------------------------------------------------------------
# parsing
# --------------------------------------------------------------------------
def parse_lines(rel, lines):
    """Parse Gherkin lines into scenario dicts (same shape as index rows)."""
    scenarios = []
    current = None
    in_background = False
    for i, raw in enumerate(lines, start=1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        m = SCENARIO_RE.match(raw)
        if m:
            current = {
                "file": rel,
                "title": m.group(2).strip(),
                "line": i,
                "body": [],
                "tags": [],
            }
            scenarios.append(current)
            in_background = False
            continue
        if line.startswith("Feature:") or line.startswith("Background:"):
            in_background = line.startswith("Background:")
            current = None
            continue
        if line.startswith("@"):
            if current is not None:
                current["tags"].extend(line[1:].split())
            continue
        if current is None:
            continue
        if in_background:
            continue
        sm = STEP_RE.match(raw)
        if sm:
            current["body"].append(sm.group(1) + " " + sm.group(2))
        else:
            current["body"].append(line)
    return scenarios


def parse_features(root):
    """Parse every *.feature under root (relative paths stay root-relative)."""
    scenarios = []
    for dirpath, _dirnames, filenames in os.walk(root):
        for fn in sorted(filenames):
            if not fn.endswith(".feature"):
                continue
            path = os.path.join(dirpath, fn)
            rel = os.path.relpath(path, root)
            with open(path, encoding="utf-8") as fh:
                scenarios += parse_lines(rel, fh.read().splitlines())
    return scenarios


def scenario_text(sc):
    """Embedding input for a scenario: title + body steps. Used identically for
    indexing and for querying so both sides live in the same vector space."""
    return sc["title"] + "\n" + "\n".join(sc["body"])


def text_to_scenarios(text, rel="<query>"):
    """Convert pasted scenario text into scenario dicts; if no Scenario header
    is present, treat the whole text as one scenario body."""
    lines = text.splitlines()
    scs = parse_lines(rel, lines)
    if scs:
        return scs
    body = [l.strip() for l in lines if l.strip() and not l.strip().startswith("#")]
    return [{"file": rel, "title": "(query)", "line": 1, "body": body, "tags": []}]


# --------------------------------------------------------------------------
# step definitions (tests/steps/*.rs)
# --------------------------------------------------------------------------
ATTR_RE = re.compile(r"#\[(given|when|then)\b", re.IGNORECASE)
FN_RE = re.compile(r"\bfn\b")


def extract_string_literal(text):
    """First Rust string literal in text, handling raw r##"..."## and "..."."""
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        if c == "r":
            m = re.match(r'r(#+)"', text[i:])
            if m:
                hashes = m.group(1)
                inner = text[i + m.end():]
                end = inner.find('"' + hashes)
                if end != -1:
                    return inner[:end]
                return None
            i += 1
            continue
        if c == '"':
            j = i + 1
            buf = []
            while j < n:
                if text[j] == "\\" and j + 1 < n:
                    buf.append(text[j:j + 2])
                    j += 2
                    continue
                if text[j] == '"':
                    return "".join(buf)
                buf.append(text[j])
                j += 1
            return None
        i += 1
    return None


def capture_fn_body(lines, fn_index):
    """From the line containing 'fn', capture the balanced-brace body text."""
    buf = []
    brace_open = False
    depth = 0
    for k in range(fn_index, len(lines)):
        line = lines[k]
        buf.append(line)
        if not brace_open:
            if "{" in line:
                brace_open = True
                depth = line.count("{") - line.count("}")
        else:
            depth += line.count("{") - line.count("}")
        if brace_open and depth <= 0:
            return "\n".join(buf)
    return "\n".join(buf)


def parse_step_defs(steps_root):
    """Parse cucumber step bindings from *.rs files under steps_root.

    Returns list of {kind, expr, body}. body is the full function text of the
    bound step, which is the implementation mechanics the report (S1-S5) says
    are being duplicated."""
    defs = []
    for dirpath, _dirnames, filenames in os.walk(steps_root):
        for fn in sorted(filenames):
            if not fn.endswith(".rs"):
                continue
            path = os.path.join(dirpath, fn)
            with open(path, encoding="utf-8") as fh:
                lines = fh.read().splitlines()
            i = 0
            while i < len(lines):
                if ATTR_RE.search(lines[i]):
                    j = i
                    attr = []
                    depth = 0
                    while j < len(lines):
                        attr.append(lines[j])
                        depth += lines[j].count("[") - lines[j].count("]")
                        j += 1
                        if depth <= 0:
                            break
                    attr_text = "\n".join(attr)
                    m = ATTR_RE.search(attr_text)
                    kind = m.group(1).lower()
                    expr = extract_string_literal(attr_text)
                    k = j
                    while k < len(lines) and not FN_RE.search(lines[k]):
                        k += 1
                    if expr and k < len(lines):
                        body = capture_fn_body(lines, k)
                        defs.append({"kind": kind, "expr": expr, "body": body})
                        i = k + len(body.splitlines())
                        continue
                i += 1
    return defs


def normalize_def_expr(expr):
    """Normalize a step-def expr like its Gherkin text for token matching."""
    return normalize(expr)


def match_step_defs(scenario, defs):
    """Append the bodies of step defs matched by this scenario's steps."""
    hit_ids = set()
    parts = []
    for step in scenario["body"]:
        if not STEP_RE.match("Given " + step) and not STEP_RE.match(step):
            continue
        step_norm = normalize(step)
        if not step_norm:
            continue
        best = None
        for d in defs:
            de = normalize_def_expr(d["expr"])
            if not de:
                continue
            ts = set(step_norm.split())
            td = set(de.split())
            if not ts or not td:
                continue
            j = len(ts & td) / len(ts | td)
            if best is None or j > best[0]:
                best = (j, d)
        if best and best[0] >= 0.4 and best[1]["body"] not in hit_ids:
            hit_ids.add(best[1]["body"])
            parts.append(best[1]["body"])
    return "\n\n".join(parts)


def scenario_text_with_defs(sc, defs):
    """Scenario title + steps + the implementation of its bound step defs."""
    return scenario_text(sc) + "\n\n" + match_step_defs(sc, defs)


# --------------------------------------------------------------------------
# baseline metrics (deterministic comparison)
# --------------------------------------------------------------------------
def normalize(text):
    t = text.lower()
    t = re.sub(r'"[^"]*"', "QUOTED", t)
    t = re.sub(r"`[^`]*`", "QUOTED", t)
    t = re.sub(r"sk-[a-zA-Z0-9_-]+", "KEY", t)
    t = re.sub(r"\d+(\.\d+)?", "NUM", t)
    t = re.sub(r"\b(and|the|a|an|of|to|for|with|when|then|given|scenario)\b", " ", t)
    t = re.sub(r"[^a-z0-9 ]+", " ", t)
    return t


def jaccard(a, b):
    ta, tb = set(normalize(a).split()), set(normalize(b).split())
    if not ta or not tb:
        return 0.0
    return len(ta & tb) / len(ta | tb)


def cosine(a, b):
    dot = float(np.dot(a, b))
    na = float(np.linalg.norm(a))
    nb = float(np.linalg.norm(b))
    return dot / (na * nb + 1e-12)


# --------------------------------------------------------------------------
# model / index IO
# --------------------------------------------------------------------------
def load_embedder(model):
    return TextEmbedding(model, cache_dir=CACHE_DIR)


def write_index(out, scenarios, vecs, with_defs=False):
    meta = [{"file": s["file"], "title": s["title"], "line": s["line"], "body": s["body"]}
            for s in scenarios]
    np.savez_compressed(out + ".npz", vecs=np.asarray(vecs, dtype=np.float32))
    payload = {"with_step_defs": bool(with_defs), "scenarios": meta}
    with open(out + ".json", "w", encoding="utf-8") as fh:
        json.dump(payload, fh, ensure_ascii=False, indent=1)
    return out + ".npz", out + ".json"


def load_index(path):
    with np.load(path + ".npz") as data:
        vecs = data["vecs"]
    with open(path + ".json", encoding="utf-8") as fh:
        payload = json.load(fh)
    if isinstance(payload, list):
        return payload, vecs, False  # legacy format
    return payload["scenarios"], vecs, payload.get("with_step_defs", False)


def build_index(watn, model, with_defs=False):
    givn_specs = os.path.join(watn, "givn", "specs")
    if not os.path.isdir(givn_specs):
        sys.exit(f"no givn/specs under {watn}")
    scenarios = parse_features(givn_specs)
    defs = []
    if with_defs:
        steps_root = os.path.join(watn, "tests", "steps")
        if os.path.isdir(steps_root):
            defs = parse_step_defs(steps_root)
            matched = sum(1 for s in scenarios if match_step_defs(s, defs))
            print(f"parsed {len(defs)} step definitions under {steps_root}; "
                  f"{matched}/{len(scenarios)} scenarios matched >=1 def")
        else:
            print(f"warning: no {steps_root}; indexing scenarios only")
    texts = [scenario_text_with_defs(s, defs) if with_defs else scenario_text(s)
             for s in scenarios]
    print(f"parsed {len(scenarios)} active scenarios under {givn_specs}"
          + (" (with step-definition bodies)" if with_defs else ""))
    embedder = load_embedder(model)
    vecs = np.asarray(list(embedder.embed(texts)), dtype=np.float32)
    return scenarios, vecs, embedder, defs


def topk(query_vec, vecs, meta, k):
    sims = np.dot(vecs, query_vec) / (
        np.linalg.norm(vecs, axis=1) * np.linalg.norm(query_vec) + 1e-12)
    order = np.argsort(-sims)
    out = []
    for idx in order:
        if sims[idx] < 0.5:
            break
        out.append((float(sims[idx]), meta[idx]))
    return out[:k]


# --------------------------------------------------------------------------
# subcommands
# --------------------------------------------------------------------------
def cmd_index(args):
    scenarios, vecs, _, _defs = build_index(args.watn, args.model, with_defs=args.with_step_defs)
    npz, js = write_index(args.out, scenarios, vecs, with_defs=args.with_step_defs)
    print(f"indexed {vecs.shape[0]} scenarios x {vecs.shape[1]} dims"
          + (" (step-definition enriched)" if args.with_step_defs else ""))
    print(f"wrote {npz}, {js}")


def cmd_query(args):
    if not os.path.exists(args.out + ".npz"):
        if not args.watn:
            sys.exit("no index found; pass --watn to build it first (or --index)")
        print(f"index missing -> building it from {args.watn}")
        scenarios, vecs, _, _ = build_index(args.watn, args.model)
        write_index(args.out, scenarios, vecs)
    meta, vecs, with_defs = load_index(args.out)
    print(f"index: {len(meta)} scenarios x {vecs.shape[1]} dims"
          + (" [step-definition enriched]" if with_defs else "")
          + " (query is embedded WITHOUT step definitions)")
    embedder = load_embedder(args.model)

    def run_query(text):
        for sc in text_to_scenarios(text):
            qv = np.asarray(list(embedder.embed([scenario_text(sc)]))[0], dtype=np.float32)
            print(f"\nquery: {sc['title']!r} ({len(sc['body'])} steps)")
            for rank, (sim, hit) in enumerate(topk(qv, vecs, meta, args.topk), start=1):
                flag = ""
                if sim >= args.diff_threshold and hit["title"] != sc["title"]:
                    flag = ("   <<< near-duplicate: MERGE if same invariant/value variant "
                            "(Examples row / parameter / delete); otherwise it is a different "
                            "boundary -> keep but rename to state the boundary explicitly")
                print(f"  {rank:>2}  sim={sim:.4f}  {hit['file']}:{hit['line']}")
                print(f"        {hit['title']}{flag}")

    if args.text:
        run_query(args.text)
    elif not sys.stdin.isatty():
        run_query(sys.stdin.read())
    else:
        print("interactive: paste a scenario (title + steps), blank line to query, Ctrl-D to quit")
        buf = []
        while True:
            try:
                line = input("scenario> " if not buf else "       > ")
            except EOFError:
                break
            if line.strip() == "":
                if buf:
                    run_query("\n".join(buf))
                    buf = []
                continue
            buf.append(line)
        if buf:
            run_query("\n".join(buf))


def cmd_evaluate(args):
    scenarios, vecs, _, defs = build_index(args.watn, args.model, with_defs=args.with_step_defs)
    keys = {(s["file"], s["title"]): s for s in scenarios}
    vec = {(s["file"], s["title"]): v for s, v in zip(scenarios, vecs)}
    def text_of(s):
        return scenario_text_with_defs(s, defs) if args.with_step_defs else scenario_text(s)

    # labeled ground truth from docs/feature-step-overlap-report.md
    K = lambda f, t: (f"{f}.feature", t)
    PAIRS = [
        ("dup", K("credential-sources/credential-sources", "A literal saved credential is authoritative over environment fallback"),
                K("provider-setup/provider-setup", "A literal saved credential is authoritative over environment fallback"), "F1 identical title"),
        ("dup", K("model-autosuggest/model-autosuggest", "The newest search result stays visible when an older result arrives later"),
                K("search-concurrency/search-concurrency", "The newest search result stays visible when an older result arrives later"), "F2 identical title"),
        ("dup", K("auto-init-config/auto-init-config", "First non-TTY run does not create a config file"),
                K("config/config", "Missing config file prints provider setup guidance"), "F3 missing-config subset"),
        ("dup", K("interactive-shell-shortcut/interactive-shell-shortcut", "The generated Bash widget runs through Bash without evaluating its result"),
                K("interactive-shell-shortcut/interactive-shell-shortcut", "The generated Bash widget keeps the request visible and does not evaluate the command"), "F4 bash widget subset"),
        ("dup", K("interactive-shell-shortcut/interactive-shell-shortcut", "Failed or empty output preserves the original command line"),
                K("interactive-shell-shortcut/interactive-shell-shortcut", "Failed or empty generation preserves the original buffer"), "F5 failed/empty output duplicate"),
        ("dup", K("model-autosuggest/model-autosuggest", "No matching model produces a clear empty state"),
                K("ratatui-model-picker/ratatui-model-picker", "Empty filter result produces a clear empty state"), "F6 empty search subset"),
        ("dup", K("reasoning-policy/reasoning-policy", "Unknown persisted reasoning remains active and is sent"),
                K("streamlined-setup/streamlined-setup", "Existing unknown reasoning remains active after rerunning setup"), "F12 unknown-reasoning duplicate"),
        ("dup", K("reasoning-policy/reasoning-policy", "Minimal reasoning is persisted and sent"),
                K("streamlined-setup/streamlined-setup", "Free-form reasoning survives persistence and request construction"), "F12 persisted-reasoning duplicate"),
        ("legit", K("ask/ask", "Version flag prints logo and version"),
                K("release-truth/release-truth", "Version flag reports the package version"), "F7 debug vs release binary"),
        ("legit", K("credential-sources/credential-sources", "A literal saved credential is authoritative over environment fallback"),
                K("credential-sources/credential-sources", "Provider-specific environment fallback precedes generic fallback"), "credential matrix: literal vs fallback"),
        ("legit", K("credential-sources/credential-sources", "Provider-specific environment fallback precedes generic fallback"),
                K("credential-sources/credential-sources", "A missing saved environment credential fails before discovery"), "credential matrix: fallback vs missing"),
        ("legit", K("incremental-sse-rendering/incremental-sse-rendering", "A DONE event completes a stream successfully"),
                K("incremental-sse-rendering/incremental-sse-rendering", "EOF without DONE is a truncated stream"), "streaming: DONE vs EOF"),
        ("legit", K("incremental-sse-rendering/incremental-sse-rendering", "Partial network reads are reassembled into complete events"),
                K("incremental-sse-rendering/incremental-sse-rendering", "Malformed nonessential events do not discard valid content"), "streaming: reassembly vs malformed"),
        ("legit", K("transport/transport", "Normal debug requests ignore test routing settings"),
                K("transport/transport", "Test-support requests use isolated routing without changing saved configuration"), "transport: normal vs test-support"),
        ("legit", K("transport/transport", "Missing or whitespace test overrides fall back to the configured provider"),
                K("transport/transport", "Provider readiness ignores the test routing setting"), "transport: fallback vs readiness"),
        ("legit", K("model-autosuggest/model-autosuggest", "The newest search result stays visible when an older result arrives later"),
                K("search-concurrency/search-concurrency", "The terminal model picker displays the newest overlapping search result"), "stale-result: regular vs PTY"),
        ("legit", K("reasoning/reasoning", "Thinking tier sends reasoning without printing it"),
                K("reasoning/reasoning", "Thinking tier with verbose flag prints reasoning to stderr"), "reasoning: request vs stderr channel"),
        ("legit", K("unified-setup-wizard/unified-setup-wizard", "Setup wizard guides provider and model configuration page by page"),
                K("streamlined-setup/streamlined-setup", "Coordinated setup completes provider models reasoning and shell choices"), "F10 unified vs streamlined flow"),
    ]

    missing = [(a, b) for _, a, b, _ in PAIRS if a not in keys or b not in keys]
    if missing:
        for a, b in missing:
            print(f"  MISSING key: {a} / {b}")
        sys.exit("ground-truth lookup failed; update PAIRS")

    rows = []
    for label, a, b, note in PAIRS:
        sim = cosine(vec[a], vec[b])
        jac = jaccard(text_of(keys[a]), text_of(keys[b]))
        tmatch = 1.0 if keys[a]["title"] == keys[b]["title"] else 0.0
        rows.append((label, a, b, note, sim, jac, tmatch))
    rows.sort(key=lambda r: -r[4])

    print("\n== labeled pairs by embedding similarity ==")
    print(f"{'label':6} {'sim':>7} {'jac':>6} {'title':>6}  note")
    for label, a, b, note, sim, jac, tmatch in rows:
        print(f"{label:6} {sim:7.4f} {jac:6.3f} {tmatch:6.0f}  {note}")

    dups = [r[4] for r in rows if r[0] == "dup"]
    legits = [r[4] for r in rows if r[0] == "legit"]
    print("\n== separation ==")
    print(f"dup   sim: min={min(dups):.4f} median={sorted(dups)[len(dups)//2]:.4f} max={max(dups):.4f} n={len(dups)}")
    print(f"legit sim: min={min(legits):.4f} median={sorted(legits)[len(legits)//2]:.4f} max={max(legits):.4f} n={len(legits)}")
    print(f"overlap band (max legit - min dup): {max(legits)-min(dups):+.4f}  "
          f"({'SEPARATED' if max(legits) < min(dups) else 'OVERLAPS'})")

    best = min((sum(1 for s in dups if s < t) + sum(1 for s in legits if s >= t), t)
               for t in sorted(set(dups + legits)))
    jbest = min((sum(1 for s in dups if s < t) + sum(1 for s in legits if s >= t), t)
                for t in sorted(set([r[5] for r in rows if r[0] == 'dup'] +
                                    [r[5] for r in rows if r[0] == 'legit'])))
    print(f"best threshold sim>={best[1]:.4f}: {best[0]}/{len(dups)+len(legits)} misclassified")
    print(f"best threshold jaccard>={jbest[1]:.4f}: {jbest[0]}/{len(dups)+len(legits)} misclassified")
    print(f"exact-title match catches {sum(1 for r in rows if r[0]=='dup' and r[6]==1.0)}/{len(dups)} dup pairs")

    # differentiation burden: near-duplicate zone = the author must state the
    # boundary explicitly. Legit pairs here are the cases that must differentiate.
    near = [r for r in rows if r[4] >= args.diff_threshold]
    print(f"\n== near-duplicate zone (sim >= {args.diff_threshold:g}) ==")
    print("   each pair must be classified: MERGE (duplicate / variant -> Examples row,")
    print("   parameter, or delete) or KEEP-AS-BOUNDARY (different production boundary ->")
    print("   rename so the wording looks different enough).")
    for label, a, b, note, sim, _j, _t in near:
        print(f"   {label:6} {sim:7.4f}  {note}")
    legit_near = [r for r in near if r[0] == "legit"]
    print(f"   {len(legit_near)}/{len(legits)} legit boundary pairs fall in the near-duplicate "
          f"zone -> KEEP-AS-BOUNDARY candidates that must be reworded to look distinct")

    # hold-one-out retrieval: would a newly-authored scenario surface its
    # duplicate partner above every other active scenario?
    keys_list = list(keys.keys())
    neighbors = {}
    for a in keys_list:
        lst = [(b, cosine(vec[a], vec[b])) for b in keys_list if b != a]
        lst.sort(key=lambda x: -x[1])
        neighbors[a] = lst

    def rank_margin(q, partner):
        rank = None
        partner_sim = None
        for i, (k, s) in enumerate(neighbors[q], start=1):
            if k == partner:
                rank, partner_sim = i, s
                break
        first, second = neighbors[q][0], neighbors[q][1] if len(neighbors[q]) > 1 else None
        other = second if first[0] == partner else first
        margin = partner_sim - other[1] if partner_sim is not None and other else None
        return rank, partner_sim, margin

    print("\n== retrieval (hold-one-out): a new scenario vs all other active ==")
    print(f"{'label':6} {'best':>4} {'marg>0':>6}  note")
    table = []
    for label, a, b, note, _s, _j, _t in rows:
        r_a, _, m_a = rank_margin(a, b)
        r_b, _, m_b = rank_margin(b, a)
        ranks = [r for r in (r_a, r_b) if r is not None]
        best_rank = min(ranks) if ranks else None
        margins = [m for m in (m_a, m_b) if m is not None]
        best_margin = max(margins) if margins else None
        table.append(dict(label=label, note=note, best_rank=best_rank,
                          best_margin=best_margin, a=a, b=b, a_rank=r_a, b_rank=r_b))
    for d in sorted(table, key=lambda x: (x["best_rank"] is None, x["best_rank"] if x["best_rank"] is not None else 999)):
        ok = d["best_margin"] is not None and d["best_margin"] > 0
        print(f"{d['label']:6} {str(d['best_rank']):>4} {str(ok):>6}  {d['note']}")

    print("\n-- hit@k: partner surfaces among the top-k nearest neighbors --")
    for lab in ("dup", "legit"):
        pairs = [d for d in table if d["label"] == lab]
        n = len(pairs)
        hits = [sum(1 for d in pairs if d["best_rank"] is not None and d["best_rank"] <= k)
                for k in (1, 3, 5)]
        pos = sum(1 for d in pairs if d["best_rank"] == 1 and d["best_margin"] and d["best_margin"] > 0)
        print(f"  {lab:6} n={n:2} hit@1={hits[0]:2} hit@3={hits[1]:2} hit@5={hits[2]:2} "
              f"partner-nearest-with-margin={pos:2}")

    for d in [d for d in table if d["label"] == "dup" and (d["best_rank"] is None or d["best_rank"] > 1)]:
        q = d["a"] if d["a_rank"] is not None and (d["b_rank"] is None or d["a_rank"] <= d["b_rank"]) else d["b"]
        part = d["b"] if q == d["a"] else d["a"]
        r, s, _ = rank_margin(q, part)
        close = neighbors[q][0]
        print(f"\n  miss: query \"{keys[q]['title']}\" ({keys[q]['file']}:{keys[q]['line']})")
        print(f"    partner rank={r} sim={s:.4f}; closest other = \"{keys[close[0]]['title']}\" sim={close[1]:.4f}")

    # all-pairs scan (informational)
    sims = []
    for a, b in itertools.combinations(keys_list, 2):
        s = cosine(vec[a], vec[b])
        if s >= 0.85:
            sims.append((s, a, b))
    sims.sort(reverse=True)
    labeled = {frozenset((a, b)): lab for lab, a, b, _ in PAIRS}
    print(f"\n== top {min(args.topn, len(sims))} most similar pairs (sim>=0.85) ==")
    for s, a, b in sims[: args.topn]:
        lab = labeled.get(frozenset((a, b)), "")
        tag = "DUP  " if lab == "dup" else ("LEGIT" if lab == "legit" else "     ")
        print(f"{s:7.4f} {tag} {a[0]}:{keys[a]['line']} :: {b[0]}:{keys[b]['line']}")
        print(f"         \"{keys[a]['title']}\" <-> \"{keys[b]['title']}\"")


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=False)

    p = sub.add_parser("index", help="parse + embed all active scenarios, write index")
    p.add_argument("--watn", required=True)
    p.add_argument("--model", default=DEFAULT_MODEL)
    p.add_argument("--out", default=DEFAULT_INDEX)
    p.add_argument("--with-step-defs", action="store_true",
                   help="also embed the bodies of the bound step definitions "
                        "from <watn>/tests/steps/*.rs (query stays definition-free)")
    p.set_defaults(fn=cmd_index)

    p = sub.add_parser("query", help="embed new scenario text and show top-k matches")
    p.add_argument("--watn", help="build index if missing (first run)")
    p.add_argument("--model", default=DEFAULT_MODEL)
    p.add_argument("--out", default=DEFAULT_INDEX)
    p.add_argument("--text", help="one-shot query instead of stdin/REPL")
    p.add_argument("--topk", type=int, default=10)
    p.add_argument("--diff-threshold", type=float, default=0.9,
                   help="sim above this flags a hit as 'must differentiate'")
    p.set_defaults(fn=cmd_query)

    p = sub.add_parser("evaluate", help="labeled-pairs + hold-one-out retrieval experiment")
    p.add_argument("--watn", required=True)
    p.add_argument("--model", default=DEFAULT_MODEL)
    p.add_argument("--topn", type=int, default=25)
    p.add_argument("--with-step-defs", action="store_true",
                   help="embed step-definition bodies in the index side of the comparison")
    p.add_argument("--diff-threshold", type=float, default=0.9,
                   help="similarity marking the near-duplicate / must-differentiate zone")
    p.set_defaults(fn=cmd_evaluate)

    args = ap.parse_args()
    if not getattr(args, "fn", None):
        ap.print_help()
        sys.exit(1)
    args.fn(args)


if __name__ == "__main__":
    main()
