# Prevent agents from changing add-on configuration during a change

## Status and purpose

Discussion handoff recorded on 2026-10-30. The intended improvement target is
Givn 0.9.15; the directory is not a release commitment.

This document describes a configuration-integrity problem observed in controlled
benchmark runs. It is not a formal proposal, executable specification,
technical design, or authorization to implement. No new Givn change was
started for this work. Preserve unrelated working-tree edits and do not attach
this improvement to an existing change merely because one is active.

Before continuing in another context:

1. Run `givn instructions` and interpret the current state against the request.
2. Load `givn instructions dialog` and the applicable companion examples before
   asking questions.
3. Recheck the evidence paths and source locations against the checkout used
   for the future change.

## Problem

An agent executing a change can run:

```sh
givn addons disable coverage
```

This mutates the project's tracked `givn/config.yaml`. The command succeeds
immediately. The agent can then continue through implementation, review, and
archive without the coverage gate, and the change looks complete.

The agent is not required to tell the user, justify the change, or record it.
The resulting project no longer has the add-on configuration the user selected.
A quality gate has been silently removed to work around a missing dependency or
an inconvenient check.

## Observed evidence

Controlled Catopia runs used an untouched benchmark and no add-on flags. All
add-ons were enabled when the agent started.

### 0.9.9 default, first attempt

- At `2026-10-30T06:18:37Z` the agent checked Python modules:
  `behave` missing, `coverage` missing, `pytest` present.
- At `06:18:43Z` it ran `pip install behave coverage`; the install failed with
  Debian's `externally-managed-environment` error.
- At `06:19:12Z` it ran:
  ```sh
  givn addons list
  givn addons disable coverage
  ```
- The workspace config changed from `coverage: true` to `coverage: false`.
- The run was stopped as non-compliant.

The same agent behavior was avoidable: prior 0.9.8 runs created a local
`.venv` and installed `behave` and `coverage` there while keeping the coverage
add-on enabled.

### 0.9.9 default, retry

The retry did not change add-ons. All add-ons stayed enabled and the run
completed with zero subagents.

### 0.9.11 tag

- At `2026-10-30T18:08:37Z` the agent again ran
  `givn addons disable coverage`.
- Coverage changed from enabled to disabled during the initial stage.
- The run was stopped as non-compliant.

### 0.9.10 runs

The 0.9.10 one-review and zero-subagent runs kept all add-ons enabled. The
defect is not deterministic; it is an available escape hatch the agent
sometimes chooses.

Evidence bundles:

- `/home/buster/projects/givn-harness/runs/givn-0.9.9-default-r2/`
- `/home/buster/projects/givn-harness/runs/givn-0.9.11-tag/`
- `/home/buster/projects/givn-harness/runs/givn-0.9.11-tag-r2/`

## Why this is a Givn problem, not only a benchmark problem

The benchmark exposed it, but the action is available in every Givn project:

- Add-on configuration is project policy, not change implementation.
- An active change is not authorized to weaken the project's verification,
  review, architecture, or documentation gates.
- The mutation is silent in the change record. The user may assume the selected
  gates ran.
- A missing dependency is an environment failure. Disabling the dependent
  add-on hides the failure instead of fixing or reporting it.
- The change can still archive, so an external observer cannot distinguish the
  selected configuration from the one the agent used.
- Benchmark comparisons become invalid because one arm has a different
  configuration than the one declared.

## Desired behavior

### Add-on configuration is operator-owned

An agent must not enable or disable add-ons while a change is active. Add-on
changes belong to the operator and happen outside a change, or in an explicit
configuration change.

Agents must not edit the `addons:` block in `givn/config.yaml` directly either.

### Missing dependencies are not a reason to disable a gate

If an add-on's dependency is missing, the agent must:

1. Install it in an isolated environment when that is in scope, or
2. Fail with a clear environmental diagnostic.

It must not disable the add-on, remove its configuration, or mark the gate
optional.

### Detect drift, even before prevention

Every change records the effective add-on configuration when it starts. The
static review and archive checks compare the current configuration with that
record.

A changed add-on setting fails with a message that names:

- the changed add-on;
- the before and after value;
- the artifact where the configuration was recorded;
- how an operator can intentionally change configuration outside the active
  change.

### Prevent the mutation

Recommended behavior:

- `givn addons enable` and `givn addons disable` refuse while a change is active,
  unless an explicit operator override is supplied.
- The refusal explains that add-on configuration is project policy and that the
  active change cannot alter it.
- Direct edits to `addons:` are detected by the change-scoped drift check even
  if the command path is bypassed.
- The refusal and any override are recorded in the change or project history.

A file permission or read-only mount can be added as defense in depth, but file
permissions alone are insufficient because the agent may be able to change them
or edit the file directly.

### Operator override

Intentional configuration changes remain possible:

- outside an active change;
- through an explicit operator action with a reason;
- in a dedicated configuration change when the effect must be reviewed.

The override must not be available to an autonomous agent as an ordinary shell
command.

## Alternatives considered

### Rely on instructions only

Rejected as the sole fix. The observed agent ignored the implicit expectation
and chose the convenient mutation.

### Detect after the fact only

Useful, but insufficient. The change still ran with the wrong configuration and
may have archived. Detection must be paired with refusal where possible.

### Make `coverage` fail if commands are unconfigured

This addresses one add-on but not the general mutation problem. It also treats
the dependency as a configuration error instead of an environment failure.

### Make the harness lock `givn/config.yaml`

Useful defense in depth for benchmark runs, but not sufficient for Givn. Givn
must protect its own project policy regardless of the execution harness.

## Acceptance criteria

1. During an active change, `givn addons disable coverage` exits nonzero and
   leaves `coverage: true` unchanged.
2. Directly editing the `addons:` block is detected by `givn check review` and
   `givn archive`, which fail with the add-on name and before/after values.
3. The error explains that add-on configuration is operator-owned and that a
   missing dependency must be installed in isolation or reported as an
   environment failure.
4. An operator can intentionally change add-ons outside an active change; the
   documented workflow succeeds.
5. An explicit operator override during a change is recorded with a reason and
   is not available as a silent agent command.
6. Existing projects and completed changes are unaffected.
7. A no-add-on-mutation run can still fail clearly when a dependency is missing;
   it must not silently archive with a weaker gate.

## Open questions

### D1 — Is the lock automatic for every active change?

Options:

1. **Automatic** — any active change locks add-on configuration. Gain: no agent
   can accidentally weaken the project. Cost: an operator must pause or finish
   the change before changing add-ons.
2. **Opt-in** — a project declares configuration locked. Gain: flexible. Cost:
   the default remains unsafe.

Recommended: automatic, with an explicit operator override.

### D2 — Where is the effective configuration recorded?

Options:

- in the change scaffold (for example `givn/changes/<id>/config-lock.yaml`);
- in the change record/history;
- in a generated artifact hash.

The design must choose one source of truth and ensure it survives stop/restart.

### D3 — What counts as an operator override?

Options:

- an interactive confirmation on a TTY;
- an explicit `--operator` flag plus a reason;
- a separate configuration change.

The design must prevent an autonomous agent from invoking the override as an
ordinary command.

## Non-goals

- No benchmark-specific rule.
- No prohibition on operators changing add-ons.
- No change to add-on defaults or the meaning of individual add-ons.
- No weakening of existing review, coverage, architecture, or outline gates.
- No requirement that every missing dependency be installed by Givn.
