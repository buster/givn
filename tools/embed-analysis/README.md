# givn scenario embedding tool

Experiments with CPU sentence embeddings for detecting duplicate / overlapping
Gherkin scenarios, using the `watn` corpus as the test bed.

## Purpose

Two questions:

1. Can an embedding-similarity threshold **gate** new scenarios (block a
   duplicate) — or only *suggest* candidates for review?
2. When a scenario is authored, does whole-scenario embedding retrieve its true
   duplicate reliably over all other active scenarios?

Run `evaluate` for the evidence. Run `index` + `query` to test retrieval
yourself with a scenario of your choosing.

## Setup

Requires `uv` (https://docs.astral.sh/uv). Everything runs on CPU via
`fastembed` (ONNX runtime). The model and index are cached in this directory
(`.embed-cache/`, `index.npz`, `index.json` — gitignored), never in the watn
checkout.

```sh
cd tools/embed-analysis
uv run python analyze.py --help
```

## Commands

### index

Parse and embed every active scenario under `<watn>/givn/specs` and persist the
index.

```sh
uv run python analyze.py index --watn $HOME/projects/watn
# writes index.npz (vectors) + index.json (metadata), 223 scenarios x 768 dims

# optionally embed the bound step-definition bodies too
uv run python analyze.py index --watn $HOME/projects/watn --with-step-defs --out index-defs
```

### query

Embed new scenario text and show the top-k most similar indexed scenarios.
Three input modes:

```sh
# one-shot
uv run python analyze.py query --text 'Scenario: The newest search result stays visible when an older result arrives later' --topk 10

# piped stdin (paste a whole scenario: title + steps)
printf 'Scenario: ...\nGiven ...\nWhen ...\nThen ...\n' | \
  uv run python analyze.py query --topk 10

# interactive REPL (paste a scenario, blank line to query, Ctrl-D to quit)
uv run python analyze.py query --topk 10
```

If `index.npz` does not exist, `query` builds it first (pass `--watn`).

The query input is the same representation that was indexed — scenario title
plus Given/When/Then steps — so a new scenario and the corpus live in the same
vector space. A candidate that ranks a known duplicate at the top is exactly
the retrieval the authoring flow needs.

When a hit is similar but not title-identical, the disposition is a binary
decision, not an unconditional "differentiate" requirement:

- **MERGE** — the scenario is a duplicate or a value variant of the hit
  (same invariant, differs only by a parameter/flag/shell/order). Disposition:
  convert to an Examples row / Scenario Outline / parameterized step, or delete.
- **KEEP-AS-BOUNDARY** — the scenario *looks* similar but exercises a different
  production boundary. Only then must givn find wording that *also looks*
  different enough: rename so the boundary is explicit in the title.

Hits above `--diff-threshold` (default 0.9) are flagged with exactly this
MERGE / KEEP-AS-BOUNDARY decision — a suggestion for the reviewer, never an
automatic block. Tune with `--diff-threshold`.

### evaluate

The offline experiment from `docs/feature-step-overlap-report.md` of the watn
checkout: 8 labeled duplicate/subset pairs vs 10 labeled legitimate
boundary-differing pairs, plus a hold-one-out retrieval test and a
differentiation-burden summary.

```sh
uv run python analyze.py evaluate --watn $HOME/projects/watn
uv run python analyze.py evaluate --watn $HOME/projects/watn --with-step-defs   # index side carries step-def bodies
```

### evaluate --german

givn scenarios may be written in German (with English source code), so the model
must also separate duplicate/variant/boundary pairs in German. The German mode
runs the same labeled experiment on a small synthetic German set (real umlauts,
English technical vocabulary), no corpus needed:

```sh
uv run python analyze.py evaluate --german --model intfloat/multilingual-e5-large
```

## Model

Default `BAAI/bge-base-en-v1.5` (768-dim, ~0.21 GB ONNX, CPU). The corpus is
small (223 scenarios) so a larger embedding is affordable. Swap via `--model`
with any fastembed-supported model (e.g.
`sentence-transformers/all-MiniLM-L6-v2` for a lighter baseline).

**Recommendation: `intfloat/multilingual-e5-large`** (1024-dim). givn scenarios
are not English-only — German Gherkin with English source code is a stated
requirement — so the model must separate duplicates in both languages. e5-large
is the best on the English corpus and the best on the German set (see Model
comparison). It is slower than the 768-dim models, which matters only for the
interactive query path.

### Model comparison (2026-08-14)

Full outputs in `results-*.txt` (English corpus) and `de-*.txt` (German set).

| Model | EN best-threshold errors | EN dup/legit hit@1 | DE best-threshold errors | DE band |
|---|---|---|---|---|
| BAAI/bge-base-en-v1.5 | 4/18 | 6/8 / 5/10 | 0/8 | −0.011 |
| jinaai/jina-embeddings-v2-base-de | 3/18 | 6/8 / 5/10 | 1/8 | +0.006 |
| sentence-transformers/paraphrase-multilingual-mpnet-base-v2 | 3/18 | 6/8 / 6/10 | 1/8 | +0.094 |
| intfloat/multilingual-e5-large | **2/18** | 6/8 / **4/10** | **0/8** | **−0.014** |

Notes:

- The watn corpus is entirely English (0 files with umlauts, no German words),
  so the multilingual models gain nothing there; e5-large still wins.
- On the synthetic German set even the English-only bge model separates
  perfectly, because the scenarios are saturated with English technical terms.
  The "German-native" jina-de and mpnet models actually *miss* the harder
  German duplicate cases (subset / paraphrase), while e5-large and bge do not.
- E5 models expect a `query:` / `passage:` prompt prefix; fastembed may not add
  it automatically, so an explicit prefix in the query path could improve e5
  further.

## Step definitions (`--with-step-defs`)

Parses the cucumber bindings in `<watn>/tests/steps/*.rs` (753 definitions in
watn), matches each scenario's Given/When/Then steps to their binding by
normalized token overlap, and appends the binding's implementation body to the
scenario text before embedding. A query never contains step definitions, so the
asymmetric case (definition-free query against a definition-enriched index) is
what you get in practice.

## Ground truth

The labeled pairs in `cmd_evaluate` are transcribed from the watn report:

- **dup** — F1/F2 (identical titles), F3/F4/F5/F6 (subsets), F12 (reasoning
  same-function pairs).
- **legit** — F7 (version flag, distinct binary), credential precedence matrix,
  streaming parser boundaries, transport override boundaries, stale-result
  regular-vs-PTY, reasoning request-vs-stderr channel, F10 setup flows.

## Results (model BAAI/bge-base-en-v1.5, 2026-08-14)

See `results.txt` (scenarios only) and `results-with-defs.txt` (with step
definitions) for the full output.

- Duplicate pairs: sim 0.777–0.980 (median 0.938). Legit pairs: sim 0.778–0.925
  (median 0.878). Overlap band `+0.148` → **no threshold separates them**.
- Best threshold misclassifies 4/18; deterministic Jaccard baseline 5/18;
  exact-title match catches only 2/8 dup pairs.
- Hold-one-out retrieval: 6/8 known duplicates are the nearest neighbor of their
  partner, all 8 in the top-5. But 5/10 legit boundary pairs are *also* nearest
  neighbors — retrieval cannot distinguish duplicate from legit at the top.
- Failure mode is dense clusters: the two F12 reasoning duplicates rank #2/#5
  because legitimately-similar reasoning neighbors ("Off reasoning omits…",
  "Minimal reasoning…") sit closer than the true duplicate partner.

### With step definitions

- Separation improves but still overlaps: dup median 0.953, legit median 0.919,
  band `+0.107`; best threshold misclassifies 2/18 (from 4/18).
- Retrieval: dup hit@1 unchanged (6/8); legit hit@1 drops 5→3 — step-definition
  bodies push legitimate boundary pairs apart, which is what a differentiation
  tool wants. The F12 dense-cluster misses persist.
- Asymmetric query (definition-free) against a definition-enriched index still
  surfaces both duplicates at rank 1–2 in the tested cases.

## Merge, not gate: the two dispositions

A near-duplicate is not one thing. Embedding similarity surfaces the
near-duplicate *zone*; the reviewer must then place each pair into one of two
dispositions:

- **MERGE** — duplicate, or a `variant` (the report's "merge examples, not
  invariants": same user action / boundary, differing only by value, flag,
  shell, provider, or order). Convert to an Examples row / Scenario Outline, or
  delete the subset.
- **KEEP-AS-BOUNDARY** — the report's `layered`: different production boundary,
  must remain a separate scenario. Only here does the wording need to change so
  the scenario *looks* different enough too.

The differentiation requirement ("find another wording") applies **only to the
second case**. Applying it to the first is wrong — a true duplicate must be
merged, not renamed. The deterministic checks (exact normalized title /
fingerprint) catch the clear duplicates; this tool catches the semantic
near-misses and forces the explicit merge-versus-boundary decision.

## Conclusion

Whole-scenario embeddings are reliable for **surfacing candidates** (retrieval →
review queue), not for **hard gating**: legitimate boundary cases are as close
as duplicates, and in dense semantic clusters the duplicate is indistinguishable
from its legit neighbors. Step-definition bodies improve the separation of
legitimate boundary pairs. A gate must use deterministic checks (exact
normalized title/fingerprint) plus a documented disposition for every
near-duplicate candidate.
