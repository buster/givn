# Proposal: review-diagnostic-corrections

## Release note

Review now corrects a non-kebab-case use-case title to its canonical identifier and withholds section-schema errors that depend on an invalid title or Level, while still reporting genuinely unknown headings.

## Use-Case Context

- Use-case ID: `verify-change`
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` — listed in the permanent `verify-change` contract (`givn/specs/verify-change/usecase.md`); no new Persona is proposed.

## Problem / Opportunity

The archived `review-friction-hardening` change made `givn check review` report every use-case schema and capability-routing problem in one pass. The B4/C3 benchmark comparison exposed two remaining defects in that diagnostic output. The gates themselves are correct; the report misleads the developer and wastes review iterations.

Evidence: `/tmp/opencode/compare-runs/candidate-c3/20261020T234012Z-candidate-c3-r1` — `stages/initial/events.stage.jsonl` and `stages/initial/agent.stdout.log`; harness findings `/home/buster/projects/givn-harness/docs/findings/2026-10-21-b4-c3-comparison.md`.

### Defect 1 — the correction preserves an invalid title

C3 reported:

```text
line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: Lending Register'; headings are case-sensitive — did you mean '# Use case: Lending Register'?
```

The suggested title keeps a space and uppercase letters, so the suggested document is still invalid. The parser treats the remainder of the title line as the recovered identifier without validating it as kebab-case. The existing recovery test only covers a valid identifier (`start-project`), so the invalid-human-title path is untested. The developer is told to make an edit that cannot pass.

### Defect 2 — an invalid root produces dependent section diagnostics

The same C3 run reported:

```text
document schema does not recognise '## Diagram'
document schema does not recognise '## Examples'
document schema does not recognise '## Minimal guarantee'
document schema does not recognise '## Out of scope'
document schema does not recognise '## Rules'
leaf use case 'Lending Register' must declare at least one capability
```

The final accepted narrative for the same change contains `## Rules`, `## Examples`, `## Minimal guarantee`, and `## Out of scope`, and it archived successfully. The malformed title and the malformed `## Level` were the root causes. The parser accepted the invalid title as the document identity, then invented a leaf Level when the Level text was invalid, and then reported schema and capability findings that depended on those invented values. The report buried the two edits the developer actually needed under a list of derived complaints. Genuinely unknown headings such as `## Guarantees` must still be reported; the defect is the dependent cascade, not the strictness.

Impact: the developer or agent spends review iterations correcting symptoms instead of the root cause. The one-pass diagnostic promise is only useful when the pass separates root causes from dependent findings.

## Proposed Solution

Review reports the root causes of an invalid use-case document first, and only reports schema-completeness findings that are meaningful once the root is valid.

1. **Canonical title correction.** When the title line has the right words but a human-readable name that is not a kebab-case identifier, review suggests the canonical kebab-case identifier. For `# Use Case: Lending Register`, the suggestion is exactly `# Use case: lending-register`. The human title is never treated as the document identifier. The schema stays strict: no title variant is accepted, and the document still fails review until the title is corrected.
2. **Root-first section diagnostics.** When the title or the `## Level` value is invalid, review reports that root diagnostic and withholds section-schema completeness findings that depend on it: missing required headings, empty required sections, the leaf-use-case capability requirement, and dependent capability-routing complaints. Independent findings are still reported in the same pass: genuinely unknown headings, malformed list bullets, and malformed interaction tables.
3. **Valid documents are unchanged.** A valid kebab-case title and a valid Level continue to be accepted, and a document with a valid root still reports every missing section and every independent schema problem in one pass.
4. **Batching is preserved.** The report remains one grouped review result with each distinct problem once. This change removes dependent noise; it does not weaken the strict schema.

Example after the change, for a document whose title is `# Use Case: Lending Register` and whose Level is malformed:

```text
line 1: expected '# Use case: <kebab-case-id>', got '# Use Case: Lending Register'; headings are case-sensitive and IDs are kebab-case — did you mean '# Use case: lending-register'?
use-case Level must be '!' or '+', got '`+` — summary use case. Its five capabilities decompose the lending-desk activity into the goals below.'
unknown heading '## Guarantees'; expected one of '## Minimal guarantee', '## Success guarantee'
```

The exact rendering of the Level text is illustrative; the observable requirement is that the title and Level are reported as roots, the unknown heading is still reported, and no missing-section or leaf-capability complaint is derived from the invalid root.

## Capability Routing

`givn spec route --change review-diagnostic-corrections` reported signal for `EXTEND givn-workflow-enforcement` during proposal routing. Later advisory re-runs vary with proposal prose and can tie unrelated capabilities; the recorded decision is `EXTEND givn-workflow-enforcement` because the change belongs to the existing review-diagnostics capability, invents no new capability, and adds no interaction.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| givn-workflow-enforcement | EXTEND givn-workflow-enforcement (signal during proposal routing; later advisory re-runs vary) | EXTEND givn-workflow-enforcement | No deviation. The existing capability owns review-time use-case schema diagnostics, and its existing interaction inventory entry already covers the review action. |

