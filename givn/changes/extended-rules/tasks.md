# Tasks: extended-rules

One section per executable scenario: RED→GREEN→REFACTOR→COMMIT.
Non-`@e2e` scenarios first, then the E2E setup task, then the `@e2e` scenarios.

Legend: an unchecked box means pending; a box is checked only when its evidence
and revision identifier are filled in. Check each box immediately after the
step — tasks.md must always reflect true state; never batch-check at the end.

**Release note (shared subject of every revision in this change, copied
verbatim from `proposal.md` §Release note):**

> Givn serves every step's rules through `givn instructions` — installed skills
> and commands direct agents there instead of carrying rule copies — and a
> project can extend any served instruction with its own rules, which
> synchronization and upgrades never fight.

**Delta inventory (verified against every delta `.feature` under
`givn/changes/extended-rules/specs/`):** 68 executable scenarios in 12
features = 25 `@givn.added` + 43 `@givn.modified` + 0 `@givn.removed`;
12 `@e2e` + 56 regular. Per feature (scenarios / added / modified / e2e):

| Feature | Scenarios | Added | Modified | E2E |
|---|---|---|---|---|
| `create-change/change-release-note.feature` | 1 | 0 | 1 | 1 |
| `create-change/prose-consolidation.feature` | 2 | 0 | 2 | 1 |
| `create-change/visual-design-contract.feature` | 2 | 0 | 2 | 1 |
| `extend-an-instruction/instruction-extension.feature` | 10 | 10 | 0 | 2 |
| `fragments/generate-ideation-guidance.feature` | 29 | 1 | 28 | 1 |
| `fragments/instruction-integrity.feature` | 4 | 2 | 2 | 0 |
| `fragments/match-guidance-to-enabled-addons.feature` | 5 | 0 | 5 | 3 |
| `fragments/refresh-generated-guidance.feature` | 2 | 1 | 1 | 1 |
| `initialize-project/initialize-a-project.feature` | 3 | 2 | 1 | 1 |
| `obtain-effective-guidance/serve-extended-rules.feature` | 6 | 6 | 0 | 1 |
| `upgrade-a-project/apply-a-managed-upgrade.feature` | 3 | 3 | 0 | 0 |
| `verify-change/visual-review-gate.feature` | 1 | 0 | 1 | 0 |
| **Total** | **68** | **25** | **43** | **12** |

The 12 `@e2e` titles (interaction-coverage-verified against `design.md`'s
Interaction Coverage Matrix, which matches the `## Interactions` tables of the
four owning use-case documents and of the `support` fragment document — this
change carries no separate `# User Interaction Inventory:` comments inside
`.feature` files):

1. `The generated implementation guidance shares the release note`
2. `Generated guidance points to the canonical E2E policy`
3. `Synchronized guidance defines the contract approval and capture mechanics`
4. `A developer records an extension and it governs the next guidance request`
5. `An upgrade preserves the extension and the next request serves it`
6. `A consumer synchronizes skills and every agent target receives the ideation facilitation skill`
7. `A consumer synchronizes skills and the generated guidance omits disabled-addon sections`
8. `A consumer enables the addons a generated section needs and the guidance gains it`
9. `A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed` (`Scenario Outline`)
10. `Synchronized guidance explains how to resolve the deterministic gates`
11. `Consumer runs init non-interactively and gets opencode skills`
12. `An agent receives the extended rules through the Givn CLI`

**Work order (recorded choice):** `design.md` names no implementation
sequencing section, so per the canonical tasks instruction this file follows
`.feature` order — features alphabetically by path (the delta-inventory order
above), scenarios in their in-file order. Two groups: the 56 non-`@e2e`
scenario sections, then the E2E setup task, then the 12 `@e2e` scenario
sections in the same feature order.

---

## Global constraints (apply to every task)

- **Runner**: `tests/features_runner.rs` (cucumber-rs 0.23.0 / gherkin 0.16,
  as pinned in `Cargo.lock`) is the single runner for `givn/specs/` and the
  change delta. Strict mode is `.fail_on_skipped()` plus the non-zero exit on
  `failed > 0 || skipped > 0`; never weaken it. `./run-tests.sh` runs it via
  `cargo nextest run --test features_runner` (fallback `GIVN_TEST_RUNNER=cargo`).
  `givn/commands.yaml` is the config home: `verify.command: ./run-tests.sh`,
  `verify.e2e_command: ./run-tests.sh --e2e` (confirm and record in the setup
  task; edit only if the actual values differ).
- **Scoped single-scenario command** (use for every RED/GREEN/REFACTOR run,
  exact form from `design.md` §Runner and strict mode, with this scenario's
  relative feature path and title):

  ```sh
  GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "<relative feature path>::<scenario title>"
  ```

  Never infer RED/GREEN from whole-suite output. Full-delta scope:
  `GIVN_FEATURES=givn/changes/extended-rules/specs ./run-tests.sh` (68
  scenarios), `... --non-e2e` (56), `... --e2e` (12).
- **Step-file ownership, one file per capability** (`design.md` §Step
  definitions and §Exact file changes): new files
  `tests/steps/serve_extended_rules_steps.rs` (extension recording/serving
  steps plus the shared served-instruction content step `the served <id>
  instruction contains/does not contain …`, beside the shared helper
  `served_instruction(env, id) -> String`) and
  `tests/steps/instruction_extension_steps.rs` (record/change/remove/reject
  steps). Modified capabilities keep their existing files:
  `tests/steps/prose_consolidation_steps.rs` (generic direction steps, the
  project-start-block step, updated generic content steps),
  `tests/steps/instruction_integrity_steps.rs` (id-resolution step, the
  no-duplication step), `tests/steps/template_conditional_rendering_steps.rs`
  (addon-condition steps read the served instruction),
  `tests/steps/init_agent_target_selection_steps.rs` (block reading + the
  full-copy fixture), `tests/steps/run_upgrade_steps.rs`,
  `tests/steps/visual_design_contract_steps.rs`,
  `tests/steps/visual_review_gate_steps.rs`; the ideation served-instruction
  step and every `the served <id> instruction contains/does not contain`
  assertion use the shared step in
  `tests/steps/serve_extended_rules_steps.rs` (design.md §Step definitions;
  the post-review correction removed the stale duplicate assignment from the
  §Exact file changes table). The visual step files add no content bindings
  and keep their existing eject and fixture steps. E2E files are
  created only in the E2E setup task.
- **Step bodies are never empty** — `{}`, a bare `pass`, a bare `return` are
  indistinguishable from PASS and banned at every phase. An unimplemented Rust
  step uses `unimplemented!()` (or `todo!()`); a step is exempt only once it
  has a real assertion (GREEN) or performs a real reused setup action. Reuse
  is never an excuse for a new step to be empty.
- **RED discipline**: remove `@wip` from THIS scenario only at RED, run the
  scoped command, paste the output, and it MUST exit non-zero — exit 0 means
  the runner is not strict or a step is empty; hard stop. Record the behavioral
  category honestly:
  - `OLD_PRODUCT_RED` — the current delivered guidance fails the new
    assertion (typical here: the requested instruction id does not exist yet
    and the CLI refuses it, or a generated file still carries a rule copy).
  - `ORACLE_RED` — the old contract already passes, or a coupled earlier
    scenario's edit already delivered it; prove discrimination with a
    controlled omission in an isolated fixture (temporary project override /
    temporary omission from the delivered excerpt), rerun the SAME assertion,
    then restore the fixture immediately. Never add a production switch,
    exception, fake success, or a source-text test for this.
  A missing binding, a build error, or an `unimplemented!()` panic alone is
  never the behavioral RED evidence for a scenario that has a behavioral
  assertion.
- **Production edit set** (nothing else; `design.md` §Exact file changes):
  `src/skills/render.rs` (expose `render_conditionals`; `render_template`
  delegates), `src/cli/instructions.rs` (render conditionals; append the
  extension on text and JSON paths), `src/config/extensions.rs` (new:
  `load` + `append`), `src/config/mod.rs` (register), `src/config/overlay.rs`
  (lenient `extensions` entry list, last-wins), `src/cli/project_hint.rs`
  (extract the marked resident-rules section; `{{RESIDENT_RULES}}`; fail
  closed on a missing marker), `assets/skills/*/SKILL.md.tmpl` (15),
  `assets/commands/*.md` (15), the new instruction assets (`archive`,
  `explore`, `ideation`, `steps`, `reverse-engineer`, `characterize`,
  `domain-modeling`, `dev-principles`, `grill-me`, `grill-with-docs`),
  `assets/instructions/review.md`, `design-review.md`, `tasks.md`,
  `tasks-reverse.md`, `assets/agents-project-hint.md`,
  `assets/config.yaml.tmpl`, the named step-definition files, and the arc42
  chapters 05/06/08/10/11 (`08-crosscutting-concepts.md`,
  `10-quality-requirements.md` per §Exact file changes). No new dependency, no
  render cache, no new command surface; `resolve_instruction` keeps its
  signature and order; the sync render path stays untouched beyond exposing
  `render_conditionals`; `src/cli/eject.rs` keeps materializing raw assets.
- **Commit discipline**: one atomic commit per scenario (RED+GREEN+REFACTOR
  together): `givn commit -m "feat(<capability>): <shared release note>"` for
  regular scenarios, `test(e2e): <shared release note>` for `@e2e` scenarios;
  the scenario title goes in the message body. Record the revision identifier
  immediately; never reuse one. A commit touching only the spec or a stub
  means the task is not done.
- **Personas are review lenses, never Gherkin actors**: the scenarios'
  actors (`the user`, `the agent`, `the developer`, `the consumer`) stay
  distinct from `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`.
- **Terminal transcript evidence (`@e2e` only)**: after the scenario's final
  assertion, commit one transcript at
  `givn/changes/extended-rules/evidence/visual/<scenario-slug>/transcript.txt`
  (`<scenario-slug>` = kebab-case scenario title, `givn::integrity_hook::scenario_slug`),
  recording the scenario title, the exact `argv`, the working-directory
  disposition, the exit code, and the raw stdout and stderr. Preserve raw
  output; never substitute file excerpts for stdout. Intermediate captures, if
  any, live under `reports/visual/` and are not review evidence.
- **Local runnability / digital twins** (`design.md` §Local runnability &
  digital twins): the system runs and tests locally with `./run-tests.sh`;
  no external or third-party service is touched, so no docker-compose stack,
  isolated network, or digital twin is required. Every changed action is a
  real CLI subprocess (or a real PTY for interactive init); no server, no
  network dependency, no interface obstacle anticipated.
- **Track progress rule**: check each box immediately after completing it —
  before starting the next scenario. Never batch-check boxes at the end.

## Domain constraints (from the owning use cases)

