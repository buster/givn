# Tasks: add-ideation-skill

This change ships a new synchronized facilitation skill. Implementation is
incremental template growth: each scenario's GREEN adds exactly the contract
phrases that scenario asserts, so every later scenario has a genuine failing
RED. All steps bind to existing definitions; the one shared helper change
(`target_skill_paths` in `tests/steps/prose_consolidation_steps.rs`, already
applied during design-review hardening) is committed with the first scenario
it unblocks. Command-side registration is deliberately deferred to the
delivery `@e2e` scenario's GREEN so that scenario keeps a genuine RED on the
missing generated command file.

Runner contract (existing, unchanged): `./run-tests.sh`, `./run-tests.sh --e2e`,
strict mode `.fail_on_skipped()` at `tests/features_runner.rs:20`.

## Setup

- [x] **Setup**: prove strict mode with an undefined-step probe, confirm the
      runner contract, and prove the e2e subset.
      ```text
      GIVN_FEATURES=tests/strict-mode-ideation-probe.feature cargo test --test features_runner -- --name '^Undefined ideation step fails the runner$'
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      Step doesn't match any function
      error: test failed, to rerun `--test features_runner`
      (exit 101 — strict mode proven; probe file deleted after the proof)
      ```
      ```text
      GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh       → 1 feature; 6 scenarios (6 failed); exit 101
      GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh --e2e → 1 feature; 1 scenario (1 failed); exit 101
      1 < 6 proves the @e2e tag filter is a strict subset; the failures are the expected pre-GREEN RED state
      ```
- [x] **Planning-layer commit**: commit the planning artifacts before the
      first scenario (proposal, spec delta with `@wip` tags, design,
      design-review, arc42 assessment, glossary terms, support-group
      inventory bullet, this tasks.md). Never stage unrelated working-tree
      files (`PLAN.md`, `TASKS.md`).
      Commit: `docs(add-ideation-skill): planning layer through design-review`
      Hash: `5139777`

## Non-E2E Scenarios

- [x] Scenario: `The ideation skill ships without any addon enabled`
  - [x] RED: remove `@wip` from this scenario only. Run:
        ```text
        GIVN_FEATURES=givn/changes/add-ideation-skill/specs cargo test --test features_runner -- --name '^The ideation skill ships without any addon enabled$'
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)
        thread 'main' panicked: 1 step failed
        error: test failed, to rerun `--test features_runner`
        (exit 101 — guidance step fails on the missing generated skill)
        ```
  - [x] GREEN: created `assets/skills/givn-ideation/SKILL.md.tmpl` (frontmatter
        `name: givn-ideation` + one-sentence description, H1, When-to-use
        positioning vs `givn-explore`, a Frame section heading), registered
        `"givn-ideation"` in `default_skill_names()` (`src/skills/render.rs`).
        Committed the helper fix `tests/steps/prose_consolidation_steps.rs`
        (real targets parsing) with this scenario. Production files:
        `assets/skills/givn-ideation/SKILL.md.tmpl`, `src/skills/render.rs`,
        `tests/steps/prose_consolidation_steps.rs`.
        ```text
        GIVN_FEATURES=givn/changes/add-ideation-skill/specs cargo test --test features_runner -- --name '^The ideation skill ships without any addon enabled$'
        ✔  Then the generated givn-ideation guidance contains "Frame"
        1 scenario (1 passed)
        3 steps (3 passed)
        (exit 0)
        ```
  - [x] REFACTOR: template already matches sibling-skill structure and tone;
        helper fix was applied during design-review hardening; no behaviour
        change. Rerun targeted:
        ```text
        1 scenario (1 passed)
        3 steps (3 passed)
        ```
  - [x] COMMIT: `feat(ideation-skill): The ideation skill ships without any addon enabled`
        Hash: `34ba5ba`

- [ ] Scenario: `The ideation guidance carries the phase ladder and the one-question rule`
  - [ ] RED: remove `@wip`. Targeted run — must fail (phrases absent; paste
        output)
  - [ ] GREEN: extend `assets/skills/givn-ideation/SKILL.md.tmpl` with the
        phase sections: `Event Storming`, `Use Case Definition`, `Use Case
        Elaboration`, `Handoff` (Frame already exists), the one-question
        interaction rule, and a quality review paragraph containing
        `quality review`. Production files: the template only.
        Targeted rerun: `<paste>`
  - [ ] REFACTOR: `<paste>`
  - [ ] COMMIT: `feat(ideation-skill): The ideation guidance carries the phase ladder and the one-question rule`
        Hash: `<fill>`

