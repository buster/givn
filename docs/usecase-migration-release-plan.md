# Use-Case Migration Release Plan

## Status

This document is the implementation handoff for the release that replaces the
group-oriented specification model with a semantic use-case model and promotes
ideation, personas, and use-case context through the workflow.

Current Givn package version: `0.6.0`.

The release target is `0.6.0`. The migration bundle is published under
`assets/migrations/0.5.0-to-0.6.0/`.

### Current Execution Status

- The canonical use-case model, migration coverage implementation, and
  ideation/Persona guidance were delivered together and archived as
  `make-usecases-canonical`; they were not separate archived Changes 1–3.
- Watn migration is archived and passes behavior, E2E, interaction, and line
  coverage comparison.
- Kiekste migration is archived and passes behavior, E2E, interaction, and
  line comparison; its `run-authority` Handoff remains explicitly pending.
- The 0.5.0-to-0.6.0 bundle is concrete and cataloged.
- Release closure is still blocked by the Givn source-coverage comparison:
  `7162/13704` before versus `7439/18235` after. Branch evidence is `0/0` on
  both sides and is therefore not materially measured.
- The Givn self-migration ledger explicitly records four intentional retired
  vocabulary assertions and one created canonical contract. The machine gate
  passes behavior/E2E/interaction comparison after applying those retirements;
  only source coverage remains blocking.

No backward-compatible runtime reader is planned. Existing consumer projects
must be migrated by the release-owned LLM-guided migration change.

Historical archives are excluded from migration. They retain the vocabulary and
paths that were valid when those changes were completed.

## Outcome

The active specification model is:

```text
givn/specs/<usecase-id>/
├── usecase.md
└── <capability>.feature
```

Reusable non-use-case behavior is separate:

```text
givn/specs/fragments/
├── fragment.md
└── <fragment>.feature
```

Ideation uses the same use-case identity and semantic document, at an earlier
lifecycle stage:

```text
givn/ideation/<topic-id>/use-cases/<usecase-id>.md
    -> explicit Handoff
givn/changes/<change-id>/specs/<usecase-id>/usecase.md
    -> archive
givn/specs/<usecase-id>/usecase.md
```

The release must support semantic corpus migration. It must not implement a
mechanical `group.md` rename and call the result complete.

## Fixed Decisions

### Vocabulary

Use these terms in active code, assets, documentation, and CLI output:

| Term | Meaning |
|---|---|
| Use case | A domain goal or summary of domain goals that owns a coherent narrative and, for leaf goals, capabilities. |
| Capability | One executable Gherkin feature file representing a behavior unit. |
| Fragment | Reusable behavior without an independent user goal. |
| Actor | A role that performs a domain interaction. |
| Stakeholder | A party affected by or interested in a use case. |
| Persona | A confirmed stakeholder segment used as a review lens. |
| Use-case ID | Stable kebab-case identity shared by ideation, change deltas, permanent specs, and relationships. |
| Change seed | An ideation handoff record from which a normal Givn change starts. |

The words `group`, `group.md`, `GroupDoc`, `group_md`, `NEW in <group>`, and
`owning group` are removed from active contracts. Unrelated technical uses of
the word `group`, such as process groups or generic collection variables, are
not renamed.

### Goal levels

| Level | Meaning | Permanent representation |
|---|---|---|
| `!` | User-goal use case | Own `usecase.md` and capabilities. |
| `+` | Summary use case | Own `usecase.md`; includes child use cases. |
| `-` | Subfunction or flow step | No permanent use-case directory. |

Summary use cases are real use cases. Their coverage is the union of their
children and is not counted a second time.

### Fragments

The old support-style collections are not renamed into fake use cases. Each old
capability is classified individually:

- Real user-goal behavior becomes a leaf use case.
- A broad domain area becomes a summary use case with child use cases.
- Reusable behavior with no independent goal becomes a fragment.
- Obsolete behavior is retired explicitly, never silently dropped.

Fragments may be included by use cases. They are not normally extended.

### Compatibility

