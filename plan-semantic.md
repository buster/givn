# Semantic Scenario Consolidation Plan

## Status

This document is the implementation sequence for extending Givn with
scenario-level semantic exploration and consolidation. It is anchored on
`main`, the root base branch of the complete stacked PR series, so every
subsequent PR inherits the same scope and ordering. While the stack is open, the
latest plan snapshot is carried by the current stack tip so downstream PRs
inherit it. It reaches `main` when the stack is merged in order; do not update
intermediate base branches separately.

The current stack has seventeen open PRs. PR 22 is the seventeenth and has
sixteen stack predecessors. Superseded closed PRs are not counted.

| Stack order | PR | Head branch | Base branch |
|---:|---:|---|---|
| 1 | #1 | `spec-refactoring` | `main` |
| 2 | #5 | `spec-overlap-gates` | `spec-refactoring` |
| 3 | #7 | `workflow-enforcement-wip-cleanup` | `spec-overlap-gates` |
| 4 | #8 | `prose-consolidation` | `workflow-enforcement-wip-cleanup` |
| 5 | #9 | `skill-rewrites` | `prose-consolidation` |
| 6 | #10 | `spec-retrieval` | `skill-rewrites` |
| 7 | #11 | `repository-wide-ownership-e2e` | `spec-retrieval` |
| 8 | #13 | `experiments/cross-encoder-reranker` | `repository-wide-ownership-e2e` |
| 9 | #14 | `experiments/nli-experiment` | `experiments/cross-encoder-reranker` |
| 10 | #15 | `experiments/watn-corpus-comparison` | `experiments/nli-experiment` |
| 11 | #16 | `semantic/01-spec-explore-permanent` | `experiments/watn-corpus-comparison` |
| 12 | #17 | `semantic/02-spec-explore-change` | `semantic/01-spec-explore-permanent` |
| 13 | #18 | `semantic/03-semantic-review` | `semantic/02-spec-explore-change` |
| 14 | #19 | `semantic/04-semantic-remediation` | `semantic/03-semantic-review` |
| 15 | #20 | `semantic/05-instruction-integrity` | `semantic/04-semantic-remediation` |
| 16 | #21 | `semantic/06-spec-duplicates` | `semantic/05-instruction-integrity` |
| 17 | #22 | `semantic/07-bge-evidence` | `semantic/06-spec-duplicates` |

### Current Delivery State

The GitHub Stacks CLI stack is stack #6 and contains the seventeen open PRs
listed above. Every PR is still a Draft and GitHub reports every PR as
mergeable. PR #21 is the spec-duplicates stack link on
`semantic/06-spec-duplicates`; PR #22 is the BGE stack tip on
`semantic/07-bge-evidence`. The BGE change is archived locally, its full suite,
E2E suite, review, coverage, and archive gates are green, and the branch is
ready for the PR update. GitHub currently reports no status checks for this
branch, so that verification remains manual.

The GitHub stack metadata was synchronized with `gh stack link` after PR #22
was added. This plan snapshot is carried on the PR #22 tip. `main` and the
intermediate base branches intentionally retain earlier snapshots until the
stack is merged in order.

The existing experiment stack is:

```text
experiments/cross-encoder-reranker
  -> experiments/nli-experiment
  -> experiments/watn-corpus-comparison
```

The semantic implementation PRs and the instruction-integrity maintenance PR
are the final five links currently in this stack. They continue on top of
`experiments/watn-corpus-comparison`, which is itself inherited from the
fourteen earlier stack links. Each PR is one independent capability slice. No
later PR may silently pull an unfinished future slice into an earlier one.

## Product Boundary

Givn helps an agent discover and consolidate similar Gherkin scenarios. Givn
does not make an unvalidated model score an automatic delete decision. The
agent changes `.feature` files in the current change; Givn produces evidence and
verifies that the requested remediation was actually applied.

The first implementation path is advisory exploration followed by a blocking
completeness check for unresolved review candidates:

```mermaid
flowchart TD
    A[One complete Gherkin scenario] --> B[spec explore]
    B --> C[Permanent candidates]
    C --> D[Change-aware explore]
    D --> E[semantic-review.md]
    E --> F[Agent classifies and edits the delta]
    F --> G[Givn verifies projected state]
    G --> H[Review and archive]
```

## PR Sequence

### PR 1: Explore Permanent Specs

**Branch:** `semantic/01-spec-explore-permanent`

**Enabler:** An agent can check a scenario before creating it.

**User interface:**

```text
givn spec explore --file <draft.feature>
```

