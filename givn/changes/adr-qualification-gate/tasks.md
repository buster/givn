# Tasks: adr-qualification-gate

RED -> GREEN -> REFACTOR -> COMMIT per scenario. Non-@e2e scenarios first,
then the two CLI @e2e scenarios. Check each box immediately after its evidence
and commit hash are recorded.

## Strict-mode proof

- [x] Create and register the separate capability step modules named in
      `design.md`: `tests/steps/adr_qualification_gate_steps.rs` and
      `tests/steps/adr_qualification_gate_e2e_steps.rs`. Keep the new steps
      non-empty; unimplemented steps use `unimplemented!()`.
- [x] **Proof-of-strictness**: run the first scenario with its first step
      undefined or stubbed and confirm a non-zero exit:
      ```sh
      GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
        cargo test --features spec-retrieval --test features_runner -- \
        --name '^The guidance defines strict qualification as an all-gates contract$'
      ```
      Output: `1 feature`, `1 scenario (1 failed)`, `1 step (1 failed)`,
      `Step doesn't match any function`, followed by a panic and non-zero
      process exit. This proves that
      `GivnWorld::cucumber().fail_on_skipped()` rejects the undefined step.

## Scenario: The guidance defines strict qualification as an all-gates contract

- [x] RED: remove only this scenario's `@wip`, add the not-implemented step
      stubs, and run the anchored command from the strict-mode proof. Output:
      `1 feature`, `1 scenario (1 failed)`, `1 step (1 failed)`,
      `Step doesn't match any function`, followed by a panic and non-zero exit.
- [x] GREEN: assert that the canonical Arc42 instruction names all five
      mandatory dimensions and Boolean AND semantics. Production/source files:
      `assets/addons/arc42-docs/instructions.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output:
      `1 feature`, `1 scenario (1 passed)`, `8 steps (8 passed)`.
- [x] REFACTOR: simplify the source-guidance assertion without changing the
      contract; rerun the anchored scenario. Output:
      `1 feature`, `1 scenario (1 passed)`, `8 steps (8 passed)` after
      `cargo fmt --all`.
- [x] COMMIT: `c5ad76e` — `docs(arc42): The guidance defines strict qualification as an all-gates contract`

## Scenario: Shared guidance alone does not qualify an ADR

- [x] RED: remove only this scenario's `@wip`, add stubs, and run:
      ```sh
      GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
        cargo test --features spec-retrieval --test features_runner -- \
        --name '^Shared guidance alone does not qualify an ADR$'
      ```
      Output: `1 feature`, `1 scenario (1 failed)`, `2 steps (1 passed, 1 failed)`,
      `Step panicked. Captured output: not implemented`, and non-zero exit.
- [x] GREEN: assert support-only `Must be shared` and single canonical routing.
      Production/source files: `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      `1 feature`, `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: rerun the anchored scenario after cleanup. Output:
      `1 feature`, `1 scenario (1 passed)`, `5 steps (5 passed)` after
      `cargo fmt --all`.
- [ ] COMMIT: `<hash>` — `docs(arc42): Shared guidance alone does not qualify an ADR`

## Scenario: The guidance routes normal design decisions to one canonical artifact

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert routing to exactly one canonical feature, design, Arc42,
      process, project-documentation, or code artifact. Production/source files:
      `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The guidance routes normal design decisions to one canonical artifact`

## Scenario: The guidance requires an existing-ADR check before drafting

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert register search, the four route values, and the no-draft
      rule. Production/source files: `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The guidance requires an existing-ADR check before drafting`

## Scenario: The structured verdict has fixed gate and route fields

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert the five mandatory fields, support field, route fields,
      and allowed enum values in the canonical guidance. Production/source files:
      `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The structured verdict has fixed gate and route fields`

## Scenario: Route targets are conditional and unused targets stay empty

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert all conditional target/null rules for the four routes.
      Production/source files: `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): Route targets are conditional and unused targets stay empty`

## Scenario: The guidance forbids invented ADR evidence

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert supplied-evidence-only, empty-missing-evidence, and
      no-invention clauses. Production/source files:
      `assets/addons/arc42-docs/instructions.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The guidance forbids invented ADR evidence`

## Scenario: An ADR refinement is routed to amendment without duplication

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert `AMEND_ADR`, required `target_adr`, and the no-parallel-ADR
      rule. Production/source files: `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): An ADR refinement is routed to amendment without duplication`

## Scenario: An architectural replacement preserves supersession history

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert `SUPERSEDE_ADR`, the replacement target, and the explicit
      old-record relationship. Production/source files:
      `assets/addons/arc42-docs/instructions.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): An architectural replacement preserves supersession history`

## Scenario: Arc42 impact does not by itself require an ADR

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert the separate chapter-impact and ADR-routing rules.
      Production/source files: `assets/addons/arc42-docs/instructions.md` and
      `assets/addons/arc42-docs/template.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): Arc42 impact does not by itself require an ADR`

## Scenario: The change assessment records ADR disposition separately from chapter impact

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert `ADR action` and `ADR target` markers and the absence of a
      duplicated MADR procedure. Production/source files:
      `assets/addons/arc42-docs/template.md` and
      `givn/changes/adr-qualification-gate/arc42.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The change assessment records ADR disposition separately from chapter impact`

## Scenario: The ADR register has one canonical ownership contract

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert the register's pointer to the canonical procedure, the
      single-source ownership, and the rationale-versus-current-facts boundary.
      Production/source files: `docs/arc42/adr/README.md`,
      `docs/arc42/09-architecture-decisions.md`. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The ADR register has one canonical ownership contract`

