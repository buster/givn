# Design: Generate the Semantic Review Report

## Boundary

This slice extends only the blocking change-review gate:

```text
givn check review --change <id>
```

The existing advisory command `givn spec review --change <id>` keeps its existing
behavior and output and does not call the semantic report entrypoint. During
`check review`, after the existing structural overlap gate and before the
existing retrieval path completes, Givn calls a dedicated semantic report
entrypoint and generates:

```text
givn/changes/<id>/semantic-review.md
```

The report is evidence and a classification worklist. Givn never edits a
feature file, permanent spec, `review.md`, or review disposition. The agent owns
all classification and remediation decisions. The generated file is change-local
while active; the existing archive move retains it under
`givn/archive/<id>/` as historical evidence. Archive behavior itself is outside
this slice.

## Architecture Impact

| File | Change |
| --- | --- |
| `src/cli/check.rs` | Call the dedicated semantic report gate only from `givn check review --change`, with the existing structural and retrieval gates preserved. The no-feature path must return the existing retrieval-unavailable error before writing a report. |
| `src/cli/spec.rs` | Keep `givn spec review --change` on its existing advisory retrieval entrypoint; it must not call or create the semantic report. |
| `src/spec/retrieval.rs` | Keep the existing advisory `review` entrypoint separate from a new blocking semantic-report entrypoint. The new entrypoint collects active delta candidates, applies the E5 boundary, compares the two allowed scopes, computes complete endpoint evidence, carries only valid classifications, writes the complete report atomically, and blocks unresolved candidates. |
| `src/overlap/fingerprint.rs` | Reuse the existing public scenario fingerprint and facts model; no behavior change is required in the structural overlap gate. |
| `tests/steps/mod.rs` | Register regular and E2E semantic-review step modules and store report/snapshot state. |
| `tests/steps/semantic_review_steps.rs` | Regular black-box fixtures for report generation, classification, hash invalidation, and non-mutation. |
| `tests/steps/semantic_review_e2e_steps.rs` | Separate real CLI-process steps for the report-generation smoke test. |
| `givn/changes/semantic-review/specs/semantic-review/semantic-review.feature` | Executable report and gate scenarios. |

No new runtime or test dependency is introduced. Existing Gherkin parsing,
E5-compatible embeddings, deterministic stubs, JSON/Markdown filesystem output,
and Cucumber infrastructure are reused.

The runtime flow is:

```mermaid
sequenceDiagram
    participant User
    participant Check as givn check review
    participant Overlap as structural overlap gate
    participant Report as semantic report gate
    participant Index as permanent E5 index
    participant Delta as active change specs
    participant Agent as agent classification

    User->>Check: review change
    Check->>Overlap: run existing structural checks
    Overlap-->>Check: structural result
    Check->>Report: call dedicated semantic report entrypoint
    Report->>Delta: collect active Added/Modified scenarios
    Report->>Index: refresh permanent-only entries
    Report->>Report: embed deltas and retain E5 scores >= 0.9500
    Report->>Report: add complete evidence, facts, ids, and order
    Report->>Report: carry only complete-evidence matches
    Report-->>Check: complete report and PASS/BLOCKED result
    alt retrieval feature unavailable
        Report-->>Check: existing retrieval-unavailable error; no report
    end
    Agent->>Report: edit classifications and feature files
    User->>Check: review change again
    Check->>Report: regenerate change-local report
```

## Candidate Corpus and Identity

The permanent index remains the persisted source for permanent scenario vectors.
For report evidence, Givn reparses the permanent feature tree and builds the
same canonical endpoint model used for delta scenarios; it does not attempt to
recover descriptions, tags, outlines, backgrounds, tables, or docstrings from
the narrow `IndexEntry` record. The index's existing `content_hash` remains an
index-reuse hash; the report's canonical `content_hash` is computed separately
from the complete evidence object.
The report generator collects all active `Added` and `Modified` delta scenarios
from the requested change, including scenarios nested under `Rule:`. Removed
scenarios are excluded before embedding. Active delta vectors are request-local
and are never written to the index.

