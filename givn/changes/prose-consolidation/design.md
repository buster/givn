# Design: prose-consolidation

## Canonical Source

`assets/instructions/specs.md` becomes the normative source for the E2E policy
and user-interaction inventory rules. It contains the complete, stable policy
wording, including:

- one E2E evidence scenario per distinct real user action;
- variants and error cases remain ordinary scenarios;
- the real-interface and primary-assertion requirements;
- separate E2E step definitions;
- strict separation of `verify.command` and `verify.e2e_command`.

The exact canonical sentence is:

```text
one E2E evidence scenario per distinct real user action
```

The other artifact instructions retain only their artifact-specific mechanics
and direct the author to read `givn instructions specs --change <id>` for this
policy. They must not reproduce alternate normative wording.

## Assets to Consolidate

Update these embedded assets:

- `assets/instructions/specs.md`: retain the canonical policy and remove
  contradictory wording.
- `assets/instructions/design.md`: retain design-specific infrastructure and
  coverage requirements, but reference the canonical policy instead of
  restating its interaction rules.
- `assets/instructions/review.md`: retain review/audit mechanics, but refer to
  the canonical policy for E2E scope and fidelity.
- `assets/instructions/tasks.md`: retain the TDD loop and runner mechanics, but
  refer to the canonical policy for E2E scope and interface fidelity.
- `assets/skills/givn-spec/SKILL.md.tmpl`: choreography only; direct the agent
  to `givn instructions specs` before writing scenarios.
- `assets/skills/givn-design/SKILL.md.tmpl`: choreography only; direct the
  agent to the canonical policy and require the design matrix.
- `assets/skills/givn-review/SKILL.md.tmpl`: choreography only; direct the
  reviewer to the canonical policy and record its verification.
- `assets/skills/givn-steps/SKILL.md.tmpl`: retain step mechanics, but refer to
  the canonical policy for E2E driver/assertion rules.
- `assets/skills/givn-tasks/SKILL.md.tmpl`: retain the TDD loop, but refer to
  the canonical policy for E2E scope and interface fidelity.
- `assets/skills/givn-implement/SKILL.md.tmpl` and
  `assets/skills/givn-design-review/SKILL.md.tmpl`: retain phase-specific
  mechanics, but remove duplicate policy wording.
- `assets/commands/givn-spec.md`: direct the command to the canonical policy
  and remove its duplicate E2E prose.
- `assets/commands/givn-design.md`, `givn-review.md`, `givn-tasks.md`,
  `givn-steps.md`, `givn-implement.md`, and `givn-design-review.md`: retain
  command choreography only and point to the canonical policy.
- `assets/templates/spec.feature` and `assets/templates/design.md`: generated
  scaffolds reference the canonical policy rather than defining a second
  inventory/E2E contract.

The allowlist above is exhaustive. Review scans every embedded asset in it for
the forbidden legacy inventory-entry wording (case-insensitive), and checks
every non-canonical asset for the canonical `givn instructions specs` reference.
Generated guidance is then checked for every configured skill target; only
`.opencode/commands/` is checked for generated slash commands because that is
the existing target contract.

The project override cascade remains the existing
`givn/artifacts/<artifact>/...` resolution path. A project override for the
specs instruction replaces the embedded canonical instruction for that
project; no new override mechanism is introduced.

## Architecture Impact

This is an embedded-asset and generated-guidance change. The Rust renderer in
`src/skills/render.rs` continues to perform the existing context substitution;
no renderer API or data model change is needed. `givn instructions` continues
to resolve one artifact instruction through the existing cascade.

No production runtime behavior, archive behavior, dependency, or external
service changes.

## Step Definitions

The executable spec uses one capability file:

```text
tests/steps/prose_consolidation_steps.rs
```

It drives the real `givn` subprocess in temporary initialized projects and
asserts on command output plus the generated guidance files. It is separate
from existing capability step files and uses no direct renderer calls in the
feature steps.

## Policy Boundaries

The inventory is normalized to distinct user-facing actions. A flag combination
that changes output is still a variant of the same CLI action unless it changes
the user's action or side effect. It therefore does not receive a second
inventory entry or a second `@e2e` scenario. The canonical specs instruction
states this explicitly; design and review guidance uses that policy for its
matrix and audit instead of defining a competing rule.

The project-override scenario is a regular regression scenario because the
override is configuration for the same `givn instructions specs` action, not a
distinct user-facing action.

## Runner and Strict Mode

The existing commands remain the runner contract:

```text
./run-tests.sh
./run-tests.sh --e2e
```

The runner is Rust `cucumber-rs`, and strict mode is
`.fail_on_skipped()` in `tests/features_runner.rs`. New unfinished steps use
`unimplemented!()`; the three scenarios reuse the real subprocess setup and
then assertions after implementation.

For one scenario, use the existing Cucumber name filter with the active change
path:

```text
GIVN_FEATURES=givn/changes/prose-consolidation/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

The two inventory scenarios are `@e2e` because each drives a real CLI
interaction. The project-override scenario is regular because it is a
configuration variant of the first action. The active change therefore has a
strictly smaller E2E subset than its full scenario set; after archive it remains
a strict subset of the full permanent suite.

## E2E Infrastructure and Local Runnability

The capability's real interface is the CLI. The E2E driver is the real
`givn` subprocess launched by the existing Cucumber world; file content is a
secondary observable consequence of `givn skills sync`, not a direct internal
call. No browser, HTTP server, database, queue, or third-party service exists.
The local command is `./run-tests.sh`; no isolated external network or digital
twin is required.

The anticipated obstacle is generated-target selection. The step definitions
read the initialized project's configured skill targets and inspect only files
that `givn skills sync` reports/generated. This follows the same target
selection as a real consumer and does not bypass the command.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `givn instructions specs --change prose-consolidation` to read the canonical authoring policy | Canonical E2E policy is emitted by specs instructions | CLI | Cucumber launches the real `givn instructions specs` subprocess and asserts stdout |
| Run `givn skills sync` to receive generated guidance that follows the canonical policy | Generated guidance points to the canonical E2E policy | CLI | Cucumber launches the real `givn skills sync` subprocess and reads the generated skill/command outputs it produced |

Each inventory entry has exactly one matching `@e2e` scenario. There is no
browser UI, so HTTP or browser automation would be the wrong driver. The
regular override scenario is intentionally outside the inventory because it
does not represent a distinct user action.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::skills::render::render_skill` and `givn::config::cascade` |

The existing scripts provide collision-safe profiles and merge both reports;
this change does not alter that process.
