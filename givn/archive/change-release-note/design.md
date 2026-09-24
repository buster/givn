# Design: change-release-note

## Overview

A change owns one release note, authored in its proposal. Every revision the
change records carries the release note as its message subject; the scenario
title moves to the message body. The rule is realized entirely in embedded
assets: the proposal scaffold, the proposal instruction, the generated
implementation guidance (skills and opencode commands), and the always-on
`AGENTS.md` hint. `givn commit` is unchanged and validates nothing (see
`Decisions`, D3): the implementation agent composes the subject from the
recorded release note, exactly as it composes the scenario title today.

## Interface classification

No screen is introduced or changed. The surface is CLI text output and file
content (`givn new` writes the scaffold, `givn instructions proposal` prints
the instruction, `givn skills sync` writes generated rules and the `AGENTS.md`
hint). Opposite-classification test: nothing in this change renders HTML, CSS,
or JavaScript in a browser, so no browser driver applies. Therefore no
`## Visual Design Contract` section exists, the visual review gate stays
inactive, and no screenshot or transcript is committed — consistent with
previous CLI-only guidance changes.

## Technology decisions

No versioned technology is chosen: the change edits embedded Markdown and
`.tmpl` assets plus durable documentation; no language, runtime, framework,
library, database, browser/driver, or container image is selected or upgraded.
The existing Rust binary embeds the assets at build time (`rust-embed`) and
the existing renderer substitutes `{{...}}` placeholders; no dependency is
added.

## Decisions

- Decides D3: **guidance only** — the implementation agent composes the
  subject from the change's recorded release note; `givn commit` neither
  composes nor warns. Realized in `assets/instructions/proposal.md`, the
  `givn-steps`, `givn-implement`, and `givn-tasks` skills and commands, and
  `assets/agents-project-hint.md`. Rationale: the delta spec contains no
  commit-command behavior, so a compose or warn path would be unverified
  behavior and an unrequested command contract change; drift stays visible
  in review of the recorded revision subjects.
- Decides D1: the release note is a **required proposal field**, authored and
  reviewed before any implementation. Realized in `assets/templates/proposal.md`
  and `assets/instructions/proposal.md`.
- Decides D2: **every change authors a release note**, including a
  one-scenario change; there is one authoring rule and one changelog entry per
  change. Realized in the same two assets, which state the requirement
  unconditionally.
- Self-application: this change's own proposal gains its `## Release note`
  section. It is the first change authored under the rule, and every revision
  this change records uses that subject.

## Exact file changes

| Path | Change |
|---|---|
| `assets/templates/proposal.md` | New `## Release note` section before the use-case context: the heading names the term, a guidance quote carries the rule, and the authored sentence follows. Heading only — no second label (see `Canonical wording`). |
| `assets/instructions/proposal.md` | New required-field rule plus the shared-subject rule; the structure list gains the release note as item 1. |
| `assets/skills/givn-propose/SKILL.md.tmpl` | Proposal structure list names the release note field. |
| `assets/commands/givn-propose.md` | Proposal-specific rules require the release note. |
| `assets/skills/givn-steps/SKILL.md.tmpl` | COMMIT step: subject is the release note, scenario title in the body. |
| `assets/commands/givn-steps.md` | Same COMMIT wording. |
| `assets/skills/givn-implement/SKILL.md.tmpl` | COMMIT steps (feature and `test(e2e)`) and the `## Committing` example. |
| `assets/commands/givn-implement.md` | Same COMMIT wording. |
| `assets/skills/givn-tasks/SKILL.md.tmpl` | COMMIT sub-tasks in both the feature and the `@e2e` loop. |
| `assets/commands/givn-tasks.md` | Same COMMIT wording. |
| `assets/commands/givn-characterize.md` | Characterization commit subject. |
| `assets/commands/givn-reverse-engineer.md` | Characterization task-list commit wording. |
| `assets/instructions/tasks.md` | COMMIT template line. |
| `assets/instructions/tasks-reverse.md` | COMMIT template line. |
| `assets/agents-project-hint.md` | The `givn-project` AGENTS.md commit bullet. |
| `README.md` | Quickstart commit sentence. |
| `docs/workflow.md` | §4 COMMIT rule. |
| `docs/arc42/12-glossary.md` | New **Release note** term. |
| `docs/arc42/08-crosscutting-concepts.md` | 8.7 records the release-note rule in the generated implementation guidance. |
| `givn/changes/change-release-note/proposal.md` | Self-application: its own `## Release note` section. |