**Scope:**

- Accept exactly one complete Gherkin scenario.
- Reject free text and malformed or multi-scenario input.
- Refresh the permanent E5 index incrementally before searching.
- Search all permanent scenarios, including scenarios in the same feature file.
- Exclude only the exact query identity when the query comes from an existing
  feature file.
- Return the top candidates in human-readable text.
- Return the same result as stable machine-readable JSON with `--format json`.
- Include candidate title, file, rank, score, model, and index metadata.
- Remain advisory and never modify specs or block the workflow.

**Out of scope:** current changes, BGE, NLI, deterministic fact explanation,
coverage, automatic edits, and review gating.

**Completion evidence:** A real CLI exploration finds a known permanent
near-duplicate from both `--file` and `--stdin`; malformed input fails clearly;
the JSON output is stable enough for an agent to consume.

### PR 2: Explore the Current Change

**Branch:** `semantic/02-spec-explore-change`

**Base:** `semantic/01-spec-explore-permanent`

**Enabler:** An agent can compare a draft against both the permanent baseline and
the scenarios already being written in the current change.

**User interface:**

```text
givn spec explore --change <id> --file <draft.feature>
givn spec explore --change <id> --stdin
```

**Scope:**

- Preserve PR 1 input validation, permanent candidate set, ranking, and existing
  output fields/order. Output metadata is intentionally additive rather than
  byte-for-byte PR 1 compatibility: JSON results gain scope/owning-change
  metadata for both no-change and change-aware calls, and text results identify
  their source.
- Accept both file and standard-input queries with an optional current-change
  scope.
- Cover `--change --stdin` as regular behavior; it does not create another
  interaction-inventory entry or `@e2e` scenario.
- Add active delta scenarios to the search set.
- Exclude `@givn.removed` scenarios.
- Handle `@givn.modified` identity without reporting a scenario against itself.
- Normalize permanent, delta-source, projected-target, and query identities with
  one project-relative forward-slash path rule for relative and absolute inputs.
- Show whether a candidate comes from permanent specs or the current change.
- Keep the index permanent-only; delta vectors are computed for the request and
  are not persisted as permanent entries.
- Keep the output advisory and non-mutating.
- Reject unsafe identifiers, missing changes, missing change specs directories,
  and malformed delta features with clear non-zero errors before refreshing the
  permanent index.

**Out of scope:** projected archive semantics, classification, remediation,
review blocking, BGE, NLI, and coverage.

**Completion evidence:** A draft is found against a permanent twin and against
another active delta scenario; removed deltas are absent; added and modified
queries exclude their own source and projected-target identities while another
eligible candidate remains visible; file and `--stdin` change-aware queries are
covered as regular scenarios; unsafe, missing, incomplete, and malformed change
requests fail clearly without mutating the permanent index where observable.

### PR 3: Generate the Semantic Review Report

**Branch:** `semantic/03-semantic-review`

**Base:** `semantic/02-spec-explore-change`

**Enabler:** A change review presents all relevant semantic candidates as an
explicit agent worklist.

**Output:**

```text
givn/changes/<id>/semantic-review.md
```

**Scope:**

- Generate the report only from blocking `givn check review --change <id>`;
  preserve advisory `givn spec review --change <id>` behavior.
- Include active Added/Modified delta-to-permanent and delta-to-delta candidates
  whose E5 cosine score is at least the binary-fixed `0.9500` boundary. Exclude
  Removed and permanent-permanent pairs; a Modified scenario excludes only its
  own projected target.
- Record both endpoints, paths, titles, scopes, scores, complete source-ordered
  evidence, content/evidence hashes, and deterministic facts without BGE/NLI.
- Require one of `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN` for
  every candidate, with an agent-authored rationale for every resolved value.
  A candidate that appears unrelated above the boundary remains unresolved
  until its vocabulary and invariant are sharpened or it receives an allowed
  classification.
- Make missing, invalid, or `UNCERTAIN` classifications fail the change review.
- Regenerate the complete change-local report atomically on every run, carry
  classifications only across unchanged full evidence, and retain the report
  as historical evidence when the existing archive move runs.
- Treat a malformed permanent index as stale, rebuild it from permanent specs,
  validate its permanent-only structure, and continue semantic review.
- Keep the existing structural overlap and coverage review contracts intact.

**Agent responsibility:** Givn generates evidence and the report. The agent
edits the current change. Givn does not rewrite or delete feature files itself.

