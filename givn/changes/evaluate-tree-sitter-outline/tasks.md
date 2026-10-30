# Tasks: evaluate-tree-sitter-outline

Owning use case: `assess-code-structure`. Personas: `developer--primary-user`,
`agent--technical-role`, `givn-maintainer--owner`.

Release note: Outline analysis now records a differential Tree-sitter evaluation for Rust and Python and retains the current adapters where parity is incomplete.

## Setup

- [x] Confirm the existing runner and strict mode; no runner change.
      Evidence: `tests/features_runner.rs` calls `.fail_on_skipped()`.
- [x] Confirm Tree-sitter is a dev dependency only and the release binary does
      not link it. Evidence: `Cargo.toml` `[dev-dependencies]`.
- [x] Confirm the evaluation unit tests run:
      `cargo test --locked --lib tree_sitter_eval`.

## Batch A — differential evaluation and default-engine regression

Design constraints:

- Add `tree-sitter 0.27.1`, `tree-sitter-rust 0.24.2`, and
  `tree-sitter-python 0.25.0` as dev dependencies.
- Add `src/outline/tree_sitter_eval/` under `#[cfg(test)]`.
- Extract units, methods, fields, calls, spans, unsupported constructs, and
  parse errors with the grammars.
- Compare against `outline_analyzer()` and classify each category as
  `equivalent`, `extra_syntax`, or `missing_required`.
- A `missing_required` category yields `RETAIN`; malformed input is recorded as
  an error and never produces partial facts.
- Add `docs/outline-tree-sitter-evaluation.md` with the matrix, gaps, decision,
  and distribution note. The decision-record unit test reads it.
- Add the regression scenario to
  `specs/assess-code-structure/outline-report.feature`: the default JSON report
  still uses schema 3 and names Rust/Python.
- No production outline module, command, schema, or default engine changes.

### Scenario: The Tree-sitter evaluation leaves the current outline engine in place

- [x] Scenario: The Tree-sitter evaluation leaves the current outline engine in place
  - [x] COMMIT: 1477d19f -- Batch: tree-sitter-evaluation

## Batch B — documentation closeout

- [ ] Update `docs/arc42/08-crosscutting-concepts.md` 8.26,
      `docs/arc42/11-risks-and-technical-debt.md` R-130/R-131, and
      `docs/glossary.md`.
- [ ] Confirm every delta step phrase has a permanent binding and no
      unimplemented stub remains.
- [ ] COMMIT: <hash> -- Batch: tree-sitter-documentation-closeout
