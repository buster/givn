# Proposal: review-friction-hardening

## Release note

Make `givn check review` report every use-case schema and capability-routing problem in one pass, and let `givn archive` reuse a green change proof when only documentation changed after it was recorded.

## Use-Case Context

- Use-case ID: `verify-change`, `archive-change`
- Ideation topic: none — the change comes from the clean B3/C2 harness comparison, not an ideation seed.
- Seed path: none
- Confirmed Personas:
  - `developer--primary-user` — the developer who runs review and archive and pays for each avoidable iteration.
  - `agent--technical-role` — the authoring agent that reads the diagnostics and the verification receipt as a machine contract.
  - `givn-maintainer--owner` — the owner of the permanent specifications, the receipt contract, and the archive proof.
- Source evidence: `/home/buster/projects/givn-harness/docs/findings/2026-10-20-clean-comparison.md`, sections "The use-case parser still reports one schema error at a time" and "The stale-receipt retry remains".

## Problem / Opportunity

Two defects remain after the clean B3/C2 comparison. Both are avoidable review or archive friction: the underlying gate is correct, but the developer pays for the gate one error or one retry at a time.

### Problem 1 — review reveals use-case schema and routing errors one at a time

The candidate run spent six review iterations on successive use-case parser and
capability-routing errors. Each repair exposed the next error in the same
family:

1. `# Use Case: lending-register` — the parser expects `# Use case:`.
2. `## Guarantees` — the parser does not recognise that heading.
3. `## Capabilities` held a prose line instead of one capability per bullet.
4. `## Interactions` named `register-readiness`, which `## Capabilities` did not declare.
5. `## Examples` held prose instead of one example per bullet.
6. The next schema or routing defect appeared only after those were fixed.

The developer cannot fix the contract in one edit because `givn check review`
stops at the first diagnostic. The gate is strict, but it is not informative:
it withholds the rest of the diagnosis.

The parser also repeats the same document-level error once per delta
capability. In the observed run, one `usecase.md` parse failure was printed
five times because five capabilities referenced the same document.

### Problem 2 — archive refuses a green proof after documentation-only review edits

The candidate initial and follow-up stages each had one archive failure. In
both cases the change verification receipt was green, the feature digest and
the verify command digest were unchanged, and only the receipt's recorded head
differed from the current head. The archive refused, the agent reran
`givn check tasks`, and the archive then succeeded in the same stage.

The exact observed refusal:

```text
error: archive blocked: the change verification receipt is stale for 'lending-register'. receipt head 08ec128de971303d3545b125bdbbe339499f23fe, current head d5ec60de47a1e41b69d3058fe0b34d1323d4a55b; receipt features 5256de975f66cbcd52f33f6cdc0c577062afa3667181c6171171dec9f6c8c813, current features 5256de975f66cbcd52f33f6cdc0c577062afa3667181c6171171dec9f6c8c813; receipt command 1031237dac4a7328aa009dc5586392005a11f5408a7dc7b8a4726efc55a93a8e, current command 1031237dac4a7328aa009dc5586392005a11f5408a7dc7b8a4726efc55a93a8e. Run `givn check tasks --change lending-register` again.
```

Re-running the change suite costs a full test run and buys no new evidence
when the tested content is byte-for-byte unchanged. The current receipt binds
the revision but not the tested content, so a commit that touches only review,
proposal, design, tasks, arc42, evidence, or the receipt itself invalidates a
proof that still holds.

## Proposed Solution

### 1. One complete review diagnosis for use-case schema and capability routing

`givn check review` reports the complete set of use-case schema and
capability-routing problems in one pass. The developer sees every offending
line, every undeclared capability, every missing section, and every malformed
list or table entry together, so one edit can repair the whole contract.

- Diagnostics are grouped by document and ordered by line.
- Each diagnostic names the line, the violated section rule, and the
  correction where one is unambiguous.
- The same document-level problem appears once, not once per capability that
  references the document.
- Every active delta use-case or fragment narrative is validated too, even
  when the routing decision extends an existing capability, so a replacement
  narrative cannot hide a schema error until archive.
- The schema stays strict. `# Use Case` is still refused, unknown headings are
  still refused, and prose in a bullet list or an interaction that names an
  undeclared capability is still refused. The change adds complete reporting,
  not permissiveness.
- A near-miss title or heading is refused with the exact accepted spelling, so
  the strict rule costs one correction rather than another review cycle.

