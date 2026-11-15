# givn — Architecture Documentation (arc42)

This is the durable architecture documentation for the current checkout,
organized according to the [arc42](https://arc42.org) template. Each chapter is
a separate file. Chapter 9 is a register whose decisions are stored as
individual MADR files.

## Documentation Ownership

Each contract has one home:

| Subject | Authoritative document |
|---|---|
| Purpose, installation, quick start, brief guarantees | [README](../../README.md) |
| Lifecycle phases, gates, batch flow, recovery overview | [Workflow](../workflow.md) |
| CLI, configuration authority, skills and invocation, runner/receipt contracts, layout | [Reference](../reference.md) |
| Step obligations an agent follows | Served instructions (`givn instructions <id>`) and their assets |
| Current architecture and constraints | These arc42 chapters |
| Decision rationale | The [ADR register](adr/README.md) |
| Post-1.0 work | Chapter [11](11-risks-and-technical-debt.md) |

An active change's proposal, design, and specs own its planned behavior until it
archives. Docs describe the current checkout; they do not present a planned or
completed change as pending work.

## Current State

The checkout contains the Rust CLI, the Git and Fossil adapters, the change
lifecycle and archive proof, the deterministic specification tools, the
skills-only guidance pipeline, the arc42 addon, the development record and
report, the clean-room trial commands, and the advisory outline report.

Current architecture facts that recur in the chapters:

- One feature-free Rust binary; no Cargo features and no model dependency.
- Configuration authority is split: `givn/config.yaml` owns policy and graph,
  `givn/commands.yaml` owns `verify`, `coverage`, and `run`.
- Every registered addon is enabled by default; an explicit `false` is the
  opt-out. `communication_policy` and `challenge` replace the retired
  `absolute_mode`, `grill_me`, and `grill_with_docs` keys without aliases.
- Skills are the only generated invocation surface. Claude Code and OpenCode
  use their skill slash, OMP uses `/skill:<name>`, the neutral `agents` target
  uses skill discovery, and Maki uses an explicit or agent-selected skill.
- `givn commit` requires explicit paths and preserves unrelated work on both
  backends. Git additionally stamps raw commits through the optional
  `commit-msg` hook; Fossil raw check-ins carry no attribution.
- Review and archive reuse a fresh change verification receipt while its tested
  content is unchanged; archive runs its merged scopes against the published
  candidate.
- Clean-room trials are materialized by `givn ise replace` from one revision and
  re-checked by `givn ise verify`; the `ise-trial/v1` record carries per-path
  retained and `e2e` identities and lives outside the trial and the checkout.
- With `ise` effective, `givn archive` refuses before mutation when a declared
  retained path in `givn/specification.yaml` would be isolated by the change
  move, naming the entry and the archived path to declare instead; the served
  `ise` guidance states that retained paths must survive archiving.
- The outline report is advisory. There is no outline review gate, verdict,
  waiver, baseline, or forced reviewer context.
- The supported migration baseline is 1.0. The shipped migration catalog is
  empty; pre-1.0 projects repair their configuration manually.

## Explicit Unknowns

These are outside the current implementation or documentation evidence and are
not claimed as delivered:

- Release binaries, Homebrew, a shell installer, a published cargo crate, and a
  verified cross-platform target matrix. See [chapter 7](07-deployment-view.md)
  and [chapter 11](11-risks-and-technical-debt.md).
- Production-scale performance of the record store and the report; no bottleneck
  has been measured. See [chapter 11](11-risks-and-technical-debt.md).
- Gherkin dialects other than English, `Rule:` publication, Java/Go structure
  adapters, complete outline coverage or metric parity, and semantic duplicate
  matching. See [chapter 11](11-risks-and-technical-debt.md).
- Future migration bundles beyond the declared 1.0 baseline. The catalog
  contract is tested with synthetic routes; no future route is shipped. See
  [the migration contract](../migrations/README.md#future-migration-bundles).

## Chapters

| # | Chapter | Purpose |
|---|---|---|
| 1 | [Introduction and Goals](01-introduction-and-goals.md) | Product purpose, requirements, quality goals, stakeholders |
| 2 | [Architecture Constraints](02-architecture-constraints.md) | Technical, configuration, and process constraints |
| 3 | [Context and Scope](03-context-and-scope.md) | System boundary, interfaces, and scope |
| 4 | [Solution Strategy](04-solution-strategy.md) | Strategic architecture choices |
| 5 | [Building Block View](05-building-block-view.md) | Crate, modules, and responsibilities |
| 6 | [Runtime View](06-runtime-view.md) | Lifecycle, gates, verification, archive, upgrade, and reporting flows |
| 7 | [Deployment View](07-deployment-view.md) | Binary, installation, targets, distribution |
| 8 | [Cross-cutting Concepts](08-crosscutting-concepts.md) | Cascade, manifest, Gherkin, skills, gates, verification, records, analysis |
| 9 | [Architecture Decisions](09-architecture-decisions.md) | Pointer to the [ADR register](adr/README.md) and individual records |
| 10 | [Quality Requirements](10-quality-requirements.md) | Quality tree, scenarios, and verification boundaries |
| 11 | [Risks and Technical Debt](11-risks-and-technical-debt.md) | Current risks, deliberate debt, resolved boundaries, post-1.0 backlog |
| 12 | [Glossary](12-glossary.md) | Canonical domain and technical vocabulary |

## Reading Order

1. Read chapters 1 and 4 for purpose and strategy.
2. Read chapter 8 for cross-cutting mechanics and chapter 6 for runtime order.
3. Read the [ADR register](adr/README.md) and then individual decisions.
4. Treat proposed, deprecated, and superseded records as historical or pending
   evidence, not as accepted architecture.
5. Use chapter 11 for known limits and deferred work rather than treating an
   experimental tool as a complete guarantee.

## Related Documents

- [`../../README.md`](../../README.md) — user-facing project overview.
- [`../workflow.md`](../workflow.md) — lifecycle, gates, and recovery overview.
- [`../reference.md`](../reference.md) — operational contracts.
- [`../migrations/README.md`](../migrations/README.md) — manual 1.0 transition
  and the future migration-bundle contract.
- [`../outline.md`](../outline.md) — the advisory structure report.

*[Architecture documentation index](README.md)*
