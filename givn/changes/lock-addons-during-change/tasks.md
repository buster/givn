# Tasks: lock-addons-during-change

Owning use case: `initialize-project`; supporting owner: `verify-change`.
Personas: `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

Release note: Add-on configuration is now recorded and locked per change, so an active change cannot silently disable a quality gate.

## Setup

- [x] Confirm the existing runner and strict mode; no runner change.
- [x] Confirm `script` (util-linux) is available for the PTY override scenario.
- [x] Confirm the lock unit tests run: `cargo test --locked --lib config::lock`.

## Batch A — add-on lock and enforcement

Design constraints:

- Add `src/config/lock.rs` with version 1, effective add-ons, proposed init
  values, differences, read/write, and override recording.
- `givn new` writes `config-lock.yaml` after creating the change directory.
- `givn addons enable|disable` refuse while a locked change is active; the
  operator path is `--operator --reason "<reason>"` on a TTY and records the
  override.
- `givn init --force` refuses only when the proposed add-on set differs from the
  recorded set; preserving forced init remains allowed.
- `givn check review` and `givn archive` compare current effective add-ons with
  the lock.
- A missing lock for an older change is skipped.
- The lock covers add-on values only.

### Scenario: A new change records the operator add-on configuration

- [x] Scenario: A new change records the operator add-on configuration
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: An active change refuses add-on mutation

- [x] Scenario: An active change refuses add-on mutation
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: A non-interactive operator override is refused

- [x] Scenario: A non-interactive operator override is refused
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: An operator override on a real terminal is recorded

- [x] Scenario: An operator override on a real terminal is recorded
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: A forced init cannot change add-ons during a locked change

- [x] Scenario: A forced init cannot change add-ons during a locked change
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: A forced init that preserves add-ons is allowed during a locked change

- [x] Scenario: A forced init that preserves add-ons is allowed during a locked change
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: Add-ons remain changeable without an active change

- [x] Scenario: Add-ons remain changeable without an active change
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

### Scenario: Review rejects add-on configuration drift

- [x] Scenario: Review rejects add-on configuration drift
  - [x] COMMIT: aae0ac56 -- Batch: addon-lock

## Batch B — documentation closeout

- [x] Update `docs/reference.md`, the affected arc42 chapters, and
      `docs/glossary.md`.
- [ ] Record the outcome in `docs/improvements/0.9.16/agent-addon-mutation.md`.
- [x] Confirm every delta step phrase has a permanent binding.
- [x] COMMIT: 492dff09 -- Batch: addon-lock-documentation-closeout