No Rust source file changes. `src/cli/new.rs` writes the template through the
existing cascade, `src/cli/instructions.rs` prints the instruction through the
existing cascade, `src/cli/project_hint.rs` reads `agents-project-hint.md`
through the existing embedded-asset lookup, and `src/skills/targets.rs`
renders the skill and command templates with the existing placeholder engine.

### Canonical wording

Every surface reuses one sentence. The proposal scaffold names the section
with the `## Release note` heading and no second label; the guidance quote
below it carries the lowercase phrase the scaffold assertion reads. The
acceptance substrings hold verbatim, case-sensitively and line-wise:

```text
release note
carries the release note as its message subject
the scenario title goes in the message body
```

Each asserted phrase stays unbroken on a single line in the hand-wrapped
assets; the canonical sentence is never split across a line break. The
generated opencode commands carry the same sentence as their skill siblings,
and no changed surface retains the superseded phrase `message references the
scenario title`.

Feature, characterization, and `@e2e` revisions differ only in the conventional
type prefix (`feat`, `characterize`, `test(e2e)`); the subject is always the
release note. A one-scenario change may reuse the scenario sentence as its
release note (D2), and the proposal stays the single authoring site.

## Step definitions

All three scenarios reuse existing bindings; the capability adds no new step
file (the one-file-per-capability rule creates a file only when a capability
adds a binding). The reused locations are:

| Capability | Step definition files (reused) |
|---|---|
| change-release-note | `tests/steps/mod.rs` — initialised project root, real `givn new` invocation, exit-code, stdout, and directory/file assertions; `tests/steps/prose_consolidation_steps.rs` — all-target `givn init` fixture, generated-skill guidance reader; `tests/steps/run_execution_e2e_steps.rs` — quoted-file content assertion used for `proposal.md` and `AGENTS.md`. |

Test runner command (`verify.command`):

```text
./run-tests.sh
```

It executes `.feature` files from `givn/specs/**` and the change's
`specs/**` via `GIVN_FEATURES` — never a parallel hand-maintained test file,
so the executable-spec invariant holds.

Single-scenario run command (used by every RED/GREEN/REFACTOR check in
`tasks.md`), where `<title>` is the scenario title:

```text
GIVN_FEATURES=givn/changes/change-release-note/specs cargo test --test features_runner -- --exact "create-change/change-release-note.feature::<title>"
```

## Strict mode

The runner is Rust `cucumber-rs`. Strict mode is `.fail_on_skipped()` on the
`Cucumber` builder in `tests/features_runner.rs`, applied in both the plain and
the `--exact` nextest path; an undefined or skipped step fails the run and
exits non-zero. Not-implemented stub for this language:

```text
unimplemented!()  (or todo!())
```

Because every step for this capability is reused-proven, RED is a genuine
assertion failure (missing template field, missing instruction wording,
missing generated guidance wording), not an undefined step.

## Local runnability & digital twins

- **Local run command:** `./run-tests.sh` (regular) and
  `./run-tests.sh --e2e` (E2E subset). Both run the whole stack — the compiled
  binary plus the cucumber runner — in one step.
- **Isolated network:** not applicable. The change talks to no database,
  queue, HTTP service, or third party; fixtures are temporary directories.
- **Digital twins:** none. There is no external dependency to fake.

## E2E smoke test infrastructure

- **Interface type:** CLI — the real `givn` binary is invoked as a subprocess.
- **E2E runner command** (`verify.e2e_command`): `./run-tests.sh --e2e`,
  tag-filtered to `@e2e` (`GIVN_CUCUMBER_TAGS=@e2e` under nextest) and
  distinct from `verify.command`.
- **E2E step location:** the shared files named in `## Step definitions`; no
  separate capability file is created because no binding is added.
- **Local test infrastructure:** none beyond the repository — no browser, no
  server, no port, no container, no database, no fake service. The fixture
  initializes a temporary project by driving the real `givn init`, then drives
  `givn new`, `givn instructions proposal`, and `givn skills sync` as real
  subprocesses through `CARGO_BIN_EXE_givn`.
