# Tasks: make-user-questions-decision-ready

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e scenarios first, then the E2E
setup task, then the @e2e scenarios.

Legend: an unchecked box means pending; a box is checked only when its evidence
and revision identifier are filled in. Check each box immediately after the
step — tasks.md must always reflect true state; never batch-check at the end.

**Release note (shared subject of every revision in this change):**

> Givn guides every user-facing question through one canonical dialog policy
> with brief sourced context, concrete answer consequences, material change
> costs, and multiple bad/good examples.

**Delta inventory** (design.md §Status and implementation boundary): 24 delta
records = 10 added + 9 modified + 5 removed; 19 executable scenarios = 14
regular + 5 `@e2e`. The 5 `@givn.removed` records carry one placeholder step and
are merge instructions — never executed, never counted as evidence. All 19
executable scenarios stay `@wip` until their own task clears the tag.

**Work order**: dialog policy content → retained instruction-guidance migration
→ shared startup overview → generated distribution and delegation → override
preservation → initialization guidance → E2E setup → E2E scenarios. The order
follows design.md's implementation sequencing (dependencies), not strict
feature-file order; a scenario whose dependency is met only by a later task
must not be started early.

---

## Global constraints (apply to every task)

- **Runner**: `tests/features_runner.rs` (cucumber-rs 0.23, `.fail_on_skipped()`)
  is the only runner. `verify.command` is `./run-tests.sh`; `verify.e2e_command`
  is `./run-tests.sh --e2e` (both from `givn/commands.yaml`).
- **Scoped single-scenario command** (use for every RED/GREEN/REFACTOR run):

  ```sh
  GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs cargo test --locked --test features_runner -- --name '^<exact scenario title>$'
  ```

  Never combine `--name` with `--tags` — cucumber makes them mutually
  exclusive. Full-delta scope commands: `GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs ./run-tests.sh`
  (19 scenarios), `... --non-e2e` (14), `... --e2e` (5).
- **Reuse the existing harness**: `TestEnv`, the real `CARGO_BIN_EXE_givn`
  subprocess helpers, isolated tempfile roots per scenario, exit-code and
  generated-consumer checks. Fixture helpers clear the parent `GIVN_FEATURES`
  before invoking production commands. No new dependency, no alternate
  hand-maintained test suite.
- **Step files, one file per capability, never one shared file**:
  `tests/steps/instruction_integrity_steps.rs`,
  `tests/steps/prose_consolidation_steps.rs`,
  `tests/steps/init_agent_target_selection_steps.rs`. The E2E files are created
  in the E2E setup task; do not register them before it.
- **Step bodies are never empty.** An unimplemented step uses the Rust stub
  `unimplemented!()` — at every phase, in every scope.
- **Behavioral RED discipline** (design.md §Scenario-by-scenario assertion and
  behavioral RED map): write the scenario's real Then assertions first, run
  them against the pre-change delivered surface with the exact fixture. A
  missing binding, a build error or an `unimplemented!()` panic is NOT the RED
  for a scenario that has a behavioral assertion. Record the RED category
  honestly:
  - `OLD_PRODUCT_RED` — the current delivered guidance fails the new assertion.
  - `ORACLE_RED` — the old contract already passes (or a coupled earlier edit
    already delivered it); prove discrimination with a controlled omission of
    only the relevant delivered guidance in an isolated fixture (project
    instruction/template override, or a temporary omitted consumer excerpt),
    rerun the same assertion, then restore the fixture immediately. Never add a
    production switch, exception, fake success or source-text test for this.
  - `DRIVER_RED` — a genuine old harness-fidelity defect (interactive selection
    only); never claim the old product's real behavior failed.
- **Production edit set** (nothing else): `assets/instructions/dialog.md`,
  `assets/agents-project-hint.md`, `assets/addons/absolute-mode/policy.md`,
  `src/cli/instructions.rs::generic_overview()`, and the 30 shipped templates
  (`assets/skills/<name>/SKILL.md.tmpl` ×15 and `assets/commands/<name>.md`
  ×15). No new dependency, module, JSON field, middleware, include mechanism or
  render-time preamble. Do not edit `src/config/cascade.rs`, target selection,
  addon config or the artifact graph.
- **Entry obligation text** (used consistently, never inside an addon
  condition):

  > For every user-facing question, follow the effective policy returned by
  > `givn instructions dialog`. Load it once in this context before the first
  > question, including clarification and confirmation outside a named
  > workflow. Pass this obligation explicitly to fresh delegated contexts; the
  > presenting agent remains responsible for the question.

- **Commit discipline**: one atomic commit per scenario (RED+GREEN+REFACTOR
  together). Message subject = the shared release note, prefixed
  `feat(<capability>): ` for regular scenarios and `test(e2e): ` for E2E
  scenarios; the scenario title goes in the message body:

  ```sh
  givn commit -m "feat(instruction-integrity): <release note>

  <Scenario title>"
  ```

  Record the revision identifier in the task's COMMIT line immediately. Never
  reuse a revision identifier across scenarios.