Each report endpoint contains a `scope` (`permanent` or `current-change`), a
normalized project-relative forward-slash source `file`, a `title`, and the
complete endpoint evidence described below. A current-change endpoint also
records its delta operation and projected target identity. A permanent endpoint
uses `permanent` as its operation and has no projected target.

The candidate universe is exactly the active pairs whose existing E5 cosine
score is at least the binary-fixed `0.9500` boundary:

- `delta-to-permanent`: one active current-change endpoint and one permanent
  endpoint;
- `delta-to-delta`: two distinct active current-change endpoints from the
  requested change.

Removed endpoints and permanent-permanent pairs never enter the universe. For a
Modified endpoint in a delta-to-permanent comparison, the comparison with its
own projected `(target file, title)` identity is excluded. Other permanent
scenarios in that target file remain eligible. Delta-to-delta endpoints are
canonically ordered by endpoint tuple so each pair is emitted once.

The endpoint tuple is `(scope, normalized file, title)`. Candidates are sorted
by ascending scope, descending score, left endpoint tuple, right endpoint tuple,
then candidate id. With the two scope strings in this slice, lexical scope order
places `delta-to-delta` before `delta-to-permanent`. Equal-score candidates
therefore use the endpoint tuples and then candidate id as their deterministic
tie-breakers. Candidate admission compares the raw E5 cosine to `0.9500`
before formatting; scores are then rendered as fixed four-decimal strings in
the range `0.9500` through `1.0000`. The exact-boundary fixture supplies a raw
score of exactly `0.9500`, not a below-boundary value that merely rounds to
`0.9500`. Candidate, content, and evidence hashes match `[0-9a-f]{16}` exactly.
The report has no timestamps, host paths, environment values, or other
run-dependent fields.

The candidate id is a stable hash of the report schema version, scope, canonical
left and right endpoint tuples, the fixed four-decimal score string, and both
endpoint evidence hashes. It is emitted once per pair. The report uses the
existing E5 passage vectors and cosine calculation only. This semantic-report
entrypoint does not invoke BGE, NLI, remediation, automatic edits, or
permanent-permanent auditing.

## Complete Endpoint Evidence

The report's `content_hash` is a fixed-format hash of canonical scenario
content. Canonical content is one ordered object containing feature, Rule, and
scenario descriptions and tags, Scenario Outline names and Examples tables,
the title, `feature_background_steps`, `rule_background_steps`, and
`normalized_steps`.
Each step record is an ordered object with exactly these keys:

```json
{"text":"Given a configured provider catalog","docstring":null,"data_table":null}
```

The canonical object also retains feature/rule/scenario descriptions,
feature/rule/scenario tags, Scenario Outline names, and Examples tables in
source order. `text` is the normalized step including its keyword. A present
`docstring` is
`{"content_type":<json-string-or-null>,"content":<json-string>}`. A present
`data_table` is an array of row arrays, with rows and cells preserved in source
order. The three step arrays are consumed in this exact order: feature
`Background`, applicable `Rule` `Background`, then scenario steps. A scenario
without a Rule background has an empty `rule_background_steps` array.

The E5 embedding text is a separate input. The existing passage-text
serialization used for E5 scoring remains unchanged; it is not replaced by the
canonical evidence JSON. The complete evidence representation below is used
for content/evidence hashes and report output only.

Each endpoint also records an `evidence_hash`, computed from this complete
canonical evidence object with keys in this order:

```json
{"scope":...,"file":...,"title":...,"feature_description":...,"rule_description":...,"scenario_description":...,"feature_tags":...,"rule_tags":...,"scenario_tags":...,"outline_name":...,"examples":...,"feature_background_steps":...,"rule_background_steps":...,"normalized_steps":...,"delta_operation":...,"projected_target":...,"content_hash":...,"model_id":...,"index_version":...,"fingerprint_schema_version":...,"evidence_schema_version":...,"report_schema_version":...,"deterministic_facts":...}
```