- **E2E framework choice and justification:** `cucumber-rs`, the existing
  runner. The real interface is a CLI, so a real subprocess invocation is the
  faithful driver; a browser or HTTP client would be the wrong boundary.
- **E2E strict-mode proof:** the same `.fail_on_skipped()` builder governs the
  `@e2e` path, so a skipped or undefined E2E step fails the run. All three
  scenarios are RED before the asset change: the scaffold lacks the field, the
  instruction lacks the rule, and the generated guidance lacks the wording.
- **Anticipated interface obstacles and their fix:** none. The one subtlety —
  the generated-guidance assertions must read exactly the targets
  `givn skills sync` writes — is handled by the existing all-target fixture
  (`givn init` selects `opencode, .agents, .claude` through stdin) and the
  reader resolves targets from the fixture's own `givn/config.yaml`, the same
  way a consumer would.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| create a change | A new change's proposal carries the release-note field | CLI | Cucumber launches the real `givn new shared-note` subprocess and reads the written scaffold's proposal file |
| read the proposal guidance | The proposal guidance requires a release note | CLI | Cucumber launches the real `givn instructions proposal --change shared-note` subprocess and asserts stdout |
| read the generated implementation guidance | The generated implementation guidance shares the release note | CLI | Cucumber launches the real `givn skills sync` subprocess and reads the generated skill guidance, the generated opencode commands, and `AGENTS.md` it produced |

Each inventory entry has exactly one matching `@e2e` scenario and each row has
a concrete subprocess driver. No browser UI exists, so a browser or HTTP
driver would be the wrong boundary. The generated-guidance scenario asserts
the three generated skills, the three generated commands, the absence of the
superseded phrase `message references the scenario title` across the generated
guidance, and the `AGENTS.md` hint — all through reused bindings, so the
command and instruction halves of the allowlist cannot silently keep the old
rule while the skills report GREEN.

## Black-Box-First policy

Every assertion runs through the real CLI subprocess. The file-content
assertions are primary observables of the commands' documented outputs —
`givn new` writes the scaffold, `givn skills sync` writes generated rules and
the `AGENTS.md` hint — not internal calls. No unit or in-process test is added
or retained, so the "which case does the internal test cover that the E2E does
not" question has no positive answer to justify one.

## Use-case traceability

Owning use-case: `create-change`. Ideation source: none. Capability:
`change-release-note` (NEW in `create-change`). Confirmed Personas:
`developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| Rule: a change's proposal carries one release note; every revision shares it as the message subject, with the scenario title in the message body | Proposal template field plus one canonical sentence reused across all guidance surfaces | The proposal guidance requires a release note; The generated implementation guidance shares the release note |
| Rule: generated guidance is synchronized from canonical source assets | Edit the embedded assets, then regenerate targets via `givn skills sync` (self-dogfood) | The generated implementation guidance shares the release note |
| Main flow 1: the developer creates a change and the system writes the scaffold | The scaffold template carries the required field | A new change's proposal carries the release-note field |
| Main flow 2: the developer reads the artifact guidance; one canonical policy per topic | The proposal instruction states the rule once; derived guidance points at the release note instead of restating a competing rule | The proposal guidance requires a release note |
| Persona impact | CLI text output; a released change's note is authored by the developer, read by the agent, and kept canonical by the maintainer | Personas named above; no browser screen, so no visual evidence |

## Architecture impact

Embedded-asset and generated-guidance change only. `cli::new`,
`cli::instructions`, `cli::project_hint`, and `skills::targets` keep their
current code paths and contracts; no module, public API, dependency, runtime
behavior, or archive behavior changes.

## Data model changes

None. No struct, schema, or persistence format changes; nothing parses the
release note at runtime (D3).

## Arc42 impact

- **12 Glossary:** the **Release note** term is added.
- **8.7 Skill and Rule Generation:** one sentence records that the generated
  implementation guidance carries the release-note commit rule.
- All other chapters: no impact — no new building block, runtime flow,
  interface, constraint, quality scenario, or risk.
- No ADR qualifies: the D3 choice has real alternatives but fixes no
  component boundary, durable contract, dependency direction, or system-wide
  quality constraint; its rationale's canonical home is `design.md`.

## Open technical questions

None.
