# Design: cover-every-agent-target

## Overview

The change removes the static copies of agent-target knowledge from the
acceptance suite and makes the consumer-view reference check follow
production. It is test support plus scenario refinements: the reference
check recognizes targets through `AgentTarget::ALL` (with an executable
coupling guard), every "all targets" fixture and every "every target" loop
derives its selection from production, the check's installed positive
requires guidance from every configured target, one new scenario proves a
configured target without guidance fails closed, and the characterization
assertions derive command-owning targets from production.

The WHAT lives in `proposal.md`; the observable behaviour is locked by the
delta feature `specs/fragments/instruction-integrity.feature` (two modified
scenarios, one added).

## Decisions

- Decides (design): agent-target recognition in the check comes from the
  production enum, not a copied list. `CANDIDATE_ROOTS` keeps the Givn-owned
  directories (`givn`, `docs`, `assets`); `recognizes_target_dir(segment)`
  answers membership from `AgentTarget::ALL` (`display()` form, e.g.
  `.opencode`). `skill_reference` uses the same predicate. `AGENT_TARGETS` is
  deleted.
- Decides (design): the recognition property gets an executable coupling
  guard. The coverage step ("every agent target Givn supports is covered")
  asserts `recognizes_target_dir(target.display())` for every
  `AgentTarget::ALL` entry, so a stale literal predicate fails a scenario
  instead of silently narrowing extraction. The guard is green before and
  after the refactor; the refactor itself is not RED-able.
- Decides (design): every "all targets" fixture selects `AgentTarget::ALL`.
  There are four Given fixtures with that semantics:
  `prose_consolidation_steps.rs:91` (`--agents`) and `:313` (stdin prompt),
  `instruction_integrity_steps.rs:1042` (addon-free `--agents`), and
  `prose_consolidation_e2e_steps.rs:71` (E2E guidance fixture). The
  `selected_targets_and_addons_unchanged` assertion derives its expected
  `targets: [...]` line from `AgentTarget::ALL`. The fixture edits and the
  assertion derivation land in one atomic step; with the literal assertion
  left in place the permanent addon-free scenario fails immediately (observed
  in the design-review copy run: `targets: [opencode, agents, claude, omp]`
  versus the literal three).
- Decides (design): two "every target" loops that iterate a literal
  three-target list derive from the configured targets:
  `core_entries_require_dialog_policy` (`instruction_integrity_steps.rs:1069`)
  and `skill_consumers_receive_replacement`
  (`prose_consolidation_steps.rs:144`). Without this, the fixtures select
  four targets while these steps silently grade three.
- Decides (design): the check's installed positive additionally fails when a
  configured target contributed no guidance: `uncovered_targets(project_root,
  sources)` returns configured targets with no source label prefixed
  `.<target>/`; the installed positive step asserts it is empty. The printed
  positive is exempt: its corpus is printed text, which has no target
  dimension.
- Decides (design): the default-config modified scenario gains "every agent
  target Givn supports is covered by the synchronized guidance" — a step
  requiring each production target to contribute at least one installed
  source and to be recognized by the check. Stronger than the
  configured-target assertion and valid for this fixture, which selects the
  production list.
- Decides (design): the characterization modified scenario gains "every
  command-owning agent target receives the givn-characterize command" — a
  step iterating `AgentTarget::ALL` filtered by `owns_commands()`, requiring
  the command file exists and carries the CLI route. The existing content
  assertion switches from the literal `["opencode", "omp"]` to the
  production predicate on the configured targets and no longer skips a
  missing file silently.
- Decides (design): "the production target list" is two collaborating
  surfaces and the design says which is which. Collection follows the
  project's configured targets (`load_skills_targets`, validated against
  `KNOWN_AGENT_TARGETS`); recognition and the coverage guard follow
  `AgentTarget::ALL`. Divergence between the two is caught loudly: a target
  the fixture writes but collection drops fails the coverage step.
