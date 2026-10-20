# Tasks: apply-readability-independent-review

Status: one coherent bundle. The five extractions, the two structural touches,
the two rationales, and the gate decisions ship together; the gate's base is
the branch state before the change directory exists, and the source changes
follow in one later commit.

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

## Evidence

- `cargo fmt --all -- --check`
- `cargo test --locked --offline --lib`
- `cargo test --locked --offline --test readability_gate_smoke`
- `cargo clippy --workspace --all-targets --offline -- -D warnings`
- `./run-tests.sh --non-e2e`
- `givn check tasks --change apply-readability-independent-review`
- `givn check review --change apply-readability-independent-review`