The final runtime contract reads only `usecase.md` and the new use-case paths.
There is no fallback reader for `group.md`, no old JSON alias, and no old
routing syntax alias.

The migration bundle is the compatibility mechanism for existing projects.

### Coverage

Migration has three hard non-regression dimensions:

1. Specification coverage: no behavior scenario, capability, or interaction is
   lost without explicit retirement.
2. E2E coverage: no `@e2e` evidence or real-interface interaction is lost.
3. Source coverage: line and branch coverage after migration are not lower than
   before migration.

## Release Workstreams

The release is implemented through four ordered Givn changes:

```text
make-usecases-canonical
        |
        +--> add-spec-coverage-nonregression
        |
        +--> promote-ideation-personas
                       |
                       v
              finalize-usecase-migration-release
                       |
                       v
             migrate Watn, then Kiekste
```

Each workstream is a normal Givn change with proposal, specs, design,
design-review, tasks, implementation, review, and archive. Each scenario is
implemented in its own RED/GREEN/REFACTOR/COMMIT cycle.

Do not implement multiple workstreams concurrently when they touch the same
production files or asset contracts.

## Change 1: make-usecases-canonical

### Purpose

Make `usecase.md`, use-case IDs, capability ownership, fragments, and typed
relationships the only active specification model.

### Production surfaces

| Responsibility | Files |
|---|---|
| Narrative parser and template | `src/gherkin/group_md.rs` renamed to `src/gherkin/usecase_md.rs` |
| Module registration | `src/gherkin/mod.rs` |
| Interaction coverage | `src/gherkin/interaction.rs` |
| Relationship graph | `src/overlap/analysis.rs` |
| Tree and find output | `src/spec/tree.rs` |
| Route recommendation | `src/spec/route.rs` |
| Use-case proposal clustering | `src/spec/cluster.rs` |
| Routing consistency | `src/spec/routing_table.rs` |
| Retrieval path projection | `src/spec/retrieval.rs` |
| Spec CLI output | `src/cli/spec.rs` |
| Lint severity and findings | `src/cli/lint.rs` |
| Review checks | `src/cli/check.rs` |
| Archive merge and rollback | `src/cli/archive.rs` |
| Artifact glob and status behavior | `src/state.rs` |
| CLI command names | `src/main.rs` |

### Parser contract

Replace `GroupDoc`, `parse_group_md`, and `group_md_template` with
`UsecaseDoc`, `parse_usecase_md`, and `usecase_md_template`.

The parser must recognize and validate:

- stable use-case ID;
- goal level, `!` or `+`;
- actors;
- goal;
- trigger;
- preconditions;
- main flow;
- extensions;
- rules;
- examples;
- minimal guarantee;
- success guarantee;
- persona references;
- capability references;
- interaction inventory;
- typed Includes;
- typed Extends;
- out-of-scope boundary;
- Mermaid diagram.

Unknown headings must be rejected. Content must never bleed into the previous
recognized section.

The parser must require the sections needed for the document kind. A summary
use case may have child use cases and no direct capabilities. A fragment
document has a separate schema and is not parsed as a use case.

### Typed relationships

Use a machine-readable form that remains readable in Markdown:

```markdown
## Includes

- usecase: authorize-build-start
- fragment: transport

## Extends

- usecase: archive-with-coverage
```

The relationship graph must:

- resolve every target;
- distinguish use-case IDs from fragment IDs;
- reject cycles;
- reject duplicate or ambiguous targets;
- reject extensions targeting fragments unless the design explicitly allows it;
- report exact source paths and relationship entries in findings.

### Interaction inventory

The inventory becomes capability-aware:

```markdown
## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| project-run | hand over a project | Consumer hands over a project |
| project-status | inspect current state | Consumer inspects project status |
```

The verifier must validate:

- every capability reference exists;
- every interaction has exactly one matching `@e2e` scenario;
- every `@e2e` scenario maps to one interaction;
- summary use cases roll up descendant coverage without double-counting;
- fragment interactions are not copied into every including use case.

