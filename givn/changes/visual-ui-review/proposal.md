# Proposal: visual-ui-review

## Use-Case Context

- Use-case ID: author-change, verify-change
- Ideation topic: none
- Confirmed Personas: none

## Problem / Opportunity

A change can touch a user-visible interface and pass every gate without any
visual design existing. Givn checks structure, scenarios, coverage, and
architecture, but nothing checks what the consumer actually sees.

Concrete evidence: the first web interface of a project built through this
workflow renders raw browser defaults. Internal state names such as
`PausedForQuestion` appear as user text. Form controls have no labels. Raw
protocol payloads and commit hashes are the main page content. Project names
are rendered twice on every screen. The error page is one unstyled sentence
with no title and no way back. All of this is visible in a single screenshot,
and none of it blocked review.

Two root causes. First, review has no artifact to reject against: without an
approved visual style there is no reference, so "undesigned" and "unusable"
are not findings. Second, review has no visual evidence: no screenshot is
required, so no reviewer — human or agent — ever looks at the result. The
same gap exists for terminal interfaces, where output formatting, labels, and
language go unchecked.

## Proposed Solution

For a change that introduces or visibly changes a user interface:

1. Before implementation, the design contains an approved visual style
   contract: the style rules (color roles, spacing, typography, component
   states) and the screens it covers, with at least one rendered reference
   view. When the user supplies no design, the authoring agent proposes
   concrete rendered options and the user picks one. The contract is
   style-level, not pixel-level: it follows a durable project design system
   and accepts improvements that stay within the style.
2. Two default viewports apply: mobile in the Samsung S24 class and desktop
   in Full HD. The interface must be usable at both.
3. Every end-to-end scenario of a visual capability produces a committed
   final-state screenshot per viewport as review evidence. Captures taken
   while implementing show intermediate states but are not review evidence.
4. A reusable baseline check runs at every assertion: every control is
   labeled, no internal identifier or state name appears in visible text,
   the page has a real title and heading, no visible content is duplicated,
   the error state has a heading and a way back, the configured language is
   used, and reduced-motion is honored. These checks are ordinary scenarios,
   not a separate tool.
5. Review ends with a visual verdict. It confirms the evidence is complete,
   the baseline holds, and one fresh-context vision review per confirmed
   Persona judges whether that persona can reach their goal with what they
   see. A change cannot pass review without the verdict. A style break is
   fixed, or the contract is updated in the same change; a taste note without
   a failed persona goal is recorded but does not block. When the active
   harness cannot process images, the verdict is explicitly unverified and a
   human must resolve it; it never silently passes.
6. A terminal interface produces a captured transcript per end-to-end
   scenario instead of screenshots. API-only capabilities are exempt.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| visual-design-contract | none of the ranked candidates matches (top: deduplicate-change-dir-detection, 7.00) | `NEW in author-change` | No existing capability owns a visual style contract, viewport rules, or the design-system home. The ranked candidates match only on the words "givn check". |
| visual-review-gate | review-disposition-gates (verify-change), 7.00 | `NEW in verify-change` | The verdict is a new gate family: evidence completeness plus vision persona review plus a machine marker. Extending review-disposition-gates would mix overlap dispositions with visual review. |
| prose-consolidation | not surfaced | `EXTEND prose-consolidation` | The canonical E2E policy in the specs instruction gains the visual baseline and evidence rules; the existing capability owns that policy text. |

## Out of Scope

- Pixel-perfect comparison and image diffing.
- Automated review of animation and motion; only reduced-motion support is
  checked. Motion-heavy changes may add human-reviewed video.
- A prescribed CSS framework, component library, or visual style.
- Authoring the durable design system content for a project; givn defines
  where it lives and when it must be updated.

## Open Questions

None. The harness-without-vision case is settled: the verdict is recorded as
unverified and a human resolves it, never a silent pass.