**Completion evidence:** A change with a semantic candidate receives a complete
report; an unclassified candidate exits non-zero; a fully classified report
exits zero while leaving feature files, permanent specs, and dispositions
untouched; a seemingly unrelated high-score candidate cannot pass without
vocabulary sharpening or an allowed classification.

### PR 4: Verify Semantic Remediation

**Branch:** `semantic/04-semantic-remediation`

**Base:** `semantic/03-semantic-review`

**Enabler:** A classification cannot be used as a substitute for the required
  change to the specs.

**Scope:**

- Project the current permanent tree with the active delta operations.
- For `DUPLIKAT`, verify that the redundant scenario is actually removed.
- For `VARIANTE`, verify that the scenarios are actually consolidated or
  parameterized.
- For `VALID-BOUNDARY`, verify that both scenarios remain and their titles/steps
  make the distinction explicit.
- Re-run the semantic comparison after remediation.
- Require a valid `VALID-BOUNDARY` pair to fall below the current binary
  similarity boundary; keeping a boundary is not an exception to the boundary.
- Invalidate the report when a referenced scenario content hash changes and
  require a fresh transient report.

**Out of scope:** full permanent-permanent auditing, BGE, NLI, Cobertura, and
automatic feature-file edits.

**Completion evidence:** Marking a candidate `DUPLIKAT` or `VARIANTE` without
changing the projected tree remains red; an actual consolidation becomes green;
a retained boundary that remains above the boundary remains red.

### PR 5: Instruction and Skill Guidance Integrity

**Branch:** `semantic/05-instruction-integrity`

**Base:** `semantic/04-semantic-remediation`

**Enabler:** The workflow's canonical instructions, generated skills, review
guidance, and machine-readable instruction contract are complete and
consistent before the remaining semantic slices continue.

**Scope:**

- Restore complete real-interface, browser-driver, and interface-obstacle
  policy in the canonical specs instruction.
- Restore design-owned E2E setup fields and remove conflicting lower-level
  interface examples.
- Document class-specific semantic-remediation evidence in generic review
  guidance and templates.
- Make retrieval guidance feature-aware for advisory, blocking, and unavailable
  modes.
- Preserve project instruction overrides through skill synchronization.
- Emit the documented `givn instructions --json` contract
  `{id, generates, requires, instruction}` and reject unsafe change ids.
- Synchronize `.opencode`, `.agents`, `.claude`, and harness guidance.

**Out of scope:** semantic scoring, projected remediation algorithms, project
override precedence, and lifecycle order.

**Completion evidence:** Canonical policy, design, review, retrieval, override,
and JSON scenarios pass through the real CLI; the full repository and E2E
suites pass; the change is archived with the review report and Arc42 updates.

### PR 6: Read-Only Permanent Duplicate Audit

**Branch:** `semantic/06-spec-duplicates`

**Base:** `semantic/05-instruction-integrity`

**Enabler:** A maintainer can inspect the entire permanent corpus independently
of a particular change.

**User interface:**

```text
givn spec duplicates
```

**Scope:**

- Compare the complete permanent scenario tree against itself.
- Include same-feature and cross-feature pairs.
- Report exact duplicates, structural overlap, and semantic candidates.
- Return deterministic ordering and machine-readable JSON.
- Remain read-only and advisory.
- Do not write a change, modify permanent specs, or block an ordinary change.
- Point maintainers to a new change as the remediation location.
- Keep E5 as the candidate generator. BGE and NLI are not part of this PR's
  implementation, but the later BGE/NLI layers MUST integrate with this
  permanent-permanent search surface rather than only with change review.

**Completion evidence:** The command scans the full permanent corpus, reports
all candidates beyond the interactive top-five view, and leaves the project
byte-for-byte unchanged.

### Next Steps

1. Keep PR #21, `semantic/06-spec-duplicates`, as the independent read-only
   permanent duplicate-audit stack link on PR #20.
2. Branch `semantic/07-bge-evidence` directly from PR #21 and keep all BGE
   implementation, approval, NLI-isolation, coverage, E2E, review, and archive
   commits on this separate PR.
3. Synchronize GitHub stack #6 so PR #22 is linked immediately above PR #21.
4. Continue with `semantic/07-nli-evidence` only after the BGE PR is green
   enough to serve as its base.

## Model Evidence Layers

### PR 7: BGE Reranking Evidence

**Branch:** `semantic/07-bge-evidence`