The gate order is preserved. Integrity and overlap still run before routing,
because the existing `verify-change` contract says structural failures outrank
other findings. The batching is within the use-case schema and routing family,
which is the family the evidence exposed.

### 2. Reuse a green change proof across documentation-only revisions

A change verification receipt records the tested content as well as the tested
revision, the delta features, and the verify command. `givn archive` and
`givn check review` accept that receipt when the tested content is unchanged,
even if the current revision differs because documentation or evidence was
committed after the proof.

- Archive reuses the proof and does not rerun the change suite.
- Archive still runs the merged regular and e2e suites exactly once, as it
  does today. No extra test invocation is added.
- Archive still refuses when source code, tests, configuration, the delta
  features, the verify command, or any other tested content changed. A stale
  refusal names the changed boundary and routes to
  `givn check tasks --change <id>`.
- The reuse is bound to the receipt: the receipt carries a digest of the
  tested content and a versioned scope name. An old receipt without that
  binding keeps the current strict revision equality and requires one rerun;
  it is never silently treated as reusable.
- A revision that rewrites history so the receipt's tested revision is no
  longer an ancestor of the current revision is refused.

The docs-only boundary is precise and closed. It covers:
- the active change's top-level Markdown artifacts: `proposal.md`,
  `design.md`, `design-review.md`, `tasks.md`, `review.md`, `arc42.md`,
  `HANDOFF.md`, and any other `*.md` directly in the change directory;
- the active change's `evidence/**`, `verification/**`, `results/**`, and
  `verification.json`;
- use-case and fragment narratives (`usecase.md`, `fragment.md`) under the
  active change's `specs/**` and under `givn/specs/**`;
- `givn/personas/**`, `givn/archive/**`, and `givn/history/**`;
- `docs/arc42/**` and `README.md`.

Everything else is tested content. In particular, `src/**`, `tests/**`,
`assets/**`, `Cargo.toml`, `Cargo.lock`, `run-tests.sh`, the measurement
scripts, `givn/commands.yaml`, `givn/config.yaml`, and every `*.feature` file in the
active change or `givn/specs/**` are never documentation-only.

### Preserved gates

The terminal interface check is unchanged. A terminal change still requires
`VISUAL-REVIEW: PASS` and committed transcript evidence under the change's
`evidence/visual/<scenario-slug>/transcript.txt`; this change adds no
`VISUAL-REVIEW: NOT APPLICABLE` path for terminal.

The Persona gate is unchanged. A terminal change still requires a confirmed
Persona file and a `PERSONAS: <persona>` line; this change adds no
`PERSONAS: none` escape hatch.

## Capability Routing

`givn spec route --change review-friction-hardening` recommends
`EXTEND givn-workflow-enforcement` from use case `verify-change` with score
35.00 and confidence `signal` (re-run after this proposal was drafted).

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` (top ranked, score 35.00) | `EXTEND givn-workflow-enforcement` | The capability owns the mandatory `givn check review` checks and the change verification receipt. Batched schema/routing diagnostics and review-side proof reuse are review-gate behavior. |
| `archive-proof` | Not returned in the ranked candidates; the top candidate was `givn-workflow-enforcement`, and the archive-change candidates returned were `enforce-post-merge-e2e-gate`, `extract-coverage-gate`, and `grouped-merge-for-new-capabilities`. | `EXTEND archive-proof` | The refused operation is archive, and the receipt validation is the archive gate's proof precondition. The owning use case is `archive-change`; route ranked proposal wording before the archive scenarios existed. Add no new capability because `archive-proof` already owns "archive a completed change" and "repair a blocked archive". |

No new use-case directory or capability is introduced. The owning use-case
documents are updated as delta narratives because their rules and interaction
inventories change: `specs/verify-change/usecase.md` and
`specs/archive-change/usecase.md`.

## Out of Scope

- Renaming the visual-review gate, verdict, or evidence directory for terminal
  interfaces. The labels stay `VISUAL-REVIEW` and `evidence/visual/`.
- `VISUAL-REVIEW: NOT APPLICABLE` for a terminal interface.
- A `PERSONAS: none` escape hatch or any change to Persona validation.
- Inventing a Persona or promoting a new one.
- Accepting `# Use Case`, case-insensitive section headings, or any other
  schema variant. Strictness stays.
- Reporting unrelated review gates (integrity, overlap, retirement, receipt)
  in the same batch as routing diagnostics. Gate order and structural
  precedence stay.
