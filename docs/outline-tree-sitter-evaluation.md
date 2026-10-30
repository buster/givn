# Outline Tree-sitter Evaluation

## Scope

This evaluation asks whether Tree-sitter preserves the outline facts Givn needs
for Rust and Python. It does not switch the default outline engine, replace the
`syn` or Python `ast` adapters, or add a new language. Tree-sitter is a
test-only dev dependency in this release.

The comparison uses a small representative fixture containing a Rust struct,
inherent methods, a trait implementation, fields, and calls, plus a Python class
with methods, a field assignment, and a call. The test asserts that every
required category is present and that the current adapter remains the default.

## Method

`src/outline/tree_sitter_eval/` parses the same production sources with
`tree-sitter`, `tree-sitter-rust`, and `tree-sitter-python`. It extracts:

- source units and their kinds;
- methods and their owning units;
- fields;
- calls and their owning methods;
- declaration spans;
- unsupported constructs and parse errors.

The current `outline_analyzer()` result is projected onto the same keys. Each
category is classified as `equivalent`, `extra_syntax`, or
`missing_required`. A `missing_required` classification means Tree-sitter did
not reproduce a fact the current outline needs.

## Result

The fixture run produced:

| Category | Current | Tree-sitter | Missing required | Extra syntax | Classification |
|---|---:|---:|---:|---:|---|
| units | 3 | 8 | 1 | 6 | missing_required |
| methods | 6 | 4 | 2 | 0 | missing_required |
| fields | 2 | 2 | 0 | 0 | equivalent |
| calls | 6 | 2 | 4 | 0 | missing_required |
| spans | 8 | 8 | 5 | 5 | missing_required |
| unsupported | 0 | 0 | 0 | 0 | equivalent |

The exact counts are fixture-specific. The decision rests on the classified
gaps, not on the counts alone.

Observed gaps:

- **Units:** the minimal adapter does not reproduce the current module and
  free-function ownership model, and it adds syntax-level units the neutral
  report does not use.
- **Methods:** Python and Rust ownership differs for methods that the current
  adapters attach to a unit or aggregate across files.
- **Calls:** the minimal adapter extracts fewer calls and has no cross-file
  resolution; a name-only match must remain uncertain rather than becoming a
  nested target.
- **Spans:** syntax-node spans are not yet normalized to the declaration and
  unit spans the report needs.
- **Fields:** the fixture's field facts match.

## Decision

Decision: RETAIN the current Rust and Python adapters for 0.9.14.

The Tree-sitter evaluation does not demonstrate feature parity. No default
engine switch, no removal of `python3`, and no additional language is part of
this release. The test-only adapter remains evidence for a later change.

This is not a rejection of Tree-sitter. It records that the current adapters
still supply facts the minimal Tree-sitter mapping does not, and that the
replacement decision needs a more complete mapping and a representative corpus.

## Distribution and maintenance

Tree-sitter and both grammars are dev dependencies. The production binary does
not link them, so release binary size is unchanged. Running the evaluation tests
requires the Rust build toolchain for the grammars' C sources; ordinary
`givn outline report` does not.

Malformed source is recorded as a parse error by the evaluation. The current
file-level failure boundary remains: no partial outline is claimed from an
error tree.

## Follow-up

A later change may replace or retain each current adapter after a fuller
mapping and a broader differential corpus. Additional languages remain out of
scope until that decision is recorded.
