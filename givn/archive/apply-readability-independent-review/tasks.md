# Tasks: apply-readability-independent-review

Status: one coherent bundle. The five extractions, the two structural touches,
the two rationales, and the gate decisions ship together. The independent
reviewer rejected the `syn` adapter rationale, so a later commit splits that
module into a directory and removes the finding. The gate's base is the branch
state before the change directory exists.

## Bundle

- [x] Apply the independent-review findings to the branch:
  - [x] Extract `decide`, `review`, `decide` (findings), `render_json`, and
        `find_block_end` into named helpers without behavior change.
  - [x] Repair the stale validator fixture and extract `validate_text` so the
        touched `rationale.rs` unit is fixed rather than newly justified.
  - [x] Touch `src/readability/syn_adapter.rs` and `src/skills/targets.rs`
        with the two small helper extractions.
  - [x] Record the two structural rationales and the review markers.
  - [x] COMMIT: da22c811617332c09464797f86b4926bbf660bd8
- [x] Record the applied review decisions and the rationale evidence:
  - [x] COMMIT: 52dec378
- [x] Split the `syn` adapter after the reviewer rejected its structural
      rationale:
  - [x] Move `mod.rs`, `impl_index.rs`, `file_builder.rs`, `facts.rs`,
        `spans.rs`, and `syntax.rs` under `src/readability/syn_adapter/`;
        keep `PendingImpl` and `ImplRecord` syn-free and all parser types
        inside the directory.
  - [x] Re-run the gate; only the target registry keeps a rationale.
  - [x] COMMIT: 1681134b
- [x] Record the 12-row arc42 assessment and correct the adapter path wording:
  - [x] COMMIT: bc249497

## Evidence

- `cargo fmt --all -- --check` — clean.
- `cargo test --locked --offline --lib` — 328 passed.
- `cargo test --locked --offline --test readability_gate_smoke` — 19 passed.
- `cargo clippy --workspace --all-targets --offline -- -D warnings` — clean.
- `./run-tests.sh --non-e2e` — the change delta passes; receipt recorded at
  `1681134b`.
- `givn check tasks --change apply-readability-independent-review` — passed.
- `givn check review --change apply-readability-independent-review` —
  `readability review: PASS` (52 touched units, 8 findings, 7 `FIXED`, 1
  `JUSTIFIED`).
