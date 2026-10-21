# Design Review: review-diagnostic-corrections

## Process

Fresh-context grilling subagent read `proposal.md`, `design.md`, the delta
feature, `HANDOFF.md`, `arc42.md`, the owning `usecase.md`, the parser,
routing, overlap analysis, the feature runner, and the step definitions. Every
required branch was walked. Two questions were put to the operator one at a
time; two more were settled from the recorded artifacts instead of asking.

## Questions and resolutions

### Q1 — Canonical suggestion for an exact-prefix invalid title

- Status: closed 2026-10-21 (resolved from the proposal; no operator question)
- Finding: the proposal promises a canonical correction for a human-readable
  title, but the design gave `# Use case: Lending Register` only the bare
  `validate_id` error while the same title with a different case got the
  correction.
- Resolution: apply the canonical suggestion on both paths. `# Use case:
  Lending Register` now reports `must be a stable kebab-case identifier` plus
  `did you mean '# Use case: lending-register'?`; the identifier is still never
  accepted. Rejected: keeping the inconsistent message (proposal drift) and
  narrowing the proposal to case mismatch only (leaves a real variant without
  the promised correction). Realized in `recover_title`,
  `canonical_title_id`, the exact-prefix unit test, and the new delta scenario
  “Review corrects an exact-prefix non-kebab-case title with the canonical
  identifier”.

### Q2 — Permanent rule and README wording

- Status: closed 2026-10-21 (operator)
- Question: amend the "verify-change" contract rule and the README sentence, or
  keep them and interpret “problem” as “actionable problem”?
- Operator decision: amend both, reworded to name all actionable problems.
- Resolution: `givn/specs/verify-change/usecase.md` now reads “A review report
  names the root cause of an invalid use-case document first and then names all
  actionable use-case schema and capability-routing problems in one pass; the
  schema stays strict and semantic validation is not weakened.” The README
  “One change lifecycle” bullet carries the same qualification. Rejected:
  leaving the literal “every problem” wording (contract drift) and amending
  only the contract (stale public description).

### Q3 — Fresh terminal transcript

- Status: closed 2026-10-21 (resolved from `HANDOFF.md` and the design decision)
- Finding: the only transcript shows the pre-correction output and therefore
  depicts the defect.
- Resolution: keep the archived transcript as historical reference only and
  add no fresh transcript. The delta adds no `@e2e` scenario, so the
  per-scenario transcript requirement is empty; `HANDOFF.md` explicitly keeps
  the terminal gate without a new transcript path. The corrected report is
  asserted by seven ordinary-CLI scenarios through the real subprocess.
  Rejected: committing a hand-made transcript (fabricated evidence) and adding
  a second `@e2e` scenario for the same review action (duplicate coverage).

### Q4 — The valid-title scenario cannot fail RED

- Status: closed 2026-10-21 (operator)
- Finding: “A valid kebab-case use-case title remains accepted” passes on
  today's code, so it cannot produce a RED failure without breaking valid
  parsing.
- Operator decision: option A — keep it as a recorded green-at-baseline
  regression guard.
- Resolution: the design marks it as a guard and the task list must not
  fabricate a RED for it. Its RED evidence is the baseline-green run plus the
  explicit guard note. Rejected: removing the scenario (loses a direct
  valid-input CLI proof) and moving the assertions into a permanent scenario as
  `@givn.modified` (still green at baseline and rewrites unrelated permanent
  behavior).

## Branch findings

- **Scope** — the delta covers both defects and the strictness boundary. The
  proposal's general canonical-correction statement was restored by Q1. No
  extra capability, interaction, or `@e2e` scenario.
- **Tech choices** — Rust, no new dependency, parser-owned gate, and
  `Option<UsecaseDoc>` consumption are the right fit. No simpler alternative
  satisfies the parser-shared callers.
- **Missing scenarios** — added the exact-prefix correction, independent local
  findings under an invalid root, dependent routing complaints, empty required
  sections, both roots invalid, and the valid-root missing-section guard.
- **Testability** — the seven error scenarios fail RED; the valid-title
  scenario is an explicitly recorded guard. The design names `fail_on_skipped`
  (runner lines 55 and 151), the `unimplemented!()` RED stub, and the
  single-scenario `GIVN_FEATURES=... cargo test --name` command. The
  exact-suggestion step asserts the whole rendered fragment, not two loose
  substrings.