- **Personas** are review lenses, never Gherkin actors:
  `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

## Domain constraints (from the owning use cases)

| Task group | Use-case rule or guarantee | Persona impact |
|---|---|---|
| dialog content (instruction-integrity: retrievable, full record, teaching contrasts, migration consent, no active change, override) | create-change: every user-facing question follows the effective dialog policy, including questions outside a named skill and questions prepared in a fresh delegated context; generated default guidance carries the obligation without copying the full policy; project-owned overrides remain authoritative. | `developer--primary-user` answers informed; `agent--technical-role` finds one policy; `givn-maintainer--owner` maintains one owner. |
| startup (instruction-integrity: session start, machine-readable startup) | create-change: one canonical policy exists per topic; derived guidance points at it. | `agent--technical-role` discovers the obligation at session start without knowing every artifact. |
| distribution and delegation (prose-consolidation) | create-change: generated guidance is synchronized from canonical source assets; project-owned overrides remain authoritative. | `givn-maintainer--owner` changes one policy, not many copies; `developer--primary-user` keeps owned replacements. |
| initialization (initialize-a-project) | initialize-project: the managed project-start block requires the effective dialog policy independently of optional addons and preserves human-authored instructions outside that block; resynchronization refreshes the obligation without duplicating the block or changing the selected agent targets. | `developer--primary-user` keeps target choices and human text; `agent--technical-role` gets guidance that does not misstate the project. |

---

## Strict-mode proof and runner setup (do first)

- [ ] Setup: configure the Gherkin runner, prove strict mode, and confirm `verify.command`
      Design constraints:
        - Runner: `tests/features_runner.rs` (cucumber-rs 0.23) is the single
          runner for `givn/specs/` and the change delta. Confirm strict mode in
          code: `.fail_on_skipped()` on the builder in both the plain
          `cargo test` path and the nextest custom-harness path; the plain path
          must exit non-zero for failed or skipped counts. Do not weaken it.
        - Step-definition skeleton, ONE FILE PER CAPABILITY (extend the existing
          files; never create one shared file):
          `tests/steps/instruction_integrity_steps.rs`,
          `tests/steps/prose_consolidation_steps.rs`,
          `tests/steps/init_agent_target_selection_steps.rs`.
        - Removal exclusion (REQUIRED before ANY delta execution, including
          `givn check tasks --change make-user-questions-decision-ready`) — edit
          ONLY `tests/features_runner.rs`, test infrastructure, not public
          runtime, using the exported
          `givn::gherkin::delta::is_removed_placeholder(&Scenario)`:
          - plain Cucumber main: use `filter_run` with a non-removed
            eligibility predicate instead of unfiltered `run`; keep normal CLI
            tag/name selection, `.fail_on_skipped()`, stats and the
            failed/skipped non-zero exit;
          - nextest discovery: apply the non-removed eligibility BEFORE user tag
            matching AND before the `--ignored` complement, so removed
            identities are absent from both list phases for empty, regular and
            E2E filters;
          - nextest exact execution: compose the same eligibility with the
            selected name predicate; an explicitly requested removed unit is
            rejected with a named non-zero error BEFORE Cucumber runs.
          - Never bind removal placeholders to dummy-success steps, never add an
            unconditional success exit, and never add
            `--tags 'not @givn.removed'` to an invocation that already uses
            `--name`.
        - No change to `src/cli/check.rs`, `src/hooks.rs`, `run-tests.sh`,
          declared verify commands or archive strictness.
      PROOF OF STRICTNESS (mandatory — non-zero exit required; paste command and
      output for each):
        1. Write a throwaway probe feature (one scenario, one step with no
           binding) under a temporary probe directory; run it scoped with
           `GIVN_FEATURES=<probe dir> cargo test --locked --test features_runner -- --name '<probe title>'`
           → MUST exit non-zero with an undefined-step failure.
        2. Add a temporary binding using `unimplemented!()` and run the same
           command → MUST exit non-zero with the panic.
        3. Repeat 1–2 with an `@e2e`-tagged probe through the same runner so the
           e2e scope is proven strict as well.
        4. Remove every probe file and binding immediately, then re-run the
           discovery listing (step 5's command) once to confirm no probe title
           appears and the delta count is back to 19 units.
      ```
      <!-- paste strictness commands + non-zero outputs (regular, unimplemented, e2e) -->
      ```
      PROOF OF REMOVAL EXCLUSION (mandatory — paste output):
        5. `GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner`
           → 19 units for this delta; none of the 5 removed titles appear.
        6. Same with `GIVN_CUCUMBER_TAGS='not @e2e'` → 14 units, and with
           `GIVN_CUCUMBER_TAGS='@e2e'` → 5 units.
        7. Repeat the `@e2e` listing with `--ignored` (the complement phase)
           → no removed title appears there either.
        8. Request one removed unit explicitly through the nextest path, by its
           unit string `<relative feature path>::<removed scenario title>`
           (same format the listing prints for kept units) → MUST exit non-zero
           with the named refusal, not a zero-match success.
      ```
      <!-- paste exclusion listings + refusal output -->
      ```
      - Confirm and record the actual values in `givn/commands.yaml`:
        `verify.command: ./run-tests.sh`, `verify.e2e_command: ./run-tests.sh --e2e`
        (the shipped defaults; edit only if they differ).
      - Run: `./run-tests.sh` on the permanent corpus → must exit 0 (delta
        scenarios are still `@wip` and are not part of that scope).
      ```
      <!-- paste the permanent-corpus exit-0 run -->
      ```
  - [ ] COMMIT: `<hash>` -- Setup: runner strict-mode proof and runner-owned removal exclusion
        Commit the runner edit and the command config only if it actually
        changed. No scenario step bodies yet.

---

## Scenario tasks (non-@e2e, in dependency order)

- [ ] instruction-integrity: The canonical dialog policy is retrievable through the Givn CLI
      Design constraints:
        - Production surface: `assets/instructions/dialog.md` is the single
          complete effective policy (broadened to every user-facing question,
          project-familiar audience, necessary-question/proportionality/source/
          consequence/material-change rules). Retrieval is the real
          `givn instructions dialog` subprocess against the prepared
          initialized root — never a source-asset read.
        - Remove the dialog request helper's hardcoded
          `--change dialog-question-quality` coupling; the Given creates the
          active change with real CLI commands.
        - Assert the returned normative sections: universal scope inside and
          outside named workflows; project familiarity without assuming every
          referenced artifact; subject/context/source/consequence obligations
          for each answer; material later-change costs and the benefit
          purchased by a difficult commitment; research-before-escalation;
          authoring obligation distinguished from runtime enforcement. A
          teaching quote does not satisfy the normative rule.
      RED:      OLD_PRODUCT_RED — write the real Then assertions, run against the
                current CLI policy with this fixture → MUST FAIL (current scope
                is limited to named interactive workflows). Paste command +
                non-zero output.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit `assets/instructions/dialog.md` to the canonical policy.
                Minimum production code: that file only. Files created/modified:
                ```
                <!-- list production files; `assets/instructions/dialog.md` expected -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up without behaviour change. Re-run scoped → still PASSES.
                Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: The canonical dialog policy is retrievable through the Givn CLI

