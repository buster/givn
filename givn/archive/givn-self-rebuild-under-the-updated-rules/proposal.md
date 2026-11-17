# Proposal: givn-self-rebuild-under-the-updated-rules

## Release note

Givn declares every build input its whole-product rebuild reads, so its next
Dry-Run Ise rebuilds the product from the declared Specification and the
recorded build inputs instead of inventing the build.

## Use-Case Context

- Use-case IDs: `declare-the-specification`
  (`specification-declaration`), `prepare-an-ise-test`
  (`clean-room-preparation`, `trial-input-verification`), and `create-change`
  (`change-strategy`, `spec-gap`)
- Ideation topic: `ise-rebuildability`
- Seed path: none. The change follows the archived self-rebuild's gap record
  and the operator's uplift instruction, not an ideation change seed.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

## Review Context

- Review kind: independent
- Reason: the change edits the project's own Specification declaration and
  the build-input guard for two capabilities (`specification-declaration`,
  `clean-room-preparation`) and produces the whole-product trial evidence
  `S14` reads. Two capabilities and that blast radius make it major; one fresh
  context owns the design review and the final review.

## Problem / Opportunity

The archived whole-product `Dry-Run Ise`
(`enable-ise-and-run-the-givn-self-rebuild`) ended at 51 of 120 frozen
scenarios. Its gap record names five `specification defect` rows against
`givn/specification.yaml`: the four integration-asset rows, where the frozen
tests assert the exact OpenCode, Claude, omp, and Maki text, and the
product-version row, where the rebuilt binary reported `0.9.10` because the
builder authored the build manifest. In all five rows the builder supplied a
build input the declaration left outside the Specification roles: the build
manifest, the lockfile, the embedded asset tree, the toolchain, and the
dependency cache.

The updated `ise` rules now require every build input to be declared under the
role that owns its content class. Givn has not applied that rule to itself. Its
declaration still names only the behavior corpus, three documents, the arc42
limits chapter, the End-to-end package, and the replaceable source roots. The
next trial would repeat the same reconstruction and the same five rows.

Without this change the maintainer cannot tell whether the declared
Specification now covers the build inputs, and `S14` has no re-run evidence
under the updated rules.

## Proposed Solution

From the maintainer's side, observably:

- **Declared build inputs.** The project's declaration names the build
  manifest and the lockfile under `contracts`, the toolchain pin under
  `limits`, the structural templates under `structure`, and the served
  instruction, skill, integration, and hook text under `behavior`. The
  End-to-end package's own manifest and lockfile stay the test build's inputs
  and are not declared as product build inputs.
- **Accepted declaration.** `givn lint` accepts the declaration: exit 0, no
  declaration-schema error, and no `specification_path_missing`,
  `specification_role_duplicate`, or `specification_e2e_boundary` advisory for
  a build-input entry. One expected `specification_feature_unretained`
  advisory names `assets/templates/spec.feature`: the template is a declared
  build input and matches no `behavior` glob. The advisory is recorded, not
  silenced by moving the template to `behavior`.
- **Build inputs in the spec copy.** `givn ise rebuild-all` carries every
  declared build input into the spec copy as a Specification file. The builder
  authors only the removed source roots; it does not author the manifest, the
  lockfile, the toolchain pin, the templates, or the served text.
- **Re-run evidence.** One whole-product `Dry-Run Ise` runs under the updated
  rules. It reads the declaration's build inputs, scans the local package
  registry for a published copy of the product, records the neighbor record,
  and classifies every difference between the frozen acceptance result and the
  rebuilt binary with exactly one `Spec gap` outcome. The five former
  `specification defect` rows are resolved by the declaration; residual
  `implementation defect` rows remain classification evidence.
- **Unpromoted.** The rebuilt binary, the spec copy, and the manifest stay
  outside the checkout. Nothing is promoted; `S14` owns that decision.

## Change Strategy

- Strategy: `whole-product rebuild`
- `Ise Test` mode: `Dry-Run Ise`
- Affected consumers: `developer--primary-user` declares the build inputs and
  reads the frozen result; `agent--technical-role` builds in the spec copy;
  `givn-maintainer--owner` decides `S14` from the evidence; the independent
  verifier runs `givn ise compare` and the frozen suite.
- Persisted state: none. Givn keeps no product state in the trial; the spec
  copy, the manifest, and the rebuilt binary stay outside the checkout.
- Rebuild boundary: every tracked file under `src/**` and `tests/**` is
  removed; the four content roles, the declaration, and the frozen `e2e`
  package stay; the `e2e` package is never copied.

## Spec Gap

The corrective record is
`givn/archive/enable-ise-and-run-the-givn-self-rebuild/evidence/ise/spec-gaps.md`.
Its final table holds five `specification defect` rows: the four
`attribution_asset` rows and the product-version row. The wrong artifact is
`givn/specification.yaml`: it did not declare the build inputs the trial
relied on. This change carries the declaration correction. The record's
residual `implementation defect` rows are not gaps in the Specification, stay
classification evidence, and are not repaired here.

## Capability Routing

`givn spec route --change
givn-self-rebuild-under-the-updated-rules` recommended `EXTEND
check-specification-health` (use case `verify-change`, score 25.50,
confidence `signal`) and ranked:

