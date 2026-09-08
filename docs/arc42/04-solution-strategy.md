# 4. Solution Strategy

This chapter summarizes the current implementation strategy. Decision rationale
is in the [ADR register](adr/README.md); mechanics are in [chapter 8](08-crosscutting-concepts.md).

## 4.1 Strategy at a Glance

| Concern | Strategy |
|---|---|
| Language and packaging | Rust crate producing the `givn` CLI; defaults are embedded in the binary. |
| Configuration | Embedded manifest plus `givn/config.yaml` overlay, then `givn/commands.yaml` overlay for project commands and coverage. |
| Facets | Project override wins over embedded template/instruction/command asset. |
| Lifecycle | Manifest `requires` graph; standard order is proposal, specs, design, design-review, tasks, review, archive. |
| Specification | `.feature` files with `@givn.*` deltas; archive merges and strips givn tags. |
| Domain ownership | Explicit `usecase.md` documents own capabilities; reusable behavior is represented by typed fragments. |
| Ideation continuity | Stable use-case IDs, confirmed Persona references, and explicit Handoff connect topic state to proposals and permanent specs. |
| State | Filesystem-derived from effective artifact paths, markers, and `.givn-skip`. |
| Agent integration | Selected `.opencode`, `.agents`, and `.claude` targets; only givn-owned fenced blocks may be changed in `AGENTS.md`. |
| Test execution | `verify.command` and `verify.e2e_command` come from `givn/commands.yaml`; lint is separate. |
| Product start | One author-owned `run:` declaration in `givn/commands.yaml` documents and gates how the built product starts; `givn run` executes it with a std-only readiness probe, a greppable ready line, a spec'd exit-code contract, and unix signal forwarding. ADR-067 records the declare-and-execute boundary. |
| Gate safety | `givn check tasks|review` contains non-deactivatable mandatory checks; `post` is additive customization. |
| Arc42 | `givn init` enables addons by default unless `--no-addons`; Arc42 chapters are scaffolded at fixed `docs/arc42/`. |
| Structural overlap | Deterministic Fingerprint evidence and disposition gate. |
| Release migration | A clean 0.2.0 project receives a managed two-commit preparation and a visible maintenance change; project semantics remain LLM-reviewed. |
| Migration-free upgrade | A release without a selected route synchronizes managed guidance and creates one allowlist-only commit when files changed; it creates no maintenance change. |

## 4.2 Configuration and Gate Strategy

The effective manifest is loaded as embedded defaults, overlaid by
`givn/config.yaml`, and then overlaid by `givn/commands.yaml` when that file is
present. Artifact fields are deep-merged by id; the top-level `verify`,
`coverage`, and `apply` blocks supplied by an overlay replace their effective
blocks.

`post` is not the source of the core review contract. The binary always runs the
mandatory checks for explicit `givn check tasks` and `givn check review`:

- tasks: WIP protection for a change delta and `verify.command`;
- review: `verify.command`, `verify-e2e`, `integrity`, the deterministic
  run-declaration check, and, for a named change, the overlap disposition gate.

Built-in hook names cannot be downgraded with `allow_failure`. Custom shell
hooks can be configured and may be allowed to fail. A no-argument `givn check`
only runs configured post hooks and does not call the explicit artifact
mandatory-check path. Archive has its own hard-coded post-merge verify/e2e and
optional coverage stages in addition to configured archive hooks.

## 4.4 Quality Goals

### Configurability without forking

The cascade controls individual assets and the overlay controls the artifact
graph. Projects can change a template, instruction, dependency, hook, or graph
node without changing the binary.

### Installability

The product target is one native executable with embedded defaults. The build
contract is a single feature-free binary: no Cargo features are declared and no
model dependencies exist.

### Upgradeability

`givn upgrade` refreshes the managed configuration scaffold while preserving
uncommented project values. For the exact 0.2.0-to-0.3.0 route it first requires
a clean Git worktree, refreshes managed generated integrations, commits only
the allowlisted Givn paths, and creates a migration maintenance change that
records the first commit. Ejected facets, durable docs, specs, and user-owned
files remain project-owned and are handled by the LLM through that change.
