---
status: accepted
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - semantic output contracts and feature matrix
informed:
  - CLI and machine-readable output consumers
---

# Version opt-in second-stage evidence output

## Qualification

- [x] **Hard to reverse** - output fields and schema versions are consumed by agents and scripts.
- [x] **Shapes quality attributes** - explicit fallback and provenance protect review integrity.
- [x] **Structural** - E5-only, BGE, and NLI output modes form separate interface contracts.
- [x] **Must be shared** - consumers must know when optional evidence participates.

## Context and Problem Statement

Existing E5-only text, JSON, and semantic report consumers have fixed fields. Adding BGE or NLI fields invisibly would break parsers and blur unavailable versus disabled behavior.

## Decision Drivers

- preserve E5-only output
- make participation and runtime metadata explicit
- isolate independent capability flags
- retain future extensibility

## Considered Options

- Change every existing E5-only format in place.
- Add unversioned optional fields to all formats.
- Keep E5-only formats and version opt-in second-stage structures.

## Decision Outcome

Chosen option: **Keep E5-only contracts stable and add explicit versioned structures only when BGE or NLI participates**. Run summaries identify raw, recommended, filtered, unscored, unavailable, model, and runtime values. NLI is selected through its independent feature and selector; it is not implied by BGE.

### Consequences

- Consumers opting into a second-stage layer must handle the participating schema.
- Disabled layers emit no evidence object; unavailable selected layers expose fallback state.
- The active implementation has separate report schema versions for E5, BGE, and NLI modes.

### Confirmation

Feature and JSON scenarios assert E5-only compatibility, opt-in fields, explicit counts, and absence of NLI when only BGE is active.

## Pros and Cons of the Options

### Versioned opt-in output

- Good, because compatibility and participation are explicit.
- Bad, because renderers and consumers need mode-specific cases.

### In-place output change

- Good, because the renderer has fewer branches.
- Bad, because existing machine consumers break silently.

### Unversioned optional fields

- Good, because the change appears small.
- Bad, because absent, disabled, and unavailable states become ambiguous.

## More Information

The NLI capability is [ADR-058](ADR-058-nli-evidence.md); feature independence is [ADR-055](ADR-055-independent-semantic-features.md).