| Score | Candidate |
|---|---|
| 25.50 | `check-specification-health` (`verify-change`) |
| 24.00 | `clean-room-preparation` (`prepare-an-ise-test`) |
| 22.50 | `specification-declaration` (`declare-the-specification`) |
| 21.00 | `apply-a-managed-upgrade` (`upgrade-a-project`) |
| 20.00 | `spec-gap` (`create-change`) |
| 19.50 | `spec-overlap-gates` (`verify-change`) |
| 19.00 | `givn-workflow-enforcement` (`verify-change`) |
| 19.00 | `validate-change-ids` (`inspect-change-state`) |
| 18.00 | `trial-input-verification` (`prepare-an-ise-test`) |
| 17.00 | `change-strategy` (`create-change`) |

Both capabilities this change extends are ranked: the copy capability second
and the declaration capability third. The top candidate,
`check-specification-health`, matches the proposal's `givn lint` and
`givn/specification.yaml` vocabulary but owns neither the declaration nor the
copy; the corresponding decisions stay `EXTEND` on the capabilities that own
them.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `specification-declaration` | `specification-declaration` (22.50) | `EXTEND specification-declaration` | The declaration's content and its acceptance are owned by `declare-the-specification`; the delta guards a declaration that names build inputs under the four content roles. |
| `clean-room-preparation` | `EXTEND clean-room-preparation` (24.00) | `EXTEND clean-room-preparation` | The spec copy is owned by `prepare-an-ise-test`; the delta guards that declared build inputs are carried into the copy while the End-to-end manifest stays out. |

No new capability and no new user action. The trial evidence belongs to
capabilities that already own it (`change-strategy`, `spec-gap`,
`trial-input-verification`).

## Out of Scope

- Repairing the residual `implementation defect` rows. They remain
  classification evidence for a later change or the next trial.
- Promoting the rebuilt binary, the spec copy, or the manifest. `S14` decides
  promotion.
- `README.md`, `CHANGELOG.md`, and other repository files the frozen tests
  read: the build does not read them, so they are not build inputs.
- Changing the declaration schema, the five role names, the rebuild commands,
  the manifest scope, the report lines, the E2E runner, or the interaction
  inventory.
- Declaring the `e2e` package's manifest and lockfile under `contracts`; they
  stay the test build's inputs under the `e2e` role.
- Editing the served `ise` instruction or the reference rule text. The
  build-input rule already exists and is not changed.
- The `S14` promotion table or the Passuff arm.

## Open Questions

### D1: Which toolchain pin the declaration carries

- Question: Givn has no toolchain pin today, so the declaration's `limits`
  entry needs one. Which pin should the project adopt before the re-run?
- Options:
  1. **Pin the installed stable, 1.97.1**: write `rust-toolchain.toml` with
     `channel = "1.97.1"` and the clippy and rustfmt components; gain: the
     checkout and the sandbox already have it, `cargo +1.97.1 check --locked`
     passes on 2026-11-17, and the frozen build resolves from the cache;
     cost: contributors on another toolchain must install it once; in
     practice: `cargo +1.97.1 check --locked` finishes green and the sandbox
     binds `~/.rustup` read-only with that toolchain present.
  2. **Pin the current stable, 1.99.1**: write the same file with
     `channel = "1.99.1"`; gain: the newest compiler; cost: neither the host
     nor `~/.rustup` holds that exact toolchain, so the host and the sandbox
     need one install before anything builds, and the re-run has one more
     setup dependency; in practice: `cargo build --offline` fails until
     `rustup toolchain install 1.99.1` runs outside the sandbox.
  3. **Use the `stable` channel alias**: write `channel = "stable"`; gain: no
     version to upgrade; cost: the declaration does not fix a toolchain, so a
     later stable release changes the build input without a revision, which
     defeats the pin; in practice: two trials a month apart compile with
     different compilers under the same declaration.
- Disposition: carried — the design's Decisions ledger decides it. Default:
  the installed 1.97.1. Trigger: the copy fails to build on 1.97.1. Switch
  cost: one line in `rust-toolchain.toml` and one toolchain install outside
  the sandbox. Owner: `givn-maintainer--owner`.

### D2: How much of the embedded asset tree the declaration retains

- Question: The five prior `specification defect` rows name four integration
  assets and the product version. Does the declaration retain only those
  surfaces, or every embedded asset under the role that owns its content
  class?
- Options:
  1. **Partition the whole `assets/**` tree by content class**: templates and
     document scaffolds under `structure`, served instruction, skill,
     integration, and hook text under `behavior`, and configuration,
     migration, and manifest assets under `contracts`; gain: no embedded asset
     is left outside the Specification, and the four integration rows and the
     template-scaffold rows are covered; cost: the spec copy grows by 95 files
     and the report counts and checksums change; in practice: the copy holds
     every asset file and the builder authors none of them.
  2. **Declare only the five named surfaces**: the four integration assets and
     the build manifest; gain: the smallest copy and count change; cost: the
     remaining 88 embedded assets, including the templates and the addon
     text, stay outside the declaration and become the next trial's
     `specification defect`; in practice: the rebuilt product still invents
     the review, design, and glossary templates.
  3. **Declare all `assets/**` under one role**: name `assets/**` under
     `behavior`; gain: one entry; cost: the templates and configuration
     schemas sit under a role that does not own their content class, which is
     the rule this change applies; in practice: a structural template is
     copied as behavior text.
- Disposition: carried — the design's Decisions ledger decides it. Default:
  the full partition by content class. Trigger: a design-review finding that
  an asset class is misplaced. Switch cost: edit the declaration entries and
  recompute the trial counts and checksums. Owner: `givn-maintainer--owner`.
