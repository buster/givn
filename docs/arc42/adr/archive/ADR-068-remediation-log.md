---
status: superseded by ADR-069
date: 2026-09-03
decision-makers:
  - givn maintainers
consulted:
  - semantic-remediation-log proposal, design, and executable specs
informed:
  - maintainers operating change review, the archive gate, and the archived evaluation dataset
---

# Keep remediation evidence in an append-only log stamped into Git history

## Context

The semantic review report (`semantic-review.md`, ADR-046) is regenerated
deterministically on every run: prior states are overwritten, so the story of a
remediation — what E5, BGE, and NLI recommended, how long each stage ran, what
the scenarios looked like before and after, which decision led to which fix, and
which commits carried it — is destroyed. Only the final report and task-list
commit hashes survive archiving. The archived corpus therefore cannot serve as
labelled evaluation material for the semantic pipeline, and remediation commits
are not findable from Git history alone.

## Decision drivers

- [x] **Evaluation evidence** — every archived change should yield labelled
      records: model recommendation, human decision, fix, commits, cost.
- [x] **Determinism boundary** — the report's strict wire grammar and
      regeneration must stay intact; history cannot live there.
- [x] **Git findability** — remediation commits must be searchable with plain
      Git tools, independent of any givn artifact.
- [x] **No silent evidence** — the gate must refuse to pass while a resolved
      candidate lacks a complete, commit-stamped log entry.

## Considered options

1. **Extend the report schema with history fields**: destroys byte-for-byte
   regeneration and carry-forward determinism; per-run state has no home in a
   grammar that forbids unknown and environment fields.
2. **Mine Git history of regenerated reports**: the BLOCKED→PASS diffs encode
   before/after, but report versions are committed incidentally, the log would
   depend on commit discipline outside givn's control, and recommendation,
   rationale, and fix text are not reliably adjacent.
3. **Separate append-only remediation log with Git stamp validation** (chosen):
   the review run writes run records and candidate entries once; the agent fills
   decision and fix fields in place; the gate cross-validates entries against
   `semantic-remediation:`-stamped commits; the archive retains the log next to
   the report.

## Decision

`givn check review` maintains `givn/changes/<id>/semantic-remediation.md` as an
append-only log beside the report:

- one **run record** per executed review run that changes the candidate
  universe, anchored by the HEAD commit (`null` without Git history) and
  recording per-stage durations (E5 recall, BGE, NLI; `null` for a stage that
  did not run, never 0);
- one **candidate entry** per newly admitted candidate evidence state, holding
  the before scores and BGE/NLI evidence states, the verbatim scenario text of
  both endpoints with their content and evidence hashes, and the decision fields
  (classification, rationale, remediation text, commits) completed by the agent;
- **after-state fields** written by the passing rerun (end state, recomputed
  score where the pair was recomputed, after anchor), plus an after run record.

The review gate validates, after the existing projected-remediation
verification, that every resolved candidate has a complete entry — a non-empty
`remediation` text and at least one commit — with entries demanded only for
candidates the log knows (a verified remediation that predates the log passes
without one); when the project has Git history with at least one commit, each
recorded commit must be reachable from HEAD and carry the exact footer line
`semantic-remediation: <change-id>/<candidate-id>`. Anchors record full
40-character SHAs; an unborn branch behaves like no Git history. The archive
moves the log next to the report, and the archive similarity gate never reads
or grows the log. The report grammar stays untouched (ADR-046), and the
existing remediation verifier remains the only projected-result authority.

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

### Evidence for the mandatory dimensions

- **alternatives** — three materially different options are recorded above
  (report-schema extension, Git-history mining of regenerated reports, separate
  append-only log).
- **architectural_impact** — the decision fixes a persisted-evidence boundary:
  what the change directory and archive contain, that the report stays
  deterministic while the log is append-only, and that the review gate now
  depends on the log plus Git history rather than the report alone.
- **durable_consequence** — reversal would require migrating archived logs
  (they become the evaluation dataset), changing the review gate, and
  re-coordinating instructions across commands; implementation effort alone is
  not the reason.
- **lower_level_artifact** — the wire grammar, field semantics, and gate
  messages are owned by the Gherkin specs and design.md; what remains here is
  the boundary decision of *where remediation evidence lives* and *how it binds
  to Git*, which no single scenario or design section owns completely.
- **existing_adr_check** — ADR-046 owns the regenerated report, ADR-047 owns
  projected remediation verification, ADR-054 owns versioned evidence output;
  none owns a persistent remediation-evidence boundary. Route: `NEW_ADR`.

### MADR checklist evidence

- [x] **Hard to reverse** — archived logs become the labelled evaluation
      corpus; a format or ownership reversal would strand every archived change
      and re-split evidence across report and history.
- [x] **Shapes quality attributes** — it sets the floor for auditability and
      evaluability of the semantic pipeline: every remediation must leave a
      complete, cross-validated record.
- [x] **Structural** — it adds a persisted artifact beside the report, a
      gate→log→Git dependency direction, and a stamp convention shared by
      review, archive, and instructions.
- [x] **Must be shared** — without this record an implementer would extend the
      report schema or leave commits unstamped, breaking determinism or
      findability; supporting only, never a gate.

## Decision Outcome

Option 3. The log is written once per run, never regenerated; the report stays
regenerable; the gate enforces the stamp; the archive retains both artifacts.

## Consequences

- The change directory gains one more artifact; `givn archive` needs no code
  change because it already moves the whole change directory.
- Agents must fill `remediation` and `commits` in the log — a new review-step
  duty carried by the instructions and skills.
- Projects without Git history record `null` anchors and skip stamp checks; the
  log still records the full decision chain.
- Per-stage durations in the log are machine-relative facts of that run; they
  are written once and never recomputed (`null` only when a stage did not
  execute; a measured 0 is valid, including cache-only runs), so they stay
  valid as historical measurements, not reproducible constants.
- Evaluation aggregation over archived logs is future work; this decision only
  fixes the record format.

## Confirmation

The delta specs in `givn/changes/semantic-remediation-log/specs/` assert log
creation, append-only behaviour, wire grammar, gate blocking, stamp validation,
and archive retention; they run via `./run-tests.sh` (strict cucumber-rs,
`.fail_on_skipped()`), including two `@e2e` scenarios through the real binary.

## Pros and Cons of the Options

**Option 1 — report-schema extension**
- Good: one artifact to read.
- Bad: destroys regeneration determinism and carry-forward; unknown-field
  discipline forbids per-run state.

**Option 2 — Git-history mining**
- Good: no new artifact.
- Bad: depends on commit discipline givn does not control; recommendation,
  rationale, and fix text are not reliably captured; fragile evaluation tooling.

**Option 3 — append-only log with Git stamp (chosen)**
- Good: full labelled record, deterministic format, Git-searchable commits,
  gate-enforced completeness.
- Bad: one more artifact and one more agent duty per remediation.

## More Information

Glossary: `Remediation log`, `Remediation log entry`, `Review run record`,
`Git stamp`, `Verbatim scenario text`, `Append-only log`
(`docs/arc42/12-glossary.md`). Runtime and cross-cutting sections: chapter 6
(review flow) and chapter 8 (§8.5j).