| Task group | Use-case rule or guarantee | Persona impact |
|---|---|---|
| `serve-extended-rules` (obtain-effective-guidance) | The served content is the extended rule: the built-in rules resolved from the canonical asset (or the project's replacement of the facet) plus the project's extension; an extension never overrides, suppresses, or reorders them; text and machine-readable paths carry identical content; a request never mutates project state; the unknown-id refusal, change-context rules, and machine contract shape are unchanged; built-in rules stay technology-agnostic. | `agent--technical-role` receives deterministic, complete content on both paths; `developer--primary-user` reads the same request manually. |
| `instruction-extension` (extend-an-instruction) | The extension lives in project-owned configuration; it never enters generated files, the managed revision, or the published clean-history surface; it is effective at the next request; a rejected entry warns and is skipped and nothing is mutated; every upgrade leaves it untouched; removing it restores the built-in rules alone; a textually contradicting rule appears beside the built-in rule (no semantic conflict detection). | `developer--primary-user` keeps rules that survive sync and upgrade; `givn-maintainer--owner` keeps one canonical asset per rule. |
| `initialize-a-project` (initialize-project) | Generated guidance carries trigger metadata, invocation orchestration, and the direction to the served instruction only; the project-start block renders its task-checkoff and commit-message obligations from the canonical source and keeps the dialog obligation under every option; reconciliation rewrites full copies to the direction, removes disabled-addon files, and preserves project-owned replacements byte-for-byte; selected targets and human-authored text are preserved. | `developer--primary-user` keeps target choices and human text; `agent--technical-role` reaches the effective guidance; `givn-maintainer--owner` maintains one source. |
| `apply-a-managed-upgrade` (upgrade-a-project) | The managed revision refreshes generated guidance to the direction and never carries the project's extensions; a project-owned replacement stays authoritative and byte-for-byte; improved built-in rules need no revision — the next request serves them. | `developer--primary-user` keeps extensions across upgrades; `givn-maintainer--owner` keeps the allowlist clean. |
| fragment capabilities `instruction-integrity`, `refresh-generated-guidance`, `match-guidance-to-enabled-addons`, `generate-ideation-guidance` (support fragment) | Resolution and the machine contract stay; every direction names an instruction id that resolves in the same or a later Givn version and is invokable in its documented context (an artifact direction names a resolvable change context; a policy or step direction names none); rule content leaves generated files; addon conditions select the served content; the verified-name gate holds without the addon. | `agent--technical-role` can always invoke the named call; `givn-maintainer--owner` grades one asset per rule. |
| `prose-consolidation`, `change-release-note`, `visual-design-contract` (create-change) | Content assertions move to the served instruction; the commit and release-note contract has exactly one home (the tasks instruction); the design, design-review, and steps rules move to the served instructions. | `developer--primary-user` sees no drift; `agent--technical-role` reads one source. |
| `visual-review-gate` (verify-change) | The review block moves to the served review instruction; a terminal change passes review only with a transcript. | `developer--primary-user` keeps review evidence; `givn-maintainer--owner` keeps the gate honest. |

---

## Strict-mode proof and runner setup (do first)

- [x] Setup: confirm the Gherkin runner and its strict mode, create the step-definition skeleton, and record `verify.command`
      Design constraints:
        - Runner: `tests/features_runner.rs` executes the `.feature` files for both
          scopes — the permanent `givn/specs/` (`./run-tests.sh`) and this change's
          delta (`GIVN_FEATURES=givn/changes/extended-rules/specs`; `design.md` §Runner
          and strict mode). A bare unit-test runner is never sufficient; do not
          replace it, and do not add a parallel hand-maintained test file.
        - Strict mode must be confirmed in code, not assumed: `.fail_on_skipped()`
          on the Cucumber builder in both the plain path (`main`) and the nextest
          exact path (`nextest_run`), and the non-zero exit on
          `failed > 0 || skipped > 0`. Paste the confirming lines; never weaken them.
        - Step-definition skeleton, ONE FILE PER CAPABILITY, exactly as `design.md`
          names: create `tests/steps/serve_extended_rules_steps.rs` and
          `tests/steps/instruction_extension_steps.rs` as module skeletons (step
          bindings are added scenario by scenario at RED) and register both in
          `tests/steps/mod.rs`; the existing per-capability files are extended as
          their scenarios come: `tests/steps/prose_consolidation_steps.rs`,
          `tests/steps/instruction_integrity_steps.rs`,
          `tests/steps/template_conditional_rendering_steps.rs`,
          `tests/steps/init_agent_target_selection_steps.rs`,
          `tests/steps/run_upgrade_steps.rs`,
          `tests/steps/visual_design_contract_steps.rs`,
          `tests/steps/visual_review_gate_steps.rs`. Never one shared step file.
          Do NOT create the E2E files before the E2E setup task.
        - Config home: in this project the verify commands live in
          `givn/commands.yaml` (not `givn/config.yaml`). Confirm and record the
          actual values; edit only if they differ from `verify.command:
          ./run-tests.sh` and `verify.e2e_command: ./run-tests.sh --e2e`.
        - Confirm delta discovery and the unit budget:
          `GIVN_FEATURES=givn/changes/extended-rules/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner`
          → 68 units; `GIVN_CUCUMBER_TAGS='not @e2e'` → 56; `GIVN_CUCUMBER_TAGS='@e2e'` → 12.
      PROOF OF STRICTNESS (mandatory — non-zero exit required; paste command +
      output for each step):
        1. Write a throwaway probe feature (one scenario, one step with no binding)
           at `givn/changes/extended-rules/probe/probe.feature` and run it scoped:
           `GIVN_FEATURES=givn/changes/extended-rules/probe cargo test --locked --test features_runner -- --exact "probe.feature::Probe with an undefined step"`
           → MUST exit non-zero with an undefined-step failure.
        2. Bind that step with an `unimplemented!()` stub and run the same command
           → MUST exit non-zero with the panic.
        3. Remove the probe file and the temporary binding immediately; re-run the
           discovery listing to confirm 68 units and no probe title.
      ```
      $ sed -n '55p;64,65p;151p' tests/features_runner.rs
        .fail_on_skipped()
    if stats.failed > 0 || stats.skipped > 0 {
        std::process::exit(1);
        .fail_on_skipped()
      $ rg -n "command:|e2e_command:" givn/commands.yaml
      2:  command: "./run-tests.sh"
      3:  e2e_command: "./run-tests.sh --e2e"

      Discovery (GIVN_FEATURES=givn/changes/extended-rules/specs):
      $ GIVN_CUCUMBER_TAGS=''        cargo nextest list --locked --test features_runner → 68 units
      $ GIVN_CUCUMBER_TAGS='not @e2e' cargo nextest list --locked --test features_runner → 56 units
      $ GIVN_CUCUMBER_TAGS='@e2e'      cargo nextest list --locked --test features_runner → 12 units

      Probe 1 — undefined step (non-zero required):
      $ GIVN_FEATURES=givn/changes/extended-rules/probe cargo test --locked --test features_runner -- --exact "probe.feature::Probe with an undefined step"
            Step doesn't match any function
        1 scenario (1 failed), 1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`        EXIT=101

      Probe 2 — `unimplemented!()` stub (non-zero required):
      $ (same command after binding the step with unimplemented!())
            Step panicked. Captured output: not implemented
        1 scenario (1 failed), 1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`        EXIT=101

      Probes removed; re-listing: 68 units, no probe title.
      Skeletons created: tests/steps/serve_extended_rules_steps.rs,
      tests/steps/instruction_extension_steps.rs; registered in
      tests/steps/mod.rs. No runner or config edit was needed.
      ```
      - [x] COMMIT: commit the runner/config edit only if it actually changed
            (expected: none — strict mode and the commands already exist) plus the
            new step-module skeletons and their `mod.rs` registration. Record the
            revision identifier:
            ```
            748a10f3  test(extended-rules): strict-mode proof and step skeletons
            ```

## Scenario tasks (non-@e2e, in .feature order)

- [x] prose-consolidation: The propose and archive commands carry the seed stamping instructions
      Design constraints:
        - Served-content seam: `givn instructions proposal --change <active>` reads `assets/instructions/proposal.md` and must contain "proposed by"; `givn instructions archive --change <active>` reads the new `assets/instructions/archive.md` and must contain "implemented in" — the seed-stamping instructions move into the served instruction assets.
        - Generated seam: `assets/commands/givn-propose.md` and `assets/commands/givn-archive.md` are rewritten to the direction (trigger metadata + invocation orchestration + the call), carrying no rule copy; the direction assertions use the generic direction steps in `tests/steps/prose_consolidation_steps.rs`.
        - Assertions: the served proposal instruction contains "proposed by"; the served archive instruction contains "implemented in"; the generated givn-propose command directs the author to the proposal instruction; the generated givn-archive command directs the archivist to the archive instruction.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/prose-consolidation.feature::The propose and archive commands carry the seed stamping instructions"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/prose-consolidation.feature::The propose and archive commands carry the seed stamping instructions"
            Feature: prose-consolidation
              Scenario: The propose and archive commands carry the seed stamping instructions
               ✔  Given a givn project with all agent skill targets enabled and an active change
               ✔  When the user runs `givn skills sync`
               ✔  Then the served proposal instruction contains "proposed by"
               ✘  And the served archive instruction contains "implemented in"
                  Step panicked. Captured output: givn instructions archive failed: error: unknown artifact 'archive'. Known artifacts: proposal, specs, design, design-review, tasks, review, arc42-docs
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            OLD_PRODUCT_RED: the `archive` instruction id does not exist yet; the CLI refuses it.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            assets/instructions/archive.md (new: archive rules — gate, seed stamping, guardrails)
            assets/commands/givn-archive.md (orchestration + direction to `givn instructions archive`)
            assets/commands/givn-propose.md (artifact-specific rules section removed; direction names `givn instructions proposal`)
            tests/steps/serve_extended_rules_steps.rs (served_instruction helper + contains/does-not-contain steps)
            tests/steps/prose_consolidation_steps.rs (given with active change; direction step)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after the production edits)
            Feature: prose-consolidation
              Scenario: The propose and archive commands carry the seed stamping instructions
               ✔  Given a givn project with all agent skill targets enabled and an active change
               ✔  When the user runs `givn skills sync`
               ✔  Then the served proposal instruction contains "proposed by"
               ✔  And the served archive instruction contains "implemented in"
               ✔  And the generated givn-propose command directs the author to the proposal instruction
               ✔  And the generated givn-archive command directs the archivist to the archive instruction
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 13.31s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 6 steps (6 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(prose-consolidation): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            1eb811fc — feat(prose-consolidation): <release note>; body: Scenario: The propose and archive commands carry the seed stamping instructions
            ```

- [x] visual-design-contract: Synchronized authoring guidance carries the classification, driver, and persona rules
      Design constraints:
        - Served-content seams: `assets/instructions/design.md` carries "renders HTML, CSS, or JavaScript in a browser" and "default browser driver"; `assets/instructions/design-review.md` carries "opposite classification"; `assets/instructions/proposal.md` carries "operator confirmation" — requested with the artifact change context.
        - Real `givn skills sync` first; every content assertion reads the served instruction, never a generated copy; the generated files only carry the direction.
        - Step files: `tests/steps/prose_consolidation_steps.rs` (shared served-content and direction steps) and `tests/steps/visual_design_contract_steps.rs` (capability-specific steps) per design.md §Step definitions.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/visual-design-contract.feature::Synchronized authoring guidance carries the classification, driver, and persona rules"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/visual-design-contract.feature::Synchronized authoring guidance carries the classification, driver, and persona rules"
            Feature: Visual Design Contract
              Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
               ✔  Given a givn project with all agent skill targets enabled and an active change
               ✔  When the user runs `givn skills sync`
               ✔  Then the served design instruction contains "renders HTML, CSS, or JavaScript in a browser"
               ✔  And the served design instruction contains "default browser driver"
               ✔  And the served design-review instruction contains "opposite classification"
               ✔  And the served proposal instruction contains "operator confirmation"
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            BASELINE_GREEN — assertion migration: every step binding already exists
            (the shared `Given a givn project with all agent skill targets enabled and
            an active change` from the prose-consolidation task, the generic sync When,
            and the shared served-instruction content steps), and the served design,
            design-review, and proposal instructions already carry the rules the old
            scenario read from the generated copies. This is the recorded
            ORACLE_RED case: the old contract already passes.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "opposite classification" → "opposite categorization" in assets/instructions/design-review.md; same scoped command)
            Feature: Visual Design Contract
              Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
               ✔  Given a givn project with all agent skill targets enabled and an active change
               ✔  When the user runs `givn skills sync`
               ✔  Then the served design instruction contains "renders HTML, CSS, or JavaScript in a browser"
               ✔  And the served design instruction contains "default browser driver"
               ✘  And the served design-review instruction contains "opposite classification"
                  Step panicked. Captured output: the served design-review instruction is missing "opposite classification"
            [Summary]
            1 feature
            1 scenario (1 failed)
            5 steps (4 passed, 1 failed)
            1 step failed                                                     EXIT=101
            The omission makes exactly the matching assertion fail, so the assertion
            discriminates; the excerpt was restored immediately (`git checkout --
            assets/instructions/design-review.md`) and the scoped command passed
            again. No production switch, exception, fake success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — assertion migration only. The delta scenario re-points the
            permanent scenario's generated-copy assertions at the served
            instructions (`assets/instructions/design.md`, `design-review.md`,
            `proposal.md`), which already carry the classification, driver, and
            persona rules; the generated-file direction rewrites are delivered by
            the scenarios whose own assertions name those directions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Visual Design Contract
              Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
               ✔  Given a givn project with all agent skill targets enabled and an active change
               ✔  When the user runs `givn skills sync`
               ✔  Then the served design instruction contains "renders HTML, CSS, or JavaScript in a browser"
               ✔  And the served design instruction contains "default browser driver"
               ✔  And the served design-review instruction contains "opposite classification"
               ✔  And the served proposal instruction contains "operator confirmation"
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 6 steps (6 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(visual-design-contract): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            dcb785cb — feat(visual-design-contract): <release note>; body: Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
            ```

- [x] instruction-extension: Changing an extension changes the next served content and touches nothing else
      Design constraints:
        - Extension source: the project-owned `extensions:` map in `givn/config.yaml`; `src/config/extensions.rs::load` reads it on every request (no render cache, nothing written); the change is a fixture edit, not a command.
        - Assertions: the served proposal content carries "Keep proposals under 300 words"; no installed guidance file carries the changed rule; no revision carries the changed rule — check the generated files and the repository history, not the config file.
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::Changing an extension changes the next served content and touches nothing else"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::Changing an extension changes the next served content and touches nothing else"
            Feature: A project extends an instruction's built-in rules
              Scenario: Changing an extension changes the next served content and touches nothing else
               ✔  Given a Givn project with an active change and an extension for the proposal instruction
               ✔  When the developer changes the project's extension for the proposal instruction to the rule "Keep proposals under 300 words"
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✘  Then the served content carries the changed rule
                  Step panicked. Captured output: the served proposal content carries no marked extension section: # Instruction: proposal
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            OLD_PRODUCT_RED: the product ignores the project's extension map
            entirely — `givn instructions proposal` succeeds but serves the
            built-in rules alone, so the changed rule is absent and no
            `## Project extension` section exists.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            src/config/extensions.rs (new: load — validate + warn per rejected entry;
              append — the marked ## Project extension section)
            src/config/overlay.rs (OverlayDoc gains the lenient `extensions` entry
              list: raw values in document order, repeated key last-wins, duplicate
              keys and malformed values never fail a command)
            src/config/mod.rs (register the new module)
            src/cli/instructions.rs (append the project's rules on the text and JSON
              serve paths, both the artifact and the non-artifact fallback)
            assets/config.yaml.tmpl (documents `extensions:` in the managed comment
              scaffold so new and upgraded projects see the key)
            tests/steps/serve_extended_rules_steps.rs (request/record fixtures plus
              the changed-rule assertion)
            tests/steps/instruction_extension_steps.rs (fixture, change, and
              no-file/no-revision assertions)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after the production edits)
            Feature: A project extends an instruction's built-in rules
              Scenario: Changing an extension changes the next served content and touches nothing else
               ✔  Given a Givn project with an active change and an extension for the proposal instruction
               ✔  When the developer changes the project's extension for the proposal instruction to the rule "Keep proposals under 300 words"
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then the served content carries the changed rule
               ✔  And no installed guidance file carries the changed rule
               ✔  And no revision carries the changed rule
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.96s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 6 steps (6 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            7c842b31 — feat(instruction-extension): <release note>; body: Scenario: Changing an extension changes the next served content and touches nothing else
            ```

- [x] instruction-extension: Removing an extension restores the built-in rules alone
      Design constraints:
        - The extension never entered installed files or the managed revision, so removal needs no cleanup: delete the fixture entry and request again.
        - Assertions: the served content is the built-in rules alone (no `## Project extension` section); the installed guidance is unchanged (byte-for-byte before/after removal).
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::Removing an extension restores the built-in rules alone"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::Removing an extension restores the built-in rules alone"
            Feature: A project extends an instruction's built-in rules
              Scenario: Removing an extension restores the built-in rules alone
               ✔  Given a Givn project with an active change and an extension for the proposal instruction
               ✔  When the developer removes the project's extension for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then the served content is the built-in rules alone
               ✔  And the installed guidance is unchanged
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — the removal semantics are delivered by
            the coupled earlier scenario's edit (the extension is read from the
            config per request, so deleting the entry restores the built-in rules
            with no cleanup). This is the recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission in the isolated fixture):
            $ (temporarily made the removal step a no-op so the recorded extension
               stays effective; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: Removing an extension restores the built-in rules alone
               ✔  Given a Givn project with an active change and an extension for the proposal instruction
               ✔  When the developer removes the project's extension for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✘  Then the served content is the built-in rules alone
                  Step panicked. Captured output: the served proposal content carries an extension section
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            With the removal omitted, the served content carries the section and
            the assertion fails, so it discriminates; the real removal step was
            restored immediately and the scoped command passed again. No
            production switch, exception, fake success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the mechanism was delivered by the previous scenario's serving
            path (src/config/extensions.rs reads the map on every request, so a
            removed entry simply stops being appended). This scenario is the
            removal negative control: no installed file or revision was ever
            touched, so removal needs no cleanup.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the real removal step)
            Feature: A project extends an instruction's built-in rules
              Scenario: Removing an extension restores the built-in rules alone
               ✔  Given a Givn project with an active change and an extension for the proposal instruction
               ✔  When the developer removes the project's extension for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then the served content is the built-in rules alone
               ✔  And the installed guidance is unchanged
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.33s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            720a41d7 — feat(instruction-extension): <release note>; body: Scenario: Removing an extension restores the built-in rules alone
            ```

- [x] instruction-extension: An extension for an unknown instruction is dropped with a warning
      Design constraints:
        - Validation runs when guidance is resolved: an instruction key that does not resolve through the request path warns `warning: ignoring extension for unknown instruction 'propozal'` and is skipped in config document order.
        - Assertions: the warning names "propozal"; the served content is the built-in rules alone; the request still succeeds; no project state changes (config byte-for-byte).
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An extension for an unknown instruction is dropped with a warning"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An extension for an unknown instruction is dropped with a warning"
            Feature: A project extends an instruction's built-in rules
              Scenario: An extension for an unknown instruction is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration names the unknown instruction "propozal"
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the unknown instruction "propozal"
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — the unknown-id validation is delivered by
            the coupled earlier scenario's edit (`extensions::load` resolves every
            id when guidance is requested and warns for the unresolvable one).
            ORACLE_RED calibration (controlled omission from the delivered output):
            $ (temporarily suppressed the unknown-instruction warning emission in
               src/config/extensions.rs; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: An extension for an unknown instruction is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration names the unknown instruction "propozal"
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✘  Then a warning names the unknown instruction "propozal"
                  Step panicked. Captured output: no warning names the unknown instruction in the request stderr: ""
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            With the warning omitted, the assertion fails on the same fixture, so
            it discriminates; the emission was restored immediately and the scoped
            command passed again. No production switch, exception, fake success,
            or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the validation was delivered by the previous scenarios' serving
            path (`src/config/extensions.rs::load` skips the unresolvable id with
            `warning: ignoring extension for unknown instruction '<id>'`); this
            scenario binds the warning assertion and the built-in-alone control.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the warning emission)
            Feature: A project extends an instruction's built-in rules
              Scenario: An extension for an unknown instruction is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration names the unknown instruction "propozal"
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the unknown instruction "propozal"
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.97s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            f7aaaa61 — feat(instruction-extension): <release note>; body: Scenario: An extension for an unknown instruction is dropped with a warning
            ```

