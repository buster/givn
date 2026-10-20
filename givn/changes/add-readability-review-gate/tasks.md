# Tasks: add-readability-review-gate

Status: implementation task list. Work non-`@e2e` scenarios first, in feature
order, then the E2E setup and scenarios. One commit per scenario. Tick a box
only when its evidence and `COMMIT:` hash are filled.

## Use-case context

- Owning use cases: `verify-change` (new capability `readability-review-gate`)
  and `assess-code-structure` (modified capability `readability-report`).
- Actors: Developer; supporting Reviewer. Personas are review lenses:
  `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.
- The experiment stays opt-in/default-off. The gate runs only when the
  `readability` addon is enabled.
- Collective code ownership applies: technical debt seen in touched code is
  yours to improve; "not my code" and "out of scope" are not acceptable
  justifications.

## Shared design constraints (apply to every scenario)

- **Report:** findings only. Remove `Action`, `ClassifiedUnit.action`, and
  `ClassifiedUnit.recommendation`; `SCHEMA_VERSION` becomes 2. A unit is a
  finding when its classification is not `within-limits`. Human output has a
  `FINDING` line and evidence, no recommendation. JSON keeps classification,
  metrics, evidence, methods, outline, capabilities, and versions.
- **Gate:** unit-level accounting. Every touched finding at committed `HEAD`
  needs `READABILITY: FIXED <unit>` (unit under threshold) or
  `READABILITY: JUSTIFIED <unit>` plus a rationale. `READABILITY: PASS` is
  required when any finding exists; `FAIL <reason>` always blocks.
  No `WATCH`, `PLAN`, or `EXCEPTION` markers.
- **Rationale floor (B):** `givn/changes/<id>/readability/rationales/<slug>.md`
  with `Unit:`, `Classification:`, `Finding:`, `Evidence:` (a metric or path
  from the finding), `Why this shape is right:` (names the responsibility),
  `Alternatives considered:` (at least one named alternative and why rejected;
  not `none`/`n/a`), `Owner:` (`givn-maintainer--owner`), and
  `Follow-up:`/`Expiry:` when temporary. Banned: not my code, not this change,
  predates my work, no capacity, out of scope, legacy, too risky, no time,
  later.
- **Base:** parent of the first reachable commit touching
  `givn/changes/<change-id>/`. Git first parent; Fossil `ancestors current`;
  root commit = empty base; shallow clone and no change-directory commit fail
  closed. No fallback to merge-base or HEAD.
- **Head:** committed `HEAD`; dirty production paths fail closed with
  `production paths are dirty; commit or discard before review: <paths>`.
- **Scope:** default production `src/`; the gate reuses the public
  `readability::analyze`/`assess` pipeline and computes no metric itself;
  `syn` stays inside the adapter; deterministic and offline; no baseline,
  ledger, cache, or project write.
- **Signatures:** `givn check review --change <id>`; the gate runs after the
  verification-receipt check. The exact PASS line is `readability review:
  PASS`; the exact failure line is `readability review: FAIL (<reason>)`.
- **Report delta:** `@givn.modified`/`@givn.removed`/`@givn.added` titles must
  match the permanent `givn/specs/assess-code-structure/readability-report.feature`
  exactly. Removed scenarios carry one placeholder step. No action or
  recommendation steps remain.

## Setup task — runner, addon, and step skeleton

- [ ] Confirm the runner and strict mode.
  - `verify.command` is `./run-tests.sh --non-e2e`; `verify.e2e_command` is
    `./run-tests.sh --e2e`; `features_runner` has `fail_on_skipped`.
  - Create `tests/steps/readability_review_gate_steps.rs`,
    `tests/steps/readability_review_gate_e2e_steps.rs`, and
    `tests/steps/readability_review_gate_support.rs`; register them in
    `tests/steps/mod.rs`.
  - Prove strictness with an undefined-step probe; paste exit 100 and the
    restored baseline.
  - Enable the addon for this repository: `givn addons enable readability`;
    commit `givn/config.yaml` with the change artifacts.
  - Evidence:
    ```
    STRICT PROBE OUTPUT AND EXIT
    ```
    ```
    BASELINE OUTPUT
    ```

## Report delta (non-`@e2e`)


### Scenario: A data-only type is not treated as a god type

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A data-only type is not treated as a god type`
  - Body: `A data-only type is not treated as a god type`
  - `COMMIT:` <hash>

### Scenario: An interface is classified as an interface, not a god type

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): An interface is classified as an interface, not a god type`
  - Body: `An interface is classified as an interface, not a god type`
  - `COMMIT:` <hash>

### Scenario: A cohesive type is not treated as a god type

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A cohesive type is not treated as a god type`
  - Body: `A cohesive type is not treated as a god type`
  - `COMMIT:` <hash>

