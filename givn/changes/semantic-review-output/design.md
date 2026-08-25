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
| Token-cap pair guidance lines (left/right/pair/limit counts) | yes — status-class content, always visible; cap evidence never hides behind a flag | yes | fields |
| E5 similarity candidates (raw list) | no when both second-stage layers participate; byte-identical today's output in fallback mode | yes, unchanged format | array |
| BGE/NLI rows with admission RECOMMENDED | yes, sorted by model confidence (NLI symmetric entailment desc / BGE symmetric raw desc) — this is the problem worklist | yes | array entries |
| BGE/NLI rows FILTERED / UNSCORED | no | yes, embedding-score order | array entries with admissions |
| Model artifact resolution chatter ("resolving … artifacts", stderr) | start and completion each collapsed to one line per stage | current lines | absent |

### Corroborated gating (union signal)

When both evidence layers participate — both runs exist with status
`*_ENABLED` covering the candidate pool — the review's failure condition
is the **union of recommendations**: any candidate with
`BGE_RECOMMENDED` or `NLI_RECOMMENDED`. E5-only hits above 0.95 that
both models filter are noise; they appear in counters and JSON evidence
only and do not fail the run.

- Fallback (fail-safe): if either layer is missing, disabled,
  unavailable, or unable to score a candidate, that candidate falls
  under today's E5-threshold rule. With no participating second stage
  at all, behaviour is byte-identical to today, including the raw E5
  list in default output.
- Both surfaces: `givn spec review` and the blocking workflow path
  (`givn check review`, `givn archive`) use the same rule. The blocking
  report's unresolved-set already exempts filtered pairs, so the report
  contract needs no change; only the embedded gate call in
  `check review`/`archive` switches from threshold-only to corroborated
  gating.
- Failure message names the union: "semantic review flagged N
  recommended candidate(s)" with the findings path appended when
  persistence applies. In fallback mode the established threshold
  message is kept verbatim.
- Exit codes are unchanged: non-zero on flagged problems, zero when
  clean.
- "Combined" keeps its existing intersection meaning (pinned by ADR-061
  and the permanent specs); it is a reporting counter only. The verdict
  uses the union count, exposed as `flagged_recommended_count`.

### Findings persistence scope

Only the authoring command (`givn spec review`) writes
`review-findings.json`. The blocking surfaces keep `semantic-review.md`
as their persisted evidence; `ReviewOptions::default()` therefore never
persists findings, and `check review`/`archive` output and artifacts
are otherwise untouched.

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
- `run_embedding_review` collects the raw candidate evidence and prints
  it only under `--verbose`; the verdict moves to the corroborated rule.
  The blocking entrypoints (`check review`, `archive`) reuse the same
  gate function via `review_without_index_notice` /
  `review_without_download`, so they inherit the corroborated rule with
  text defaults.
- `GIVN_SPEC_NLI_REPORT_LIMIT` stays parsed but ignored (deprecated
  alias); documented as such, no error, no warning line.
- Resolution chatter: hoist repeated "resolving … artifacts" eprintln
  sites behind per-stage dedupe; default emits at most one start line
  and one ready summary per stage. Download-event streaming
  (`GIVN_SPEC_DOWNLOAD_EVENTS`) is untouched.
