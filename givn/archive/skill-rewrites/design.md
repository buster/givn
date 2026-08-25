# Design: skill-rewrites

## Technical Approach

This change edits embedded instruction, skill, and slash-command assets only.
The previous prose-consolidation change supplies the canonical policy source;
this change adds gate-specific choreography and the Black-Box-First decision.

No overlap engine, archive behavior, renderer API, dependency, or data model
changes are needed. The existing cascade and `givn skills sync` regeneration
path are reused.

## Asset Responsibilities

- `assets/skills/givn-review/SKILL.md.tmpl` and
  `assets/instructions/review.md`: require **Overlap dispositions**,
  **split-or-keep**, and removed+added supersession explanations before signoff.
- `assets/skills/givn-design/SKILL.md.tmpl`,
  `assets/instructions/design.md`, and their command choreography: require the
  **Black-Box-First** decision and the question **which case this test covers
  that the E2E does not** for every retained internal test.
- `assets/skills/givn-steps/SKILL.md.tmpl` and its command choreography: carry
  the same Black-Box-First question into step reuse and test choices.
- `assets/skills/givn-spec/SKILL.md.tmpl` and `assets/commands/givn-spec.md`:
  point to deterministic gates only and explicitly defer `givn spec index`
  until the retrieval change.
- `assets/skills/givn-tasks`, `givn-implement`, and design-review assets keep
  their phase mechanics and reference the canonical gate/Black-Box policy.

All generated targets use the existing `skills sync` path. No new template
interpolation or override mechanism is introduced.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| skill-rewrites | `tests/steps/skill_rewrites_steps.rs` | Real `givn` CLI subprocess |

The capability reuses generic generated-guidance assertions from
`tests/steps/prose_consolidation_steps.rs`; the promised capability-specific
file above contains the combined instruction step.

The new file only handles the combined design/tasks instruction interaction;
existing generic generated-guidance assertions are reused. No direct asset,
renderer, or cascade call is used by a feature step.

## Runner and Strict Mode

The existing runner contract remains:

```text
./run-tests.sh
./run-tests.sh --e2e
```

Strict mode is `.fail_on_skipped()` in `tests/features_runner.rs`; new Rust
stubs use `unimplemented!()`. The single-scenario command is:

```text
GIVN_FEATURES=givn/changes/skill-rewrites/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

The capability has one `@e2e` inventory scenario and three regular scenarios,
so the E2E count is strictly smaller than the full active-change count.

## Local Runnability and E2E

The real interface is the CLI. The runner launches the compiled `givn`
subprocess in a temporary initialized project; generated files are secondary
effects of the CLI command. `./run-tests.sh` is the local run command. No
server, browser, database, queue, third-party service, network, or digital twin
is required.

The only foreseeable obstacle is selecting generated agent targets. The E2E
fixture explicitly enables `opencode`, `.agents`, and `.claude`, and checks each
skill target plus the opencode commands directory. This uses the same config
surface a real consumer uses.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `givn skills sync` to receive workflow guidance for deterministic gates | Synchronized guidance explains how to resolve the deterministic gates | CLI | Real `givn skills sync` subprocess; assert stdout, generated skills, and opencode command |

The three instruction-output scenarios are regular variants of the same
guidance-reading workflow and do not add inventory actions.

## Coverage

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build | separate non-E2E/E2E Cobertura reports | `./merge-coverages.sh` | `givn::skills::render::render_skill` |

The existing project coverage scripts remain unchanged.