- [ ] instruction-integrity: The canonical dialog policy demonstrates the full question record
      Design constraints:
        - `assets/instructions/dialog.md` must keep the complete decision
          record (context, genuine alternatives, consequences, resolution with
          chosen/rejected reasons), gloss cross-references, and add the
          proportional treatment: routine information requests explain use and
          immediate effect without invented alternatives; confirmations
          disclose both acceptance and revision consequences without treating
          consent as a rule waiver. Keep the existing decision-record example
          as a separate illustration.
        - Assertions read the returned document contract (record fields,
          rejection reasons, glossed references, clarification without
          fictional options), not a verbatim prose snapshot.
      RED:      OLD_PRODUCT_RED on the missing proportional/confirmation
                treatment (run against current policy → FAIL). If scenario 1's
                coupled edit already delivers part of it, use ORACLE_RED:
                temporary `givn/artifacts/dialog/instructions.md` override that
                omits ONLY that treatment, leaving the record illustration
                intact; rerun the same assertion → FAIL; restore immediately.
                Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit `assets/instructions/dialog.md`. Files created/modified:
                ```
                <!-- list production files -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: The canonical dialog policy demonstrates the full question record

- [ ] instruction-integrity: Dialog teaching contrasts show complete decisions rather than bare labels
      Design constraints:
        - Publish the proposal's five complete contrasts
          (proposal.md §"Multi-shot examples for publication in the dialog
          instructions") inside `assets/instructions/dialog.md`, after the
          normative rules and the retained question-record illustration. Keep
          heading anchors `example-1-decision-record` through
          `example-5-decision-record` intact and the in-file record links
          resolvable. Declare all five hypothetical and their source links
          illustrative — not verified product files or behavior; do not invent
          a real migration or library.
        - Assertions: all five record sections and links; example 1 shows
          exactly the three supplied goal rows and excludes staff/cross-member
          authority; example 2 asks only target intent; example 3 shows both
          general/task availability outcomes; example 4 preserves port 8080 and
          names compatibility cost/benefit; example 5 reports the
          inactive/active-label counterexample and rejects a second answer.
      RED:      OLD_PRODUCT_RED — current policy contains none of the five
                records; the new assertions FAIL. ORACLE_RED calibration
                (optional, if content is already delivered): remove one goal row
                or one linked record from a delivered override while keeping all
                headings → assertion FAILS. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit `assets/instructions/dialog.md`. Files created/modified:
                ```
                <!-- list production files -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: Dialog teaching contrasts show complete decisions rather than bare labels

