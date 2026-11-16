# Proposal: move-givn-e2e-to-black-box-package

## Release note

Givn's `@e2e` scenarios run from a dependency-free package that reaches the
product only through the built binary, so the E2E verification scope proves the
installed CLI instead of the library it was compiled with.

## Use-Case Context

- Use-case IDs: `verify-change` (`givn-workflow-enforcement`) and
  `obtain-effective-guidance` (`serve-extended-rules`, consumed unchanged)
- Ideation topic: `ise-rebuildability`
- Seed path:
  `givn/ideation/ise-rebuildability/changes/move-givn-e2e-to-black-box-package.md`
- Confirmed Personas: `agent--technical-role`, `givn-maintainer--owner`

## Review Context

- Review kind: independent
- Reason: major change. It moves the repository's acceptance authority into a
  new package, changes which command proves the `@e2e` corpus, and adds a
  workspace manifest that outlives code replacement. The author of the move
  cannot sign off the suite that judges the move.

## Problem / Opportunity

Givn's acceptance suite runs inside the target that links the product. Nine step
files under `tests/steps/` call `givn::` APIs, `tests/features_runner.rs` calls
`givn::gherkin::delta::is_removed_placeholder`, and `verify.e2e_command` runs the
same `features_runner` binary as the regular scope. The Ise rule this project
enforces on other projects says the declared End-to-end (E2E) test package holds
the runner and every `@e2e` step definition, links no production source, and is
the target of `verify.e2e_command`.

Givn therefore cannot hold its own rule. `cargo tree` for the test target lists
`givn`, an `ise`-effective check on this repository would refuse its declared
package, and a step definition that calls the product in process can pass after
the binary breaks. A rebuild of Givn also cannot reuse its own suite as
independent evidence, because the suite shares the implementation it judges.

## Proposed Solution

From the maintainer's side:

- **A separate E2E package.** `e2e/` holds its own manifest with a
  `[workspace]` section, the Gherkin runner, every step definition bound to an
  `@e2e` scenario, the browser driver assets, and the fixtures those steps need.
  No table in the package names `givn`, and `cargo tree` under the package lists
  no product crate. The package ships nothing: `publish = false`, and the root
  package `exclude` list gains `e2e/**`.
- **Binary-only reach.** Every `@e2e` scenario reaches the product by executing
  the built `givn` binary. Fixtures are files, fixture projects are created with
  the real `givn init` and `givn new`, and expected values come from the
  binary's output. No `@e2e` step imports a product type, constant, template, or
  trait.
- **Scope separation.** `verify.e2e_command` runs the new package; the regular
  scope keeps the in-process component tier. Both read the same feature files
  and select by tag, so the feature files stay where they are.
- **Wording preserved.** The current corpus has 111 `@e2e` `Scenario:`
  declarations plus 3 `Scenario Outline:` declarations whose 8 example rows
  execute (119 executions) in 56 feature files, not the seed's 108 in 51; the
  corpus grew after the seed. All of them keep their wording. Four of them
  contain a step that calls a product
  helper; each step is rewritten against the binary's output or a literal
  fixture. Zero scenarios are retagged. The package's inventory records
  every scenario and its bound step definitions.
- **The declaration names the package.** `givn/specification.yaml` declares the
  plan's roles: `behavior: ["givn/specs/**"]`,
  `structure: ["docs/glossary.md", "docs/domain-model.md"]`,
  `contracts: ["docs/reference.md"]`,
  `limits: ["docs/arc42/02-architecture-constraints.md"]`, `e2e: ["e2e"]`, and
  `source: ["src/**", "tests/**"]`. The file stays inert because `ise` is off;
  this change does not enable the addon.
- **The gate stays consistent.** The blocking change check already resolves the
  declared `e2e` role, refuses a package that links source, runs the E2E command
  when the delta has `@e2e` scenarios, and binds the receipt. This change
  supplies the package those checks were written for and proves the
  standalone-workspace layout passes the tasks check on an `ise`-effective
  fixture. On this repository `ise` stays off, so the gate is proven on that
  fixture, not on Givn itself.
- **A qualification path.** `scripts/qualify-givn-e2e.sh` applies one seeded
  product defect to a scratch copy, requires the package to fail, reverts the
  defect, requires the package to pass, and requires the package's dependency
  tree to name no product crate. The real tree's hash is compared before and
  after, including on failure.

## Capability Routing

`givn spec route --change move-givn-e2e-to-black-box-package` reports
`EXTEND enable-optional-addons (use case 'initialize-project', score 28.00)`,
confidence `signal`, and ranks `gvn-workflow-enforcement` second at 26.00. It
scores the fixture commands that build an `ise`-effective project (`givn
addons`, `givn init`, `givn new`) as addon enablement. The table records the
recommendation and the decision; the decision column is authoritative.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `givn-workflow-enforcement` | `EXTEND enable-optional-addons` (28.00); `gvn-workflow-enforcement` ranked second (26.00) | `EXTEND givn-workflow-enforcement` | Deviated. The proposal names `givn init` and `givn new` only to build fixtures, and the change adds no addon behavior. The runner target, the declared package, and the E2E scope are the change-scoped verification this capability owns, as the seed assigns. |
| `adr-qualification-gate` | `no signal` | `EXTEND adr-qualification-gate` | The change adds ADR-079, so the register count the permanent scenario asserts must change from 30 to 31. The scenario is corrected by `@givn.modified`; the capability gains no behavior. |

