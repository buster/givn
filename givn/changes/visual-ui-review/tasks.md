# Tasks: visual-ui-review

Runner: `./run-tests.sh` (regular) / `./run-tests.sh --e2e` (e2e). Strict mode:
`tests/features_runner.rs` uses `.fail_on_skipped()` (cucumber-rs) — undefined
steps fail. Not-implemented stub: `unimplemented!()`.

Single-scenario run command (all scenarios):

```
GIVN_FEATURES=givn/changes/visual-ui-review/specs cargo test --test features_runner -- --name "<scenario title>"
```

Use-case constraints carried into every scenario task:

- `author-change`: generated guidance is synchronized from canonical source
  assets; the change scaffold and its guidance must stay consistent.
- `verify-change`: an invalid change cannot pass the verification gate; a
  missing review disposition blocks completion.
- Both use cases declare `Personas: none`. No Persona is invented or promoted;
  actors stay interaction roles.

Capabilities and step files:

- `prose-consolidation` — reused steps in `tests/steps/prose_consolidation_steps.rs`
- `visual-design-contract` — `tests/steps/visual_design_contract_steps.rs` plus reused steps
- `visual-review-gate` — `tests/steps/visual_review_gate_steps.rs`

## Setup

- [x] Confirm strict mode and the runner: `tests/features_runner.rs` uses
      `.fail_on_skipped()`; `verify.command` is `./run-tests.sh` and
      `verify.e2e_command` is `./run-tests.sh --e2e`. No new runner is needed.
- [x] Proof-of-strictness: wrote a temporary `tests/strict-mode-probe.feature`
      with one undefined step and ran
      `GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --test features_runner`.
      Non-zero output, then the probe was deleted.
      Evidence:
      ```
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `--test features_runner`
      exit status: 1
      ```
- [x] Created the step skeletons `tests/steps/visual_review_gate_steps.rs` and
      `tests/steps/visual_design_contract_steps.rs` with `unimplemented!()`
      bodies for the new steps; `cargo check --locked --tests` succeeds.
      Evidence:
      ```
      Checking givn v0.7.0
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.38s
      ```
- [x] COMMIT: `d9e57752` -- `test(visual-ui-review): strict-mode proof and step skeletons`

## Scenarios

### prose-consolidation

- [x] Scenario: `Canonical policy requires the visual baseline at every assertion`
  - [x] RED: removed `@wip` from this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the visual baseline list to
        `assets/instructions/specs.md` (visible label, internal identifier,
        no visible content is duplicated, way back, configured language,
        reduced-motion). Files: `assets/instructions/specs.md`. Run → zero.
        Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] REFACTOR: re-ran → zero. Evidence:
        ```
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        ```
  - [x] COMMIT: `c9d3c4f1` -- `Canonical policy requires the visual baseline at every assertion`

- [x] Scenario: `Canonical policy uses transcripts for terminal interfaces`
  - [x] RED: removed `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the terminal transcript rule and the API-only exemption
        to `assets/instructions/specs.md`. Run → zero. Evidence:
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
  - [x] COMMIT: `2de63e96` -- `feat(prose-consolidation): Canonical policy uses transcripts for terminal interfaces`

### visual-design-contract

- [x] Scenario: `Design guidance names the default viewports`
  - [x] RED: removed `@wip` for this scenario only; ran the single-scenario
        command → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the default viewport dimensions (Samsung S24 class,
        360x780; Full HD, 1920x1080) to `assets/instructions/design.md`.
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
  - [x] COMMIT: `420fe734` -- `feat(visual-design-contract): Design guidance names the default viewports`

- [x] Scenario: `Design guidance keeps improvements inside the style`
  - [x] RED: removed `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        exit status: 1
        ```
  - [x] GREEN: added the improvement rule ("a style improvement that stays
        within the contract updates the contract, it is not an error") to
        `assets/instructions/design.md`. Run → zero. Evidence:
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
  - [x] COMMIT: `415e8047` -- `feat(visual-design-contract): Design guidance keeps improvements inside the style`

- [ ] Scenario: `Design-review guidance requires an approved contract`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the visual contract branch (propose rendered options,
        user approves, record the decision) to
        `assets/instructions/design-review.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Design-review guidance requires an approved contract`

- [ ] Scenario: `The design template carries the visual contract section`
  - [ ] RED: remove `@wip` for this scenario only; implement the ejected
        design template step as a stub. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the `## Visual Design Contract` section (with the
        `Interface:` line) to `assets/templates/design.md` and implement the
        `the ejected design template contains` step. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `The design template carries the visual contract section`

### visual-review-gate

- [ ] Scenario: `A visual change without a visual verdict fails review`
  - [ ] RED: remove `@wip` for this scenario only; implement the contract and
        review.md fixture steps as stubs. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implement `parse_visual_contract` and the verdict check in
        `src/integrity_hook.rs`, wired into `run_integrity_hook`, plus the
        fixture steps. Files: `src/integrity_hook.rs`,
        `tests/steps/visual_review_gate_steps.rs`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A visual change without a visual verdict fails review`

- [ ] Scenario: `A change without a visual contract is exempt from the visual gate`
  - [ ] RED: remove `@wip` for this scenario only; implement the
        "declares no visual design contract" step as a stub. Run → non-zero.
        Evidence:
        ```
        ```
  - [ ] GREEN: implement the absence exemption (no section → skip). Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A change without a visual contract is exempt from the visual gate`