- [ ] Scenario: `The ideation guidance binds personas to the artifacts that affect them`
  - [ ] RED: remove `@wip`. Targeted run — must fail (paste output)
  - [ ] GREEN: extend the template with the persona sections: creation after
        Frame, `stakeholder type`, `persona segment`, living persona record,
        the affected-artifact rule (`every artifact that affects`), value
        ranking at Use Case Definition, critique at Use Case Elaboration,
        `persona promotion` at Handoff, user-confirmation authority.
        Production files: the template only. Targeted rerun: `<paste>`
  - [ ] REFACTOR: `<paste>`
  - [ ] COMMIT: `feat(ideation-skill): The ideation guidance binds personas to the artifacts that affect them`
        Hash: `<fill>`

- [ ] Scenario: `The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes`
  - [ ] RED: remove `@wip`. Targeted run — must fail (paste output)
  - [ ] GREEN: extend the template with the topic state layout
        (`givn/ideation/` root, `session.md` resume, open-question parking,
        change seed files) and the handoff section (publish to
        `docs/ideation/`, glossary merge proposal, persona promotion offer,
        start-order suggestion) containing the literal sentence `Handoff
        never starts a change`. Production files: the template only.
        Targeted rerun: `<paste>`
  - [ ] REFACTOR: `<paste>`
  - [ ] COMMIT: `feat(ideation-skill): The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes`
        Hash: `<fill>`

- [ ] Scenario: `The explore guidance points to ideation for structured domain work`
  - [ ] RED: remove `@wip`. Targeted run — must fail (paste output)
  - [ ] GREEN: add the transition row to
        `assets/skills/givn-explore/SKILL.md.tmpl` (structured domain work
        with artifacts and multiple changes → `/givn-ideation`) and the
        offer line in `assets/commands/givn-explore.md` ("When no change
        exists"). Production files: both explore assets. Targeted rerun:
        `<paste>`
  - [ ] REFACTOR: `<paste>`
  - [ ] COMMIT: `feat(ideation-skill): The explore guidance points to ideation for structured domain work`
        Hash: `<fill>`

## E2E Scenarios

- [ ] E2E setup: local runnability is the CLI subprocess in a temp project
      (no server, browser, database, or digital twins); the runner contract
      is `./run-tests.sh --e2e` with the `@e2e` tag filter (subset proof
      recorded in Setup). No additional infrastructure.

- [ ] Scenario: `A consumer synchronizes skills and every agent target receives the ideation facilitation skill`
  - [ ] RED: remove `@wip`. Targeted run — must fail on the missing
        generated command file (`.opencode/commands/givn-ideation.md` does
        not exist because command registration is still deferred; paste
        output)
  - [ ] GREEN: create `assets/commands/givn-ideation.md` (description
        frontmatter, argument note, pointer into the skill, givn awareness,
        guardrails summary) and register `"givn-ideation"` in
        `default_command_names()` (`src/skills/targets.rs`). Production
        files: both. Targeted rerun via
        `GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh --e2e`:
        must pass all three targets plus the command file; paste output
  - [ ] REFACTOR: `<paste>`
  - [ ] COMMIT: `test(e2e): A consumer synchronizes skills and every agent target receives the ideation facilitation skill`
        Hash: `<fill>`

## Final verification

- [ ] Full change suite green:
      ```text
      GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh      → exit 0 (6/6)
      GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh --e2e → exit 0 (1/1)
      (paste outputs)
      ```
- [ ] `givn lint --change add-ideation-skill` → exit 0, no `@wip` findings;
      pre-planned dispositions for the intra-delta SHAPE/SUBST findings
      recorded for the review phase (VALID-BOUNDARY, differing assertion
      sets).
- [ ] `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`
      clean.
- [ ] Unit RED anchor now satisfied: `cargo test --lib` pairing test
      `all_command_templates_are_embedded` passes (command registration +
      asset present).
- [ ] Self-dogfood per AGENTS.md: `cargo install --path . --locked --force`;
      `givn skills sync`; `givn instructions`; `./run-tests.sh`;
      `./run-tests.sh --e2e`. Installed `$HOME/.cargo/bin/givn` is the
      current checkout's binary.