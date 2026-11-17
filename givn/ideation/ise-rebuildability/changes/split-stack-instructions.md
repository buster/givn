# Change seed: split-stack-instructions

Status: implemented in split-stack-instructions

- Use-case IDs: `obtain-effective-guidance` (`serve-extended-rules`) and
  fragment `support` (`instruction-integrity`)
- Topic: `ise-rebuildability`
- Personas: `agent--technical-role`, `givn-maintainer--owner`
- Suggested start: 2; depends on `add-ise-addon-and-specification`; may run
  in parallel with `enforce-ise-e2e-boundary`
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

Core step instructions stop growing because stack mechanics are served on
request by id. Each core step instruction (`specs`, `steps`, `design`, and
`review`) carries its universal rules and names each stack instruction id with
the condition for loading it; the stack instruction body is served only by a
request for its own id. A named stack instruction id that does not resolve is
a named defect of the guidance corpus, not a silent omission.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `obtain-effective-guidance` | `serve-extended-rules` | A core step instruction names the stack instruction id and its load condition and carries no stack body; a request for the stack instruction id serves the body; an unknown id still refuses with the known list; text and JSON carry the same content. |
| `support` fragment | `instruction-integrity` | The reference check resolves every instruction id the core guidance names; an unresolved named id fails the check by name. The `ise` obligations and the design and review record requirements are delivered by `add-ise-addon-and-specification`. |

## Accepted decisions

### Q15: No new term; `Instruction` plus the id convention

- Context: The core step instructions were growing with per-stack mechanics.
  The choice was a new noun for the on-demand instruction or the existing
  `Instruction` type plus the id naming convention.
- Options:
  1. **`Stack recipe`**: In practice: `givn instructions steps-rust` serves
     the recipe. Gain: two words that name the content and its scope, and the
     canvas already used them. Cost: it asserts a prescribed procedure while
     the asset is guidance; `stack` excludes `dialog-examples`; it has no Ise
     counterpart.
  2. **`Companion instruction`**: In practice: `C: request the companion
     instruction`. Gain: the genus is `Instruction`, and it covers the
     existing `dialog-examples` precedent. Cost: it names the relation, not
     the content; `companion` also names migration-bundle assets, so the
     compound must stay qualified.
  3. **`Mechanism sheet`**: In practice: `C: request the mechanism sheet`.
     Gain: names the content without asserting a procedure. Cost: `sheet`
     reads as a reference table, not prose guidance; no Givn or Ise
     precedent.
  4. **No new term**: In practice: sentences say "the Rust instruction" or
     name `steps-rust`; the pairing convention (`steps` to `steps-rust`,
     `dialog` to `dialog-examples`) carries the relation. Gain: no coinage and
     no new glossary row. Cost: no short handle, so every sentence names the
     id or says "the stack instruction".
- Chosen: **No new term**. Rejected: `Stack recipe` because it asserts a
  procedure and excludes `dialog-examples`; `Companion instruction` because it
  names the relation; `Mechanism sheet` because it reads as a table.

## Open questions

### Which stack instructions ship in the first split?

- Blocks: `S5`'s content split and the instruction line-budget gate (`G2`).
- Context: Q15 settled the mechanism and the id convention. It did not settle
  which stack bodies leave the core instructions in the first cut. The seed's
  guarantee is about growth, not about moving every existing body at once.
- Question: Which stacks receive a separate instruction in this seed?
- Options:
  1. **Rust only**: In practice: `steps-rust` plus the existing
     `dialog-examples` precedent. Gain: the smallest split, and it matches the
     Rust pilots (`givn`, Passuff, Catopia). Cost: other stack bodies remain
     in the core instructions, so the first cut proves the rule without
     relieving their budget.
  2. **Every stack the core instructions currently carry**: In practice:
     inventory each per-stack body and move it to `<id>-<stack>`. Gain: the
     immediate line-budget relief G2 measures. Cost: a large content move that
     can drop a universal rule, and the inventory must name every body.
  3. **Every stack the outline supports**: In practice: Rust, Python, Java,
     TypeScript, and JavaScript instructions. Gain: one uniform set. Cost:
     instructions for stacks no current project uses, and a larger surface to
     keep true.
- Example: a Python project requests `givn instructions steps`. With Rust
  only, the Python runner mechanics remain in the core text; with option 2 or
  3, the core text names `steps-python` and the body arrives only on that
  request.