- [ ] instruction-integrity: Migration consent does not imply an unverified recovery guarantee
      Design constraints:
        - The material-commitment contrast (example 4) must distinguish
          conversion/recovery required before rollout from recovery already
          implemented, state older-release incompatibility and unchanged saved
          projects for the less-committing answer, and the normative rule must
          require unknown compatibility facts to be established or consent
          narrowed before rollout approval.
        - Do not invent a real saved-format migration fixture or promise a
          reversible upgrade; examples stay hypothetical.
      RED:      OLD_PRODUCT_RED — current policy has no material-commitment
                contrast or unknown-recovery boundary. If a coupled edit already
                delivered the contrast, ORACLE_RED: omit only the
                recovery/compatibility qualification, keeping the conversion
                choice and port value intact → assertion FAILS. Paste command +
                output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit `assets/instructions/dialog.md`. Files created/modified:
                ```
                <!-- list production files -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: Migration consent does not imply an unverified recovery guarantee

- [ ] instruction-integrity: The dialog policy is retrievable without an active change
      Design constraints:
        - Real `givn instructions dialog` in a root with `givn/` initialized and
          NO active change: exits 0 and returns the effective universal/
          proportional question-preparation contract without requiring a change
          selection. Keep the untouched unknown-id non-zero
          `unknown artifact` error contract.
        - Migrate this retained caller's incidental phrase assertion to the real
          returned-policy contract; do not re-pin old wording. Once no callers
          remain, remove the obsolete named-artifact/option/standalone/
          rejected-alternative substring handlers and the source-asset-loading/
          full-record handlers in `tests/steps/instruction_integrity_steps.rs`.
      RED:      Availability may be BASELINE_GREEN: OLD_PRODUCT_RED on the
                missing explicit proportional/universal treatment, or
                ORACLE_RED by omitting only that normative clause through a
                temporary dialog override after coupled delivery. A missing
                binding is never its behavioral RED. Paste command + output +
                category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes only if the assertion exposes a real gap
                (`assets/instructions/dialog.md`); otherwise this is assertion
                migration — state that explicitly with the file list. Files
                created/modified:
                ```
                <!-- list production files (may be none: migration scenario) -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up retired handlers; re-run scoped → still PASSES. Paste
                output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: The dialog policy is retrievable without an active change

- [ ] instruction-integrity: Design guidance requires the complete E2E setup record
      Design constraints:
        - Real `givn instructions design` returned guidance must require: the
          runner's isolated E2E scope, undefined/pending proof, separate
          bindings, a real driver, local infrastructure, framework
          justification, a matching interaction coverage matrix, and the
          terminal-transcript vs browser-viewport distinction.
        - This replaces the removed E2E identity `An author requests the
          technical E2E design guidance`; move its consumer-contract assertions
          into the regular bindings in `tests/steps/instruction_integrity_steps.rs`
          using the shared subprocess mechanism. Never register duplicate step
          text and never keep a source-reader test.
      RED:      Existing product content already defines the setup record →
                BASELINE_GREEN permitted. ORACLE_RED: temporary
                `givn/artifacts/design/instructions.md` override with the
                scope/strictness requirement omitted, rerun the real document
                assertions → FAIL; restore immediately. Never claim an old
                product defect. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes only if the assertions expose a real gap
                (`assets/instructions/design.md`); otherwise state
                "migration only" with the file list. Files created/modified:
                ```
                <!-- list production files / migration note -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Remove the stale source reader; re-run scoped → still PASSES.
                Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: Design guidance requires the complete E2E setup record

- [ ] instruction-integrity: Review guidance names the deterministic disposition gates
      Design constraints:
        - Real `givn instructions review` returned guidance must require a
          reasoned disposition for every reported overlap pair, a split-or-keep
          decision for every reported long scenario, and the distinction
          between an advisory finding and completed verification evidence.
        - This replaces the removed E2E identity `A reviewer requests
          disposition-aware review guidance`; same migration rules as the
          design-guidance task: real returned CLI guidance, shared subprocess
          mechanism, no source readers, no duplicate step text.
      RED:      BASELINE_GREEN permitted. ORACLE_RED: temporary
                `givn/artifacts/review/instructions.md` override omitting the
                per-pair or split/keep requirement → assertion FAILS; restore
                immediately. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes only if the assertions expose a real gap;
                otherwise state "migration only" with the file list. Files
                created/modified:
                ```
                <!-- list production files / migration note -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Remove the stale source reader; re-run scoped → still PASSES.
                Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: Review guidance names the deterministic disposition gates

