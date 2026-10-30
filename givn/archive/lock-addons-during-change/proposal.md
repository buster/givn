# Proposal: lock-addons-during-change

## Release note

Add-on configuration is now recorded and locked per change, so an active change cannot silently disable a quality gate.

## Use-Case Context

- Use-case ID: `initialize-project` — supporting owner: `verify-change` for the review and archive drift gates.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.
- Source: `docs/improvements/0.9.16/agent-addon-mutation.md`; the operator selected the automatic lock, the change-local lock file, and the TTY/reason override.

## Review Context

- Review kind: self-review
- Reason: configuration-integrity behavior with explicit refusal and tested drift gates; no new lifecycle artifact, persistent product data, or security boundary.

## Problem / Opportunity

An agent executing a change can run `givn addons disable coverage`. The command succeeds immediately, edits tracked `givn/config.yaml`, and the change can continue without the coverage gate. The agent does not need to justify the change or record it. The user may assume the selected gates ran.

The observed benchmark runs show the behavior at 0.9.9 and 0.9.11. The defect is an available escape hatch, not a deterministic agent behavior. The same path exists for every add-on: architecture docs, README enforcement, outline, domain modeling, and challenge.

A missing dependency is an environment failure. Disabling the dependent gate hides the failure instead of fixing or reporting it.

## Proposed Solution

Givn now treats add-on configuration as operator-owned project policy.

- `givn new` records every effective add-on value in `givn/changes/<id>/config-lock.yaml` with a format version and an empty override list.
- While a locked change is active, `givn addons enable|disable` refuses. The refusal names the add-on, explains the operator-owned policy, and says a missing dependency must be installed in isolation or reported as an environment failure.
- `givn check review` and `givn archive` compare the current effective configuration with the lock. A drift fails with the add-on, before/after values, the lock path, and the operator instructions.
- An explicit operator override uses `--operator --reason "<reason>"` and requires an interactive terminal. It updates the recorded value and appends the add-on, before/after, reason, and timestamp to the lock.
- A forced init that would change add-ons during a locked change refuses before writing. A forced init that preserves the recorded add-ons is allowed.
- Changes created before this version have no lock and remain unaffected. Deleting a lock is deliberate tampering outside the observed accidental path; it is documented as a residual limit.
- The lock covers add-on policy only, not rules policy, targets, or commands.

## Capability Routing

`givn spec route --change lock-addons-during-change` reports `EXTEND givn-workflow-enforcement` (signal), which is the review/archive verification owner.

| Proposed capability | Route's recommendation | Decision | Rationale |
|---|---|---|---|
| `enable-optional-addons` | `EXTEND givn-workflow-enforcement` | `EXTEND enable-optional-addons` | The lock is add-on configuration policy; the CLI refusal and scaffold belong to initialization. |
| `givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` | The drift check runs in review and archive, which the workflow-enforcement capability owns. |

## Out of Scope

- Cryptographic protection or prevention of deliberate lock-file deletion.
- Guarding `givn upgrade`, which does not change add-on values.
- A standalone configuration-change lifecycle.
- Benchmark-specific rules.
- Changing add-on defaults or the meaning of individual add-ons.
- Installing missing dependencies on the operator's behalf.

## Open Questions

### D1 — Is the lock automatic?

- Status: answered 2026-10-30 (user)
- Disposition: automatic for every new change.

### D2 — Where is the baseline recorded?

- Status: answered 2026-10-30 (user)
- Disposition: `givn/changes/<id>/config-lock.yaml`.

### D3 — What counts as an operator override?

- Status: answered 2026-10-30 (user)
- Disposition: `--operator --reason "<reason>"` on an interactive terminal, recorded in the lock.

### D4 — How is a missing lock treated?

- Status: answered 2026-10-30 (user)
- Disposition: a missing lock means an older change and is skipped. No extra required marker in 0.9.16; deliberate deletion remains a documented residual limit.

### D5 — Is `givn init --force` guarded?

- Status: answered 2026-10-30 (user)
- Disposition: guard only when the forced init would change the effective add-on set. A preserving forced init remains allowed.
