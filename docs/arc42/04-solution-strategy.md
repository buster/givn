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
| Development record | One append-only, scope-tagged local record of what Givn and the coding agent did, written best-effort by Givn's own capture points and by harnesses through a single intake command (ADR-073); `givn timeline` renders one scope as a self-contained interactive report. |
| SCM backend | One selection seam probes the project root: a Git worktree selects Git; a Fossil checkout selects Fossil; both markers, no version control, a nested root, or a missing executable are refused by name before mutation. The archive and the managed upgrade run on the selected backend. No configuration key declares the backend. ADR-071 records the boundary. |
| Agent integration | Selected `.opencode`, `.agents`, `.claude`, and `.omp` targets; only givn-owned fenced blocks may be changed in `AGENTS.md`. |
| Commit attribution | `givn commit` is the SCM-neutral commit path; on Git a checked-in message hook additionally stamps raw commits, while a Fossil checkout relies on `givn commit`. OpenCode, Claude, and omp adapters provide best-effort runtime values without making human commits a gate. |
| Test execution | `verify.command` and `verify.e2e_command` come from `givn/commands.yaml`; lint is separate. |
| Change verification | `givn check tasks` is the only change-scoped test execution: it records a receipt binding the tested revision, selected delta features, verify command, tested content, and green counts. Review and archive validate that receipt without rerunning the change suite; a documentation-only revision reuses the same proof, while any tested-content change makes it stale. Archive still runs the merged suite once. |
| Review diagnostics | `givn check review` reports every use-case schema and capability-routing problem in one pass, grouped by document, while the schema stays strict and the existing gate order is preserved. |
| Archive publication | Forward-only and SCM-backed (Git or Fossil): the merged archive candidate revision is published before verification, each verification scope runs once and reports fail-closed counts, and the verification receipt is committed as finalization. Publication is witness-bounded and exact on both backends (ADR-072). The active change moves to the archive location in one atomic rename after every pre-publication gate passes; an interrupted transition leaves either the active change or an unproven candidate, and an identical both-directory state is reconciled on rerun. A failed candidate is repaired forward, never rolled back. ADR-070 records the proof boundary. |
| Product start | One author-owned `run:` declaration in `givn/commands.yaml` documents and gates how the built product starts; `givn run` executes it with a std-only readiness probe, a greppable ready line, a spec'd exit-code contract, and unix signal forwarding. ADR-067 records the declare-and-execute boundary. |
| Gate safety | `givn check tasks|review` contains non-deactivatable mandatory checks; `post` is additive customization. |
| Code-structure report | An opt-in, default-off `outline` addon adds an advisory report over production sources. It prints every unit's method outline first, then shared metrics and classification as hints above a neutral `SourceAnalyzer` port; `SynAnalyzer` is the first adapter and tree-sitter can replace it without changing the report contract. A unit carries one classification and evidence, never an action or recommendation (schema version 2). The report stores no baseline and its hints never gate a change. |
| Structural review gate | The same addon adds a change-relative review gate inside `givn check review`: print the outline of every touched unit with methods at the change base and committed head, then require one fresh-context verdict per outline — `OUTLINE: PASS <unit> \| <responsibility; call order>` or `OUTLINE: FAIL <unit> \| <reason>`. A missing verdict, an empty PASS explanation, a FAIL, or conflicting verdicts blocks; the reviewer must differ from the author, and there is no justification path. Metric hints stay advisory. |
| Arc42 | `givn init` enables default-on addons unless `--no-addons`; Arc42 chapters are scaffolded at fixed `docs/arc42/`. |
| Structural overlap | Deterministic Fingerprint evidence and disposition gate. |
| Release migration | A clean project behind a declared route (currently from 0.7.0) receives a managed two-revision preparation and a visible maintenance change; project semantics remain LLM-reviewed. |
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
- review: `integrity`, the deterministic run-declaration check, and, for a
  named change, the overlap disposition gate. The full suite is not re-run in
  review; the archive verification receipt is the execution proof.

Built-in hook names cannot be downgraded with `allow_failure`. Custom shell
hooks can be configured and may be allowed to fail. A no-argument `givn check`
only runs configured post hooks and does not call the explicit artifact
mandatory-check path. Archive has its own hard-coded candidate, verification,
and finalization stages in addition to configured archive hooks.

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
uncommented project values. For a declared migration route it first requires
a clean checkout of the selected backend, refreshes managed generated
integrations, commits only the allowlisted Givn paths, and creates a migration
maintenance change that records the first revision. A project below the
oldest route the migration catalog declares (0.7.0) receives the managed
refresh only. Ejected
facets, durable docs, specs, and user-owned files remain project-owned and
are handled by the LLM through that change.

*[Architecture documentation index](README.md)*
