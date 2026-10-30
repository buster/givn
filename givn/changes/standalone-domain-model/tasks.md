# Tasks: standalone-domain-model

Owning use case: `obtain-effective-guidance`
(`givn/specs/obtain-effective-guidance/usecase.md`). Supporting owner:
`initialize-project` (`givn/specs/initialize-project/usecase.md`) and the
reusable `fragments` document (`givn/specs/fragments/fragment.md`). Personas:
`agent--technical-role`, `developer--primary-user`, `givn-maintainer--owner`.

Release note: Domain modeling now keeps the project's glossary and structural model in standalone documents that work without arc42.

## Setup

- [x] Confirm the existing runner and strict mode; no new runner is installed.
      Evidence: `tests/features_runner.rs` calls
      `GivnWorld::cucumber().fail_on_skipped()` and exits non-zero on failed or
      skipped scenarios; `givn/commands.yaml` sets
      `verify.command = ./run-tests.sh --non-e2e` and
      `verify.e2e_command = ./run-tests.sh --e2e`.
- [x] Confirm the step definition files named by `design.md` exist and remain
      the homes for this change:
      `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`,
      `tests/steps/template_conditional_rendering_steps.rs`,
      `tests/steps/default_all_addons_steps.rs`,
      `tests/steps/arc42_review_alignment_steps.rs`.
- [x] Add the generic fixture bindings used by the scaffold scenarios in
      `tests/steps/default_all_addons_steps.rs`:
      `a project file "<rel>" with content "<text>"`,
      `the file "<rel>" contains "<text>"`, and
      `the file "<rel>" does not contain "<text>"`. Seed `docs/` when absent;
      preserve the exact relative path.
- [x] No new E2E action is added. The modified `@e2e` scenario keeps the
      existing `givn skills sync` + `givn instructions` CLI actions, its real
      subprocess driver, and its terminal transcript evidence.

## Batch A — standalone document scaffolds

Design constraints for every scenario in this batch:

- Add `assets/templates/glossary.md` with a `# Glossary` heading, one sentence
  naming `docs/glossary.md` as the sole vocabulary authority, and a
  `| Term | Definition |` table.
- Add `assets/templates/domain-model.md` with the scope/status section, the
  separate Mermaid legend (`layout: elk`, `flowchart TB`, collapsed aggregate
  and external-system notation, `This is notation only, not part of the domain
  model`), `## Boundaries and responsibilities`, `## Relationships` with a
  governing-contract column, and `## Open modeling questions`.
- `src/cli/addons.rs`:
  - `scaffold_enabled("arc42")` calls `scaffold_arc42()` then
    `scaffold_glossary()`.
  - `scaffold_enabled("domain_modeling")` calls `scaffold_glossary()` then
    `scaffold_domain_model()`.
  - A shared `scaffold_document(rel, asset_key, label)` creates `docs/` and
    writes only when `path.exists()` is false; print `created <rel>` or
    `exists <rel> (content preserved)`.
  - Disabling an addon never deletes either document.
- Replace `assets/addons/arc42-docs/chapters/12-glossary.md` with a heading and
  the relative link `../glossary.md`; keep no term table.
- Preserve consumer content byte-for-byte: existence checks precede every
  write; the embedded templates are the only source.
- Contract: Addon enablement and scaffold — implemented only by
  `src/cli/addons.rs`; consumers use `givn init` and `givn addons enable`.

### Scenario: givn init --no-addons produces no skill or command addon files

- [x] Scenario: givn init --no-addons produces no skill or command addon files
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: givn init enables every addon by default

- [x] Scenario: givn init enables every addon by default
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: The scaffolded glossary is the standalone vocabulary authority

- [x] Scenario: The scaffolded glossary is the standalone vocabulary authority
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: The scaffolded domain model carries the notation legend

- [x] Scenario: The scaffolded domain model carries the notation legend
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: Existing domain documents survive initialization

- [x] Scenario: Existing domain documents survive initialization
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: Enabling arc42 scaffolds only the standalone glossary

- [x] Scenario: Enabling arc42 scaffolds only the standalone glossary
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: Enabling domain modeling scaffolds both durable documents

- [x] Scenario: Enabling domain modeling scaffolds both durable documents
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: Enabling an addon preserves an existing glossary

