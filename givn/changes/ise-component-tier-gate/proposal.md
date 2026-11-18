# Proposal: ise-component-tier-gate

## Release note

A whole-system Ise trial now carries a runnable verification contract and cannot be reported green without result evidence for every declared scenario and the rebuilt product it drove, while `givn ise compare` reports frozen-input equality only.

## Use-Case Context

- Use-case ID: prepare-an-ise-test
- Ideation topic: ise-rebuildability
- Seed path: givn/ideation/ise-rebuildability/changes/add-ise-clean-room-commands.md
- Confirmed Personas: developer--primary-user; agent--technical-role; givn-maintainer--owner

Spec gap outcome: `specification defect`. `prepare-an-ise-test` required a spec copy and frozen-input equality, but never required the kept component corpus to execute. The missing obligation is in that use case's specification and in the served `ise` instruction, not in the passuff builder.

## Review Context

- Review kind: independent
- Reason: the change alters the meaning of a green Ise trial result and adds an execution gate on an existing command surface; an independent review checks that input equality, trial success, coverage, and the not-run decision are not conflated.

## Problem / Opportunity

A whole-system Ise trial can pass while the component Gherkin corpus never runs. The passuff rebuild at `372cd83` kept 7 feature files and 244 scenarios (215 component, 29 `@e2e`). The rebuild removed `crates/passuff-cli/tests/**` with the source root and the builder authored no replacement runner. The frozen `@e2e` suite passed 29 of 29, the component tier executed 0 scenarios, measured component coverage was 0 of 4,965 lines, and the trial still looked successful.

Three gaps allow this:

- `givn ise rebuild-all` copies only the declared Specification roles. `givn/commands.yaml` and `givn/config.yaml` are not in the copy, so the builder sees no verification contract.
- `givn ise compare` reports `Specification files: match` and `End-to-end tests: match` and exits 0. Those lines prove frozen-input equality, not that the component corpus ran.
- No command reads a component runner result for a trial, and the coverage addon runs only during archive, not during a trial.

The result is a false success: "29 e2e scenarios pass" is reported where "the Specification is covered" was required. A missing component runner is invisible, and a trial can report success with 0% measured line coverage.

## Proposed Solution

A whole-system rebuild writes a verification contract into the spec copy. The contract names the component command and feature path, the frozen End-to-end command and feature path, the declared execution-unit counts for both tiers, and the coverage command and report paths when the coverage addon is effective. The manifest records the contract path and checksum.

`givn ise compare` remains read-only. It verifies the frozen Specification files, the End-to-end tests, and the verification contract, and prints `Trial result: unverified (frozen inputs only; run givn ise verify-trial ...)`. Its exit 0 means input equality only.

`givn ise verify-trial --manifest <file> --copy <dir> --result <file>` is the only command that can report a whole-system trial result. It runs the verification contract: the component command inside the spec copy, the frozen End-to-end command in the checkout, and the coverage measurement when the addon is effective. It binds the End-to-end run to the rebuilt product by hashing a declared product path, passing it through a declared environment variable, and requiring the End-to-end result to name the same product digest. It requires fresh runner results that name every declared scenario title as well as exact declared execution-unit counts, zero failed and zero skipped, non-zero component line coverage, and distinct instrumented tier reports. A count-only or title-less result refuses. It writes an `ise-trial-result/v1` record with both tiers' declared, executed, and scenario-title evidence, the product path and digest, and per-tier line and branch coverage. A whole-system rebuild change must commit a clean record before `givn check tasks` passes.

A trial whose component corpus is not executed refuses by name. It never prints a pass line and never writes `clean: true`. An explicit component not-run decision is accepted only when the declared component corpus is empty; otherwise the command refuses.

## Capability Routing

`givn spec route` recommended `EXTEND givn-workflow-enforcement` from the scaffold text. That recommendation reflects the template's `givn check` and `givn spec` mentions, not this change's actor goal. The decisions below follow the owning use cases.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `clean-room-preparation` | `EXTEND givn-workflow-enforcement` | `EXTEND clean-room-preparation` | The contract is written by the rebuild that owns the spec copy; the actor goal is preparing an Ise test, not enforcing the change workflow. |
| `trial-input-verification` | `EXTEND givn-workflow-enforcement` | `EXTEND trial-input-verification` | The verifier already owns comparison; the new execution gate completes that capability instead of adding a second comparison path. |
| `serve-extended-rules` | `EXTEND givn-workflow-enforcement` | `EXTEND serve-extended-rules` | The instruction is served through the existing guidance capability; no new capability owns instruction text. |

## Out of Scope

- Changing the archive coverage gate. It already rejects zero measured lines and identical tier reports; this change adds the trial-time gate.
- Promoting rebuilt code from a spec copy. A spec-copy trial remains discard-only.
- Changing the `@e2e` package boundary or the frozen-suite rules.
- A harness-specific grader. The gate is a Givn command that any harness or verifier can call.
- Reintroducing the retired `givn ise verify` name; the new command is `givn ise verify-trial`.

## Open Questions

### D1 — Exact or minimum declared count?

