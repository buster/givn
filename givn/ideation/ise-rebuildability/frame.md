# Frame: ise-rebuildability

Status: confirmed 2026-11-13 (user) for goal, scope boundary, exclusions, and stakeholders.

## Problem

Givn structures feature delivery around executable Gherkin deltas, review gates,
and archive receipts, yet a project that passes every Givn gate can still fail
the Ise Test ("set the old code aside for one module or the whole system, and
build it again from the spec alone; if all tests pass and all requirements are
met, the spec passes the test" — https://iseprinciple.org/).

Four concrete gaps prevent reliable specification-only reconstruction today:

1. **Unprotected verification semantics.** Keeping `.feature` files unchanged
   preserves scenario wording, not what executing those scenarios tests. In the
   Passuff (`gh-notify`) pilot inspection, `scripts/verify.sh --e2e` runs the
   same in-process `features_runner` binary as regular scope
   (`scripts/verify.sh:57`), `crates/passuff-cli/tests/steps/tui.rs` calls
   `passuff::tui::keys::parse_keys`, and
   `crates/passuff-cli/tests/support/world.rs:572` builds the inbox through
   `InboxService` instead of driving `passuff-tui` over a real PTY. Givn's own
   acceptance suite similarly imports `givn::*` across 9 files under
   `tests/steps/`.
2. **Incomplete durable specification knowledge.** Rules for CLI flags, exit
   codes, persisted file/database schemas, cross-version state migrations, and
   terminal/browser viewport budgets frequently remain scattered across `src/`,
   `givn/changes/` history, or prose docs rather than in a declared set of
   retained inputs that outlive `src/`.
3. **All-enabled addon posture and instruction budget risk.** Every registered
   addon in `src/config/manifest.rs` (`Addons::NAMES`, 9 addons) currently
   defaults to `enabled_by_default()`. Meanwhile `assets/instructions/` already
   totals 4,397 lines (`review.md` 587, `ideation.md` 475, `design.md` 361,
   `specs.md` 354). In `givn-harness` (`docs/findings/2026-10-31-givn-0.9.12-to-0.9.16.md`),
   adding defensive guidance in `v0.9.15` kept the Catopia hidden grade at
   `25/25` while collapsing test volume from `130` scenarios (`87.3% / 79.9%`
   coverage) to `76` scenarios (`67.7% / 60.8%`).
4. **No separation between feature co-evolution and frozen reconstruction.**
   Ordinary Givn changes rightly evolve specifications, tests, and code
   together. Rebuilds require the opposite contract: read-only specifications
   and frozen black-box end-to-end verification outside the builder's write
   authority, clean-room removal of exactly the declared replacement-unit paths
   and their listed internal unit tests, preservation of persisted state across
   versions, and an explicit four-way
   `Spec gap` classification of differences (`E2E test defect`, `implementation
   defect`, `specification defect`, or `acceptable undocumented change`).

User direction (2026-11-13, closed in Q1): start `givn-ideation` for `addons.ise`
and instruction slimming first, paired with an experimental `givn-harness`
rebuild/evolution evaluation track on Catopia (`catopia-v2` and
`catopia-spring-v2`), then validate transfer on the full Passuff (`gh-notify`)
pilot and finally Givn self-rebuildability.

## Vision

Givn supports Ise-compliant development through an experimental, opt-in `ise`
addon (`default: false`) built on `architecture_principles`, paired with
on-demand extraction of stack instructions so core step instructions do not
bloat. When `addons.ise` is enabled:

- **Durable Specification:** A project declares and maintains the exact
  authored specifications, glossary, structural domain model, surface/schema
  contracts, and state-migration rules; that Specification is the sole build
  input, so a component or the whole application is rebuilt without reading
  the Source code it replaces.
- **Two-layer test boundary (Ise Pattern 03):** End-to-end (`@e2e`) verification
  runs through real external interfaces with zero imports from production source
  modules and is bound into verification receipts so candidates cannot weaken
  assertions or runners; component `.feature` scenarios stay in the
  Specification set while component step definitions, test setup, and fakes are
  implementation and may be rewritten with the stack.
- **Proportionate change strategy selection (Ise Pattern 01 vs. incremental vs.
  full rebuild):** Changes explicitly choose and justify incremental
  modification, Constant Ise (rebuilding one impacted unit behind its unchanged
  consumer contract), or full-application rebuild, while preserving persisted
  data and state compatibility ("data, state, and contracts outlive the code").
- **Empirical proof before default:** `givn-harness` measures `ise=off` vs.
  `ise=on` on Catopia (`initial` cold rebuild, `followup` evolution vs. unit
  replacement, `followup2` multi-tenant state continuity) under a protected
  checkpoint grader, followed by the Passuff complete-product dry run and Givn
  machinery self-rebuild (retaining Givn's authored instruction and template
  corpus as product content).

## Scope boundary

In scope (confirmed in Q2, 2026-11-13):

- **Givn product capabilities (modeled into change seeds in this topic):**
  1. **Opt-in addon posture (`addons.ise`):** Supporting an explicitly opt-in
     (`default: false`) addon alongside the existing enabled-by-default addons,
     respecting `config-lock.yaml` and `givn addons enable|disable`.
  2. **Instruction slimming / on-demand stack instructions:** Extracting
     per-language/runner implementation mechanics out of core step instructions
     (`specs`, `steps`, `review`, `design`) into on-demand stack instructions
     named by the id convention (`steps` -> `steps-rust`, `dialog` ->
     `dialog-examples`) so enabling `ise` does not trigger context bloat or
     scenario collapse.
  3. **`ise` guidance and lifecycle obligations:** Specification
     completeness, Pattern 03 two-layer test separation, proportionate change
     strategy selection (incremental vs. Constant Ise vs. full rebuild),
     persisted-state continuity rules, and the four-way `Spec gap` classification.
  4. **Mechanical E2E-test-boundary checks, clean-room commands, and receipt
     binding:** A `givn/specification.yaml` declaring the five role globs
     (`behavior`, `structure`, `contracts`, `limits`, `e2e`) and the
     replaceable `source:` roots, with the component's paths passed per trial
     as `givn ise prepare [--at <revision>] [--replace <path>...]`, with the
     revision defaulting to `HEAD`. The declared `e2e`
     role names one package holding the runner and every `@e2e` step
     definition; that package links no production source, and
     `verify.e2e_command` runs it and fails on an unbound or skipped step,
     while feature files stay shared. `givn ise prepare` materializes the
     clean-room directory, removes the paths named with `--replace`, and
     writes the trial record when a receipt path is given; `givn ise verify --trial <receipt> --at <dir>`
     re-checks the two digests and reports drift per path. Both commands are
     bound to `addons.ise` and refuse by name with the addon off. `givn check
     tasks` enforces the blocking `@e2e` boundary, while `givn lint` reports
     the four declaration findings as advisory output; the change verification
     receipt binds the `e2e` package digests, the E2E runner command digest,
     and the running Givn version. `Q3` and `Q5` closed 2026-11-14.
- **Companion validation tracks (external consumers coordinated with this topic,
  not Givn binary features):**
  - `givn-harness` stage-checkpoint freezing, Stage 1 -> 2 -> 3 state-continuity
    grading, target-stage failure semantics, isolated candidate execution, and
    versioned Ise task variants on Catopia (`catopia-v2`, `catopia-spring-v2`).
  - Passuff (`gh-notify`) zero-import PTY (`80x24` / `120x40`) + HTTP mock
    E2E-suite qualification, Constant Ise, and complete-product
    Dry-Run Ise transfer check.
  - Givn self-rebuild dogfood boundary (retaining `assets/` as product content
    while decoupling `tests/steps/` `@e2e` from `use givn::*`).

Out of scope (confirmed in Q2, 2026-11-13):

- Making `addons.ise` enabled by default before empirical Catopia and Passuff
  evidence justifies promotion.
- Mutating existing historical `catopia-v2` / `catopia-spring-v2` task prompts
  in place (any rebuild-permitting follow-up prompt in `givn-harness` must be an
  explicitly versioned variant).
- Requiring multi-agent `Blind Verification` (`Draft 06`) or `Adversarial
  Breaker` (`Draft 07`) in `ise` v0.1 (single-context zero-subagent runs must
  work; WIP drafts remain future extensions, parked for a future topic).
- Adding in-binary LLM calls, network services, or language runtimes to `givn`
  (TC-2, TC-3, single static feature-free Rust binary).
- Replacing Givn's built-in lifecycle graph (`propose` through `archive`) with
  user-defined custom workflows.

## Stakeholders

| Stakeholder type | Interest |
|---|---|
| Consumer-project owner (developer / maintainer) | Wants specifications and black-box `@e2e` suites strong enough that a module or whole product can be rebuilt cleanly without losing persisted user data or undocumented requirements; wants `ise` opt-in so ordinary projects pay zero ceremony until chosen. |
| Authoring / rebuilding agent (technical role / automation caller) | Needs concise, non-bloated step instructions, deterministic `givn check` errors when `@e2e` isolation or Specification contracts are incomplete, and a clear distinction between normal feature co-evolution and frozen clean-room reconstruction. |
| Givn maintainer (owner) | Owns the single-binary Rust architecture, the addon registry (`TC-14`), the permanent spec corpus, and instruction token discipline; requires empirical Catopia + Passuff proof that `ise` improves modularity and rebuildability without `v0.9.15`-style test collapse, and wants Givn's own Rust machinery rebuildable while retaining authored `assets/`. |
| External contributor / independent verifier (open-source consumer) | Clones a published repository (or receives a read-only Specification in a harness sandbox) and builds, runs, and verifies the product through real external interfaces with `givn` absent from PATH and no hidden coupling to deleted internals. |

The external contributor / independent verifier is not a separate persona in
this topic: `givn-maintainer--owner` carries the bare-clone and clean-room
frozen-E2E-test lens, including the requirement that a package builds, runs,
and verifies with `givn` absent from PATH (user decision 2026-11-13, `Q7`).

## Research protocol

### What exists

- **Ise Principle primary source (`https://iseprinciple.org/`, Draft v0.5):**
  Three rules — (1) spec must be enough to build from scratch, (2) anything not
  in the spec is free to change, (3) rebuild by routine. Core patterns:
  `Pattern 01` Constant Ise (delete and rebuild one impacted unit behind
  unchanged neighbor contracts), `Pattern 02` Dry-Run Ise (throwaway clean-room
  build to find spec gaps), `Pattern 03` E2E & Component Tests (untouched
  stack-independent black-box E2E with zero shared code/DTOs vs. in-process
  component tests whose `.feature` scenarios stay fixed and setup/fakes swap),
  `Pattern 04` Strangler Ise. WIP drafts: `Draft 05` Shadow Ise, `Draft 06`
  Blind Verification, `Draft 07` Adversarial Breaker. Explicit consequence:
  "Data, state, and contracts outlive the code."
- **Addon registry and defaults (`src/config/manifest.rs`, `src/cli/addons.rs`,
  TC-14):** Nine compiled-in addons (`arc42`, `communication_policy`, `readme`,
  `coverage`, `domain_modeling`, `dev_principles`, `architecture_principles`,
  `challenge`, `outline`), all defaulting to `true` via `enabled_by_default()`.
  `givn/changes/<id>/config-lock.yaml` locks addon states per active change. No
  default-off addon exists today (`TC-14` states all registered addons are
  enabled by default).
- **Architecture and domain guidance (`assets/instructions/architecture-principles.md`,
  `domain-modeling.md`):** `architecture_principles` (127 lines) already defines
  decision timing, consumer-side narrow contracts, one-responsibility
  components, ports, and adapters. `domain_modeling` (296 lines) owns
  `docs/glossary.md` and `docs/domain-model.md`.
- **Instruction delivery and size (`assets/instructions/*.md`,
  `src/cli/instructions.rs`, `src/config/extensions.rs`):** 21 embedded
  instruction files totaling 4,397 lines (`review.md` 587, `ideation.md` 475,
  `design.md` 361, `specs.md` 354, `tasks.md` 290, `dialog-examples.md` 281,
  `steps.md` 261). Companion on-demand loading already exists for `dialog` ->
  `dialog-examples`. Project-level additive `extensions:` exist in
  `givn/config.yaml`.
- **Verification and receipt machinery (`src/proof.rs`, `src/integrity_hook.rs`,
  `src/cli/check.rs`, `src/cli/archive.rs`):** `change-proof/v1` hashes tested
  content, delta features, and `verify.command`, and checks `GIVN_RESULT_FILE`
  four-integer counts. Terminal visual evidence in `src/integrity_hook.rs:442-449`
  checks only that a transcript file exists and is non-empty; no mechanical gate
  checks whether `@e2e` step files import production libraries or bypass the
  real binary/PTY.
- **Givn's own acceptance boundary (`tests/steps/`):** 9 files under
  `tests/steps/` currently contain `use givn::*` imports (including
  `development_report_e2e_steps.rs` and `outline_report_support.rs`).
- **Evaluation harness (`/home/buster/projects/givn-harness`):** Supports
  `--givn-addons` (`tests/test_givn_addons.py`), multi-stage Catopia benchmarks
  (`benchmarks/catopia-v2`, `benchmarks/catopia-spring-v2`), LLM judges, and
  `benchmarks/catopia-v2/private/grader/run_grader.py`. Current limits for Ise:
  the follow-up tasks forbid rebuilding from scratch
  (`benchmarks/catopia-v2/public/task-followup.md:28`, `task-followup2.md:28`);
  `run_grader.py` runs host `pip install -r requirements.txt`
  (`run_grader.py:480-481`), marks `P*/T*/M*/A*` `ungraded` when
  `_petorania_supported` cannot run `--tenant petorania law show`
  (`run_grader.py:463-471`, `518-523`), grades the mutable final workspace, and
  proves legacy data by importing `public/legacy-export.csv` into the graded
  workspace (`run_grader.py:299-312`, `check_r12`) rather than carrying a Stage 1
  data directory into Stages 2 and 3.
- **Passuff (`gh-notify`) baseline (`/home/buster/projects/gh-notify`):**
  `scripts/verify.sh` invokes `cargo test -p passuff --features test-support
  --test features_runner` for both regular and `--e2e`; `tui.rs` calls
  `passuff::tui::keys::parse_keys` and exercises in-process TUI helpers.

### What the permanent specs already own

- `initialize-project/enable-optional-addons.feature` — addon toggling,
  `givn init --no-addons`, skill sync/removal per addon, and addon lock refusal
  during an active change.
- `obtain-effective-guidance/serve-extended-rules.feature` — `givn instructions`
  text/JSON serving, addon-conditional sections, project extensions, RED/GREEN/
  REFACTOR batch guidance, and advisory outline use.
- `verify-change/givn-workflow-enforcement.feature` & `visual-review-gate.feature`
  — `givn check tasks`, `givn check review`, receipt freshness, and visual/
  terminal transcript evidence checks.
- `archive-change/archive-proof.feature` & `enforce-post-merge-e2e-gate.feature`
  — forward-only candidate publication, regular + `@e2e` scope execution, and
  proven receipts.
- `assess-code-structure/outline-report.feature` — multi-language static unit
  and call-tree extraction (`Rust`, `Python`, `Java`, `TypeScript`, `JavaScript`).
- `operate-product/declare-the-product-start.feature` — `run:` block validation
  and `givn run` readiness probing.

### Gaps

| ID | Gap |
|---|---|
| G1 | **No opt-in addon default:** `TC-14` and `enable-optional-addons` assume every registered addon defaults to `true`; `addons.ise` needs `default: false` without breaking existing addon semantics or `config-lock.yaml`. |
| G2 | **Instruction context pressure:** `specs.md`, `design.md`, `steps.md`, and `review.md` interleave universal rules with per-stack runner/driver mechanics; adding `ise` without the on-demand stack-instruction split risks `v0.9.15`-style scenario/coverage collapse. |
| G3 | **Undefined Specification in Givn:** No canonical rule or check defines where durable schemas, CLI/wire contracts, technical/viewport limits, and persisted-state migration rules live so a clean-room rebuild has complete inputs after `givn/changes/` is archived; no file declares them, so a rebuild cannot state which paths it deletes (`Q4` closed 2026-11-14 with `givn/specification.yaml`: five role globs plus `source:` roots, no component list). |
| G4 | **Unenforced `@e2e` black-box isolation:** Givn has no mechanical check preventing `@e2e` tests from importing production crates/modules or routing `verify.e2e_command` to an in-process test harness; the rule is now a package boundary, with the runner and every `@e2e` step definition in an `e2e` package that links no production `source`, and a `verify.e2e_command` that fails on an unbound or skipped step (`Q5` closed 2026-11-14). |
| G5 | **Incomplete E2E test identity in receipts:** Verification receipts do not distinguish protected black-box E2E tests from candidate-editable component step definitions, nor bind the running `givn` binary version during an Ise Test; the receipt now binds the `e2e` package digests, and an `Ise Test`'s frozen digest covers the `@e2e` scenarios selected by tag plus that package (`Q5` closed 2026-11-14). |
| G6 | **No reconstruction workflow vs. feature workflow boundary:** Givn instructions assume co-evolving specs and code; there is no clean-room rebuild protocol (what the paths named with `--replace` delete, what stays read-only to the builder, how Constant Ise vs. incremental vs. full rebuild is chosen, and how Rule 2 differences receive a `Spec gap` classification); `givn ise prepare` and `givn ise verify`, bound to `addons.ise`, now materialize and re-check the trial (`Q3` closed 2026-11-14). |
| G7 | **Givn self-rebuild coupling:** Givn's own `@e2e` step support imports `givn::*` library items, and CLI/JSON/YAML schemas are split across `docs/reference.md`, `docs/arc42/`, and `src/`. |

## Open questions

- Closed: `Q1` (entry-point sequence — Option 2 selected 2026-11-13); `Q2`
  (this Frame confirmed with Option 1, 2026-11-13); `Q7` (three personas, the
  bare-clone lens under `givn-maintainer--owner`, 2026-11-13); `Q8` (addon
  posture named `Opt-in addon`, 2026-11-13); `Q9` (all seven Event Storming
  terms revisited one by one on Ise wording, 2026-11-13); `Q10` (Term 1 named
  `Specification`, contrasted with `Source code`, 2026-11-13); `Q11` (Term 2
  `End-to-end (E2E) test` with `black-box` as the no-shared-code
  qualifier, 2026-11-13); `Q12` (Term 3 named `Constant Ise`, 2026-11-14);
  `Q13` (Term 4 named `Ise Test`, with `Constant Ise` as the one-unit scope
  and `Dry-Run Ise` / `Full replacement` as the two execution modes,
  2026-11-14); `Q14` (Term 5 named `Spec gap`, the four-way classification
  record with exactly four outcome values, 2026-11-14); `Q15` (Term 6 added
  no new term: Givn's `Instruction` plus the id convention
  `<id>-examples` / `<id>-<stack>` carry the on-demand instruction,
  2026-11-14); `Q16` (Term 7 named `Change strategy`: the recorded choice
  among incremental modification, `Constant Ise`, and a whole-product
  rebuild, held as the design's `Decides D<n>` entry with the `Ise Test`
  mode beside it when a rebuild runs and no new mandatory field, 2026-11-14);
  `Q6` (the `ise` addon depends on `architecture_principles`: enabling `ise`
  refuses until the prerequisite is on and disabling the prerequisite refuses
  while `ise` is on, so the guidance can use `Component` and the other
  established nouns, 2026-11-14).
- Settled 2026-11-14 (see `session.md` "Settled constraints"): deletion is set
  arithmetic on the paths named with `--replace`, each inside the
  `source:` roots, plus its listed internal unit tests, with no stack-marker
  inference; files in those directories you did not name are reported as
  `Not replaced`, a containment count; component
  step definitions, fakes, and harness are implementation and are never frozen or
  diffed, while component `.feature` scenarios stay in the Specification set;
  only the `End-to-end (E2E) test` target is frozen as acceptance authority;
  changes to neighboring components are review guidance, not an artifact; and
  an E2E freeze applies
  only after the scenario was observed failing for the intended reason.
- `Q4` closed 2026-11-14 (user): a dedicated `givn/specification.yaml`
  declaring the five role globs and one `source:` list of replaceable roots,
  with no component list; the replaced unit is trial input.
- `Q5` closed 2026-11-14 (user): the declared `e2e` role names one package
  holding the runner and every `@e2e` step definition, which links no
  production source and fails on an unbound or skipped step, while feature
  files stay shared; the frozen digest covers the `@e2e` scenarios selected by
  tag plus that package. `Q3` closed 2026-11-14 (user): `givn ise prepare`
  materializes the clean-room directory and writes the trial record when a
  receipt path is given, and
  `givn ise verify --trial <receipt> --at <dir>` re-checks the two digests,
  both bound to `addons.ise`.
- Active now: none. All Event Storming and coupling questions are closed
  (`Q1`-`Q16`); Use Case Definition was reviewed 2026-11-14, Use Case
  Elaboration is complete in `use-cases/`, and Handoff remains.
- Parked for upcoming steps: none.
