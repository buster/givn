# Design: persona-identity-reuse

## Technical Approach

Guidance plus two validation sites. The ideation skill and command gain the
identity criteria and the reuse protocol; a new persona template defines the
schema; `givn lint` gains an advisory dangling-ID finding; the review gate's
persona disposition resolves the ID to a project persona file. No new
artifact, manifest entry, or gate is introduced.

## Identity Criteria

`assets/skills/givn-ideation/SKILL.md.tmpl`'s Personas section gains the
criteria: a new persona exists only when it differs from an existing one in at
least one of **authority boundary** (what it may do or decide), **evaluation
capability** (what it can judge), or **own vocabulary** (terms it uses or
refuses). The criteria are disjunctive. Topic-specific demands go into a dated
`## Topic stances` appendix, never into the stable core. The three
adapter-implementer-shaped files collapse to one persona with three stance
blocks because they share the authority (ship an adapter against a port) and
the evaluation (conformance suite).

## Persona Template

New `assets/templates/persona.md`:

```markdown
# Persona: <name>

## Stable core

- Role: <one line>
- Authority boundary: <what it may do or decide>
- Evaluation capability: <what it can judge>
- Own vocabulary: <terms it uses or refuses>
- Success criterion: <what success means for it>

## Running record

Append-only, dated entries per review or topic stance. Never rewrite earlier
entries.

- <YYYY-MM-DD> — <entry>

## Topic stances

Topic-specific demands, dated, never in the stable core.

- <YYYY-MM-DD> — <topic>: <stance>
```

## Reuse-First Protocol

The Personas, Topic state, and Handoff sections of the ideation skill, plus
the guardrails in `assets/commands/givn-ideation.md`, carry the four steps:

1. Inventory `givn/personas/` and every sibling topic's `personas/` into a
   session-state table against the topic's stakeholder types.
2. Match by ID, or record a one-sentence new-segment justification naming the
   distinguishing criterion.
3. A reused persona's topic-local file contains a header line plus new
   running-record entries only — never a biography copy.
4. Handoff merge appends record entries to the project persona; the user
   confirms the merged diff; the topic file stays as historical record.

One standardized header line:

```text
Reuse: <persona-id>, base <short-hash or date>; this file adds record entries only
New: <persona-id> — <distinguishing criterion>
```

