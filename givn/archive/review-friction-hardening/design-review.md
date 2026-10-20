# Design Review: review-friction-hardening

## Method

Fresh-context grilling against `proposal.md`, `design.md`, `HANDOFF.md`,
`arc42.md`, both delta features, both delta use-case narratives, the current
source and test seams, the specs instruction, the review instruction, and the
arc42-docs instruction. The grilling agent walked every required branch and
reported findings; the orchestrator applied the hardening edits and re-checked
the affected artifacts. The operator's recorded directions are the change
handoff and the confirmed proposal constraints.

## Q1 — Is the documentation-only archive scenario a distinct interaction?

- Status: closed 2026-10-21 (recorded handoff direction).
- Blocks: the `archive-change` interaction inventory, the `@e2e` tag, and the
  required visual transcript set.
- Context: the specs instruction says variants of one user action do not create
  additional inventory entries or `@e2e` scenarios. The ordinary archive
  outline already proves "archive a completed change". This scenario's
  observable contract is different: a green receipt followed only by
  documentation/evidence revisions must archive without invoking the change
  suite. The handoff requires exactly two transcripts, one of them this
  scenario.
- Options:
  1. **Keep the distinct inventory entry and `@e2e`.** Meaning: the proof-reuse
     state transition is treated as its own interaction. Example: the fixture
     commits `review.md`, then archives; the assertion is a proven archive plus
     one merged `givn/specs` invocation and no change-features invocation.
     Gain: the no-rerun contract is proven through the real CLI. Cost: one more
     inventory row and one more transcript than a pure variant would need.
  2. **Treat it as a variant of the ordinary archive.** Meaning: keep the
     assertions as a regular scenario and let the existing backend-parameterized
     outline own `@e2e`. Example: the ordinary archive scenario archives with a
     current receipt; no scenario asserts the absence of a change-features
     invocation. Gain: fewer `@e2e` rows. Cost: the handoff's required
     transcript has no scenario, and the no-rerun side effect is unproven.
- Resolution: option 1. The handoff already requires the transcript, and the
  observable side effect (no change-suite invocation) is not a flag variation
  of the ordinary action. Rejected: option 2 removes the evidence the owner
  requires and hides the defect this change fixes.

## Q2 — Should Fossil prove ancestry or only revision existence?