### CLI contract

Replace active group terminology in public output:

| Current | New |
|---|---|
| `GroupRow` | `UsecaseRow` |
| `RouteCandidate.group` | `RouteCandidate.usecase` |
| `suggested_group` | `suggested_usecase` |
| `NEW in <group>` | `NEW in <usecase-id>` |
| `givn spec regroup` | `givn spec propose-usecases` |
| `group_md_invalid` | `usecase_invalid` |
| `group` output field | `usecase` output field |

Because this release is not backward-compatible, do not retain aliases.
Update all JSON and text-output feature scenarios at the same time.

### Archive behavior

The `specs` artifact owns both feature deltas and use-case documents.

For a new use case, archive creates the permanent use-case directory and
installs `usecase.md` before or atomically with feature merge.

For an existing use case, the change-side `usecase.md` is a complete desired
replacement. It records a base content hash or equivalent revision. Archive
must refuse replacement when the permanent document has changed since the
change was authored.

The existing specs backup and rollback transaction must include:

- permanent use-case documents;
- fragment documents;
- feature files;
- relationship changes.

For a pure feature-only change, no use-case replacement occurs. For a pure
refactoring, the specs artifact may be skipped.

### Givn corpus migration

Migrate Givn's own corpus in this change. Do not mechanically rename all seven
current narratives.

Classify and transform:

- `givn/specs/archival/`;
- `givn/specs/authoring/`;
- `givn/specs/maintenance/`;
- `givn/specs/spec-discovery/`;
- `givn/specs/status/`;
- `givn/specs/support/`;
- `givn/specs/verification/`.

The support collection must be split into real use cases and fragments where
the content requires it.

Historical `givn/archive/` content is not rewritten.

### Scenario families

The specs for this change must cover:

- valid use-case document parsing;
- invalid heading and identity detection;
- summary use-case parsing;
- fragment parsing;
- typed includes and extends;
- dangling-reference failure;
- relationship-cycle failure;
- use-case tree output;
- use-case route output;
- use-case routing consistency;
- capability ownership moves;
- new use-case creation;
- use-case document replacement;
- stale use-case replacement failure;
- archive rollback preserving use-case documents;
- use-case-aware interaction coverage.

## Change 2: add-spec-coverage-nonregression

### Actual delivery

Implemented inside `make-usecases-canonical`; no separate Change 2 archive was
created. The closure change adds the missing regression scenarios and explicit
retirement handling.

### Purpose

Make corpus migration coverage machine-checkable rather than an LLM claim.

### New commands

Add:

```text
givn spec coverage --format json
givn spec coverage --baseline <path>
givn check migration --change <migration-id>
```

The migration check becomes mandatory when the change contains `migration.yaml`.
It also runs as part of the review check for aggregate migration changes.

### Specification inventory

The coverage snapshot records:

- feature path;
- use-case owner;
- capability;
- scenario title;
- deterministic behavior hash;
- `@e2e` status;
- interaction mapping;
- persona impact when declared.

Paths are not behavior identity. A moved feature remains the same behavior when
its canonical hash and mapped scenario identity remain unchanged.

### Migration ledger

The migration change must contain a ledger such as:

```markdown
| Old entry | Old owner | Operation | New owner | New capability | Old @e2e | New @e2e | Behavior changed | Reason |
|---|---|---|---|---|---:|---:|---|---|
```

Allowed operations:

- `MOVE`;
- `SPLIT`;
- `MERGE`;
- `REASSIGN_CAPABILITY`;
- `CLASSIFY_AS_FRAGMENT`;
- `CREATE`;
- `RENAME`;
- `RETIRE`;
- `UPDATE_RELATIONSHIP`.

Every old scenario must map exactly once unless it is explicitly retired.

### Source coverage

Reuse `src/cobertura.rs` to compare before and after reports:

```text
line coverage after >= line coverage before
branch coverage after >= branch coverage before
```

Any decrease is a blocking migration finding. The migration report must include
covered and total counters, not just percentages.