No new capability and no new interaction are introduced. The existing `verify-change` interaction row “run the review check on a change with use-case schema errors” remains the interaction owner; the delta adds error-case scenarios under the same capability.

## Out of Scope

- Changing gate correctness, gate order, archive behavior, or the strict use-case schema.
- Accepting the canonicalized title or any other title variant without an explicit edit.
- Adding a `PERSONAS: none` escape or a `VISUAL-REVIEW: NOT APPLICABLE` path for terminal interfaces.
- Persona validation, terminal visual-review behavior, and evidence requirements.
- Changing the benchmark or harness, and any paid benchmark run.
- Modifying the archived `review-friction-hardening` change.
- Fixing the other C3 review friction: promised-file integrity, overlap dispositions, receipt ordering, capability-routing mismatches, and the missing visual verdict in the follow-up.
- Adding a new capability, a new interaction inventory entry, or a new `@e2e` scenario for the same review action.

## Open Questions

The scope fixes the two defects and the strictness boundary. The remaining choices concern the exact correction and suppression mechanics; each is carried to the technical design with the named owner and deciding artifact.

### D1 — How is the canonical title suggestion derived?

- Question: When the title remainder is a human-readable name such as `Lending Register`, should review derive a canonical kebab-case identifier from that text, require the author to look up the document identifier, or receive the enclosing document context, and should the derived value ever be accepted as the title?
- Context: The parser currently sees only document content. C3's suggestion echoed the human title, which cannot pass. The correction must never suggest an invalid identifier and must not weaken the strict schema. The final accepted use case for C3 is `givn/specs/lending-register/usecase.md`, whose identifier is `lending-register`.
- Options:
  1. **Slugify the title text into a candidate identifier and suggest it without accepting it.** Meaning: lowercase the remainder, turn runs of characters outside the identifier alphabet into single hyphens, and validate the result; the suggestion is advisory and the document still fails. Gain: deterministic exact correction that needs no parser context and works for permanent and delta narratives. Cost: a human title that differs from the intended identifier can produce a plausible but wrong suggestion; the author must confirm. In practice: `# Use Case: Lending Register` suggests `# Use case: lending-register`; `# Use Case: Start Project` suggests `# Use case: start-project`.
  2. **Require the author to look up the identifier; provide no suggestion for a non-kebab-case remainder.** Meaning: only a case-mismatched but already-valid identifier receives an exact suggestion. Gain: never suggests a wrong identifier. Cost: the C3 author still has to find `lending-register` elsewhere, which is the iteration this change is meant to remove. In practice: the report says only that the title is invalid and the identifier must be kebab-case.
  3. **Thread the enclosing document or change identifier into the parser and suggest that.** Meaning: `givn check review` passes the change or use-case context so the suggestion is exact. Gain: the suggestion cannot diverge from the intended identifier. Cost: every parser caller gains path/context plumbing, including permanent specs and fragments, for a diagnostic-only concern. In practice: the parser would need a new context parameter in addition to content.
- Disposition: carried — owner: thinking architect; deciding artifact: `design.md`. The design must choose one option, record the rejected alternatives, and never accept a derived identifier as the document title.

### D2 — Which section diagnostics are suppressed when the title or Level is invalid?

- Question: When the root title or Level is invalid, should review suppress every section diagnostic, suppress only the schema-completeness and dependent routing diagnostics while keeping independent local findings, or suppress nothing beyond the title suggestion fix?
- Context: C3 mixed two root causes with missing-section and leaf-capability findings. Suppressing too little leaves the cascade; suppressing too much loses the one-pass batching that `review-friction-hardening` added. Genuinely unknown headings such as `## Guarantees` must still be reported.
- Options:
  1. **Suppress only dependent schema-completeness and routing findings.** Meaning: withhold missing required headings, empty required sections, the leaf-capability requirement, and capability-routing complaints that need a valid document; keep unknown headings, malformed list bullets, and malformed interaction tables. Gain: removes the C3 cascade while preserving independent findings in the same pass. Cost: an invalid-root document can still show some local shape errors; the author fixes the root, reruns, and then sees any genuine completeness errors. In practice: C3 would report the title and Level, keep `unknown heading '## Guarantees'`, and not report `document schema does not recognise '## Rules'` or `leaf use case ...`.
  2. **Suppress every section diagnostic until the root is valid.** Meaning: an invalid title or Level produces only the root diagnostic. Gain: the shortest possible report. Cost: independent local findings disappear, so fixing the root can reveal a second wave of problems; this weakens the one-pass promise. In practice: a malformed title plus a malformed capability bullet reports only the title.
  3. **Suppress nothing; only fix the title suggestion.** Meaning: keep the current diagnostic set and correct the suggestion. Gain: smallest change. Cost: the C3 cascade remains, so defect 2 is not fixed. In practice: the invalid title still yields missing-section and leaf-capability complaints.
- Disposition: carried — owner: thinking architect; deciding artifact: `design.md`. The design must name the exact suppression boundary and prove that a valid-root document still reports missing sections.

