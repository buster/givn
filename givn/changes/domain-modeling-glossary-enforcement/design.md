# Design: domain-modeling-glossary-enforcement

## Overview

The domain-modeling skill stops producing a throwaway per-change `domain.md`
and instead maintains the project's durable arc42 glossary
(`docs/arc42/12-glossary.md`). The skill becomes active in virtually every
workflow step: its trigger covers the whole change lifecycle, and `givn
instructions` names it at every stage when the addon is on. Design review and
review gain a mandatory human check that the ubiquitous language is followed,
used consistently everywhere, and recorded in the glossary. Domain modeling
requires the arc42 glossary: the router only names the skill when both the
`domain_modeling` and `arc42` addons are enabled.

No term-matching scanner is introduced; vocabulary is validated by people.
The only automated surface is the deterministic config fact that the glossary
must exist (arc42 on) for the skill pointer to be emitted.

## Technology Decisions

- **Language/framework**: Rust (repo standard). cucumber-rs for the feature
  runner — no new dependency, versions pinned by the repo's `Cargo.lock`
  (latest at lock time; nothing new to resolve).
- **Routing predicate**: `domain_modeling && arc42` decides whether the
  domain skill is named, replacing the current artifact-scoped (specs/design
  only) rule. Chosen because "loaded in practically every step" is a
  lifecycle-wide property, and the arc42 requirement is a deterministic
  config fact (per the coupling decision).
- **No scanner**: explicit rejection of term-membership checking. Specs and
  design prose are human-validated; a word-match gate would false-positive on
  inflection and incidental prose and is not reliable enough to gate with
  (per the settled scope decision).
- **Artifact lifecycle**: the glossary remains an arc42 chapter with the
  existing conformance gate (mark `Affected: Yes` → the gate forces a real
  edit). No new artifact type, no new merge path.

## Step Definitions

One file per capability — reuse the existing capability step files (this
change extends the same `domain-skill-trigger` capability):

| Capability | Step definition file |
|---|---|
| Domain-modeling skill is triggered and routed (regular) | tests/steps/fix_domain_modeling_skill_trigger_steps.rs |
| Domain-modeling skill is triggered and routed (e2e) | tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs |

New regular steps (in the regular file): template-inspection Given/When/Then
for the design-review and review skill templates; a CLI guard Given for
"domain_modeling on, arc42 off"; a CLI Given for "change awaiting review"
(next required artifact is `review`). New e2e assertions (in the e2e file)
for the modified sync scenario: generated skill loads during every workflow
step and instructs maintaining the project's arc42 glossary.

Test runner command (from `verify.command`):
```
./run-tests.sh
```

Single-scenario run command (exact invocation for ONE named scenario):
```
GIVN_FEATURES=givn/changes/domain-modeling-glossary-enforcement/specs GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=disabled cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "<scenario title>"
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `domain_skill_to_load` | `fn` | Decides the skill to route to. Signature changes to `(domain_modeling: bool, arc42: bool) -> Option<&'static str>`; returns `Some("givn-domain-modeling")` when both are true, for any workflow step. |
| `report_change` | `fn` | Generic next-step report. Signature gains an `arc42: bool` parameter so the generic path uses the same lifecycle-wide predicate. |

No new structs or enums.

## Strict Mode (mandatory)

The runner (`tests/features_runner.rs`) uses cucumber-rs `.fail_on_skipped()`,
so an undefined or pending step fails the run. RED step bodies use
`unimplemented!()` — never an empty body.

```
cucumber-rs: .fail_on_skipped() configured in tests/features_runner.rs;
RED step bodies use unimplemented!("...")
```

## Local Runnability & Digital Twins (mandatory)

**Local run command**: none — CLI-only tool, no server, no stack to start.

**Isolated network**: N/A — no external services.

**Digital twins**: none — the system talks to no third-party service; the
tests drive the compiled `givn` binary as a subprocess.

**Anticipated interface obstacles and their fix**: None. The e2e surface is a
real CLI subprocess (`givn skills sync`, `givn instructions`,
`givn instructions specs|design --change <id>`); the fixture project is
created in a temp dir via `crate::common::make_init_root()`.

Stage detection is file- and marker-based, so the "change awaiting review"
fixture must satisfy each status rule: after `givn new <id>` (proposal.md
present) write a spec `.feature` (specs glob), `design.md`, a
`design-review.md` containing the `DESIGN-REVIEW: PASS` marker (design-review
is Done only with the marker), and a `tasks.md` with a fully checked task list
(tasks is Done only when all boxes are checked:
`parse_tasks_progress().is_complete()`). Leaving `review.md` absent makes
`ChangeState::next_required()` yield `review`. The coupling-guard fixture runs
`givn addons disable arc42` after init so `arc42` is off while
`domain_modeling` stays on.

## E2E Smoke Test Infrastructure

**Interface type**:
```
CLI — real compiled Givn binary invoked as a subprocess.
```

**E2E runner command** (→ `verify.e2e_command`):
```
./run-tests.sh --e2e
```

**E2E step location**: `tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs`.

**Local test infrastructure**: fixture projects via `crate::common::run_givn`
against the built binary in a temp root; `make_init_root()` enables all
addons (then a given step toggles: `givn addons disable arc42` for the
coupling guard, or explicit positions out the stage files).

**E2E framework choice and justification**: cucumber-rs, identical to the
existing capability e2e — the inventory actions (sync, next-step, specs
instructions, design instructions) already have one `@e2e` scenario each; this
change modifies the sync scenario's assertions and adds no new `@e2e`
scenarios (the lifecycle-breadth routing is a variant of the existing
next-step action, so it is a regular scenario).

**Interaction Coverage Matrix** — one row per User Interaction Inventory
entry:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run `givn skills sync` to regenerate skills | An agent synchronizes skills and the domain skill is routed (modified) | CLI | Real compiled Givn subprocess `givn skills sync`; read `.opencode/skills/givn-domain-modeling/SKILL.md` |
| request next-step guidance via `givn instructions` | An agent requests next-step guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess; inspect stdout |
| request the specs instructions via `givn instructions specs --change <id>` | An agent requests the specs guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess; inspect stdout |
| request the design instructions via `givn instructions design --change <id>` | An agent requests the design guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess; inspect stdout |

**Real-interface assertion rule**: primary assertions on CLI stdout and the
generated skill file content.

**Real-interface driving rule**: the step definitions invoke the compiled
binary as a subprocess and assert on its output — the CLI is the real
interface.

**E2E scope**: one `@e2e` per distinct real action; no additional `@e2e`
scenarios beyond the existing four inventory entries.

## Architecture Impact

- `src/cli/instructions.rs` — `domain_skill_to_load` predicate and
  `report_change` signature; `run_generic` call sites pass
  `manifest.addons.arc42`.
- `assets/skills/givn-domain-modeling/SKILL.md.tmpl` — retarget to the arc42
  glossary; trigger covers every workflow step; requires arc42.
- `assets/skills/givn-design-review/SKILL.md.tmpl` — new mandatory
  ubiquitous-language check.
- `assets/skills/givn-review/SKILL.md.tmpl` — new mandatory
  ubiquitous-language check.
- `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` /
  `fix_domain_modeling_skill_trigger_e2e_steps.rs` — new steps.

No new modules.

## Data Model Changes

None — no manifest schema, artifact, or config-key changes.

## Open Technical Questions

None — the coupling decision (domain modeling requires arc42) is settled.