### Migration evidence

The migration change retains:

```text
usecase-migration.md
coverage-before.json
coverage-after.json
coverage-comparison.json
source-coverage-before.json
source-coverage-after.json
```

### Scenario families

Cover:

- unchanged behavior after a feature move;
- complete mapping after a group split;
- complete mapping after a group merge;
- fragment classification without coverage loss;
- new use-case creation with preserved scenarios;
- missing-scenario detection;
- lost-E2E detection;
- lower interaction coverage detection;
- lower line-coverage detection;
- lower branch-coverage detection;
- unresolved behavior-change detection.

## Change 3: promote-ideation-personas

### Actual delivery

Implemented inside `make-usecases-canonical`; no separate Change 3 archive was
created. Generated source assets and configured targets were synchronized in the
same delivery.

### Purpose

Use the same use-case model from ideation through permanent specifications and
make confirmed personas visible throughout proposal, specification, design,
implementation, review, and archive.

### Source assets

Update these source assets, then regenerate all configured targets:

```text
assets/skills/givn-ideation/SKILL.md.tmpl
assets/skills/givn-explore/SKILL.md.tmpl
assets/skills/givn-propose/SKILL.md.tmpl
assets/skills/givn-spec/SKILL.md.tmpl
assets/skills/givn-design/SKILL.md.tmpl
assets/skills/givn-design-review/SKILL.md.tmpl
assets/skills/givn-tasks/SKILL.md.tmpl
assets/skills/givn-implement/SKILL.md.tmpl
assets/skills/givn-review/SKILL.md.tmpl
assets/skills/givn-archive/SKILL.md.tmpl
assets/skills/givn-domain-modeling/SKILL.md.tmpl
assets/instructions/
assets/templates/
assets/commands/
```

Generated copies under `.agents`, `.claude`, and `.opencode` are outputs, not
manual edit targets.

### Required behavior

- Ideation drafts use the permanent use-case schema.
- Change seeds carry stable use-case IDs.
- Proposal records use-case identity and confirmed affected personas.
- Specs derive behavior from the use-case contract.
- Feature files use actors, not persona biographies.
- Design records domain guarantees and persona impact.
- Design-review records persona findings and user dispositions.
- Tasks copy relevant domain constraints into each scenario task.
- Implementation agents read the use-case context but treat Gherkin as the
  executable authority.
- Review confirms that implementation preserves the use-case contract.
- Archive preserves the accepted use-case document.

### Workflow consumption matrix

Every existing workflow phase is updated only where the use-case or persona
context affects its responsibility:

| Phase or surface | Required context | Required behavior |
|---|---|---|
| `/givn-ideation` | Topic use-case drafts, stakeholder types, confirmed personas, events, open questions | Produce the shared use-case schema, stable IDs, persona review records, and change seeds. Handoff remains explicit and never starts a change. |
| `/givn-explore` | User intent to move from exploration to structured domain work | Route users to `/givn-ideation` when they need durable use cases, personas, or multiple future changes. |
| `/givn-propose` | Change seed, permanent use-case ID, confirmed personas, relevant topic decisions | Read the selected use-case source, record use-case identity and persona impact in `proposal.md`, and preserve the WHAT/WHY boundary. |
| `proposal` instructions/template | Use-case identity and affected stakeholders | Add the optional use-case context contract and require capability routing with `NEW in <usecase-id>` or `EXTEND <capability>`. |
| `/givn-spec` | `proposal.md`, `usecase.md`, actor/persona distinction, interaction inventory | Write feature deltas under the owning use-case ID, update the use-case interaction inventory, and derive observable behavior from the use-case contract. |
| `specs` instructions/template | Permanent `usecase.md`, capability ownership, typed relationships | Replace group wording, require capability-aware inventory entries, preserve actors in Gherkin, and keep persona biographies out of feature files. |
| `/givn-design` | Use-case guarantees, rules, events relevant to the change, persona impact | Add domain-to-architecture traceability and map use-case guarantees to technical decisions and E2E coverage. |
| `design` instructions/template | Selected use-case contract and persona/stakeholder impact | Require the use-case traceability table, relevant persona impact, unresolved-question disposition, and normal strict-mode/E2E design fields. |
| `/givn-design-review` | Proposal, use-case document, personas, design traceability | Add a domain-context branch that checks value preservation, persona coverage, actor/persona distinction, relationship correctness, and explicit dispositions. |
| `/givn-tasks` | Design constraints plus affected use-case guarantees and personas | Copy domain constraints into every scenario task so implementation agents cannot lose use-case context. |
| `/givn-implement` | Proposal, use-case context, feature scenarios, design, task constraints | Read the use-case context, preserve approved domain terms, and treat the feature file as executable authority. |
| `/givn-review` | Use-case contract, persona impact, migration ledger where applicable | Verify implementation conformance, persona-relevant outcomes, use-case document updates, and no fabricated or downgraded E2E coverage. |
| `/givn-archive` | Use-case delta, base revision, feature deltas, coverage evidence | Merge or replace use-case documents atomically with features, block stale replacements, and preserve migration evidence. |
| `givn instructions` | Current project/change state and resolved artifact instructions | Update the generic overview and artifact guidance so agents are told to consult the owning use case and ideation path where relevant. Do not inject full persona biographies into the JSON contract. |

