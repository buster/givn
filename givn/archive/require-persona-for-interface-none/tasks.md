# Tasks: require-persona-for-interface-none

RED→GREEN→REFACTOR→COMMIT per delta scenario, non-`@e2e` first. All four delta
scenarios are non-`@e2e`. No delta `@e2e` scenario is added or expected: the
review-check and guidance actions already have permanent `@e2e` coverage and
this change introduces no new consumer action.

Legend: unchecked boxes are pending; check a box only when its evidence and
commit hash are filled in. Check each box immediately after its step — never
batch-check.

## Interaction coverage pre-flight

Owning use case: `verify-change`
(`givn/specs/verify-change/usecase.md`). Inventory cross-check against
design.md's Interaction Coverage Matrix:

| Inventory entry | Matrix row | Existing `@e2e` scenario | Gap |
|---|---|---|---|
| `visual-review-gate` — run the review check on a visual change | Present | `A visual change with complete evidence passes review` (`givn/specs/verify-change/visual-review-gate.feature`, `@e2e`) | none |
| `visual-review-gate` — read the visual review guidance | Present | `Review guidance requires the visual verdict and persona lenses` (`givn/specs/verify-change/visual-review-gate.feature`, `@e2e`) | none |

Result: **2/2 inventory entries mapped; no coverage gap.** The new
`Interface: none` cases are variants of the existing review-check action, and
the new guidance case is a variant of the existing guidance action. No new
interaction, capability, or `@e2e` scenario is created, and the two permanent
`@e2e` scenarios stay untouched.

Overlap dispositions to carry into `review.md` at review time:

- The two `Interface: none` failure scenarios are variants of each other.
- The modified exemption scenario is a boundary against the permanent
  no-contract exemption.
- The guidance scenario is a variant of the existing guidance action.

## Domain constraints (from the use case and design)

- Goal: check a change for structural validity, completeness, and unresolved
  overlap before implementation or archive.
- Minimal guarantee: an invalid change cannot pass the verification gate.
- Success guarantee: the developer receives a complete verification result.
- Main flow 5: the failure names the missing or offending Persona value so the
  developer can correct it in one pass.
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`. `fixture-persona` is a test fixture file under
  `givn/personas/`, not a project Persona.
- The Persona requirement follows the presence of a declared
  `## Visual Design Contract`, not the rendered interface. `Interface: none`
  skips only the visual verdict and visual evidence. A change with no contract
  section remains fully exempt.

Behavior rule (from design.md):

| Design declaration | Persona disposition | Visual verdict | Visual evidence |
|---|---|---|---|
| No `## Visual Design Contract` section | Not required; a reasoned none is allowed | Not required; `VISUAL-REVIEW: NOT APPLICABLE` valid | Not required |
| Contract with `- Interface: none` | Required: at least one confirmed Persona resolving to `givn/personas/<id>.md` | Not required | Not required |
| Contract with `- Interface: web` | Required | Required (`VISUAL-REVIEW: PASS`) | Required per viewport and declared state |
| Contract with `- Interface: terminal` | Required | Required (`VISUAL-REVIEW: PASS`) | Required transcript per `@e2e` scenario |

`PERSONAS: none`, `PERSONAS: none — <reason>`, and an empty value never
satisfy a declared contract.

`check_visual_review` order (design.md §Controlled behavior change):

1. `design.md` absent → `Ok(())`.
2. `parse_visual_contract` returns `None` → `Ok(())`.
3. `Interface: web` with empty viewports → existing viewport error.
4. `Interface: none` → read `review.md`; if missing, bail with the
   missing-persona message; call `check_persona_disposition(&review,
   change_dir)?`; return `Ok(())` **without** reading `VISUAL-REVIEW:` or
   calling `check_visual_evidence`.
5. `web`/`terminal` → existing order: review present, exactly one verdict,
   `check_persona_disposition`, then `check_visual_evidence`.

Error strings (exact):

- Missing line or missing `review.md`: `review.md is missing the persona
  disposition — add 'PERSONAS: <confirmed persona>' to review.md`
- Empty or `none` value: `a declared visual design contract requires at least
  one confirmed Persona — 'PERSONAS: {value}' is not a confirmed persona`