The object includes endpoint scope, normalized source file, title, all ordered
step records and table cells, delta operation and projected target identity,
content hash, E5 model id and permanent index version, deterministic
Fingerprint facts, fingerprint/evidence schema version, and semantic report
schema version. A permanent endpoint uses `permanent` for its operation and
`null` for its projected target, so its evidence is canonical too. The current
fixed values are model `intfloat/multilingual-e5-large`, index version `1`,
fingerprint/evidence schema version `1`, and report schema version `1`.

JSON strings and arrays use the compact deterministic JSON serialization used by
the report writer. Quotes, backslashes, control characters, newlines, and table
cells are escaped as JSON; array order is never sorted. `null` is the only
absent representation. Bare `absent`, omitted fields, empty strings, and
unknown object keys are invalid. The evidence hash is the complete
carry-forward boundary: changing any listed component invalidates a prior
classification, even when the scenario title and visible steps are unchanged.

## Deterministic Facts

The report reuses the existing `Fingerprint` facts for both endpoints and
includes the facts in each endpoint's evidence hash. It records available facts
without interpreting them as a semantic classification:

- extracted exit code when present;
- `contains` assertion polarity by position;
- masked data-table facts;
- `[DONE]` title marker;
- TTY mention.

Missing facts are represented as `null` in the report and evidence. Fact
contradiction is computed from both endpoint fact sets at candidate level. It is
not included in either endpoint evidence object or endpoint evidence hash, and
it does not assign a Givn classification automatically.
The fingerprint schema version is part of the endpoint evidence so a change in
fact extraction invalidates carry-forward.

## Report Format and Classification Carry-Forward

`semantic-review.md` has a strict wire grammar. It is UTF-8 with LF line
endings, has no BOM, and has exactly one final newline with no trailing blank
line. The first five lines are always, in this exact order:

```text
# Semantic Review
Report schema version: 1
Change: <json-string>
Status: PASS|BLOCKED
Candidate count: <canonical-unsigned-integer>
```

When the candidate count is greater than zero, one blank line follows the
header and each candidate block follows in contiguous ordinal order. Candidate
blocks are separated by one blank line; there is no blank line after the final
field. A zero-candidate report ends after the `Candidate count` line.

The candidate block header is exactly `## Candidate <ordinal>`. Its allowed
fields, exactly once and in this order, are:

```text
candidate_id
classification
rationale
scope
left_scope
left_file
left_title
left_evidence_json
left_feature_description
left_rule_description
left_scenario_description
left_feature_tags
left_rule_tags
left_scenario_tags
left_outline_name
left_examples
left_feature_background_steps
left_rule_background_steps
left_normalized_steps
left_delta_operation
left_projected_target
left_content_hash
left_evidence_hash
left_model_id
left_index_version
left_fingerprint_schema_version
left_evidence_schema_version
left_report_schema_version
left_deterministic_facts
right_scope
right_file
right_title
right_evidence_json
right_feature_description
right_rule_description
right_scenario_description
right_feature_tags
right_rule_tags
right_scenario_tags
right_outline_name
right_examples
right_feature_background_steps
right_rule_background_steps
right_normalized_steps
right_delta_operation
right_projected_target
right_content_hash
right_evidence_hash
right_model_id
right_index_version
right_fingerprint_schema_version
right_evidence_schema_version
right_report_schema_version
right_deterministic_facts
embedding_score
fact_contradiction
```

The complete one-candidate wire example is:

```text
# Semantic Review
Report schema version: 1
Change: "running-change"
Status: BLOCKED
Candidate count: 1

## Candidate 1
- candidate_id: 0123456789abcdef
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/running-change/specs/running.feature"
- left_title: "Unknown provider is rejected"
- left_feature_description: null
- left_rule_description: null
- left_scenario_description: null
- left_feature_tags: ["givn.delta", "running-change"]
- left_rule_tags: []
- left_scenario_tags: ["givn.added"]
- left_outline_name: null
- left_examples: []
- left_feature_background_steps: []
- left_rule_background_steps: []
- left_normalized_steps: [{"text":"Given a configured provider catalog","docstring":null,"data_table":null},{"text":"When the user selects an unknown provider","docstring":null,"data_table":null},{"text":"Then the command rejects the request","docstring":null,"data_table":null}]
- left_delta_operation: added
- left_projected_target: null
- left_content_hash: 0123456789abcdef
- left_evidence_hash: fedcba9876543210
- left_model_id: "intfloat/multilingual-e5-large"
- left_index_version: 1
- left_fingerprint_schema_version: 1
- left_evidence_schema_version: 1
- left_report_schema_version: 1
- left_deterministic_facts: {"exit_code":null,"contains_polarity":[],"tables":[],"done":false,"tty":false}
- right_scope: permanent
- right_file: "givn/specs/retrieval.feature"
- right_title: "Provider name is unknown"
- right_feature_description: null
- right_rule_description: null
- right_scenario_description: null
- right_feature_tags: []
- right_rule_tags: []
- right_scenario_tags: []
- right_outline_name: null
- right_examples: []
- right_feature_background_steps: []
- right_rule_background_steps: []
- right_normalized_steps: [{"text":"Given a configured provider catalog","docstring":null,"data_table":null},{"text":"When the user selects an unknown provider","docstring":null,"data_table":null},{"text":"Then the command rejects the request","docstring":null,"data_table":null}]
- right_delta_operation: permanent
- right_projected_target: null
- right_content_hash: 0011223344556677
- right_evidence_hash: 7766554433221100
- right_model_id: "intfloat/multilingual-e5-large"
- right_index_version: 1
- right_fingerprint_schema_version: 1
- right_evidence_schema_version: 1
- right_report_schema_version: 1
- right_deterministic_facts: {"exit_code":null,"contains_polarity":[],"tables":[],"done":false,"tty":false}
- embedding_score: 0.9876
- fact_contradiction: false
```

Non-empty deterministic-facts values use the same compact JSON shape, for
example `contains_polarity:[[0,"positive"]]` and
`tables:[["header","masked-value"]]` in the serialized object. The parser
validates these arrays and their ordering rather than treating them as opaque
free text.

Enums are bare tokens with the following allowed values: `classification` is
`DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`;
`scope` is `delta-to-delta` or `delta-to-permanent`; endpoint scope is
`permanent` or `current-change`; and `delta_operation` is `added`, `modified`,
or `permanent`. Versions and candidate count are canonical unsigned decimal
integers. `embedding_score` is a decimal matching `0\.(?:9[5-9][0-9]{2})` or
`1.0000`, with exactly four digits after the decimal point. All candidate,
content, and evidence hashes match `[0-9a-f]{16}` exactly. A projected target
is either `null` or a two-string JSON array `[target_file, title]` in that
order.

Every JSON-valued field uses compact deterministic JSON serialization. Strings use
JSON escaping, arrays preserve source order, step objects use exactly the keys
`text`, `docstring`, and `data_table` in that order, and table values are JSON
arrays of rows whose cells are JSON strings in source order. A present docstring
object uses exactly `content_type` followed by `content`; a deterministic-facts
object uses exactly `exit_code`, `contains_polarity`, `tables`, `done`, and
`tty`, with position/key pair arrays ordered numerically/lexically. `null` is the
only absent representation; bare `absent`, omitted fields, empty strings, and
unknown fields are invalid. Timestamps, host paths, environment values, and
other run-dependent fields are not allowed anywhere in the document.

The report status is `PASS` when the candidate count is zero or every candidate
has one of `DUPLIKAT`, `VARIANTE`, or `VALID-BOUNDARY` and a non-empty
agent-authored `rationale`. A candidate that appears unrelated but remains
above the boundary stays unresolved until vocabulary and invariants are
sharpened or it is resolved as one of the three allowed classifications. It is
`BLOCKED` when any candidate is missing a classification or rationale, has an
invalid value, or remains `UNCERTAIN`. A blocked report is written completely
before the command exits non-zero.