### D3 — What happens to a document whose Level is invalid?

- Question: When `## Level` is neither `!` nor `+`, should review keep the current fallback to a leaf Level, treat the document as root-invalid and stop deriving dependent findings, or choose another default?
- Context: C3's Level text was `` `+` — summary use case. Its five capabilities decompose ...``. The parser failed to parse it, defaulted to leaf, and then reported `leaf use case 'Lending Register' must declare at least one capability`. The fallback invented a Level the author never wrote.
- Options:
  1. **Treat the invalid Level as root-invalid and produce no usable document.** Meaning: report the Level rule violation, do not invent a Level, and suppress dependent schema and routing findings. Gain: no fabricated semantics; the C3 leaf-capability complaint disappears. Cost: routing-level checks wait for a valid Level, so a document with an invalid Level gets one root-focused review pass. In practice: the report names the malformed Level and the title; it does not claim the document is a leaf.
  2. **Keep the fallback to leaf and only suppress the leaf-capability check.** Meaning: the document remains usable for other checks under a leaf assumption. Gain: fewer changes to the parse result. Cost: other level-dependent behavior can still act on an invented Level, and the document can feed routing checks despite being invalid. In practice: a malformed `+` document is treated as a leaf for every check except the one diagnostic that was patched.
  3. **Fall back to summary Level.** Meaning: assume `+` when the Level is invalid. Gain: avoids the leaf-capability complaint for C3-like documents. Cost: equally invented, and it suppresses the leaf-capability requirement for genuinely leaf documents with a typo. In practice: a one-character Level typo silently changes the schema expectations.
- Disposition: carried — owner: thinking architect; deciding artifact: `design.md`. The design must record the chosen behavior and the rejected defaults.

### D4 — Where does the root-validity gate live?

- Question: Should the parser own root validation and emit only non-cascading diagnostics, or should the routing/report layer post-filter diagnostics after the parser has emitted them?
- Context: `parse_usecase_md_diagnostics` is shared by direct parsing, routing, coverage, tree inspection, and archive. A routing-only filter would leave the other callers with the cascade and duplicate the title/Level rules.
- Options:
  1. **Parser owns root validation; routing consumes the existing optional document.** Meaning: the parser reports the root diagnostics and returns no usable document when the root is invalid; `check_routing_consistency` already skips checks that need a document when none is returned. Gain: one source of truth for every caller; no string post-filtering. Cost: the parser's private scan flow changes and its tests must cover the suppression boundary. In practice: `givn check review`, direct parsing, and archive all get the same non-cascading report.
  2. **Routing layer filters the parser's diagnostic list.** Meaning: the parser keeps emitting everything and routing removes lines that look dependent. Gain: no parser refactor. Cost: duplicated title/Level validation, brittle string matching, and other callers still cascade. In practice: `parse_usecase_md` still reports the C3 cascade while `givn check review` hides part of it.
- Disposition: carried — owner: thinking architect; deciding artifact: `design.md`. The design must choose the parser-owned gate unless it can show a concrete caller that cannot use the optional-document contract.

### D5 — Does the change need a new E2E scenario or inventory entry?

- Question: Should the canonical correction and the suppression behavior be proved by a new `@e2e` scenario with a new interaction inventory entry, by modifying the existing review-diagnostics `@e2e` scenario, or by reusing the existing entry and adding regular scenarios?
- Context: The existing `verify-change` inventory already maps “run the review check on a change with use-case schema errors” to the `@e2e` scenario “Consumer runs review and receives every use-case schema problem in one pass”. The canonical-correction scenarios are error variants of the same review action, not a new user action.
- Options:
  1. **Reuse the existing inventory entry and `@e2e` scenario; add regular scenarios.** Meaning: the new behaviors are error-case scenarios under `givn-workflow-enforcement`; the existing real-CLI `@e2e` scenario remains the action evidence. Gain: no duplicate E2E action, no extra runtime, and the one-E2E-per-action rule stays intact. Cost: no new terminal transcript specifically shows the canonical correction; parser unit tests and real-CLI regular scenarios cover it. In practice: the delta contains six `@givn.added @wip` scenarios and no new `@e2e` tag.
  2. **Add a new `@e2e` scenario and inventory row.** Meaning: the canonical correction gets its own real-CLI E2E evidence. Gain: a transcript of the exact new output. Cost: duplicates the review action, violates the one-E2E-per-distinct-action rule, and adds terminal evidence work. In practice: two inventory rows would describe the same `givn check review` invocation with different inputs.
  3. **Modify the existing `@e2e` scenario to use the invalid human title.** Meaning: the existing E2E action now proves the correction. Gain: one E2E scenario, new output covered. Cost: the existing scenario's batched valid-identifier case is lost or must be split, weakening the archived batch evidence. In practice: the permanent scenario “Consumer runs review and receives every use-case schema problem in one pass” would change its fixture and no longer prove the case-mismatched valid-ID path.
- Disposition: carried — owner: thinking architect; deciding artifact: `design.md`. The design must record the chosen coverage and the Interaction Coverage Matrix row.
