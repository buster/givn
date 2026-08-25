# Design: semantic-review-output

## Technical Approach

All changes stay within the existing Rust retrieval reporting path.
No new dependencies. Versioned choices are pinned in `Cargo.toml`
(fastembed =5.17.2, ort =2.0.0-rc.12 — checked there, not recalled);
no new versioned dependency is introduced.

### CLI surface

`SpecCommands::Review` (`src/main.rs:199`) gains:

- `#[arg(long)] verbose: bool`
- `#[arg(long, value_enum, default_value_t = SpecFormat::Text)] format: SpecFormat`

reusing the existing `SpecFormat {Text, Json}` enum, matching
`spec explore` and `spec duplicates`. Plumb both through
`cli::spec::run_review` into `retrieval::review`.

### Reporting policy

One policy governs every detail listing; row class decides visibility:

| Content | default text | --verbose | json |
|---|---|---|---|
| Stage status lines (enabled/model/pairs/runtime) | yes | yes | stage summaries |
| Accounting + distribution counters | yes | yes | fields |
| E5 similarity candidates (gate-binding verdict evidence) | yes, unchanged | yes, unchanged | array |
| BGE/NLI rows with admission RECOMMENDED | yes, sorted by model confidence (NLI symmetric entailment desc / BGE symmetric raw desc) | yes | array entries |
| BGE/NLI rows FILTERED / UNSCORED / TOKEN_CAP | no | yes, embedding-score order | array entries with admissions |
| Model artifact resolution chatter ("resolving … artifacts") | collapsed to one summary line per stage | current lines | absent |

Line formats for retained rows stay byte-identical to today's,
including the advisory disclaimers. The NLI truncation disclaimer
("reported at most 20 candidates") prints only when truncation actually
occurs — which after this change happens only in deprecated-env mode.

Implementation points:

- `print_nli_results` (`src/spec/retrieval.rs:7205`): replace
  `candidates.iter().take(report_limit)` with visibility filtering on
  `nli_admission`; drop the unconditional top-20 window.
- The BGE per-pair loop (~`:5720`) gains the same filter; it currently
  has none.
- `run_embedding_review` keeps printing its candidate list by default —
  it is the binding gate evidence until corroboration semantics change.
- `GIVN_SPEC_NLI_REPORT_LIMIT` stays parsed but ignored (deprecated
  alias); documented as such, no error, no warning line.
- Resolution chatter: hoist repeated "resolving … artifacts" eprintln
  sites behind a `verbose` flag; default emits one line per stage
  summarizing prepared model and cache state. Download-event streaming
  (`GIVN_SPEC_DOWNLOAD_EVENTS`) is untouched.

### JSON document

New serializer producing one document (schema_version 1):

```json
{
  "schema_version": 1,
  "change": "<id>",
  "verdict": {"similarity_gate_failed": true, "e5_candidates": 115, "threshold": 0.95},
  "stages": {
    "e5":   {"status": "...", "considered": 115},
    "bge":  {"status": "BGE_ENABLED", "model": "...", "considered": 113,
             "recommended": 0, "filtered": 113, "unscored": 0, "runtime_ms": 166801},
    "nli":  {"status": "NLI_ENABLED", "model": "...", "distribution":
             {"entailment": 1, "neutral": 43, "contradiction": 69}, "...": null}
  },
  "candidates": [
    {"scope": ["delta-to-delta"],
     "left": {"title": "...", "file": "..."},
     "right": {"title": "...", "file": "..."},
     "embedding_score": 0.9264,
     "admission": {"bge": "BGE_FILTERED", "nli": "NLI_FILTERED"},
     "evidence": {"bge_symmetric_raw": -2.3944,
                  "nli_symmetric": {"entailment": 0.0092, "neutral": 0.0403,
                                     "contradiction": 0.9505}}}
  ],
  "combined": {"recommended": 0, "of": 113}
}
```

Exact field set finalised against existing run structs (`BgeRun`,
`NliRun`, `SimilarityCandidate`) during implementation; the contract is
"every number visible in today's text counters is present as a JSON
field". Exit codes and stderr failure messages do not change.

### Persisted findings

On similarity-gate failure (and only then), write the identical
document to `givn/changes/<change>/review-findings.json`, overwriting
prior runs. Write failures degrade to a warning naming the path — they
never mask the gate error. Success runs write nothing (no churn on
green corpora).

## Architecture Impact

- `src/main.rs`: Review variant flags.
- `src/cli/spec.rs`: signature change for `run_review`.
- `src/spec/retrieval.rs`: reporting policy filter, JSON document
  builder, findings-file writer, chatter gating, deprecated env var.
- `tests/steps/mod.rs`: register bindings.
- `tests/steps/semantic_review_output_steps.rs`: regular bindings for
  visibility policy, ordering, JSON shape, persisted file, deprecated
  env var behaviour.
- `tests/steps/semantic_review_output_e2e_steps.rs`: compiled-CLI
  subprocess bindings.
- Characterization first: existing assertions in
  `tests/steps/bge_reranking_evidence_steps.rs` (:982 asserts absence,
  :999 presence of `BGE_FILTERED`), `nli_evidence_steps.rs` (:601-605),
  `semantic_review_steps.rs`, and `semantic_token_cap_review_steps.rs`
  pin today's output per fixture. Capture current behaviour as
  characterization scenarios before flipping defaults; migrate those
  fixtures' expectations deliberately rather than wholesale.

## Test Runner And Strictness

Runner: `./run-tests.sh`; E2E smoke: `./run-tests.sh --e2e`
(`givn/commands.yaml`). Strict mode:
`GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`; not-implemented stubs raise
`unimplemented!()`.

Single-scenario command template:

```sh
GIVN_FEATURES=givn/changes/semantic-review-output/specs \
  cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  --test features_runner -- \
  --name "<scenario title>"
```

Local runnability: deterministic stub backends cover all behaviours —
visibility, ordering, JSON shape, findings file, deprecated env var.
No external services or digital twins required; none apply. Real-model
runs remain manual verification outside the suite.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run review, recommended pair visible without flags while filtered pairs are not | Default output reports only model-recommended second-stage pairs | CLI / terminal | Cucumber E2E binding starts compiled `givn spec review --change <id>` subprocess; asserts stdout contains the recommended pair's detail line and no FILTERED detail lines |
| run review with `--verbose` and see every scored pair | Verbose output lists all second-stage evidence | CLI / terminal | Same binding with flag; asserts FILTERED rows present and truncation disclaimer absent |
| run review with `--format json` and parse stdout | JSON review output is a single structured document | CLI / terminal | Same binding; asserts stdout parses as schema_version 1 document with counter parity to text run |
| force a similarity-gate failure and inspect the written report | A failed similarity gate persists its findings beside the change | CLI / terminal | Same binding on a failing corpus; asserts `review-findings.json` exists under the change dir, parses, matches stdout document |

## Decisions

- `--format` over a bare `--json` flag: house consistency with
  `spec explore` / `spec duplicates`.
- Recommended-first ordering by model confidence, not embedding rank:
  the ranking that surfaced scaffolding noise above the lone genuine
  entailment in the observed 113-pair run.
- Persist on failure only: green runs must not accumulate artifacts;
  failed reviews are the ones worth auditing without rerun.
- E5 list stays default-visible while it remains the binding gate
  evidence; revisited by the future corroboration-semantics change.
- Deprecated env var is silently honoured-but-ignored instead of
  removed: zero-cost compatibility for scripts and fixtures.
