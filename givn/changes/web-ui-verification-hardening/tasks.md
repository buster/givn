# Tasks: web-ui-verification-hardening

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/web-ui-verification-hardening/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `author-change`: generated guidance is synchronized from canonical source
  assets; the change scaffold and its guidance must stay consistent.
- `verify-change`: an invalid change cannot pass the verification gate; a
  missing review disposition blocks completion.
- Both use cases declare `Personas: none`. No Persona is invented or promoted;
  actors stay interaction roles. The `none` disposition is flagged for
  post-change revision (assumption A12).
- A mandatory check is never silently deferred: the operator question and its
  recorded decision precede any deferral.
- A browser-rendered capability is a Web UI and needs a browser driver; an
  HTTP client is not a valid driver.

Capabilities and step files:

- `prose-consolidation` — reused steps in `tests/steps/prose_consolidation_steps.rs` and `tests/steps/mod.rs`
- `visual-design-contract` — `tests/steps/visual_design_contract_steps.rs` plus reused steps
- `visual-review-gate` — `tests/steps/visual_review_gate_steps.rs` plus the shared `review_md_contains` fixture

Positive/negative pairing (assumption A10): the positive gate scenarios
(`A web contract with complete state evidence passes review`, `A visual change
with a confirmed persona disposition passes review`, `A deferral with an
operator decision passes review`) are proven in the same task as their
negative counterpart; the negative scenario is the RED driver.

## Setup

- [x] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [x] Proof-of-strictness: wrote a temporary `tests/strict-mode-probe.feature`
      with one undefined step, ran it, confirmed non-zero exit, then deleted
      the probe. Evidence:
      ```
      GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --test features_runner
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `--test features_runner`
      exit status: 1
      ```
- [x] Created the step skeletons in `tests/steps/visual_review_gate_steps.rs`
      and `tests/steps/visual_design_contract_steps.rs` with `unimplemented!()`
      bodies for the new steps; `cargo check --locked --tests` succeeds.
      Evidence:
      ```
      Checking givn v0.7.0 ($HOME/projects/givn)
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.72s
      ```
- [x] COMMIT: `3c62d3be` -- `test(web-ui-verification-hardening): strict-mode proof and step skeletons`

## Scenarios

### prose-consolidation

- [x] Scenario: `Canonical policy classifies a browser-rendered page as a Web UI`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        stdout missing "renders HTML, CSS, or JavaScript in a browser"
        exit status: 1
        ```
  - [x] GREEN: added the fail-closed classification to
        `assets/instructions/specs.md` (unwrapped sentences so the asserted
        phrases stay on one line; `cargo clean -p givn` before the run because
        `rust_embed` does not track `assets/`). Files:
        `assets/instructions/specs.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `e1d2a2f3` -- `feat(prose-consolidation): Canonical policy classifies a browser-rendered page as a Web UI`

- [x] Scenario: `Canonical policy requires a rendered-state assertion`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "rendered state"
        exit status: 1
        ```
  - [x] GREEN: added the rendered-state assertion rule to
        `assets/instructions/specs.md` (unwrapped sentences). Files:
        `assets/instructions/specs.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `4c944227` -- `feat(prose-consolidation): Canonical policy requires a rendered-state assertion`

- [x] Scenario: `Canonical policy requires evidence for every declared UI state`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "declares UI states"
        exit status: 1
        ```
  - [x] GREEN: added the per-state evidence rule to
        `assets/instructions/specs.md`. Files: `assets/instructions/specs.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `1331246f` -- `feat(prose-consolidation): Canonical policy requires evidence for every declared UI state`

- [x] Scenario: `Canonical policy refuses a silent deferral of a mandatory check`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "never deferred"
        exit status: 1
        ```
  - [x] GREEN: added the no-silent-deferral rule to
        `assets/instructions/specs.md`. Files: `assets/instructions/specs.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `49286197` -- `feat(prose-consolidation): Canonical policy refuses a silent deferral of a mandatory check`

### visual-design-contract

- [x] Scenario: `Design guidance classifies a browser-rendered screen as a Web UI`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "renders HTML, CSS, or JavaScript in a browser"
        exit status: 1
        ```
  - [x] GREEN: added the fail-closed classification to
        `assets/instructions/design.md`. Files: `assets/instructions/design.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `b82fe9b5` -- `feat(visual-design-contract): Design guidance classifies a browser-rendered screen as a Web UI`

