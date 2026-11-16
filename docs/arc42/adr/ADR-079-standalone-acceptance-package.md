---
status: accepted
date: 2026-11-16
decision-makers:
  - givn-maintainer--owner
consulted:
  - move-givn-e2e-to-black-box-package design and the Ise rebuildability topic
informed:
  - contributors, agents, and CI users
---

# Keep the black-box acceptance suite in a standalone workspace package

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - every `@e2e` step definition, the runner, the
      browser assets, and the receipt identity move to one package; reversing
      the choice moves the suite back and invalidates every stored E2E identity.
- [x] **Shapes quality attributes** - the acceptance suite proves the installed
      binary rather than the library it was compiled with, and the package
      builds independently of the product's workspace.
- [x] **Structural** - it defines the repository's acceptance-authority
      boundary, the package's dependency direction (none into `src/**` or
      `tests/**`), and the project runner's binary-path contract.
- [x] **Must be shared** - an author who adds a step to `tests/` instead of
      `e2e/` silently breaks the boundary; the rule must be known by every
      contributor and agent.

## Context and Problem Statement

Givn's `@e2e` scenarios ran in the same Cargo target as the product: step
definitions imported `givn::` APIs, the runner called
`givn::gherkin::delta::is_removed_placeholder`, and `verify.e2e_command`
executed the same `features_runner` binary as the component tier. The Ise rule
Givn enforces on other projects says the declared End-to-end (E2E) test package
holds the runner and every `@e2e` step definition, links no production source,
and is the target of `verify.e2e_command`. A suite that lives beside the
implementation can pass after the installed binary breaks, and a rebuild cannot
use it as independent acceptance evidence.

## Decision Drivers

- the declared `e2e` package is the project's acceptance authority;
- the package must resolve its manifest and lockfile without the product
  workspace;
- the package must ship nothing in the published crate;
- the dependency absence must be provable mechanically;
- the existing feature files and scenario wording stay unchanged.

## Considered Options

- **A standalone `e2e` workspace package excluded from the published crate**:
  `e2e/Cargo.toml` declares `[workspace]`, `publish = false`, the runner, the
  step definitions, and the fixtures; the product binary is reached through
  `GIVN_BIN`.
- **A member of the product workspace**: the root manifest becomes a workspace
  and lists `e2e`; one lockfile and one lint run cover both.
- **A consumer artifact that compiles against a published `givn` API**: the
  package drives the binary but keeps an import edge into the product.
- **Retag every `@e2e` scenario as `@component`**: no second package exists and
  the component tier absorbs the suite.

## Decision Outcome

Chosen option: "A standalone `e2e` workspace package excluded from the published
crate", because the acceptance authority must hold no dependency edge into the
product and must resolve its manifest and lockfile without the product
workspace.

### Consequences

- Good, because `cargo tree --manifest-path e2e/Cargo.toml` proves the absence
  of every product crate, and the change check reads the same package.
- Good, because the package keeps its manifest and lockfile independent, which
  a workspace member cannot.
- Bad, because the repository now has a second lockfile and a second
  format/lint invocation; a stale lockfile fails the suite under `--locked`.
- Bad, because a product-formatted value (slug, fixture narrative) that the
  package mirrors can drift; the qualification script and the unbound-step
  failure are the guards.
- Bad, because the static boundary reader admits a registry or git dependency
  that resolves no project path; the committed `e2e/Cargo.lock` and the package
  runner's refusal of a product package name are the durable guard.

### Confirmation

`scripts/qualify-givn-e2e.sh` fails when `cargo tree --manifest-path
e2e/Cargo.toml` names a product crate, and requires the suite to fail on a
seeded product defect and pass after the revert. The change check's
`gvn-workflow-enforcement` scenario requires a standalone-workspace package to
pass the boundary and bind the E2E identity, with the root named as
`./run-tests.sh --e2e --package e2e`. `./run-tests.sh --e2e --package e2e`
fails on any unbound or skipped step, and the package runner exits 2 when
`e2e/Cargo.lock` names `givn`, `givn-config`, or `givn-core`.

## Pros and Cons of the Options

### A standalone `e2e` workspace package excluded from the published crate

- Good, because the dependency boundary is provable and independent.
- Good, because the package keeps its manifest and lockfile independent of the
  product workspace.
- Neutral, because the package needs a required `GIVN_BIN` path.
- Bad, because a second lockfile and explicit format/lint invocations are
  maintained.

### A member of the product workspace

- Good, because one lockfile and one `cargo fmt`/`clippy` run cover both.
- Neutral, because `cargo tree -p givn-e2e` runs at the root without a manifest
  path.
- Bad, because the package shares the product workspace graph, so the
  dependency check proves a path relation inside one graph rather than an
  independent package.
- Bad, because its manifest and lockfile resolve only through the root
  workspace.

### A consumer artifact that compiles against a published `givn` API

- Good, because one manifest and one lockfile exist.
- Bad, because the import edge into `source` remains and the blocking boundary
  gate refuses the package.
- Bad, because the package links the implementation it is supposed to outlive.

### Retag every `@e2e` scenario as `@component`

- Good, because no second package or manifest exists.
- Bad, because every consumer action loses its black-box acceptance evidence.
- Bad, because the repository holds the package rule it enforces on other
  projects while keeping no compliant package.

## More Information

The change `move-givn-e2e-to-black-box-package` records the measured inventory,
the retag count (zero), and the runner contract. Related decisions:
[ADR-069](ADR-069-single-feature-free-binary.md) owns the single feature-free
binary, [ADR-070](ADR-070-forward-only-archive-proof.md) owns the E2E-bound
change receipt, and
[ADR-078](ADR-078-frozen-package-outside-the-trial.md) owns the frozen package's
exclusion from a clean-room trial.
