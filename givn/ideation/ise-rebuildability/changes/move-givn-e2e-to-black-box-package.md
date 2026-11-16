# Change seed: move-givn-e2e-to-black-box-package

Status: implemented in move-givn-e2e-to-black-box-package

- Use-case IDs: `verify-change` (`givn-workflow-enforcement`) and
  `obtain-effective-guidance` (`serve-extended-rules`, consumed by the served
  policy that names the declared package)
- Topic: `ise-rebuildability`
- Personas: `agent--technical-role`, `givn-maintainer--owner`
- Suggested start: 4; depends on `enforce-ise-e2e-boundary` for the blocking
  package gate and the E2E identity
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

Givn's `@e2e` step definitions run from a package that links no production
crate, and `verify.e2e_command` runs that package. Every scenario keeps its
wording; a scenario that can only run in process is retagged `@component`
with a recorded reason, and each user action stays covered by a black-box
scenario. The package adds a workspace member that ships nothing in the
published crate.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `verify-change` | `givn-workflow-enforcement` | `cargo tree -p givn-e2e` shows no `givn` dependency, `verify.e2e_command` names the new package, and the boundary gate passes on a fixture project whose change lock holds `ise: true`. |
| `obtain-effective-guidance` | `serve-extended-rules` | The served guidance for Givn's changes names the new package as the runner target; this seed adds no instruction text and consumes the policy text delivered by `enforce-ise-e2e-boundary`. |

## Accepted decisions

### `S13` Change A and `Q5`: a dependency-free `e2e` workspace member

- Context: `plan.md` `S13` Change A moves Givn's `@e2e` step definitions out
  of the target that links `givn`. The declared package must have no manifest
  edge into `source: ["src/**", "tests/**"]`, while the feature files and the
  component runner stay where they are.
- Options:
  1. **A new `e2e/` workspace member**: In practice: `e2e/` holds its own
     manifest, the runner, every `@e2e` step definition, and the browser
     assets; `e2e/**` joins the package `exclude` list. Gain: the boundary is
     provable with `cargo tree -p givn-e2e`, and the package ships nothing.
     Cost: a second manifest and a workspace member to maintain.
  2. **A consumer artifact**: In practice: the step definitions keep compiling
     against a published `givn` API while the runner drives the installed
     binary. Gain: one manifest. Cost: the package still has a dependency edge
     into `source`, so the blocking gate refuses it.
  3. **Retag every `@e2e` scenario instead of moving it**: In practice: the
     suite becomes `@component` and no separate package exists. Gain: no
     restructure. Cost: the project has no black-box acceptance authority,
     which contradicts the settled package boundary and `S13`'s exit
     criterion.
- Chosen: **A new `e2e/` workspace member** because the gate reads the
  manifest graph, and a workspace member excluded from the published crate
  keeps `TC-2` and `TC-3` true. Rejected: the consumer artifact because the
  dependency edge remains; retagging every scenario because it removes the
  black-box suite.

### `S13` Change A: move or retag, never weaken

- Context: 108 `@e2e` scenarios in 51 feature files use phrases from 22 step
  files, and some step definitions can only run in process. The move must
  keep the suite's assertions and account for every scenario.
- Options:
  1. **Move what can drive the installed binary; retag the rest with a
     reason**: In practice: a step definition that only calls in-process APIs
     is replaced by one that drives the installed binary where possible; a
     scenario that cannot follow is retagged `@component` with a recorded
     reason, and its user action keeps a black-box scenario. Gain: every
     scenario is accounted for and the assertions stay. Cost: an inventory
     and a scenario-by-scenario review.
  2. **Retag every scenario that uses an in-process step**: In practice: the
     whole affected subset becomes `@component`. Gain: the fastest way to a
     clean manifest. Cost: user actions lose their black-box coverage, and
     the same cause is recorded many times.
  3. **Keep an in-process target for the subset**: In practice: two runners,
     one in the `givn`-linked target. Gain: no scenario changes. Cost: the
     frozen set is no longer one package, so the receipt cannot name the
     protected suite.
- Chosen: **Move or retag with a recorded reason** because `S13` requires all
  108 scenarios accounted for and every retag justified, and the policy
  forbids weakening an assertion. Rejected: retagging every in-process step
  because it drops black-box coverage; a second in-process target because the
  frozen set must be one package.

## Open questions

### Which recorded evidence satisfies the external-evidence prerequisite?

- Blocks: this seed's start and any claim that moving Givn's suite validates
  the Ise protocol.
- Context: `plan.md` `S7` to `S12` supply the Catopia and Passuff evidence.
  This seed takes that evidence as a prerequisite for starting: a rebuild
  started before it exists would test an unvalidated protocol on its own
  authoring tool, and a failure would have no earlier trial to separate a
  tool defect from a protocol defect. The topic did not settle which records
  are read to confirm the prerequisite.
- Question: Which recorded evidence must exist before this seed starts?
- Options:
  1. **Every planned run in `S7` to `S12`, with its evidence**: In practice:
     the Catopia E1 to E4 results and the Passuff package, Constant Ise, and
     full-product results are recorded. Gain: the protocol is validated across
     a controlled campaign and a product. Cost: the seed waits for the slowest
     phase.
  2. **The two frozen-suite results only (`S8`, `S12`)**: In practice: a cold
     rebuild and a full-product rebuild passed or were fully classified.
     Gain: the seed waits on the two results that exercise the same guarantee.
     Cost: Constant Ise (`S9`, `S11`) and the state checks in `S9` E4 stay
     unproven.
  3. **The last full-product trial only (`S12`)**: In practice: one complete
     product rebuilt from its own corpus. Gain: the earliest defensible start.
     Cost: no campaign evidence, so a protocol defect found on Givn has one
     prior data point.
- Example: `S12` records a green Passuff full-product trial while the Catopia
  `S8` cold rebuild is still unrun. Option 3 starts; option 1 waits; option 2
  waits because one of its two results is missing.