- [x] instruction-extension: A malformed extension entry is dropped with a warning
      Design constraints:
        - A value that is not a list, or a list element that is not a string, warns `warning: ignoring malformed extension for '<id>': …` and is skipped; one warning line per rejected entry, in config document order.
        - Assertions: a warning names the malformed entry; the served content is the built-in rules alone; nothing is mutated.
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A malformed extension entry is dropped with a warning"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A malformed extension entry is dropped with a warning"
            Feature: A project extends an instruction's built-in rules
              Scenario: A malformed extension entry is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration carries a malformed extension entry for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the malformed entry
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — the malformed-entry validation is
            delivered by the coupled earlier scenario's edit (`extensions::load`
            warns and skips a value that is not a rule list).
            ORACLE_RED calibration (controlled omission from the delivered output):
            $ (temporarily suppressed the malformed-entry warning emission in
               src/config/extensions.rs; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: A malformed extension entry is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration carries a malformed extension entry for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✘  Then a warning names the malformed entry
                  Step panicked. Captured output: no malformed-extension warning in the request stderr: ""
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            With the warning omitted, the assertion fails on the same fixture, so
            it discriminates; the emission was restored immediately (the file is
            byte-identical to the committed state) and the scoped command passed
            again. No production switch, exception, fake success, or source-text
            test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the validation was delivered by the earlier scenarios' serving
            path (`src/config/extensions.rs::load` warns
            `warning: ignoring malformed extension for '<id>': expected a list of
            rule strings` and skips the entry); this scenario binds the warning
            assertion and the built-in-alone control.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the warning emission)
            Feature: A project extends an instruction's built-in rules
              Scenario: A malformed extension entry is dropped with a warning
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration carries a malformed extension entry for the proposal instruction
               ✔  And the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the malformed entry
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.91s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            0722f40c — feat(instruction-extension): <release note>; body: Scenario: A malformed extension entry is dropped with a warning
            ```

- [x] instruction-extension: An empty extension list is valid and adds nothing
      Design constraints:
        - An empty list — like a list whose entries were all skipped — is valid and adds nothing; it is not a rejection and produces no warning.
        - Assertions: the command exits 0; the served content carries the built-in proposal rules; the served content carries no extension section.
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An empty extension list is valid and adds nothing"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An empty extension list is valid and adds nothing"
            Feature: A project extends an instruction's built-in rules
              Scenario: An empty extension list is valid and adds nothing
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries an empty extension list for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then the command exits 0
               ✔  And the served content carries the built-in proposal rules
               ✔  And the served content carries no extension section
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — an empty list adds nothing by
            construction (`extensions::append` returns the content unchanged when
            no rule remains), delivered by the coupled earlier scenario's edit.
            ORACLE_RED calibration (controlled omission from the delivered output):
            $ (temporarily omitted the empty-list guard in
               src/config/extensions.rs::append so the marked section was appended
               even with no rules; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: An empty extension list is valid and adds nothing
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries an empty extension list for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then the command exits 0
               ✔  And the served content carries the built-in proposal rules
               ✘  And the served content carries no extension section
                  Step panicked. Captured output: the served proposal content carries an extension section
            [Summary]
            1 feature
            1 scenario (1 failed)
            6 steps (5 passed, 1 failed)                                  EXIT=101
            With the guard omitted, the section appears and exactly the last
            assertion fails, so it discriminates; the guard was restored
            immediately (the file is byte-identical to the committed state) and
            the scoped command passed again. No production switch, exception,
            fake success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the empty-list behavior was delivered by the earlier scenarios'
            serving path (`extensions::append` adds nothing when no rule remains);
            this scenario binds the exit-0, built-in-rules, and no-section
            controls.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the empty-list guard)
            Feature: A project extends an instruction's built-in rules
              Scenario: An empty extension list is valid and adds nothing
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries an empty extension list for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then the command exits 0
               ✔  And the served content carries the built-in proposal rules
               ✔  And the served content carries no extension section
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.85s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 6 steps (6 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            e6fff160 — feat(instruction-extension): <release note>; body: Scenario: An empty extension list is valid and adds nothing
            ```

- [x] instruction-extension: A whitespace-only rule entry is skipped with a warning
      Design constraints:
        - A rule that is empty or whitespace-only after trimming is skipped with the malformed warning naming the proposal instruction; every rejection is deterministic, one warning line per rejected entry.
        - Assertions: a warning names the malformed extension for the proposal instruction; the served content is the built-in rules alone.
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A whitespace-only rule entry is skipped with a warning"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A whitespace-only rule entry is skipped with a warning"
            Feature: A project extends an instruction's built-in rules
              Scenario: A whitespace-only rule entry is skipped with a warning
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries a whitespace-only rule for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the malformed extension for the proposal instruction
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — the trim-and-warn validation is delivered
            by the coupled earlier scenario's edit (`extensions::load` warns and
            skips an element that is empty after trimming).
            ORACLE_RED calibration (controlled omission from the delivered output):
            $ (temporarily suppressed the element-level malformed warning emission in
               src/config/extensions.rs; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: A whitespace-only rule entry is skipped with a warning
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries a whitespace-only rule for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✘  Then a warning names the malformed extension for the proposal instruction
                  Step panicked. Captured output: no malformed-extension warning in the request stderr: ""
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            With the warning omitted, the assertion fails on the same fixture, so
            it discriminates; the emission was restored immediately (the file is
            byte-identical to the committed state) and the scoped command passed
            again. No production switch, exception, fake success, or source-text
            test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the whitespace-only validation was delivered by the earlier
            scenarios' serving path (`extensions::load` trims every rule and warns
            `warning: ignoring malformed extension for '<id>': a rule must be a
            non-empty string` for an empty result); this scenario binds the warning
            assertion and the built-in-alone control.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the warning emission)
            Feature: A project extends an instruction's built-in rules
              Scenario: A whitespace-only rule entry is skipped with a warning
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries a whitespace-only rule for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then a warning names the malformed extension for the proposal instruction
               ✔  And the served content is the built-in rules alone
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.79s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            a5d744b9 — feat(instruction-extension): <release note>; body: Scenario: A whitespace-only rule entry is skipped with a warning
            ```

- [x] instruction-extension: A repeated instruction key keeps the last entry
      Design constraints:
        - `src/config/overlay.rs` reads `extensions` as an ordered entry list (a small private wrapper carrying raw `serde_yaml_ng::Value`s) instead of a duplicate-checked `Value` mapping, because the pinned parser rejects duplicate keys inside a `Value` mapping; a repeated instruction key keeps its last occurrence (recorded behavior).
        - Assertions: the served content carries the rule from the last entry; the served content does not carry the rule from the earlier entry.
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A repeated instruction key keeps the last entry"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A repeated instruction key keeps the last entry"
            Feature: A project extends an instruction's built-in rules
              Scenario: A repeated instruction key keeps the last entry
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries two extension entries for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then the served content carries the rule from the last entry
               ✔  And the served content does not carry the rule from the earlier entry
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            BASELINE_GREEN / ORACLE_RED — the lenient entry-list read (duplicate
            keys parse; the last occurrence replaces the earlier one) is delivered
            by the coupled earlier scenario's edit in `src/config/overlay.rs`.
            ORACLE_RED calibration (controlled omission from the delivered behavior):
            $ (temporarily reversed the wrapper to keep the FIRST occurrence instead
               of the last; same scoped command)
            Feature: A project extends an instruction's built-in rules
              Scenario: A repeated instruction key keeps the last entry
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries two extension entries for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✘  Then the served content carries the rule from the last entry
                  Step panicked. Captured output: the served proposal extension section is missing the last entry's rule "Keep proposals under 900 words"
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            With first-wins, exactly the last-entry assertion fails, so it
            discriminates; last-wins was restored immediately (the file is
            byte-identical to the committed state) and the scoped command passed
            again. No production switch, exception, fake success, or source-text
            test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the repeated-key behavior was delivered by the earlier
            scenarios' serving path (`src/config/overlay.rs::ExtensionEntries`
            replaces a repeated instruction key with its last occurrence; the
            duplicate keys never reach a `serde_yaml_ng::Value` mapping). This
            scenario binds the last-entry and earlier-entry-absent assertions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring last-wins)
            Feature: A project extends an instruction's built-in rules
              Scenario: A repeated instruction key keeps the last entry
               ✔  Given a Givn project with an active change
               ✔  And the project's configuration carries two extension entries for the proposal instruction
               ✔  When the agent requests the proposal instructions through the Givn CLI
               ✔  Then the served content carries the rule from the last entry
               ✔  And the served content does not carry the rule from the earlier entry
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.96s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            e8231e5f — feat(instruction-extension): <release note>; body: Scenario: A repeated instruction key keeps the last entry
            ```