- Decides (design): the `enable-optional-addons` stub helper ("the skill
  `givn-domain-modeling` exists in all targets") and that feature's
  `opencode, agents, claude` scenario keep their explicit three-target
  assertions — deliberate scope, recorded below. They are not conflated with
  the addon-free all-targets fixture this change does update.
- Decides (design): no production code changes. The suite consumes the
  production list read-only; `src/` is untouched.

## Exact file changes

| Path | Change |
|---|---|
| `tests/common/reference_check.rs` | Production-derived recognition (`recognizes_target_dir` over `AgentTarget::ALL`); delete `AGENT_TARGETS`; add `uncovered_targets`. |
| `tests/steps/prose_consolidation_steps.rs` | Both "all targets" fixtures build their selection from `AgentTarget::ALL`; `skill_consumers_receive_replacement` iterates the configured targets. |
| `tests/steps/prose_consolidation_e2e_steps.rs` | The E2E guidance fixture selects `AgentTarget::ALL`; its consumer loop iterates the configured targets. |
| `tests/steps/instruction_integrity_steps.rs` | Addon-free all-targets fixture derives from production; `selected_targets_and_addons_unchanged` expectation derived; `core_entries_require_dialog_policy` iterates configured targets; installed positive Then asserts no uncovered configured target; characterize command step uses `owns_commands`; four new steps. |
| `givn/changes/cover-every-agent-target/specs/fragments/instruction-integrity.feature` | Delta: modified characterization scenario, modified default-configuration scenario, added fail-closed scenario. |
| `docs/arc42/10-quality-requirements.md` | Q-97 row gains the no-guidance clause; the 10.3 bullet states the check follows the production target list and requires guidance from every covered target. |
| `docs/arc42/11-risks-and-technical-debt.md` | Add D-21: per-target coverage is "at least one source", not completeness. |

### New wording, for the implementer

`reference_check.rs` recognition:

```rust
/// The Givn-owned directories the check grades besides agent targets.
const CANDIDATE_ROOTS: &[&str] = &["givn", "docs", "assets"];

/// Whether a first path segment names a production agent target directory.
pub fn recognizes_target_dir(segment: &str) -> bool {
    givn::skills::targets::AgentTarget::ALL
        .iter()
        .any(|target| target.display() == segment)
}
```

`is_candidate` accepts `CANDIDATE_ROOTS.contains(&first) || recognizes_target_dir(first)`;
`skill_reference` gates on `recognizes_target_dir(segments[0])`.

`uncovered_targets`:

```rust
/// Configured targets (directory form, e.g. `.claude`) that contributed no
/// installed guidance.
pub fn uncovered_targets(project_root: &Path, sources: &[GuidanceSource]) -> Vec<String> {
    let targets = givn::config::overlay::load_skills_targets(project_root)
        .expect("load configured skills targets");
    targets
        .into_iter()
        .filter(|target| {
            !sources
                .iter()
                .any(|source| source.label.starts_with(&format!(".{target}/")))
        })
        .map(|target| format!(".{target}"))
        .collect()
}
```

Fixture selection (all four fixtures):

```rust
let selection = AgentTarget::ALL
    .iter()
    .map(|target| target.name())
    .collect::<Vec<_>>()
    .join(", ");
```

The prompt fixture appends `\n`; the `--agents` fixtures pass the joined
string.

`selected_targets_and_addons_unchanged` expectation:

```rust
let expected = AgentTarget::ALL
    .iter()
    .map(|target| target.name())
    .collect::<Vec<_>>()
    .join(", ");
assert!(
    active.contains(&format!("targets: [{expected}]")),
    "selected targets changed after sync: {active}"
);
```

New steps in `instruction_integrity_steps.rs`:

| Step | Behaviour |
|---|---|
| `every agent target Givn supports is covered by the synchronized guidance` (Then) | Collects installed guidance; asserts every `AgentTarget::ALL` target (a) is recognized by `recognizes_target_dir` and (b) has at least one source whose label starts with `.<target>/`; the failure names the missing targets. |
| `every command-owning agent target receives the givn-characterize command` (Then) | Iterates `AgentTarget::ALL` filtered by `owns_commands()`; reads `.<target>/commands/givn-characterize.md` and asserts the CLI route is present; a missing file fails. |
| `the skills of the agent target "<target>" are removed` (When, regex) | Validates the name via `AgentTarget::from_name`, then removes `<project>/<target>/skills`. |
| `the configured-target check fails and names the agent target "<target>"` (Then, regex) | Runs `uncovered_targets` and asserts the list contains `<target>`. |

## Step definitions

One file per capability. `instruction-integrity` steps live in
`tests/steps/instruction_integrity_steps.rs` (existing, including the
strengthened positive, the derived loops, and the four new steps). The
shared fixtures live in `tests/steps/prose_consolidation_steps.rs` and
`tests/steps/prose_consolidation_e2e_steps.rs`. The classifier helper change
is in `tests/common/reference_check.rs`. No new step file; no production
code.

## Runner and strict mode

- Verify command (`givn/commands.yaml`): `./run-tests.sh`; E2E smoke:
  `./run-tests.sh --e2e`.
- Strict mode: `tests/features_runner.rs` calls `.fail_on_skipped()` and exits
  non-zero when `stats.failed > 0 || stats.skipped > 0`. `@wip` marks the
  delta's not-yet-implemented steps; `givn lint` reports them.
- Change-scope run:
  `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs ./run-tests.sh`
- Single scenario:
  `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs cargo test --locked --test features_runner -- --exact "fragments/instruction-integrity.feature::A configured target without synchronized guidance fails the check"`

## Use-case traceability

| Use-case element | Technical realization | E2E evidence |
|---|---|---|
| Capability `instruction-integrity` (fragment `support`): guidance remains complete and trustworthy for every supported harness | Production-derived recognition plus coupling guard; all-targets fixtures and loops; per-target coverage assertion; fail-closed scenario; production-derived command-owning assertion | Existing `@e2e` "An agent requests machine-readable artifact guidance" and "A consumer synchronizes skills and every agent target receives the ideation facilitation skill"; the delta scenarios assert the coverage property |
| Confirmed Personas: `givn-maintainer--owner` (owns the suite and the target list), `agent--technical-role` (its harnesses are graded) | The suite reads the production list; the failure message names the uncovered target | The capability has CLI text output; no new consumer-facing text |

Actors in the scenarios (`the user`, `the author`) stay distinct from
Personas; the Persona row records review context only.

## Visual Design Contract

- Interface: terminal.
- Viewports: not applicable — nothing renders HTML, CSS, or JavaScript; the
  change edits test support and Gherkin text consumed through the terminal.
- Surfaces covered: none changed; generated guidance text is unchanged.
- Evidence: no new evidence is required because the change adds no delta
  `@e2e` scenario; the existing `@e2e` transcripts remain unchanged.

## Interaction Coverage Matrix

The change adds no new consumer action, so no new inventory entry exists.
The existing coverage for the touched capability is unchanged:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| instruction-integrity: request effective instructions by id | "An agent requests machine-readable artifact guidance" | CLI | Real `givn instructions` subprocess |
| instruction-integrity: request normal session-start instructions | "Author starts a session with the universal dialog obligation" | CLI | Real `givn instructions` subprocess |

## Version freshness

No language, framework, library, or image version is introduced or changed.
The change edits test-support Rust and Gherkin; the lockfile is untouched.

## Local runnability & digital twins

No external or third-party dependency is touched. Local run and test command:
`./run-tests.sh`; E2E smoke via the configured `verify.e2e_command`. No digital
twin is required.

## Coverage process boundaries

N/A — the change adds no instrumented process and no measurement command; it
edits test support that drives the existing binary.

## Arc42 impact

Chapter 10 (quality requirements): the Q-97 row gains the no-guidance clause
("and a configured target with no synchronized guidance") and the 10.3
verification-signal bullet states that the check derives its covered targets
from the production target list and requires guidance from every covered
target. Chapter 11 (risks and technical debt): D-21 records that per-target
coverage is "at least one source", not completeness — removing one skill from
a target still passes. Chapter 8 is unaffected: 8.7's resolution rule is
unchanged; target derivation is a property of the check hosted by 10.3.
Chapter 12 is unaffected: `Agent target` already defines all four
destinations. The change's `arc42.md` carries the full 12-row assessment and
the ADR verdict (`NOT_QUALIFIED`, routing `CANONICAL_ARTIFACT` to
`docs/arc42/10-quality-requirements.md` section 10.3).

## Out of scope, recorded deliberately

- The `enable-optional-addons` stub helper ("the skill ... exists in all
  targets", `tests/steps/mod.rs:981`) and that feature's `opencode, agents,
  claude` scenario keep explicit three-target assertions; their steps name
  the targets they assert.
- Other hardcoded target knowledge judged deliberate and untriggered by this
  change: `template_conditional_rendering_support.rs` `targets()`, the
  `release_0_3_0` rule-path allowlist, the PTY option-order assumptions in
  `initialize_a_project_e2e_steps.rs`, the disabled-addon spot checks in
  `instruction_integrity_steps.rs`, and the attribution fixtures
  (`opencode, claude, omp` — `.agents` has no integration).

## Internal Primitives

None — no production module, struct, or function is added or changed. The
only new Rust is test support under `tests/common/` and `tests/steps/`.