The source of truth for these changes is the asset set under `assets/`. The
generated `.agents`, `.claude`, and `.opencode` copies are regenerated and
verified after the source assets change.

### Persona rules

- Never invent a persona.
- Never auto-promote a persona.
- Never silently alter a persona record.
- Validate referenced persona IDs when a use case declares them.
- Keep persona content human-reviewed Markdown in `givn/personas/`.
- Keep non-ideated changes valid without persona context.

### Scenario families

Cover generated guidance that:

- uses use-case IDs;
- replaces group paths with use-case paths;
- distinguishes actors and personas;
- carries use-case context into proposal;
- carries persona impact into design;
- requires persona disposition in design-review;
- preserves context in tasks, implementation, review, and archive;
- never performs Handoff automatically;
- never teaches `group.md`.

## Change 4: add-usecase-release-migration

### Purpose

Ship the release-owned LLM migration bundle from Givn `0.5.0` to `0.6.0`.

### Assets

Add:

```text
assets/migrations/0.5.0-to-0.6.0/proposal.md
assets/migrations/0.5.0-to-0.6.0/design.md
```

Add the catalog entry in:

```text
assets/migrations/catalog.yaml
```

The migration bundle must explain semantic transformation, not a filename
rename.

### Migration phases

#### Phase 1: Preflight

The LLM must:

- require a clean Git worktree;
- record Givn version and configuration overrides;
- enumerate active changes;
- enumerate permanent features and narrative files;
- enumerate ideation topics and personas;
- create coverage baselines;
- stop on unidentified uncommitted work.

#### Phase 2: Corpus inventory

For every old directory, classify it as:

- leaf use case;
- summary use case;
- fragment collection;
- mixed group requiring split;
- obsolete or retired content.

Produce the mapping ledger before edits.

#### Phase 3: Confirmation

Present and confirm:

- group-to-use-case mapping;
- capability moves;
- fragment classifications;
- new use cases;
- summary/child relationships;
- persona references;
- relationship changes;
- coverage impact.

No semantic move is applied without confirmation.

#### Phase 4: Apply moves

Use `git mv` where files are moved. Create and update `usecase.md`,
`fragment.md`, relationships, inventories, and active references.

Do not rewrite observable scenario behavior during this phase. Any required
behavior change becomes a separate normal Givn change.

#### Phase 5: Active changes

For every active change:

- move delta paths if use-case IDs changed;
- update proposal routing;
- update design interaction matrices;
- update review evidence;
- update tasks and implementation paths;
- preserve scenario text and tags;
- re-run scoped lint and review checks.

#### Phase 6: Ideation

