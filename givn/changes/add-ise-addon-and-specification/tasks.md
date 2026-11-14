# Tasks: add-ise-addon-and-specification

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`). Strict mode: `.fail_on_skipped()` on both cucumber-rs
builders in `tests/features_runner.rs`; an undefined or skipped step fails the
run. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (design.md, "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
  cargo test --locked --test features_runner -- --exact "<usecase>/<capability>.feature::<title>"
```

`<usecase>/<capability>.feature` is the path under this change's `specs/`.
`<title>` is the scenario title without its `@wip` tag.

Release note (every batch commit subject, verbatim from `proposal.md`, without
backticks):

```text
A project can opt into ise, receive a strictly validated declaration of its retained Specification and replaceable source roots, see four advisory placement findings in givn lint, and be served the ise obligations and the two change records that guidance names.
```

Batch commit rule: one atomic `givn commit` per batch with explicit paths. The
subject is the release note; the body is `Batch: <batch-name>`, plus the
scenario title for a single-scenario batch. Every scenario of the batch records
the same revision as `COMMIT: <hash> -- Batch: <batch-name>`. A commit must
contain the batch's implementation or its regression test. `@wip` is removed
from a scenario when its batch starts, never in bulk; `givn check tasks`
refuses the change while any selected delta scenario still carries `@wip`.

Evidence fields: per the served `tasks` instruction, no separate
TEST-FIRST/RED/TEST-ORDER field is authored here. Each scenario's
implementation checkbox carries the evidence slot the implementer fills with
the actual observed failure and the actual GREEN run (`givn instructions steps`,
TESTS and IMPLEMENTATION). A scenario that passes on its first run is recorded
as existing behavior protected; no production edit is manufactured for it.

Verification command per batch: the single-scenario command above, once per
batch scenario. Whole-change checks are in `## Closeout`.

---

## Interaction coverage pre-flight

