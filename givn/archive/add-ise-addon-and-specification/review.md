# Review: add-ise-addon-and-specification

Review kind: independent
Author: add-ise-addon-and-specification author (omp)
Reviewer context: fd3e0fd7-9ae1-4767-b2b2-dc6ddd46984f (the review context that produced the finding brief)

README-IMPACT: none

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Review context and scope

This is the independent review required by the proposal. The reviewer did not
author the change and received only the artifacts, the repository state, and
the finding brief. The review read `proposal.md`, `design.md`,
`design-review.md`, `arc42.md`, `tasks.md`, every delta feature and use-case
document, the production diff `60db843b..HEAD`, every step file that carries a
new binding, the six committed transcripts, and the change verification
receipt. It ran only read-only checks: `givn status`, `givn check tasks`, and
`givn check review`; it did not rerun the suites and did not edit production
code, tests, specs, design, tasks, glossary, coverage, or README.

The README decision is `none`: the README states no add-on or configuration
fact this change falsifies. Its "Built-in architecture & code health addons"
list is a curated subset (it already omits `coverage`, `readme`, `challenge`,
`dev_principles`, `architecture_principles`, and `communication_policy`), and
the new addon, file, and findings are documented in `docs/reference.md`, which
the README links as the reference.

## Tasks preflight

`givn status --change add-ise-addon-and-specification` reports 207/219 boxes
checked. The 12 unchecked boxes are the six Setup process boxes and the six
Closeout boxes; all 207 behavior/scenario boxes are checked. The scenario
evidence is complete (51 delta scenarios, each with a `COMMIT:` line), the
setup artifacts exist, and this review verified strict mode independently
(below). The unchecked Setup and Closeout boxes are still a task-record finding
(F6): the author must record the strict-mode probe output, tick the Setup
boxes, run and record the Closeout commands, and tick the Closeout boxes
before archive.

## Implementation evidence check

### 1. `@e2e` tag integrity

No delta `@e2e` tag was removed. `git diff 60db843b..HEAD` over the delta specs
shows only `@wip` removals (`- @givn.added @e2e @wip` / `+ @givn.added @e2e`)
for five scenarios. The delta contains six `@e2e` scenarios and 51
non-retired scenarios (56 scenario lines minus five `@givn.removed`
placeholders). The delta `@e2e` set is selected by
`./run-tests.sh --e2e` (`@e2e and not @givn.removed`).

### 2. Empty or trivial step bodies

0 empty, whitespace-only, `todo!()`, or `unimplemented!()` step bodies found
across the 11 step files that carry this change's new bindings:
`enable_optional_addons_ise_steps.rs`,
`enable_optional_addons_ise_e2e_steps.rs`,
`specification_declaration_steps.rs`,
`specification_declaration_e2e_steps.rs`,
`specification_health_steps.rs`, `serve_extended_rules_steps.rs`,
`instruction_integrity_steps.rs`, `change_strategy_steps.rs`,
`change_strategy_e2e_steps.rs`, `spec_gap_steps.rs`, and
`spec_gap_e2e_steps.rs`. The five extended files
(`initialize_a_project_e2e_steps.rs`, `run_upgrade_steps.rs`,
`default_all_addons_steps.rs`, `template_conditional_rendering_steps.rs`, and
`tests/common/mod.rs`) were read; their new bodies assert exit codes, file
bytes, config lines, served text, or revision contents. The only `todo!()`
string under `tests/` is inside a Rust source fixture in
`tests/steps/outline_report_support.rs`, not a step body.

### 3. Checked tasks and commits

All 207 checked boxes name a `COMMIT:` revision; shared revisions name
`Batch: <name>`. Spot checks of `b5fe6114`, `013aaa4b`, `ca2c4b51`,
`c883c8ee`, `715deaf9`, `7efd0b2b`, `a389115c`, `b5b46d9b`, `8584dc3a`,
`a9fe8e07`, `384fb668`, `a89ea27f`, and `c31c3ffa` confirm the commits carry
the batch's production code, step bindings, feature change, or transcript.
One evidence note: `384fb668` is the `@wip` removal plus the transcript for
`A consumer disables a required addon and the refusal names ise`; the
behavior and bindings landed in `013aaa4b` and the batch-1 step file, and this
commit supplies the e2e evidence. It is not a production/test diff, but the
tasks record says so explicitly and the behavior is committed.

### 4. Promised components and contract use

| Design promise | Exists | Consumers |
|---|---|---|
| `config::addons` / `AddonSpec` / `declared_default` / `requires` | `src/config/addons.rs` | `cli::addons`, `cli::init`, `config::lock`, `skills::render` |
| `config::declaration` / `SpecificationDeclaration` / `Role` / `DeclarationError` / `load` / `is_effective` | `src/config/declaration.rs` | `cli::lint`, `cli::addons` (path and scaffold), `cli::migration` |
| `SpecificationHealth` / `specification_findings` / `walk_project` / `Glob` | `src/cli/lint/specification.rs` | `cli::lint` default scope |
| Cargo manifest reader (`CargoPackage`, `dependency_edges`, `nearest_manifest`) | `src/cli/lint/manifest.rs` | `cli::lint` `e2e` boundary check |
| Addon-bound serving (`ADDON_BOUND_INSTRUCTION_IDS`, `POLICY_INSTRUCTION_IDS`) | `src/cli/instructions.rs` | `cli::instructions` |
| `assets/specification.yaml.tmpl`, `assets/instructions/ise.md`, four guidance blocks | present | `scaffold_enabled`, `prepare_managed_upgrade`, instruction cascade |

`config` imports no `cli` type; `cli::addons` imports the registry and
re-exports nothing; `config::declaration` imports no `cli` type. One boundary
finding remains: the e2e step file imports the registry implementation
(`givn::config::addons::KNOWN_ADDONS`) contrary to the reviewed batch
constraint that step files never import `config::addons` (F3).

### 5. Strict-mode proof

`tests/features_runner.rs` calls `.fail_on_skipped()` at both registration
points. The reviewer ran one temporary probe feature against the built binary:

```text
Feature: probe
  Scenario: An undefined step fails the run
   ✘  Given a deliberately unbound phrase for the strict mode probe
      Step doesn't match any function
[Summary]
1 feature
1 scenario (1 failed)
1 step (1 failed)
process exited with status 1
```

Strict mode is in effect. The tasks.md Setup proof is missing and its box is
unchecked (F6); this review's probe is independent evidence, not a substitute
for the recorded setup task.

### 6. E2E primary assertions

| `@e2e` scenario | Driver | Primary assertion |
|---|---|---|
| `Consumer selects agent targets during interactive init` | real `givn init` in a `script` PTY | labeled prompt, selected targets, exit 0, config/skill/absence state, transcript |
| `A consumer lists addons and each declared default is reported` | real `givn init` then `givn addons list` | `[disabled]  ise`, `[enabled ]  outline`, the declared-default tip, transcript |
| `A consumer disables a required addon and the refusal names ise` | real `givn addons disable` | exact refusal, exit 1, unchanged flags, transcript |
| `The declared roles are read by the project-health pass` | real `givn lint` | exit 0, no `DECL ` line, transcript; the transcript also records two `[SPEC ]` advisories (F4 notes the assertion is weaker than the title) |
| `The design guidance requires one recorded change strategy` | real `givn instructions design --change` | served change-strategy block, transcript |
| `The review guidance requires one spec gap outcome before any fix` | real `givn instructions review --change` | served spec-gap block, transcript |

Every step drives `common::run_givn` (which spawns `env!("CARGO_BIN_EXE_givn")`)
or the `script` PTY helper. No E2E step sends HTTP, calls a repository API
instead of observing output, or substitutes an in-process call for the binary.
The product is a CLI; the opposite classification (a browser UI) does not hold:
no HTML, CSS, or JavaScript page is rendered, so a browser driver would test an
interface the product does not have.

### 7. `verify.e2e_command` target and parallel implementations

`givn/commands.yaml` declares `e2e_command: "./run-tests.sh --e2e"`. The script
runs `tests/features_runner.rs` with the tag expression
`@e2e and not @givn.removed`. The delta's `@e2e` bindings live only in the six
files named above; a tree search (including untracked files) found no second
implementation of the same `ise` actions. The only untracked paths are the two
`verification/*.json` receipts.

### 8. E2E scope selection and normalized action scope

`run-tests.sh` derives `result_scope=e2e` for `--e2e`, `regular` otherwise, and
exports `GIVN_RESULT_SCOPE` under the result-file contract; the tasks receipt
records `scope: change`, 52/52. The delta has six `@e2e` scenarios for six
distinct consumer actions; refusal, flag, and data variants are regular
scenarios. The full scope count in tasks.md is 6 `@e2e` of 51 delta scenarios.

### 9. Implementation against `design.md`

The registry, declaration loader, health pass, instruction serving, scaffold,
upgrade, guidance blocks, and vocabulary land where the design names them,
except for the findings below: the virtual-workspace dependency path (F1), the
inner-parent matcher (F2), the step-file implementation import (F3), and the
stale design paths/symbols (F8, F9).

### 10. Interaction coverage

The owning use-case inventories contain 66 entries (initialize-project 6,
upgrade-a-project 4, declare-the-specification 1, verify-change 13,
obtain-effective-guidance 1, create-change 9, support fragment 32). The design
matrix has 66 rows; every inventory scenario title has an exact matrix row, and
every matrix title resolves to a scenario in the permanent tree or this
change's delta. No unmapped inventory entry and no extra matrix row exists.
The full cross-reference:

| Owning document | Capability | Consumer action | `@e2e` scenario | Driving mechanism |
|---|---|---|---|---|
| create-change | prose-consolidation | request canonical E2E guidance | Canonical E2E policy is emitted by specs instructions | Real `givn instructions specs --change <id>` subprocess; assert the canonical policy section |
| create-change | prose-consolidation | synchronize and inspect generated E2E guidance | Generated guidance points to the canonical E2E policy | Real `givn skills sync`; read the generated `SKILL.md` |
| create-change | visual-design-contract | request the design guidance for a visual interface | Author requests the visual design contract guidance | Real `givn instructions design --change <id>` subprocess; assert the contract section |
| create-change | visual-design-contract | read the synchronized design and step guidance | Synchronized guidance defines the contract approval and capture mechanics | Real `givn skills sync`; read the generated design and steps skills |
| create-change | change-release-note | create a change | A new change's proposal carries the release-note field | Real `givn new <id>` subprocess; read the generated `proposal.md` |
| create-change | change-release-note | read the proposal guidance | The proposal guidance requires a release note | Real `givn instructions proposal --change <id>` subprocess; assert the served text |
| create-change | change-release-note | read the generated implementation guidance | The generated implementation guidance shares the release note | Real `givn skills sync`; read the generated implement skill |
| create-change | change-strategy | read the design guidance for a rebuilding change (design-review guidance and the three strategy values are variants) | The design guidance requires one recorded change strategy | Real `givn instructions design --change <id>` subprocess; assert served text and transcript |
| create-change | spec-gap | read the review guidance after an Ise Test (the proposal guidance and the four outcomes are variants) | The review guidance requires one spec gap outcome before any fix | Real `givn instructions review --change <id>` subprocess; assert served text and transcript |
| declare-the-specification | specification-declaration | declare the five roles and the replaceable source roots (an unknown key, an unknown or repeated role, an omitted role, an absent source list, and a path outside the project are refusal variants) | The declared roles are read by the project-health pass | Real `givn lint` in an ise-enabled temp project with a written declaration; assert exit 0, no schema error, and the captured transcript |
| initialize-project | initialize-a-project | select agent targets in the interactive terminal (the fresh default addon posture is asserted here) | Consumer selects agent targets during interactive init | Real `givn init` subprocess driven through a PTY; read the selection prompt, choose `opencode, claude`, assert the written settings, absence of the declaration, and the captured transcript |
| initialize-project | initialize-a-project | initialize with an explicit noninteractive target selection (SCM backend, no-SCM, and `--no-addons` are precondition variants) | Consumer runs init non-interactively and gets opencode skills | Real `givn init --no-addons --agents opencode` subprocess; assert stdout and generated skills |
| initialize-project | initialize-a-project | resynchronize selected targets | Consumer re-syncs skills after init and the selection is honoured | Real `givn skills sync` subprocess; assert stdout and target contents |
| initialize-project | enable-optional-addons | enable an optional addon (the `ise` prerequisite check and the declaration scaffold are variants) | Consumer enables the outline addon and it is listed as enabled | Real `givn addons enable outline` then `givn addons list`; assert stdout |
| initialize-project | enable-optional-addons | disable an optional addon (the reverse prerequisite refusal is a variant) | A consumer disables a required addon and the refusal names ise | Real `givn addons disable architecture_principles` on an ise-enabled temp project; assert exit code, stderr, the unchanged config, and the captured transcript |
| initialize-project | enable-optional-addons | list the registered addons | A consumer lists addons and each declared default is reported | Real `givn init` then `givn addons list` subprocess; assert `[disabled]  ise`, `[enabled ]  outline`, the tip line, and the captured transcript |
| obtain-effective-guidance | serve-extended-rules | request effective instructions by id (artifact, policy, step, addon-bound, challenge subject, JSON, and change context are variants) | An agent receives the extended rules through the Givn CLI | Real `givn instructions` subprocess; assert served text |
| upgrade-a-project | apply-a-managed-upgrade | run a managed upgrade (an absent, an edited, and a `ise`-disabled declaration are precondition variants) | A clean project without migration requirements receives one managed upgrade commit | Real `givn upgrade` in a committed Git temp project; assert revision and report |
| upgrade-a-project | apply-a-managed-upgrade | invoke a retired engine command | A consumer invoking a retired engine command is told it does not exist | Real binary invocation of the retired command; assert exit code and stderr |
| upgrade-a-project | apply-a-managed-upgrade | invoke the retired history rewrite command | A consumer invoking the retired history rewrite command is told it does not exist | Real binary invocation of the retired command; assert exit code and stderr |
| upgrade-a-project | apply-a-managed-upgrade | archive a maintenance migration | A completed maintenance migration can be archived without a product specification | Real `givn archive --change <id>` subprocess; assert the archived candidate and receipt |
| verify-change | validate-artifact-ids | check an unknown artifact | Consumer invokes givn check with an unknown artifact and gets a helpful error | Real `givn check unknown-artifact` subprocess; assert nonzero and the known-artifact list |
| verify-change | visual-review-gate | run the review check on a visual change | A visual change with complete evidence passes review | Real `givn check review --change <id>` on a project with complete visual evidence; assert the verdict |
| verify-change | visual-review-gate | read the visual review guidance | Review guidance requires the visual verdict and persona lenses | Real `givn instructions review --change <id>` subprocess; assert the served text |
| verify-change | spec-overlap-gates | detect a duplicate title | A duplicate title across two permanent spec files is a hard error | Real `givn lint` on a fixture with two permanent files sharing a title; assert exit 3 and the duplicate message |
| verify-change | spec-overlap-gates | inspect duplicate JSON | The json output reports duplicate-title findings machine-readably | Real `givn lint --json` subprocess; parse the finding kind |
| verify-change | resolve-change-directories | run integrity in a change | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected | Real `givn check review --change <id>`; assert the integrity phase runs against `givn/changes/<id>/specs` |
| verify-change | resolve-change-directories | reject placeholder commit evidence | Integrity check detects a placeholder COMMIT hash in tasks.md and fails | Real `givn check review --change <id>` with a placeholder hash; assert the refusal |
| verify-change | givn-workflow-enforcement | run migration coverage check | Consumer runs migration coverage check and sees passed evidence | Real `givn check migration --change <id>` subprocess; assert the receipt |
| verify-change | check-specification-health | lint the specification corpus (the four declaration findings are data variants of this action) | All clean .feature files report a clean summary on stdout | Real `givn lint` subprocess; assert exit 0 and the clean summary |
| verify-change | givn-workflow-enforcement | run the change-scoped checks | Consumer runs check tasks and receives a change verification receipt | Real `givn check tasks --change <id>`; assert the receipt path and counts |
| verify-change | givn-workflow-enforcement | request bare-check guidance without execution | Consumer invokes bare check and receives explicit non-executing guidance | Real `givn check` subprocess; assert the guidance and that no runner executed |
| verify-change | givn-workflow-enforcement | write a runner result | The result helper writes four counts without a scope | Real `givn test-result --total 3 --passed 3 --failed 0 --skipped 0`; read `GIVN_RESULT_FILE` |
| verify-change | givn-workflow-enforcement | run the review check on a change with use-case schema errors | Consumer runs review and receives every use-case schema problem in one pass | Real `givn check review --change <id>` on a fixture with several schema errors; assert every problem in one output |
| fragments | instruction-integrity | resolve an instruction and expose the machine contract | An agent requests machine-readable artifact guidance | Real `givn instructions design --change <id> --json` subprocess; parse `instruction` and compare with the text path |
| fragments | instruction-integrity | request normal session-start instructions | Author starts a session with the universal dialog obligation | Real `givn instructions` subprocess; assert the dialog pointer |
| fragments | refresh-generated-guidance | synchronize selected skills | Synchronized guidance explains how to resolve the deterministic gates | Real `givn skills sync`; read a generated `SKILL.md` |
| fragments | adr-qualification-gate | request architecture-decision guidance (design-review selection is a regular variant) | An author requests the strict ADR guidance through the Givn CLI | Real `givn instructions arc42-docs --change <id>` subprocess; assert the qualification section |
| fragments | review-architecture-documentation | request architecture-aware review guidance | Arc42-enabled review instructions require conformance evidence | Real `givn instructions review --change <id>` subprocess; assert the Arc42 block |
| fragments | review-architecture-documentation | eject the review template | The review template includes the Arc42 conformance block | Real `givn eject review:template`; read the ejected project file |
| fragments | review-architecture-documentation | synchronize the generated review skill | Generated review guidance stays aligned after skill sync | Real `givn skills sync`; compare the generated skill with the served instruction |
| fragments | attribute-commits | initialize shared attribution plumbing | Consumer initializes a project and receives shared attribution plumbing | Real `givn init` subprocess; assert `.githooks/commit-msg` and the installation report |
| fragments | attribute-commits | commit explicit selected paths with attribution | An attributed commit exposes the current LLM and harness | Real `givn commit <paths> -m ...`; read the revision trailer |
| fragments | attribute-commits | prepare attribution with OpenCode's selected model | OpenCode prepares attribution using its selected model | Real OpenCode plugin invocation with a recorded payload; read the prepared message |
| fragments | attribute-commits | attribute a commit with Claude's current model | Claude attributes a commit using its current model | Real Claude hook invocation with a recorded payload; read the trailer |
| fragments | attribute-commits | attribute a commit with omp's current model | omp attributes a commit using its current model | Real omp hook invocation with a recorded payload; read the trailer |
| fragments | attribute-commits | attribute a commit with Maki's current model | Maki attributes a commit using its current model | Real Maki Lua hook invocation with a recorded payload; read the trailer |
| fragments | enable-commit-attribution | inspect attribution installation | Consumer sees the commit attribution installation report | Real `givn init` subprocess; assert the report line |
| fragments | enable-commit-attribution | upgrade missing attribution plumbing | Consumer upgrades an existing project with attribution plumbing missing | Real `givn upgrade` subprocess; assert the refreshed hook |
| fragments | enable-commit-attribution | initialize Fossil attribution | Consumer initializes a Fossil project and receives the givn commit path without a hook | Real `givn init` in a Fossil checkout; assert no hook and the reported commit path |
| fragments | measure-coverage-per-project | inspect initialized test and coverage scripts | An initialized project documents reusable test and coverage scripts | Real `givn init`; read `run-tests.sh` and `measure-coverage.sh` |
| fragments | measure-coverage-per-project | publish line and branch badges when archiving | Archiving publishes line and branch badges from the merged report | Real `givn archive --change <id>`; read the managed README block |
| fragments | coverage-readme-example | receive a complete README coverage example | Coverage-enabled project receives a complete README coverage example | Real `givn init`; read `README.md` |
| fragments | detect-the-project-repository | detect the repository from a Git worktree at the project root | A Git worktree at the project root selects the Git backend | Real `givn init` in a temp Git worktree; assert the Git backend line |
| fragments | find-existing-scenarios | find a permanent scenario by a step text token | Lexical search locates a permanent scenario by a step text token | Real `givn spec find <token>` subprocess; assert the scenario identity |
| fragments | find-existing-scenarios | inspect per-use-case counts in the spec tree | The spec tree reports per-use-case counts and the use-case narrative | Real `givn spec tree` subprocess; assert the counts and narrative |
| fragments | guide-domain-modeling | receive domain guidance for the next step | An agent requests next-step guidance and is routed to the domain skill | Real `givn instructions` subprocess; assert the domain-skill route |
| fragments | guide-domain-modeling | receive domain guidance for design | An agent requests the design guidance and is routed to the domain skill | Real `givn instructions design --change <id>`; assert the domain-skill pointer |
| fragments | guide-domain-modeling | receive domain guidance for specs | An agent requests the specs guidance and is routed to the domain skill | Real `givn instructions specs --change <id>`; assert the domain-skill pointer |
| fragments | guide-domain-modeling | receive the domain skill after synchronizing skills | An agent synchronizes skills and the domain skill is routed | Real `givn skills sync`; read the generated skills |
| fragments | match-guidance-to-enabled-addons | enable the addons a generated section needs | A consumer enables the addons a generated section needs and the guidance gains it | Real `givn addons enable <addon>` then `givn instructions <id>`; assert the gained section |
| fragments | match-guidance-to-enabled-addons | synchronize skills with disabled-addon sections omitted | A consumer synchronizes skills and the generated guidance omits disabled-addon sections | Real `givn skills sync` with an addon off; read the generated skill |
| fragments | route-a-change | route a proposal and inspect ranked candidates | Routing a proposal prints ranked candidates with evidence | Real `givn spec route --change <id>`; assert the ranked output |
| fragments | show-coverage-in-the-readme | place the coverage badge after the README title | Coverage-enabled project is guided to place the coverage badge directly after the README title | Real `givn instructions readme --change <id>` subprocess; assert the placement guidance |
| fragments | verify-repository-ownership | archive and publish consolidated permanent specifications | Archive publishes consolidated permanent specifications | Real `givn archive --change <id>`; read the merged permanent spec and revision |
| fragments | verify-repository-ownership | accept explicit overlap dispositions in repository-wide review | Repository-wide review accepts explicit overlap dispositions | Real `givn check review --change <id>` with a disposition line; assert the accepted disposition |

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Add-on posture and the one opt-in addon | `docs/arc42/01-introduction-and-goals.md:24` | Yes | Overview, Decisions | Batch 1, 8 | `AddonSpec.default_enabled`, `Addons.ise` serde default | Yes |
| `TC-11` lint path | `docs/arc42/02-architecture-constraints.md:21` | Yes | `src/cli/lint.rs` (stale) | Batch 6 and refactor | `src/cli/lint/{mod,specification,manifest,hygiene}.rs` | Chapter yes; design stale (F9) |
| `TC-14` per-addon declared default | `docs/arc42/02-architecture-constraints.md:24,81` | Yes | Registry and manifest | Batch 1 | `AddonSpec`, `Addons`, `config::lock` | Yes; evidence column omits `src/config/addons.rs` |
| Declaration input at the technical boundary | `docs/arc42/03-context-and-scope.md:49` | Yes | Data model | Batch 3 | `config::declaration` | Yes |
| Addons strategy row | `docs/arc42/04-solution-strategy.md:30` | Yes | Overview | Batch 1 | Registry and prerequisite refusals | Yes |
| Building blocks `config::addons` and `config::declaration` | `docs/arc42/05-building-block-view.md:76-78` | Yes | Architecture Impact | Batches 1, 3 | Both modules and their consumers | Yes |
| Runtime flows 6.4, 6.12, 6.13 | `docs/arc42/06-runtime-view.md:171,537,576` | Yes | Health, enable, upgrade | Batches 4-6 | `specification_health`, `run_enable`/`run_disable`, `prepare_managed_upgrade` | Yes |
| Deployment view | `docs/arc42/07-deployment-view.md` | No | No packaging change | n/a | Single binary, no topology change | Yes (unaffected) |
| Third configuration authority and exit codes | `docs/arc42/08-crosscutting-concepts.md:52-59,425-441,576` | Yes | Data model, health table | Batches 3, 6 | `config::declaration`, `severity_for`, `DECL `/`SPEC ` | Yes |
| ADR-010 amendment | `docs/arc42/09-architecture-decisions.md`, `adr/ADR-010-manifest-overlay.md:66-78` | Yes | Decisions | Batch 1, 3 | The declaration authority is documented | Yes |
| Quality requirements | `docs/arc42/10-quality-requirements.md` | No | No quality scene | n/a | Functional Gherkin only | Yes (unaffected) |
| `D-28` Cargo-only edge limit | `docs/arc42/11-risks-and-technical-debt.md:156` | Yes | Decisions D3, risks | Batch 6 | `CargoPackage` reader; no command invocation | Yes (limit documented) |
| Glossary link | `docs/arc42/12-glossary.md` | Yes | Vocabulary | All relevant batches | Chapter 12 is only a link; rows are in `docs/glossary.md` | Yes |

