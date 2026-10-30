# Design: standalone-domain-model

## Overview

The change separates durable domain vocabulary from durable domain structure and removes the arc42 prerequisite from domain modeling.

Three boundaries define the work:

1. **Document boundary.** `docs/glossary.md` is the sole vocabulary authority. `docs/domain-model.md` is the thin structural map. `docs/arc42/12-glossary.md` becomes a link so an arc42 project has one unqualified definition table.
2. **Scaffold boundary.** Core documentation templates materialize as files at `givn init` and `givn addons enable`. Creation is conditional on the effective addons and never overwrites an existing file. No new command, manifest artifact, configuration key, or generator is introduced.
3. **Guidance and routing boundary.** The `givn-domain-modeling` skill is generated and routed whenever `domain_modeling` is enabled. The served instruction owns the document lifecycle, the ownership and change-impact checks, the term introduction test, and the migration of an existing arc42 glossary. Review, design-review, ideation, and arc42 guidance point at the correct owner.

## Visual Design Contract

- Interface: terminal
- Viewports: not applicable
- Design system: not applicable
- Persona: agent--technical-role, developer--primary-user, givn-maintainer--owner

The product surface is served CLI text and scaffolded Markdown. No browser screen is rendered: the assertions read stdout, stderr, and files written to the fixture project. The changed `@e2e` scenario commits a terminal transcript under `givn/changes/standalone-domain-model/evidence/visual/<scenario-slug>/transcript.txt`.

## Use-Case Traceability

Owning use case: `obtain-effective-guidance` (`givn/specs/obtain-effective-guidance/usecase.md`). Supporting owner: `initialize-project` (`givn/specs/initialize-project/usecase.md`) and the reusable `fragments` document (`givn/specs/fragments/fragment.md`).

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| The served content is the effective guidance resolved through the cascade | The domain-modeling instruction remains an embedded policy asset; project overrides still win | `guide-domain-modeling`, `An agent synchronizes skills and the domain skill is routed` |
| The built-in rules stay technology-agnostic | Document paths, ownership checks, and Mermaid notation only; no project-specific schema or language | `guide-domain-modeling`, `The domain model and glossary have separate owners` |
| Optional addons gate their own guidance | The domain skill depends only on `domain_modeling`; glossary scaffolding follows `arc42` or `domain_modeling` | `match-guidance-to-enabled-addons`, `Enabling domain modeling without arc42 generates the domain-modeling skill` |
| Initialization scaffolds project configuration and addon-owned files without overwriting | Two embedded documentation templates write only when the target file is absent | `enable-optional-addons`, `Existing domain documents survive initialization` |
| Initialization never overwrites project-owned content | The scaffold helper checks existence before writing and reports the preserved path | `enable-optional-addons`, `Enabling an addon preserves an existing glossary` |
| `--no-addons` opts out of optional addon behaviour | No addon scaffold runs, so neither durable document is written | `enable-optional-addons`, `givn init --no-addons produces no skill or command addon files` |
| Review checks vocabulary against its owner | The review and design-review ubiquitous-language block names `docs/glossary.md` and `docs/domain-model.md` | `review-architecture-documentation`, `Review guidance checks vocabulary and structure at their owners` |
| A change assesses structural impact | Review guidance requires the question and rejects no-op edits as evidence | `review-architecture-documentation`, `Review does not accept a no-op model edit as maintenance evidence` |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | `Cargo.toml` / `Cargo.lock`; no new dependency |
| cucumber-rs | existing workspace pin | `Cargo.lock`; `.fail_on_skipped()` remains the strict-mode mechanism |
| Mermaid | host-provided | The template requests `layout: elk` and top-down `flowchart TB`; no Mermaid CLI or browser is added to the project or test suite |
| Markdown templates | n/a | Existing `rust-embed` asset tree; no new renderer directive or schema |

## Decisions