Before regeneration, a strict parser validates the prior report's exact header,
final-newline rule, schema version, change id, candidate count, contiguous
candidate blocks, field order, allowed values, endpoint tuples, fixed
score/hash formats, JSON serialization, and absence of unknown fields. It
extracts only candidate id, classification, rationale, and both complete endpoint
evidence hashes. Status and candidate count are never trusted as prior state. A missing,
invalid, duplicate, stale, unknown-field, or structurally malformed entry is
discarded; duplicate ids discard every entry with that id. A malformed report
header or schema discards all prior entries. Regenerated candidates with no
valid matching prior entry start as `UNCERTAIN`.

A classification and its rationale are carried forward only when the candidate
id and both complete endpoint evidence hashes match exactly. A missing rationale
remains blocked. The
evidence hash covers content,
feature and Rule backgrounds, scenario steps, docstrings, table rows/cells,
operation, projected target, model, index, and schema versions, so any changed
component invalidates the decision. Status and count are recomputed from the
regenerated candidates. The complete report is written to a temporary file in
the report directory and atomically renamed into place before a blocked result
is returned. No artificial rename-failure seam is required.

The version-invalidation fixtures do not mutate compile-time constants through
the CLI. For model, index, fingerprint, evidence, and report version examples,
the fixture starts with a classified report whose candidate metadata/evidence
uses an older version, then regenerates with the current fixed version. The
observable result is a different regenerated evidence hash for the specific
candidate and `UNCERTAIN` classification.

The command error names the unresolved count and instructs the agent to edit
`semantic-review.md`. A successful run reports that the semantic review passed;
existing structural overlap and retrieval advisory output remain available.

## Gate Integration

`givn check review --change` retains this order:

1. Existing configured verification command.
2. Existing E2E verification command.
3. Existing integrity checks.
4. Existing structural overlap/disposition gate.
5. New semantic report generation and classification gate.
6. Existing feature-gated retrieval behavior, with the strict similarity gate
   enabled only when `spec-similarity-gate` (which includes `spec-retrieval`) is
   compiled.

The dedicated semantic report entrypoint is called only by blocking `givn check
review --change`. The advisory `givn spec review --change` continues to call its
existing retrieval path, remains advisory, and does not create
`semantic-review.md`. A regression invokes both commands against the same
fixture and asserts that only the blocking command can create the report.

The semantic report gate does not replace or suppress the existing strict
similarity gate. A build with `spec-similarity-gate` runs that existing gate
after the report gate; a report classification cannot override a strict-gate
failure. The semantic-review fixture matrix uses `spec-retrieval` without
`spec-similarity-gate` for report behavior, while the strict-gate branch remains
covered by its existing contract.

When the binary lacks `spec-retrieval`, the dedicated entrypoint returns the
existing `spec retrieval is unavailable in this build; rebuild with
--features spec-retrieval` error. It exits non-zero, does not report semantic
`PASS`, and does not create `semantic-review.md`.

The report generator may use the existing permanent-only index refresh. A
missing, malformed, stale, or incompatible index is treated as stale; refresh
rebuilds it from `givn/specs/**/*.feature` and semantic report generation then
continues. The non-mutation contract covers feature files, permanent specs,
`review.md`, and disposition rows byte-for-byte, plus request-local delta
vectors and report classifications. The index is asserted structurally, not
byte-for-byte: after the run it contains only permanent `givn/specs` entries
with permanent vectors, and contains no `Delta` state, delta vectors,
classifications, or semantic-report state.

Before reuse or scoring, the index validator rejects entries with duplicate
`(file, title)` identities, paths outside `givn/specs/`, non-finite or empty
vectors, vectors with a dimension different from the other entries, or invalid
top-level model/version metadata. Any such entry invalidates the entire index
and triggers the permanent-only rebuild. An empty permanent corpus remains a
valid empty index.

