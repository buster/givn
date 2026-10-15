# 1. Introduction and Goals

## 1.1 What is givn?

`givn` is a Rust, Gherkin-first workflow CLI. It manages a change lifecycle in
which a `.feature` file is the behavior specification, the executable test input,
and, after archive, the permanent record. It has no parallel `spec.md` format.

The default lifecycle graph is:

```mermaid
flowchart LR
    proposal --> specs --> design --> design_review[design-review] --> tasks --> review --> archive
```

The binary derives state from project files. Embedded assets provide defaults;
project files under `givn/` override them. The initialized project also gets
`givn/commands.yaml`, whose project-owned commands are applied after the main
configuration overlay.

`givn init` currently enables all registered addons by default, including the
`arc42` addon, unless `--no-addons` is supplied. The embedded default manifest
itself has addons disabled; initialization is the operation that writes the
enabled addon settings and scaffolds durable Arc42 chapters.

## 1.2 Requirements Overview

### Functional requirements

| ID | Requirement |
|---|---|
| FR-1 | Scaffold `givn/` and the project-owned command file with `givn init`. |
| FR-2 | Create and track changes through proposal, specs, design, design-review, tasks, review, and archive. |
| FR-3 | Treat `.feature` files as the behavior source of truth. |
| FR-4 | Support `@givn.delta`, `@givn.added`, `@givn.modified`, and `@givn.removed`. |
| FR-5 | Merge deltas into permanent specs and strip `@givn.*` tags on archive. |
| FR-6 | Run static Gherkin lint with distinct parse, duplicate-title, WIP, and clean outcomes. |
| FR-7 | Emit resolved artifact instructions in human-readable and JSON forms. |
| FR-8 | Support scenario-by-scenario RED/GREEN/REFACTOR guidance. |
| FR-9 | Review coverage gaps by classification rather than a percentage threshold. |
| FR-10 | Generate skills and commands to selected targets; preserve all `AGENTS.md` content outside givn-owned fenced blocks. |
| FR-11 | Resolve templates and instructions through a project-overrides-first cascade. |
| FR-12 | Materialize embedded defaults through `givn eject`. |
| FR-13 | Derive lifecycle state from the effective manifest and filesystem. |
| FR-14 | Declare test and coverage commands in `givn/commands.yaml`. |
| FR-15 | Provide an explicit non-zero missing-runner sentinel as the embedded fallback. |
| FR-16 | Generate a documented and versioned `givn/config.yaml` scaffold. |
| FR-17 | Upgrade the scaffold while preserving user-owned values. |
| FR-18 | Allow artifact graph add, remove, reorder, and optional behavior through overlay. |
| FR-19 | Keep proposals and feature files implementation-independent except for observable requirements. |
| FR-20 | Maintain durable Arc42 chapters through the bundled `arc42` addon and fixed `docs/arc42/` output. |
| FR-21 | Run project commands at explicit gate points and keep binary mandatory checks non-deactivatable. |
| FR-22 | Keep one normative interaction and E2E authoring policy. |
| FR-30 | Audit the complete permanent corpus through `givn spec duplicates` without writing audit state. |
| FR-33 | Publish each release with a complete changelog and its release-owned migration routes. |
| FR-34 | Prepare legacy projects through a visible, reviewable migration change without inventing product specifications. |
| FR-35 | Refresh and commit Givn-owned files for migration-free upgrades without creating a migration change. |
| FR-36 | Resolve multiple release-owned migration bundles into one ordered aggregate maintenance change. |
| FR-41 | Declare how the built product is started in `givn/commands.yaml` (`run:` block): product type (server, cli, library), start command, exactly one port source for servers (fixed port or `port_env` hand-off), readiness path, and readiness timeout. |
| FR-42 | Scaffold the run declaration on `givn init`, backfill it through the managed upgrade into existing projects, validate it with deterministic rules, render it into a managed README run block, and block `givn check review` and `givn archive` while it is unconfigured. |
| FR-43 | Start the declared product with `givn run`: readiness probe against the declared path, one greppable ready line, `--port` and `--print` forms, forwarded child exit codes, and SIGINT/SIGTERM forwarding on unix. |
| FR-44 | Never derive the product type or start command from repository content; the declaration is the single source of truth. |

### Non-functional requirements

| ID | Requirement |
|---|---|
| NFR-1 | The product is a single feature-free native binary without npm, root, or language runtime requirements; no Cargo features are declared. |
| NFR-2 | Defaults are embedded; project-owned files are not silently overwritten. |
| NFR-3 | Project configuration and lifecycle state are visible under `givn/`. |
| NFR-6 | Automatic migration commits are narrowly scoped, reproducible, and never mix uncommitted user work with Givn-owned files. |

## 1.3 Quality Goals

| Priority | Quality goal | Motivation |
|---|---|---|
| 1 | **Configurability without forking** | Project overlays and facet overrides change workflow behavior without modifying the binary. |
| 2 | **Installability** | The normal product boundary is one native executable with embedded defaults. |
| 3 | **Upgradeability** | Project-owned values and files remain reviewable and are not clobbered by scaffold updates. |
| 4 | **Architecture contract clarity** | The active ADR register contains only independent durable decisions; current facts and implementation details have one canonical home. |

Secondary goals are merge correctness, agent ergonomics, interface-fidelity in
E2E evidence, and non-redundant architecture documentation.

The approved `make-user-questions-decision-ready` target, pending implementation
and behavioral review, refines agent ergonomics: a project-familiar Developer
receives necessary questions with the decision subject, brief sourced context,
and concrete answer consequences; the Authoring agent discovers one effective
policy at startup, direct invocation, and fresh delegation; the Givn maintainer
changes one policy rather than competing copies. These are outcomes for the
existing stakeholders, not a new priority order or a runtime compliance claim.
Acceptance targets and evidence boundaries are Q-93 through Q-95 in chapter 10.

## 1.4 Stakeholders

| Stakeholder | Interest |
|---|---|
| Tool maintainers | Own the CLI, embedded assets, release path, and architecture. |
| OSS developers | Need a usable local workflow and project-owned customization. |
| AI coding agents | Consume instructions, skills, and Arc42 evidence. |
| Contributors | Need stable Rust modules, contracts, and review gates. |
| CI systems | Need deterministic lint, test-runner, and archive outcomes. |
| Consumer repositories | Use Givn's workflow but do not host Givn's own acceptance fixtures. |
| Migrating project maintainers | Need a clean version-control boundary and an explicit plan for project-specific migration work. |
| Archiving developers | Need the archive proof to hold on the backend that owns their project, with the same candidate, receipt, and repair contract on Git and Fossil. |
| Automation callers around givn (for example the kiekste preview host) | Consume the run declaration and the `givn run` machine contract (ready line, exit codes, port hand-off) to start products without guessing. |

## 1.5 Acceptance Ownership

Givn's review, archive, and rollback behavior is accepted in this repository
through temporary-project fixtures that invoke the compiled CLI. Consumer
repositories retain their own domain specifications; they are not the hidden
acceptance host for Givn's product contract.
