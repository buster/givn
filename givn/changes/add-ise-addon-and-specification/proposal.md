# Proposal: add-ise-addon-and-specification

## Release note

A project can opt into `ise`, receive a strictly validated declaration of its retained Specification and replaceable source roots, see four advisory placement findings in `givn lint`, and be served the `ise` obligations and the two change records that guidance names.

## Use-Case Context

- Use-case IDs: `initialize-project` (`enable-optional-addons`),
  `upgrade-a-project` (`apply-a-managed-upgrade`),
  `declare-the-specification` (`specification-declaration`),
  `verify-change` (`check-specification-health`),
  `obtain-effective-guidance` (`serve-extended-rules`),
  `create-change` (`change-strategy`, `spec-gap`), and fragment `support`
  (`instruction-integrity`, `match-guidance-to-enabled-addons`)
- Ideation topic: `ise-rebuildability`
- Seed path: `givn/ideation/ise-rebuildability/changes/add-ise-addon-and-specification.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

## Review Context

- Review kind: independent
- Reason: major change. It changes the addon registry contract (a registered
  addon may declare a default of `false` and a prerequisite), introduces a new
  project-owned file type with a strict schema, adds a finding surface to
  `givn lint`, and writes eight durable glossary rows plus six rewrites that
  govern vocabulary used by every later change.

## Problem / Opportunity

Every registered addon is currently enabled by default, and the durable
vocabulary says so: `Addon` reads "Every registered addon is enabled by
default", and `docs/arc42/01-introduction-and-goals.md` states that there is no
default-off addon. A gated capability therefore cannot exist without either
altering every project silently or inventing a second configuration authority.

No project can declare the boundary a clean-room rebuild needs: which authoring
input stays unchanged (`Specification`) and which application code is deleted
and generated again (`Source code`). Without that declaration there is no
subject for the later `ise` commands, no way to report whether a declared
retained path still exists, and no way to say which package is the frozen
black-box acceptance authority.

Change authors also have no recorded choice between incremental modification,
a one-component rebuild, and a whole-product rebuild, and no recorded
classification for a difference observed during a rebuild. Guidance that names
a rebuild therefore cannot be checked for having asked the question, and a
difference can be closed as silent drift.

Impact: without this change a project cannot opt into `ise`, cannot declare its
Specification, cannot receive placement findings, and is never served the
obligations a rebuild requires. The later boundary gate, clean-room commands,
and stack split all depend on the artifacts this change creates.

## Proposed Solution

Observable behavior, from the project's side:

- **Opt-in registration.** `givn addons list` reports `ise` as disabled on a
  project that never enabled it. A fresh `givn init` writes an explicit `true`
  for every addon whose declared default is on, writes no `ise` key, and writes
  no declaration file. `givn init --no-addons` writes an explicit `false` for
  all ten addons. An omitted key in a partial `addons:` block resolves that
  addon's declared default; the listing's prose says so instead of claiming
  every omitted key means enabled.
- **Validated prerequisite.** `givn addons enable ise` with
  `architecture_principles` off prints
  `addon 'ise' requires 'architecture_principles'; enable it first`, exits
  nonzero, and writes neither stored flag. `givn addons disable
  architecture_principles` with `ise` on prints
  `addon 'architecture_principles' is required by 'ise'; disable 'ise' first`,
  exits nonzero, and writes neither stored flag.
- **Declaration scaffold.** The first effective enable of `ise` creates
  `givn/specification.yaml` and reports the path it wrote. A second enable, or
  an enable on a project that already edited the declaration, leaves the file
  byte-for-byte and reports it as existing. The managed upgrade writes and
  stages the scaffold when `ise` is effective and the file is absent, stages
  nothing for an existing declaration, and reports which of the two happened.
  A repeated upgrade produces no second revision.
- **Strict declaration schema.** `givn/specification.yaml` holds exactly five
  roles (`behavior`, `structure`, `contracts`, `limits`, `e2e`) as path globs
  and one `source:` list of replaceable roots. A declaration with an unknown
  key, an unknown role, a repeated role, an omitted role, an absent `source:`
  list, or a path outside the project is refused by name; the file is never
  rewritten and no reader acts on a partial role set. A project with `ise` off
  never reads the file, even when one exists.
- **Four advisory findings.** `givn lint` reports a declared path missing from
  the tree, one path declared in two roles, a `.feature` file outside the
  `behavior` globs, and a declared `e2e` package that is missing or carries a
  dependency edge into `source`. Each names the path or package, each is
  `advisory`, and the exit code stays 0. Change-scoped lint keeps its existing
  refusals and does not turn the four findings into review blockers.
- **Addon-bound guidance.** `givn instructions ise` serves the rebuild
  obligations only while `ise` is effective; with the addon off the request
  exits nonzero and names `ise`. The text and JSON paths carry the same
  content. The `ise` instruction is the only `ise` guidance a generated skill
  carries, and only when the addon is on.
- **Two change records.** The design guidance requires exactly one `Change
  strategy` as the existing `Decides D<n>` ledger entry: `incremental
  modification`, `Constant Ise`, or a whole-product rebuild, with `Ise Test
  mode` (`Dry-Run Ise` or `Full replacement`) recorded beside it when the
  change rebuilds, and the affected consumers, persisted state, and rebuild
  boundary named. No mandatory design field is added. The review guidance
  requires exactly one `Spec gap` outcome before any fix (`E2E test defect`,
  `implementation defect`, `specification defect`, `acceptable undocumented
  change`), states that only `specification defect` is a gap in the
  Specification, and asks the neighboring-components question as a question,
  never a gate.
- **Vocabulary lands with the code.** Eight confirmed glossary rows, six
  existing-row rewrites, five anti-term additions, two scenario-title
  disambiguations, and one arc42 sentence land in the batches whose code makes
  each statement true.

## Capability Routing

Route reports one recommendation for the whole proposal:
`EXTEND enable-optional-addons (use case 'initialize-project', score 33.50) —
confidence: signal`. Its ranked candidates also name
`check-specification-health` (27.50), `initialize-a-project` (27.00), and
`serve-extended-rules` (25.50). The table records that recommendation where it
applies and records `no signal` where route ranked no candidate for the
capability. The decision column records the placement the use-case contracts
and the ideation handoff assign.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `enable-optional-addons` | `EXTEND enable-optional-addons` (33.50) | `EXTEND enable-optional-addons` | Route agrees. The behavior registers an addon, checks its prerequisite, and scaffolds a declaration; `initialize-project` owns addon posture. |
| `apply-a-managed-upgrade` | `no signal` | `EXTEND apply-a-managed-upgrade` | The managed write and staged allowlist belong to `upgrade-a-project`; route ranked no candidate because the proposal names `givn upgrade` once. |
| `specification-declaration` | `no signal` | `NEW in declare-the-specification` | No permanent contract exists for declaring retained roles; the handoff cuts `declare-the-specification` as a new leaf. |
| `check-specification-health` | `EXTEND check-specification-health` (27.50) | `EXTEND check-specification-health` | Route agrees. The four declaration findings are project-level advisories; the blocking change gate is a different capability in a later change. |
| `serve-extended-rules` | `EXTEND serve-extended-rules` (25.50) | `EXTEND serve-extended-rules` | Route agrees. Addon-bound instruction serving belongs to `obtain-effective-guidance`. |
| `change-strategy` | `no signal` | `NEW in create-change` | No permanent capability owns the design ledger requirement; the handoff assigns it to `create-change`. |
| `spec-gap` | `no signal` | `NEW in create-change` | No permanent capability owns the four-way difference classification; the handoff assigns it to `create-change`. |
| `instruction-integrity` | `no signal` | `EXTEND instruction-integrity` | The `ise` obligations and the design, design-review, and review requirements are guidance-corpus rules owned by the `support` fragment. |
| `match-guidance-to-enabled-addons` | `no signal` | `EXTEND match-guidance-to-enabled-addons` | Two scenario titles use "the opt-in addon" for any enabled addon; the reserved term needs the disambiguation. |

## Out of Scope

- The blocking `@e2e` package gate, the frozen acceptance boundary, and the
  E2E-bound verification receipt (`enforce-ise-e2e-boundary`).
- `givn ise prepare` and `givn ise verify`, clean-room preparation, trial-input
  verification, and the `--replace` deletion set.
- The core/stack instruction split, `steps-rust` and the other stack
  instruction bodies, and the instruction-id reference check
  (`split-stack-instructions`).
- Enabling `ise` on this repository, declaring Givn's own Specification, and
  running any Ise Test.
- Unit-test inference, a `components:` key or per-component file list, a
  step-phrase uniqueness rule, a blast-radius gate, and builder containment.
- Changing guidance for a project that never enables `ise`: no new key, no new
  file, no new gate, and no new finding.

## Open Questions

### D1 — Which active values the first declaration scaffold writes

- Question: When `givn addons enable ise` or `givn upgrade` materializes an
  absent `givn/specification.yaml`, which active values do the five roles and
  the `source:` list carry? The declared schema requires all six keys, so the
  scaffold must choose values before any developer edit, and the choice is
  visible on every project that enables the addon.
- Options:
  1. **Empty role lists and an empty `source:` list** — the file validates
     immediately and retains nothing until the developer edits it; gain: no
     invented paths and no language knowledge in Givn; cost: the first trial
     refuses because the declaration retains nothing, and the developer must
     edit before any rebuild; in practice: `givn addons enable ise` writes
     `behavior: []`, `structure: []`, `contracts: []`, `limits: []`, `e2e: []`,
     `source: []` and the project's first `givn lint` reports no declaration
     finding.
  2. **Empty active lists with commented example paths** — the active lists
     stay empty while comments show `givn/specs/**`, `docs/glossary.md`,
     `docs/domain-model.md`, and `src/**`; gain: guidance without invented
     active paths; cost: the developer still edits before a trial, and a later
     tool rewrite of the declaration would drop the comments; in practice: the
     file shows `# behavior: ["givn/specs/**"]` beside `behavior: []`.
  3. **Detected paths from the existing tree** — the scaffold writes
     `givn/specs/**` when that directory exists and a source root for the
     project; gain: a declaration that works immediately; cost: detection is
     inference about a project's layout, which the settled constraints forbid
     for tests and which no use-case rule authorizes for source roots, and the
     empty-project case still needs a fallback; in practice: one project gets
     `source: ["src/**"]` and another gets `source: ["crates/app/src/**"]` from
     the same command.
- Disposition: carried — the design.md Decisions ledger decides it under an
  owner named there; the deciding artifact is `design.md`.

### D2 — Whether the `ise` instruction belongs to this change or to `split-stack-instructions`

- Question: Does this change serve the `ise` instruction, or does
  `split-stack-instructions` add it together with the stack split? The
  obligations and the records they require ship here; the reference check and
  the line budget see every new instruction file.
- Options:
  1. **This change** — `givn instructions ise` is served as soon as `ise` is
     effective, and `split-stack-instructions` only adds the stack-id reference
     check for it; gain: the obligations and the records they require ship
     together; cost: this change grows by one instruction file and its serving
     scenarios; in practice: a project enabling `ise` after this change
     archives reads the five obligations immediately.
  2. **`split-stack-instructions`** — the obligations and their records ship
     first and the instruction text arrives with the split; gain: this change
     stays smaller; cost: an enabled project has the declaration, the findings,
     and the change records but no served instruction for one change, and the
     split change's guarantee is about stack growth, not addon serving; in
     practice: `givn instructions ise` refuses the id as unknown until the
     split lands.
  3. **Both changes** — this change serves it and the split adds it to the
     line-budget and reference checks; gain: ownership explicit on both sides;
     cost: the instruction and its tests are touched twice and both changes
     must agree on the id first; in practice: the same file is revised in two
     consecutive changes.
- Disposition: answered — the seed records "This seed" because the
  obligations are released only if the instruction that states them is served
  in the same guarantee, and `split-stack-instructions` changes where existing
  stack bodies live, not which addon-bound instructions exist.

### D3 — Which package manifests the `e2e` dependency-edge finding reads

- Question: The fourth advisory finding must say whether the declared `e2e`
  package links production source. Which manifests does the first cut read, and
  what does the finding do on a project whose package graph it cannot read? The
  seed records this as a guarantee limit: on a non-Cargo project the finding
  would be silent unless a broader reader is named.
- Options:
  1. **Cargo manifests only, with the limit stated** — the finding reads
     `Cargo.toml` package manifests and reports the edge for Cargo packages;
     on a project with no Cargo manifest it reports nothing and the limit is
     documented; gain: one deterministic reader, no network, no test-runner
     invocation, no second language runtime, and Givn itself is the reference
     project; cost: a .NET or Node project receives no `e2e` edge finding in
     the first cut; in practice: a workspace member whose `Cargo.toml` names
     the product crate as a path dependency is refused by name, while a
     `package.json`-only project receives the other three findings and nothing
     for this one.
  2. **Read every supported package manifest** — Cargo, npm, and NuGet
     manifests are all parsed; gain: the finding covers more projects; cost:
     three parsers, three naming conventions, and a much larger surface with no
     reference project to test it against; in practice: the finding must decide
     which npm `dependencies` name production source.
  3. **Delegate to the project's own tool** — a configured command reports the
     edge; gain: the project decides its package graph; cost: `givn lint`
     becomes non-static and invokes project tooling, contradicting its
     documented contract, and a broken tool command makes the advisory finding
     a blocking failure; in practice: `givn lint` runs `cargo metadata` or
     `npm ls` and fails when the toolchain is absent.
- Disposition: carried — the design.md Decisions ledger decides it under an
  owner named there; the deciding artifact is `design.md`, and the recorded
  limit is repeated in the `check-specification-health` use-case document.