- [ ] Scenario: `A change that declares no visual interface is exempt`
  - [ ] RED: remove `@wip` for this scenario only; implement the
        `Interface: none` step as a stub. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: parse `- Interface: none` as exempt. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A change that declares no visual interface is exempt`

- [ ] Scenario: `A web contract without viewports fails review`
  - [ ] RED: remove `@wip` for this scenario only; implement the
        no-viewports fixture step as a stub. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: fail a web contract with an empty viewport list, naming
        "viewports". Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A web contract without viewports fails review`

- [ ] Scenario: `A visual change with incomplete evidence fails review`
  - [ ] RED: remove `@wip` for this scenario only; implement the delta-spec
        and screenshot fixture steps as stubs. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: implement `scenario_slug`, `collect_e2e_scenario_titles`, and
        the per-viewport evidence check. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A visual change with incomplete evidence fails review`

- [ ] Scenario: `An empty evidence file fails review`
  - [ ] RED: remove `@wip` for this scenario only; implement the empty-file
        fixture step as a stub. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: treat a zero-byte evidence file as missing. Run → zero.
        Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `An empty evidence file fails review`

- [ ] Scenario: `An unverified visual verdict cannot pass review`
  - [ ] RED: remove `@wip` for this scenario only; implement the
        `VISUAL-REVIEW: UNVERIFIED` fixture step as a stub. Run → non-zero.
        Evidence:
        ```
        ```
  - [ ] GREEN: fail an unverified verdict with the human sign-off message,
        and fail when both markers are present. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `An unverified visual verdict cannot pass review`

- [ ] Scenario: `A terminal change passes review with a transcript`
  - [ ] RED: remove `@wip` for this scenario only; implement the terminal
        contract and transcript fixture steps as stubs. Run → non-zero.
        Evidence:
        ```
        ```
  - [ ] GREEN: implement terminal evidence as
        `<slug>/transcript.txt` and skip viewport checks. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A terminal change passes review with a transcript`

- [ ] Scenario: `Review guidance treats style improvements as contract updates`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the no-pixel-findings rule and the improvement-updates-
        contract rule to `assets/instructions/review.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Review guidance treats style improvements as contract updates`

- [ ] Scenario: `Review guidance records an unverified verdict without vision`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the unverified verdict and human sign-off rule to
        `assets/instructions/review.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Review guidance records an unverified verdict without vision`

- [ ] Scenario: `The review template carries the visual review section`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the `## Visual Review` section with the verdict markers
        and the persona lens to `assets/templates/review.md`. Run → zero.
        Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `The review template carries the visual review section`

## E2E scenarios

- [ ] Scenario: `Canonical E2E policy is emitted by specs instructions`
      (`@givn.modified`)
  - [ ] RED: remove `@wip` from this scenario only. Run → non-zero because the
        evidence rule is absent. Evidence:
        ```
        ```
  - [ ] GREEN: add the final-state screenshot rule and the viewport classes
        to `assets/instructions/specs.md`. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Canonical E2E policy is emitted by specs instructions`

- [ ] Scenario: `Author requests the visual design contract guidance`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the contract section, style-level/not-pixel rules, the
        design-system home, and the contract content (design tokens, component
        states, rendered reference) to `assets/instructions/design.md`. Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Author requests the visual design contract guidance`

- [ ] Scenario: `Synchronized guidance defines the contract approval and capture mechanics`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the contract approval branch to
        `assets/skills/givn-design-review/SKILL.md.tmpl` and the capture
        mechanics (evidence/visual, final-state, animations are frozen,
        reports/) to `assets/skills/givn-steps/SKILL.md.tmpl`. Run → zero.
        Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Synchronized guidance defines the contract approval and capture mechanics`

- [ ] Scenario: `A visual change with complete evidence passes review`
  - [ ] RED: remove `@wip` for this scenario only; the pass fixture is not yet
        green. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: complete the fixture (verdict, README-IMPACT, unique delta
        steps) and the gate path for a passing change. Run → zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `A visual change with complete evidence passes review`

- [ ] Scenario: `Review guidance requires the visual verdict and persona lenses`
  - [ ] RED: remove `@wip` for this scenario only. Run → non-zero. Evidence:
        ```
        ```
  - [ ] GREEN: add the `## Visual review` block (verdict, confirmed Persona
        vision review, transcript rule) to `assets/instructions/review.md` and
        the visual block to `assets/skills/givn-review/SKILL.md.tmpl`. Run →
        zero. Evidence:
        ```
        ```
  - [ ] REFACTOR: re-run → zero. Evidence:
        ```
        ```
  - [ ] COMMIT: `<hash>` -- `Review guidance requires the visual verdict and persona lenses`

## Verification

- [ ] `cargo check --locked` passes; `cargo clippy --workspace --all-targets -- -D warnings` passes.
- [ ] `givn lint --change visual-ui-review` exits 0 or 2 (no parse errors).
- [ ] `./run-tests.sh` GREEN with no `@wip` left in the delta.
- [ ] `./run-tests.sh --e2e` GREEN; e2e scope count strictly smaller than the regular scope.
- [ ] This change's own `design.md` parses as exempt (fence-aware parser unit test).