- Status: closed 2026-10-21 (codebase evidence plus the handoff's file scope).
- Blocks: the freshness result and the honesty of the "history rewrite" rule.
- Context: `Backend::is_ancestor` is real Git ancestry, but the Fossil seam
  answers existence only (`src/fossil.rs:189-193`). The proposal promises a
  refusal after a history rewrite; the handoff scopes `src/fossil.rs` to
  `content_paths()` only.
- Options:
  1. **Narrow the promise to "tested revision available".** Meaning: Git checks
     ancestry, Fossil checks existence, and the tested-content digest remains
     the proof. Example: a Git rebase that drops the receipt revision refuses;
     a Fossil revision that still exists is accepted. Gain: no new SCM
     operation, and the design stops claiming a check it cannot perform. Cost:
     Fossil accepts a rewritten but existing tested revision.
  2. **Implement real Fossil ancestry.** Meaning: add a new Fossil query and
     treat its answer like Git ancestry. Example: both backends refuse a dropped
     tested revision. Gain: one rule on both backends. Cost: a new SCM
     operation and compatibility surface, outside the handoff's file scope.
  3. **Leave the false promise.** Meaning: keep "reachable" wording. Gain: no
     edit. Cost: the refusal can never fire on Fossil, and the docs claim
     behavior the tool does not have.
- Resolution: option 1, recorded as R-106. Rejected: option 2 expands scope
  without evidence that Fossil ancestry is needed; option 3 is dishonest.

## Q3 — Fix the pre-existing full-lint mismatch here?

- Status: closed 2026-10-21 (recorded handoff direction).
- Blocks: whether this change also edits the `write a runner result` inventory
  row or tags that scenario.
- Context: a full `givn lint` exits 3 because
  `The result helper writes four counts without a scope` is advertised as the
  E2E scenario for `write a runner result` but carries no `@e2e` tag. The
  handoff explicitly records this as pre-existing and outside this change.
- Options:
  1. **Leave it.** Meaning: the delta-only lint is the gate; the full-lint
     mismatch stays a separate decision. Example: `givn lint --change
     review-friction-hardening` still exits 2 only for `@wip`. Gain: the change
     stays inside its two defects. Cost: a known blocking full-lint result
     remains.
  2. **Fix it here.** Meaning: add `@e2e` or correct the inventory row. Example:
     the full lint exits 0 after the change. Gain: one clean full lint. Cost:
     touches an unrelated capability and changes its E2E evidence obligation
     without the owner's approval.
- Resolution: option 1, per the handoff. Rejected: option 2 silently widens the
  change and changes an unrelated scenario's evidence contract.

## Q4 — Keep or rename "documentation-only revision"?

- Status: closed 2026-10-21 (recorded handoff direction).
- Blocks: the glossary term, the reuse message, and the scenario titles.
- Context: the check proves unchanged tested content, not the intervening diff;
  a code change followed by a revert would reuse the proof. The handoff names
  the term and requires the reuse message.
- Options:
  1. **Keep the term, define the proven condition precisely.** Meaning: the
     glossary says the receipt proves content equality under an available
     revision; the ordinary case is a documentation/evidence-only change.
     Example: a receipt-only commit reuses; a documented revert also reuses
     because the tested bytes are identical. Gain: the owner's vocabulary and
     message stay stable. Cost: the term slightly overstates the proven diff.
  2. **Rename the state and message to "tested content unchanged".** Meaning:
     the output states exactly what was checked. Example: `reused with unchanged
     tested content`. Gain: no overstatement. Cost: breaks the handoff's named
     term and every assertion that uses it.
- Resolution: option 1. The glossary and design now state the check's actual
  guarantee. Rejected: option 2 changes the approved vocabulary for a
  distinction the glossary now makes explicit.

## Q5 — AMEND_ADR, NEW_ADR, or SUPERSEDE_ADR?

- Status: closed 2026-10-21 (independent qualification).
- Blocks: whether the receipt binding gets a new record or amends ADR-070.
- Context: ADR-070 owns the forward-only archive proof and its driver says
  verification must not be duplicated across review and archive. No ADR names
  the change-scoped `tasks-verification.json`.
- Options:
  1. **Amend ADR-070.** Meaning: record the tested-content binding and schema
     compatibility as a refinement of the existing proof contract. Example:
     ADR-070 gains an amendment section; the publication boundary is unchanged.
     Gain: one record per boundary, no parallel proof. Cost: ADR-070 grows.
  2. **New ADR.** Meaning: a separate record for the change-scoped receipt.
     Example: ADR-074 owns the pre-publication receipt. Gain: a focused record.
     Cost: splits one proof boundary across two records and duplicates the
     forward-only owner.
  3. **Supersede ADR-070.** Meaning: replace the archive-proof record. Example:
     ADR-070 becomes historical. Gain: none. Cost: discards the unchanged
     publication decision and forces readers to a replacement for a refinement.
- Resolution: option 1. The amendment records the corrected evidence (ADR-070's
  no-duplicate-verification driver and the absence of any record for
  `tasks-verification.json`). Rejected: options 2 and 3 misclassify a refinement
  as a new or replacement boundary.

## Findings and hardening edits

### Blocking findings resolved

1. **Routing scenario was already-green or unsatisfiable.** The parser
   `ParseResult` is now defined as a recoverable document plus diagnostics, and
   the scenario now targets the real RED defect: one document parse failure
   repeated once per delta capability. The repeated-per-capability report is
   replaced by one grouped report.
2. **The proposal's repeat-per-capability defect had no scenario.** The
   "One use-case parse failure is reported once across several delta
   capabilities" scenario now covers it.
3. **Delta narratives were bypassed by routing early returns.** The design now
   collects and validates active delta narratives before both early returns,
   including a narrative-only delta, and keeps the routing-table advisory for
   routing decisions only.
4. **Fossil ancestry was claimed but unavailable.** The freshness result now
   says "available" (Git ancestry, Fossil existence), records R-106, and the
   runtime view and glossary no longer claim ancestry on both backends.
5. **Fossil `content_paths` had no test.** The test strategy now requires
   `content_paths` unit coverage on both backends and a digest round trip.
6. **The docs-only term overclaimed the diff.** The design and glossary now
   state that the check proves unchanged tested content, not the intervening
   diff.
7. **The README impact deliverable was missing.** `design.md` now has a
   documentation-impact section and `README.md` records both observable
   changes; review will record `README-IMPACT: updated`.
8. **ADR-070 amendment evidence was wrong.** The amendment now cites ADR-070's
   no-duplicate-verification driver, states that no record owns
   `tasks-verification.json`, and includes the four MADR checklist items.
9. **The use-case main flow contradicted the gate order.** The delta narrative
   now runs structural, overlap, retirement, and routing checks before the
   receipt check, matching `src/cli/check.rs`.

### Non-blocking findings dispositioned

- Near-miss heading suggestions now have their own scenario; Q-108 is accurate.
- Missing required sections and a malformed interaction table now have a
  scenario and unit-test coverage.
- The `is_documentation_or_evidence_path` signature now takes the manifest,
  reuses `is_evidence_path` for evidence, and keeps README unconditional.
- Gitlinks hash their recorded object id, symlinks hash their target bytes, and
  unreadable paths are named errors.
- Missing-title diagnostics are document-level and carry no `line 0` prefix.
- An unknown content scope is `UnsupportedContentScope`, not a legacy receipt.
- A feature or command change may also change the tested-content digest; the
  design now states that both reasons may be named because they are distinct
  boundaries.
- Generated semantic caches are justified as derived indexes, never test input.
- The fourth Persona row was removed; the proposal's three confirmed Personas
  are the review lenses.
- Fixture mechanics are named: `skip_receipt_autowrite`, a pinned routing
  decision, and committed tested-content changes in the outline.
- Chapter 5 now has `spec::routing_table`; chapter 6 has the batched review
  flow; chapter 8 §8.12 names the complete review check order; chapter 10 has
  Q-108 in order; chapter 11 has R-104 through R-106.
- The docs-only archive `@e2e` is retained per Q1, with the normalization
  rationale recorded in the design.

## Branch coverage

- **Scope:** the spec covers both proposal fixes. Two new schema scenarios and
  the dedupe scenario close the previously missing behaviors. No extra scope.
- **Tech choices:** diagnostic collector, backend-neutral digest, and one
  freshness function are the simplest shapes that satisfy the scenarios. The
  public wrapper adds no invariant but is harmless.
- **Missing scenarios:** resolved by the near-miss heading, missing-section/
  malformed-table, and dedupe scenarios; schema 1, unknown scope/schema, dirty
  tested file, gitlinks, symlinks, and Fossil content paths are unit tests.
- **Testability:** strict mode is `.fail_on_skipped()`; the single-scenario
  command and undefined-step probe exit non-zero. Every scenario can fail RED:
  the dedupe scenario fails today's repeated report, and the reuse scenarios
  fail today's strict head equality.
- **E2E fidelity:** CLI subprocess is the real interface; the assertions are on
  exit code and raw stdout/stderr; the archive scenario asserts the proven
  archive, the single merged regular invocation with `givn/specs`, and no
  change-features invocation.
- **Interaction coverage:** every new inventory entry has a matrix row; the
  docs-only archive action is a distinct state transition per Q1.
- **Risk:** the most likely failure is a digest path-classification drift
  between `check tasks` and review/archive. The round-trip unit tests and both
  backend listings are the mitigation. The second is a parser regression; the
  existing parser tests are the characterization baseline and the new scenario
  table covers each recovery branch.
- **Use-case and Persona context:** base hashes match the permanent narratives,
  capabilities remain `EXTEND givn-workflow-enforcement` and `EXTEND
  archive-proof`, relationships are unchanged, and only confirmed Personas are
  used. The review rejection criteria pass.
- **Decision ledger:** D1-D6 all resolve in `design.md`; D2/D3/D6 are carried
  into the ADR amendment.
- **Visual design contract:** `Interface: terminal`, two required transcripts,
  no `NOT APPLICABLE` path for terminal.
- **Deferrals:** none. The full-lint mismatch is a declared out-of-scope
  pre-existing finding, not a deferred mandatory check.
- **Persona disposition and interface classification:** terminal is correct;
  the opposite web classification cannot hold (plain stdout/stderr, no
  browser-rendered surface), and `none` would wrongly exempt the transcript
  gate.
- **ADR qualification:** AMEND_ADR to ADR-070 with all five mandatory gates
  passing and the corrected existing-record evidence.
- **Architecture documentation:** the independent 12-row walk matches
  `arc42.md` after the chapter-8 correction; all 12 chapter files exist, carry
  content, and contain only Mermaid diagrams.
- **Ubiquitous language:** `Current`, `DocumentationOnly`,
  `Documentation-only revision`, and `Tested content` are now distinguished in
  the glossary and used consistently in the design and specs.

## Hardening verification

- `givn lint --change review-friction-hardening` exits 2 with one `@wip` finding
  per scenario and subset advisories only.
- The use-case delta narratives still carry their base hashes; the permanent
  narratives are untouched.
- `docs/arc42/09-architecture-decisions.md` and
  `docs/arc42/adr/README.md` record the ADR-070 amendment; the ADR body carries
  the qualification verdict and checklist.

DESIGN-REVIEW: PASS