- [ ] instruction-integrity: Machine-readable startup guidance retains the dialog obligation while reporting an active change
      Design constraints:
        - `src/cli/instructions.rs::generic_overview()` supplies the shared
          overview for every text report and the JSON `overview` field; append
          the short entry obligation to it. Add no `dialog_policy` field and no
          full contrasts; preserve existing `id`, `next_command`, `next_skill`,
          status/stage data and next-action semantics.
        - Fixture: an initialized project with an active change whose exact id is
          `decision-ready-json`, a completed proposal, and specs next. Assert
          JSON `id` equals that value, the next command requests specs for that
          id, the next skill is `givn-spec`, the `overview` carries the
          obligation, and no extra policy field or worked contrasts appear.
          Identity/next-action checks must discriminate a wrong stage or an
          unrelated active change, not merely a nonempty JSON.
      RED:      OLD_PRODUCT_RED — current JSON overview lacks the obligation.
                Run the real assertions against the prepared fixture → FAIL.
                ORACLE_RED alternative after a coupled edit: omit only the
                overview obligation. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit `src/cli/instructions.rs::generic_overview()`. Files
                created/modified:
                ```
                <!-- list production files -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: Machine-readable startup guidance retains the dialog obligation while reporting an active change

- [ ] prose-consolidation: Disabling optional addons does not disable the dialog obligation
      Design constraints:
        - Fixture: all three agent targets selected, every optional addon
          disabled → real `givn skills sync`. Independently require the 13
          enumerated core skill families in every selected target and the 13
          core commands only in opencode; optional families (domain-modeling,
          dev-principles, grill-me, grill-with-docs) absent; no disabled-addon
          vocabulary in the core obligation; selected targets and addon
          configuration preserved.
        - Expected installed families come from the public template inventory in
          design.md §Complete shipped-template inventory, never from the
          production generator's own output list, so a missing family cannot
          make actual and expected silently shrink.
        - The obligation names no addon and stays outside addon conditionals.
      RED:      OLD_PRODUCT_RED — a core entry currently lacks the obligation.
                If a coupled delivery already added it, ORACLE_RED: omit one
                core reference while optional families stay disabled → the
                per-entry assertion FAILS. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit the core skill/command templates under `assets/skills/`
                and `assets/commands/`. Files created/modified:
                ```
                <!-- list every template edited -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- prose-consolidation: Disabling optional addons does not disable the dialog obligation

- [ ] prose-consolidation: Delegated question preparation carries the policy without assuming inherited context
      Design constraints:
        - Insert the obligation INSIDE each self-contained delegated prompt,
          never merely in the parent's surrounding prose: command templates
          `givn-propose`, `givn-design-review` (grilling and hardening
          reviewers), `givn-implement`, `givn-review`, `givn-characterize`,
          `givn-reverse-engineer`; and the spawning/review passages in skills
          `givn-implement`, `givn-design-review`, `givn-review`,
          `givn-characterize`, `givn-ideation`, `givn-reverse-engineer`.
          Delegated contexts load the effective policy themselves before
          preparing a question; the orchestrator verifies a delegated question
          before it reaches the person; question-tool labels/recommendations do
          not replace visible context and effects. Reference-only scope in
          `givn-steps` and principles: no new interaction step.
        - Slice every enumerated question-preparing prompt/passage; a file's
          top-level obligation must not satisfy a missing nested prompt
          obligation. Do not register duplicate step text.
      RED:      OLD_PRODUCT_RED on the current nested prompt (absent
                obligation). ORACLE_RED: leave the file's top-level obligation
                intact but omit it in one nested prompt → the prompt-local
                assertion FAILS. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Edit the six command templates and six skill templates named
                above. Files created/modified:
                ```
                <!-- list every template edited -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- prose-consolidation: Delegated question preparation carries the policy without assuming inherited context

- [ ] prose-consolidation: Successful project skill and command replacements are preserved without policy injection
      Design constraints:
        - Fixture (design.md §Successful replacement fixture and consumer
          paths): init with all three agent targets; write valid LF-terminated
          project replacements at `givn/skills/givn-propose/SKILL.md.tmpl`
          (frontmatter name `givn-propose`, nonempty description, one
          project-owned scope instruction) and `givn/commands/givn-propose.md`
          (valid command frontmatter, one project-owned scope instruction), both
          deliberately WITHOUT the bundled dialog obligation, template
          variables or addon conditions; capture exact source bytes.
        - After real `givn skills sync`: source bytes unchanged; exact equality
          of `.opencode/skills/givn-propose/SKILL.md`,
          `.agents/skills/givn-propose/SKILL.md`,
          `.claude/skills/givn-propose/SKILL.md` and
          `.opencode/commands/givn-propose.md` to those expected bytes
          (equality rejects any hidden prefix/suffix); no command under
          agents/claude; the managed `givn-project` block in AGENTS.md still
          loads the effective policy.
      RED:      Preservation may be BASELINE_GREEN; OLD_PRODUCT_RED on the
                current managed block's missing obligation. ORACLE_RED: omit
                that managed reference after coupled delivery; calibrate the
                no-injection assertion against a consumer with an added default
                prefix WITHOUT changing production. Paste command + output +
                category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes needed only for the managed hint obligation
                (`assets/agents-project-hint.md`); preserve-replacement
                behavior must not be modified. Files created/modified:
                ```
                <!-- list production files; note preservation is unchanged behavior -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Remove the temporary fixture calibration; re-run scoped → still
                PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- prose-consolidation: Successful project skill and command replacements are preserved without policy injection