- [x] instruction-extension: An extension extends a policy and a step instruction alike
      Design constraints:
        - The map's ids may name a policy instruction (`dialog`) and a step instruction (`ideation`); both resolve through the non-artifact fallback and require no change context.
        - Assertions: each served content carries its project's rule under the marked `## Project extension` section; the built-in rules stay unmodified (extension adds, never overrides, suppresses, or reorders).
        - Step file: `tests/steps/instruction_extension_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An extension extends a policy and a step instruction alike"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An extension extends a policy and a step instruction alike"
            Feature: A project extends an instruction's built-in rules
              Scenario: An extension extends a policy and a step instruction alike
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration records a rule for the dialog policy and for the ideation step
               ✔  And the agent requests the dialog policy through the Givn CLI
               ✔  And the agent requests the ideation instructions through the Givn CLI
               ✘  Then each served content carries its project's rule in the marked extension section
                  Step panicked. Captured output: the served ideation content carries no marked extension section:
            [Summary]
            1 feature
            1 scenario (1 failed)
            5 steps (4 passed, 1 failed)                                  EXIT=101
            OLD_PRODUCT_RED: the `ideation` step instruction does not exist yet,
            so `givn instructions ideation` is refused with the unknown-artifact
            error; the served ideation content is empty and carries no extension
            section. (The `dialog` policy request already serves its extension.)
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            assets/instructions/ideation.md (new canonical step-instruction asset:
              the minimum content that makes the id resolve through the non-artifact
              fallback and serve; the full ideation discipline migration is owned by
              the generate-ideation-guidance scenarios, whose content assertions
              bind each excerpt as it moves out of the givn-ideation skill template)
            tests/steps/serve_extended_rules_steps.rs (recorded-served-ids fixture
              plus the each-served-content assertion)
            tests/steps/instruction_extension_steps.rs (policy-and-step fixture step)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after creating assets/instructions/ideation.md)
            Feature: A project extends an instruction's built-in rules
              Scenario: An extension extends a policy and a step instruction alike
               ✔  Given a Givn project with an active change
               ✔  When the project's configuration records a rule for the dialog policy and for the ideation step
               ✔  And the agent requests the dialog policy through the Givn CLI
               ✔  And the agent requests the ideation instructions through the Givn CLI
               ✔  Then each served content carries its project's rule in the marked extension section
            [Summary]
            1 feature
            1 scenario (1 passed)
            5 steps (5 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.28s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 5 steps (5 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-extension): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            1844715d — feat(instruction-extension): <release note>; body: Scenario: An extension extends a policy and a step instruction alike
            ```