`obtain-effective-guidance` (`serve-extended-rules`) is recorded in the
Use-Case Context because the seed names it. The policy sentence that names the
declared package was delivered by `enforce-ise-e2e-boundary`; this change
supplies the configuration the sentence names and adds no instruction text, so
the capability has no delta.

## Out of Scope

- Enabling `ise` on this repository and running a Dry-Run Ise
  (`enable-ise-on-givn-and-run-a-dry-run`).
- Every change to what a scenario asserts, its title, or its tag. The retag
  count is zero; a future retag needs its own recorded reason.
- New or changed instruction text, the canonical `@e2e` policy, and the stack
  instructions.
- The blocking boundary check, the advisory findings, the receipt schema, and
  the Cargo dependency reader delivered by `enforce-ise-e2e-boundary`.
- The component tier and its step definitions, which stay in `tests/`.
- The published product: one crate, one binary, no new runtime dependency.

## Open Questions

### D1: Which recorded evidence satisfies the seed's external-evidence prerequisite?

- Question: The seed starts only after the recorded campaign evidence exists in
  `S7` to `S12` of `plan.md`, because a rebuild started earlier would test an
  unvalidated protocol on its authoring tool. Which recorded evidence must
  exist before this seed starts?
- Options:
  1. **Every planned run in `S7` to `S12`, with its evidence**: the Catopia
     `E1` to `E4` results and the Passuff package, Constant Ise, and
     full-product results are recorded; gain: the protocol is validated across a
     controlled campaign and a product; cost: the seed waits for the slowest
     phase; in practice: the change stays blocked until the Catopia `S8` bundle
     and the Passuff `S12` bundle both exist.
  2. **The two frozen-suite results only (`S8`, `S12`)**: a cold rebuild and a
     full-product rebuild passed or were fully classified; gain: the seed waits
     on the two results that exercise the same guarantee; cost: Constant Ise
     (`S9`, `S11`) stays unproven; in practice: the change starts when the two
     frozen-suite bundles exist even if no Constant Ise run does.
  3. **The last full-product trial only (`S12`)**: one complete product rebuilt
     from its own corpus; gain: the earliest defensible start; cost: a protocol
     defect found on Givn has one prior data point; in practice: a green
     full-product Passuff trial starts the change.
- Disposition: carried. Default: this change proceeds on the operator's
  authorization of 2026-11-16 and makes no claim that the Ise protocol is
  validated. Owner: `givn-maintainer--owner`. The deciding artifact is the
  topic's evidence table in
  `givn/ideation/ise-rebuildability/plan.md` (`S7` to `S12`); the trigger is the
  next seed's start (`enable-ise-on-givn-and-run-a-dry-run`). No scenario in
  this change depends on the answer. Rejected option 1 for this change because
  the move of a test package does not exercise the protocol.

### D2: Does this change create the declaration while `ise` is off?

- Question: `plan.md` `S13` Change A names the declaration roles, but `ise`
  stays off in Change A and an `ise`-off project never reads the declaration.
  Does this change write `givn/specification.yaml` now, or does the enabling
  change scaffold it?
- Options:
  1. **Write the declaration now**: record the plan's six keys; the file is
     inert because `ise` is off, and Change B only enables the addon; gain: the
     package and the role that names it land together, so Change A's boundary
     evidence is complete; cost: an unread file can drift before Change B; in
     practice: `givn/specification.yaml` exists with `e2e: ["e2e"]`, and `givn
     lint` reports nothing because the addon is off.
  2. **Defer to the enabling change**: let `givn addons enable ise` scaffold the
     declaration in Change B; gain: no inert file; cost: the package lands
     without the role that names it, and Change A cannot show the layout the
     gate will read; in practice: `givn/specification.yaml` appears only when
     the addon is enabled.
- Disposition: carried. The deciding artifact is `design.md`; the design
  records the choice and its default.

### D3: Standalone workspace or root workspace member?

- Question: `plan.md` `S13` says both "add `e2e` as a workspace member" and
  "add `e2e/**` to the package `exclude` list", while the Passuff precedent
  declares its own `[workspace]` so the package survives a clean-room copy.
  Cargo forbids a package that is both a member of the root workspace and its
  own workspace root. Which shape does Givn's package take?
- Options:
  1. **Standalone workspace excluded from the root package** (the Passuff
     pattern): `e2e/Cargo.toml` declares `[workspace]`, and the root package
     `exclude` list gains `e2e/**`; gain: the package tree runs without the
     repository root and its dependency graph is its own, so `cargo tree`
     proves the absence of every product crate; cost: a second lockfile, and
     the root `cargo fmt`/`clippy --all-targets` does not cover it, so
     `run-tests.sh` adds explicit invocations; in practice:
     `cargo tree --manifest-path e2e/Cargo.toml` lists no `givn`.
  2. **Root workspace member**: convert the root manifest to a workspace with
     `members = ["e2e"]` and drop the package's own `[workspace]`; gain: one
     lockfile and one format/lint run; cost: the package shares the product's
     workspace graph and lockfile, the dependency check reads the product
     workspace instead of an independent package, and the tree cannot be
     copied into a clean room; in practice: `cargo tree -p givn-e2e` runs at
     the root but the package is not standalone.
- Disposition: carried. The deciding artifact is `design.md`; the design
  records the chosen shape and the plan-wording deviation.
