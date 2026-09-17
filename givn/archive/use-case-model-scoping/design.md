# Design: use-case-model-scoping

## Technical Approach

Guidance and documentation change with one template edit. No parser change, no
new warning, no router change. The eight rules and ten rejection criteria are
prose in the assets; the only Rust change is the text of
`usecase_md_template()`.

## Canonical Definition and Rules

The definition and the eight scoping rules live in three places with one
owner each:

- `assets/skills/givn-ideation/SKILL.md.tmpl` — the authoring surface: the
  definition, the eight rules, the authoring requirements, and the Handoff
  mapping. This is what the agent reads while cutting and elaborating.
- `assets/instructions/review.md` — the rejection criteria R1–R10, each tied
  to the observable document section that proves it.
- `docs/use-case-model.md` — the durable reference: definition, rules,
  authoring requirements, criteria, and the worked before/after example.

The definition, verbatim:

> A use case is a contract between one primary actor and the system: the
> actor's single goal, stated as observable outcome, with the success and
> minimal guarantees the system makes while pursuing it. A use case stays
> true if the system is rebuilt with different technology.

The eight rules, each with its falsification test:

| # | Rule | Falsification / test |
|---|---|---|
| 1 | One goal, one primary actor | An "and" joining two verb phrases in Goal, or two distinct outcomes in the success guarantee, means split |
| 2 | Goal-shaped, never component-shaped | Placement test: which actor goal does completing this advance? Channel is never an answer |
| 3 | Level discipline | `!` owns capabilities directly; `+` owns only child `!`; a leaf with three or more distinct actor goals must be promoted and split |
| 4 | Mutation containment | An observing case owns zero mutating interactions |
| 5 | Extension discipline | Every extension names its main-flow step and the guarantee it protects; a new outcome is never an extension |
| 6 | Precondition and flow consistency | Preconditions hold before the trigger; the success guarantee is established by the flow; the trigger is an actor action or event |
| 7 | Implementation independence | Mechanism vocabulary never appears in Goal, Trigger, Main flow, or Guarantees |
| 8 | Relationship semantics | `Includes` is unconditional reuse; `Extends` is optional insertion with a stated condition and never targets fragments |

Authoring requirements in the same skill section: the first actor is the
primary actor and additional actors are marked `Supporting:`; main-flow steps
are numbered and pair actor intention with system responsibility; extensions
use `- <step>: <condition> → <handling>; protects <guarantee>`; Minimal is
true even on failure; Success is the goal's observable outcome and nothing
else; Out of scope names excluded actor goals.

## Review Rejection Criteria

`assets/instructions/review.md` gains a "Use-case scope rejection criteria"
block with R1–R10: goal conjunction; missing primary actor; placement
failure; mutation under an observing goal; extension without a step
reference; inconsistent preconditions or flow; mechanism vocabulary;
component- or channel-shaped name; oversized leaf; relationship misuse.
`assets/instructions/design-review.md`'s use-case branch points at the
criteria instead of restating them.

## Specs and Spec-Skill Guidance

`assets/instructions/specs.md` and `assets/skills/givn-spec/SKILL.md.tmpl`
gain the capability placement test for `NEW in <leaf>` routing: which actor
goal does completing this capability advance; a channel is never an answer; a
variant of an action lives under the action's goal; capability names are
goal-shaped, never component- or channel-shaped. Both point at
`docs/use-case-model.md` as the durable reference.

## Durable Document

`docs/use-case-model.md` holds the definition, the eight rules, the authoring
requirements, the ten rejection criteria, and the worked example: an
observing mega-leaf split into focused leaves and a substrate fragment, and a
conjunction-goal leaf promoted to a summary and split into the goals ideation
already named. The document names the sections that prove each rejection
criterion. `docs/workflow.md` §3 and `docs/reference.md` point at it.

## Template Guidance

Reach note: no command emits `usecase_md_template()` today. It is the source
for generated `usecase.md` fixtures and for the archive tests that install a
new use-case document; the author-visible guidance surfaces are the generated
ideation skill, the specs instruction, and `docs/use-case-model.md`. The
template edit keeps the document skeleton aligned with those surfaces and is
guarded by the delta scenario; adding a `givn eject usecase:template` action
would introduce a new CLI action and is out of scope.

Trade-off: because the parser accepts guidance only inside its fixed shapes,
the primary-actor convention sits in the Actors bullet, the extension format
replaces the `- none` example, and the promotion note sits in the Goal text.
Those fields are slots an author replaces; no lint or tree code path consumes
the parsed extensions list, so the trade-off is acceptable and the parser
stays untouched.

`src/gherkin/usecase_md.rs::usecase_md_template()` gains:

- the primary-actor convention in the Actors bullet: "the first actor is the
  primary actor; mark additional actors `Supporting:`";
- a numbered main-flow example pairing actor intention with system
  responsibility;
- the extension format bullet `- <step>: <condition> → <handling>; protects
  <guarantee>`;
- the promotion note in the Goal placeholder: a leaf whose flow names three or
  more distinct actor goals, or whose conjunction success guarantee, must be
  promoted to `+` and split.