### Scenario: A type with many public methods is an interface

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A type with many public methods is an interface`
  - Body: `A type with many public methods is an interface`
  - `COMMIT:` <hash>

### Scenario: A large type in the inspect band is a cohesive large type

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A large type in the inspect band is a cohesive large type`
  - Body: `A large type in the inspect band is a cohesive large type`
  - `COMMIT:` <hash>

### Scenario: A module over the plan limit is a module finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: module finding is a separate unit; a type rationale does not cover it; every finding needs FIXED or JUSTIFIED.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A module over the plan limit is a module finding`
  - Body: `A module over the plan limit is a module finding`
  - `COMMIT:` <hash>

### Scenario: A long method is a method-length finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A long method is a method-length finding`
  - Body: `A long method is a method-length finding`
  - `COMMIT:` <hash>

### Scenario: A type at the readable size limit is within limits

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A type at the readable size limit is within limits`
  - Body: `A type at the readable size limit is within limits`
  - `COMMIT:` <hash>

### Scenario: A type one line over the readable size limit is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A type one line over the readable size limit is a finding`
  - Body: `A type one line over the readable size limit is a finding`
  - `COMMIT:` <hash>

### Scenario: A report is available in machine-readable form

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Findings-only report schema 2: no action/recommendation; classification != within-limits is a finding; JSON keeps metrics/evidence/outline; delta modifies the permanent report scenario by title.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A report is available in machine-readable form`
  - Body: `A report is available in machine-readable form`
  - `COMMIT:` <hash>

### Scenario: The JSON findings count counts every non-within-limits unit

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): The JSON findings count counts every non-within-limits unit`
  - Body: `The JSON findings count counts every non-within-limits unit`
  - `COMMIT:` <hash>

### Scenario: The JSON report carries the schema and rules versions

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Findings-only report schema 2: no action/recommendation; classification != within-limits is a finding; JSON keeps metrics/evidence/outline; delta modifies the permanent report scenario by title.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): The JSON report carries the schema and rules versions`
  - Body: `The JSON report carries the schema and rules versions`
  - `COMMIT:` <hash>

### Scenario: Every analyzed unit has exactly one classification

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): Every analyzed unit has exactly one classification`
  - Body: `Every analyzed unit has exactly one classification`
  - `COMMIT:` <hash>

### Scenario: Units are sorted by classification and path

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): Units are sorted by classification and path`
  - Body: `Units are sorted by classification and path`
  - `COMMIT:` <hash>

### Scenario: The readability review does not run when the experiment is disabled

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): The readability review does not run when the experiment is disabled`
  - Body: `The readability review does not run when the experiment is disabled`
  - `COMMIT:` <hash>

### Scenario: A new type over the readable size with no decision fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A new type over the readable size with no decision fails`
  - Body: `A new type over the readable size with no decision fails`
  - `COMMIT:` <hash>

### Scenario: A new module over the readable size with no decision fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: module finding is a separate unit; a type rationale does not cover it; every finding needs FIXED or JUSTIFIED.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A new module over the readable size with no decision fails`
  - Body: `A new module over the readable size with no decision fails`
  - `COMMIT:` <hash>

### Scenario: A fixed marker for a unit still over its threshold fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: FIXED is accepted only when the unit is under its threshold at head and the marker names the unit id; a stale FIXED fails.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A fixed marker for a unit still over its threshold fails`
  - Body: `A fixed marker for a unit still over its threshold fails`
  - `COMMIT:` <hash>

### Scenario: A fixed marker for a unit under its threshold passes

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: FIXED is accepted only when the unit is under its threshold at head and the marker names the unit id; a stale FIXED fails.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A fixed marker for a unit under its threshold passes`
  - Body: `A fixed marker for a unit under its threshold passes`
  - `COMMIT:` <hash>

### Scenario: A justified marker with a complete rationale passes

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A justified marker with a complete rationale passes`
  - Body: `A justified marker with a complete rationale passes`
  - `COMMIT:` <hash>

### Scenario: A concrete rationale with a named alternative passes

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A concrete rationale with a named alternative passes`
  - Body: `A concrete rationale with a named alternative passes`
  - `COMMIT:` <hash>

### Scenario: A justified marker without a rationale fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A justified marker without a rationale fails`
  - Body: `A justified marker without a rationale fails`
  - `COMMIT:` <hash>