- Turning the existing advisory for a completely missing or entirely malformed
  routing table into a blocking gate. This change batches diagnostics when
  routing is evaluated.
- Rerunning the change suite at archive, adding a second merged-suite run, or
  caching test results beyond the receipt.
- Changing the archive's candidate revision, finalization revision, coverage
  gate, or merged-suite count.
- Changing Git/Fossil behavior beyond the shared tested-content digest.
- Changing the benchmark, the harness, or running a paid benchmark.
- Performance work beyond removing the avoidable review and archive
  iterations.

## Open Questions

### D1 — Strict schema or accepted heading variants?

- Question: Should `givn check review` keep the existing exact heading and
  title spelling, or accept common variants such as `# Use Case` and
  case-insensitive section headings, while batching diagnostics?
- Options:
  1. **Strict schema with batched diagnostics.** `# Use case:` and the exact
     headings remain the only accepted forms; every error is reported in one
     pass, and a near miss suggests the exact spelling. Gain: one canonical
     contract, no silent drift, no second accepted dialect in specs and
     archive. Cost: the first `# Use Case` still fails, but the developer sees
     the correction and every other problem in the same run. In practice:
     `# Use Case: lending-register` produces one diagnostic with
     `did you mean '# Use case: lending-register'?` alongside the unknown
     heading and list errors.
  2. **Accept common variants.** Treat `# Use Case` and case-insensitive
     headings as valid. Gain: fewer title/heading failures. Cost: two accepted
     dialects, every consumer and fixture must agree on which forms are valid,
     and typos such as `## Guarantees` still need a decision. In practice:
     `# Use Case:` passes while `## Guarantees` remains unknown, so the
     contract is only partly permissive.
  3. **Accept only a case-insensitive title, strict headings.** Gain: fixes the
     most common typo. Cost: an inconsistent rule that is harder to document
     and teach than either full strictness or full case-insensitivity. In
     practice: `# Use Case:` passes but `## guarantees` fails.
- Disposition: answered — the user constraint says "Do not weaken semantic
  validation; the primary requirement is complete diagnostics, not
  permissiveness", and the harness finding identifies iteration count, not
  strictness, as the defect. Option 1 is chosen and realized in `design.md`
  (D1).

### D2 — What exactly counts as documentation-only?

- Question: Which changed paths may follow a green change proof without
  invalidating it, and which paths must still force a new change-suite run?
- Options:
  1. **Closed allowlist of change docs, arc42, README, evidence, and spec
     narratives.** Only the explicitly named paths are docs-only; everything
     else, including configuration and permanent features, is tested content.
     Gain: a small, reviewable boundary; a code or configuration change can
     never hide behind a docs commit. Cost: some doc-like files outside the
     allowlist (for example `CHANGELOG.md` or `docs/ideation/**`) still stale
     the proof. In practice: a review-only commit to `review.md`,
     `usecase.md`, and `docs/arc42/06-runtime-view.md` reuses the proof; a
     commit to `src/proof.rs` does not.
  2. **All Markdown files are documentation.** Any `*.md` change is docs-only.
     Gain: the smallest friction surface. Cost: tests or tooling that parse
     Markdown can change behavior without a retest, and a release note or
     instruction asset can silently alter the suite. In practice: editing
     `assets/instructions/review.md` would reuse a proof even though it can
     change tested output.
  3. **Classify the SCM diff from the receipt revision as docs-only.** Gain: no
     new content digest. Cost: the decision depends on history rather than on
     the receipt, so a history rewrite or a different backend changes the
     answer; the receipt itself does not bind the tested content. In practice:
     the same tree can be reusable on one backend and stale on another.
- Disposition: carried — the exact boundary is an architecture decision; owner
  is the design; deciding artifact is `design.md` (D2).

### D3 — How is the reuse bound to the receipt?

- Question: What does the receipt record so archive can distinguish a
  documentation-only revision from a tested-content change without rerunning
  the suite?