- [x] Scenario: `Design guidance names the default browser driver`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "default browser driver"
        exit status: 1
        ```
  - [x] GREEN: added the Playwright default and the per-change version
        resolution to `assets/instructions/design.md`. Files:
        `assets/instructions/design.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `5536e8c0` -- `feat(visual-design-contract): Design guidance names the default browser driver`

- [x] Scenario: `Design guidance requires breakpoints and accessibility`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "Breakpoints"
        exit status: 1
        ```
  - [x] GREEN: added the breakpoint and accessibility bullets to
        `assets/instructions/design.md`. Files: `assets/instructions/design.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        ```
  - [x] COMMIT: `a9385537` -- `feat(visual-design-contract): Design guidance requires breakpoints and accessibility`

- [x] Scenario: `Design guidance requires a driver for every Web UI matrix row`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "is a Web UI row"
        exit status: 1
        ```
  - [x] GREEN: linked classification and driver in the matrix rule in
        `assets/instructions/design.md`. Files: `assets/instructions/design.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        ```
  - [x] COMMIT: `0a4360ff` -- `feat(visual-design-contract): Design guidance requires a driver for every Web UI matrix row`

- [x] Scenario: `Design guidance names the user-visible persona boundary`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "user-visible"
        exit status: 1
        ```
  - [x] GREEN: added the user-visible persona boundary to
        `assets/instructions/design.md`. Files: `assets/instructions/design.md`.
        Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `54a2e9a7` -- `feat(visual-design-contract): Design guidance names the user-visible persona boundary`

- [x] Scenario: `Design-review guidance tests the opposite interface classification`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "opposite classification"
        exit status: 1
        ```
  - [x] GREEN: added the opposite-classification branch to
        `assets/instructions/design-review.md`. Files:
        `assets/instructions/design-review.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `d7fa4b6a` -- `feat(visual-design-contract): Design-review guidance tests the opposite interface classification`

- [x] Scenario: `Design-review guidance records the persona disposition`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "per persona"
        exit status: 1
        ```
  - [x] GREEN: added the per-persona relevance and evidence rule to
        `assets/instructions/design-review.md`. Files:
        `assets/instructions/design-review.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `cc8bc2a4` -- `feat(visual-design-contract): Design-review guidance records the persona disposition`

- [x] Scenario: `Design-review guidance requires an operator question before a deferral`
  - [x] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        stdout missing "operator question"
        exit status: 1
        ```
  - [x] GREEN: added the deferral branch to
        `assets/instructions/design-review.md`. Files:
        `assets/instructions/design-review.md`. Run → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        ```
  - [x] COMMIT: `3b3741df` -- `feat(visual-design-contract): Design-review guidance requires an operator question before a deferral`

- [ ] Scenario: `Proposal guidance requires confirmed personas or a reasoned none`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the persona confirmation rule to
        `assets/instructions/proposal.md` and
        `assets/skills/givn-propose/SKILL.md.tmpl`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): Proposal guidance requires confirmed personas or a reasoned none`

- [ ] Scenario: `The design template carries the state, breakpoint, and accessibility lines`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the `States:` line and the breakpoint and accessibility
        bullets to `assets/templates/design.md`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): The design template carries the state, breakpoint, and accessibility lines`

- [ ] Scenario: `The proposal template carries the persona confirmation line`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: updated the `Confirmed Personas` line in
        `assets/templates/proposal.md`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): The proposal template carries the persona confirmation line`

- [ ] Scenario: `The review template carries the persona disposition and deferral markers`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the fenced `PERSONAS:`, `DEFERRAL:`, and
        `OPERATOR-DECISION:` scaffold to `assets/templates/review.md`. Run →
        zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): The review template carries the persona disposition and deferral markers`

- [ ] Scenario: `The design-review template carries the persona disposition table`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the Relevance and Evidence columns to the persona table
        in `assets/templates/design-review.md`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): The design-review template carries the persona disposition table`

- [ ] Scenario: `Synchronized authoring guidance carries the classification, driver, and persona rules`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: mirrored the phrases in the synchronized skill templates
        (`givn-design`, `givn-design-review`, `givn-propose`) and verified the
        generated output. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-design-contract): Synchronized authoring guidance carries the classification, driver, and persona rules`

### visual-review-gate

- [ ] Scenario: `Review guidance tests the opposite interface classification`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the opposite-classification check to
        `assets/instructions/review.md` and
        `assets/skills/givn-review/SKILL.md.tmpl`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): Review guidance tests the opposite interface classification`