### Scenario: A justified marker for a unit under its threshold fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A justified marker for a unit under its threshold fails`
  - Body: `A justified marker for a unit under its threshold fails`
  - `COMMIT:` <hash>

### Scenario: A rationale whose unit does not match the finding fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A rationale whose unit does not match the finding fails`
  - Body: `A rationale whose unit does not match the finding fails`
  - `COMMIT:` <hash>

### Scenario: Two units with the same rationale slug fail

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Two units with the same rationale slug fail`
  - Body: `Two units with the same rationale slug fail`
  - `COMMIT:` <hash>

### Scenario: A rationale whose classification does not match the finding fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A rationale whose classification does not match the finding fails`
  - Body: `A rationale whose classification does not match the finding fails`
  - `COMMIT:` <hash>

### Scenario: A rationale without an owner fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A rationale without an owner fails`
  - Body: `A rationale without an owner fails`
  - `COMMIT:` <hash>

### Scenario: A rationale from an unconfirmed owner fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A rationale from an unconfirmed owner fails`
  - Body: `A rationale from an unconfirmed owner fails`
  - `COMMIT:` <hash>

### Scenario: A justification for "not my code" is rejected

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Rationale floor (B): concrete evidence, a named rejected alternative, a responsibility statement, a confirmed owner; banned ownership/scope/legacy phrases.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A justification for "not my code" is rejected`
  - Body: `A justification for "not my code" is rejected`
  - `COMMIT:` <hash>

### Scenario: A justification for "out of scope" is rejected

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Rationale floor (B): concrete evidence, a named rejected alternative, a responsibility statement, a confirmed owner; banned ownership/scope/legacy phrases.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A justification for "out of scope" is rejected`
  - Body: `A justification for "out of scope" is rejected`
  - `COMMIT:` <hash>

### Scenario: A boilerplate rationale is rejected

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Rationale floor (B): concrete evidence, a named rejected alternative, a responsibility statement, a confirmed owner; banned ownership/scope/legacy phrases.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A boilerplate rationale is rejected`
  - Body: `A boilerplate rationale is rejected`
  - `COMMIT:` <hash>

### Scenario: A type rationale does not account for the module finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A type rationale does not account for the module finding`
  - Body: `A type rationale does not account for the module finding`
  - `COMMIT:` <hash>

### Scenario: Two types in one file need two rationales

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Two types in one file need two rationales`
  - Body: `Two types in one file need two rationales`
  - `COMMIT:` <hash>

### Scenario: A fixed finding without a fixed marker fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: FIXED is accepted only when the unit is under its threshold at head and the marker names the unit id; a stale FIXED fails.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A fixed finding without a fixed marker fails`
  - Body: `A fixed finding without a fixed marker fails`
  - `COMMIT:` <hash>

### Scenario: An existing non-growing finding without a decision fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): An existing non-growing finding without a decision fails`
  - Body: `An existing non-growing finding without a decision fails`
  - `COMMIT:` <hash>

### Scenario: An existing growing finding passes with a justified marker

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): An existing growing finding passes with a justified marker`
  - Body: `An existing growing finding passes with a justified marker`
  - `COMMIT:` <hash>

### Scenario: A new cohesive large type passes with a justified marker

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A new cohesive large type passes with a justified marker`
  - Body: `A new cohesive large type passes with a justified marker`
  - `COMMIT:` <hash>

### Scenario: An existing data-only type is exempt from the size gate

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): An existing data-only type is exempt from the size gate`
  - Body: `An existing data-only type is exempt from the size gate`
  - `COMMIT:` <hash>

### Scenario: A data-only type with a long method is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A data-only type with a long method is a finding`
  - Body: `A data-only type with a long method is a finding`
  - `COMMIT:` <hash>

### Scenario: A new interface under the readable size with more than 50 public methods is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A new interface under the readable size with more than 50 public methods is a finding`
  - Body: `A new interface under the readable size with more than 50 public methods is a finding`
  - `COMMIT:` <hash>

### Scenario: A module of exactly 1000 code lines is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: module finding is a separate unit; a type rationale does not cover it; every finding needs FIXED or JUSTIFIED.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A module of exactly 1000 code lines is a finding`
  - Body: `A module of exactly 1000 code lines is a finding`
  - `COMMIT:` <hash>

### Scenario: A type at exactly 500 code lines is not a gate finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A type at exactly 500 code lines is not a gate finding`
  - Body: `A type at exactly 500 code lines is not a gate finding`
  - `COMMIT:` <hash>

### Scenario: A long method on a type within the size limit is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level accounting: type finding is evaluated across files; every type finding needs FIXED or JUSTIFIED; a file move that keeps the type does not reset it.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A long method on a type within the size limit is a finding`
  - Body: `A long method on a type within the size limit is a finding`
  - `COMMIT:` <hash>