- [x] generate-ideation-guidance: The ideation skill ships without any addon enabled
      Design constraints:
        - Content seam: the new canonical `assets/instructions/ideation.md` renders through `givn instructions ideation` (step id, no change context) with no addon enabled — the unconditional discipline renders, the addon-gated fragments are omitted; shared served-instruction step per Global constraints §Step-file ownership.
        - Generated seam: `assets/skills/givn-ideation/SKILL.md.tmpl` becomes the direction; real `givn skills sync` in a project root without addons.
        - Assertions: the served ideation instruction contains "Frame"; the generated givn-ideation guidance directs the ideator to the ideation instruction.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation skill ships without any addon enabled"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation skill ships without any addon enabled"
            Feature: Generate ideation guidance
              Scenario: The ideation skill ships without any addon enabled
               ✔  Given a project root with givn/ initialised without addons
               ✔  When the user runs `givn skills sync`
               ✘  Then the served ideation instruction contains "Frame"
                  Step panicked. Captured output: the served ideation instruction is missing "Frame": # Instruction: ideation
                  ... (the minimal placeholder: intro plus the three Scope bullets)
            [Summary]
            1 feature
            1 scenario (1 failed)
            3 steps (2 passed, 1 failed)                                  EXIT=101
            OLD_PRODUCT_RED: the served instruction is still the minimal
            placeholder asset — it carries the scope bullets only, no "Frame"
            section — and the generated givn-ideation skill still carries the
            old discipline copy and names no `givn instructions ideation`
            direction.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            assets/instructions/ideation.md (rewritten: the whole ideation discipline
              moved out of the skill template — context and Frame, Personas, Topic
              state, the question/term/seed/use-case schemas, Handoff, One question at
              a time, Event Storming, Use Case Definition, Use Case Elaboration,
              Quality reviews, What you must NOT do — with the domain_modeling and
              arc42 gated fragments as `{{#if addons.<key>}}` blocks)
            assets/skills/givn-ideation/SKILL.md.tmpl (direction: the discipline is
              served by `givn instructions ideation`; rule copy removed, invocation
              orchestration kept)
            src/skills/render.rs (render_conditionals exposed; render_template delegates)
            src/cli/instructions.rs (render the resolved instruction with the
              project's enabled addons on the text and JSON paths, both the artifact
              and the non-artifact fallback, before appending the extension)
            tests/steps/prose_consolidation_steps.rs (generic direction step
              `the generated givn-<file> guidance directs …`)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after the production edits)
            Feature: Generate ideation guidance
              Scenario: The ideation skill ships without any addon enabled
               ✔  Given a project root with givn/ initialised without addons
               ✔  When the user runs `givn skills sync`
               ✔  Then the served ideation instruction contains "Frame"
               ✔  And the generated givn-ideation guidance directs the ideator to the ideation instruction
            [Summary]
            1 feature
            1 scenario (1 passed)
            4 steps (4 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.89s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 4 steps (4 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            2d389d11 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation skill ships without any addon enabled
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the phase ladder and the one-question rule
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the whole ideation discipline (moved out of the skill template); the dialog obligation stays a direction, so "givn instructions dialog" appears.
        - Assertions (served ideation instruction): contains "Event Storming", "Use Case Definition", "Use Case Elaboration", "Handoff", "one question at a time", "quality review", "givn instructions dialog", "In practice:", "Rejected:".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the phase ladder and the one-question rule"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after binding the shared `Given a Givn project`
               fixture and removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the phase ladder and the one-question rule
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "Event Storming"
               ✔  And the served ideation instruction contains "Use Case Definition"
               ✔  And the served ideation instruction contains "Use Case Elaboration"
               ✔  And the served ideation instruction contains "Handoff"
               ✔  And the served ideation instruction contains "one question at a time"
               ✔  And the served ideation instruction contains "quality review"
               ✔  And the served ideation instruction contains "givn instructions dialog"
               ✔  And the served ideation instruction contains "In practice:"
               ✔  And the served ideation instruction contains "Rejected:"
            [Summary]
            1 feature
            1 scenario (1 passed)
            11 steps (11 passed)                                         EXIT=0
            BASELINE_GREEN / ORACLE_RED — the previous scenario's slice moved
            the whole discipline into the served asset, so the new assertions
            already pass. This is the recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "Event Storming" → "Eventstorming" in
               assets/instructions/ideation.md; same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the phase ladder and the one-question rule
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✘  Then the served ideation instruction contains "Event Storming"
                  Step panicked. Captured output: the served ideation instruction is missing "Event Storming": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            3 steps (2 passed, 1 failed)                                  EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the content was delivered by the previous scenario's slice
            (`assets/instructions/ideation.md` carries the phase ladder, the One
            question at a time section, Quality reviews, the dialog direction, and
            the In practice/Rejected schemas). This scenario binds the served-content
            assertions and adds the shared `Given a Givn project` fixture
            (tests/steps/serve_extended_rules_steps.rs).
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the phase ladder and the one-question rule
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "Event Storming"
               ✔  And the served ideation instruction contains "Use Case Definition"
               ✔  And the served ideation instruction contains "Use Case Elaboration"
               ✔  And the served ideation instruction contains "Handoff"
               ✔  And the served ideation instruction contains "one question at a time"
               ✔  And the served ideation instruction contains "quality review"
               ✔  And the served ideation instruction contains "givn instructions dialog"
               ✔  And the served ideation instruction contains "In practice:"
               ✔  And the served ideation instruction contains "Rejected:"
            [Summary]
            1 feature
            1 scenario (1 passed)
            11 steps (11 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.21s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 11 steps (11 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            2aa7a2c4 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the phase ladder and the one-question rule
            ```

- [x] generate-ideation-guidance: The ideation guidance binds personas to the artifacts that affect them
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` keeps the persona-to-artifact binding rule.
        - Assertions (served ideation instruction): contains "stakeholder type", "persona segment", "persona promotion", "every artifact that affects".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance binds personas to the artifacts that affect them"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance binds personas to the artifacts that affect them
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "stakeholder type"
               ✔  And the served ideation instruction contains "persona segment"
               ✔  And the served ideation instruction contains "persona promotion"
               ✔  And the served ideation instruction contains "every artifact that affects"
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                           EXIT=0
            BASELINE_GREEN / ORACLE_RED — the whole discipline (Personas section
            included) was delivered by the first scenario's slice. This is the
            recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "stakeholder type" → "stakeholder group" in
               assets/instructions/ideation.md; same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance binds personas to the artifacts that affect them
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✘  Then the served ideation instruction contains "stakeholder type"
                  Step panicked. Captured output: the served ideation instruction is missing "stakeholder type": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            3 steps (2 passed, 1 failed)                                  EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the Personas section moved with the first scenario's slice
            (`assets/instructions/ideation.md`); this scenario binds the
            persona-to-artifact binding assertions on the served surface.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance binds personas to the artifacts that affect them
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "stakeholder type"
               ✔  And the served ideation instruction contains "persona segment"
               ✔  And the served ideation instruction contains "persona promotion"
               ✔  And the served ideation instruction contains "every artifact that affects"
            [Summary]
            1 feature
            1 scenario (1 passed)
            6 steps (6 passed)                                           EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 6 steps (6 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            87d1767e — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance binds personas to the artifacts that affect them
            ```

- [x] generate-ideation-guidance: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` keeps the topic-state persistence and handoff boundary rules.
        - Assertions (served ideation instruction): contains "givn/ideation/", "open question", "change seed", "session.md", "docs/ideation/", "Handoff never starts a change".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "givn/ideation/"
               ✔  And the served ideation instruction contains "open question"
               ✔  And the served ideation instruction contains "change seed"
               ✔  And the served ideation instruction contains "session.md"
               ✔  And the served ideation instruction contains "docs/ideation/"
               ✔  And the served ideation instruction contains "Handoff never starts a change"
            [Summary]
            1 feature
            1 scenario (1 passed)
            8 steps (8 passed)                                           EXIT=0
            BASELINE_GREEN / ORACLE_RED — the Topic state and Handoff sections
            were delivered by the first scenario's slice. This is the recorded
            ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "Handoff never starts a change" →
               "Handoff never begins a change" in assets/instructions/ideation.md;
               same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "givn/ideation/"
               ✔  And the served ideation instruction contains "open question"
               ✔  And the served ideation instruction contains "change seed"
               ✔  And the served ideation instruction contains "session.md"
               ✔  And the served ideation instruction contains "docs/ideation/"
               ✘  And the served ideation instruction contains "Handoff never starts a change"
                  Step panicked. Captured output: the served ideation instruction is missing "Handoff never starts a change": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            8 steps (7 passed, 1 failed)                                  EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the Topic state and Handoff sections moved with the first
            scenario's slice (`assets/instructions/ideation.md`); this scenario
            binds the persistence, parking, and handoff-boundary assertions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "givn/ideation/"
               ✔  And the served ideation instruction contains "open question"
               ✔  And the served ideation instruction contains "change seed"
               ✔  And the served ideation instruction contains "session.md"
               ✔  And the served ideation instruction contains "docs/ideation/"
               ✔  And the served ideation instruction contains "Handoff never starts a change"
            [Summary]
            1 feature
            1 scenario (1 passed)
            8 steps (8 passed)                                           EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 8 steps (8 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            25af5ee8 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
            ```

- [x] generate-ideation-guidance: The explore guidance points to ideation for structured domain work
      Design constraints:
        - Content seam: the new canonical `assets/instructions/explore.md` carries the explore discipline and the direction to ideation.
        - Assertion: the served explore instruction (step id, no change context) contains "givn-ideation".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The explore guidance points to ideation for structured domain work"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The explore guidance points to ideation for structured domain work"
            Feature: Generate ideation guidance
              Scenario: The explore guidance points to ideation for structured domain work
               ✔  Given a Givn project
               ✔  When the agent requests the explore instructions through the Givn CLI
               ✘  Then the served explore instruction contains "givn-ideation"
                  Step panicked. Captured output: givn instructions explore failed: error: unknown artifact 'explore'. Known artifacts: proposal, specs, design, design-review, tasks, review, arc42-docs
            [Summary]
            1 feature
            1 scenario (1 failed)
            2 steps (2 passed, 1 failed)                                  EXIT=101
            OLD_PRODUCT_RED: the `explore` instruction id does not exist yet;
            the CLI refuses it, so the served content cannot be asserted.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            assets/instructions/explore.md (new canonical step-instruction asset:
              the explore discipline moved out of the givn-explore skill — the
              stance, what you might do, the must-nots, the transitioning-out table,
              and the explicit `/givn-ideation` direction for structured domain work)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after creating assets/instructions/explore.md)
            Feature: Generate ideation guidance
              Scenario: The explore guidance points to ideation for structured domain work
               ✔  Given a Givn project
               ✔  When the agent requests the explore instructions through the Givn CLI
               ✔  Then the served explore instruction contains "givn-ideation"
            [Summary]
            1 feature
            1 scenario (1 passed)
            3 steps (3 passed)                                            EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 3 steps (3 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ec188787 — feat(generate-ideation-guidance): <release note>; body: Scenario: The explore guidance points to ideation for structured domain work
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the seed schema and its status values
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the change-seed schema and its status values.
        - Assertions (served ideation instruction): contains "change seed", "Status: ready", "proposed by", "implemented in", "superseded by", "withdrawn", "Accepted decisions", "Suggested start".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the seed schema and its status values"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the seed schema and its status values
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "change seed"
               ✔  And the served ideation instruction contains "Status: ready"
               ✔  And the served ideation instruction contains "proposed by"
               ✔  And the served ideation instruction contains "implemented in"
               ✔  And the served ideation instruction contains "superseded by"
               ✔  And the served ideation instruction contains "withdrawn"
               ✔  And the served ideation instruction contains "Accepted decisions"
               ✔  And the served ideation instruction contains "Suggested start"
            [Summary]
            1 feature
            1 scenario (1 passed)
            10 steps (10 passed)                                         EXIT=0
            BASELINE_GREEN / ORACLE_RED — the seed schema and status lines were
            delivered by the first scenario's slice (the Handoff section and the
            seed template moved into the served asset). This is the recorded
            ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "Status: ready" → "Status: pending" in
               assets/instructions/ideation.md; same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the seed schema and its status values
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "change seed"
               ✘  And the served ideation instruction contains "Status: ready"
                  Step panicked. Captured output: the served ideation instruction is missing "Status: ready": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            4 steps (3 passed, 1 failed)                                  EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the Handoff and seed-schema content moved with the first
            scenario's slice (`assets/instructions/ideation.md`); this scenario
            binds the schema-field and status-value assertions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the seed schema and its status values
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "change seed"
               ✔  And the served ideation instruction contains "Status: ready"
               ✔  And the served ideation instruction contains "proposed by"
               ✔  And the served ideation instruction contains "implemented in"
               ✔  And the served ideation instruction contains "superseded by"
               ✔  And the served ideation instruction contains "withdrawn"
               ✔  And the served ideation instruction contains "Accepted decisions"
               ✔  And the served ideation instruction contains "Suggested start"
            [Summary]
            1 feature
            1 scenario (1 passed)
            10 steps (10 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 10 steps (10 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            5ab8be8c — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the seed schema and its status values
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the term schema and the collision procedure
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the domain-term record schema and the collision-check procedure (including the direction to the domain-modeling skill).
        - Assertions (served ideation instruction): contains "domain-terms.md", "`candidate`, `confirmed`, or `rejected`", "Contrast", "First use", "definition and first use", "recorded collision check", "only `confirmed` terms", "never deleted", "givn-domain-modeling", "Collision check", "Grep the durable glossary", "without recorded commands is not a record".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the term schema and the collision procedure"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the term schema and the collision procedure
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "domain-terms.md"
               ✔  And the served ideation instruction contains "`candidate`, `confirmed`, or `rejected`"
               ✔  And the served ideation instruction contains "Contrast"
               ✔  And the served ideation instruction contains "First use"
               ✔  And the served ideation instruction contains "definition and first use"
               ✔  And the served ideation instruction contains "recorded collision check"
               ✔  And the served ideation instruction contains "only `confirmed` terms"
               ✔  And the served ideation instruction contains "never deleted"
               ✔  And the served ideation instruction contains "givn-domain-modeling"
               ✔  And the served ideation instruction contains "Collision check"
               ✔  And the served ideation instruction contains "Grep the durable glossary"
               ✔  And the served ideation instruction contains "without recorded commands is not a record"
            [Summary]
            1 feature
            1 scenario (1 passed)
            14 steps (14 passed)                                         EXIT=0
            BASELINE_GREEN / ORACLE_RED — the term schema, the search-before-coin
            procedure, and the addon-gated directions were delivered by the first
            scenario's slice (`assets/instructions/ideation.md`, rendered with the
            default project's enabled addons). This is the recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "Grep the durable glossary" →
               "Grep the glossary" in assets/instructions/ideation.md; same
               scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the term schema and the collision procedure
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "domain-terms.md"
               ✔  And the served ideation instruction contains "`candidate`, `confirmed`, or `rejected`"
               ✔  And the served ideation instruction contains "Contrast"
               ✔  And the served ideation instruction contains "First use"
               ✔  And the served ideation instruction contains "definition and first use"
               ✔  And the served ideation instruction contains "recorded collision check"
               ✔  And the served ideation instruction contains "only `confirmed` terms"
               ✔  And the served ideation instruction contains "never deleted"
               ✔  And the served ideation instruction contains "givn-domain-modeling"
               ✔  And the served ideation instruction contains "Collision check"
               ✘  And the served ideation instruction contains "Grep the durable glossary"
                  Step panicked. Captured output: the served ideation instruction is missing "Grep the durable glossary": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            13 steps (12 passed, 1 failed)                                EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the term schema and collision procedure moved with the first
            scenario's slice (`assets/instructions/ideation.md`), including the
            `addons.domain_modeling` and `addons.arc42` gated fragments; this
            scenario binds the schema, status, and collision-check assertions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the term schema and the collision procedure
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "domain-terms.md"
               ✔  And the served ideation instruction contains "`candidate`, `confirmed`, or `rejected`"
               ✔  And the served ideation instruction contains "Contrast"
               ✔  And the served ideation instruction contains "First use"
               ✔  And the served ideation instruction contains "definition and first use"
               ✔  And the served ideation instruction contains "recorded collision check"
               ✔  And the served ideation instruction contains "only `confirmed` terms"
               ✔  And the served ideation instruction contains "never deleted"
               ✔  And the served ideation instruction contains "givn-domain-modeling"
               ✔  And the served ideation instruction contains "Collision check"
               ✔  And the served ideation instruction contains "Grep the durable glossary"
               ✔  And the served ideation instruction contains "without recorded commands is not a record"
            [Summary]
            1 feature
            1 scenario (1 passed)
            14 steps (14 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 14 steps (14 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            2c14c968 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the term schema and the collision procedure
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the canonical definition and the eight scoping rules
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the canonical use-case definition and the eight scoping rules.
        - Assertions (served ideation instruction): contains "contract between one primary actor and the system", "One goal, one primary actor", "Goal-shaped, never component-shaped", "Level discipline", "promotion rule", "three or more distinct actor goals", "Placement test", "Mutation containment", "Extension discipline", "Precondition and flow consistency", "Implementation independence", "Relationship semantics".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the canonical definition and the eight scoping rules"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "contract between one primary actor and the system"
               ✔  And the served ideation instruction contains "One goal, one primary actor"
               ✔  And the served ideation instruction contains "Goal-shaped, never component-shaped"
               ✔  And the served ideation instruction contains "Level discipline"
               ✔  And the served ideation instruction contains "promotion rule"
               ✔  And the served ideation instruction contains "three or more distinct actor goals"
               ✔  And the served ideation instruction contains "Placement test"
               ✔  And the served ideation instruction contains "Mutation containment"
               ✔  And the served ideation instruction contains "Extension discipline"
               ✔  And the served ideation instruction contains "Precondition and flow consistency"
               ✔  And the served ideation instruction contains "Implementation independence"
               ✔  And the served ideation instruction contains "Relationship semantics"
            [Summary]
            1 feature
            1 scenario (1 passed)
            14 steps (14 passed)                                         EXIT=0
            BASELINE_GREEN / ORACLE_RED — the Use Case Definition section
            (canonical definition and the eight rules) moved with the first
            scenario's slice. This is the recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "Mutation containment" → "Mutation boundaries"
               in assets/instructions/ideation.md; same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "contract between one primary actor and the system"
               ✔  And the served ideation instruction contains "One goal, one primary actor"
               ✔  And the served ideation instruction contains "Goal-shaped, never component-shaped"
               ✔  And the served ideation instruction contains "Level discipline"
               ✔  And the served ideation instruction contains "promotion rule"
               ✔  And the served ideation instruction contains "three or more distinct actor goals"
               ✔  And the served ideation instruction contains "Placement test"
               ✘  And the served ideation instruction contains "Mutation containment"
                  Step panicked. Captured output: the served ideation instruction is missing "Mutation containment": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            10 steps (9 passed, 1 failed)                                 EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the Use Case Definition section moved with the first scenario's
            slice (`assets/instructions/ideation.md`); this scenario binds the
            canonical-definition and scoping-rule assertions.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "contract between one primary actor and the system"
               ✔  And the served ideation instruction contains "One goal, one primary actor"
               ✔  And the served ideation instruction contains "Goal-shaped, never component-shaped"
               ✔  And the served ideation instruction contains "Level discipline"
               ✔  And the served ideation instruction contains "promotion rule"
               ✔  And the served ideation instruction contains "three or more distinct actor goals"
               ✔  And the served ideation instruction contains "Placement test"
               ✔  And the served ideation instruction contains "Mutation containment"
               ✔  And the served ideation instruction contains "Extension discipline"
               ✔  And the served ideation instruction contains "Precondition and flow consistency"
               ✔  And the served ideation instruction contains "Implementation independence"
               ✔  And the served ideation instruction contains "Relationship semantics"
            [Summary]
            1 feature
            1 scenario (1 passed)
            14 steps (14 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 14 steps (14 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            8d1e182f — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the authoring requirements and the handoff mapping
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the use-case authoring requirements and the handoff mapping rules.
        - Assertions (served ideation instruction): contains "the first actor is the primary actor", "Supporting:", "protects <guarantee>", "never an extension", "never to a capability inside another goal".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the authoring requirements and the handoff mapping"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "the first actor is the primary actor"
               ✔  And the served ideation instruction contains "Supporting:"
               ✔  And the served ideation instruction contains "protects <guarantee>"
               ✔  And the served ideation instruction contains "never an extension"
               ✔  And the served ideation instruction contains "never to a capability inside another goal"
            [Summary]
            1 feature
            1 scenario (1 passed)
            7 steps (7 passed)                                          EXIT=0
            BASELINE_GREEN / ORACLE_RED — the Use Case Elaboration authoring
            requirements and the Handoff mapping line moved with the first
            scenario's slice. This is the recorded ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "the first actor is the primary actor" →
               "the first actor is the main actor" in assets/instructions/ideation.md;
               same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✘  Then the served ideation instruction contains "the first actor is the primary actor"
                  Step panicked. Captured output: the served ideation instruction is missing "the first actor is the primary actor": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            3 steps (2 passed, 1 failed)                                  EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the Use Case Elaboration authoring requirements and the Handoff
            mapping rule moved with the first scenario's slice
            (`assets/instructions/ideation.md`); this scenario binds those
            assertions on the served surface.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "the first actor is the primary actor"
               ✔  And the served ideation instruction contains "Supporting:"
               ✔  And the served ideation instruction contains "protects <guarantee>"
               ✔  And the served ideation instruction contains "never an extension"
               ✔  And the served ideation instruction contains "never to a capability inside another goal"
            [Summary]
            1 feature
            1 scenario (1 passed)
            7 steps (7 passed)                                          EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 7 steps (7 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            51132a38 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
            ```

- [x] generate-ideation-guidance: The ideation guidance carries the persona identity criteria and the reuse protocol
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the persona identity criteria and the reuse protocol.
        - Assertions (served ideation instruction): contains "authority boundary", "evaluation capability", "own vocabulary", "topic stance", "inventory", "Reuse:", "New:", "append-only", "one persona with three stance blocks", "record entries only".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the persona identity criteria and the reuse protocol"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ (same scoped command after removing `@wip` from this scenario only)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "authority boundary"
               ✔  And the served ideation instruction contains "evaluation capability"
               ✔  And the served ideation instruction contains "own vocabulary"
               ✔  And the served ideation instruction contains "topic stance"
               ✔  And the served ideation instruction contains "inventory"
               ✔  And the served ideation instruction contains "Reuse:"
               ✔  And the served ideation instruction contains "New:"
               ✔  And the served ideation instruction contains "append-only"
               ✔  And the served ideation instruction contains "one persona with three stance blocks"
               ✔  And the served ideation instruction contains "record entries only"
            [Summary]
            1 feature
            1 scenario (1 passed)
            12 steps (12 passed)                                         EXIT=0
            BASELINE_GREEN / ORACLE_RED — the Personas identity criteria and the
            reuse protocol (including the persona-file schema and the Reuse:/New:
            block) moved with the first scenario's slice. This is the recorded
            ORACLE_RED case.
            ORACLE_RED calibration (controlled omission from the delivered excerpt):
            $ (temporarily rewrote "one persona with three stance blocks" →
               "one persona with three stance parts" in
               assets/instructions/ideation.md; same scoped command)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "authority boundary"
               ✔  And the served ideation instruction contains "evaluation capability"
               ✔  And the served ideation instruction contains "own vocabulary"
               ✔  And the served ideation instruction contains "topic stance"
               ✔  And the served ideation instruction contains "inventory"
               ✔  And the served ideation instruction contains "Reuse:"
               ✔  And the served ideation instruction contains "New:"
               ✔  And the served ideation instruction contains "append-only"
               ✘  And the served ideation instruction contains "one persona with three stance blocks"
                  Step panicked. Captured output: the served ideation instruction is missing "one persona with three stance blocks": # Instruction: ideation
                  ...
            [Summary]
            1 feature
            1 scenario (1 failed)
            12 steps (11 passed, 1 failed)                                EXIT=101
            The omission makes exactly the matching assertion fail, so the
            assertion discriminates; the excerpt was restored immediately
            (`git checkout -- assets/instructions/ideation.md`) and the scoped
            command passed again. No production switch, exception, fake
            success, or source-text test.
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            none — the persona identity criteria and the reuse protocol moved with
            the first scenario's slice (`assets/instructions/ideation.md`); this
            scenario binds those assertions on the served surface.
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after restoring the excerpt)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "authority boundary"
               ✔  And the served ideation instruction contains "evaluation capability"
               ✔  And the served ideation instruction contains "own vocabulary"
               ✔  And the served ideation instruction contains "topic stance"
               ✔  And the served ideation instruction contains "inventory"
               ✔  And the served ideation instruction contains "Reuse:"
               ✔  And the served ideation instruction contains "New:"
               ✔  And the served ideation instruction contains "append-only"
               ✔  And the served ideation instruction contains "one persona with three stance blocks"
               ✔  And the served ideation instruction contains "record entries only"
            [Summary]
            1 feature
            1 scenario (1 passed)
            12 steps (12 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 12 steps (12 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            da3bd6d7 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
            ```

- [x] generate-ideation-guidance: The ideation guidance keeps the confirmed persona behaviors
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` keeps the confirmed persona behaviors (reuse/inventory/arbitration boundaries).
        - Assertions (served ideation instruction): contains "stakeholder type", "persona segment", "persona promotion", "every artifact that affects", "at most three", "never auto-promote", "confirms every persona", "arbitrates", "inventory", "match", "separate from Actors".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance keeps the confirmed persona behaviors"`
      - [x] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            $ GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance keeps the confirmed persona behaviors"
            Feature: Generate ideation guidance
              Scenario: The ideation guidance keeps the confirmed persona behaviors
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "stakeholder type"
               ✔  And the served ideation instruction contains "persona segment"
               ✔  And the served ideation instruction contains "persona promotion"
               ✔  And the served ideation instruction contains "every artifact that affects"
               ✔  And the served ideation instruction contains "at most three"
               ✔  And the served ideation instruction contains "never auto-promote"
               ✔  And the served ideation instruction contains "confirms every persona"
               ✔  And the served ideation instruction contains "arbitrates"
               ✔  And the served ideation instruction contains "inventory"
               ✔  And the served ideation instruction contains "match"
               ✘  And the served ideation instruction contains "separate from Actors"
                  Step panicked. Captured output: the served ideation instruction is missing "separate from Actors": # Instruction: ideation
                  ... (full served content elided)
            [Summary]
            1 feature
            1 scenario (1 failed)
            13 steps (12 passed, 1 failed)                                EXIT=101
            OLD_PRODUCT_RED: the served ideation instruction never carried the
            persona/actor separation rule — it still lived as a rule copy in
            `assets/commands/givn-ideation.md` ("Keep confirmed Persona IDs
            separate from Actors").
            ```
      - [x] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            assets/instructions/ideation.md (one Personas-section bullet:
            "Confirmed persona IDs stay separate from Actors: Actors appear in
            Gherkin; persona biographies remain human-reviewed Markdown under
            `givn/personas/`." — the rule copy moved out of the command seam
            into the served instruction)
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            $ (same scoped command after the production edit)
            Feature: Generate ideation guidance
              Scenario: The ideation guidance keeps the confirmed persona behaviors
               ✔  Given a Givn project
               ✔  When the agent requests the ideation instructions through the Givn CLI
               ✔  Then the served ideation instruction contains "stakeholder type"
               ✔  And the served ideation instruction contains "persona segment"
               ✔  And the served ideation instruction contains "persona promotion"
               ✔  And the served ideation instruction contains "every artifact that affects"
               ✔  And the served ideation instruction contains "at most three"
               ✔  And the served ideation instruction contains "never auto-promote"
               ✔  And the served ideation instruction contains "confirms every persona"
               ✔  And the served ideation instruction contains "arbitrates"
               ✔  And the served ideation instruction contains "inventory"
               ✔  And the served ideation instruction contains "match"
               ✔  And the served ideation instruction contains "separate from Actors"
            [Summary]
            1 feature
            1 scenario (1 passed)
            13 steps (13 passed)                                         EXIT=0
            ```
      - [x] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            $ cargo fmt && cargo clippy --locked --test features_runner -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s     (clean)
            $ (re-ran the scoped command) → 1 scenario (1 passed), 13 steps (13 passed)  EXIT=0
            ```
      - [x] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ad36aa18 — feat(generate-ideation-guidance): <release note>; body: Scenario: The ideation guidance keeps the confirmed persona behaviors
            ```

- [ ] generate-ideation-guidance: The ideation guidance carries the open-question qualifier rule
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the open-question qualifier rule.
        - Assertions (served ideation instruction): contains "(open: Q<N>)", "must not be presented as unconditional", "removes the qualifier".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the open-question qualifier rule"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance carries the live-question standard
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the live-question standard (no bare option lists).
        - Assertions (served ideation instruction): contains "A bare option list is a defect", "the decision it blocks", "context that forces it".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the live-question standard"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance stops questions at decided directions
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the decided-direction question-stop rule.
        - Assertions (served ideation instruction): contains "never asked again", "applied and reported", "phase report", "only when a new fork", "share one direction are one question".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance stops questions at decided directions"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The frame is confirmed once and later phase boundaries report
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the confirm-frame-once rule and the later-boundary reporting rule.
        - Assertions (served ideation instruction): contains "Confirm the Frame", "report what was recorded and proceed".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The frame is confirmed once and later phase boundaries report"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: Handoff cuts change seeds by deliverable guarantee
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the seed-cutting rule by deliverable guarantee and the seed path/suggested order.
        - Assertions (served ideation instruction): contains "deliverable guarantee", "true end to end", "can ship alone", "may cover several use cases", "one use case may yield several seeds", "confirmed rule untrue", "dependencies and a suggested start order", "changes/<change-id>.md".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::Handoff cuts change seeds by deliverable guarantee"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation command guardrails carry the same conduct rules
      Design constraints:
        - Content seam: `assets/commands/givn-ideation.md` keeps invocation orchestration and the direction only; the conduct rules are read from the served instruction.
        - Assertions (served ideation instruction): contains "deliverable guarantee", "never asked again".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation command guardrails carry the same conduct rules"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance requires a canvas with distinguishable element kinds
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the canvas requirement with distinguishable element kinds.
        - Assertions (served ideation instruction): contains "one or more mermaid diagrams", "show every event, command, aggregate, and actor, with the kinds", "(per-node kind markers or a legend)", "the actor that issues it and the aggregate it changes", "A text chain without a canvas is an incomplete event storming", "the canvas must render", "drawn as a rendered mermaid canvas", "aggregate (the consistency boundary the command changes)".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance requires a canvas with distinguishable element kinds"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance keeps aggregates to declared consistency boundaries
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the aggregate discipline (declared consistency boundaries only).
        - Assertions (served ideation instruction): contains "Aggregates are given, not minted", "a domain-noun root that receives commands and emits events under one", "confirmed before the canvas relies on it", "Reuse the aggregates the project already declares", "never an aggregate", "Name the kind".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance keeps aggregates to declared consistency boundaries"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The event storming review checks canvas coverage, legality, agreement, and rendering
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the event-storming review checks.
        - Assertions (served ideation instruction): contains "canvas that shows every event, command, aggregate, and actor with its kinds", "every aggregate a declared consistency boundary or a", "drawn as an aggregate is a defect", "the lines and the picture naming the", "a canvas that renders".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The event storming review checks canvas coverage, legality, agreement, and rendering"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation command guardrails require the canvas and legal aggregates
      Design constraints:
        - Content seam: `assets/commands/givn-ideation.md` orchestration references the canvas/aggregate checks now served by the instruction.
        - Assertions (served ideation instruction): contains "rendered mermaid canvas naming every event", "every aggregate is a declared", "never a record or a specification".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation command guardrails require the canvas and legal aggregates"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance binds every confirmed term to the user's verified name
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the verified-name binding for every confirmed term.
        - Assertion: the served ideation instruction contains "the user's verified name".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance binds every confirmed term to the user's verified name"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance asks the naming question as soon as a name fails
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the immediate-naming-question rule.
        - Assertions (served ideation instruction): contains "generic, overloaded", "cannot fill its genus slot", "ask the naming question immediately", "is collision-checked like any other".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance asks the naming question as soon as a name fails"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance asks the term check before a phase closes
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the phase-closing term check.
        - Assertions (served ideation instruction): contains "The user verifies every new core term before it is used", "Term check — <phase>", "Confirm all, or name the terms to revisit", "one-sentence description", "Confirm all makes them `confirmed`", "naming dialog, one at a time, before confirmation", "the other terms stand", "a replaced name becomes an anti-term".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance asks the term check before a phase closes"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The verified-name gate holds without the addon
      Design constraints:
        - Content seam: the verified-name gate and the phase term check must sit OUTSIDE every `{{#if addons.*}}` block in `assets/instructions/ideation.md` — the fixture initializes `givn/` without addons and still receives them.
        - Assertions (served ideation instruction): contains "the user's verified name", "Term check — <phase>".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The verified-name gate holds without the addon"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation command guardrails carry the verified-name rule
      Design constraints:
        - Content seam: `assets/commands/givn-ideation.md` orchestration points at the phase term check, which the served instruction now owns.
        - Assertions (served ideation instruction): contains "verifies every new core term", "phase term check", "naming dialog", "before the canvas, use cases, or seeds rely on it".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation command guardrails carry the verified-name rule"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The ideation guidance carries the persona schema
      Design constraints:
        - Content seam: `assets/instructions/ideation.md` carries the persona schema (including the conditional `domain_modeling`/`arc42` fragments moved from the skill template — this fixture has the addons enabled).
        - Assertions (served ideation instruction): contains "Stable core", "Authority boundary", "Evaluation capability", "Own vocabulary", "Success criterion", "Running record", "append-only", "Topic stances".
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance carries the persona schema"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: The generated ideation files direct the agent to the ideation instruction
      Design constraints:
        - Generated seam: both `assets/skills/givn-ideation/SKILL.md.tmpl` and `assets/commands/givn-ideation.md` become direction-only files; real `givn skills sync` with all targets enabled.
        - Assertions: the generated givn-ideation guidance directs the ideator to the ideation instruction; the generated givn-ideation command directs the ideator to the ideation instruction.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The generated ideation files direct the agent to the ideation instruction"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(generate-ideation-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] instruction-integrity: Retrieval guidance distinguishes build capabilities
      Design constraints:
        - Served-content seam: `givn instructions specs --change <active>` reads `assets/instructions/specs.md`; the deterministic spec toolchain wording stays, and engine-backed retrieval commands are never named.
        - Assertions: the returned guidance does not mention engine-backed retrieval commands; does not offer advisory or blocking retrieval paths; keeps the deterministic spec toolchain wording.
        - Step file: `tests/steps/instruction_integrity_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::Retrieval guidance distinguishes build capabilities"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-integrity): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] instruction-integrity: The specs guidance carries the capability placement test
      Design constraints:
        - Served-content seam: `assets/instructions/specs.md` carries the capability placement test from the canonical policy.
        - Assertions: the returned guidance carries the placement test; rejects a channel as an answer; requires goal-shaped capability names.
        - Step file: `tests/steps/instruction_integrity_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::The specs guidance carries the capability placement test"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-integrity): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] instruction-integrity: Every instruction id named by generated guidance resolves through the Givn CLI
      Design constraints:
        - Real `givn skills sync` with all targets enabled; every generated skill and command names a served instruction via the direction (`givn instructions <id>`); the id-resolution step lives in `tests/steps/instruction_integrity_steps.rs`.
        - Assertion: every named instruction id resolves through the Givn CLI in the same Givn version or a later one (design.md §inner decision: "instruction-id references are a graded reference class").
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::Every instruction id named by generated guidance resolves through the Givn CLI"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-integrity): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] instruction-integrity: Every generated direction is invokable in its documented context
      Design constraints:
        - Graded two-class rule: a direction to an artifact instruction carries a resolvable change context (the named `--change` or the project's single active change); a direction to a policy or step instruction carries no change context and resolves without one; a syntactically resolvable id that cannot be invoked in its documented context is a defect.
        - Fail-closed guard analogous to the Q-97 file-reference check; `resolve_instruction` keeps its signature and order; assertion runs through the real CLI per context, not through source text.
        - Step file: `tests/steps/instruction_integrity_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::Every generated direction is invokable in its documented context"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(instruction-integrity): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] match-guidance-to-enabled-addons: Review guidance omits the Arc42 section when the Arc42 addon is disabled
      Design constraints:
        - Condition seam: the Arc42 implementation conformance section moves out of the review skill/command templates into `assets/instructions/review.md` wrapped in `{{#if addons.arc42}}`; `src/skills/render.rs` gains `render_conditionals` (the existing `render_lines` semantics: fail-closed on an unknown addon key or an unbalanced conditional); `src/cli/instructions.rs` renders with the project's enabled addons before printing.
        - Assertions: the served review instruction omits the Arc42 implementation conformance section; the generated review skill and command carry no Arc42 section; the generated review skill directs the reviewer to the review instruction.
        - Step file: `tests/steps/template_conditional_rendering_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/match-guidance-to-enabled-addons.feature::Review guidance omits the Arc42 section when the Arc42 addon is disabled"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(match-guidance-to-enabled-addons): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] match-guidance-to-enabled-addons: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled
      Design constraints:
        - Condition seam: the Architecture documentation (arc42) branch moves into `assets/instructions/design-review.md` as a `{{#if addons.arc42}}` block; the generated files carry the direction only; the prose "N/A when disabled" clauses are absorbed by the conditional selection.
        - Assertions: the served design-review instruction omits the Architecture documentation (arc42) branch; the generated design-review skill and command carry none.
        - Step file: `tests/steps/template_conditional_rendering_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/match-guidance-to-enabled-addons.feature::Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(match-guidance-to-enabled-addons): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] refresh-generated-guidance: Generated guidance carries no copy of a built-in rule
      Design constraints:
        - Real `givn skills sync` with all targets enabled; every generated skill and command carries trigger metadata and a direction, and none copies the built-in rules of its instruction.
        - No-duplication step (design.md §Step definitions, lives in `tests/steps/instruction_integrity_steps.rs`): the file names `givn instructions <id>`, and NONE of the representative marker list appears — "Artifact-specific rules", "carries the release note as its message subject", "Overlap dispositions", "Black-Box-First", "the user's verified name", "VISUAL-REVIEW: PASS", "one question at a time"; the list samples rules owned by different instruction assets so a copy reappearing anywhere is caught.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/refresh-generated-guidance.feature::Generated guidance carries no copy of a built-in rule"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(refresh-generated-guidance): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] initialize-a-project: A full copy installed by an earlier version is reconciled to the direction
      Design constraints:
        - Fixture seam: a new fixture in `tests/steps/init_agent_target_selection_steps.rs` installs a full-copy `.opencode/skills/givn-propose/SKILL.md` as written by an earlier version (rule text, no direction).
        - Reconciliation is the existing overwrite: the sync render path (`src/skills/render.rs`, `src/skills/targets.rs::sync`) is untouched; the next real `givn skills sync` rewrites the managed file.
        - Assertions: exit 0; the file directs the author to the proposal instruction; the file carries no built-in rule text.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "initialize-project/initialize-a-project.feature::A full copy installed by an earlier version is reconciled to the direction"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(initialize-a-project): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] initialize-a-project: The project-start block renders its resident rules from the tasks instruction
      Design constraints:
        - Seams: `assets/instructions/tasks.md` carries the task-checkoff obligation and the atomic-commit obligation (backend-neutral attribution wording) between `<!-- givn:resident-rules:begin -->` and `<!-- givn:resident-rules:end -->`; `assets/agents-project-hint.md` replaces its two resident bullets with `{{RESIDENT_RULES}}`; `src/cli/project_hint.rs::apply` extracts the marked section from the same embedded `instructions/tasks.md` the request path serves and substitutes it.
        - Fail-closed: a missing marker is a shipped-asset defect — the call fails naming the asset instead of writing a block without the obligations.
        - Assertions: after real `givn skills sync`, the project-start block carries the resident rules section of the tasks instruction; the block requires the effective dialog policy.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "initialize-project/initialize-a-project.feature::The project-start block renders its resident rules from the tasks instruction"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(initialize-a-project): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: The built-in rules are served alone when no extension exists
      Design constraints:
        - Seams: `src/config/extensions.rs::load` yields empty for an absent `extensions:` key; `append` is a no-op without rules; `src/cli/instructions.rs` is the only caller of the serve path.
        - Assertions: `givn instructions specs --change <active>` serves the built-in specs rules; the served content carries no extension section.
        - Step file: `tests/steps/serve_extended_rules_steps.rs` (shared served-instruction step beside `served_instruction(env, id)`).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::The built-in rules are served alone when no extension exists"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(serve-extended-rules): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: The extension adds to the built-in rules without replacing or reordering them
      Design constraints:
        - Append seam: `src/cli/instructions.rs` renders the built-in rules, then appends the project's extension under the marked `## Project extension` section (heading is the acceptance marker; rule order follows config order).
        - Contradiction fixture: the extension records "Never write a release note" while the built-in release-note rule stays; both appear — rules are extended, never overridden, suppressed, or reordered, and no semantic conflict detection exists (R-79).
        - Assertions: every built-in rule of the proposal instruction remains in the served content; the built-in rules appear before the marked extension section; the contradicting project rule appears beside the built-in release-note rule.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::The extension adds to the built-in rules without replacing or reordering them"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(serve-extended-rules): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: The machine-readable path carries the same extended content as the text path
      Design constraints:
        - JSON contract (arc42 §8.3b): `id`, `generates`, `requires`, `skill_to_load` are unchanged; the `instruction` field carries the same rendered-and-extended text as the text path, extension included.
        - Assertions: the response identifies the artifact and its generated paths and prerequisites; the response carries the same instruction content as the text path; the response carries the project's rule in the marked extension section.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::The machine-readable path carries the same extended content as the text path"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(serve-extended-rules): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: The extension applies on top of a project-owned replacement
      Design constraints:
        - D4: the project's whole-facet replacement is the effective base and the marked extension section is appended to it; the replacement file itself is never modified or injected (R-74 unchanged); `src/config/cascade.rs::resolve_instruction` keeps its signature and order (project override, embedded, addon); `src/cli/eject.rs` keeps materializing raw assets.
        - Assertions: the served proposal content carries the replacement text; the served content carries the project's rule in the marked extension section after it.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::The extension applies on top of a project-owned replacement"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(serve-extended-rules): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: A guidance request changes nothing in the project
      Design constraints:
        - No render cache and no new command surface: `givn instructions` resolves, renders, appends, and prints on every call; the extension map is read from `givn/config.yaml` on every call; nothing is written.
        - Assertion: after requesting the instructions for every served step, a project snapshot shows no changed file (the request path must not mutate configuration, generated files, or state).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::A guidance request changes nothing in the project"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(serve-extended-rules): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] apply-a-managed-upgrade: An upgrade reconciles full-copy guidance to the direction
      Design constraints:
        - Existing managed-path synchronization renders the rewritten assets; ignored managed paths are force-staged only through the calculated allowlist; the fixture is a committed project root with a tracked full-copy `.opencode/skills/givn-propose/SKILL.md`.
        - Assertions: the command exits 0; the revision shows `.opencode/skills/givn-propose/SKILL.md` rewritten to the direction (the rules now arrive only through `givn instructions`).
        - Step file: `tests/steps/run_upgrade_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "upgrade-a-project/apply-a-managed-upgrade.feature::An upgrade reconciles full-copy guidance to the direction"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(apply-a-managed-upgrade): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] apply-a-managed-upgrade: An upgrade leaves the project's extensions untouched
      Design constraints:
        - The `extensions:` key is user-owned configuration outside the allowlist; the managed revision never stages, merges, or rewrites it.
        - Assertions: the command exits 0; the upgrade revision carries no extension content; the recorded extension is unchanged (byte-for-byte).
        - Step file: `tests/steps/run_upgrade_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "upgrade-a-project/apply-a-managed-upgrade.feature::An upgrade leaves the project's extensions untouched"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(apply-a-managed-upgrade): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] apply-a-managed-upgrade: A project-owned replacement stays byte-for-byte across an upgrade
      Design constraints:
        - The project's whole-facet replacement stays authoritative; the upgrade never modifies it or injects text into it (R-74 unchanged).
        - Assertions: the command exits 0; the project-owned replacement is byte-for-byte unchanged; the upgrade revision carries no injected text for the replacement.
        - Step file: `tests/steps/run_upgrade_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "upgrade-a-project/apply-a-managed-upgrade.feature::A project-owned replacement stays byte-for-byte across an upgrade"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(apply-a-managed-upgrade): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

- [ ] visual-review-gate: Synchronized review guidance carries the visual block
      Design constraints:
        - Served-content seam: `assets/instructions/review.md` carries the visual block; the generated givn-review file carries the direction only.
        - Assertions: the generated givn-review guidance directs the reviewer to the review instruction; the served review instruction contains "VISUAL-REVIEW: PASS", "confirmed Persona", "no pixel-level".
        - Step file: `tests/steps/visual_review_gate_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "verify-change/visual-review-gate.feature::Synchronized review guidance carries the visual block"`
      - [ ] RED: bind this scenario's steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only, run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions and the minimum production code. Production files created/modified:
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `feat(visual-review-gate): <shared release note>`, scenario title in the message body. Record the revision identifier:
            ```
            ```

---

## E2E setup (after all non-@e2e scenarios are GREEN)

- [ ] Setup: e2e step skeletons, e2e strict-mode proof, `verify.e2e_command` isolation proof, and local runnability
      Design constraints:
        - E2E step files, separate from the regular steps, per `design.md` §E2E
          smoke test infrastructure ("Step locations"): create
          `tests/steps/serve_extended_rules_e2e_steps.rs` and
          `tests/steps/instruction_extension_e2e_steps.rs` and register both in
          `tests/steps/mod.rs`; the modified features' e2e steps stay in their
          existing per-capability files (for example
          `tests/steps/prose_consolidation_e2e_steps.rs`,
          `tests/steps/template_conditional_rendering_e2e_steps.rs`,
          `tests/steps/initialize_a_project_e2e_steps.rs`) — never one shared
          e2e file.
        - Framework: `cucumber-rs` over real subprocess invocations
          (`CARGO_BIN_EXE_givn`); the CLI is the real interface, so no browser
          or HTTP driver applies.
        - `verify.e2e_command` is exactly `./run-tests.sh --e2e`, never
          byte-identical to `verify.command` (`./run-tests.sh`); the `@e2e`
          scoping uses the runner's own tag-filter mechanism
          (`GIVN_CUCUMBER_TAGS='@e2e'` under nextest, `--tags '@e2e'` under the
          cargo fallback) — never a second hand-maintained suite.
        - Local runnability (`design.md` §Local runnability & digital twins):
          no docker-compose, no isolated network, no digital twin — every
          changed action is a real CLI subprocess (or a real PTY for
          interactive init); the only local preparation is the script's
          existing `cargo build --locked --test ready_fixture`; nothing to
          start or stop (state this explicitly); a temporary project directory
          plus the compiled givn binary is the whole environment.
      PROOF OF E2E STRICTNESS (mandatory — non-zero exit required; paste command
      + output for each step):
        1. Write a throwaway `@e2e` probe feature (one scenario, one step with no
           binding) under `givn/changes/extended-rules/probe-e2e/` and run
           `GIVN_FEATURES=givn/changes/extended-rules/probe-e2e ./run-tests.sh --e2e`
           → MUST exit non-zero with an undefined-step failure.
        2. Bind that step with an `unimplemented!()` stub and rerun the same
           command → MUST exit non-zero with the panic.
        3. Remove the probe feature and the temporary binding immediately.
      PROOF OF ISOLATION (mandatory — paste both counts and both command
      strings):
        4. `GIVN_FEATURES=givn/changes/extended-rules/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner`
           → 68 units (the full delta).
        5. Same with `GIVN_CUCUMBER_TAGS='@e2e'` → 12 units; with
           `GIVN_CUCUMBER_TAGS='not @e2e'` → 56 units. The e2e count (12) MUST be
           strictly smaller than the full count (68) — the "all scenarios are
           @e2e" exception does not apply — and `verify.e2e_command`
           (`./run-tests.sh --e2e`) MUST differ from `verify.command`
           (`./run-tests.sh`); record both strings.
        6. Smoke the real boundary before writing any scenario steps: run the
           compiled binary once through the e2e harness path (for example
           `cargo run --locked -- instructions` in a temporary initialized
           project) → exits 0 and prints guidance, proving the e2e runner
           reaches the real CLI.
      ```
      ```
      - [ ] COMMIT: commit the two e2e step-module skeletons and their `mod.rs`
            registration; the e2e run commands are already the configured
            `givn/commands.yaml` values (edit only if the recorded values
            differed). Record the revision identifier:
            ```
            ```

---

## E2E Smoke Tests

- [ ] change-release-note: The generated implementation guidance shares the release note
      Design constraints:
        - Real `givn skills sync` with all agent targets enabled; the primary assertion is on the real sync output plus the served tasks instruction (`givn instructions tasks --change <active>`), never on asset source text.
        - Served-content seam: the commit and release-note contract has exactly one home — `assets/instructions/tasks.md` — and the block's obligations come from the same asset; the generated givn-steps, givn-implement, and givn-tasks files carry the direction only.
        - Assertions: exit 0; the generated givn-steps guidance directs the step writer to the steps instruction; the generated givn-implement guidance directs the implementer to the tasks instruction; the generated givn-tasks guidance directs the task author to the tasks instruction; the served tasks instruction contains "carries the release note as its message subject" and "the scenario title goes in the message body"; generated guidance does not contain "message references the scenario title"; the file "AGENTS.md" contains "carries the release note as its message subject".
        - Evidence: capture argv, cwd, exit, raw sync stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: existing per-capability file (`tests/steps/prose_consolidation_e2e_steps.rs`); direction steps per `tests/steps/prose_consolidation_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/change-release-note.feature::The generated implementation guidance shares the release note"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] prose-consolidation: Generated guidance points to the canonical E2E policy
      Design constraints:
        - Real `givn skills sync` with all agent targets and optional guidance enabled; the primary assertion is on the emitted files and the served instructions, not on asset source text.
        - Served-content seam: `assets/instructions/specs.md` carries the canonical interaction and E2E policy (including the `# User Interaction Inventory:` policy and the terminal-transcript rule); the emitted skills and commands direct to their instructions and keep the dialog-policy obligation.
        - Assertions: every emitted skill and command requires the effective dialog policy before its context asks a user-facing question; direct skill invocation has the same obligation as session-start invocation; reference skills do not narrow the obligation to one workflow phase; the generated givn-spec guidance directs the author to the specs instruction; the served specs instruction carries the canonical interaction and E2E policy; detailed dialog rules and worked examples remain in the effective dialog instructions rather than competing generated copies; the synchronization result reports the selected guidance successfully.
        - Evidence: capture argv, cwd, exit, raw sync stdout, path-labeled delivered excerpts, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/prose_consolidation_e2e_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/prose-consolidation.feature::Generated guidance points to the canonical E2E policy"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] visual-design-contract: Synchronized guidance defines the contract approval and capture mechanics
      Design constraints:
        - Real `givn skills sync` with all agent targets enabled; the primary assertion is on the served instructions, not on generated copies.
        - Served-content seam: `assets/instructions/design-review.md` carries the contract approval mechanics and `assets/instructions/steps.md` (new asset) carries the capture mechanics; the generated givn-design-review and givn-steps files carry the direction only.
        - Assertions: the generated givn-design-review guidance directs the reviewer to the design-review instruction; the served design-review instruction contains "Visual Design Contract", "propose", "user approves", "rendered options"; the generated givn-steps guidance directs the step writer to the steps instruction; the served steps instruction contains "evidence/visual", "final-state", "animations are frozen", "reports/".
        - Evidence: capture argv, cwd, exit, raw sync stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: existing per-capability files (`tests/steps/prose_consolidation_e2e_steps.rs` for the sync; `tests/steps/visual_design_contract_steps.rs` for the served-content steps).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "create-change/visual-design-contract.feature::Synchronized guidance defines the contract approval and capture mechanics"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] instruction-extension: A developer records an extension and it governs the next guidance request
      Design constraints:
        - The e2e action is configuration maintenance plus a real guidance request (design-review finding 2): the fixture writes the project's rule into the `extensions:` map of `givn/config.yaml` (no recording command exists — D2), then a real `givn instructions proposal --change <active>` subprocess runs.
        - Primary assertion on the real subprocess stdout: the served content carries the project's rule in the marked extension section; the built-in proposal rules stay unchanged (they are unmodified by the extension and still appear).
        - The request never mutates the project; nothing is written by `givn instructions`.
        - Evidence: capture argv, cwd, exit, raw stdout/stderr, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/instruction_extension_e2e_steps.rs` (real CLI).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::A developer records an extension and it governs the next guidance request"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] instruction-extension: An upgrade preserves the extension and the next request serves it
      Design constraints:
        - Fixture: a committed Givn project with an active change and a recorded extension for the proposal instruction; a real `givn upgrade` subprocess runs first, then a real `givn instructions proposal` subprocess.
        - Primary assertion on the real subprocesses: the upgrade revision carries no extension content (the `extensions:` key is user-owned config outside the allowlist); the served content carries the project's rule beside the current built-in rules.
        - Evidence: capture argv, cwd, exit, raw stdout/stderr of both commands, the revision excerpt, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/instruction_extension_e2e_steps.rs` (real CLI).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "extend-an-instruction/instruction-extension.feature::An upgrade preserves the extension and the next request serves it"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] generate-ideation-guidance: A consumer synchronizes skills and every agent target receives the ideation facilitation skill
      Design constraints:
        - Real `givn skills sync` with all agent targets enabled; the primary assertion is on the real sync stdout plus the served ideation instruction.
        - Served-content seam: `assets/instructions/ideation.md` (new) carries the ideation discipline; `assets/skills/givn-ideation/SKILL.md.tmpl` and `assets/commands/givn-ideation.md` carry the direction only.
        - Assertions: stdout contains "Syncing skills and agent commands..." and "synced skill: givn-ideation"; the served ideation instruction contains "Frame"; the generated givn-ideation command directs the ideator to the ideation instruction.
        - Evidence: capture argv, cwd, exit, raw sync stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: existing per-capability files (`tests/steps/prose_consolidation_e2e_steps.rs` for the sync; the shared served-instruction step per Global constraints §Step-file ownership).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::A consumer synchronizes skills and every agent target receives the ideation facilitation skill"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] match-guidance-to-enabled-addons: A consumer synchronizes skills and the generated guidance omits disabled-addon sections
      Design constraints:
        - Real `givn skills sync` with the arc42, domain-modeling, and coverage addons disabled; primary assertions on the served review instruction and the generated files.
        - Condition seam: `{{#if addons.*}}` blocks in `assets/instructions/review.md`; `render_conditionals` at request time; disabled-addon files (the domain-modeling skill) are removed by the existing reconciliation (`reconcile_addon_artifacts`).
        - Assertions: the served review instruction omits the Arc42 implementation conformance section and the Ubiquitous language conformance section; the generated review skill carries neither; no generated domain-modeling skill exists; no generated skill or command mentions any disabled addon.
        - Evidence: capture argv, cwd, exit, raw sync stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/template_conditional_rendering_e2e_steps.rs` (real addon toggles/sync) and `tests/steps/template_conditional_rendering_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/match-guidance-to-enabled-addons.feature::A consumer synchronizes skills and the generated guidance omits disabled-addon sections"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] match-guidance-to-enabled-addons: A consumer enables the addons a generated section needs and the guidance gains it
      Design constraints:
        - Real `givn addons enable arc42 domain_modeling` through the Givn CLI after a sync; primary assertions on the served instructions and the generated files.
        - Assertions: the served review instruction contains the Arc42 implementation conformance section and the Ubiquitous language conformance section; the served design-review instruction contains the Architecture documentation (arc42) branch; a generated domain-modeling skill exists; the generated review skill and command direct the reviewer to the review instruction.
        - E2E steps: `tests/steps/template_conditional_rendering_e2e_steps.rs` (the existing `a consumer enables the arc42 and domain-modeling addons through the Givn CLI` step) and `tests/steps/template_conditional_rendering_steps.rs`.
        - Evidence: capture argv, cwd, exit, raw stdout/stderr, and the final transcript at the design path after the last assertion.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/match-guidance-to-enabled-addons.feature::A consumer enables the addons a generated section needs and the guidance gains it"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] match-guidance-to-enabled-addons: A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed
      Design constraints:
        - `Scenario Outline` — the tag marks the outline as a whole as the e2e evidence for ONE inventory entry; both Examples rows (`arc42`, `domain-modeling`) are input variation and run in one process; assertions must hold for each example.
        - Real `givn addons disable <addon>` through the Givn CLI from a project with the domain-modeling and arc42 addons enabled and synchronized.
        - Assertions: no generated domain-modeling skill exists; the served review instruction omits the Ubiquitous language conformance section.
        - E2E steps: `tests/steps/template_conditional_rendering_e2e_steps.rs` (the existing `a consumer disables the <addon> addon through the Givn CLI` step) and `tests/steps/template_conditional_rendering_steps.rs`.
        - Evidence: capture argv, cwd, exit, raw stdout/stderr for both examples, and the final transcript at the design path after the last assertion.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/match-guidance-to-enabled-addons.feature::A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] refresh-generated-guidance: Synchronized guidance explains how to resolve the deterministic gates
      Design constraints:
        - Real `givn skills sync` with all agent targets enabled; primary assertions on the real stdout and the served instructions, never on asset source text.
        - Served-content seam: the deterministic gate rules move into `assets/instructions/review.md`, `assets/instructions/design.md`, and the new `assets/instructions/steps.md`; the generated givn-review, givn-design, and givn-steps files carry the direction only.
        - Assertions: stdout contains "Syncing skills and agent commands..."; the generated givn-review guidance directs the reviewer to the review instruction; the generated givn-design guidance directs the designer to the design instruction; the generated givn-steps guidance directs the step writer to the steps instruction; the served review instruction contains "Overlap dispositions", "split-or-keep", "removed+added"; the served design instruction contains "Black-Box-First"; the served steps instruction contains "which case this test covers"; generated guidance does not contain "Run `givn spec index`".
        - Evidence: capture argv, cwd, exit, raw sync stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/prose_consolidation_e2e_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "fragments/refresh-generated-guidance.feature::Synchronized guidance explains how to resolve the deterministic gates"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] initialize-a-project: Consumer runs init non-interactively and gets opencode skills
      Design constraints:
        - Real `givn init --no-addons --agents opencode` in a root without `givn/` that contains human-authored agent instructions to preserve; primary assertions on the real stdout and the generated files.
        - Seam: the refreshed project-start block renders `{{RESIDENT_RULES}}` from `assets/instructions/tasks.md` through `src/cli/project_hint.rs`; the generated givn-propose file is the direction; the dialog obligation stays as today.
        - Assertions: exit 0; the report names opencode and states optional addons are disabled; `.opencode/skills/givn-propose/SKILL.md` exists; the installed givn-propose guidance directs the author to the proposal instruction; `.claude/skills/givn-propose` does not exist; the project-start guidance requires the effective dialog policy before every context's first user-facing question; the obligation covers questions outside named skills and explicit handoff to delegated agents; the project-start guidance contains a policy reference rather than the worked dialog examples; the human-authored agent instructions remain unchanged; no optional tone addon is required for the obligation.
        - Evidence: capture argv, cwd, exit, raw stdout/stderr, the managed document excerpt (path-labeled), and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/initialize_a_project_e2e_steps.rs`.
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "initialize-project/initialize-a-project.feature::Consumer runs init non-interactively and gets opencode skills"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

- [ ] serve-extended-rules: An agent receives the extended rules through the Givn CLI
      Design constraints:
        - Real `givn instructions proposal --change <active>` subprocess with the project's extension recorded; the primary assertion is on the real subprocess stdout — built-in rules plus the marked extension section.
        - Seam: `src/cli/instructions.rs` renders the resolved instruction (conditionals with the project's enabled addons) and appends `config/extensions::append`'s marked section; JSON carries the same text (D2; arc42 §8.3b).
        - Assertions: the command exits 0; the served content carries the built-in proposal rules; the served content carries the project's rule in the marked extension section.
        - Evidence: capture argv, cwd, exit, raw stdout, and the final transcript at the design path after the last assertion.
        - E2E steps: `tests/steps/serve_extended_rules_e2e_steps.rs` (real CLI).
      Scoped single-scenario run (design.md §Runner and strict mode):
        `GIVN_FEATURES=givn/changes/extended-rules/specs cargo test --locked --test features_runner -- --exact "obtain-effective-guidance/serve-extended-rules.feature::An agent receives the extended rules through the Givn CLI"`
      - [ ] RED: bind this scenario's e2e steps (unimplemented bodies use `unimplemented!()`), remove `@wip` from this scenario only (never touch the `@e2e` tag), run the scoped command → MUST exit non-zero. Paste command + output:
            ```
            ```
      - [ ] GREEN: replace the stubs with real assertions against the real CLI and the minimum production code; capture the scenario's terminal transcript (title, exact argv, working-directory disposition, exit code, raw stdout/stderr) at the design-named evidence path after the final assertion. Production files created/modified:
            ```
            ```
            Evidence transcript path (design.md §Visual Design Contract):
            ```
            ```
            Run scoped → MUST exit 0. Paste command + output:
            ```
            ```
      - [ ] REFACTOR: clean up the e2e code without behaviour change; re-run scoped → MUST still exit 0. Paste command + output:
            ```
            ```
      - [ ] COMMIT: one atomic commit for RED+GREEN+REFACTOR — subject `test(e2e): <shared release note>`, scenario title in the message body, transcript included. Record the revision identifier:
            ```
            ```

