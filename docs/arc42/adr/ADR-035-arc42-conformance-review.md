---
status: accepted
date: 2026-08-10
decision-makers:
  - givn maintainers
consulted:
  - Arc42 lifecycle assets and integrity checks
informed:
  - users enabling the Arc42 addon
---

# Review Arc42 conformance after implementation

## Qualification

- [x] **Hard to reverse** - omitting a post-implementation architecture check allows stale durable facts to accumulate across changes.
- [x] **Shapes quality attributes** - the review protects architecture consistency and maintainability.
- [x] **Structural** - it defines a conformance checkpoint between durable docs, change assessment, design, tasks, and implementation.
- [x] **Must be shared** - Arc42-enabled changes need the same evidence expectations across review entry points.

## Context and Problem Statement

Design review checks the planned impact before tasks, but implementation can drift and durable chapters can remain unchanged. A text-only automatic source parser cannot reliably judge architecture tradeoffs.

## Decision Drivers

- catch design-to-implementation drift
- catch stale durable chapters
- retain language and runner neutrality
- require concrete human evidence

## Considered Options

- Use a prompt-layer evidence review plus narrow mechanical checks.
- Build an automatic source parser and architecture fact checker.
- Keep only the pre-task design review.

## Decision Outcome

Chosen option: **Use a prompt-layer, evidence-based completion review when Arc42 is enabled**. The reviewer compares durable chapters and the change-level assessment with `design.md`, mapped `tasks.md`, and implementation evidence. The mandatory integrity path also checks that resolvable affected chapter files differ from the embedded templates; an unresolvable chapter row is currently skipped. This is not a semantic source-code inference engine.

### Consequences

- `ARC42 CONFORMANCE: CLEAN` and `REVIEW: PASS` require reviewer evidence.
- When Arc42 is disabled, the conformance section is not applicable.
- A change-level `arc42.md` file is tracked by the lifecycle; durable chapters remain in `docs/arc42/`.

### Confirmation

Arc42 alignment scenarios assert disabled behavior, affected-chapter checks, required evidence, and generated guidance parity.

## Pros and Cons of the Options

### Evidence review plus narrow checks

- Good, because humans can judge context while code verifies basic omissions.
- Bad, because reviewer discipline remains a residual risk.

### Automatic source parser

- Good, because some facts could be checked mechanically.
- Bad, because names and text cannot establish architecture intent across languages.

### Pre-task review only

- Good, because it adds no post-implementation work.
- Bad, because it cannot detect implementation drift or stale durable chapters.

## More Information

See [chapter 8.14](../08-crosscutting-concepts.md#814-arc42-implementation-conformance-review) and [ADR-026](ADR-026-arc42-addon.md).