- [ ] Scenario: `Review guidance fails a deferral without an operator decision`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the deferral rule to `assets/instructions/review.md` and
        `assets/skills/givn-review/SKILL.md.tmpl`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): Review guidance fails a deferral without an operator decision`

- [ ] Scenario: `Review guidance records the persona disposition`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the persona disposition rule to
        `assets/instructions/review.md` and
        `assets/skills/givn-review/SKILL.md.tmpl`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): Review guidance records the persona disposition`

- [ ] Scenario: `Review guidance names the state screenshots the vision review saw`
  - [ ] RED: removed `@wip` for this scenario only; run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the state-screenshot naming rule to
        `assets/instructions/review.md`, `assets/skills/givn-review/SKILL.md.tmpl`,
        and `assets/skills/givn-steps/SKILL.md.tmpl`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): Review guidance names the state screenshots the vision review saw`

- [ ] Scenario: `A web contract with missing state evidence fails review`
  - [ ] RED: removed `@wip` for this scenario only; added the fixture steps
        (declared UI states, state screenshots) with `unimplemented!()`. Run →
        non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: implemented `- States:` parsing and the state-evidence check in
        `src/integrity_hook.rs`; implemented the fixture steps. Files:
        `src/integrity_hook.rs`, `tests/steps/visual_review_gate_steps.rs`.
        Run → zero (the scenario expects non-zero from `givn check review`,
        which the step asserts). Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A web contract with missing state evidence fails review`

- [ ] Scenario: `A web contract with complete state evidence passes review`
  - [ ] RED: paired with the negative scenario above; the positive scenario
        failed before the state check existed only as a fixture gap, so RED is
        recorded together with the negative. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: added the state screenshots fixture and confirmed exit 0 with
        the implemented check. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A web contract with complete state evidence passes review`

- [ ] Scenario: `A visual change without a persona disposition fails review`
  - [ ] RED: removed `@wip` for this scenario only; added the
        no-persona fixture with `unimplemented!()`. Run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: implemented the persona-disposition check in
        `src/integrity_hook.rs` and updated the shared `review_md_contains`
        fixture with `PERSONAS: fixture-persona`. Files:
        `src/integrity_hook.rs`, `tests/steps/visual_review_gate_steps.rs`.
        Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A visual change without a persona disposition fails review`

- [ ] Scenario: `A visual change with a confirmed persona disposition passes review`
  - [ ] RED: paired with the negative scenario above. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: confirmed exit 0 with the fixture's `PERSONAS: fixture-persona`.
        Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A visual change with a confirmed persona disposition passes review`

- [ ] Scenario: `A visual interface with an empty persona disposition fails review`
  - [ ] RED: removed `@wip` for this scenario only; the empty-value fixture
        step is new. Run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: the persona check rejects an empty value. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A visual interface with an empty persona disposition fails review`

- [ ] Scenario: `An empty state screenshot fails review`
  - [ ] RED: removed `@wip` for this scenario only; added the empty-state
        fixture step with `unimplemented!()`. Run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: the state-evidence check counts a zero-byte file as missing.
        Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): An empty state screenshot fails review`

- [ ] Scenario: `A deferral without an operator decision fails review`
  - [ ] RED: removed `@wip` for this scenario only; added the `also contains`
        fixture step with `unimplemented!()`. Run → non-zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: implemented `check_deferrals` in `src/integrity_hook.rs`,
        wired into `run_integrity_hook`, with fenced-block stripping. Files:
        `src/integrity_hook.rs`. Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A deferral without an operator decision fails review`

- [ ] Scenario: `A deferral with an operator decision passes review`
  - [ ] RED: paired with the negative scenario above. Evidence:
        ```
        <paste output>
        ```
  - [ ] GREEN: confirmed exit 0 with the paired `OPERATOR-DECISION:` line.
        Run → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] REFACTOR: re-ran → zero. Evidence:
        ```
        <paste output>
        ```
  - [ ] COMMIT: `<hash>` -- `feat(visual-review-gate): A deferral with an operator decision passes review`

## Finalization

- [ ] Remove every `@wip` tag from the delta features and confirm
      `givn lint --change web-ui-verification-hardening` exits 0.
- [ ] Run the full regular suite and the e2e suite; paste the summaries.
- [ ] Sync the generated agent integrations (`givn skills sync`) and commit
      the regenerated trees.
- [ ] Update `assumptions.md` with any new assumption discovered during
      implementation.