- [ ] instruction-integrity: A project dialog override remains the effective policy after synchronization
      Design constraints:
        - Fixture: project-owned dialog override at
          `givn/artifacts/dialog/instructions.md` requiring human approval
          before broadening the selected project scope. Real `givn skills sync`,
          then request the dialog instructions in text and JSON.
        - Assert: returned text/JSON is the project replacement (bytes match);
          the override source file is unchanged on disk; the synchronized entry
          documents name EFFECTIVE policy retrieval (not a bundled copy) and
          contain no bundled contrasts. Precedence is existing behavior — the
          new part is the entry loading obligation.
      RED:      OLD_PRODUCT_RED on the missing entry loading obligation (not on
                the already-working precedence). ORACLE_RED: omit that reference
                from one consumer entry while preserving the override → the
                reference assertion FAILS. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    The entry references come from the generated templates edited in
                the earlier distribution tasks; only edit templates here if the
                assertion exposes a genuine gap. Files created/modified:
                ```
                <!-- list production files; note override behavior unchanged -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- instruction-integrity: A project dialog override remains the effective policy after synchronization

- [ ] initialize-a-project: Fossil initialization names the supported attribution path without Git plumbing
      Design constraints:
        - Fixture: the existing Fossil fixture — a Fossil checkout without
          `givn/`. Run real `givn init --agents opencode`.
        - Assert: exit 0; stdout names the givn commit attribution path (no
          Git-plumbing claim); no `.githooks` directory exists; the generated
          implementation guidance names the active backend's commit path and
          states that a raw `fossil commit` carries no attribution.
        - This replaces the removed E2E identity `Consumer initializes a Fossil
          checkout and receives backend-aware guidance`; backend choice is a
          precondition variant, not a second init action.
      RED:      BASELINE_GREEN permitted. ORACLE_RED: omit the relevant
                delivered backend-attribution guidance excerpt from the fixture
                (or override the generated excerpt), rerun the assertion → FAIL;
                restore immediately. Never present the oracle as an old-product
                failure. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes only if the assertion exposes a real gap
                (`assets/agents-project-hint.md` / template wording); otherwise
                state "precondition variant, content already delivered" with the
                file list. Files created/modified:
                ```
                <!-- list production files / migration note -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- initialize-a-project: Fossil initialization names the supported attribution path without Git plumbing

- [ ] initialize-a-project: Initialization without SCM preserves guidance and names the later archive refusal
      Design constraints:
        - Fixture: a root without a Git worktree or Fossil checkout. Run real
          `givn init --agents opencode`.
        - Assert: exit 0; `.opencode/skills/givn-propose/SKILL.md` exists;
          stdout warns that archive and managed upgrade are unavailable; a
          later archive attempt is refused BY NAME — this last boundary must run
          the real second command (`givn archive`), never a dummy output.
        - Replaces the removed E2E identity `Consumer initializes a plain
          directory and receives a named warning`; no-SCM is a precondition
          variant.
      RED:      BASELINE_GREEN permitted. ORACLE_RED: omit the returned warning
                from the captured consumer transcript and rerun its warning
                assertion → FAIL; restore immediately. Paste command + output +
                category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production changes only if the assertion exposes a real gap;
                otherwise state "precondition variant" with the file list. Files
                created/modified:
                ```
                <!-- list production files / migration note -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- initialize-a-project: Initialization without SCM preserves guidance and names the later archive refusal

---

## E2E setup (after all non-@e2e scenarios are GREEN)

- [ ] Setup: e2e step skeletons, e2e strict-mode proof, and `verify.e2e_command` isolation proof
      Design constraints:
        - E2E step files, one file per capability, separate from the regular
          steps: extend `tests/steps/instruction_integrity_e2e_steps.rs`; create
          `tests/steps/prose_consolidation_e2e_steps.rs`; create
          `tests/steps/initialize_a_project_e2e_steps.rs`. Register ONLY the two
          new modules in `tests/steps/mod.rs`.
        - `verify.e2e_command` is exactly `./run-tests.sh --e2e` — never the same
          string as `verify.command` (`./run-tests.sh`). No browser, HTTP API,
          server, database, model service or network dependency is introduced:
          every changed action is a real CLI subprocess or a real PTY; the only
          local preparation is the script's existing
          `cargo build --locked --test ready_fixture`. Local environment
          start/stop: not applicable (nothing to start); state this explicitly.
        - Strictness in the e2e scope: same proof procedure as the main setup
          (temporary `@e2e` probe with an undefined step and one with
          `unimplemented!()`), run through the e2e scope → MUST exit non-zero;
          remove the probes immediately.
      PROOF OF ISOLATION (mandatory — paste both listings):
        - `GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner`
          → 19 units.
        - Same with `GIVN_CUCUMBER_TAGS='@e2e'` → 5 units, strictly fewer than
          the default scope; and `'not @e2e'` → 14.
      ```
      <!-- paste e2e strictness proof + the three listings -->
      ```
  - [ ] COMMIT: `<hash>` -- Setup: e2e step skeletons, strict-mode proof and scope isolation

---

## E2E Smoke Tests

- [ ] @e2e instruction-integrity: Author starts a session with the universal dialog obligation
      Design constraints:
        - Real CLI subprocess: `CARGO_BIN_EXE_givn instructions` (text, no
          `--change`, no artifact id) in an initialized root WITHOUT an active
          change. This is the one genuinely new E2E action (session start).
        - Assert stdout: the startup guidance requires loading the effective
          dialog policy before the context's first user-facing question; the
          obligation applies outside named skills and to delegated question
          preparation; loading once per context is distinguished from rereading
          before every question; the begin-change guidance (`givn new`) is
          retained. Apply the applicable terminal baseline assertions (real
          heading/subject, no internal-identifier-only labels, no duplicated
          content, English).
        - Capture actual argv, cwd, exit, raw stdout into the existing world
          fields and commit the final transcript at
          `givn/changes/make-user-questions-decision-ready/evidence/visual/<scenario-slug>/transcript.txt`
          using the existing `givn::integrity_hook::scenario_slug` rule, after
          the final assertion. Preserve raw output; do not label file excerpts
          as stdout.
      RED:      OLD_PRODUCT_RED — run the new assertions against the current
                no-change startup output with this fixture → FAIL (missing
                loading/handoff sentence). ORACLE_RED alternative after coupled
                delivery: omit the loading/handoff sentence from a temporary
                overview excerpt but preserve the next-action text → FAIL. Paste
                command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production surface: `src/cli/instructions.rs::generic_overview()`
                plus `assets/agents-project-hint.md` (unconditional startup
                obligation). Set up the e2e fixture with real CLI commands.
                Files created/modified:
                ```
                <!-- list production files + the new e2e step file + evidence transcript -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up e2e code. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- test(e2e): Author starts a session with the universal dialog obligation