### Scenario: The review requires a readability verdict when a finding exists

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): The review requires a readability verdict when a finding exists`
  - Body: `The review requires a readability verdict when a finding exists`
  - `COMMIT:` <hash>

### Scenario: A fail marker fails the review even when every finding is justified

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A fail marker fails the review even when every finding is justified`
  - Body: `A fail marker fails the review even when every finding is justified`
  - `COMMIT:` <hash>

### Scenario: A missing review record with a finding fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A missing review record with a finding fails`
  - Body: `A missing review record with a finding fails`
  - `COMMIT:` <hash>

### Scenario: A touched production file that cannot be parsed fails closed

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A touched production file that cannot be parsed fails closed`
  - Body: `A touched production file that cannot be parsed fails closed`
  - `COMMIT:` <hash>

### Scenario: A changed Rust file the analyzer skips fails closed

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A changed Rust file the analyzer skips fails closed`
  - Body: `A changed Rust file the analyzer skips fails closed`
  - `COMMIT:` <hash>

### Scenario: A temporary rationale without a follow-up fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A temporary rationale without a follow-up fails`
  - Body: `A temporary rationale without a follow-up fails`
  - `COMMIT:` <hash>

### Scenario: A temporary rationale with a follow-up and expiry passes

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A temporary rationale with a follow-up and expiry passes`
  - Body: `A temporary rationale with a follow-up and expiry passes`
  - `COMMIT:` <hash>

### Scenario: Conflicting decisions for one unit fail

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Conflicting decisions for one unit fail`
  - Body: `Conflicting decisions for one unit fail`
  - `COMMIT:` <hash>

### Scenario: An unknown readability marker fails

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): An unknown readability marker fails`
  - Body: `An unknown readability marker fails`
  - `COMMIT:` <hash>

### Scenario: Several failures are reported together

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Findings-only report schema 2: no action/recommendation; classification != within-limits is a finding; JSON keeps metrics/evidence/outline; delta modifies the permanent report scenario by title.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Several failures are reported together`
  - Body: `Several failures are reported together`
  - `COMMIT:` <hash>

### Scenario: A symlink in place of a touched production file fails closed

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A symlink in place of a touched production file fails closed`
  - Body: `A symlink in place of a touched production file fails closed`
  - `COMMIT:` <hash>

### Scenario: A base with no production sources makes every head unit new

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A base with no production sources makes every head unit new`
  - Body: `A base with no production sources makes every head unit new`
  - `COMMIT:` <hash>

### Scenario: A head with no production sources passes with no findings

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A head with no production sources passes with no findings`
  - Body: `A head with no production sources passes with no findings`
  - `COMMIT:` <hash>

### Scenario: A production commit before the change directory is part of the base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A production commit before the change directory is part of the base`
  - Body: `A production commit before the change directory is part of the base`
  - `COMMIT:` <hash>

### Scenario: A root commit whose change directory is the first commit has an empty base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A root commit whose change directory is the first commit has an empty base`
  - Body: `A root commit whose change directory is the first commit has an empty base`
  - `COMMIT:` <hash>

### Scenario: A merge commit uses its first parent as the change base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A merge commit uses its first parent as the change base`
  - Body: `A merge commit uses its first parent as the change base`
  - `COMMIT:` <hash>

### Scenario: A rebase recomputes the change base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A rebase recomputes the change base`
  - Body: `A rebase recomputes the change base`
  - `COMMIT:` <hash>

### Scenario: A change directory without a reachable commit fails closed

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A change directory without a reachable commit fails closed`
  - Body: `A change directory without a reachable commit fails closed`
  - `COMMIT:` <hash>

### Scenario: A shallow clone fails closed

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A shallow clone fails closed`
  - Body: `A shallow clone fails closed`
  - `COMMIT:` <hash>

### Scenario: A sibling Fossil timeline is not the change base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A sibling Fossil timeline is not the change base`
  - Body: `A sibling Fossil timeline is not the change base`
  - `COMMIT:` <hash>

### Scenario: A Fossil merge check-in uses its parent field as the change base

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Base rule: parent of the first reachable commit touching givn/changes/<id>/; Git first parent, Fossil ancestors current; root = empty base; shallow/no-commit fail closed; no fallback.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A Fossil merge check-in uses its parent field as the change base`
  - Body: `A Fossil merge check-in uses its parent field as the change base`
  - `COMMIT:` <hash>

### Scenario: Dirty production paths fail closed before analysis

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Committed HEAD only; dirty production paths fail with `production paths are dirty; commit or discard before review: <paths>`.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Dirty production paths fail closed before analysis`
  - Body: `Dirty production paths fail closed before analysis`
  - `COMMIT:` <hash>