ARC42 CONFORMANCE: CLEAN

The re-audit on 2026-11-14 confirmed the design corrections: `design.md` names
`src/cli/lint/specification.rs` for the health pass, describes the lint split
from `src/cli/lint.rs`, and names `prepare_managed_upgrade`. Every affected
chapter matches the durable docs, design, tasks, and implementation.

## Ubiquitous language conformance

`docs/glossary.md` holds the eight confirmed rows this change relies on:
`Specification`, `Source code`, `End-to-end (E2E) test`, `Constant Ise`,
`Ise Test`, `Spec gap`, `Change strategy`, and `Opt-in addon`. The six
rewrites (`Addon`, `Addon default`, `Feature file`, `Permanent spec`,
`Use Case Elaboration`, `Verification scope`) and the five anti-terms are
present. The served `ise` instruction, the four guidance blocks, the specs, and
the code use the same terms:

| Term | Code symbol or artifact |
|---|---|
| `Addon` / `Addon default` | `AddonSpec.default_enabled`, `Addons` serde defaults, `declared_default` |
| `Opt-in addon` | `ise` with `default_enabled: false`; `requires: ["architecture_principles"]` |
| `Specification` | `SpecificationDeclaration`, `givn/specification.yaml` |
| `Source code` | `source_roots()`, the `source:` list, `specification_e2e_boundary` |
| `End-to-end (E2E) test` | `e2e` role, `specification_e2e_boundary`, package resolution |
| `Change strategy` | design guidance `## Decisions` block, `change-strategy` scenarios |
| `Ise Test` | `Dry-Run Ise` / `Full replacement` in the design guidance and `ise` instruction |
| `Spec gap` | review/proposal guidance, `spec-gap` scenarios |