- [ ] @e2e prose-consolidation: Generated guidance points to the canonical E2E policy
      Design constraints:
        - Real `givn skills sync` in a project with all three agent targets and
          all optional guidance enabled. Independently expect 15 skill families
          in EACH selected target and 15 commands only in opencode
          (`.opencode/commands`); assert from the public template inventory, not
          the generator's own list.
        - Assert: every emitted skill and command requires the effective dialog
          policy before its context asks a user-facing question; direct skill
          invocation has the same obligation as session-start invocation;
          reference skills do not narrow it to one workflow phase; canonical
          interaction/E2E references remain; detailed dialog rules and worked
          examples remain in the effective dialog instructions rather than
          competing generated copies; the sync report names the selected
          guidance successfully. Assert the applicable terminal baseline.
        - Evidence: raw sync stdout plus path-labeled delivered excerpts, and the
          final transcript at `evidence/visual/<scenario-slug>/transcript.txt`.
      RED:      OLD_PRODUCT_RED on a currently missing entry obligation across
                the 30 templates. ORACLE_RED: omit one reference in a real
                project template replacement, then sync → the per-entry
                assertion FAILS; every expected consumer must still be checked.
                Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production surface: the 13 core skill templates, the 2 optional
                skill templates, the 13 core command templates, the 2 optional
                command templates, and `assets/agents-project-hint.md`. Files
                created/modified:
                ```
                <!-- list production files + new e2e step file + transcript -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up e2e code. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- test(e2e): Generated guidance points to the canonical E2E policy

- [ ] @e2e initialize-a-project: Consumer runs init non-interactively and gets opencode skills
      Design constraints:
        - Real `givn init --no-addons --agents opencode` in a root without
          `givn/`, containing human-authored agent instructions to preserve.
        - Assert: exit 0; the report names opencode and states optional addons
          are disabled; `.opencode/skills/givn-propose/SKILL.md` exists and
          `.claude/skills/givn-propose` does not; the generated managed
          `givn-project` block requires the effective dialog policy before every
          context's first user-facing question, covers questions outside named
          skills and explicit handoff to delegated agents, and contains a policy
          reference rather than the worked examples; human-authored agent bytes
          unchanged; no optional tone addon required. Assert the applicable
          terminal baseline.
        - Evidence: raw stdout/stderr plus the managed document excerpt
          (path-labeled) and the final transcript at
          `evidence/visual/<scenario-slug>/transcript.txt`.
      RED:      OLD_PRODUCT_RED — current init's managed hint lacks the
                obligation; run the real assertions with this fixture → FAIL.
                ORACLE_RED alternative: omit only the managed reference while
                keeping the successful install report and files. Paste command +
                output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production surface: `assets/agents-project-hint.md` (and
                `src/cli/instructions.rs::generic_overview()` only if the
                manifest demands it). Files created/modified:
                ```
                <!-- list production files + new e2e step file + transcript -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up e2e code. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- test(e2e): Consumer runs init non-interactively and gets opencode skills

