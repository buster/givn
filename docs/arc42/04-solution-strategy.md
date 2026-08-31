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
| State | Filesystem-derived from effective artifact paths, markers, and `.givn-skip`. |
| Agent integration | Selected `.opencode`, `.agents`, and `.claude` targets; only givn-owned fenced blocks may be changed in `AGENTS.md`. |
| Test execution | `verify.command` and `verify.e2e_command` come from `givn/commands.yaml`; lint is separate. |
| Gate safety | `givn check tasks|review` contains non-deactivatable mandatory checks; `post` is additive customization. |
| Arc42 | `givn init` enables addons by default unless `--no-addons`; Arc42 chapters are scaffolded at fixed `docs/arc42/`. |
| Structural overlap | Deterministic Fingerprint evidence and disposition gate. |
| Semantic retrieval | Opt-in E5 vectors and permanent content-hash index. |
| Semantic strictness | One unified change-scoped rulebook (single E5 `0.90` admission, hybrid selection, per-pair judgement cache) drives advisory review, blocking review, and the archive gate. |
| BGE/NLI | Optional local, bidirectional evidence; the change-scoped rulebook pool for review surfaces, surface-specific pools elsewhere, explicit statuses, no automatic Givn classification. |
| Model availability | Explicit semantic commands resolve and validate missing artifacts in the XDG user cache; project-local index state remains separate. |
| Second-stage budget | Current checkout implements the proposed positive `GIVN_SPEC_MAX_RERANK_PAIRS` budget; ADR-063 remains proposed while its change is incomplete. |
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
- review: `verify.command`, `verify-e2e`, `integrity`, and, for a named change,
  overlap and retrieval checks.

Built-in hook names cannot be downgraded with `allow_failure`. Custom shell
hooks can be configured and may be allowed to fail. A no-argument `givn check`
only runs configured post hooks and does not call the explicit artifact
mandatory-check path. Archive has its own hard-coded post-merge verify/e2e and
optional coverage stages in addition to configured archive hooks.

## 4.3 Semantic Strategy

The semantic model input is one complete, compact, deterministic serialization
of parsed Gherkin: feature and optional rule context, inherited backgrounds,
scenario identity and description, resolved step roles, tables, docstrings,
outline information, and examples. Empty optional fields are omitted. The exact
E5 passage text is hashed for index reuse; paths, tags, scores, classifications,
and Fingerprint evidence remain metadata.

The actual consuming tokenizer preflights E5 passages and selected BGE/NLI
pairs without truncation or padding. Fitting E5 passages enter the index;
over-cap passages are additive metadata with no vector or semantic pair score.

Model setup precedes E5 token measurement. E5 uses FastEmbed's cache-aware
initialization; BGE and NLI use validated synchronous artifact resolution. The
default cache is user-level XDG storage, while explicit model paths remain a
controlled escape hatch.

Second-stage selection is not one universal pool:

| Surface | E5 candidate input | BGE selection | NLI selection |
|---|---|---|---|
| Duplicate Audit | all remaining semantic pairs after exact/structural findings | endpoint top-five union, then E5-ranked budget | full E5 semantic set, independently budgeted |
| Explore | at most five ranked permanent/request-local results | that result set, then budget | that result set, independently |
| Advisory review | existing delta/permanent and projected retrieval pool | that pool, then budget | that pool, independently |
| Blocking review | the rulebook-selected pairs at E5 `0.9000` | selected pairs, then budget | selected pairs, independently |

`BGE_FILTERED` and `NLI_FILTERED` are non-blocking evidence. `BGE_UNSCORED`
token-cap states remain visible and blocking. A combined recommendation is the
intersection of participating model recommendations.

## 4.4 Quality Goals

### Configurability without forking

The cascade controls individual assets and the overlay controls the artifact
graph. Projects can change a template, instruction, dependency, hook, or graph
node without changing the binary.

### Installability

The product target is one native executable with embedded defaults. The default
Cargo install enables all declared retrieval capabilities; `--no-default-features`
remains the explicit minimal-build escape hatch.

### Upgradeability

`givn upgrade` refreshes the managed configuration scaffold while preserving
uncommented project values. For the exact 0.2.0-to-0.3.0 route it first requires
a clean Git worktree, refreshes managed generated integrations, commits only
the allowlisted Givn paths, and creates a migration maintenance change that
records the first commit. Ejected facets, durable docs, specs, and user-owned
files remain project-owned and are handled by the LLM through that change.

## 4.5 Current Evidence Boundary

The PR #27 report records `394` permanent scenarios, `1274` E5 candidates, and
`795` BGE top-five-union pairs for the current local corpus. Its raw JSON inputs
are local and ephemeral. Earlier values (`288/414?`, `385/100`,
`397/1311/803`) are historical and are not current strategy baselines.