The promotion sentence and the glossary define merge as append-record, not
replace-biography. Preserved behaviors, kept verbatim where they exist today:
user confirmation before use, the review duty, the cap ("at most three
segments per stakeholder type, about ten in total"), advisory findings with
user arbitration, handoff promotion with "never auto-promote", and the
actor/persona distinction.

## Lint Advisory

New function `check_persona_references(project_root)` in `src/cli/lint.rs`,
alongside the seed check:

- Scan homes: `givn/changes/*/specs/*/usecase.md` (active change use-case
  documents) and `givn/ideation/*/use-cases/*.md` (ideation use cases).
- Resolution homes for a `## Personas` entry: `givn/personas/<id>.md` or any
  `givn/ideation/*/personas/<id>.md`. A reference resolves when any home
  holds the file; forward references during an active topic therefore stay
  quiet. Matching is ID-only: the entry text must equal a persona file's name
  stem. givn's own ideation use cases use persona titles rather than IDs, so
  the change lands with about twenty advisory warnings on this repository's
  own corpus; the exit code is unchanged, and migrating those labels is out
  of scope.
- Finding shape: `kind = "persona_ref"`, `level = "warning"`, label `[PERS ]`,
  message `persona reference '<id>' matches no project or topic persona file`.
- Advisory: counted in the summary's finding count, never sets the exit-code
  flags. The scan is a no-op without the scan homes. It runs in the default
  whole-tree scope and in the `--change` scope (that change's documents);
  explicit paths keep their current scope.

## Review-Time Validation

`src/integrity_hook.rs::check_persona_disposition` gains the personas root.
The change directory is `givn/changes/<id>`, so the project root is three
levels up — the same derivation `check_promised_files_exist` already uses —
and the personas root is `<project_root>/givn/personas`. After the existing
empty/none checks, the value must resolve to `givn/personas/<value>.md`; an
unresolved ID fails with a message naming the value.

Behavior note: today a declared visual interface with
`PERSONAS: none — operator confirmed` passes because only the exact value
`none` is rejected. After this change it fails, because no such persona file
exists — and that is intended: a declared visual interface is a user-visible
surface, so the reasoned none belongs only to a change without one. The delta
spec pins both the failure and the exempt-path success.

Fixture consequence: the visual-review-gate fixture's review.md carries
`PERSONAS: fixture-persona`, so the fixture project must create
`givn/personas/fixture-persona.md`; otherwise the permanent passing
scenarios would fail. The shared Given `a givn project with a change that has
passed design-review` creates the file, covering every permanent and delta
scenario that reaches the check.

## Glossary

`docs/arc42/12-glossary.md`: the `Persona segment` row gains the three
identity criteria and the disjunctive rule; the `Persona promotion` row
becomes append-record with a user-confirmed merged diff, never a biography
replacement.

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| The ideation guidance carries the persona identity criteria and the reuse protocol | "authority boundary", "evaluation capability", "own vocabulary", "topic stance", "inventory", "Reuse:", "New:", "append-only", "one persona with three stance blocks", "record entries only" |
| The ideation guidance keeps the confirmed persona behaviors | "stakeholder type", "persona segment", "persona promotion", "every artifact that affects", "at most three", "never auto-promote", "confirms every persona", "arbitrates", plus the generated givn-ideation command's "inventory", "match", and "separate from Actors" |
| The persona template carries the stable core and the running record | reads `assets/templates/persona.md`: "authority boundary", "evaluation capability", "own vocabulary", "success criterion", "Running record", "append-only", "Topic stances" |
| A visual change with an unknown persona disposition fails review | exit non-zero, stderr names "unknown-persona" |
| A visual change with a reasoned none fails review | exit non-zero, stderr names the value |
| A change without a visual interface passes review with a reasoned none | exit 0 (guard; immediate GREEN by reuse) |
| A dangling persona reference is reported as an advisory | exit 0, stderr `[PERS ]` + the ID |
| A persona reference resolved by a project persona is not reported | exit 0, no `[PERS ]` finding |
| A persona reference resolved by a topic persona is not reported | exit 0, no `[PERS ]` finding |
| The change scope reports a dangling persona reference | `--change` scope, exit 0, stderr `[PERS ]` + the ID |

Two scenarios are legitimate immediate-GREEN guards: the exempt-path reasoned
none and the preserved-behaviors scenario. They are recorded as such in
tasks.md; the unknown-persona review scenario and the dangling-reference
scenarios are true REDs.

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| Personas are confirmed before use and review affected artifacts | `givn/specs/fragments/ideation-skill.feature` | Preserved behaviors kept verbatim; criteria and protocol added | Both ideation scenarios |
| A declared visual interface requires a confirmed Persona | `givn/specs/verify-change/visual-review-gate.feature` | Review-time ID resolution | The unknown-persona scenario |
| Lint stays a static check with a stable exit-code contract | `givn/specs/verify-change/usecase.md` | Advisory finding never changes the exit code | The dangling-ID scenario |

Persona impact: `agent--technical-role` (generated guidance) and
`givn-maintainer--owner` (rule corpus, glossary). No Persona is invented or
promoted; Gherkin actors stay the maintainer, reviewer, and agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| visual-review-gate | `tests/steps/visual_review_gate_steps.rs` | Real `givn check review` subprocess; the fixture creates the persona file for the passing path |
| lint-command | `tests/steps/mod.rs` | Real `givn lint` subprocess; adds two Givens (record a persona in the generated use-case document; create a project persona file) |

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/persona-identity-reuse/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI for the lint and review scenarios; direct asset
  reads for the guidance scenarios.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| visual-review-gate: run the review check on a visual change (existing) | A visual change with complete evidence passes review (existing) | CLI | Real `givn check review` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing) | CLI | Real `givn skills sync` subprocess |
| lint-command: run the static lint check | A duplicate title across two permanent spec files is a hard error (existing, sibling `spec-overlap-gates` capability) | CLI | Real `givn lint` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end,
and the new scenarios are guidance-content and advisory variants. No browser
UI exists.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::cli::lint::check_persona_references` and `givn::integrity_hook::check_persona_disposition` |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

Guidance content, one template asset, one advisory lint finding family, and a
review-time ID resolution. The exit-code contract, the visual gate's evidence
rules, and the artifact graph are unchanged.

## Risks

- The criteria may be thin for non-workflow domains. They stay disjunctive and
  the stance appendix stays open-ended.
- Record-only topic files are less self-contained. Accepted because IDs are
  stable and the project file is the single biography home.
- Parallel topics appending to one project persona can collide. Entries stay
  topic-tagged and append-only, and the user confirms the merged diff.
- The validation is asymmetric by design: warning at lint time, enforcement at
  review time, so active ideation forward references do not break.
