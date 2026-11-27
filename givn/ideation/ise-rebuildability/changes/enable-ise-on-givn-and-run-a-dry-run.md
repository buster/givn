# Change seed: enable-ise-on-givn-and-run-a-dry-run

Status: proposed by enable-ise-and-run-the-givn-self-rebuild

- Use-case IDs: `prepare-an-ise-test` (`clean-room-preparation`,
  `trial-input-verification`), `declare-the-specification`
  (`specification-declaration`), and `create-change` (`change-strategy`,
  `spec-gap`)
- Topic: `ise-rebuildability`
- Personas: `agent--technical-role`, `developer--primary-user`,
  `givn-maintainer--owner`
- Suggested start: 5; depends on `add-ise-clean-room-commands` for preparation
  and verification, and `move-givn-e2e-to-black-box-package` for the frozen
  `e2e` package
- Handoff: seed only; `/givn-propose` not started (2026-11-14)

## Guarantee

Givn declares its own Specification set, enables `ise`, runs one `Dry-Run Ise`
against itself, and every observed difference is recorded as a `Spec gap`
outcome.

## Capabilities delivered

| Use case | Capability | Behavior this seed delivers |
|---|---|---|
| `declare-the-specification` | `specification-declaration` | Givn writes `givn/specification.yaml` with `behavior: ["givn/specs/**"]`, `structure: ["docs/glossary.md", "docs/domain-model.md"]`, `contracts: ["docs/reference.md"]`, `limits: ["docs/arc42/02-architecture-constraints.md"]`, `e2e: ["e2e"]`, and `source: ["src/**", "tests/**"]`. |
| `prepare-an-ise-test` | `clean-room-preparation`, `trial-input-verification` | `givn ise prepare --into <dir>` with no `--replace` sets the whole declared `source:` set aside, records the three identities, and `givn ise verify` re-checks them on request. |
| `create-change` | `change-strategy`, `spec-gap` | The change records a whole-product rebuild with `Ise Test` mode `Dry-Run Ise`, and every observed difference carries one `Spec gap` outcome before any corrective edit. |

The retained component runner keeps the component scenarios; both runners read
the same feature files and filter by tag, and only the `e2e` package is
frozen.

## Accepted decisions

### `S13`: enable `ise` only after the package boundary is green

- Context: `plan.md` `S13` splits the work into Change A (the boundary) and
  Change B (enablement and the trial). With `ise` enabled first,
  `givn check tasks` refuses Givn's own `givn`-linked E2E package, and
  `givn ise prepare` refuses while the addon is off, so the two orders cannot
  collapse into one step.
- Options:
  1. **Enable after the boundary change archives**: In practice:
     `addons.ise: true` lands with the declaration, the change lock records
     `ise: true`, and `givn check tasks` passes on Givn itself. Gain: the gate
     validates the finished package and the enablement is one reviewable
     revision. Cost: two changes instead of one.
  2. **Enable in the boundary change**: In practice: the same change moves the
     steps and turns `ise` on. Gain: one change. Cost: the boundary is red
     while the change is in flight, so the change's own `givn check tasks`
     refuses it, and gate `G10` stops `S13` before enablement.
  3. **Prepare the trial before enabling `ise`**: In practice: run
     `givn ise prepare` with the addon off. Gain: no enablement before the
     trial. Cost: `S4` binds both commands to `addons.ise`, so preparation
     refuses by name with the addon off.
- Chosen: **Enable after the boundary change archives** because `G10` requires
  a green boundary before the project opts in and the declaration is read only
  while `ise` is effective. Rejected: enabling in the boundary change because
  the change's own check would fail; preparing with the addon off because the
  command refuses.

### Q14 and the settled model: one `Spec gap` outcome before any fix

- Context: the dry run replaces the whole declared `source:` set, so every
  difference between the frozen E2E suite and the declared Specification must
  be classified before a corrective edit, or the outcome is inferred from the
  fix.
- Options:
  1. **Classify before fixing**: In practice: each difference receives one of
     E2E test defect, implementation defect, specification defect, or
     acceptable undocumented change. Gain: the Specification grows
     deliberately and no difference closes as drift. Cost: a classification
     pass with no edits until every outcome is recorded.
  2. **Fix as found and record the outcome after**: In practice: repair first,
     then label the difference. Gain: faster repair. Cost: the outcome is
     chosen to fit the fix, and what the Specification should have said
     becomes unobservable.
  3. **Stop the trial on the first difference**: In practice: one clean
     difference at a time. Gain: a small difference set per trial. Cost: the
     full set is unknown, and `S13`'s exit criterion requires every difference
     classified.
- Chosen: **Classify before fixing** because `S12` and `S13` require one
  outcome per difference with no drift left open; a trial defect stops the
  trial only to correct the test before the trial restarts. Rejected:
  record-after because the outcome stops being evidence; stop-on-first because
  it cannot produce the complete set.

## Open questions

### Which recorded evidence satisfies the external-evidence prerequisite?

- Blocks: this seed's start and the claim that a Givn self-rebuild tests a
  validated protocol.
- Context: `plan.md` `S7` to `S12` supply the Catopia and Passuff evidence.
  This seed takes that evidence as a prerequisite for starting: a rebuild
  started before it exists would test an unvalidated protocol on its own
  authoring tool. The topic did not settle which records are read to confirm
  the prerequisite.
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
