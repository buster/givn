# Instruction: steps-rust

`cucumber-rs` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition file: `tests/steps/<capability>.rs`, one per capability.
- Binding form: `#[given(`, `#[when(`, `#[then(`; one expression per binding.
- Unimplemented body: `unimplemented!()` or `todo!()`.
- Strict mode: `.fail_on_skipped()` on the `Cucumber` builder.
- Single scenario: `cargo test <name> -- --exact`.