Parser tolerance: Actors, Preconditions, Extensions, Rules, Examples, and
Personas are plain bullet lists, and Goal/Trigger/Main flow/Guarantees/Out of
scope are free text; the added guidance stays inside those shapes, so
`parse_usecase_md` needs no change. Existing fixture placeholders
(`<actor role>`, `<the domain goal>`, `<capability-id>`, `<scenario title>`)
are preserved so `tests/steps/mod.rs` and `tests/steps/spec_discovery_deterministic_steps.rs`
keep replacing them. No new warning is added, and `givn lint` behavior for
existing corpora is unchanged.

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| The durable use-case model publishes the canonical definition and rules | "contract between one primary actor and the system", "promotion rule", "Placement test", all ten criterion titles ("Goal conjunction" through "Relationship misuse"), the worked observing split and its R4/R8 tie-ins, the worked conjunction promotion and its R1/R9 tie-ins, and the two document references (`docs/workflow.md` and `docs/reference.md` both contain `use-case-model.md`) |
| The generated use-case document template carries the authoring guidance | "the first actor is the primary actor", "protects <guarantee>", "System responsibility", "promoted to `+`", and the six fixture placeholders (`<usecase-id>`, `<capability-id>`, `<actor role>`, `<the domain goal>`, `<consumer action>`, `<scenario title>`) |
| The ideation guidance carries the canonical definition and the eight scoping rules | "contract between one primary actor and the system", the eight rule titles, "promotion rule", "three or more distinct actor goals", "Placement test" |
| The ideation guidance carries the authoring requirements and the handoff mapping | "the first actor is the primary actor", "Supporting:", "protects <guarantee>", "never an extension", "never to a capability inside another goal" |
| The review instruction carries the use-case scope rejection criteria | "Goal conjunction", "Missing primary actor", "Placement failure", "Mutation under an observing goal", "Extension without a step reference", "Inconsistent preconditions", "Mechanism vocabulary", "Component- or channel-shaped name", "Oversized leaf", "Relationship misuse" |
| The design-review guidance points at the use-case scope criteria | "use-case scope rejection criteria" |
| The specs guidance carries the capability placement test | "placement test", "Channel is never an answer", "goal-shaped" |

The template scenario's main-flow step asserts "System responsibility" — a
phrase only the new numbered pairing produces — so it cannot pass on the
current single `<first step>` line. The lint acceptance criterion needs no
scenario: the only production change is a string constant with no parser or
lint code path, so the existing lint suite is the evidence.

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| A capability has exactly one use-case owner | `givn/specs/make-usecases-canonical/usecase.md` | Placement test and rejection criteria R3/R8 gate ownership | The review instruction carries the use-case scope rejection criteria |
| The corpus remains inspectable when a narrative is invalid | same | No parser or lint change; guidance only | Existing `givn spec tree` and lint scenarios stay green |
| Generated ideation guidance is synchronized from canonical assets | `givn/specs/author-change/usecase.md` | Definition, rules, and authoring requirements live in the ideation skill template | The ideation guidance scenarios |
| Instruction and skill guidance stays complete | `instruction-integrity` | Review and specs instructions carry the criteria and the placement test | The review/specs scenarios |

Persona impact: `agent--technical-role` consumes the generated guidance and
the template; `givn-maintainer--owner` owns the model document and the
criteria. No Persona is invented or promoted; Gherkin actors stay the
Developer and the Authoring agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| canonical-usecase-contract | `tests/steps/spec_discovery_deterministic_steps.rs` | Direct read of `docs/use-case-model.md` and the in-process `usecase_md_template()` call (the existing file already calls that function) |
| ideation-skill | `tests/steps/prose_consolidation_steps.rs` | Reuse: the existing generic generated-guidance regex steps; no new file |
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` | Direct asset read for the review and specs instructions, matching the existing steps in that file |

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/use-case-model-scoping/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI for the generated-guidance scenarios; direct file
  and template reads for the contract scenarios.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| canonical-usecase-contract: inspect the use-case tree (existing) | Use-case tree reports explicit use-case ownership (existing) | CLI | Real `givn spec tree` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | A consumer synchronizes skills and every agent target receives the ideation facilitation skill (existing) | CLI | Real `givn skills sync` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | An agent requests machine-readable artifact guidance (existing) | CLI | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end,
and the new scenarios are guidance-content and document variants. No browser
UI exists.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::gherkin::usecase_md::usecase_md_template` and the skill renderer path |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

Guidance content, one template string, and one new project document. No
parser, router, gate, manifest, or runtime behavior changes. The
`singleton_usecase` warning and all other lint findings are untouched.

## Risks

- **Phrase drift (most likely failure).** The definition, the eight rule
  titles, the ten criterion titles, and the authoring-requirement phrases are
  written by hand into three artifacts (the ideation skill, the review
  instruction, the durable document) and matched as exact substrings by the
  spec. Mitigation: copy the phrases verbatim from this design into each
  artifact and run the delta scope after each artifact edit.
- Fixture placeholder breakage: guidance appended inside a placeholder would
  silently stop the fixture replacements. Mitigation: the template scenario
  asserts all six placeholders survive.
- Prose-only enforcement: absorption can continue where reviewers are lax.
  Accepted risk; the structural fix (router and parser signals) is explicitly
  out of scope.
- The criteria must not become a rubber stamp. Each R-number names the
  observable document section that proves it, so a reviewer checks
  mechanically.
- The rules enable incremental repair (re-homing capabilities one at a time);
  the guidance does not demand a big-bang rewrite.