`docs/domain-model.md` states that the permanent specification corpus is the
behavior aggregate of the Specification and excludes contracts, limits, and the
`e2e` package; the declaration is a project input, not a new aggregate. No
second unqualified definition was found. The stale `opt-in default` phrase
appears only in the glossary anti-term row and archived historical records.

## Coverage

No measured coverage evidence exists for this change yet: `coverage/` is
absent and the tasks receipt covers the delta scenario counts, not branch
instrumentation. This review classifies from inspection; the archive receipt
will carry the measured result.

| Bucket | Finding | Disposition |
|---|---|---|
| Dead code | None. `AddonSpec::declared_default`, `Role::ALL`, `roles()`, `source_roots()`, `is_effective`, `detail`, `Glob`, `walk_project`, `CargoPackage`, and `dependency_edges` are all reachable from the production paths or the tests. | None required. |
| Missing test coverage | `Glob` `*`, `?`, and multi-byte wildcard branches have no committed test; the reviewer probe confirms they work. | Add a `#[cfg(test)]` module for the matcher (F5). |
| Missing test coverage | The `e2e` package physically under a `source:` root branch has no committed scenario; the reviewer probe confirms it reports `e2e package 'e2e' sits under a source root`. | Add one scenario or unit test (F5). |
| Missing test coverage | The workspace-inherited dependency path has no test and is broken for a virtual workspace root (F1). | Fix the reader and add a virtual-root scenario (F1). |
| Legitimately hard to test | None. | None. |

Redundant unit tests were not introduced: the new registry and loader unit
tests protect distinct schema and default invariants, not the same path as a
feature scenario.

## E2E coverage

The capability's real interface is the CLI. The six delta `@e2e` scenarios
drive the built `givn` binary, and every one has an observable primary
assertion (exit code plus named stdout/stderr content or file bytes). No
scenario asserts only a repository state. The interaction matrix and the full
cross-reference above show one `@e2e` scenario per normalized action and
regular variants for refusals and data changes. The health E2E assertion is
secondary for its title's "read" claim (F4), but the read itself is covered by
the regular `A declared path missing from the tree is reported as advisory`
scenario and by the committed transcript's `[SPEC ]` advisories.

## Visual review

- Interface: `terminal` (no viewports, no screenshots, no browser).
- Evidence: six committed transcripts under
  `givn/changes/add-ise-addon-and-specification/evidence/visual/<slug>/transcript.txt`:

| Scenario | Evidence file | State shown |
|---|---|---|
| A consumer lists addons and each declared default is reported | `a-consumer-lists-addons-and-each-declared-default-is-reported/transcript.txt` | exit 0; `[disabled]  ise`; `[enabled ]  outline`; the declared-default tip |
| A consumer disables a required addon and the refusal names ise | `a-consumer-disables-a-required-addon-and-the-refusal-names-ise/transcript.txt` | exit 1; the exact prerequisite refusal |
| Consumer selects agent targets during interactive init | `consumer-selects-agent-targets-during-interactive-init/transcript.txt` | real `script` PTY prompt, `opencode, claude`, exit 0 |
| The declared roles are read by the project-health pass | `the-declared-roles-are-read-by-the-project-health-pass/transcript.txt` | exit 0; two `[SPEC ]` advisories; no `DECL ` |
| The design guidance requires one recorded change strategy | `the-design-guidance-requires-one-recorded-change-strategy/transcript.txt` | the served change-strategy block |
| The review guidance requires one spec gap outcome before any fix | `the-review-guidance-requires-one-spec-gap-outcome-before-any-fix/transcript.txt` | the served spec-gap block |

- Realness: the interactive transcript is a real `script` PTY session and shows
  the prompt, the toggled selection, the banner, the addon summary, and
  `COMMAND_EXIT_CODE="0"`. The other five are captured stdout/stderr of the
  built binary through `common::run_givn`; each carries its scenario title,
  argv, exit code, and output. None is hand-written.
- Persona dispositions: `developer--primary-user` sees `ise` disabled by
  default, the exact prerequisite refusal, the scaffold path, and the advisory
  findings without a blocked exit; `agent--technical-role` receives the five
  obligations through `givn instructions ise` and the design/review guidance,
  and the refusal names the addon and the enable command;
  `givn-maintainer--owner` sees the upgrade write and stage the scaffold once
  and report an existing declaration as project-owned. No Persona goal fails.
- Baseline: messages name the addon, role, path, or flag; refusals name the
  condition and the next action; no internal Rust identifier appears; the
  documented exit codes hold; text and JSON carry the same instruction content.
  No finding against the baseline.
- Opposite classification: a browser-rendered page would be a Web UI and
  require a browser driver. This change renders no page; a browser driver would
  not exercise the CLI contract.

VISUAL-REVIEW: PASS

## Overlap dispositions

| Scenario A | Scenario B | Decision | Reason |
|---|---|---|---|
| A declaration that is not valid YAML is refused by name | A role that is not a list is refused by name | variant | Different malformed input and refusal class; merging would lose which malformation is named. |
| A declaration that is not valid YAML is refused by name | An unreadable declaration is refused by name | boundary | Parse failure versus an unreadable path (a directory); different loader branch. |
| A role that is not a list is refused by name | An unreadable declaration is refused by name | boundary | Schema type failure versus file-read failure. |
| A project with ise disabled reads no declaration | Change-scoped lint does not read the declaration | boundary | Different gate scopes and addon states; both are required "not read" boundaries. |
| A project with ise disabled reads no declaration | An empty role list retains nothing and produces no finding | boundary | Addon-off precondition versus addon-on empty declaration. |
| A declared path missing from the tree is reported as advisory | A .feature file outside the behavior globs is reported as advisory | variant | Two distinct advisory kinds; both must stay separately attributable. |
| A declared e2e package with a dependency edge into source is reported as advisory | A declared e2e package with a transitive dependency edge into source is reported as advisory | variant | Direct and transitive reachability are distinct graph depths. |
| Change-scoped lint does not read the declaration | An empty role list retains nothing and produces no finding | boundary | Change scope versus default scope with an empty role. |
| Architecture principles guidance follows the opt-in addon | Architecture principles guidance follows an enabled addon | variant | The new title supersedes the removed title under the corrected `Opt-in addon` vocabulary; the delta removes the old title. |
| Outline guidance follows the opt-in addon | Outline guidance follows an enabled addon | variant | The new title supersedes the removed title under the corrected vocabulary; the scenario body is unchanged. |

The 1,254 `[SUBST]` advisories are cross-scenario assertion-subset notes
between the delta and permanent scenarios. They are advisory evidence; no pair
is a duplicate of a required behavior, and no disposition row is claimed for
them beyond this boundary/variant classification.

## Split-or-keep

No delta scenario exceeds the 14-step threshold; no split-or-keep decision is
required.

## Deferrals

None. The Cargo-only `e2e` edge limit is a documented product limit (D3,
`D-28`, `docs/reference.md`), not a deferred mandatory check. The missing
treatments above are findings, not deferrals.

## Findings

