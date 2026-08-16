# Semantic Scenario Consolidation Plan

## Status

This document is the implementation sequence for extending Givn with
scenario-level semantic exploration and consolidation. It is committed to the
stack base so every subsequent PR inherits the same scope and ordering.

The existing experiment stack is:

```text
experiments/cross-encoder-reranker
  -> experiments/nli-experiment
  -> experiments/watn-corpus-comparison
```

The new implementation PRs continue on top of
`experiments/watn-corpus-comparison`. Each PR is one independent capability
slice. No later PR may silently pull an unfinished future slice into an earlier
one.

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
```

**Scope:**

- Keep all PR 1 behavior unchanged.
- Add active delta scenarios to the search set.
- Exclude `@givn.removed` scenarios.
- Handle `@givn.modified` identity without reporting a scenario against itself.
- Show whether a candidate comes from permanent specs or the current change.
- Keep the index permanent-only; delta vectors are computed for the request and
  are not persisted as permanent entries.
- Keep the output advisory and non-mutating.

**Out of scope:** projected archive semantics, classification, remediation,
review blocking, BGE, NLI, and coverage.

**Completion evidence:** A draft is found against a permanent twin and against
another active delta scenario; removed deltas are absent; the permanent index is
unchanged by the exploration.

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

- Generate candidates involving added and modified delta scenarios.
- Compare delta scenarios against permanent scenarios and against active delta
  scenarios.
- Use the existing E5 candidate generation and deterministic candidate identity.
- Record both endpoints, paths, titles, scores, scopes, and content hashes.
- Record the available deterministic facts without adding BGE or NLI yet.
- Require one of `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, `UNRELATED`, or
  `UNCERTAIN` for every candidate.
- Make missing or `UNCERTAIN` classifications fail the change review.
- Regenerate the file completely on every run. The report is intentionally a
  transient working document; the agent re-evaluates after each edit.
- Keep the existing structural overlap and coverage review contracts intact.

**Agent responsibility:** Givn generates evidence and the report. The agent
edits the current change. Givn does not rewrite or delete feature files itself.

**Completion evidence:** A change with a semantic candidate receives a complete
report; an unclassified candidate exits non-zero; a fully classified report
exits zero while still leaving the specs untouched.

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

### PR 5: Read-Only Permanent Duplicate Audit

**Branch:** `semantic/05-spec-duplicates`

**Base:** `semantic/04-semantic-remediation`

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

**Completion evidence:** The command scans the full permanent corpus, reports
all candidates beyond the interactive top-five view, and leaves the project
byte-for-byte unchanged.

## Model Evidence Layers

### PR 6: BGE Reranking Evidence

**Branch:** `semantic/06-bge-evidence`

**Base:** `semantic/05-spec-duplicates`

**Enabler:** Semantic candidates can be prioritized with a jointly scored local
relevance model.

**Scope:**

- Add the local BGE cross-encoder as a second-stage evidence layer.
- Score both pair directions and report the symmetric raw score.
- Include model identity, score, and runtime in explore/review output.
- Use it to prioritize agent review, not to perform automatic merge/delete.
- Keep the current E5 candidate generator and deterministic facts available.
- Record a benchmark against the combined Givn/Watn corpus.

**Out of scope:** treating the BGE raw logit as a Givn class probability,
automatic deletion, and a new hard threshold without labeled evidence.

### PR 7: NLI Evidence

**Branch:** `semantic/07-nli-evidence`

**Base:** `semantic/06-bge-evidence`

**Enabler:** Candidates expose bidirectional entailment, neutral, and
contradiction evidence in addition to similarity and relevance.

**Scope:**

- Add the local multilingual NLI model behind the retrieval feature.
- Score `(A, B)` and `(B, A)`.
- Report directional probabilities and the symmetric summary.
- Preserve the distinction between NLI labels and Givn classifications.
- Keep NLI diagnostic and non-authoritative.
- Record model artifact, tokenizer, hashes, runtime, and benchmark results.

**Out of scope:** training a Gherkin-specific classifier, mapping NLI labels
directly to `DUPLIKAT`/`VARIANTE`/`VALID-BOUNDARY`, and automatic gates.

## Full Audit In Normal Review

### PR 8: Permanent Audit in Normal Review

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

Train or calibrate a classifier for `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, and
`UNRELATED` using adjudicated Givn and Watn pairs. No off-the-shelf BGE or NLI
score may be treated as this classifier.

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