Cross-checked every owning use-case `## Interactions` inventory against
design.md's Interaction Coverage Matrix (`design.md:539-615`).

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/initialize-project/usecase.md` | 6 | 6 | yes |
| `specs/upgrade-a-project/usecase.md` | 4 | 4 | yes |
| `specs/declare-the-specification/usecase.md` | 1 | 1 | yes |
| `specs/verify-change/usecase.md` | 13 | 13 | yes |
| `specs/obtain-effective-guidance/usecase.md` | 1 | 1 | yes |
| `specs/create-change/usecase.md` | 9 | 9 | yes |
| `specs/fragments/fragment.md` (support) | 32 | 32 | yes |
| **Total** | **66** | **66** | **all 66 tagged `@e2e`** |

No gap: every matrix target title resolves to a scenario in the permanent tree
or this change's `specs/`, and every target carries `@e2e`. No task is written
for an `@e2e` scenario that does not exist.

---

## Domain Constraints

Copied from the owning use-case documents before any scenario task. Each
scenario's Design constraints block repeats the rules its behavior exercises.

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `enable-optional-addons` | Each registered addon declares its own default; `ise` is the one `Opt-in addon` (`default_enabled: false`) and requires `architecture_principles`. A fresh init writes explicit `true` for default-on addons and omits `ise`; `--no-addons` writes explicit `false` for all ten. Both prerequisite refusals name the addons, exit nonzero, and write neither flag. The first effective enable materializes `givn/specification.yaml` once; an existing declaration is project-owned byte-for-byte; a path occupied by a directory is not an existing declaration. Disabling `ise` removes nothing. A lock written before `ise` existed omits its key and the comparison reads only recorded keys. | `developer--primary-user`, `givn-maintainer--owner` |
| `initialize-a-project` | Contract unchanged: no fresh-initialization path writes the declaration; setup guidance stays technology-agnostic. The interactive selection still requires at least one target and the fresh posture omits `ise`. | `developer--primary-user` |
| `apply-a-managed-upgrade` | The declaration is written and staged only when `ise` is effective and the file is absent; content is the documented shape, not a project decision. An existing declaration joins neither the write nor the staged set and is reported project-owned. The write and the staged set extend together; the failure snapshot names a written declaration. A write failure exits nonzero, names the path and completed writes, and claims no rollback. A repeated upgrade on an unchanged project creates no revision. | `developer--primary-user`, `givn-maintainer--owner` |
| `specification-declaration` | `givn/specification.yaml` holds exactly five roles (`behavior`, `structure`, `contracts`, `limits`, `e2e`) plus one `source:` list; every path is project-bounded. The schema is strict: unknown key, unknown or repeated role, omitted role, absent `source:`, path outside the project, non-list value, malformed YAML, or unreadable file is refused by name and never read. A refusal never rewrites the file and exposes no partial role set. An empty role list retains nothing. One declaration feeds every reader. | `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner` |
| `check-specification-health` | Four advisory findings: declared path missing (four roles other than `e2e`), one path in two roles, `.feature` outside the `behavior` globs, `e2e` role resolving no package or an edge into `source`. Advisory findings never change the exit code. The `e2e` absent package is reported once by the boundary finding. The feature scan excludes the active-change tree and the archive. A malformed declaration is a blocking declaration error (exit 1), not an advisory. An absent/unreadable declaration on an `ise`-effective project exits 1 and reports the declaration checks as not performed, with no advisory computed. The load runs before the empty-feature early return. Change-scoped lint keeps its refusals and reads no declaration. A project with `ise` off reads no declaration. The `e2e` edge comparison is Cargo-only and static; zero manifests or a malformed manifest is reported/skipped as designed. | `developer--primary-user`, `givn-maintainer--owner` |
| `serve-extended-rules` | An addon-bound id is served only while its addon is effective; with `ise` off the request exits nonzero and names `ise` without printing content. Text and machine-readable paths carry the same `content`. Addon state is read at request time. Unknown ids keep the known-id diagnostic with `ise` listed. | `agent--technical-role` |
| `change-strategy`, `spec-gap` | One `Change strategy` as a `Decides D<n>` ledger entry: `incremental modification`, `Constant Ise`, or a whole-product rebuild, with the `Ise Test` mode (`Dry-Run Ise` or `Full replacement`) beside it when the change rebuilds; the entry names the affected consumers, persisted state, and rebuild boundary; no new mandatory field. An unresolved strategy or a rebuild with no mode is a design-review finding. One `Spec gap` outcome before any fix out of `E2E test defect`, `implementation defect`, `specification defect`, `acceptable undocumented change`; the record names the wrong artifact, not who found it; only `specification defect` is a gap in the Specification; `E2E test defect` stops the trial and restarts it after the test is corrected; no difference is closed as silent drift. | `agent--technical-role`, `developer--primary-user` |
| `instruction-integrity`, `match-guidance-to-enabled-addons` | The `ise` instruction carries the five obligations and uses `Component`, `Port`, contract, `Specification`, `Source code` because `ise` requires `architecture_principles`. The review asks the neighboring-components question as a question, never a gate. `Opt-in addon` names one posture; "an enabled addon" names any addon that is on. | `agent--technical-role` |

Guarantees carried by every batch: a refused request leaves project state
unchanged and names the condition; a declared path, role, addon, or file is
never invented; user-facing output names the flag, role, addon, or path it is
about and never an internal identifier.

---

## Contradictions

Recorded, not fixed. Each is resolved in the task text by naming the
authoritative artifact; `design.md` and the delta features are unchanged.

1. **`design.md` has no "Implementation Plan" or "Behavior Batches" section.**
   The brief names them. Batch order below follows the design's dependency
   order (Overview surface order; Registry and prerequisite behavior ->
   Upgrade behavior -> Specification-health pass -> Instruction serving ->
   Guidance amendments) and the manifest/`NAMES` ordering constraint the
   design states in "Registry and manifest". No design decision is re-made.
2. **`design.md` "Upgrade behavior" names `run_managed_upgrade`; no such
   function exists.** The real entry point is
   `crate::cli::migration::prepare_managed_upgrade` (`src/cli/migration.rs:403`),
   which calls `apply_managed_upgrade` (`:567`) and
   `commit_managed_upgrade` (`:611`). Tasks name the real functions; the
   design's `declaration_absent` decision is unchanged.
3. **`design.md` "Vocabulary and documents" says each glossary row is
   committed in the batch whose code makes it true, but the rows are already
   committed.** `60db843b` carries `docs/glossary.md` (+20 lines),
   `docs/arc42/{01,02,03,04,05,06,08,09,11}`, `docs/arc42/adr/ADR-010`,
   `docs/domain-model.md`, and `docs/reference.md` (+36 lines). The batches
   therefore verify those statements against the implemented behavior instead
   of re-committing them; placement per row is in the next section.
4. **`design.md` "Vocabulary and documents" also names
   `assets/config.yaml.tmpl` prose; that file is not yet changed.** It still
   says "Every known addon is enabled by default" and "A missing key in a
   partial `addons:` block means enabled". It lands in batch `addon-registry`
   because `src/config/manifest.rs` requires every documented key to map to a
   schema field, so the `ise` comment line and the `ise` field must land
   together.
5. **Spec delta `enable-optional-addons.feature` asserts
   `stderr does not contain "add-on configuration changed during change"`,
   while `design.md` says the lock comparison reads only recorded keys.** Not a
   conflict: the scenario asserts the absence of the drift message, which the
   recorded-keys comparison produces. The task pins the assertion to the
   absence of that message, not a zero exit, because
   `givn check review` fails for unrelated reasons on a fixture change.

---

## Vocabulary and arc42 placement

The text is committed in `60db843b`. Each row is verified in the batch whose
code makes the statement true; no batch re-commits it.

| Statement | Committed location | Verified in batch | What makes it true |
|---|---|---|---|
| `Addon`, `Addon default`, `Opt-in addon` rows | `docs/glossary.md:8-9,195` | `addon-registry` | `AddonSpec.declared_default`, `ise` with `default_enabled: false`, `Addons.ise` serde default |
| arc42 01 addon-posture sentence | `docs/arc42/01-introduction-and-goals.md:24` | `addon-registry` | same registry default |
| arc42 02 `TC-14` | `docs/arc42/02-architecture-constraints.md:24` | `addon-registry` | same registry default + partial-block resolution |
| arc42 04 strategy row, arc42 05 `config::addons`, arc42 08 §8.11 | `docs/arc42/{04-solution-strategy,05-building-block-view,08-crosscutting-concepts}.md` | `addon-registry` | the new `config::addons` module and its consumers |
| `docs/reference.md` addon section (`ise`, prerequisite, listing) | `docs/reference.md:104,120` | `addon-registry` | implemented default, prerequisite, listing prose |
| `Specification`, `Source code`, `Feature file`, `Permanent spec`, `Use Case Elaboration` rows | `docs/glossary.md:86,91,140,142,176` | `declaration-loader` | the strict five-role-plus-`source` schema in `config::declaration` |
| arc42 03 declaration input, arc42 05 `config::declaration`, arc42 08 §8.2/§8.4/§8.8, ADR-010 amendment | `docs/arc42/{03,05,08}/…`, `docs/arc42/adr/ADR-010-manifest-overlay.md` | `declaration-loader` | the read-only declaration authority and its blocking error |
| `docs/domain-model.md:94` corpus clarification | `docs/domain-model.md` | `declaration-loader` | the Specification umbrella vs. the behavior corpus |
| `End-to-end (E2E) test` row | `docs/glossary.md:72` | `specification-health` | the declared `e2e` package resolution and edge finding |
| `change strategy`, `Constant Ise`, `Ise Test`, `Spec gap` rows | `docs/glossary.md:41-45,141` | `guidance-amendments` | the design, design-review, proposal, and review guidance blocks |
| arc42 06 §6.4 health pass, §6.12 opt-in enable/disable, §6.13 upgrade scaffold | `docs/arc42/06-runtime-view.md` | `specification-health`, `declaration-scaffold-enable`, `declaration-scaffold-upgrade` | the three runtime flows |
| arc42 11 `D-28` Cargo-only limit | `docs/arc42/11-risks-and-technical-debt.md` | `specification-health` | the `e2e` boundary finding's Cargo reader |

---

## Setup

- [ ] Read the owning use-case documents and this change's seed before writing
      any step: `specs/*/usecase.md` (guarantees, rules, Personas),
      `specs/fragments/fragment.md` (the `ise` obligations paragraph), and
      `givn/ideation/ise-rebuildability/changes/add-ise-addon-and-specification.md`.
      Domain constraints are in `## Domain Constraints` above.

- [ ] Confirm the runner and `verify.command`. No new runner is installed:
      `tests/features_runner.rs` uses cucumber-rs 0.23 with `.fail_on_skipped()`
      at both registration points, `verify.command` is
      `./run-tests.sh --non-e2e`, and `verify.e2e_command` is
      `./run-tests.sh --e2e`. The runner executes `givn/specs/**` by default and
      the change's `specs/**` under `GIVN_FEATURES`.

- [ ] Create the step-definition skeletons, one file per capability, each
      registered in `tests/steps/mod.rs` with `unimplemented!()` bodies:
      `tests/steps/enable_optional_addons_ise_steps.rs`,
      `tests/steps/enable_optional_addons_ise_e2e_steps.rs`,
      `tests/steps/specification_declaration_steps.rs`,
      `tests/steps/specification_declaration_e2e_steps.rs`,
      `tests/steps/specification_health_steps.rs`,
      `tests/steps/change_strategy_steps.rs`,
      `tests/steps/change_strategy_e2e_steps.rs`,
      `tests/steps/spec_gap_steps.rs`,
      `tests/steps/spec_gap_e2e_steps.rs`.
      Extended existing files: `tests/steps/initialize_a_project_e2e_steps.rs`,
      `tests/steps/run_upgrade_steps.rs`,
      `tests/steps/serve_extended_rules_steps.rs`,
      `tests/steps/instruction_integrity_steps.rs`,
      `tests/steps/template_conditional_rendering_steps.rs`.
      Reuse, never re-bind: `the user runs `cmd``, `the consumer runs `cmd``,
      `the command exits 0` / `non-zero`, `stdout contains "…"`,
      `stderr contains "…"`, `the file "…" exists` / `does not exist`,
      `the terminal transcript is captured for this scenario`
      (`workflow_enforcement_e2e_steps.rs:50`),
      `an initialized project whose only explicit addon setting disables outline`
      and `the user synchronizes skills and lists addons`
      (`default_all_addons_steps.rs`), and
      `the command exits successfully and reports the managed upgrade revision`
      (`run_upgrade_steps.rs:636`).
      A phrase is bound exactly once across all files; a duplicate registration
      panics the runner.
      ```
      <!-- paste: cargo check --locked --tests output and the file list -->
      ```

- [ ] Proof-of-strictness. Write one temporary probe feature under
      `givn/changes/add-ise-addon-and-specification/specs/` containing one
      scenario whose step is undefined, run the single-scenario command against
      it, and confirm a non-zero exit.
      ```
      <!-- paste: command + non-zero output + the cucumber "undefined/skipped" report -->
      ```
      If it exits 0, stop: `.fail_on_skipped()` is not in effect on the builder
      the run used.

- [ ] Restore a passing baseline: delete the probe, run
      `./run-tests.sh --non-e2e`, and confirm exit 0. The delta scenarios stay
      `@wip`; the permanent tree does not use any phrase above.
      ```
      <!-- paste: ./run-tests.sh --non-e2e summary -->
      ```

- [ ] COMMIT: `<hash>` -- Batch: `setup`

---

## Behavior batches

Each batch names the complete file set for its scenarios; the scenarios share
that one test phase, one implementation phase, and one refactor phase. A
scenario's Design constraints name the step-definition file that binds it and
the production symbols it exercises, so no per-scenario file list repeats the
batch header.

### Batch 1: `addon-registry`

One test phase, one implementation phase, one refactor phase, one commit.
Implementation order inside the batch: (a) the `config`-owned registry, which
unblocks `config::lock`; (b) the `Addons` `ise` field, `NAMES`, `entries()`,
and `Default`; (c) the consumers.

**Files created:** `src/config/addons.rs`.
**Files modified:** `src/config/mod.rs`, `src/config/manifest.rs`,
`src/config/lock.rs`, `src/skills/render.rs`, `src/cli/addons.rs`,
`src/cli/init.rs` (only if the fresh/forced loops need the declared default),
`assets/config.yaml.tmpl`, `tests/steps/enable_optional_addons_ise_steps.rs`,
and `tests/common/mod.rs` only if a shared fixture helper is added there (test
support, never a product API).
**Design constraints (batch-wide):**
- `AddonSpec { id, description, default_enabled, requires: &'static [&'static str] }`
  and `KNOWN_ADDONS` move from `cli::addons` to `config::addons`; registry
  order is `arc42, communication_policy, readme, coverage, domain_modeling,
  dev_principles, architecture_principles, challenge, outline, ise`. The nine
  existing entries get `requires: &[]`; `ise` gets
  `default_enabled: false, requires: &["architecture_principles"]`.
- `AddonSpec::declared_default(id) -> Option<bool>` is the single declared
  default read by `givn addons list`, `config::lock::proposed_addons`, and
  `run_enable`'s recorded `before`.
- `cli::addons` imports the registry and re-exports nothing; `config` imports
  no `cli` type; `src/config/mod.rs` declares `addons` beside `cascade`.
- `Addons` gains `pub ise: bool` with `#[serde(default = "disabled_by_default")]`
  (new `fn disabled_by_default() -> bool { false }`); `NAMES` becomes
  `[&'static str; 10]`, `entries()` becomes `[(&'static str, bool); 10]`, and
  `Default` adds `ise: false`; `ise` is **last** in all three.
- `src/skills/render.rs`: `KNOWN_ADDON_KEYS` gains `ise`;
  `manifest_addon_enabled` maps `"ise" => manifest.addons.ise`. This must land
  before any asset uses `{{#if addons.ise}}`, otherwise `givn skills sync`
  fails with "unknown addon".
- `config::lock::proposed_addons` proposes `declared_default(name)` (false when
  `no_addons`), not `!no_addons`; `config::lock::effective_addons` and
  `differences()` are unchanged.
- `givn addons list` keeps the `  [{state}]  {id}` shape (`[enabled ]`,
  `[disabled]`) and replaces the tip sentence with prose containing the
  literal lowercase substring `each omitted key resolves that addon's declared default`.
- `assets/config.yaml.tmpl`: state the per-addon declared default in the addon
  prose, add one commented `ise` line, and keep every key the schema test
  requires (`src/config/manifest.rs:410-445`).
- Step files drive the built binary (`common::run_givn`) and read files; they
  never import `config::addons` or `config::lock`. Unit tests inside the module
  may call the API directly.

**Batch verification:** the two single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: addon-registry'`.

- [x] Scenario: `givn init writes the nine default-on addons and omits ise`
  - Design constraints:
    - Fixture: a temp root without `givn/`; run `givn init --agents opencode`.
    - Fresh path: the loop over `KNOWN_ADDONS` already gates on
      `addon.default_enabled`, so `ise` is skipped without a new branch; the
      nine others are written through `enable_scaffold`.
    - `givn/config.yaml` must contain the lines `  arc42: true` and
      `  outline: true`, must not contain any line matching `  ise:`, and
      `givn/specification.yaml` must not exist.
    - No fresh-initialization flag makes `ise` effective.
    - Steps: new bindings in `tests/steps/enable_optional_addons_ise_steps.rs`
      for the config line assertions; reuse `the user runs`, `the command
      exits 0`, `the file "…" does not exist`.
  - [x] Implement the scenario and its tests (test-first per `design.md`).
        Evidence (observed failure, then GREEN):
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::givn init writes the nine default-on addons and omits ise"

        before the registry change: 1 scenario (1 passed); 7 steps (7 passed)
        existing behavior protected: the fresh-init loop already skips an addon
        whose declared default is false, so the scenario rejects a registry
        entry that makes `ise` default-on or writes its key.
        after the registry change:  1 scenario (1 passed); 7 steps (7 passed)
        ```
  - [x] REFACTOR: inspected `givn outline report --path src/config` for
        `config::addons`, `config::manifest`, and `config::lock`: the registry
        owns declared defaults and prerequisites, the manifest owns the serde
        schema, the lock consumes `declared_default` with no `cli` dependency;
        names read as domain operations. Re-ran: 7 steps passed. No behavior
        change.
  - [x] COMMIT: `b5fe611455ddd3d31191e94fddb50b1cc17df024` -- Batch: `addon-registry`

- [x] Scenario: `a partial addons block resolves each omitted addon to its declared default`
  - Design constraints:
    - Fixture: reuse the existing Given
      `an initialized project whose only explicit addon setting disables outline`
      (`default_all_addons_steps.rs:127`) and the existing When
      `the user synchronizes skills and lists addons` (`:233`).
    - Reuse the existing Then
      `outline is disabled in listing, guidance, artifacts, and addon execution`
      (`:339`); add the two new Then bindings.
    - `givn addons list` must report `[disabled]` for `ise` (declared default
      false, omitted key) and `[enabled ]` for omitted default-on addons,
      including `challenge` and `communication_policy`.
    - The same effective defaults must be used for artifacts, skills, listing,
      and execution; one source is `manifest.addons.get`, not a second table.
  - [x] Implement the scenario and its tests. Evidence (observed failure, then GREEN):
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::a partial addons block resolves each omitted addon to its declared default"

        RED: 4 steps (3 passed, 1 failed)
          ✘ And the listing reports ise as disabled
          Step panicked. Captured output: the addon listing does not report ise disabled:
            (the listing carried only the nine pre-existing addons)
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran; 5 steps passed. No behavior change.
  - [x] COMMIT: `b5fe611455ddd3d31191e94fddb50b1cc17df024` -- Batch: `addon-registry`

---

### Batch 2: `addon-prerequisites`

**Files modified:** `src/config/addons.rs` (only if the `requires` read lives
there), `src/cli/addons.rs`, `tests/steps/enable_optional_addons_ise_steps.rs`.
**Design constraints (batch-wide):**
- `run_enable` reads the effective manifest once, then bails **before**
  `set_addon` or any scaffold when a `requires` entry is effective-off:
  `addon 'ise' requires 'architecture_principles'; enable it first`.
- `run_disable` bails when a registered addon whose `requires` contains this id
  is effective-on, naming the first dependent in registry order:
  `addon 'architecture_principles' is required by 'ise'; disable 'ise' first`.
- Both refusals exit nonzero and leave both stored flags unchanged; no file is
  written.
- Refusal strings are exact; no internal identifier appears in output.

**Batch verification:** the single-scenario command below; the reverse
direction's E2E is batch `addon-prerequisites-e2e`.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: addon-prerequisites'`.

- [x] Scenario: `Enabling ise without its prerequisite is refused by name`
  - Design constraints:
    - Fixture: temp root, `givn init --agents opencode`, then
      `givn addons disable architecture_principles`.
    - Run `givn addons enable ise`; assert exit non-zero, stderr contains the
      exact string above, both addons stay disabled in `givn/config.yaml`,
      and `givn/specification.yaml` is absent.
    - No write occurs before the refusal; the config bytes are unchanged.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::Enabling ise without its prerequisite is refused by name"

        RED: 3 steps (2 passed, 1 failed)
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
          stdout=enabled addon 'ise' in givn/config.yaml
        GREEN: 1 scenario (1 passed); 7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran; 7 steps passed. No behavior change.
  - [x] COMMIT: `013aaa4b8953609ca44838719238f3cdfe0baf39` -- Batch: `addon-prerequisites`

Reverse-direction check through the real binary in a throwaway temp project
(its `@e2e` scenario is batch `addon-prerequisites-e2e`):

```
$ givn addons disable architecture_principles
error: addon 'architecture_principles' is required by 'ise'; disable 'ise' first
exit 1; givn/config.yaml bytes unchanged (sha256 equal before/after)

$ givn addons enable ise            # architecture_principles off
error: addon 'ise' requires 'architecture_principles'; enable it first
exit 1; givn/config.yaml bytes unchanged; givn/specification.yaml absent
```

---

### Batch 3: `declaration-loader`

**Files created:** `src/config/declaration.rs`.
**Files modified:** `src/config/mod.rs`, `src/cli/lint.rs` (blocking
declaration error only; advisories are batch `specification-health`),
`tests/steps/enable_optional_addons_ise_steps.rs` (fixture helpers shared with
batch 1), `tests/steps/specification_declaration_steps.rs`.
**Design constraints (batch-wide):**
- `DECLARATION_PATH = "givn/specification.yaml"`.
  `SpecificationDeclaration { behavior, structure, contracts, limits, e2e, source }`
  with `Role { Behavior, Structure, Contracts, Limits, E2e }` (`as_str`, `ALL`),
  `DeclarationError`, `load(project_root) -> Result<Option<Self>, DeclarationError>`,
  `roles() -> [(Role, &[String]); 5]`, `is_effective(&Manifest) -> bool`
  (`manifest.addons.ise`).
- Strictness: exactly six top-level keys; all required; each a list of strings;
  `serde_yaml_ng` with `deny_unknown_fields`; a source-line scan of
  `^[A-Za-z_][A-Za-z0-9_]*:` rejects a repeated top-level key before parsing.
- Every value is project-relative, non-empty, not `/`-prefixed, no Windows
  prefix, and cannot escape the project after `Path::components` normalization.
- Refusal messages (design.md "Refusal messages"; each names the offending
  element and the file):
  `givn/specification.yaml: unknown key '<key>' — the declaration holds only behavior, structure, contracts, limits, e2e, and source`;
  `givn/specification.yaml: '<role>' is declared more than once`;
  `givn/specification.yaml: missing role '<role>'`;
  `givn/specification.yaml: missing the 'source:' list of replaceable roots`;
  `givn/specification.yaml: '<path>' is outside the project`;
  `givn/specification.yaml: <yaml, type, or read reason>`.
- Ordering: the unknown-key scan runs before the missing-role check, so a
  misspelled role is named once and the role it replaces is not also reported.
- `Ok(None)` when the file is absent; both variants carry no advisory data.
- `src/cli/lint.rs`: when `default_scope` and
  `SpecificationDeclaration::is_effective(&manifest)`, load **before** the
  `feature_files.is_empty()` early return. `Err` or `Ok(None)` produce one
  `Severity::Blocking` finding of kind `specification_declaration` on stderr,
  exit 1, and no advisory finding. Nothing is loaded when `ise` is off or in
  change scope. The declaration is never written by the loader.
- Step files run `givn lint` through `common::run_givn`; they never import
  `config::declaration`.

**Batch verification:** the six single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: declaration-loader'`.

- [x] Scenario: `A malformed declaration is refused by name and left unchanged`
  - Design constraints:
    - Fixture: `givn init --agents opencode`, `givn addons enable ise`
      (scaffold), record the declaration bytes, then apply one mutation per
      Example.
    - The seven Examples map to: unknown key `components`; role spelled
      `behaviour`; repeated `e2e`; omitted `limits`; omitted `source:`; source
      root `../other/crates/**`; structure path `/opt/specs/**`.
    - Assert exit non-zero, stderr names the mutation's `<named>` value, and
      the recorded bytes equal the bytes after the run.
    - `SpecificationDeclaration::load` performs no write; the lint command
      never rewrites the file.
  - [x] Implement the scenario and its tests (all Examples). Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::A malformed declaration is refused by name and left unchanged"

        RED: 7 scenarios (7 failed); 21 steps (14 passed, 7 failed) — each Example
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
            stdout=givn lint: no .feature files found under givn/
        GREEN: 7 scenarios (7 passed); 35 steps (35 passed)
        ```
  - [x] REFACTOR: re-ran all Examples; 35 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

- [x] Scenario: `A declaration that is not valid YAML is refused by name`
  - Design constraints:
    - Fixture: an `ise`-enabled project whose declaration is syntactically
      invalid YAML (e.g. an unclosed list).
    - `DeclarationError::Malformed` names `givn/specification.yaml` and the
      parser reason; exit 1.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::A declaration that is not valid YAML is refused by name"

        RED: 3 steps (2 passed, 1 failed)
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
            stdout=givn lint: no .feature files found under givn/
        GREEN: 1 scenario (1 passed); 4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran; 4 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

- [x] Scenario: `A role that is not a list is refused by name`
  - Design constraints:
    - Fixture: `behavior: "givn/specs/**"` (scalar) with the other five keys
      valid.
    - The type error names `behavior`; exit 1.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::A role that is not a list is refused by name"

        RED: 3 steps (2 passed, 1 failed)
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
            stdout=givn lint: no .feature files found under givn/
        GREEN: 1 scenario (1 passed); 4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran; 4 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

- [x] Scenario: `An unreadable declaration is refused by name`
  - Design constraints:
    - Fixture: `givn/specification.yaml` is a directory.
    - The read error is `DeclarationError::Malformed` naming the path; exit 1.
    - The path-occupied-by-a-directory case is the same condition the scaffold
      batch reports on the write path; here the loader reports it on read.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::An unreadable declaration is refused by name"

        RED: 3 steps (2 passed, 1 failed)
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
            stdout=givn lint: no .feature files found under givn/
        GREEN: 1 scenario (1 passed); 4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran; 4 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

- [x] Scenario: `An unknown role is named before the role it replaces`
  - Design constraints:
    - Fixture: the declaration spells `behaviour` and omits `behavior`
      (the other four roles and `source:` valid).
    - One refusal only: stderr names `behaviour` and does not report the
      omitted role `behavior` (assert the absence of the missing-role message,
      not of the word in the permitted-key list).
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::An unknown role is named before the role it replaces"

        RED: 3 steps (2 passed, 1 failed)
          ✘ Then the command exits non-zero
          Step panicked. Captured output: expected failure, got exit Some(0):
            stdout=givn lint: no .feature files found under givn/
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran; 5 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

- [x] Scenario: `A project with ise disabled reads no declaration`
  - Design constraints:
    - Fixture: initialized project selecting opencode, `ise` disabled, with a
      malformed declaration written at `givn/specification.yaml`.
    - Run `givn lint`; assert exit 0 and no declaration finding on stderr. The
      load is gated on `is_effective`, so the malformed file is never read.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "declare-the-specification/specification-declaration.feature::A project with ise disabled reads no declaration"

        existing behavior protected: lint already ignored the declaration, so the
        scenario passed before the loader existed — 1 scenario (1 passed); 4 steps
        (4 passed). It now rejects a load that is not gated on `is_effective`.
        GREEN after the loader: 1 scenario (1 passed); 4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran; 4 steps passed. No behavior change.
  - [x] COMMIT: `ca2c4b511e93ff7451776e9e881a1fd00fcf77fd` -- Batch: `declaration-loader`

Refusal messages through the real binary in a throwaway temp project
(`givn init --agents opencode`, `givn addons enable ise`, one declaration
mutation per run; exit 1 in every case, declaration bytes unchanged):

```
$ givn lint            # valid declaration
givn lint: no .feature files found under givn/            exit 0

[DECL ] blocking  givn/specification.yaml  unknown key 'components' — the declaration holds only behavior, structure, contracts, limits, e2e, and source
[DECL ] blocking  givn/specification.yaml  unknown key 'behaviour' — the declaration holds only behavior, structure, contracts, limits, e2e, and source
[DECL ] blocking  givn/specification.yaml  'e2e' is declared more than once
[DECL ] blocking  givn/specification.yaml  missing role 'limits'
[DECL ] blocking  givn/specification.yaml  missing the 'source:' list of replaceable roots
[DECL ] blocking  givn/specification.yaml  '../other/crates/**' is outside the project
[DECL ] blocking  givn/specification.yaml  '/opt/specs/**' is outside the project
[DECL ] blocking  givn/specification.yaml  behavior: invalid type: string "givn/specs/**", expected a sequence at line 1 column 11
[DECL ] blocking  givn/specification.yaml  did not find expected ',' or ']' at line 2 column 1, while parsing a flow sequence at line 1 column 11
[DECL ] blocking  givn/specification.yaml  Is a directory (os error 21)
[DECL ] blocking  givn/specification.yaml  the declaration is absent; the declaration checks were not performed

$ givn lint --json      # absent declaration
"kind": "specification_declaration", "level": "error", "severity": "blocking"   exit 1

$ givn addons disable ise && givn lint                 # malformed file on disk
givn lint: no .feature files found under givn/            exit 0
```

Batch REFACTOR check (outline addon enabled):

```
target/debug/givn outline report --path src/config       → 10 files, 34 units, 0 hints
target/debug/givn outline report --path src/cli/lint.rs  → 1 file, 3 units, 1 hint (multi-type-module)
```

Covered: Rust; `config::declaration`, `config::addons`, `config::manifest`,
`config::lock`, `config::overlay` and `cli::lint`. Omitted: `tests/steps`
(step glue, not a component) and the embedded assets. `config::declaration`
states one responsibility — read and strictly validate the declaration — in
its declarations alone: `Role`, `SpecificationDeclaration` (`load`, `roles`,
`is_effective`), `DeclarationError` (`detail`), and the private helpers
`scan_top_level_keys`, `is_key`, `validate_role_paths`, `is_project_relative`,
`has_windows_prefix`; the flow is `load` -> `parse` -> key scan -> role
presence -> parse -> `validate_paths`. `cli::lint`'s `multi-type-module` hint
predates this batch (`SeedStatus` and `Severity`) and
`specification_declaration_finding` appears only in `run_with_root`'s call
flow. No structural change made; behaviour unchanged.

---

### Batch 4: `declaration-scaffold-enable`

Realizes decision D1. **Files created:** `assets/specification.yaml.tmpl`.
**Files modified:** `src/cli/addons.rs` (`scaffold_enabled`, `disable_scaffold`,
declaration-specific write), `src/cli/init.rs` (forced path only if needed),
`tests/steps/enable_optional_addons_ise_steps.rs`.
**Design constraints (batch-wide):**
- The template writes the six required keys with **empty active lists** and a
  comment block naming each role with one example path; it validates
  immediately and invents no active project path. Realized by D1.
- `scaffold_enabled("ise")` writes `givn/specification.yaml` from the template
  through a declaration-specific write that checks `is_file()`, not `exists()`.
  An existing regular file is project-owned: bytes preserved, reported
  `content preserved`, exit 0. A directory or other non-regular file is not an
  existing declaration: the write is attempted and its failure returns an
  error naming the path, the reason `not a regular file`, and the note that
  `addons.ise` was already written; no completion is claimed and exit 0 is not
  reported.
- `disable_scaffold("ise")` removes nothing and prints
  `  exists   givn/specification.yaml  (project-owned; not removed)` when the
  declaration is a regular file; the disable report names it project-owned.
- `givn init --force` preserves an effective `ise` and calls
  `scaffold_enabled("ise")`, producing the same file; a repeated enable changes
  nothing.
- One materialization path shared by init and `givn addons enable`.

**Batch verification:** the six single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: declaration-scaffold-enable'`.

- [x] Scenario: `Enabling ise writes the setting and scaffolds the declaration`
  - Design constraints:
    - Fixture: initialized project selecting opencode (nine default-on addons;
      `architecture_principles` is on).
    - `givn addons enable ise` exits 0, stdout names
      `givn/specification.yaml`, `ise` is enabled in the config, the file
      exists, and it names exactly `behavior, structure, contracts, limits,
      e2e` plus one `source` list.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::Enabling ise writes the setting and scaffolds the declaration"

        RED: stdout does not name "givn/specification.yaml":
             enabled addon 'ise' in givn/config.yaml
        GREEN: 1 scenario (1 passed); 7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 7 steps (7 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

- [x] Scenario: `Enabling ise preserves an edited declaration`
  - Design constraints:
    - Fixture: initialized project, a recorded declaration at
      `givn/specification.yaml` with project bytes.
    - Exit 0, stdout contains `content preserved`, bytes unchanged. The
      declared default is not overwritten by the scaffold.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::Enabling ise preserves an edited declaration"

        RED: stdout missing "content preserved":
             enabled addon 'ise' in givn/config.yaml
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 6 steps (6 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

- [x] Scenario: `Repeating the enable changes nothing`
  - Design constraints:
    - Fixture: `ise`-enabled project with recorded declaration bytes.
    - A second `givn addons enable ise` exits 0 and leaves the bytes equal to
      the recorded bytes.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::Repeating the enable changes nothing"

        existing behavior protected: passed on its first run
        (1 scenario (1 passed); 4 steps (4 passed)). No production edit was
        manufactured: a second enable already left the recorded declaration
        bytes unchanged.
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 4 steps (4 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

- [x] Scenario: `A forced re-initialization preserves an effective ise and its declaration`
  - Design constraints:
    - Fixture: `ise`-enabled project selecting opencode with recorded bytes.
    - `givn init --force --agents opencode` exits 0, `ise` stays enabled, bytes
      unchanged. The forced loop iterates all ten and applies the preserved
      value; the declaration is not rewritten.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::A forced re-initialization preserves an effective ise and its declaration"

        existing behavior protected: passed on its first run
        (1 scenario (1 passed); 5 steps (5 passed)). The forced loop already
        restored `ise: true` and called the addon scaffold; this batch's
        scaffold preserves an existing regular file. No production edit was
        manufactured for this scenario.
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 5 steps (5 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

- [x] Scenario: `Disabling ise preserves the declaration and reports it project-owned`
  - Design constraints:
    - Fixture: `ise`-enabled project with recorded bytes.
    - `givn addons disable ise` exits 0, stdout contains `project-owned`, `ise`
      is disabled, bytes unchanged. No file is removed.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::Disabling ise preserves the declaration and reports it project-owned"

        RED: stdout missing "project-owned":
             disabled addon 'ise' in givn/config.yaml
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 6 steps (6 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

- [x] Scenario: `A declaration scaffold failure names the path and the applied setting`
  - Design constraints:
    - Fixture: initialized project; `givn/specification.yaml` is created as a
      directory before the enable.
    - `givn addons enable ise` exits non-zero, stderr names
      `givn/specification.yaml`, and `ise` stays enabled because `set_addon`
      wrote the flag before the scaffold.
    - The error text names the reason and the already-written setting; no
      success line is printed.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "initialize-project/enable-optional-addons.feature::A declaration scaffold failure names the path and the applied setting"

        RED: expected failure, got exit Some(0):
             stdout=enabled addon 'ise' in givn/config.yaml
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        observed stderr: write the declaration 'givn/specification.yaml':
             Is a directory (os error 21); the path is not a regular file, and
             addons.ise was already written to givn/config.yaml; no completion
             is claimed
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 6 steps (6 passed)).
  - [x] COMMIT: `c883c8ee7d197eb517ff49b5a6e7ba905b7cf77f` -- Batch: `declaration-scaffold-enable`

---

### Batch 5: `declaration-scaffold-upgrade`

**Files modified:** `src/cli/migration.rs`
(`prepare_managed_upgrade`, `apply_managed_upgrade`,
`commit_managed_upgrade`), `tests/steps/run_upgrade_steps.rs`.
**Design constraints (batch-wide):**
- In `prepare_managed_upgrade` (design.md says `run_managed_upgrade`; see
  `## Contradictions` 2): compute
  `declaration_absent = ise_effective && !path.is_file()`. A path occupied by a
  directory counts as absent, so the write is attempted and its failure is
  reported.
- Add the declaration path to `snapshot_paths` when
  `declaration_absent && !path.exists()`, so a failed upgrade restores it and
  `report.completed` names it.
- `apply_managed_upgrade` writes the scaffold from
  `assets/specification.yaml.tmpl` when `declaration_absent`, pushes
  `givn/specification.yaml (declared Specification)` onto `report.completed`,
  and returns whether it wrote the file. When the file exists it prints
  `  exists   givn/specification.yaml  (project-owned; not staged)` and reports
  it as project-owned.
- `commit_managed_upgrade` receives the declaration path only in the staged
  allowlist when it was written; an existing declaration is never staged.
  `ise` off: no write, no stage, no read; a leftover declaration is untouched.
- A write failure exits nonzero, names the path and the completed writes, and
  offers checkpoint-based recovery with no rollback claim.
- The declaration content is the documented shape (D1), not a project decision;
  it is not a migration plan and creates no change or plan revision.
- Step files drive the binary through `common::run_givn` / Git fixtures and
  inspect the revision with `common::git`; no `migration` import.

**Batch verification:** the five single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: declaration-scaffold-upgrade'`.

- [x] Scenario: `An upgrade writes and stages the declaration when ise is effective and it is absent`
  - Design constraints:
    - Fixture: clean Git project at a committed checkpoint with `ise`
      effective and no declaration.
    - `givn upgrade` exits 0 and reports the managed revision; exactly one
      allowlist-only managed revision exists; the revision contains
      `givn/specification.yaml`; the declaration names the five roles and one
      `source` list; the report names the declaration as written.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "upgrade-a-project/apply-a-managed-upgrade.feature::An upgrade writes and stages the declaration when ise is effective and it is absent"

        RED: the managed revision does not contain givn/specification.yaml
             (the revision carried only generated-integration changes)
        GREEN: 1 scenario (1 passed); 7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 7 steps (7 passed)).
  - [x] COMMIT: `715deaf954e1d64a3d480cc8b8a442e0817df848` -- Batch: `declaration-scaffold-upgrade`

- [x] Scenario: `An upgrade preserves an edited declaration`
  - Design constraints:
    - Fixture: clean Git project with committed checkpoint, `ise` effective,
      and a project-edited declaration whose bytes are recorded.
    - The revision carries no diff for the path; bytes unchanged; the report
      names the declaration as project-owned. It never joins the staged set.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "upgrade-a-project/apply-a-managed-upgrade.feature::An upgrade preserves an edited declaration"

        RED: the report does not name givn/specification.yaml
             (stdout listed only attribution and integration lines)
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 6 steps (6 passed)).
  - [x] COMMIT: `715deaf954e1d64a3d480cc8b8a442e0817df848` -- Batch: `declaration-scaffold-upgrade`

- [x] Scenario: `An upgrade writes no declaration when ise is not effective`
  - Design constraints:
    - Fixture: clean Git project, `ise` disabled, leftover declaration whose
      bytes are recorded.
    - The revision contains no `givn/specification.yaml`; bytes unchanged; no
      read of the declaration.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "upgrade-a-project/apply-a-managed-upgrade.feature::An upgrade writes no declaration when ise is not effective"

        existing behavior protected: passed on its first run
        (1 scenario (1 passed); 5 steps (5 passed)). The upgrade already left
        the declaration untouched with `ise` off; no production edit was
        manufactured for it.
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 5 steps (5 passed)).
  - [x] COMMIT: `715deaf954e1d64a3d480cc8b8a442e0817df848` -- Batch: `declaration-scaffold-upgrade`

- [x] Scenario: `Repeating an upgrade after the declaration scaffold creates no second revision`
  - Design constraints:
    - Fixture: clean Git project with `ise` effective and an
      upgrade-written declaration (run the upgrade once).
    - The second `givn upgrade` prints
      `Managed Givn files are already up to date for <version> (no changes)`
      (`src/cli/upgrade.rs:72`), creates no second revision, and leaves the
      declaration bytes unchanged.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "upgrade-a-project/apply-a-managed-upgrade.feature::Repeating an upgrade after the declaration scaffold creates no second revision"

        RED: the first upgrade writes the declaration: Os { code: 2,
             kind: NotFound, message: "No such file or directory" }
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 5 steps (5 passed)).
  - [x] COMMIT: `715deaf954e1d64a3d480cc8b8a442e0817df848` -- Batch: `declaration-scaffold-upgrade`

- [x] Scenario: `A declaration scaffold write failure reports the path and the completed writes`
  - Design constraints:
    - Fixture: clean Git project, `ise` effective, and
      `givn/specification.yaml` is a directory.
    - Exit nonzero; the failure names the path; the report identifies the
      completed writes and the checkpoint; no complete rollback is promised.
    - `FileSnapshot` restores what it captured; the directory is not a captured
      file and is not removed.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs \
          cargo test --locked --test features_runner -- --exact \
          "upgrade-a-project/apply-a-managed-upgrade.feature::A declaration scaffold write failure reports the path and the completed writes"

        RED: upgrade unexpectedly succeeded
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran the scenario; no behavior change
        (1 scenario (1 passed), 5 steps (5 passed)).
  - [x] COMMIT: `715deaf954e1d64a3d480cc8b8a442e0817df848` -- Batch: `declaration-scaffold-upgrade`

---

### Batch 6: `specification-health`

**Files modified:** `Cargo.toml`/`Cargo.lock` (`cargo add toml`, latest stable
at implementation time; record the resolved version in this batch revision),
`src/cli/lint.rs` (health pass, `Glob`, `walk_project`,
`specification_findings`, `CargoPackage`, severity/level arms), `src/cli/lint.rs`
doc comment (exit precedence),
`tests/steps/specification_health_steps.rs`.
**Design constraints (batch-wide):**
- The pass runs only when `paths.is_empty() && change.is_none()` and
  `SpecificationDeclaration::is_effective(&manifest)`. It runs before the
  empty-feature early return; the load and its blocking error are already in
  place from `declaration-loader`, and a load error short-circuits (one
  blocking finding, no advisory).
- Kinds/severity/exit: `specification_declaration` blocking (1);
  `specification_path_missing`, `specification_role_duplicate`,
  `specification_feature_unretained`, `specification_e2e_boundary` advisory
  (0).
- `specification_path_missing` covers `behavior`, `structure`, `contracts`,
  `limits` only; `e2e` is reported by the boundary finding, once.
- Findings iterate declared non-empty lists only; an empty role list retains
  nothing.
- Prose labels: `DECL` for the blocking declaration finding, `SPEC` for the
  four advisory ones; the existing `[{level}] {severity}  {location}  {message}`
  shape on stderr is kept (`render_prose`), and `severity` renders `advisory`.
- JSON: `findings_json` carries `kind` and `severity`; a finding names the path
  or package.
- Walk exclusions: `.git`, `.hg`, `.svn`, `.fossil`, `target`, `givn/archive`,
  `givn/changes`. Matching is against project-relative paths; literal means
  existence; `*` within one segment; `?` one character; `**` zero or more
  segments. No glob crate: a bounded matcher plus one bounded walker.
- `e2e` resolution: union of matched paths; the nearest project-relative
  `Cargo.toml` walking up, bounded at the root; distinct manifests form the
  dependency set; path and workspace-inherited dependencies build the
  transitive graph; zero matches or zero manifests produces the boundary
  finding naming the role entry and the reason; a matched path with no manifest
  names the path and the reason `no package manifest`; a manifest directory
  under a `source:` root, or a transitive edge to one, is the edge finding.
  A malformed `Cargo.toml` skips the edge comparison for that package and emits
  no edge finding. No `cargo` invocation.
- `**` must match zero segments so a declared `source: ["crates/app/**"]` covers
  the package directory `crates/app`.
- The change-scoped path keeps its existing refusals and reads no declaration.
- Step files run `givn lint`/`givn lint --json` through `common::run_givn`.

**Batch verification:** the twelve single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: specification-health'`.

- [x] Scenario: `An absent declaration is reported by name and no advisory finding is computed`
  - Design constraints:
    - Fixture: `ise`-enabled project selecting opencode with
      `givn/specification.yaml` deleted.
    - Exit non-zero; stderr names the path and contains
      `checks were not performed`; no advisory finding.
    - Reported on a project with or without `.feature` files (load before the
      early return).
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::An absent declaration is reported by name and no advisory finding is computed"
        PASS: 1 scenario (1 passed); 5 steps (5 passed).
        Existing behavior protected: the absent-declaration blocking path landed
        in `declaration-loader`.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A declared path missing from the tree is reported as advisory`
  - Design constraints:
    - Fixture: declaration whose `limits` declares `docs/performance.md`;
      that path is absent.
    - Exit 0; stderr contains `advisory` and names `docs/performance.md`.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A declared path missing from the tree is reported as advisory"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `One path declared in two roles is reported as advisory`
  - Design constraints:
    - Fixture: `structure` and `behavior` both declare `givn/specs/**`.
    - Exit 0; stderr contains `advisory`, names `givn/specs/**`, and reports
      the duplicate once (one finding per duplicated string, not per role pair).
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::One path declared in two roles is reported as advisory"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A .feature file outside the behavior globs is reported as advisory`
  - Design constraints:
    - Fixture: `behavior` declares only `givn/specs/orders/**`; a feature file
      exists at `givn/specs/billing/invoice.feature`.
    - Exit 0; stderr contains `advisory` and names that file. The scan excludes
      `givn/changes` and `givn/archive`.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A .feature file outside the behavior globs is reported as advisory"
        RED: ✘ And stderr names "givn/specs/billing/invoice.feature"
             Step panicked. Captured output: stderr missing
             "givn/specs/billing/invoice.feature": [SPEC ] advisory
             givn/specification.yaml & givn/specs/orders/** ...
        GREEN: 1 scenario (1 passed); 6 steps (6 passed)
        Fixture repair during RED: the feature directory needs a valid
        `usecase.md`, or the project-wide use-case gate exits 1 before the
        finding under test is observable.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A declared e2e package missing from the tree is reported as advisory`
  - Design constraints:
    - Fixture: `e2e` declares `tests/e2e`; the path does not exist.
    - Exit 0; stderr contains `advisory`, names `tests/e2e`, and reports the
      missing package once (no second path-missing finding for `e2e`).
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A declared e2e package missing from the tree is reported as advisory"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 7 steps (7 passed)
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A declared e2e package with a dependency edge into source is reported as advisory`
  - Design constraints:
    - Fixture: `tests/e2e/Cargo.toml` (package `e2e`) with a path dependency
      on the source package at `crates/app`, and `source: ["crates/app/**"]`
      (the `**` matches zero segments, so the package directory is covered).
    - Exit 0; stderr contains `advisory`, names the e2e package and the edge
      into `source`.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A declared e2e package with a dependency edge into source is reported as advisory"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        Note: the first GREEN attempt still failed because `toml::Value`'s
        `FromStr` parses one value, not a document; `toml::from_str` is the
        document entry point.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A declared e2e package with a transitive dependency edge into source is reported as advisory`
  - Design constraints:
    - Fixture: `e2e` -> `mid` -> `app`, where `app` sits under a `source:` root.
    - The graph is built from parsed manifests (path and workspace-inherited
      dependencies), not from a command; exit 0 and the same advisory shape.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A declared e2e package with a transitive dependency edge into source is reported as advisory"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A declared e2e path with no package manifest is reported by the boundary finding`
  - Design constraints:
    - Fixture: the declared `e2e` path exists but has no `Cargo.toml` at or
      above it before the project root.
    - Exit 0; stderr contains `advisory`, names `tests/e2e` and the reason
      `no package manifest`.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A declared e2e path with no package manifest is reported by the boundary finding"
        RED: ✘ And stderr contains "advisory"
             Step panicked. Captured output: stderr missing "advisory":
        GREEN: 1 scenario (1 passed); 5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `A malformed package manifest skips the dependency-edge finding`
  - Design constraints:
    - Fixture: the e2e package manifest is invalid TOML.
    - Exit 0; stderr contains no dependency-edge finding (the malformed
      manifest skips the comparison; no other finding is invented for it).
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::A malformed package manifest skips the dependency-edge finding"
        PASS: 1 scenario (1 passed); 4 steps (4 passed).
        Existing behavior protected: a manifest that exists but does not parse
        is skipped for the edge comparison, and the step also rejects reporting
        it as `no package manifest`.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `The JSON path exposes declaration findings with advisory severity`
  - Design constraints:
    - Fixture: same missing `docs/performance.md` declaration.
    - `givn lint --json` exits 0; stdout parses as JSON and contains a finding
      with `severity: "advisory"` naming `docs/performance.md`; the finding
      carries `kind` and `severity`.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::The JSON path exposes declaration findings with advisory severity"
        PASS: 1 scenario (1 passed); 5 steps (5 passed).
        Existing behavior protected: the shared `findings_json` path carries the
        advisory finding and its `kind`/`severity`.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `Change-scoped lint does not read the declaration`
  - Design constraints:
    - Fixture: `ise`-enabled project with a malformed declaration and an
      active change with one clean delta feature
      (`common::write_fixture_delta(env, "add-declaration-check", <capability>, <scenario>)`).
    - Run `givn lint --change add-declaration-check`; exit 0 and no
      declaration finding. The health pass runs only in default scope.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::Change-scoped lint does not read the declaration"
        PASS: 1 scenario (1 passed); 5 steps (5 passed).
        Existing behavior protected: the health pass is gated on the default
        project scope, so `--change` keeps its refusals and reads nothing.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

- [x] Scenario: `An empty role list retains nothing and produces no finding`
  - Design constraints:
    - Fixture: `ise`-enabled project with the scaffolded declaration (six empty
      lists).
    - Exit 0 and no declaration finding: findings iterate declared non-empty
      lists only.
  - [x] Implement the scenario and its tests. Evidence:
        ```
        $ GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs cargo test --locked --test features_runner -- --exact "verify-change/check-specification-health.feature::An empty role list retains nothing and produces no finding"
        PASS: 1 scenario (1 passed); 4 steps (4 passed).
        Existing behavior protected: every finding is guarded by a declared
        non-empty list, so the empty scaffold is silent.
        ```
  - [x] REFACTOR: re-ran all twelve scenarios; no behavior change
        (12 scenarios (12 passed), 63 steps (63 passed)).
  - [x] COMMIT: `7efd0b2b4676c8b6216d548b3164dea25dc4f636` -- Batch: `specification-health`

---

### Batch 7: `ise-instruction-serving`

Realizes decision D2. **Files created:** `assets/instructions/ise.md`.
**Files modified:** `src/cli/instructions.rs`,
`tests/steps/serve_extended_rules_steps.rs`,
`tests/steps/instruction_integrity_steps.rs`.
**Design constraints (batch-wide):**
- `POLICY_INSTRUCTION_IDS` gains `ise`.
- `const ADDON_BOUND_INSTRUCTION_IDS: &[(&str, &str)] = &[("ise", "ise")];`
- In `run_artifact`, after the retired-id check and after `load_effective()`,
  before the artifact/policy lookup: if the id is addon-bound and the addon is
  effective-off, bail with
  ``instruction 'ise' requires the 'ise' addon; enable it with `givn addons enable ise` ``.
  No content is printed. Otherwise resolve the cascade as today.
- One resolved `content` feeds the text and JSON paths; JSON carries it in
  `instruction`. The text path may append `Skill to load now: ...` when
  `domain_modeling` is on; the equality step compares the JSON `instruction`
  with the served content, not that trailer.
- `assets/instructions/ise.md` states the five obligations with these literal
  labels: `Specification completeness`, `two-layer test boundary`,
  `persisted-state continuity`, `change strategy`, `Spec gap` classification.
  It uses `Component`, `Port`, contract, `Specification`, `Source code`, and
  states in one line that it may use them because `ise` requires
  `architecture_principles`.
- No new binary, no new command, no skill file (`ise` is not a skill addon).
- Step files run the binary and compare stdout/JSON; they never import
  `cli::instructions`.

**Batch verification:** the four single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: ise-instruction-serving'`.

- [ ] Scenario: `The ise instruction is served while ise is effective`
  - Design constraints:
    - Fixture: `ise`-enabled project; run `givn instructions ise` with no
      `--change` (policy id needs none).
    - Exit 0 and the served text names the five obligations.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "obtain-effective-guidance/serve-extended-rules.feature::The ise instruction is served while ise is effective" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `ise-instruction-serving`

- [ ] Scenario: `An ise instruction request with the addon off is refused by name`
  - Design constraints:
    - Fixture: initialized project with `ise` disabled.
    - Exit non-zero, stderr names `ise`, and no served instruction content is
      printed on stdout.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "obtain-effective-guidance/serve-extended-rules.feature::An ise instruction request with the addon off is refused by name" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `ise-instruction-serving`

- [ ] Scenario: `The machine-readable ise path carries the same content as the text path`
  - Design constraints:
    - Fixture: `ise`-enabled project. Run the text path and
      `givn instructions ise --json` on the same project.
    - Exit 0; JSON `instruction` equals the served text after removing the
      optional `Skill to load now: ...` trailer.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "obtain-effective-guidance/serve-extended-rules.feature::The machine-readable ise path carries the same content as the text path" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `ise-instruction-serving`

- [ ] Scenario: `The ise instruction carries the five rebuild obligations`
  - Design constraints:
    - Fixture: project with `ise` enabled and skills synchronized; request the
      `ise` instruction.
    - Assert the five obligation labels and the five vocabulary terms on the
      served text. Bind in `tests/steps/instruction_integrity_steps.rs`.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "fragments/instruction-integrity.feature::The ise instruction carries the five rebuild obligations" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `ise-instruction-serving`

---

### Batch 8: `guidance-amendments`

**Files modified:** `assets/instructions/design.md`,
`assets/instructions/design-review.md`, `assets/instructions/review.md`,
`assets/instructions/proposal.md`, `tests/steps/change_strategy_steps.rs`,
`tests/steps/spec_gap_steps.rs`, `tests/steps/instruction_integrity_steps.rs`.
(`tests/steps/template_conditional_rendering_steps.rs` already binds both
`match-guidance-to-enabled-addons` scenario bodies; no new binding is needed
for them.)
**Design constraints (batch-wide):**
- Every added block is wrapped in `{{#if addons.ise}}`; with `ise` off the
  served guidance is byte-identical to today. `ise` is in
  `KNOWN_ADDON_KEYS` from batch 1.
- `design.md`: the Decisions section requires exactly one `change strategy`
  entry as a `Decides D<n>` ledger entry naming `incremental modification`,
  `Constant Ise`, and a whole-product rebuild; a rebuilding change records the
  `Ise Test` mode (`Dry-Run Ise` or `Full replacement`) beside it; the entry
  names the affected consumers, the persisted state, and the rebuild boundary;
  no new mandatory design field. Prose writes `change strategy` in full, never
  a bare `strategy`.
- `design-review.md`: an unresolved or unrecorded change strategy is a finding;
  a rebuild with no recorded `Ise Test` mode is a finding.
- `proposal.md`: a corrective proposal names the `Spec gap` record and its one
  outcome before any fix; the record names the wrong artifact, not who found
  it; `specification defect` and `E2E test defect` carry the corpus or E2E
  suite edit; `implementation defect` returns to the builder with no corpus
  edit; `acceptable undocumented change` is recorded in the corpus.
- `review.md`: exactly one `Spec gap` outcome before any fix, the four
  outcomes, only `specification defect` is a gap in the Specification, an
  `E2E test defect` stops the trial and the trial restarts after the test is
  corrected (a Specification correction, not an exception to the freeze), and
  the neighboring-components question (components changed outside the replaced
  component, and the reason) is one question, never a gate.
- Step files run `givn instructions <id> --change <id>` and assert the served
  text; they never read the asset file.
- The two `match-guidance-to-enabled-addons` scenarios have the same bodies as
  the permanent scenarios they replace and their steps are already bound; they
  protect the existing conditional-guidance behavior. Their batch evidence is a
  passing run, not a manufactured production diff.

**Batch verification:** the seven single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: guidance-amendments'`.

- [ ] Scenario: `The design-review guidance keeps a rebuild without an Ise Test mode a finding`
  - Design constraints:
    - Fixture: `ise`-enabled project with skills synchronized and an active
      change; request `givn instructions design-review --change <id>`.
    - Exit 0; the served text keeps an unresolved change strategy a finding and
      a rebuild with no recorded `Ise Test` mode a finding.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/change-strategy.feature::The design-review guidance keeps a rebuild without an Ise Test mode a finding" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `The proposal guidance names the corrective route for each outcome`
  - Design constraints:
    - Fixture: `ise`-enabled project with an active change; request
      `givn instructions proposal --change <id>`.
    - The served text names the record and its outcome for a corrective change,
      routes `implementation defect` back to the builder with no corpus edit,
      carries the corpus or E2E suite edit for `specification defect` and
      `E2E test defect`, and records `acceptable undocumented change` in the
      corpus.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/spec-gap.feature::The proposal guidance names the corrective route for each outcome" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `A spec gap record names the artifact rather than its finder`
  - Design constraints:
    - Same fixture; the proposal guidance requires the record to name the wrong
      artifact, not who found it.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/spec-gap.feature::A spec gap record names the artifact rather than its finder" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `An E2E test defect stops the Ise Test until the test is corrected`
  - Design constraints:
    - Same fixture; request `givn instructions review --change <id>`.
    - The served text stops an Ise Test on `E2E test defect` and restarts it
      after the E2E test is corrected, and states the corrected test is a
      Specification correction, not an exception to the freeze.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/spec-gap.feature::An E2E test defect stops the Ise Test until the test is corrected" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `The review guidance asks the neighboring-components question without a gate`
  - Design constraints:
    - Same fixture; the served review guidance asks for the neighboring
      components changed outside the replaced component and the reason, and
      states the answer is a review question, never a gate.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "fragments/instruction-integrity.feature::The review guidance asks the neighboring-components question without a gate" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `Architecture principles guidance follows an enabled addon`
  - Design constraints:
    - Fixture: `givn` project with `architecture_principles` disabled and
      skills synchronized; enable it through the Givn CLI.
    - The served design instruction contains the Contracts and components
      section; the served design-review instruction contains the Contracts and
      components branch. Steps already bound in
      `template_conditional_rendering_steps.rs`; this protects the existing
      conditional-guidance behavior while the title stops calling any enabled
      addon "opt-in".
  - [ ] Implement the scenario and its tests (expected to pass immediately;
        record it as existing behavior protected). Evidence:
        ```
        <!-- paste: --exact
             "fragments/match-guidance-to-enabled-addons.feature::Architecture principles guidance follows an enabled addon" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

- [ ] Scenario: `Outline guidance follows an enabled addon`
  - Design constraints:
    - Fixture: `givn` project with the `outline` addon `enabled`/`disabled`
      and skills synchronized (Outline, both Examples).
    - Served steps and review instructions contain/omit the advisory outline
      section as the Example says; no generated skill carries outline guidance.
      Steps already bound; existing behavior protected with the corrected
      posture wording.
  - [ ] Implement the scenario and its tests (expected to pass immediately;
        record it as existing behavior protected). Evidence:
        ```
        <!-- paste: --exact
             "fragments/match-guidance-to-enabled-addons.feature::Outline guidance follows an enabled addon" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments`

---

### Batch 9: `no-addons-and-lock`

**Files modified:** `tests/steps/enable_optional_addons_ise_steps.rs`.
(`src/cli/addons.rs::disable_all` and `config::lock::differences` come from
batch 1; this batch proves the two remaining declared requirements.)
**Design constraints (batch-wide):**
- `--no-addons` iterates `KNOWN_ADDONS` (ten) and writes an explicit `false`
  for each, `ise` included; the order is registry order with `ise` last:
  `arc42, communication_policy, readme, coverage, domain_modeling,
  dev_principles, architecture_principles, challenge, outline, ise`.
- A lock written before `ise` existed omits the key; `differences()` iterates
  the recorded keys and the lock check therefore reports no drift.
- Step files run the binary; the lock fixture writes
  `givn/changes/add-declaration-check/config-lock.yaml` with the nine
  default-on addons and `overrides: []`.

**Batch verification:** the two single-scenario commands below.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: no-addons-and-lock'`.

- [ ] Scenario: `givn init --no-addons writes an explicit false for every addon including ise`
  - Design constraints:
    - Fixture: temp root without `givn/`; run
      `givn init --no-addons --agents opencode`.
    - Exit 0; the active addon settings disable exactly the ten names above;
      `givn/specification.yaml` does not exist. Mandatory dialog bootstrap is
      unaffected.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "initialize-project/enable-optional-addons.feature::givn init --no-addons writes an explicit false for every addon including ise" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `no-addons-and-lock`

- [ ] Scenario: `A change locked before ise existed accepts the enabled addon`
  - Design constraints:
    - Fixture: `ise`-enabled project; active change
      `add-declaration-check` (`common::write_fixture_delta`) whose
      `config-lock.yaml` lists the nine default-on addons and omits `ise`.
    - Run `givn check review --change add-declaration-check`; assert stderr
      does not contain `add-on configuration changed during change`. Do not
      assert the exit code: unrelated review checks may fail on the fixture.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "initialize-project/enable-optional-addons.feature::A change locked before ise existed accepts the enabled addon" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `no-addons-and-lock`

---

## E2E batches

E2E setup (once, before the first `@e2e` batch):

- [ ] Confirm the e2e infrastructure from design.md "E2E smoke-test
      infrastructure": no server, database, container, browser, or digital twin.
      Each scenario builds a `tempfile::TempDir`, writes a minimal Givn project,
      and runs `env!("CARGO_BIN_EXE_givn")`; Git-backed scenarios initialize a
      repository. `verify.e2e_command` is `./run-tests.sh --e2e`, and
      `GIVN_RESULT_SCOPE=e2e` supplies the explicit scope under the archive
      result contract. Both executions share `features_runner` but select
      disjoint scopes with their own tag expression (`@e2e and not
      @givn.removed` vs `not @e2e`).
      ```
      <!-- paste: GIVN_FEATURES=… ./run-tests.sh --e2e summary,
           and the delta count proof: @e2e 6 < full 51 -->
      ```
- [ ] E2E strict-mode proof: the `.fail_on_skipped()` builder is the same one
      proven in `Setup`. Record the probe evidence there; do not repeat the
      probe.
- [ ] COMMIT: `<hash>` -- Batch: `e2e-setup`

### Batch E1: `addon-registry-e2e`

**Files modified:** `tests/steps/enable_optional_addons_ise_e2e_steps.rs`,
`tests/steps/initialize_a_project_e2e_steps.rs`, plus the shared posture steps
bound once in `enable_optional_addons_ise_steps.rs`.
**Design constraints:** real `givn` subprocesses; a transcript per scenario at
`givn/changes/add-ise-addon-and-specification/evidence/visual/<scenario-slug>/transcript.txt`
(`Interface: terminal`, no viewports); animations/fonts do not apply, but the
capture is deterministic (fixed args, temp project). Steps capture the
transcript through the existing terminal-instruction step.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: addon-registry-e2e'`.

- [ ] Scenario: `Consumer selects agent targets during interactive init`
  - Design constraints:
    - Real `givn init` subprocess driven through the PTY selection prompt;
      choose `opencode, claude`; assert the labeled selection, exit 0,
      `opencode, claude` in `givn/config.yaml`, both skills present,
      `.agents/skills/givn-propose` absent, the active addon settings enable
      exactly the nine default-on addons, do not name `ise`, and
      `givn/specification.yaml` absent.
    - This is the `@givn.modified` scenario: it keeps its existing steps and
      gains the posture assertions plus the transcript. It is expected to pass
      once the shared posture steps exist.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "initialize-project/initialize-a-project.feature::Consumer selects agent targets during interactive init" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `addon-registry-e2e`

- [ ] Scenario: `A consumer lists addons and each declared default is reported`
  - Design constraints:
    - Real `givn init --agents opencode` then `givn addons list`; assert exit 0,
      stdout contains `[disabled]  ise`, `[enabled ]  outline`, and
      `each omitted key resolves that addon's declared default`; capture the
      transcript.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "initialize-project/enable-optional-addons.feature::A consumer lists addons and each declared default is reported" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `addon-registry-e2e`

### Batch E2: `addon-prerequisites-e2e`

**Files modified:** `tests/steps/enable_optional_addons_ise_e2e_steps.rs`.
**Design constraints:** real binary; the reverse refusal is asserted on an
`ise`-enabled temp project; stderr carries the exact string; both flags stay
enabled; transcript captured.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: addon-prerequisites-e2e'`.

- [ ] Scenario: `A consumer disables a required addon and the refusal names ise`
  - Design constraints:
    - Fixture: `ise`-enabled project selecting opencode.
    - `givn addons disable architecture_principles` exits non-zero; stderr
      contains
      `addon 'architecture_principles' is required by 'ise'; disable 'ise' first`;
      both addons stay enabled; transcript captured.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "initialize-project/enable-optional-addons.feature::A consumer disables a required addon and the refusal names ise" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `addon-prerequisites-e2e`

### Batch E3: `specification-health-e2e`

**Files modified:** `tests/steps/specification_declaration_e2e_steps.rs`.
**Design constraints:** real `givn lint` subprocess on an `ise`-enabled temp
project with a declaration naming the five roles and two `source` roots; assert
exit 0, no declaration-schema error, transcript captured. The declaration's
paths must exist or the four advisory findings are allowed but must not be
schema errors.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: specification-health-e2e'`.

- [ ] Scenario: `The declared roles are read by the project-health pass`
  - Design constraints:
    - Fixture: `ise`-enabled project selecting opencode; write a declaration
      with `behavior`, `structure`, `contracts`, `limits`, `e2e` globs and two
      `source` roots.
    - Exit 0; stderr contains no declaration-schema error; transcript captured
      at `evidence/visual/the-declared-roles-are-read-by-the-project-health-pass/transcript.txt`.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "declare-the-specification/specification-declaration.feature::The declared roles are read by the project-health pass" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `specification-health-e2e`

### Batch E4: `guidance-amendments-e2e`

**Files modified:** `tests/steps/change_strategy_e2e_steps.rs`,
`tests/steps/spec_gap_e2e_steps.rs`.
**Design constraints:** real `givn instructions` subprocesses on an
`ise`-enabled project with synchronized skills and an active change; assert the
served text; transcript per scenario.
**Commit:** `givn commit <paths> -m '<release note>\n\nBatch: guidance-amendments-e2e'`.

- [ ] Scenario: `The design guidance requires one recorded change strategy`
  - Design constraints:
    - `givn instructions design --change <id>` exits 0; the served text
      requires exactly one change strategy as a `Decides D` entry, names
      `incremental modification`, `Constant Ise`, and a whole-product rebuild,
      and requires the affected consumers, the persisted state, and the rebuild
      boundary; transcript captured.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/change-strategy.feature::The design guidance requires one recorded change strategy" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments-e2e`

- [ ] Scenario: `The review guidance requires one spec gap outcome before any fix`
  - Design constraints:
    - `givn instructions review --change <id>` exits 0; the served text
      requires exactly one spec gap outcome before any fix, names the four
      outcomes, states that only `specification defect` is a gap in the
      Specification; transcript captured.
  - [ ] Implement the scenario and its tests. Evidence:
        ```
        <!-- paste: --exact
             "create-change/spec-gap.feature::The review guidance requires one spec gap outcome before any fix" -->
        ```
  - [ ] REFACTOR: re-run; no behavior change.
  - [ ] COMMIT: `<hash>` -- Batch: `guidance-amendments-e2e`

---

## Closeout

- [ ] Every delta phrase is bound in a permanent per-capability step file and
      no delta step copy exists under this change's `specs/`:
      `find givn/changes/add-ise-addon-and-specification/specs -name '*steps*' -print`
      prints nothing, and each new binding lives in the `tests/steps/` file
      named in design.md's Step Definitions table (the archive gate re-checks
      the same condition before mutation).
- [ ] No empty or no-op stub remains: `grep -rn 'unimplemented!\|todo!()' tests/steps/`
      prints nothing.
- [ ] No delta scenario still carries `@wip`
      (`givn check tasks --change add-ise-addon-and-specification` refuses
      otherwise).
- [ ] Whole-change verification, in this order:
      ```
      cargo fmt --all -- --check
      cargo clippy --workspace --all-targets -- -D warnings
      cargo nextest run
      ./run-tests.sh --non-e2e
      ./run-tests.sh --e2e
      GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs ./run-tests.sh --non-e2e
      GIVN_FEATURES=givn/changes/add-ise-addon-and-specification/specs ./run-tests.sh --e2e
      givn lint --change add-ise-addon-and-specification
      ```
      All exit 0 with zero failed and zero skipped. The e2e scope count is
      strictly less than the full delta count (6 `@e2e` of 51).
- [ ] `givn check tasks --change add-ise-addon-and-specification` exits 0 and
      records the change verification receipt.
- [ ] `givn check review --change add-ise-addon-and-specification` passes the
      scenario-commit, integrity, routing, and add-on lock checks.

**Done when:** every box is checked with its batch `COMMIT:` revision, both
`./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the full
permanent suite, the delta suite is green in both scopes, and the terminal
transcripts for the six `@e2e` scenarios are committed under
`givn/changes/add-ise-addon-and-specification/evidence/visual/`.