| ID | Finding | Evidence | Classification | Required fix |
|---|---|---|---|---|
| F1 | A virtual Cargo workspace root (`[workspace]` without `[package]`) is skipped by `CargoPackage::read`, so a member's `workspace = true` dependency on a source package produces no `specification_e2e_boundary` finding. | `src/cli/lint/manifest.rs:48` requires `[package].name`; `workspace_root` (`:138-156`) searches only parsed packages. Probe: virtual root plus `app = { workspace = true }` printed no finding; the explicit `path` control printed the edge. Design `:357,519` and tasks `:958,1098` promise workspace-inherited deps. | Production defect; missing test (bucket 2). | Parse the virtual workspace root's `[workspace.dependencies]`, resolve `workspace = true` through it, and add a scenario with a virtual root. |
| F2 | A declared path with an inner `..` (accepted by `is_project_relative` and its unit test) never matches in `Glob::compile`, which drops `ParentDir`; an existing path is reported as missing. | `src/config/declaration.rs:336` accepts `src/../src/lib.rs`; `src/cli/lint/specification.rs:375-386` filters components to `Normal` only. Probe: `structure: ["docs/../docs/glossary.md"]` with the file present produced `matches no path`. | Production defect (false advisory); missing test. | Normalize the glob pattern the same way validation does (or refuse inner `..` in validation), and add a test. |
| F3 | The e2e step file imports the registry implementation, contrary to the reviewed batch constraint. | `tests/steps/enable_optional_addons_ise_e2e_steps.rs:28` imports `givn::config::addons::KNOWN_ADDONS` and `:48` uses `Addons::NAMES`; `design.md` batch rule says step files never import `config::addons` or `config::lock`. | Architecture boundary finding. | Remove the import and compare the scenario's literal set; keep the registry check in the module unit test. |
| F4 | The health E2E assertion is weaker than its title: exit 0 plus no `DECL ` would also pass if the loader read were removed. | `tests/steps/specification_declaration_e2e_steps.rs:45-52`; the committed transcript shows two `[SPEC ]` advisories but the step does not assert them. | Evidence weakness (bucket 2); the read is proven by the regular missing-path scenario and the transcript. | Assert one `SPEC ` advisory (or retitle the scenario to the acceptance it proves). |
| F5 | Missing permanent tests for the `Glob` `*`/`?` branches and the "e2e package sits under a source root" branch. | `src/cli/lint/specification.rs` has no `#[cfg(test)]`; no delta scenario matches those branches; reviewer probes confirm the behavior. | Missing test coverage (bucket 2). | Add matcher unit tests (`*`, `?`, multi-byte, parent) and one under-source scenario/unit test. |
| F6 | Strict-mode proof is not recorded in `tasks.md`, and 12 Setup/Closeout boxes are unchecked. | `tasks.md:200-217` placeholder `<!-- paste: ... -->`; `givn status` 207/219. Reviewer probe shows strict mode works. | Task-record / mandatory-proof finding. | Paste the probe output, tick Setup, run the Closeout commands, tick Closeout. |
| F7 | The proposal's Capability Routing table omits the delta capability `initialize-a-project`. | `givn check review` error; proposal table has nine rows and no `initialize-a-project`; the delta modifies `initialize-a-project.feature`. | Gate blocker (proposal record). | Add `EXTEND initialize-a-project` with its rationale; the capability already exists in the permanent corpus. |
| F8 | `design.md` names `run_managed_upgrade`, which does not exist. | `design.md:75,307`; `src/cli/migration.rs:403,567,611` has `prepare_managed_upgrade`; `tasks.md:101,790` records the contradiction. | Documentation defect (design stale). | Rename to `prepare_managed_upgrade`; no decision changes. |
| F9 | `design.md` still names `src/cli/lint.rs` after the reviewed module split. | `design.md:14,93`; implementation is `src/cli/lint/{mod,specification,manifest,hygiene}.rs`; `docs/arc42/02` TC-11 was corrected. | Documentation defect (design stale). | Update the two paths; if the split is treated as structural, re-run design-review. |

No finding is excused under a fourth bucket. F1 and F2 are production defects;
F3-F5 are evidence/coverage gaps; F6 and F7 block the gate records; F8 and F9
are stale design text that the reviewed design-review accepted.

Re-audit status (2026-11-14): all nine findings are closed. F1 and F5 closed in
`092b30b2`; F2 in `39110af3` and `092b30b2`; F3 and F4 in `761ee474`; F6 in
`8feec643`, `cfc10889`, `e17f6f6b`, `c3e9b7d6`, and `6fbb425c`; F7 in
`3beb4d56`; F8 and F9 in `1ad92134`. The per-finding evidence and the
independent re-audit probes are recorded in `## Finding closure` and
`## Re-audit (2026-11-14)` below.

## Sign-off

- [x] Fabrication audit clean — F3 and F4 closed in `761ee474`; the health
      E2E asserts the exact `[SPEC ]` advisory and the step file no longer
      imports the registry.
- [x] Every checked task has a verified commit touching production code or its
      regression test; the e2e transcript-only batch is disclosed.
- [x] Every promised component exists; the registry and declaration consumers
      use the declared modules.
- [x] Every consumer uses only its declared contract — F3 closed.
- [x] Strict-mode proof present in `tasks.md` — `8feec643` records the probe and
      all six Setup boxes; the final `givn check review` box is the gate this
      re-audit passes, and the author ticks it after the committed verdict.
- [x] Scenario execution evidence is recorded per scope in the change receipt
      (52/52) and the tasks e2e counts (6/6 delta; 106/106 full).
- [x] Coverage gaps classified and resolved — F1, F2, and F5 closed in
      `092b30b2` and `39110af3`.
- [x] Dead code deleted; no `todo!()` in steps.
- [x] No `@wip` tags remain; no implementation-layer detail appears in the
      specs beyond observable file paths, exit codes, and output.
- [x] Canonical E2E policy applied; one `@e2e` scenario per normalized action;
      variants are regular scenarios.
- [x] Every E2E primary assertion is the strongest real-interface assertion —
      F4 closed in `761ee474`.
- [x] The local run command starts the CLI; no service, database, or twin.
- [x] `verify.e2e_command` target identified; no parallel implementation.
- [x] E2E scope is selected explicitly; required executions report zero
      failures and zero skips.
- [x] Implementation matches `design.md` — F1, F8, and F9 closed in
      `092b30b2` and `1ad92134`.
- [x] Interaction coverage verified: 66/66 inventory-to-matrix-to-scenario.
- [x] No finding excused with a classification outside the three buckets.

## Finding closure

Author closure pass, re-audited independently on 2026-11-14. The reviewer
verified every fix and flipped the verdict in `## Re-audit (2026-11-14)`.