### Scenario: A file move closes the old unit and needs a decision for the new unit

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A file move closes the old unit and needs a decision for the new unit`
  - Body: `A file move closes the old unit and needs a decision for the new unit`
  - `COMMIT:` <hash>

### Scenario: Deleting a finding requires a fixed marker

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: FIXED is accepted only when the unit is under its threshold at head and the marker names the unit id; a stale FIXED fails.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Deleting a finding requires a fixed marker`
  - Body: `Deleting a finding requires a fixed marker`
  - `COMMIT:` <hash>

### Scenario: Deleting a production file needs a decision for its removed unit

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): Deleting a production file needs a decision for its removed unit`
  - Body: `Deleting a production file needs a decision for its removed unit`
  - `COMMIT:` <hash>

### Scenario: The readability review is deterministic

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): The readability review is deterministic`
  - Body: `The readability review is deterministic`
  - `COMMIT:` <hash>

### Scenario: The readability review stores no classification baseline

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Unit-level rationale schema and mechanical floor: Unit/Classification/Finding/Evidence/Why this shape is right/Alternatives considered (named + rejected)/Owner/Follow-up+Expiry; owner judgment for truth.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): The readability review stores no classification baseline`
  - Body: `The readability review stores no classification baseline`
  - `COMMIT:` <hash>

### Scenario: The review guidance carries the fixed readability vocabulary

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: FIXED is accepted only when the unit is under its threshold at head and the marker names the unit id; a stale FIXED fails.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): The review guidance carries the fixed readability vocabulary`
  - Body: `The review guidance carries the fixed readability vocabulary`
  - `COMMIT:` <hash>

## E2E setup task

- [ ] Build the review-ready fixture and prove the E2E scope.
  - Exact sequence: `givn init`; enable `readability` and commit
    `givn/config.yaml`; commit the base `src/` type; commit the change
    directory; create the design step files, tasks with a real `COMMIT:` hash,
    a current receipt, `README-IMPACT: none`, and the run/coverage commands;
    run `givn check tasks`; write `review.md` markers and the rationale; run
    `givn check review`.
  - `verify.e2e_command` is `./run-tests.sh --e2e`, strictly scoped to `@e2e`.
  - Record non-E2E and E2E scenario counts; E2E must be strictly smaller.
  - Evidence:
    ```
    NON-E2E COUNT
    ```
    ```
    E2E COUNT
    ```

## E2E scenarios


### Scenario: A developer runs a readability report and sees the classified units

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Findings-only report schema 2: no action/recommendation; classification != within-limits is a finding; JSON keeps metrics/evidence/outline; delta modifies the permanent report scenario by title.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): A developer runs a readability report and sees the classified units`
  - Body: `A developer runs a readability report and sees the classified units`
  - `COMMIT:` <hash>

### Scenario: A developer runs the review with accounted readability findings and it passes

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the single-scenario command; paste non-zero output.
  - Design constraints: Fix-or-justify gate: every touched finding at committed HEAD must be FIXED or JUSTIFIED with a valid rationale; READABILITY: PASS when any finding exists; FAIL always blocks.
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the rule; list every production file created/modified; compile; run the same scenario; paste exit 0.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: clean up without behavior change; re-run; paste exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-review-gate): A developer runs the review with accounted readability findings and it passes`
  - Body: `A developer runs the review with accounted readability findings and it passes`
  - `COMMIT:` <hash>

## Completion checklist

- [ ] Every report-delta and gate scenario has RED/GREEN/REFACTOR evidence and
  one `COMMIT:` hash naming the scenario in the message body.
- [ ] Both E2E scenarios have real-subprocess evidence and one commit each.
- [ ] `givn lint --change add-readability-review-gate` exits 0 or 2 with only
  @wip/shape advisories and no `UCDOC blocking`.
- [ ] `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the delta
  and on `givn/specs`.
- [ ] The report schema is 2; no `action`/`recommendation` remains; no
  `WATCH`/`PLAN`/`EXCEPTION` marker remains.
- [ ] The change's own review records `READABILITY: JUSTIFIED module:src/cli/check.rs`
  only if that module is a finding at head; otherwise move the first rationale
  to the first touched gate finding.
- [ ] `README-IMPACT: updated - What it is, Blocking gates` and the README
  bullets are updated.
- [ ] ADR-028 and ADR-071 amendments are present and dated; register counts are
  consistent.