- Value that resolves to no file: `the persona disposition '{value}' matches no
  file in givn/personas/ — name a confirmed Persona; a reasoned none is valid
  only when the design declares no visual design contract`

No other production behavior changes: `parse_visual_contract`,
`VisualContract`, `VisualInterface`, `visual_verdict`, `check_visual_evidence`,
`collect_e2e_scenario_titles`, `check_deferrals`, and `run_integrity_hook` keep
their behavior.

### Sequencing note for the two failure variants

Scenarios 2 and 3 are both closed by the `Interface: none` branch introduced
for scenario 1. To keep RED honest, run their targeted commands **before
scenario 1's GREEN**, while their `@wip` tags are still present — this runner
does not filter `@wip`, so the commands execute. Paste each non-zero output into
its own RED field. Remove each `@wip` in that scenario's own RED step after
scenario 1's commit; the tag has no execution effect, so the captured output
remains the RED evidence. Their own GREEN phases then add the regression
coverage and the error-string text, and each still gets its own commit.

### Commit convention

Every scenario uses one atomic commit. Message subject is the change release
note; message body is the scenario title. No two scenarios share a revision.

```
givn commit -m "$(printf '%s\n\n%s\n' \
  'Review now requires a confirmed Persona for every change that declares a visual design contract, including `Interface: none`, while a change that declares no visual design contract stays exempt.' \
  '<scenario title>')"
```

Immediately after the commit, replace that scenario's pending COMMIT line with
exactly one real reachable checked COMMIT line and check the scenario box.
Never check a box before the commit exists.

Before every scenario commit, run `cargo fmt --all` and
`cargo clippy --locked --all-targets -- -D warnings`; fix any formatting or
lint finding inside REFACTOR with no behavior change.

---

## Setup — runner, strict mode, and baseline (no scenario commit)

Design constraints: `verify.command` is `./run-tests.sh --non-e2e` and
`verify.e2e_command` is `./run-tests.sh --e2e` in `givn/commands.yaml`;
`tests/features_runner.rs` is the cucumber-rs runner (`harness = false`,
`.fail_on_skipped()`, exit 1 when `stats.failed > 0 || stats.skipped > 0`);
step definitions for this capability live in
`tests/steps/visual_review_gate_steps.rs`, shared CLI steps in
`tests/steps/mod.rs`. No new runner binary or step file is needed.

- [x] **Verify runner and strict-mode configuration is already in place.**
      Confirm all of the following and paste the evidence:
      - `givn/commands.yaml` lines: `verify.command: "./run-tests.sh --non-e2e"`,
        `verify.e2e_command: "./run-tests.sh --e2e"`.
      - `tests/features_runner.rs` calls `.fail_on_skipped()` and exits `1`
        when `stats.failed > 0 || stats.skipped > 0`.
      - `tests/steps/visual_review_gate_steps.rs` exists and is the single
        step file for `visual-review-gate`.
      ```
      <!-- paste the four confirmed lines -->
      ```
- [x] **Proof-of-strictness.** Run the runner against a throwaway feature with
      an undefined step. Must exit non-zero. If it exits 0, enable strict mode
      and re-prove before continuing.
      ```
      mkdir -p /tmp/opencode/givn-strict-probe
      printf 'Feature: Strict probe\n\n  Scenario: Strict mode probe\n    Given no step definition exists for this probe\n' \
        > /tmp/opencode/givn-strict-probe/probe.feature
      GIVN_FEATURES=/tmp/opencode/givn-strict-probe \
        cargo test --locked --test features_runner -- --name "Strict mode probe"
      ```
      ```
      <!-- paste command and non-zero-exit output -->
      ```
- [x] **Remove the probe and restore the passing baseline.** Remove the probe
      directory, then run the permanent non-e2e suite. Must exit 0.
      ```
      rm -rf /tmp/opencode/givn-strict-probe
      ./run-tests.sh --non-e2e
      ```
      ```
      <!-- paste summary and exit 0 -->
      ```

---

## Scenario 1 — A change that declares no visual interface is exempt

`@givn.modified` — passing boundary case for a declared `Interface: none`
contract.

Design constraints:

- Behavior row: contract with `- Interface: none` requires a confirmed Persona
  but no visual verdict and no visual evidence.