- Decides D1: `docs/glossary.md` is the sole vocabulary authority for every project; `docs/arc42/12-glossary.md` becomes a link — realized in `assets/templates/glossary.md`, `assets/addons/arc42-docs/chapters/12-glossary.md`, and `assets/instructions/domain-modeling.md`.
- Decides D2: migration is agent-directed and preservation-first; no CLI transforms existing glossary content — realized in the served migration section and its scenarios.
- Decides D3: the templates ship as embedded assets and materialize as project files; they are not embedded in the instruction and do not require new instruction ids — realized in `src/cli/addons.rs` and `assets/templates/`.
- Decides D4: glossary scaffolding follows `arc42` or `domain_modeling`; domain-model scaffolding follows `domain_modeling`; `--no-addons` writes neither — realized in `scaffold_enabled`.
- Decides D5: the template requests ELK and the guidance states the host limitation; a rendered-diagram review is required — realized in the template and instruction.
- Decides D6: structural impact is an existing review question, not a new artifact, marker, or gate — realized in `assets/instructions/review.md` and `assets/instructions/design-review.md`.
- Decides D7: the template stays a thin map; inapplicable sections are removed and unknown structure remains qualified — realized in `assets/templates/domain-model.md` and the instruction.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Effective guidance | obtain-effective-guidance | `givn instructions`, next-step guidance, generated skills | `src/cli/instructions.rs` plus the embedded instruction assets | A second content source or project-specific guidance variants appear |
| Generated guidance | obtain-effective-guidance, initialize-project | `givn skills sync`, upgrade reconciliation | `src/skills/targets.rs` | Skill generation needs a second source of addon policy |
| Addon enablement and scaffold | initialize-project | `givn init`, `givn addons enable` | `src/cli/addons.rs` `scaffold_enabled` | A non-embedded template source, a third durable document kind, or automatic content migration is introduced |

### Module changes

- `src/cli/addons.rs`
  - `scaffold_enabled("arc42")` calls `scaffold_arc42()` and `scaffold_glossary()`.
  - `scaffold_enabled("domain_modeling")` calls `scaffold_glossary()` and `scaffold_domain_model()`.
  - A shared `scaffold_document(relative_path, asset_key, description)` creates `docs/` and writes the embedded asset only when the target does not exist; it prints `created <path>` or `exists <path> (content preserved)`.
  - Disabling an addon never deletes the documents, matching the existing arc42 behaviour.
- `src/skills/targets.rs`
  - `domain_modeling_present` returns `manifest.addons.domain_modeling`.
  - `reconcile_addon_artifacts` removes `givn-domain-modeling` only when `domain_modeling` is disabled.
- `src/cli/instructions.rs`
  - `domain_skill_to_load(domain_modeling)` takes one flag; every caller passes only the domain-modeling addon flag.
- `assets/templates/glossary.md` (new)
  - Heading, one-sentence authority statement, and a `| Term | Definition |` table scaffold.
- `assets/templates/domain-model.md` (new)
  - Scope and modeling status; a separate Mermaid notation legend; a top-down `flowchart TB` overview scaffold with `layout: elk`, collapsed aggregate and external-system notation; boundaries and responsibilities tables; relationships table with a governing-contract column; open modeling questions.
- `assets/addons/arc42-docs/chapters/12-glossary.md`
  - Heading plus a relative link to `../glossary.md`; no definition table.
- `docs/glossary.md` (new in this repository)
  - The current `docs/arc42/12-glossary.md` table, moved byte-for-byte apart from the heading level required by the new path.
- `docs/domain-model.md` (new in this repository)
  - A real model of givn's documented domain, not the example from the improvement note.
- `docs/arc42/12-glossary.md` (this repository)
  - Heading plus link to `../glossary.md`; the table is removed.
- Guidance assets updated: `assets/instructions/domain-modeling.md` (rewritten), `assets/skills/givn-domain-modeling/SKILL.md.tmpl`, `assets/instructions/ideation.md`, `assets/instructions/review.md`, `assets/instructions/design-review.md`, `assets/addons/arc42-docs/instructions.md`, `assets/addons/arc42-docs/chapters/README.md`, `assets/config.yaml.tmpl`.
- Documentation updated: `docs/arc42/08-crosscutting-concepts.md` (addon and documentation ownership), `docs/INDEX.md`, `docs/reference.md`, `docs/workflow.md`, `README.md` if it names the coupled requirement.

