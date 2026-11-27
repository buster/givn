# Working-tree ship probe

Scratch project: `/tmp/ise-ship-probe`, Git, `ise` effective, `givn/specification.yaml` declaring `behavior`, `structure`, `contracts`, `limits`, `e2e`, and `source: [src/**]`.

## Sequence

| Step | Observed state |
|---|---|
| Previous Component recorded | `079acba record the previous parser`; `git show 079acba:src/parser.rs` holds the old `parse` and its unit test. |
| Active Specification written | `givn new ship-probe`; the delta at `givn/changes/ship-probe/specs/demo/parse.feature` adds the hexadecimal scenario and keeps the decimal scenario. |
| Old implementation removed | `src/parser.rs` deleted from the working tree; `src/lib.rs` still names `pub mod parser`, so the tree does not build until the rebuild. |
| Rebuilt in place | `src/parser.rs` rewritten from the delta with `0x` support and a new coupled unit test. |
| Verified | `cargo test`: 2 unit tests and 2 black-box public-API tests passed, including `parse("0x10") == 16` and `parse("42") == 42`. |
| Shipped | `60c22ae ship the rebuilt parser (hex support)`; the previous implementation remains at `079acba`. |

`givn ise rebuild-component` was not invoked. No spec copy, candidate revision, new manifest, or early archive merge was created or required.

## What it proves and does not prove

The replacement's tested behavior passes, and the delta (decimal plus hexadecimal) exercised it. The previous implementation stayed accessible in `079acba`; the coding agent was instructed not to consult it. This does not prove that the Specification alone was sufficient.

The probe does not run a Givn archive. The change's own `givn check tasks` runs the delta scenarios that assert the served guidance, and its archive merges that delta once.