**Experiment reference:** The existing comparison implementation is kept on
`experiments/cross-encoder-reranker` (GitHub PR #13). That branch is evidence for
this later layer; it is not active in the PR 1 product build unless the explicit
`cross-encoder-reranker` Cargo feature is enabled.

**Base:** `semantic/06-spec-duplicates`

**Enabler:** Semantic candidates can be prioritized with a jointly scored local
relevance model.

**Scope:**

- Add the local BGE cross-encoder as a second-stage evidence layer.
- Score both pair directions and report the symmetric raw score.
- Include model identity, score, and runtime in explore/review output.
- Use it to prioritize agent review, not to perform automatic merge/delete.
- Keep the current E5 candidate generator and deterministic facts available.
- Include BGE in every E5-backed search surface: permanent duplicate audit,
  permanent exploration, change-aware exploration, advisory change review, and
  blocking semantic review.
- Keep BGE behind the existing `cross-encoder-reranker` feature flag. The
  default E5-only path must remain available for comparison and fallback.
- Run BGE only on a deterministic filtered top-K subset produced by E5 and
  deterministic exclusions. For permanent-permanent audit, use the union of
  each endpoint's top-K neighbours; for change searches, use the union of the
  top-K permanent and active-change neighbours for each query. Never run BGE
  over the complete pairwise corpus.
- Record top-K recall, candidate counts, BGE CPU time, and the effect of the
  flag combination on precision and false-positive rate.
- Record a benchmark against the combined Givn/Watn corpus.

**Out of scope:** treating the BGE raw logit as a Givn class probability,
automatic deletion, and a new hard threshold without labeled evidence.

### PR 8: NLI Evidence

**Branch:** `semantic/07-nli-evidence`

**Base:** `semantic/07-bge-evidence`

**Enabler:** Candidates expose bidirectional entailment, neutral, and
contradiction evidence in addition to similarity and relevance.

**Scope:**

- Add the local multilingual NLI model behind the retrieval feature.
- Score `(A, B)` and `(B, A)`.
- Report directional probabilities and the symmetric summary.
- Preserve the distinction between NLI labels and Givn classifications.
- Keep NLI diagnostic and non-authoritative.
- Include NLI in every E5-backed search surface covered by PR 7: duplicate
  audit and all permanent/current-change search and review paths.
- Put NLI behind an independent `nli-evidence` feature flag so E5-only,
  E5+BGE, E5+NLI, and E5+BGE+NLI runs can be compared independently.
- Reuse the same filtered E5 top-K union as BGE. NLI must not expand the
  candidate set or run over the full corpus; it is a second-stage signal on the
  bounded subset only.
- Record NLI CPU time, directional/symmetric outputs, top-K recall, and
  precision/false-positive changes for each flag combination.
- Record model artifact, tokenizer, hashes, runtime, and benchmark results.

**Out of scope:** training a Gherkin-specific classifier, mapping NLI labels
directly to `DUPLIKAT`/`VARIANTE`/`VALID-BOUNDARY`, and automatic gates.

### Cross-Cutting Integration Requirement: BGE and NLI

PR 7 and PR 8 are not complete when the models work only in one review path.
Both layers must be available in every semantic search that starts from E5
embeddings:

| Search surface | Candidate generator | BGE/NLI scope |
|---|---|---|
| `givn spec explore` permanent search | E5 top-K per query | Filtered top-K union only |
| `givn spec explore --change` | E5 top-K per query across permanent and active candidates | Filtered top-K union only |
| `givn spec review --change` / blocking change review | E5 top-K for delta-to-permanent and delta-to-delta pairs | Filtered top-K union only |
| `givn spec duplicates` | E5 top-K per permanent endpoint | Filtered permanent-permanent top-K union only |

The two feature flags are independent and must expose the four measurable
configurations E5-only, E5+BGE, E5+NLI, and E5+BGE+NLI. Disabled models must
not change the E5 candidate set. Missing model artifacts must produce an
explicit unavailable-model result, not silently fall back while claiming that
the model participated. The top-K value and filtering rules must be fixed or
explicitly reviewed; an unbounded all-pairs BGE/NLI pass is not acceptable on
CPU.

### Risk Hypothesis: False Positives and Verbosity Gaming

The current E5-only behavior has a specific product risk: false positives may
be frequent enough that an LLM learns to write scenario text that is unusually
long and overly specific merely to push the similarity score below `0.9500`.
Conversely, a candidate above `0.9500` may describe a real duplicate or variant
even when there is no safe way to reduce the score without distorting the
scenario. A low score is therefore not evidence that the scenario is well
designed, and a high score is not by itself proof that it should be deleted.

This hypothesis must be measured rather than handled by lowering the boundary:

- Label a representative candidate sample as duplicate, variant, valid
  boundary, or unrelated and compare E5-only, E5+BGE, E5+NLI, and combined
  precision, recall, false-positive, and false-negative rates.
- Measure scenario character count, step count, assertion count, and semantic
  scores before and after authoring guidance changes. Look specifically for a
  correlation between verbosity and score reduction.
- Evaluate candidate selection quality with top-K recall before BGE/NLI and
  after each second-stage layer. A reranker that improves precision but drops
  true candidates out of the E5 subset is not an improvement.
- Add an authoring constraint or LLM evaluation that penalizes unnecessary
  length, duplicated setup, overloaded steps, and irrelevant qualifiers. The
  agent must sharpen the invariant and boundary, not add prose as a scoring
  workaround.
- Preserve the candidate above `0.9500` as review evidence until the labeled
  models and deterministic facts provide a better explanation; do not let the
  LLM evade review by making scenarios artificially verbose.

The exit criterion for this risk is a documented model comparison with a
bounded CPU runtime and an authoring evaluation showing that concise scenarios
are not systematically penalized relative to verbose score-avoidance text.

## Full Audit In Normal Review

### PR 9: Permanent Audit in Normal Review

**Branch:** `semantic/08-permanent-audit-review`

**Base:** `semantic/07-nli-evidence`

**Enabler:** A normal change review also verifies the complete permanent corpus,
not only pairs involving the current delta.

**Scope:**

- Extend the normal semantic review to include permanent-permanent pairs.
- Continue to include current delta versus permanent and delta versus delta.
- Put all candidates into the transient `semantic-review.md` report.
- Require classifications for every reported candidate.
- Require actual remediation for `DUPLIKAT` and `VARIANTE`.
- Require rewritten `VALID-BOUNDARY` scenarios to fall below the current
  similarity boundary.
- Run a fresh complete baseline over Givn's own permanent corpus before the
  gate is considered green.
- Do not grandfather existing candidates through hidden dispositions.
- Require the integrated, independently feature-flagged BGE/NLI layers from
  PR 7/8 on the same bounded E5 top-K candidate subset for every search surface
  before declaring the normal-review baseline clean.

**Dogfood prerequisite:** The current permanent baseline must be measured and
resolved before this gate can pass. Existing candidates cannot be silently
accepted merely because they predate the audit.

**Completion evidence:** A normal `givn spec review --change <id>` proves both
the change-local and permanent-permanent scopes, and a clean Givn baseline is
recorded.

## Future Optional Experiments

These are intentionally excluded from PRs 1 through 8:

### Scenario-Level Cobertura Branch Attribution

Documented in `docs/future-coverage-experiment.md`. It would compare permanent
and projected branch sets and, where supported, calculate leave-one-out marginal
coverage per scenario. It is not a redundancy proof and requires stable branch
identity plus isolated scenario execution.

### Mutation-Based Scenario Value

Measure whether an individual scenario kills mutations that other scenarios do
not. This may provide stronger evidence than branch coverage but has unknown
runtime and tooling cost.

### Gherkin-Specific Pair Classifier

Train or calibrate a classifier for `DUPLIKAT`, `VARIANTE`, and
`VALID-BOUNDARY` using adjudicated Givn and Watn pairs. A candidate that appears
unrelated above the active boundary is not a fourth class: its vocabulary or
invariant must be clarified. No off-the-shelf BGE or NLI score may be treated as
this classifier.

### LLM-Assisted Adjudication

Use an LLM to propose a classification and rationale for the small candidate
set, while keeping the final decision and remediation verification explicit.
The LLM is not a deterministic gate by itself.

### Step-Definition and Runtime Trace Evidence

Resolve Gherkin steps to implementation bindings or runtime traces as an
additional boundary signal. This is project-specific and outside the initial
generic Givn feature.

## Per-PR Workflow

Every PR in this sequence is a formal Givn change and follows the normal
proposal/spec/design/design-review/tasks/implementation/review workflow. The
experiment PRs above remain experiment evidence and are not retroactively
converted into product changes.

For each implementation PR:

1. Branch from the preceding stack head.
2. Create one focused Givn change for that slice.
3. Add executable scenarios for the observable CLI behavior.
4. Implement only that slice.
5. Run the full required checks.
6. Commit and push the branch.
7. Synchronize the GitHub stack before starting the next slice.

No implementation PR should begin until the preceding PR is green enough to
serve as its base. The current task stops after documenting this plan; no
production extension is started here.