- New fixture step:
  `#[given("the change's review.md names a confirmed persona without a visual verdict")]`
  in `tests/steps/visual_review_gate_steps.rs`. It writes `README-IMPACT: none`,
  `REVIEW: PASS`, `PERSONAS: fixture-persona`, and no `VISUAL-REVIEW:` line.
  The existing `Given a givn project with a change that has passed
  design-review` (in `tests/steps/workflow_enforcement_steps.rs`) supplies the
  `givn/personas/fixture-persona.md` file.
- Production: new `Interface: none` branch in `check_visual_review`
  (`src/integrity_hook.rs`), exactly design step 4 above.
- Assertion: `givn check review --change test-change` exits 0; no evidence file
  and no verdict are required.
- Unit tests in `src/integrity_hook.rs`, `mod tests`:
  - `interface_none_does_not_require_visual_evidence`: design with
    `Interface: none`, a delta `@e2e` scenario, a confirmed Persona
    (`shopper-anna`), and no verdict/screenshot passes `check_visual_review`.
  - `no_visual_contract_remains_exempt`: a design with no contract section and
    no `review.md` passes `check_visual_review`.
- Single-scenario command:
  ```
  GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
    cargo test --locked --test features_runner -- \
    --name "A change that declares no visual interface is exempt"
  ```

- [x] Scenario: A change that declares no visual interface is exempt
  - [x] **RED.** Remove only `@wip` from this scenario in
        `givn/changes/require-persona-for-interface-none/specs/verify-change/visual-review-gate.feature`
        (keep `@givn.modified`). The new Given step is undefined, so the
        targeted run must exit non-zero.
        ```
        <!-- paste the targeted run output showing the undefined step and non-zero exit -->
        ```
  - [x] **GREEN.** Before running this GREEN, capture the RED output for
        scenarios 2 and 3 while their `@wip` tags remain (see Sequencing
        note). Then add the fixture step and the `Interface: none` branch; add
        the two named unit tests. Files created/modified:
        `src/integrity_hook.rs`, `tests/steps/visual_review_gate_steps.rs`.
        Run the unit tests first, then the targeted scenario. Both must exit 0.
        ```
        cargo test --locked integrity_hook::tests::interface_none_does_not_require_visual_evidence
        cargo test --locked integrity_hook::tests::no_visual_contract_remains_exempt
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          cargo test --locked --test features_runner -- \
          --name "A change that declares no visual interface is exempt"
        ```
        ```
        <!-- paste unit and targeted run output, exit 0 -->
        ```
  - [x] **REFACTOR.** No behavior change. Re-run the two unit tests and the
        targeted scenario; must still exit 0.
        ```
        <!-- paste re-run output, exit 0 -->
        ```
  - [x] **COMMIT.** One commit containing the branch, the fixture step, the
        unit tests, and this scenario's `@wip` removal. Use the commit
        convention above with body
        `A change that declares no visual interface is exempt`.
        Record exactly one checked COMMIT line with the real hash.
  - [x] COMMIT: `14a686fdf9dba61b57d93bd15e26c218a78dc1cd`

## Scenario 2 — A change that declares the interface none without a persona disposition fails review

`@givn.added` — missing Persona blocks even when a visual verdict is present.

Design constraints:

- Behavior row: a declared `Interface: none` contract without a confirmed
  Persona fails; the first failure is the Persona disposition because `none`
  has no verdict check.
- Fixture: reuse `the change's review.md declares no persona disposition`
  (writes `VISUAL-REVIEW: PASS` and no `PERSONAS:` line).