- Question: Must an executed tier equal its declared execution-unit count exactly, or may it exceed it?
- Options:
  1. **Exact** — executed equals declared; gain: extra, duplicated, or accidentally substituted scenarios are visible; cost: a runner that counts scenarios differently must be corrected; in practice: 10 declared and 11 executed refuses.
  2. **Minimum** — executed is at least declared; gain: outline expansion or retained extra tests never fail the gate; cost: duplicate or substituted scenarios can hide a missing declared scenario; in practice: 10 declared and 11 executed passes.
- Disposition: answered — operator direction 2026-11-18: exact count, with one execution unit per plain scenario and one per Scenario Outline example row.

### D2 — Does `compare` execute the verification contract?

- Question: Should `givn ise compare` run the component and End-to-end commands, or remain read-only?
- Options:
  1. **Separate explicit gate** — compare stays read-only and a new command executes; gain: existing read-only comparison stays safe against untrusted copies and its exit code keeps one meaning; cost: a verifier that claims a trial result must invoke one more command; in practice: `compare` prints matches and `Trial result: unverified`, then `verify-trial` executes.
  2. **Compare executes automatically** — gain: one command; cost: a comparison of an untrusted copy runs copied scripts, and exit 0 would conflate input equality with trial success; in practice: a malicious or broken `scripts/verify.sh` runs during a read-only check.
- Disposition: answered — operator direction 2026-11-18: keep compare read-only; make the trial gate explicit.

### D3 — What is the gate command called?

- Question: Reintroduce `givn ise verify` or add a new name?
- Options:
  1. **`givn ise verify-trial`** — gain: names the trial result and avoids the retired command; cost: a longer command; in practice: `givn ise verify-trial --manifest ... --copy ... --result ...`.
  2. **`givn ise verify`** — gain: shorter; cost: contradicts the permanent scenario that pins the retired name and its exit 2; in practice: the retired-name scenario must be rewritten.
- Disposition: answered — operator direction 2026-11-18: use `verify-trial`; the retired `verify` name keeps its exit-2 contract.

### D4 — How is a Scenario Outline counted?

- Question: Is an outline one declared execution unit or one per example row?
- Options:
  1. **One per example row** — gain: the declared count matches the runner's expanded scenario count; cost: the count differs from `givn spec tree`'s scenario-heading count; in practice: two examples declare two units.
  2. **One per outline** — gain: matches the heading count; cost: a runner that expands examples always executes more than declared and the exact gate refuses every honest outline; in practice: two examples declare one unit and execute two.
- Disposition: answered — operator direction 2026-11-18: one execution unit per example row, without changing `givn spec tree`.

### D5 — How is a 0-line End-to-end coverage report declared?

- Question: When an End-to-end suite runs the product as a subprocess, how may a 0-line report be accepted?
- Options:
  1. **Explicit `coverage.e2e_subprocess: true`** — gain: the limitation is declared once and recorded; cost: one new commands key; in practice: the contract carries the declaration and the gate records `subprocess-uninstrumented`.
  2. **Accept any 0-line report** — gain: no new key; cost: a measurement failure or copied empty report passes; in practice: a 0-line report is indistinguishable from an uninstrumented run.
- Disposition: answered — operator direction 2026-11-18: require the explicit declaration.

### D6 — How is the frozen End-to-end run bound to the rebuilt product?

- Question: Should the gate trust the project's End-to-end command to build the product, or bind and record the product it drove?
- Options:
  1. **Declared product path and digest** — `verify.product_path` names the product in the copy, `verify.product_env` names the variable the End-to-end command reads, and the result must carry the product path and SHA-256; gain: a script that rebuilds the checkout cannot pass; cost: the End-to-end command must honor one variable and record one digest; in practice: the gate refuses an E2E result whose `product_sha256` differs.
  2. **Trust the command** — gain: no new contract field; cost: the passuff false clean remains possible; in practice: an E2E script builds the checkout's binary while the copy holds a different one.
- Disposition: answered — operator direction 2026-11-19: bind the product path and digest.

### D7 — Is a count-only runner result sufficient?

- Question: May the gate accept a result with total/passed/failed/skipped only?
- Options:
  1. **Require scenario titles** — the result names every declared scenario; gain: a printf count cannot pass; cost: every runner must emit titles; in practice: a 10/10 count with no titles refuses.
  2. **Counts only** — gain: the smallest result contract; cost: the release note's execution claim is unsupported; in practice: a script that prints 10 passes.
- Disposition: answered — operator direction 2026-11-19: require titles.

### D8 — Is `verify-trial` optional for a whole-system change?

- Question: Can a whole-system rebuild change pass `givn check tasks` without a trial result?
- Options:
  1. **Mandatory workflow gate** — `check tasks` requires a clean `ise-trial-result/v1` under the change's evidence when the design records `Whole-system rebuild`; gain: the workflow cannot call the change green without the trial; cost: a whole-system change must commit the result before the tasks gate; in practice: a missing result refuses.
  2. **Optional tooling** — gain: no extra task; cost: the old unverified route remains; in practice: a change archives without a trial.
- Disposition: answered — operator direction 2026-11-19: make it mandatory for whole-system rebuild changes.
