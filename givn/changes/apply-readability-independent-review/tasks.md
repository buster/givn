# Tasks: apply-readability-independent-review

Status: one coherent bundle. The five extractions, the two structural touches,
the two rationales, and the gate decisions ship together; the gate's base is
the branch state before the change directory exists, and the source changes
follow in one later commit.

## Bundle

- [ ] Apply the independent-review findings to the branch:
  - [ ] Extract `decide`, `review`, `decide` (findings), `render_json`, and
        `find_block_end` into named helpers without behavior change.
  - [ ] Touch `src/readability/syn_adapter.rs` and `src/skills/targets.rs`
        with the two small helper extractions.
  - [ ] Record the two structural rationales and the review markers.
  - [ ] COMMIT: pending

## Evidence

- `cargo fmt --all -- --check`
- `cargo test --locked --offline --lib`
- `cargo test --locked --offline --test readability_gate_smoke`
- `cargo clippy --workspace --all-targets --offline -- -D warnings`
- `./run-tests.sh --non-e2e`
- `givn check tasks --change apply-readability-independent-review`
- `givn check review --change apply-readability-independent-review`