- [ ] @e2e initialize-a-project: Consumer re-syncs skills after init and the selection is honoured
      Design constraints:
        - Fixture: `givn/` initialized; `givn/config.yaml` skills targets equal
          `opencode, claude`; an older managed startup block beside
          human-authored agent instructions. Real `givn skills sync`.
        - Assert: exit 0; the report identifies the selected guidance
          destinations; `.opencode/skills/givn-propose/SKILL.md` and
          `.claude/skills/givn-propose/SKILL.md` exist while
          `.agents/skills/givn-propose` does not; exactly ONE managed
          begin/end block replaces the stale content (no duplication);
          outside-human bytes unchanged; the persisted selection and unrelated
          configuration unchanged; the managed block now requires the effective
          policy for every user-facing question. Assert the applicable terminal
          baseline (exactly one occurrence of the obligation, not repeated full
          rules).
        - Evidence: raw stdout/stderr, exact before/after byte comparisons, and
          the final transcript at `evidence/visual/<scenario-slug>/transcript.txt`.
      RED:      OLD_PRODUCT_RED on the missing refreshed obligation. ORACLE_RED:
                leave one stale block or omit the new reference in an isolated
                delivered excerpt → the assertion FAILS; a successful exit alone
                cannot pass. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Production surface: `assets/agents-project-hint.md` and the
                managed-block writing path in `src/cli/agents_block.rs` only if
                the assertion exposes a real gap — behavior preservation is not
                modified. Files created/modified:
                ```
                <!-- list production files + new e2e step file + transcript -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up e2e code. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- test(e2e): Consumer re-syncs skills after init and the selection is honoured

- [ ] @e2e initialize-a-project: Consumer selects agent targets during interactive init
      Design constraints:
        - REAL terminal: reuse the existing script-backed helper
          (`script -qec` via `run_init_in_terminal`, 60-second deadline,
          propagated child exit status). MOVE the helper from
          `tests/steps/init_agent_target_selection_steps.rs` into
          `tests/common/mod.rs`, preserve raw-key semantics, timeout and
          propagated status, and update EVERY existing caller in that cutover.
          Piped stdin is NOT acceptable evidence. Do not change the product
          prompt.
        - Verified option order is opencode, agents, claude with only opencode
          premarked: send the key bytes `\x1b[B\x1b[B\x20\n` to mark claude and
          confirm — not the literal string `opencode, .claude`.
        - Assert: the terminal stream shows a labeled agent-target selection and
          confirms the selected integrations; exit 0; persisted
          `givn skills targets` equal `opencode, claude`; the opencode and claude
          skill outputs exist; `.agents/skills/givn-propose` does not exist;
          terminal baseline covers readable option labels. PTY output is one
          terminal stream — do not claim separate stdout/stderr fidelity.
        - Evidence: final raw PTY transcript plus the selected configuration
          after the assertions, committed at
          `evidence/visual/<scenario-slug>/transcript.txt`, and the final
          transcript capture after the last assertion.
      RED:      DRIVER_RED — run the new prompt assertion against the pre-change
                piped E2E output with the same fixture: it contains no checkbox
                prompt, so the assertion FAILS. Never claim the old product's
                real PTY prompt failed. Paste command + output + category.
      ```
      <!-- paste RED evidence: command + output + category -->
      ```
      GREEN:    Test-fidelity cutover: helper move into `tests/common/mod.rs`,
                every caller updated, new
                `tests/steps/initialize_a_project_e2e_steps.rs` binding. Files
                created/modified:
                ```
                <!-- list test-infrastructure files + transcript; production prompt unchanged -->
                ```
                Run scoped → PASSES. Paste output.
      ```
      <!-- paste GREEN evidence: command + output + category -->
      ```
      REFACTOR: Clean up e2e code. Re-run scoped → still PASSES. Paste output.
      ```
      <!-- paste REFACTOR evidence -->
      ```
      - [ ] COMMIT: `<hash>` -- test(e2e): Consumer selects agent targets during interactive init

---

**Done when:** every box is checked with pasted evidence and a unique revision
identifier, all 19 `@wip` tags are legitimately cleared, and with
`GIVN_FEATURES=givn/changes/make-user-questions-decision-ready/specs`:
`./run-tests.sh` exits 0 (19 scenarios), `./run-tests.sh --non-e2e` exits 0
(14), `./run-tests.sh --e2e` exits 0 (5). The five E2E transcripts exist under
`evidence/visual/<scenario-slug>/transcript.txt`. Remaining review-phase work
(semantic fresh-context exercise with unseen situations, Persona and
`VISUAL-REVIEW` judgment) belongs to `/givn-review`; it is not a task here and
must not be silently waived.
