# 4. Solution Strategy

This chapter summarizes the current implementation strategy. Decision rationale
is in the [ADR register](adr/README.md); mechanics are in
[chapter 8](08-crosscutting-concepts.md).

## 4.1 Strategy at a Glance

| Concern | Strategy |
|---|---|
| Language and packaging | Rust crate producing the `givn` CLI; defaults are embedded in the binary. |
| Configuration | Embedded manifest plus a `givn/config.yaml` policy/graph overlay, then a dedicated `givn/commands.yaml` schema for executable settings. |
| Facets | Project override wins over embedded template/instruction/skill asset. |
| Lifecycle | Manifest `requires` graph; standard order is proposal, specs, design, design-review, tasks, review, archive. New or unclassified work enters through `givn-start`, which may recommend an answer, direct work, ideation, or a formal change. |
| Specification | `.feature` files with `@givn.*` deltas; archive merges and strips givn tags. |
| Domain ownership | Explicit `usecase.md` documents own capabilities; reusable behavior is represented by typed fragments. |
| Ideation continuity | Stable use-case IDs, confirmed Persona references, and explicit Handoff connect topic state to proposals and permanent specs. |
| State | Filesystem-derived from effective artifact paths, markers, and `.givn-skip`. |
| Guidance | Skills only. Selected targets receive `<target>/skills/<name>/SKILL.md`; served instructions own the rules; project extensions are appended at request time. `givn-start` is the proportionate entry skill; `givn instructions` reports state without authorizing unrelated work. |
| Agent targets | `opencode`, `agents`, `claude`, `omp`, and `maki`. Invocation is harness-owned; no generated command files. |
| Commit attribution | `givn commit` is the SCM-neutral explicit-path commit path; on Git a checked-in message hook additionally stamps raw commits, while a Fossil checkout relies on `givn commit`. OpenCode, Claude, omp, and Maki adapters provide best-effort runtime values without making human commits a gate. |
| Test execution | `verify.command` and `verify.e2e_command` come from `givn/commands.yaml`; lint is separate. |
| Change verification | `givn check tasks` is the only change-scoped test execution: it records a receipt binding the tested revision, selected delta features, verify command, tested content, and green counts. With `ise` effective it also enforces the declared E2E package boundary, runs the change-scoped E2E scope when the delta has `@e2e` scenarios, and binds the E2E identity. Review validates that receipt without rerunning the change suite; archive still runs the merged suite once. |
| Review diagnostics | `givn check review` reports every use-case schema and capability-routing problem in one pass, grouped by document, while the schema stays strict and the existing gate order is preserved. |
| Review closure | One substantive review, then one targeted closure over recorded findings and material regressions from their fixes. Minor new observations enter the backlog; closure is not another open-ended audit. |
| Archive publication | Forward-only and SCM-backed on Git or Fossil: the merged candidate is published before verification, each scope runs once and reports fail-closed counts, and the receipt is committed as finalization. Successful scope evidence is persisted and reused only when its inputs match. |
| Product start | One author-owned `run:` declaration documents and gates how the built product starts; `givn run` executes it with a std-only readiness probe, a greppable ready line, a spec'd exit-code contract, and unix signal forwarding. |
| Gate safety | `givn check tasks|review` contains non-deactivatable mandatory checks; `post` is additive customization. |
| Code-structure report | The enabled-by-default `outline` addon is advisory over production Rust, Python, Java, TypeScript, and JavaScript. It prints each unit's per-component call tree first, then metrics and classification as hints above a neutral `SourceAnalyzer` port. It stores no baseline, has no verdict vocabulary, and never gates. |
| Addons | Every addon except an opt-in addon is enabled by default. `ise` joins as the one opt-in addon with an `architecture_principles` prerequisite, so a project that never enables it receives no new key, file, gate, or finding. Once a project enables it, an `ise`-effective change gains the declared E2E package boundary and the E2E-bound receipt. |
| Arc42 | Arc42 chapters are scaffolded at fixed `docs/arc42/`. The first completed change establishes the twelve-chapter baseline; later changes update affected chapters. |
| Structural overlap | Deterministic Fingerprint evidence and review dispositions. |
| Migration | The supported baseline is 1.0. The shipped catalog is empty; pre-1.0 projects follow a documented manual transition. Future catalog routes use the same resolver and bundle contract. |
| Migration-free upgrade | A release without a selected route synchronizes managed guidance and creates one allowlist-only commit when files changed. |

## 4.2 Configuration and Gate Strategy

The effective manifest is loaded as embedded defaults, overlaid by
`givn/config.yaml`, and then amended by the dedicated `givn/commands.yaml`
schema for `verify`, `coverage`, and `run`. Artifact fields are deep-merged by
id; the top-level `apply` block supplied by an overlay replaces its effective
block. Policy keys in `commands.yaml` and executable keys in `config.yaml` are
refused before use.

`post` is not the source of the core review contract. The binary always runs the
mandatory checks for explicit `givn check tasks` and `givn check review`:

- tasks: WIP protection for a change delta, the declared E2E package boundary
  when the change lock has `ise` effective, change verification, the
  change-scoped E2E execution, and the E2E-bound receipt;
- review: integrity, the deterministic run-declaration check, change-scoped
  lint, overlap dispositions, retirement disclosure, capability routing, the
  change verification receipt including its E2E identity, and scenario
  commits. The full suite is not re-run in review.

Built-in hook names cannot be downgraded with `allow_failure`. Custom shell
hooks can be configured and may be allowed to fail. A no-argument `givn check`
executes nothing. Archive has its own hard-coded candidate, verification,
coverage, and finalization stages in addition to configured archive hooks.

Before a fresh archive moves the change, and before an unproven candidate
completes its merge, the archive projects the accepted specification corpus
from an in-memory `SpecCorpus` and refuses an interaction-inventory mismatch,
an undeclared feature, or an unplaceable delta capability. Review and
change-scoped lint run the same projection, so the failure reaches the
developer while the change is still active. A merged candidate rechecks the
actual permanent inventory before its verification scopes. The projection and
the archive share one merge-text implementation (`spec::merge_plan`);
`spec::projection` owns the projected corpus and `spec::corpus` owns the
in-memory representation.

## 4.3 Quality Goals

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
uncommented project values. It requires a clean checkout of the selected
backend, captures a checkpoint, refreshes managed generated guidance, commits
only the allowlisted Givn paths, and reports the actual completed writes and
revisions. It performs no reset or history rewrite. Ejected facets, durable
docs, specs, and user-owned files remain project-owned and are handled by the
project through its normal review.

### Reliable evidence

Verification receipts bind stable inputs and fail closed. Archive recovery is
fix-forward with persisted scope evidence. Advisory metrics and missing
measurements are labeled unavailable rather than zero, and the outline report
never turns a hint into a verdict.

*[Architecture documentation index](README.md)*