- JSON stdout purity: the index-refresh banner lines ("Indexed N
  permanent scenarios", "Reused M vectors", "E5 token-cap candidates:
  N") stay on stdout in text mode unchanged; in JSON mode their content
  is embedded in the document as an `index` summary section instead of
  being printed as banner lines. Stdout then parses as exactly one JSON
  document.

### Test-only seams

- Resolution-notice seam: a test-only stub backend mode that emits the
  repeated artifact-resolution notices deterministically, so the
  chatter-collapse and verbose-restores-detail scenarios can fail in
  RED under stub backends. Assertions target stderr, where the notices
  actually print.

### JSON document

New serializer producing one document (schema_version 1) that **extends
the existing `spec duplicates --format json` contract** rather than
inventing a second dialect: identical field names and nesting for the
shared run structs and candidates (`bge_run`, `nli_run`,
`evidence.bge`/`evidence.nli`, `nli_filtered_candidates`,
`unscored_candidates`, `combined_recommended_candidates`) under the same
ADR-054 versioning regime, plus review-only additions:

```json
{
  "schema_version": 1,
  "source": "givn spec review",
  "change": "<id>",
  "index": {"indexed": 120, "reused": 118, "e5_token_caps": 0},
  "verdict": {"problem_flagged": true, "flagged_recommended_count": 1,
              "threshold_fallback": false, "e5_candidates": 115, "threshold": 0.95},
  "candidates": [
    {"scope": ["delta-to-delta"],
     "left": {"title": "...", "file": "..."},
     "right": {"title": "...", "file": "..."},
     "embedding_score": 0.9264,
     "admission": {"bge": "BGE_FILTERED", "nli": "NLI_FILTERED"},
     "evidence": {"bge_symmetric_raw": -2.3944,
                   "nli_symmetric": {"entailment": 0.0092, "neutral": 0.0403,
                                      "contradiction": 0.9505}}}
  ]
}
```

The `candidates` array reuses the duplicates candidate entry grammar;
`admission` and `evidence` reuse the existing per-candidate evidence
nesting. The contract is: a consumer that parses the duplicates JSON
parses the review document's shared sections unchanged; the review-only
sections (`change`, `index`, `verdict`) are additive. Exit codes and
stderr failure messages do not change.

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
- Named permanent-pin migrations (stakeholder-visible behaviour
  changes, each its own implementation task inside the flipping
  scenario's commit): `tests/steps/nli_evidence_steps.rs:160-168`
  (permanent scenario "Combined recommendations..." pins
  `BGE_FILTERED` detail presence and intersection wording),
  `tests/steps/bge_reranking_evidence_steps.rs:999` ("...recorded as
  filtered from the recommended list"),
  `tests/steps/nli_evidence_e2e_steps.rs:253` (`NLI_FILTERED`
  detail presence), `tests/steps/nli_evidence_steps.rs:601-605`,
  `tests/steps/bge_reranking_evidence_steps.rs:982`.
- Characterization first: existing assertions in
  `tests/steps/bge_reranking_evidence_steps.rs` (:982 asserts absence,
  :999 presence of `BGE_FILTERED`), `nli_evidence_steps.rs` (:601-605),
  `nli_evidence_e2e_steps.rs` (:253 asserts `NLI_FILTERED` present in
  default advisory-review stdout — inverted by this change),
  `semantic_review_steps.rs`, and `semantic_token_cap_review_steps.rs`
  pin today's output per fixture. Capture current behaviour as
  characterization scenarios before flipping defaults; migrate those
  fixtures' expectations deliberately rather than wholesale. Pins
  backing permanent-spec scenarios (`nli-evidence`,
  `bge-reranking-evidence`) require migrating the permanent scenario
  expectations too; each affected permanent scenario is an explicit
  implementation task, not fixture churn. Audit all step files for
  assertions on presence of `*_FILTERED`/unscored detail in review
  stdout before flipping defaults.

## Test Runner And Strictness

Runner: `./run-tests.sh`; E2E smoke: `./run-tests.sh --e2e`
(`givn/commands.yaml`). The delta scenarios carry `@review-output`, a
dedicated runner group (`run_review_output` in `run-tests.sh`) that
compiles `spec-similarity-gate` on top of the combined stub stack;
existing `@nli`/`@nli-combined` groups exclude the tag. Strict mode:
`GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`; not-implemented stubs raise
`unimplemented!()`.

Single-scenario command template:

```sh
GIVN_FEATURES=givn/changes/semantic-review-output/specs \
  cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence,spec-similarity-gate \
  --test features_runner -- \
  --name "<scenario title>"
```

Local runnability: deterministic stub backends cover all behaviours —
visibility, ordering, JSON shape, findings file, deprecated env var,
chatter via the resolution-notice seam. No external services or digital
twins required; none apply. Real-model runs remain manual verification
outside the suite.

Determinism rule for output comparisons: elapsed-runtime values vary
between processes. Step bindings that compare two runs' outputs
byte-wise or assert full-output equality normalize runtime fields
(strip `runtime ... ms` / `runtime_ms`) before comparing; every other
line must match exactly.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run review, recommended pair visible without flags while filtered pairs are not | Real CLI review surfaces only recommended second-stage pairs | CLI / terminal | Cucumber E2E binding starts compiled `givn spec review --change <id>` subprocess; asserts stdout contains the recommended pair's detail line and no FILTERED detail lines |
| run review with `--verbose` and see every scored pair | Real CLI verbose review lists all second-stage evidence | CLI / terminal | Same binding with flag; asserts FILTERED rows present, raw E5 list present, truncation disclaimer absent |
| run review with `--format json` and parse stdout | Real CLI JSON review emits one structured document | CLI / terminal | Same binding; asserts stdout parses as schema_version 1 document with counter parity to text run |
| force a flagged review and inspect the written report | Real CLI persists findings beside the change after flagged recommendations | CLI / terminal | Same binding on a union-flagged corpus; asserts `review-findings.json` exists under the change dir, parses, matches stdout document |
| run the blocking workflow gate over a recommended pair | Real CLI blocking review flags recommendations through the workflow gate | CLI / terminal | Compiled-binary subprocess runs `givn check review --change <id>` on a union-flagged corpus; exits non-zero with the flagged message; `semantic-review.md` exists; no findings file is written by this surface |

The first four inventory entries are normalized actions of "run
advisory review" (flag variants); the fifth is the distinct blocking
workflow action.

## Decisions

- `--format` over a bare `--json` flag: house consistency with
  `spec explore` / `spec duplicates`.
- Recommended-first ordering by model confidence, not embedding rank:
  the ranking that surfaced scaffolding noise above the lone genuine
  entailment in the observed 113-pair run.
- Persist on failure only: green runs must not accumulate artifacts;
  failed reviews are the ones worth auditing without rerun.
- E5 raw list leaves the default output (stakeholder decision): an E5
  score alone is not actionable — observed 113 filtered pairs buried
  the one real finding. Verbose restores the list; counters and JSON
  keep the numbers. Reverses the earlier "E5 stays default-visible"
  decision together with the gate change below.
- Corroborated gating with union signal (stakeholder decision):
  problems are BGE- or NLI-recommended pairs; E5-only hits are noise.
  Fail-safe fallback keeps the E5 threshold binding whenever a second
  stage cannot participate, so recall boundaries (R-13/R-14/R-15) hold.
  Applies to both surfaces; blocking-report contract unchanged because
  its unresolved-set already exempts filtered pairs.
- Deprecated env var is silently honoured-but-ignored instead of
  removed: zero-cost compatibility for scripts and fixtures.
- Token-cap guidance lines stay default-visible (design-review): cap
  evidence is status-class content; hiding it would reduce a capped
  pair to an unexplained counter and break the shorten-or-split
  remediation loop mandated by the retrieval instructions.
- Index banners embed into the JSON document (design-review): in JSON
  mode the refresh outcome becomes an `index` summary section instead
  of stdout banner lines, keeping stdout parseable as one document;
  text mode keeps today's banner lines byte-identical.
- Chatter scenarios assert stderr and use a test-only resolution-notice
  stub seam (design-review): stub backends emit no real resolution
  notices, so without the seam both scenarios pass vacuously.
- Review JSON extends the duplicates document shape (design-review):
  one evidence grammar under ADR-054 versioning; review-only sections
  (`change`, `index`, `verdict`) are additive. A second dialect for the
  same run structs was rejected as consumer burden.