- Preserve all topic state.
- Map ideation use-case IDs to permanent IDs.
- Do not perform Handoff automatically.
- Do not promote personas automatically.
- Record elaborated use cases with no permanent implementation.
- Record permanent use cases with no ideation source.

#### Phase 7: Coverage

Run:

```text
givn spec coverage --format json
givn check migration --change <migration-id>
verify.command
verify.e2e_command
coverage.measure_command
coverage.merge_command
```

Block on any specification, E2E, line, or branch coverage regression.

#### Phase 8: Final audit

The migration is incomplete while active project files contain:

- `group.md`;
- `GroupDoc`;
- `group_md`;
- `NEW in <group>`;
- `owning group`;
- unresolved old paths;
- unmapped capabilities or scenarios.

The audit excludes `givn/archive/`.

## Fresh-Context Session Protocol

Every implementation session starts with:

```text
givn instructions
```

The agent then reads the current change's proposal, specs, design, and tasks.

No fresh-context agent assumes another agent's self-report. It verifies files,
tests, and Git history directly.

### Session 1: Contract and design

Create and design `make-usecases-canonical`.

Resolve the schema, typed relationships, fragments, archive semantics, CLI
contract, coverage ownership, and no-compatibility boundary.

Stop after design-review. Do not implement.

### Session 2: Parser and document schema

Implement the parser and document model one scenario at a time.

Exit criteria:

- valid use-case documents parse;
- invalid documents fail clearly;
- summary, leaf, and fragment documents are distinct;
- typed relationships parse;
- unknown headings fail.

### Session 3: Discovery and relationship graph

Implement tree, route, find, lint, relationship resolution, cycle detection,
and typed fragment/use-case output.

Exit criteria:

- no active output uses group terminology;
- all relationship scenarios pass;
- JSON output matches the new contract.

### Session 4: Archive and use-case deltas

Implement new, modified, stale, feature-only, use-case-only, and refactoring
archive paths.

Exit criteria:

- use-case documents archive atomically;
- stale replacement blocks before mutation;
- rollback restores use-case and feature trees;
- pure refactoring does not require a use-case document.

### Session 5: Coverage comparison

Implement specification snapshots, migration ledger validation, E2E checks, and
Cobertura comparison.

Exit criteria:

- missing behavior blocks;
- lost E2E blocks;
- lower interaction coverage blocks;
- lower line coverage blocks;
- lower branch coverage blocks.

### Session 6: Givn corpus migration

Transform Givn's own permanent corpus, support fragments, feature paths, tests,
and documentation.

Exit criteria:

- active corpus contains no `group.md`;
- Givn's full test suite passes;
- Givn's own coverage does not regress;
- migration ledger accounts for every moved feature and scenario.

### Session 7: Ideation and persona guidance

Update source assets, regenerate targets, and add generated-content scenarios.

Exit criteria:

- all configured targets receive identical canonical guidance;
- proposal-to-archive use-case traceability is explicit;
- persona confirmation and Handoff rules are explicit;
- no generated active guidance teaches groups.

### Session 8: Release migration bundle

Write and validate the release proposal/design assets and catalog entry.

Exit criteria:

- migration operations are explicit;
- the mapping ledger is required;
- coverage baselines are required;
- active changes are covered;
- ideation Handoff is not implicit;
- historical archives are protected.

### Session 9: Watn dogfood

Watn is currently clean and has no active changes.

Migration tasks:

- classify `corpus-infra` as fragments or split its capabilities;
- determine whether the remaining directories are leaf or summary use cases;
- create the mapping ledger;
- move capabilities with `git mv`;
- create proper use-case documents;
- preserve all scenarios and `@e2e` evidence;
- run source and spec coverage comparisons;
- archive the migration change.

Do not edit Watn's archived migration history.

### Session 10: Kiekste dogfood

Kiekste's ideation and migration work is intentionally resolved. The worktree
is clean, the migration is archived, and the `run-authority` topic remains
Handoff-pending.

Migration tasks:

- baseline the permanent and active corpus;
- classify `run-a-project`, `follow-the-run`, and `runtime-boundary`;
- use `run-authority/use-case-definitions.md` as confirmed domain evidence;
- map the six elaborated use cases;
- split summary use cases into child use cases where confirmed;
- `change-run-scheduler` was archived before the migration on the dogfood branch;
  its historical archive remains untouched as required;
- the migration ledger records that sequencing deviation;
- preserve its feature paths and E2E scenarios unless a use-case ID changes;
- preserve the four confirmed personas in topic state;
- do not perform Handoff automatically;
- run all three coverage comparisons.

### Session 11: Release verification

Run the full Givn suite, generated-target checks, migration tests, Watn suite,
and Kiekste suite.

Then follow the repository dogfood sequence after each archived Givn change:

```text
cargo install --path . --locked --force
./run-tests.sh
./run-tests.sh --e2e
```

## Complete File Inventory

### Core production code

```text
src/gherkin/group_md.rs -> src/gherkin/usecase_md.rs
src/gherkin/mod.rs
src/gherkin/interaction.rs
src/overlap/analysis.rs
src/spec/tree.rs
src/spec/route.rs
src/spec/cluster.rs
src/spec/routing_table.rs
src/spec/retrieval.rs
src/cli/spec.rs
src/cli/lint.rs
src/cli/check.rs
src/cli/archive.rs
src/cli/migration.rs
src/cli/instructions.rs
src/cobertura.rs
src/state.rs
src/main.rs
```

### Source assets

```text
assets/default-manifest.yaml
assets/instructions/
assets/templates/
assets/skills/
assets/commands/
assets/migrations/catalog.yaml
assets/migrations/0.5.0-to-0.6.0/proposal.md
assets/migrations/0.5.0-to-0.6.0/design.md
```

### Givn corpus and tests

```text
givn/specs/
givn/specs/fragments/
givn/.spec-index.json
givn/.spec-pair-cache.json
tests/steps/spec_discovery_deterministic_steps.rs
tests/steps/grouped_merge_steps.rs
tests/steps/mod.rs
tests/features_runner.rs
```

### Durable documentation

```text
docs/INDEX.md
docs/workflow.md
docs/reference.md
docs/migrations/README.md
docs/arc42/06-runtime-view.md
docs/arc42/08-crosscutting-concepts.md
docs/arc42/11-risks-and-technical-debt.md
docs/arc42/12-glossary.md
```

### Generated outputs

Regenerate and verify, but do not hand-edit:

```text
.agents/
.claude/
.opencode/
```

### Consumer migration locations

Watn:

```text
/home/buster/projects/watn/givn/specs/
/home/buster/projects/watn/givn/changes/
/home/buster/projects/watn/givn/ideation/
/home/buster/projects/watn/givn/personas/
/home/buster/projects/watn/.agents/
/home/buster/projects/watn/.claude/
/home/buster/projects/watn/.opencode/
```

Kiekste:

```text
/home/buster/projects/kiekste/givn/specs/
/home/buster/projects/kiekste/givn/changes/change-run-scheduler/
/home/buster/projects/kiekste/givn/ideation/run-authority/
/home/buster/projects/kiekste/givn/personas/
/home/buster/projects/kiekste/.agents/
/home/buster/projects/kiekste/.claude/
/home/buster/projects/kiekste/.opencode/
```

## Completion Definition

The release is complete only when:

- Givn has no active group-based runtime contract;
- Givn's own corpus is semantically migrated;
- fragments are explicit;
- summary and leaf use cases are represented correctly;
- use-case documents archive atomically;
- stale use-case updates block safely;
- ideation uses the same use-case identity;
- personas are referenced and reviewed without being invented;
- migration bundles can split, merge, move, create, classify, and retire;
- Watn migration passes;
- Kiekste migration passes after its worktree is clean;
- no behavior scenario is lost;
- no E2E evidence is lost;
- no interaction coverage is lost;
- line coverage is not lower;
- branch coverage is not lower;
- generated guidance is synchronized;
- archived history remains untouched;
- all migration evidence is committed and reviewable.