No persistent schema, routing table, or command surface changes. The new files are plain Markdown produced from embedded assets.

## Data Model Changes

None. No configuration key, manifest field, migration catalog, or state file is added. The scaffold decision reads the existing effective addon flags.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `guide-domain-modeling` | `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` | same file |
| `match-guidance-to-enabled-addons` | `tests/steps/template_conditional_rendering_steps.rs` | same file |
| `enable-optional-addons` | `tests/steps/default_all_addons_steps.rs` | same file |
| `review-architecture-documentation` | `tests/steps/arc42_review_alignment_steps.rs` | same file |

Test runner command (from `givn/commands.yaml`):

```text
./run-tests.sh --non-e2e
```

Single-scenario command:

```text
GIVN_FEATURES=givn/changes/standalone-domain-model/specs cargo test --locked --test features_runner -- --name "<scenario title>"
```

Strict mode is unchanged: `tests/features_runner.rs` calls `.fail_on_skipped()`, so an undefined or pending step fails the run.

## Interaction Coverage Matrix

The change adds no new user action. It varies existing CLI actions and scaffolds files during them. The existing smokes remain the E2E evidence.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `enable-optional-addons` — enable an addon (arc42 and domain_modeling are variants of the outline action) | Consumer enables the outline addon and it is listed as enabled | CLI / existing harness | Existing subprocess fixture runs `givn addons enable`; the new regular scenarios vary the addon and assert the scaffolded files |
| `initialize-a-project` — initialize with explicit noninteractive selection | Consumer runs init non-interactively and gets opencode skills | CLI / existing harness | Existing subprocess fixture runs `givn init`; the new regular scenarios assert the default scaffold and `--no-addons` absence |
| `refresh-generated-guidance` — synchronize skills and read the domain guidance | Synchronized guidance explains how to resolve the deterministic gates | CLI / existing harness | Existing subprocess fixture runs `givn skills sync` and `givn instructions`; modified assertions read the served instruction |

No scenario switches to a lower-level boundary. File assertions are secondary evidence for the scaffold; the CLI invocation remains the driving action.

## External Boundary Cases

| Boundary | Adversarial cases | Handling |
|---|---|---|
| Filesystem scaffold | `docs/` missing; target path already exists; target path exists as a directory; unwritable project root | Missing directory is created; an existing file or directory is preserved and reported; a write failure propagates as a non-zero addon/init failure without claiming success |
| Embedded assets | template asset missing from the binary | The command fails naming the missing asset; the test `all_documentation_templates_are_embedded` catches it at build time |
| Addon flags | `--no-addons`; arc42 disabled but domain modeling enabled; domain modeling disabled but arc42 enabled; both disabled | No scaffold for disabled guidance; glossary for arc42-only or domain-modeling; neither for both disabled |
| Existing content | existing glossary, existing model, existing chapter 12 | Never overwritten; chapter 12 migration is agent-directed |

## Local Runnability

No server, container, database, or external service is involved. `cargo test --locked --test features_runner` and `./run-tests.sh` run the whole suite locally. The CLI is started through the existing test harness.

## Coverage Process Boundaries

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Feature runner | `./run-tests.sh --non-e2e` / `--e2e` | `features_runner` and the `givn` binary via `CARGO_BIN_EXE_givn` | `target/llvm-cov-target` profraw | `merge-coverages.sh` | `src/cli/addons.rs` scaffold functions and `src/cli/instructions.rs` routing |
| Ready fixture | `run-tests.sh` before the suite | `ready_fixture` test target | same profraw set, discarded before measurement | n/a | fixture startup path only; no production source |

Measured population: production source under `src/`. Excluded: tests, generated code, coverage scripts, and embedded Markdown assets. Changed after result: forbidden.

## Out-of-Scope Technical Decisions

- No Markdown parser or transformer for existing glossary content.
- No new CLI command, flags, manifest artifact, or configuration key.
- No Mermaid renderer in the binary, test suite, or CI.
- No enforcement gate for the presence or freshness of the domain model.
- No migration of other projects' documents during `givn upgrade`; the scaffold runs only on init/addon enable, and existing content migration remains agent-directed.