- Assertions: command exits non-zero; stderr contains `PERSONAS`.
- Production (text only; behavior comes from scenario 1's branch):
  - Broaden the `check_persona_disposition` doc comment from "a declared visual
    interface" to "a declared visual design contract".
  - Error string #1 (missing line or missing `review.md`): `review.md is
    missing the persona disposition — add 'PERSONAS: <confirmed persona>' to
    review.md`.
  - Error string #2 (empty or `none` value): `a declared visual design contract
    requires at least one confirmed Persona — 'PERSONAS: {value}' is not a
    confirmed persona`.
- Unit tests in `src/integrity_hook.rs`, `mod tests`:
  - `interface_none_requires_confirmed_persona`: `Interface: none` plus
    `review.md` naming `shopper-anna` passes; missing `review.md`, missing
    `PERSONAS:` line, `PERSONAS: none`, and `PERSONAS: none — operator
    confirmed` all fail. Assert the missing cases contain error string #1 and
    the `none` case contains error string #2.
  - `interface_none_default_requires_persona`: a contract section with no
    recognized `- Interface:` line, and a contract with an unrecognized value,
    are both treated as `Interface: none`; without a confirmed Persona both
    fail.
- Single-scenario command:
  ```
  GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
    cargo test --locked --test features_runner -- \
    --name "A change that declares the interface none without a persona disposition fails review"
  ```

- [x] Scenario: A change that declares the interface none without a persona disposition fails review
  - [x] **RED.** Remove only `@wip` from this scenario. The named run was
        executed before scenario 1's GREEN, while `@wip` was still present
        (see Sequencing note); paste that output here. The current gate
        returned `Ok(())` for `Interface: none`, so the command exited 0 and
        the `Then` failed — the recorded run exited non-zero. Do not re-run
        now: the branch exists and the scenario would be green.
        ```
        <!-- paste the recorded targeted run output showing exit 0 from the gate and non-zero test exit -->
        ```
  - [x] **GREEN.** Apply the doc comment and the two error strings; add the
        two named unit tests. Files created/modified: `src/integrity_hook.rs`
        (production text + tests), plus this scenario's `@wip` removal. Run the
        unit tests, then the targeted scenario. Both must exit 0.
        ```
        cargo test --locked integrity_hook::tests::interface_none_requires_confirmed_persona
        cargo test --locked integrity_hook::tests::interface_none_default_requires_persona
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          cargo test --locked --test features_runner -- \
          --name "A change that declares the interface none without a persona disposition fails review"
        ```
        ```
        <!-- paste unit and targeted run output, exit 0 -->
        ```
  - [x] **REFACTOR.** No behavior change. Re-run the two unit tests and the
        targeted scenario; must still exit 0.
        ```
        <!-- paste re-run output, exit 0 -->
        ```
  - [x] **COMMIT.** One commit containing the error strings, the unit tests,
        and this scenario's `@wip` removal. Body:
        `A change that declares the interface none without a persona disposition fails review`.
        Record exactly one checked COMMIT line with the real hash.
  - [x] COMMIT: `4f55484d6bdfeb19a1f02c3522af35a078548591`

## Scenario 3 — A change that declares the interface none with a reasoned none fails review

`@givn.added` — a reasoned none still fails for a declared contract and the
offending value is reported.

Design constraints:

- Behavior row: `PERSONAS: none — <reason>` never satisfies a declared
  contract; the value does not resolve to `givn/personas/<value>.md`.
- Fixture: reuse `the change's review.md declares the persona disposition
  "none — operator confirmed"` (writes `VISUAL-REVIEW: PASS` and the value).
- Assertions: command exits non-zero; stderr contains
  `none — operator confirmed`.
- Production (text only): error string #3 (value resolves to no file):
  `the persona disposition '{value}' matches no file in givn/personas/ — name
  a confirmed Persona; a reasoned none is valid only when the design declares
  no visual design contract`.
- Unit test: extend `interface_none_requires_confirmed_persona` (added in
  scenario 2) so the reasoned-none failure also asserts the message contains
  `a reasoned none is valid only when the design declares no visual design
  contract`.
- Single-scenario command:
  ```
  GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
    cargo test --locked --test features_runner -- \
    --name "A change that declares the interface none with a reasoned none fails review"
  ```

- [x] Scenario: A change that declares the interface none with a reasoned none fails review
  - [x] **RED.** Remove only `@wip` from this scenario. The named run was
        executed before scenario 1's GREEN, while `@wip` was still present
        (see Sequencing note); paste that output here. The current gate
        returned `Ok(())` for `Interface: none`, so the command exited 0 and
        the `Then` failed — the recorded run exited non-zero. Do not re-run
        now: the branch exists and the scenario would be green.
        ```
        <!-- paste the recorded targeted run output showing exit 0 from the gate and non-zero test exit -->
        ```
  - [x] **GREEN.** Apply error string #3 and the unit-test message assertion.
        Files created/modified: `src/integrity_hook.rs` (production text +
        test), plus this scenario's `@wip` removal. Run the unit test, then the
        targeted scenario. Both must exit 0.
        ```
        cargo test --locked integrity_hook::tests::interface_none_requires_confirmed_persona
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          cargo test --locked --test features_runner -- \
          --name "A change that declares the interface none with a reasoned none fails review"
        ```
        ```
        <!-- paste unit and targeted run output, exit 0 -->
        ```
  - [x] **REFACTOR.** No behavior change. Re-run the unit test and the targeted
        scenario; must still exit 0.
        ```
        <!-- paste re-run output, exit 0 -->
        ```
  - [x] **COMMIT.** One commit containing the error string, the test
        assertion, and this scenario's `@wip` removal. Body:
        `A change that declares the interface none with a reasoned none fails review`.
        Record exactly one checked COMMIT line with the real hash.
  - [x] COMMIT: `c5fc4e7983b0c8c1c4fede2c126ed43f60fd32d7`

## Scenario 4 — Review guidance requires a persona for a declared interface none contract

`@givn.added` — the served review instruction must state the contract-based
rule. No new `@e2e`.

Design constraints:

- Assertions: `givn instructions review --change visual-review-gate` stdout
  contains `Interface: none` and `declared visual design contract`.
- Review instruction (`assets/instructions/review.md`), conformance paragraph:
  keep `Record the disposition for every confirmed Persona; when none is
  confirmed, record the reason.` and replace the surface sentence with:
  "A declared visual design contract — including `Interface: none` — requires
  at least one confirmed Persona; only a change that declares no visual design
  contract may record a reasoned `none`."
  The `## Visual Review` block stays conditional on a non-`none` interface.
- Design instruction (`assets/instructions/design.md`): `Interface: none` is a
  declared contract; the section's presence, not the rendered screen, triggers
  the Persona requirement. Keep the `user-visible`, `CLI text`, and `reason`
  sentences.
- Design-review instruction (`assets/instructions/design-review.md`): the
  Persona disposition follows the declared contract, including
  `Interface: none`; only a no-contract design records a reasoned none. Keep
  `per persona`, `relevance and evidence`, and `operator confirmation`.
- Proposal instruction (`assets/instructions/proposal.md`): any change that
  will declare a contract, including `Interface: none`, names at least one
  confirmed Persona. Keep `user-visible`, `CLI text`, `operator confirmation`,
  and `reason`.
- Templates:
  - `assets/templates/design.md`: distinguish "no contract, omit the section"
    from "`Interface: none`, Persona still required". Keep `Visual Design
    Contract`, `Interface:`, `States:`, `Breakpoints`, `Accessibility`.
  - `assets/templates/design-review.md`: Persona disposition note follows the
    contract-based rule. Keep `persona`, `Relevance`, `Evidence`.
  - `assets/templates/review.md`: Persona note and `## Visual Review` N/A note
    follow the rule. Keep `PERSONAS:`, `DEFERRAL:`, `OPERATOR-DECISION:`,
    `VISUAL-REVIEW: PASS`, and the word `persona`.
- Durable docs already edited in the working tree; design-review verified they
  match design.md's affected-files table. Commit them here:
  `docs/arc42/05-building-block-view.md`, `06-runtime-view.md`,
  `08-crosscutting-concepts.md`, `09-architecture-decisions.md`,
  `10-quality-requirements.md`, `11-risks-and-technical-debt.md`,
  `12-glossary.md`, `README.md`.
- Surgical-edit rule: do not rewrite unrelated guidance prose. The permanent
  corpus asserts these phrases — every one must survive:
  `Visual Design Contract`, `Interface:`, `States:`, `Breakpoints`,
  `Accessibility`, `style-level`, `not pixel`, `docs/design/`, `design tokens`,
  `component states`, `rendered reference`, `improvement`,
  `updates the contract`, `Samsung S24`, `360x780`, `Full HD`, `1920x1080`,
  `renders HTML, CSS, or JavaScript in a browser`,
  `even when it needs only a page request`, `no controls`,
  `default browser driver`, `Playwright`, `does not pin`, `is a Web UI row`,
  `needs a browser driver`, `user-visible`, `CLI text`, `reason`,
  `operator confirmation`, `opposite classification`, `rendered in a browser`,
  `must say why`, `propose`, `user approves`, `rendered options`, `per persona`,
  `relevance and evidence`, `operator question`, `mandatory check`,
  `do not sign off`, `persona`, `Relevance`, `Evidence`, `PERSONAS:`,
  `DEFERRAL:`, `OPERATOR-DECISION:`, `VISUAL-REVIEW: PASS`,
  `confirmed Persona`, `disposition for every confirmed Persona`,
  `at least one confirmed Persona`, `record the reason`, `no pixel-level`,
  `state screenshots the review saw`, `each declared UI state`, `vision`,
  `transcript`.
- Single-scenario command:
  ```
  GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
    cargo test --locked --test features_runner -- \
    --name "Review guidance requires a persona for a declared interface none contract"
  ```

- [x] Scenario: Review guidance requires a persona for a declared interface none contract
  - [x] **RED.** Remove only `@wip` from this scenario. The served instruction
        still says a user-visible surface requires a Persona, so the targeted
        run fails on `stdout missing "Interface: none"`. Must exit non-zero.
        ```
        <!-- paste the targeted run output showing the missing-string failure and non-zero exit -->
        ```
  - [x] **GREEN.** Apply the guidance, template, and arc42 edits. Files
        created/modified: `assets/instructions/review.md`,
        `assets/instructions/design.md`, `assets/instructions/design-review.md`,
        `assets/instructions/proposal.md`, `assets/templates/design.md`,
        `assets/templates/design-review.md`, `assets/templates/review.md`, the
        eight `docs/arc42/` files above, plus this scenario's `@wip` removal.
        Run the targeted scenario; must exit 0.
        ```
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          cargo test --locked --test features_runner -- \
          --name "Review guidance requires a persona for a declared interface none contract"
        ```
        ```
        <!-- paste targeted run output, exit 0 -->
        ```
  - [x] **REFACTOR.** Preserve behavior; re-run the targeted scenario, the full
        delta scope, and the permanent non-e2e suite. All must exit 0. The
        permanent run is required because the guidance assertions live in
        `givn/specs/create-change/visual-design-contract.feature` and
        `givn/specs/verify-change/visual-review-gate.feature`.
        ```
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          cargo test --locked --test features_runner -- \
          --name "Review guidance requires a persona for a declared interface none contract"
        GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
          ./run-tests.sh --non-e2e
        ./run-tests.sh --non-e2e
        ```
        ```
        <!-- paste three summaries, all exit 0 -->
        ```
  - [x] **COMMIT.** One commit containing the guidance, templates, arc42 docs,
        and this scenario's `@wip` removal. Body:
        `Review guidance requires a persona for a declared interface none contract`.
        Record exactly one checked COMMIT line with the real hash.
  - [x] COMMIT: `6c0e7bef6b00bf470ce5aa7ea8110a33c1300952`

---

## Final verification (no scenario commit)

- [x] Full delta scope exits 0 and all four scenarios report passed, none
      skipped:
      ```
      GIVN_FEATURES=givn/changes/require-persona-for-interface-none/specs \
        ./run-tests.sh --non-e2e
      ```
      ```
      <!-- paste scenario/pass/skip counts and exit 0 -->
      ```
- [x] `verify.command` exits 0 on the permanent non-e2e corpus:
      ```
      ./run-tests.sh --non-e2e
      ```
      ```
      <!-- paste scenario/pass/skip counts and exit 0 -->
      ```
- [x] `verify.e2e_command` exits 0 on the permanent `@e2e` corpus. No delta
      `@e2e` scenario exists, so this scope is unchanged; record its count
      against the non-e2e count to show the tag filter is real:
      ```
      ./run-tests.sh --e2e
      ```
      ```
      <!-- paste e2e scenario/pass/skip counts, exit 0, and the non-e2e count -->
      ```
- [x] `givn lint --change require-persona-for-interface-none` reports no
      `@wip` finding and exits 0; shape/subset advisories do not affect the
      exit code. Paste the result.
- [x] `givn status --change require-persona-for-interface-none` shows `tasks`
      done and `review` next.

**Done when:** all four scenario boxes are checked with pasted RED/GREEN/
REFACTOR evidence and exactly one real reachable checked COMMIT line each,
the delta scope and both verification commands exit 0, and no `@wip` remains.
