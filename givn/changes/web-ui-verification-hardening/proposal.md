# Proposal: web-ui-verification-hardening

## Use-Case Context

- Use-case ID: author-change, verify-change
- Ideation topic: none
- Confirmed Personas: none — both owning use cases declare `Personas: none`,
  and no persona was invented for this change.

## Problem / Opportunity

A real dogfooding run exposed an escape hatch in the verification discipline.
The authoring agent classified a browser-rendered page as an "HTTP-served
static document with no controls" and used a plain HTTP request as the
end-to-end driver. The review confirmed the classification. The observable
consequences:

- No browser drove the page, so rendering, dynamic values, and user
  interaction were never exercised end to end.
- No screenshot exists, so nobody saw what the consumer saw.
- No persona was considered, so no stakeholder lens judged the result.
- The project's "no dependencies" constraint was treated as a reason to defer
  the browser check instead of a conflict the operator must decide.

The guidance already forbids an HTTP client as a Web UI driver, but it never
defines when a capability *is* a Web UI. It also leaves the evidence scope,
the persona disposition, and constraint conflicts under-specified, so each
one can be waved through at review time. This change closes those openings
fail-closed.

## Proposed Solution

Observable behaviour:

1. Requested design guidance states that a capability whose interface renders
   HTML/CSS/JS in a browser is a Web UI even when it needs only a page
   request and exposes no controls; for such a capability a real browser
   driver (the default is fixed to one named driver) is mandatory, and an
   HTTP client is not a valid driver.
2. Requested design guidance requires the interaction coverage matrix to name
   a concrete browser driver and the action it performs for every Web UI row.
3. Requested end-to-end guidance requires at least one assertion on rendered
   state — a value produced by rendering that changes between two
   observations — in addition to the successful document delivery check.
4. Requested visual-evidence guidance requires one screenshot per end-to-end
   scenario, per declared viewport, plus one per UI state the scenario
   traverses (empty, loading, error); the review names the screenshots it
   saw.
5. Requested design guidance requires the design-system section to name
   breakpoints and accessibility checks (contrast, focus, semantics) with how
   each is verified.
6. Requested proposal guidance requires the operator to confirm the affected
   personas or an explicit, reasoned "none". Requested design-review guidance
   records, per persona, the relevance and the evidence; requested review
   guidance records the disposition.
7. Requested review guidance requires the reviewer to test the opposite
   classification (a rendered page is a Web UI) before accepting a non-browser
   classification, and to fail a change whose mandatory check was deferred
   without a recorded operator decision.
8. Requested design-review and review guidance require an operator question
   before any mandatory check may be deferred because of a project
   constraint; a deferral without the operator's recorded decision fails the
   review.
9. The review gate requires the state screenshots declared by the contract,
   in addition to the existing per-viewport final-state evidence.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| prose-consolidation | EXTEND (advisory ranking inconclusive: top candidate `archive-proof` at 3.00 with no content signal) | `EXTEND prose-consolidation` | The specs instruction owns the canonical E2E policy; the classification rule, the rendered-state assertion, and the state evidence rule are policy text it emits. |
| visual-design-contract | EXTEND (same inconclusive ranking) | `EXTEND visual-design-contract` | The design and design-review guidance (classification, default driver, design system, persona impact, deferral) are owned by the visual contract capability. |
| visual-review-gate | EXTEND (same inconclusive ranking) | `EXTEND visual-review-gate` | The review guidance and the per-viewport evidence gate are owned there; the state evidence extends the same gate. |

## Out of Scope

- No new browser framework, runtime, or dependency is added to givn itself;
  the driver belongs to the consuming project.
- The style-level (not pixel-level) nature of the visual design contract is
  unchanged.
- Migration, archive, and overlap gates are unchanged.
- No personas are invented for givn's own use cases.

## Open Questions

None. The four open decisions were taken with the operator:

- Driver: Playwright default, version resolved per change with a recorded
  source, no central pin.
- Screenshots: per scenario × viewport plus per traversed state.
- Persona lifecycle: the operator confirms per change; design-review records
  relevance and evidence per persona; review records the disposition;
  personas are updated after each review they take part in.
- "none": allowed only when the change has no user-visible surface (browser,
  terminal, CLI text) and must be reasoned and operator-signed.