- [x] Scenario: Enabling an addon preserves an existing glossary
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

### Scenario: Disabling domain modeling leaves the durable documents in place

- [x] Scenario: Disabling domain modeling leaves the durable documents in place
  - [x] COMMIT: e14e7a76 -- Batch: standalone-document-scaffolds

## Batch B — domain skill routing without arc42

Design constraints for every scenario in this batch:

- `src/skills/targets.rs::domain_modeling_present` returns only
  `manifest.addons.domain_modeling`; `reconcile_addon_artifacts` removes
  `givn-domain-modeling` only when that flag is false.
- `src/cli/instructions.rs::domain_skill_to_load` takes only the
  `domain_modeling` flag; every caller passes
  `manifest.addons.domain_modeling`.
- The generated skill body remains thin: it names `givn-domain-modeling` and
  directs the reader to `givn instructions domain-modeling`; it carries no
  copied rules and no arc42 prerequisite.
- Fixtures: `common::make_init_root()` enables every addon;
  `support::set_addon` writes `givn/config.yaml`; `support::sync` runs
  `givn skills sync`; `template_conditional_rendering_steps.rs` already owns
  the skill-exists assertions.
- Contract: Generated guidance — implemented only by `src/skills/targets.rs`.
  Contract: Effective guidance — implemented only by `src/cli/instructions.rs`.

### Scenario: Disabling arc42 keeps the generated domain-modeling skill

- [x] Scenario: Disabling arc42 keeps the generated domain-modeling skill
  - [x] COMMIT: 39a87413 -- Batch: domain-skill-routing

### Scenario: Disabling the domain-modeling addon removes the generated domain-modeling skill

- [x] Scenario: Disabling the domain-modeling addon removes the generated domain-modeling skill
  - [x] COMMIT: 39a87413 -- Batch: domain-skill-routing

### Scenario: Enabling domain modeling without arc42 generates the domain-modeling skill

- [x] Scenario: Enabling domain modeling without arc42 generates the domain-modeling skill
  - [x] COMMIT: 39a87413 -- Batch: domain-skill-routing

### Scenario: A project without arc42 is still pointed at the domain skill

- [x] Scenario: A project without arc42 is still pointed at the domain skill
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

## Batch C — standalone guidance, templates, and migration

Design constraints for every scenario in this batch:

- Rewrite `assets/instructions/domain-modeling.md` around `docs/glossary.md` and
  `docs/domain-model.md`. Preserve these asserted sentences: `Challenge generic
  or overloaded terms`, `Name aggregates with domain nouns`, `the glossary
  inline`, `maintaining the project's glossary`, `the *domain's own*`,
  `trip reset`, the full term-introduction test, and the naming-question
  example (`ShoppingList`, `Candidates:`, `Basket`, `collides`, `should the
  record use, or what is yours`).
- The instruction must contain exactly these ownership statements:
  `Vocabulary belongs in the glossary`, `Structure belongs in the domain model`,
  `Behaviour belongs to use-case contracts and specifications`.
- Add the three checks: ownership, added value, change impact. State that a
  no-op document edit, timestamp, or boilerplate rewrite is `not evidence`.
- Add the document lifecycle: create only when absent, reuse the existing
  model, never a per-change copy, qualify `established structure` versus a
  `proposed model`, `do not invent` structure, and keep `unknown` structure
  explicit.
- Embed the full domain-model template, including the separate legend, the
  boundaries/relationships scaffold, the governing-contract column, and
  `Do not add lifecycle, event, policy, or aggregate-detail sections by
  default`; a focused section is allowed `only when it provides a specific
  benefit`.
- Require a `rendered-diagram review`; state that `syntactically plausible
  Mermaid source is not evidence` and relationships must be
  `individually traceable`. State the ELK host limitation.
- Add the migration section: an older `docs/arc42/12-glossary.md` owner moves
  entries into `docs/glossary.md`, preserves their wording and ordering, and
  leaves chapter 12 as a link. Do not automate this.
- Update `assets/skills/givn-domain-modeling/SKILL.md.tmpl`:
  `maintaining the project's glossary` and the standalone model, no arc42
  prerequisite.