- **E2E fidelity** — one existing review action, terminal interface, real
  `givn check review` subprocess, primary stderr and exit-code assertions. No
  duplicate action and no new `@e2e` scenario.
- **Interaction Coverage** — all 13 inventory rows are present in the design
  matrix; the delta's comment-block copy of one row was removed, and the
  inventory remains only in `usecase.md` §Interactions.
- **Risk** — the most likely failure is moving a section reader into the
  document conditional, which silently drops independent findings. The design
  now requires all section values to be computed into locals first and adds
  unit test `invalid_root_still_reports_local_shape_findings`. A second risk,
  checks keyed on `title.is_some()`, is closed by keying duplicate-title and
  outside-content checks on `title_seen` with its own unit test.
- **Use-case and Persona context** — `verify-change` remains the owner; no
  capability or interaction is invented. The three confirmed Personas are
  developer, agent, and maintainer; no biography is used as an actor.
- **Decision ledger** — every proposal decision `D1`–`D5` resolves in the
  design's Decisions section with rejected alternatives. Q1/Q2 do not add a new
  open decision.
- **Visual design contract** — terminal interface, no browser, no new
  viewports. `VISUAL-REVIEW: PASS` and a confirmed `PERSONAS:` line remain
  mandatory; no `NOT APPLICABLE` or `PERSONAS: none` path was added.
- **Deferrals** — none.
- **Persona disposition** — developer, agent, and maintainer impacts are
  recorded; two Personas receive the corrected report and the maintainer owns
  the suppression boundary.
- **Interface classification** — terminal, not Web UI: the output is plain CLI
  text and the scenarios drive a subprocess, not a browser.
- **ADR qualification** — `arc42.md` records `NOT_QUALIFIED` /
  `CANONICAL_ARTIFACT: design.md` for D1–D5. No existing ADR records
  diagnostic ordering, title recovery, or Level fallback; no new ADR is
  warranted. The cheaper home is `design.md` for every candidate.
- **Architecture documentation** — the independent 12-row derivation matches
  `arc42.md`: chapters 6, 8, 11, and 12 are affected; the other eight are not.
  Chapter 12 already contains the four new terms. All 12 chapter files exist,
  contain real content, and contain no ASCII art or box-drawing pseudo-diagrams.
- **Ubiquitous language** — **root diagnostic**, **dependent diagnostic**,
  **title line**, and **section schema** are in the glossary and used
  consistently in the design, feature, and chapter updates.

## Hardening applied

1. `design.md`: exact-prefix canonical suggestion; `TitleRecovery.suggestion`
   is optional; `title_seen` governs duplicate-title and outside-content
   checks; `report_undeclared` is gated on root validity plus a present
   capability heading; all section values are hoisted before `UsecaseDoc`
   construction; behavior matrix covers both-roots-invalid and empty sections.
2. `design.md`: seven scenarios, the exact-suggestion step, the
   `unimplemented!()` stub, the green-at-baseline guard, all 13 interaction
   rows, `src/overlap/analysis.rs` as an unchanged consumer, the historical
   transcript note, README/contract/arc42 documentation impact, and
   `cucumber 0.23.0`.
3. Delta feature: six error variants plus the recorded valid-title guard;
   exact suggestions; no missing-section, empty-section, leaf-capability, or
   dependent-routing complaints; independent local findings preserved (the
   Rule-bullet assertion is reachable only after the section-reader hoist).
4. `givn/specs/verify-change/usecase.md` and `README.md`: the
   actionable-problems wording approved in Q2.
5. Arc42: chapter 6 and chapter 8 root-first flow, chapter 11 risks R-119 and
   R-120, chapter 12 glossary terms, and the matching 12-row assessment.
6. Parser unit-test plan adds exact-prefix suggestion, local-shape preservation,
   empty-section suppression, duplicate/outside-content preservation, both-roots
   invalid, and valid-root missing-section guard.

## Verification

- `givn lint --change review-diagnostic-corrections` exits 2 with 8 `@wip`
  findings, which is the expected pre-implementation state.
- No `@e2e` tag was added or removed.
- No production code changed in this step.

DESIGN-REVIEW: PASS