| ID | Fix | Fixing commit |
|---|---|---|
| F1 | `CargoPackage::read` now reads a manifest that declares `[workspace]` without `[package]`, so `workspace_root` finds a virtual root and resolves a member's `workspace = true` dependency through its `[workspace.dependencies]`; `label(dir)` names a root that declares no package. Tests: `a_virtual_root_is_read_and_a_workspace_dependency_resolves_through_it`, `a_workspace_inherited_dependency_is_reported_as_an_e2e_edge` (a binary probe on a virtual root prints the `dependency edge into source` advisory; the explicit-path control prints the same line). | `092b30b2` |
| F2 | `SpecificationDeclaration::parse` normalizes every declared value with `normalize_project_path`: `.` and repeated separators disappear, an inner `..` cancels the preceding segment, and an escaping `..` is refused by name (`OutsideProject`). `Glob::compile` applies the same resolution instead of dropping `ParentDir`. Tests: `a_declared_path_is_normalized_lexically`, `the_loader_stores_the_normalized_path`, `a_value_that_escapes_the_project_is_refused_by_name`, `an_inner_parent_segment_compiles_to_the_same_pattern`. A binary probe with `docs/../docs/glossary.md` present reports no advisory; the missing-path control still reports one. | `39110af3`, `092b30b2` |
| F3 | `tests/steps/enable_optional_addons_ise_e2e_steps.rs` no longer references `givn::config::addons` (or `Addons::NAMES`). It compares the `true` entries of the written `givn/config.yaml` addons block with the scenario's literal set and checks each named addon in the effective manifest. Re-ran the scenario: 12 steps passed. | `761ee474` |
| F4 | The health `@e2e` scenario asserts `stderr contains the declared-path advisory for the contracts role`, i.e. `the 'contracts' role declares 'docs/reference.md', which matches no path in the project`, which cannot appear when the declaration read is removed. Re-ran the scenario: 6 steps passed. | `761ee474` |
| F5 | `src/cli/lint/specification.rs` gains a test module covering `*` within one segment, `?` matching one character, multi-byte characters counted once, an inner-parent pattern, the `e2e package sits under a source root` branch, and the workspace-inherited edge. | `092b30b2` |
| F6 | The strict-mode probe command and its exit-101 output, the `cargo check --locked --tests` and step-file listing, and the `./run-tests.sh --non-e2e` baseline summary are recorded in `## Setup`; all six Setup boxes are checked. `## Closeout` records the binding, stub, `@wip`, whole-change verification, and `givn check tasks` evidence. The final `givn check review` box is the gate this re-audit passes; the author ticks it after the committed verdict. Receipt: 52/52, 0 failed, 0 skipped. | `8feec643`, `cfc10889` |
| F7 | The proposal's Capability Routing table gains `EXTEND initialize-a-project` with route's 27.00 recommendation and its rationale; `givn spec route` still ranks it at 27.00. The review gate no longer reports the undeclared capability. | `3beb4d56` |
| F8 | `design.md` names `prepare_managed_upgrade` in the upgrade rule row and in `## Upgrade behavior`; no decision changed. | `1ad92134` |
| F9 | `design.md` names `src/cli/lint/specification.rs` for the health pass and lists the split files `src/cli/lint/{mod,specification,manifest,hygiene}.rs`. | `1ad92134` |

Verification after the fixes: `cargo fmt --all -- --check` exit 0; `cargo clippy --locked --workspace --all-targets -- -D warnings` exit 0; `cargo nextest run --locked` 1553/1553; `./run-tests.sh --non-e2e` 923 passed (106 e2e skipped); `./run-tests.sh --e2e` 106 passed (923 skipped); delta non-e2e 45 passed (6 skipped); delta e2e 6 passed (45 skipped); `givn lint --change` clean; `givn check tasks` exit 0.

## Re-audit (2026-11-14)

Independent re-audit of every reported closure against
`6fbb425cb1d92d680ffce06c70b60c044e9932b9`. Nothing was accepted from the
author's report; each behavioral closure was re-probed against the built
binary or the named test.

| ID | Result | Independent evidence |
|---|---|---|
| F1 | Closed | `CargoPackage::read` no longer requires `[package].name`; a `[workspace]`-only manifest is read, `label(dir)` names it, and `dependency_edges` resolves `workspace = true` through `[workspace.dependencies]`. `manifest::tests::a_virtual_root_is_read_and_a_workspace_dependency_resolves_through_it` and `specification::tests::a_workspace_inherited_dependency_is_reported_as_an_e2e_edge` pass. Binary probe on a virtual root: `[SPEC ] advisory ... e2e package 'e2e' has a dependency edge into source (root 'crates/app/**') through package 'app'`; the no-dependency control prints nothing. |
| F2 | Closed | `normalize_project_path` resolves `.`, repeated separators, and inner `..` at load; `Glob::compile` applies the same resolution. Five declaration tests pass. Binary probes: `docs/../docs/glossary.md` with the file present reports no advisory; the missing-target control reports `the 'structure' role declares 'docs/absent.md'` (normalized); `../outside/glossary.md` is refused as `[DECL ] blocking ... '../outside/glossary.md' is outside the project` with exit 1. |
| F3 | Closed | `grep 'givn::config::addons\\|Addons::NAMES'` over the step file returns nothing. The step compares the written `addons:` `true` entries with the scenario literal and checks each named addon in the effective manifest. `Consumer selects agent targets during interactive init` re-ran: 12/12 steps passed. |
| F4 | Closed | The delta feature now carries `And stderr contains the declared-path advisory for the contracts role`; the step asserts one line starting `[SPEC ] advisory` and ending with the exact contracts message. The scenario re-ran: 6/6 steps passed. With the declaration read removed, that line cannot exist. |
| F5 | Closed | `specification::tests` passes 6/6: `*` within one segment, `?` one character, multi-byte counted once, inner-parent compile, `e2e` package under a source root, and the workspace-inherited edge. |
| F6 | Closed | `## Setup` records the strict-mode probe (undefined step, exit 101), the skeleton check, and the baseline; all six Setup boxes are checked. `## Closeout` records binding, stub, `@wip`, whole-change verification, and the receipt. `givn status` is 218/219; the only unchecked box is `givn check review`, the gate this re-audit passes, which the author ticks after the committed verdict. |
| F7 | Closed | The proposal records `EXTEND initialize-a-project` at route's 27.00. The gate now prints `capability 'initialize-a-project' already owned by .../givn/specs/initialize-project` and no routing error. |
| F8 | Closed | `grep run_managed_upgrade design.md` returns nothing; the upgrade rule row and `## Upgrade behavior` name `prepare_managed_upgrade`, the real entry point. |
| F9 | Closed | The Overview names `src/cli/lint/specification.rs`; `## Architecture Impact` describes the split from `src/cli/lint.rs` into `mod.rs`, `specification.rs`, `manifest.rs`, and `hygiene.rs`, matching the tree and arc42 TC-11. |

The durable arc42 chapters, `design.md`, and the implementation now agree;
`ARC42 CONFORMANCE: CLEAN` is restored. The closure changes the design only in
stale symbols and paths, so no design decision changed and the reviewed
decision set stands.

Review gate: `givn check review --change add-ise-addon-and-specification`
exits 0 with this verdict; the receipt is reused across the documentation-only
review revision.

REVIEW: PASS