- Update `assets/instructions/ideation.md` so Event Storming reuses aggregates
  from `docs/domain-model.md` and searches `docs/glossary.md`; update
  `assets/instructions/review.md` and `assets/instructions/design-review.md` so
  the ubiquitous-language block is gated on `addons.domain_modeling`, names
  `docs/glossary.md`, `docs/domain-model.md`, and requires a `structural
  impact` assessment.
- Update `assets/addons/arc42-docs/instructions.md`,
  `assets/addons/arc42-docs/chapters/README.md`, `assets/config.yaml.tmpl`,
  `src/cli/addons.rs` AddonSpec descriptions, `docs/INDEX.md`,
  `docs/reference.md`, `docs/workflow.md`, and `README.md` where they state the
  old coupling.
- Contract: Effective guidance — implemented only by
  `src/cli/instructions.rs` plus the embedded assets.

### Scenario: The skill body is an active discipline

- [x] Scenario: The skill body is an active discipline
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The design-review skill checks that the ubiquitous language is followed

- [x] Scenario: The design-review skill checks that the ubiquitous language is followed
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The review skill checks that the ubiquitous language is followed

- [x] Scenario: The review skill checks that the ubiquitous language is followed
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The domain model and glossary have separate owners

- [x] Scenario: The domain model and glossary have separate owners
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Durable documents are reused instead of copied per change

- [x] Scenario: Durable documents are reused instead of copied per change
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Unknown structure stays qualified

- [x] Scenario: Unknown structure stays qualified
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: A rendered diagram is required evidence

- [x] Scenario: A rendered diagram is required evidence
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: An existing arc42 glossary migrates without losing definitions

- [x] Scenario: An existing arc42 glossary migrates without losing definitions
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The structural template keeps its notation legend separate

- [x] Scenario: The structural template keeps its notation legend separate
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The structural template does not require a lifecycle or event catalogue

- [x] Scenario: The structural template does not require a lifecycle or event catalogue
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: The structural template provides the boundaries and relationships scaffold

- [x] Scenario: The structural template provides the boundaries and relationships scaffold
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Relationships link to their governing contract instead of copying rules

- [x] Scenario: Relationships link to their governing contract instead of copying rules
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Review guidance checks vocabulary and structure at their owners

- [x] Scenario: Review guidance checks vocabulary and structure at their owners
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Design-review guidance checks vocabulary and structure at their owners

- [x] Scenario: Design-review guidance checks vocabulary and structure at their owners
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

### Scenario: Review does not accept a no-op model edit as maintenance evidence

- [x] Scenario: Review does not accept a no-op model edit as maintenance evidence
  - [x] COMMIT: 666ce56a -- Batch: standalone-guidance

## Batch D — repository glossary migration and closeout

- [x] Move the current `docs/arc42/12-glossary.md` table to
      `docs/glossary.md`; replace chapter 12 with the heading and link to
      `../glossary.md`; keep every term and definition.
- [x] Create `docs/domain-model.md` for this repository using the shipped
      template; cover the documented givn domain without inventing structure.
- [x] Update `docs/arc42/08-crosscutting-concepts.md` (8.11, 8.27),
      `docs/arc42/05-building-block-view.md`,
      `docs/arc42/06-runtime-view.md`,
      `docs/arc42/11-risks-and-technical-debt.md` (R-127), and the index links
      to the standalone glossary.
- [x] Confirm every delta step phrase has a permanent binding (the step files
      named above), and no unimplemented `todo!()`/`unimplemented!()` remains.
- [x] COMMIT: 40d1b63b -- Batch: standalone-documentation-closeout

## Batch E — @e2e

Design constraints: the modified `@e2e` scenario keeps the existing
`givn skills sync` and `givn instructions` real subprocess actions. Assert the
generated skill name, the every-step trigger, the direction to the instruction,
`maintaining the project's glossary` in the served content, and the absence of
the givn change lifecycle. Capture the terminal transcript after the primary
assertions. Contract: Generated guidance — `src/skills/targets.rs`; Effective
guidance — `src/cli/instructions.rs`.

### Scenario: An agent synchronizes skills and the domain skill is routed

- [x] Scenario: An agent synchronizes skills and the domain skill is routed
  - [x] COMMIT: 39a87413 -- Batch: standalone-domain-e2e
