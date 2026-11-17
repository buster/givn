# Instruction: ise

The `ise` addon governs a product that can be rebuilt: its `Specification`
stays declared while the `Source code` behind it is replaceable. This
instruction names the obligations every ise change carries.

**Opt-in.** This instruction applies while `addons.ise` is enabled in
`givn/config.yaml`. `givn instructions ise` refuses by name while the addon is
off.

**Vocabulary.** This instruction uses `Component`, `Port`, contract,
`Specification`, and `Source code` because `ise` requires
`architecture_principles`; that instruction defines them. `spec copy`,
`manifest`, and `checksum` are defined by `docs/reference.md` and the
project glossary. `Build input`, `Neighbor record`, and `Replaced unit` are
defined by the project glossary.

## Commands

Three commands serve the clean room. Each one writes or reads the spec copy; the
rebuild happens in that copy, by the agent, and never in the command.

| Command | What it does |
|---|---|
| `givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Writes the spec copy in which the whole product is rebuilt: every declared source root is removed from it. |
| `givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Writes the spec copy in which one component is rebuilt: only the named paths are removed, and neighbouring components and their contracts stay. |
| `givn ise compare --manifest <file> --copy <dir>` | Reports every difference between the manifest and the copy's Specification files, the checkout's Specification files, and the End-to-end (E2E) tests. |

A refusal prints `ise rebuild-all:`, `ise rebuild-component:`, or
`ise compare:` to match the command that refused.

## The spec copy rule

The spec copy is written with the project's Specification files. During a
rebuild the agent adds files, and those are the rebuilt Source code. The agent
may edit Source code kept because it is the product, but it does not delete any
file the copy was written with. The agent does not modify any Specification file
the copy was written with; adding a file under a declared Specification path is
a Specification change, not the product, and is refused.

A `Dry-Run Ise` trial promotes nothing: the rebuilt work stays in the copy, and
the trial review records that no rebuilt file was promoted.

## The clean-room boundary

The builder reads no product source outside the spec copy and the declared
inputs. Before the rebuild, scan the local package registry's source, cache,
and git stores for a published copy of the product, and quarantine every hit
outside the builder's reach; a store that does not exist has no hit. Record the
scan result in the neighbor record. A published copy the builder can read is not
a clean-room rebuild.

## Obligations

- **Specification completeness** — every behavior, contract, limit, acceptance
  test, and build input the rebuild relies on is declared in
  `givn/specification.yaml` and reachable from the Specification paths. A build
  input is declared under the role that owns its content class: a build manifest
  and a lockfile under `contracts`, a toolchain pin under `limits`, a structural
  template under `structure`, a schema or migration asset under `contracts`, and
  served instruction or skill text under `behavior`. The frozen End-to-end (E2E)
  tests fix the bytes they assert; a structural template or a schema asset fixes
  its content through the contract that names its shape. A build input the
  declaration leaves outside the Specification roles is a `specification defect`,
  and a neighbor when it lies outside the replaced unit; the trial records it
  with its consequence. An asset the rebuild generates inside the replaced unit
  whose bytes no frozen test fixes is an `acceptable undocumented change`, not a
  neighbor. The dependency cache and the installed toolchain are environment
  inputs, never declared files: the lockfile declares the resolved dependencies,
  the toolchain pin declares the toolchain, the trial records how both were
  supplied, and a locked dependency the cache cannot resolve blocks the rebuild;
  the blocked rebuild is a setup failure, not a `Spec gap`, and it is not a
  `specification defect`. A contract the frozen tests rely on enumerates every
  value it fixes. A declared Specification path must survive `givn archive`: a
  change's artifacts move from `givn/changes/<id>/` to `givn/archive/<id>/`, so a
  Specification role names the archived path or another durable path, never an
  active change's artifact. `givn archive` refuses such a declaration before
  moving the change. The spec copy always carries the declaration itself:
  `givn/specification.yaml` is an implicit Specification entry, not a sixth role,
  so a project need not list it and the agent reads the declared roles from the
  copy the selected revision records.
- **two-layer test boundary** — the `@e2e` scenarios stay in the Specification
  under the `behavior` role with their exact titles, and run from the declared
  `e2e` package, which holds the runner and every `@e2e` step definition and
  survives a rebuild. Component scenarios stay in the Specification unchanged;
  only their step definitions and fakes are implementation and may be rewritten
  inside the component boundary.
- **persisted-state continuity** — a replacement preserves persisted state and
  public contracts; a required manual migration is a blocked rebuild.
- **change strategy** — the change records one `change strategy` as a
  `Decides D<n>` entry, with the `Ise Test` mode when it rebuilds.
- **Spec gap classification** — a corrective change records exactly one
  `Spec gap` outcome before any fix: `E2E test defect`, `implementation defect`,
  `specification defect`, or `acceptable undocumented change`.

## Stack mechanics

`givn instructions steps` maps the runner to its stack instruction and
carries the universal rules; `steps-rust` is one worked id. Request the stack
instruction `steps` names for the runner in hand.

An ise change carries three records:

| Record | Home | Content |
|---|---|---|
| `change strategy` | the design's `## Decisions` ledger | one `Decides D<n>` entry naming `incremental modification`, `Constant Ise`, or `whole-product rebuild`, the `Ise Test` mode for a rebuild, the affected consumers, the persisted state, and the rebuild boundary |
| `Spec gap` | the corrective change's proposal | exactly one outcome and the artifact that is wrong, not who found it |
| `Neighbor record` | the trial review's evidence directory | every file changed outside the declared Specification or the replaced unit, and every consolidation, each with its reason; the trial review records the neighbor record |

A consolidation merges or splits rebuilt paths; it is recorded with its reason
even when no file lies outside the Specification. A rebuild with no neighbor
records an empty table.

Only `specification defect` is a gap in the Specification; an
`E2E test defect` is a gap in the test, an `implementation defect` is a gap in
the build, and an `acceptable undocumented change` is no gap at all.

The rules for writing each record are served by
`givn instructions design --change <id>` and
`givn instructions proposal --change <id>`; this instruction names the
obligation, not a second copy of the rules.
