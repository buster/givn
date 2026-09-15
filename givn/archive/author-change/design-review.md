# Design Review: author-change

## Grilling

The grilling was performed in-session against the recorded mechanics (per the
operator's standing assumption directive, A3): the seam already exposes
stage/commit primitives on both backends, the hook's stamping is idempotent
(`stamp_message` replaces managed trailers), and `fossil add` on an unchanged
managed file is a `SKIP` no-op, so wrapper re-runs cannot corrupt history.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| F1 | Blocker | `managed_generated_paths` includes `.githooks/commit-msg`, which a Fossil project will not have once `install` becomes backend-aware; allowlist staging on Fossil would fail on the missing path. | `FossilCheckout::stage_paths` skips paths that do not exist in the checkout (nothing to record); the seed-3 Fossil fixtures still carry the file, so the migration e2e is unaffected. |
| F2 | Blocker | Double stamping on Git when the chained hook runs after `givn commit` already stamped. | Stamping replaces managed trailers (idempotent by construction); verified against `stamp_message_values`' filter. |
| S3 | Should-fix | The seed's open question (register vs fold `llm-attribution-installation`). | Registered in `givn/specs/fragments/fragment.md` (narrative delta with base hash); the capability owns a permanent feature and inventory rows. |
| N4 | Note | The wrapper's no-context gate must match the hook's exactly. | Both call the same `GIVN_ATTRIBUTION_ACTIVE` gate via `attribution::stamped_message`. |
| N5 | Note | `givn commit` on a Fossil checkout runs `before-commit` hooks; a rejection propagates as a failed revision. | Accepted: the same hook-rejection semantics as the archive (unproven/repairable). |

## Branch dispositions

- **Scope** — the delta covers the seed's scope note: backend-aware init
  report and warning, conditional plumbing, guidance note, and the wrapper;
  no run-command or archive scope leaks.
- **Tech choices** — the wrapper reuses the seam primitives; no new backend
  surface beyond `stage_all`/`record_all_changes`.
- **Missing scenarios** — the wrapper's no-context variant is added; raw
  Fossil commits without attribution are out of scope by the use case.
- **Testability** — all six scenarios assert concrete observables (message
  content, report lines, generated file content, refusal wording).
- **Risk** — the most likely failure is the missing-path staging (F1),
  removed by the skip rule.
- **Personas** — the three confirmed Personas are review lenses; no
  deferrals; no visual contract (CLI text).
- **Interaction coverage** — the two new init e2e rows match their scenarios;
  the wrapper scenarios are regular variants of the existing attributed-
  revision row.
- **ADR qualification** — no new ADR: `givn commit` is observable behavior
  plus a seam method reuse (Gherkin/design own it).
- **Arc42** — affected chapters: 02 (TC-27/TC-28 attribution wording), 03
  (Fossil CLI entity gains the wrapper), 05 (new `cli::commit` row), 06
  (attribution flow), 08 (8.x attribution), 09 (no change), 12 (commit
  attribution entry already names the givn-commit path).
- **Ubiquitous language** — revision, checkout, attribution trailer,
  installation report are glossary terms used consistently.

DESIGN-REVIEW: PASS