## Scenario: The migration removes non-ADRs and preserves independent architecture history

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert the 35 accepted plus one proposed count, reserved IDs,
      removed non-ADRs, four absorbed refinements, three historical superseded
      records, removed Explore/audit ADRs, and untouched proposed ADR-063.
      Production/source files: `docs/arc42/adr/README.md`, retained ADR files,
      and deleted ADR files. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The migration removes non-ADRs and preserves independent architecture history`

## Scenario: All effective guidance sources use the same ADR policy

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: compare canonical and derived source assets, assert no competing
      legacy rule, verify generated mirrors, and verify a project override can
      remain effective. Production/source files: all ADR guidance sources named
      in `design.md` and generated mirrors. Test file:
      `tests/steps/adr_qualification_gate_steps.rs`. Output: _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): All effective guidance sources use the same ADR policy`

## Scenario: The ADR guidance requires an adversarial qualification record

- [ ] RED: remove only this scenario's `@wip`, add stubs, and run the anchored
      scenario command. Output: _pending_. Must exit non-zero.
- [ ] GREEN: assert real alternatives, supplied evidence, one canonical target,
      duplicate/amend/supersession review, and the checklist's evidence-only
      role. Production/source files: `assets/addons/arc42-docs/instructions.md`.
      Test file: `tests/steps/adr_qualification_gate_steps.rs`. Output:
      _pending_.
- [ ] REFACTOR: rerun the anchored scenario. Output: _pending_.
- [ ] COMMIT: `<hash>` — `docs(arc42): The ADR guidance requires an adversarial qualification record`

## E2E Smoke Tests

- [ ] E2E setup: verify `./run-tests.sh --e2e` is distinct from
      `./run-tests.sh`, uses the `@e2e` filters, and the capability-specific
      fixture creates an initialized project with active change
      `adr-qualification-gate`. Full/e2e scenario-count proof: _pending_.
- [ ] E2E strict proof: run each anchored E2E command with an unimplemented
      step and confirm non-zero exit before GREEN. Output: _pending_.

## Scenario: An author requests the strict ADR guidance through the Givn CLI

- [ ] RED: remove only this scenario's `@wip`, add E2E stubs, and run:
      ```sh
      GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
        cargo test --features spec-retrieval --test features_runner -- \
        --name '^An author requests the strict ADR guidance through the Givn CLI$'
      ```
      Output: _pending_. Must exit non-zero.
- [ ] GREEN: invoke the compiled `givn instructions arc42-docs --change
      adr-qualification-gate` subprocess and assert CLI stdout and exit status.
      Production files: none; E2E file:
      `tests/steps/adr_qualification_gate_e2e_steps.rs`. Output: _pending_.
- [ ] REFACTOR: clean up the E2E fixture without weakening the CLI assertion;
      rerun the anchored command. Output: _pending_.
- [ ] COMMIT: `<hash>` — `test(e2e): An author requests the strict ADR guidance through the Givn CLI`

## Scenario: A reviewer requests design-review guidance through the Givn CLI

- [ ] RED: remove only this scenario's `@wip`, add E2E stubs, and run:
      ```sh
      GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
        cargo test --features spec-retrieval --test features_runner -- \
        --name '^A reviewer requests design-review guidance through the Givn CLI$'
      ```
      Output: _pending_. Must exit non-zero.
- [ ] GREEN: invoke the compiled `givn instructions design-review --change
      adr-qualification-gate` subprocess and assert CLI stdout and exit status.
      Production files: none; E2E file:
      `tests/steps/adr_qualification_gate_e2e_steps.rs`. Output: _pending_.
- [ ] REFACTOR: clean up the E2E fixture without weakening the CLI assertion;
      rerun the anchored command. Output: _pending_.
- [ ] COMMIT: `<hash>` — `test(e2e): A reviewer requests design-review guidance through the Givn CLI`

## Final Verification

- [ ] `givn lint --change adr-qualification-gate` exits 0 with no WIP findings.
- [ ] `./run-tests.sh` passes for the full configured feature scope.
- [ ] `./run-tests.sh --e2e` passes with a strict subset of the full scenario
      count.
- [ ] `givn check review --change adr-qualification-gate` passes all required
      source, Arc42, and review gates.