- Options:
  1. **Tested-content digest plus a versioned scope name in the receipt.**
     `givn check tasks` hashes the tracked tested content, records the digest
     and `change-proof/v1`, and archive recomputes the digest before reuse.
     Gain: the receipt itself proves what was tested; the check works on both
     SCMs and survives a docs-only revision. Cost: a new receipt field and a
     tracked-file listing through the SCM seam. In practice: the C2 receipt
     gains `tested_content_digest`; committing the receipt changes the head
     but not the digest, so archive reuses it.
  2. **Use the SCM diff between the receipt head and the current head.**
     Gain: no new receipt field. Cost: archive must trust history, not the
     receipt; renames, history rewrites, and the Fossil/Git difference become
     part of the gate. In practice: a rebased documentation commit has no
     reachable receipt head, so the proof is lost even though the content is
     identical.
  3. **Refresh the receipt head on a docs-only edit without rerunning.** Gain:
     the check stays a simple head comparison. Cost: the receipt would claim a
     revision was tested when only the old revision was tested; the proof
     becomes dishonest and unauditable. In practice: a docs commit advances
     the receipt head and archive accepts content that was never tested.
- Disposition: carried — the receipt binding is an architecture decision;
  owner is the design; deciding artifact is `design.md` (D3).

### D4 — Does proof reuse apply to review as well as archive?

- Question: Should `givn check review` use the same documentation-only reuse
  rule as `givn archive`, or should only archive accept a docs-only revision?
- Options:
  1. **Shared rule in review and archive.** Both commands accept the same
     receipt under the same boundary. Gain: one contract, no disagreement
     between the review gate and the archive gate, and no forced rerun before
     review after a docs edit. Cost: review behavior changes too, so its
     receipt tests need a docs-only case. In practice: after editing
     `review.md`, `givn check review` passes and `givn archive` also passes
     without `givn check tasks`.
  2. **Archive only.** Keep review strict on head equality. Gain: the smallest
     behavior change. Cost: a documentation edit after the proof still forces
     `givn check tasks` before review, which is the same friction the change
     is meant to remove. In practice: review refuses, the agent reruns the
     suite, and archive then succeeds — exactly the C2 loop.
- Disposition: carried — the shared-rule decision is an architecture decision;
  owner is the design; deciding artifact is `design.md` (D4).

### D5 — How wide is the diagnostic aggregation boundary?

- Question: Should `givn check review` batch only use-case schema and
  capability-routing diagnostics, or aggregate every review gate into one
  report?
- Options:
  1. **Batch the schema/routing family and keep gate order.** Integrity,
     overlap, retirement, routing, and receipt keep their current order; the
     routing gate reports all schema and routing diagnostics together. Gain:
     preserves the `verify-change` rule that structural failures outrank other
     findings; a corrupt or unsafe change still stops before later gates.
     Cost: if overlap fails first, routing diagnostics wait for the next run.
     In practice: a change with both a missing overlap disposition and a
     broken use-case sees the overlap failure first, then the complete
     schema/routing report.
  2. **Aggregate every review gate.** Run integrity, overlap, retirement,
     routing, and receipt independently and print every finding together.
     Gain: one report for every gate. Cost: changes the established gate order
     and can bury a structural failure among advisory findings; every review
     output contract and test changes. In practice: a missing integrity
     hash and a schema typo appear together, and the developer must decide
     which one is structural.
- Disposition: answered — the user scope names "the complete set of use-case
  schema and capability-routing diagnostics", and `verify-change` already
  states "Structural failures outrank other findings." Option 1 is chosen and
  realized in `design.md` (D5).

### D6 — How are older receipts handled?

- Question: What should archive do with a receipt written before the
  tested-content digest existed?
- Options:
  1. **Schema 2 with an optional digest; schema 1 keeps strict head equality.**
     New receipts carry the digest and scope; an old receipt without them is
     accepted only when its head equals the current head, otherwise it is
     stale. Gain: a safe upgrade with no false reuse. Cost: one rerun for a
     change already in flight when the new binary lands. In practice: an old
     receipt after a docs commit refuses and routes to `givn check tasks`;
     after the rerun, the new receipt is reusable.
  2. **Treat a missing digest as docs-only eligible.** Gain: no rerun on
     upgrade. Cost: an unbound receipt would authorize content it never
     tested; the gate becomes unsound. In practice: any old receipt would
     pass archive after arbitrary code changes.
  3. **Reject every schema 1 receipt outright.** Gain: one unambiguous rule.
     Cost: an in-flight change cannot archive until the suite is rerun even
     when nothing changed; more friction than necessary. In practice: a
     docs-free change with a current old receipt is refused.
- Disposition: carried — backward compatibility is an architecture decision;
  owner is the design; deciding artifact is `design.md` (D6).