The semantic report score layer has an explicit fixture-only override when the
embedder is the deterministic stub. A `GIVN_SPEC_STUB_PAIR_SCORES` mapping
names canonical endpoint title pairs and supplies fixed scores such as `0.9499`,
`0.9500`, or an equal score. The override is applied after vector generation
and before boundary admission; the stub embedder itself remains pair-independent.
Normal runs never set this variable, so real E5 scoring is unaffected.

The generated report is change-local evidence rather than a disposable file. It
remains under the active change until the existing archive move places the
complete change directory under `givn/archive/<id>/`; that historical retention
is documented but archive implementation is outside this slice.

## Step Definitions

The existing `features_runner` uses `.fail_on_skipped()` as strict mode. New
regular bindings live in `tests/steps/semantic_review_steps.rs`; new E2E
bindings live separately in `tests/steps/semantic_review_e2e_steps.rs`.
Every new RED step starts with Rust `unimplemented!()`. No empty step body is
permitted.

The repository runner is:

```sh
./run-tests.sh
```

The change-specific runner is:

```sh
GIVN_FEATURES=givn/changes/semantic-review/specs \
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
cargo test --features spec-retrieval --test features_runner
```

The single-scenario command is:

```sh
GIVN_FEATURES=givn/changes/semantic-review/specs \
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub \
cargo test --features spec-retrieval --test features_runner -- \
  --name "Removed delta scenarios produce a clean semantic report"
```

The E2E command is the configured `./run-tests.sh --e2e` with the change
features path set for the change-specific run. It starts the compiled CLI
subprocess and asserts on its exit status and generated report contents.

## Local Runnability and External Dependencies

The complete local command is `./run-tests.sh`. This capability is CLI-only.
It has no server, database, queue, browser, or external service, so no digital
twin or isolated service network is required. Fixtures create a temporary
project and use `GIVN_SPEC_EMBEDDER=stub`; no model download or network access
is required. Unavailable-model, malformed-delta, malformed-index, and no-feature
scenarios assert non-zero failure rather than a false semantic pass; the
no-feature path also asserts that no report is created.

The e2e step writes the fixture project, invokes the actual compiled CLI, waits
for completion, and reads the report file and captured CLI output. There is no
stdin lifecycle or session obstacle in this interaction.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Change feature runner, non-E2E | `measure-coverage.sh` | `features_runner` under `cargo llvm-cov` | `coverage/non-e2e-cobertura.xml` | `merge-coverages.sh` | `src/spec/retrieval.rs` report generation and gate path |
| Change feature runner, E2E | `measure-coverage.sh` | `features_runner` under `cargo llvm-cov` | `coverage/e2e-cobertura.xml` | `merge-coverages.sh` | CLI subprocess dispatch and report output |

The existing project scripts provide collision-safe separate reports and merge
them into the configured Cobertura output.

## Interaction Coverage Matrix

| Inventory entry | `@e2e` scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| invoke `givn check review --change <id>` and inspect the generated semantic review report and exit status | Change review generates a complete report for a semantic candidate | CLI | `std::process::Command` starts the compiled `givn` binary with `check review --change running-change`; the scenario asserts CLI exit status and report evidence produced by that real invocation. |

The E2E scenario's primary assertions are the real CLI exit status and generated
report content. Filesystem reads are the observable report artifact produced by
that same CLI invocation, not a substitute for invoking the interface.

## Compatibility and Boundaries

The existing `givn spec review --change` advisory output remains unchanged.
Structural overlap findings, disposition parsing, coverage hooks, permanent
index refresh mechanics, feature-gated BGE/NLI behavior, remediation, and
archive merge behavior remain in their existing paths. This slice adds only the
E5-bound semantic report and its classification gate to `givn check review
--change`; the generated report is retained by the existing archive directory
move as historical evidence.

This is an intentional compatibility change for the default feature-free
binary: blocking `givn check review --change` now returns the existing retrieval
unavailable error instead of silently passing. The repository's dogfood binary
must therefore be installed with `--features spec-retrieval`; no-feature builds
remain a separately tested compatibility target and must fail explicitly rather
than claim semantic review completeness.
