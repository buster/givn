---
status: accepted
date: 2026-08-20
decision-makers:
  - givn maintainers
consulted:
  - compact serialization implementation and scenarios
informed:
  - retrieval maintainers and agents consuming semantic evidence
---

# Omit empty optional content from semantic passages

## Qualification

- [x] **Hard to reverse** - changing passage content changes hashes, vector reuse, and every semantic result.
- [x] **Shapes quality attributes** - omission reduces shared boilerplate, token use, and false similarity.
- [x] **Structural** - the rule is part of the shared serializer input boundary.
- [x] **Must be shared** - every semantic model must receive the same empty-field behavior.

## Context and Problem Statement

The complete serializer initially emitted empty markers for absent optional fields. Those markers carry no domain meaning, consume token budget, and can raise similarity through shared boilerplate.

## Decision Drivers

- retain every non-empty value
- remove synthetic empty markers
- preserve deterministic order and hash invalidation
- avoid changing thresholds or downstream statuses

## Considered Options

- Emit fixed empty markers for every optional field.
- Remove optional fields even when they contain content.
- Emit optional sections and fields only when normalized content is non-empty.

## Decision Outcome

Chosen option: **Omit optional sections and fields whose normalized content is empty**. Feature and scenario identity plus non-empty behavior steps remain; non-empty descriptions, backgrounds, tables, docstrings, outline templates, and examples remain in stable order. The resulting exact passage remains the E5/BGE/NLI input and E5 `content_hash` source.

### Consequences

- Existing entries are re-embedded through the normal content-hash mismatch path.
- Model text has no fixed empty-field markers; absence must not be confused with empty domain content.
- Report metadata and Fingerprint evidence remain separate.

### Confirmation

Compact serializer tests assert marker absence, preservation of each non-empty optional value, deterministic output, and fitting index refresh.

## Pros and Cons of the Options

### Omit only empty optional content

- Good, because complete meaning is retained without synthetic tokens.
- Bad, because consumers must handle absent optional sections explicitly.

### Fixed empty markers

- Good, because the text shape looks uniform.
- Bad, because repeated boilerplate increases token use and similarity noise.

### Remove all optional content

- Good, because passages are short.
- Bad, because meaningful context is lost.

## More Information

This amends the serialization detail in [ADR-059](ADR-059-complete-semantic-serialization.md) and is used by [ADR-060](ADR-060-actual-tokenizer-caps.md).
