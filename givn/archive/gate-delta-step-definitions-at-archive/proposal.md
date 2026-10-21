# Proposal: gate-delta-step-definitions-at-archive

## Release note

> Required. One complete, user-facing sentence naming what this change does.
> Every revision this change records carries the release note as its message subject, with the scenario title in the message body.

givn archive refuses a change whose delta step definitions are missing from the permanent test tree, naming the project-relative delta file and the missing step phrases (bounded with a remainder count).

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: archive-change
- Ideation topic: none — the change comes from a reproduced archive failure in a Python project, not from an ideation topic.
- Seed path: none
- Confirmed Personas: developer--primary-user, agent--technical-role, givn-maintainer--owner — all three are recorded on the owning use case and were confirmed by the operator for this change.

## Problem / Opportunity

Archive moves a change's delta feature files into the permanent specification
corpus, but it never moves step definitions. Step definitions are project
source: a delta copy may live beside the change's features because the runner
resolves step files relative to the feature root, while the permanent
capability step file is the durable home. When a developer forgets to mirror a
new definition into that permanent file, the archive still mutates the project,
publishes a candidate, and only then fails the merged suite with undefined
steps.

The failure has been reproduced twice in a Behave project: one archive failed
the post-move verification and needed a repair commit with the missing
definitions, and a second archive failed with 44 delta step phrases absent from
the permanent step files. Every occurrence costs a failed candidate, a manual
repair, and a rerun. The failure is always discoverable before the mutation:
the phrase set of the delta step files can be compared with the permanent step
files while the change is still active.

## Proposed Solution

Before givn archive mutates anything, it compares the step phrases bound by the
change's delta step-definition files with the phrases bound by the permanent
test tree — the project root minus the change, archive, history,
version-control, build, and cache trees. A step definition is a file under a
directory named `steps` or ending in `_steps`, or a file whose name ends in
`_steps.<ext>`. The phrases are the literals bound by the step bindings the
file declares; the comparison reads each binding expression whole, so a
multi-line decorator, an adjacent Python literal, and a raw-string regex all
yield their complete phrase text.

- If any delta phrase has no permanent definition, the archive stops before
  the move, exits non-zero, and reports the project-relative delta file
  together with the missing phrases (a bounded list followed by a remainder
  count). The change stays active, so nothing was lost and nothing needs
  unpicking.
- The refusal names the repair: merge each definition into its permanent
  per-capability step file, keep the delta copy for the change suite, and
  rerun the archive.
- When every phrase is present, the archive proceeds exactly as before. The
  delta copies are not moved or deleted; they travel into the archive as
  history.
- A resumed unproven candidate applies the same comparison before it verifies
  the candidate again: a candidate whose permanent tree is still missing a
  delta phrase refuses instead of being re-verified and reported unproven
  again.

The behaviour is independent of the test language: Behave, Cucumber, and Rust
step files all expose their bindings as source text.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. When route reports no signal,
> omit the recommendation column and record the decision with its rationale
> directly. Editing this table IS the declaration — `givn check review`
> compares what you write here against the delta you actually author, not
> against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| require-merged-step-definitions | `EXTEND givn-workflow-enforcement` (use case `verify-change`, score 17.00) | `NEW in archive-change` | Route matched the word "check": the proposal mentions `givn check review` and the capability text contains `route`. The observable behaviour is not a `verify-change` check; it is the archive mutation boundary and the permanent-corpus merge. Archive decides whether the merged corpus can run at all, before it publishes it. The owning goal is "archive a completed change with proven publication", so the capability belongs to `archive-change`. |

## Out of Scope

- Automatically copying or merging step definitions into the permanent test
  tree. Step definitions are durable project source, and archive never writes
  or relocates them.
- Validating the body of a step definition, or whether a definition that
  exists actually asserts anything. Existing runner strictness covers empty
  bodies.
- Recognising every possible binding syntax. Dynamic or unrecognised bindings
  are skipped; the resulting gap is recorded as a risk.
- Changing how a runner resolves `steps/` relative to the feature root. The
  project keeps its chosen layout.
- Removing the delta step copies at archive time.

## Open Questions

> One entry per question, with a stable ID, the full question with its option
> treatment (context, per-option meaning, gain, cost, and a concrete example),
> and a Disposition: answered (with its source), carried (with an owner and
> the artifact that will decide it), or out of scope (with a reason).

### D1 — Should archive merge delta step definitions into the permanent tree automatically?

- Question: When a change's delta step files contain a binding that the
  permanent test tree lacks, should `givn archive` copy or merge that
  definition into the permanent step file itself, or refuse before mutating?
- Options:
  1. **Refuse with a named repair** — archive compares phrase sets and stops
     before the move, reporting the delta file and the missing phrases; gain:
     the archive never publishes a corpus that cannot run, and no automated
     edit touches project source the maintainer did not review; cost: the
     developer must make one more edit before the archive proceeds; in
     practice: `givn archive --change run-benchmark-stage` exits non-zero
     before the move, names `specs/archive-change/benchmark_steps.py`, and the
     developer merges the phrases into `givn/specs/.../steps/..._steps.py`.
  2. **Auto-merge the definitions** — archive appends missing definitions to a
     target step file; gain: one less manual step; cost: archive writes
     project source it cannot validate, chooses a target file, and edits code
     that the developer is responsible for; in practice: archive appends a
     Python function into `benchmark_steps.py`, possibly duplicating a helper
     or importing nothing the body needs.
  3. **Leave the archive behaviour unchanged and rely on post-move
     verification** — gain: no new gate; cost: every occurrence loses a
     candidate and requires a repair commit, which is the reproduced failure;
     in practice: a 44-phrase gap is discovered only after the candidate is
     published and the merged suite fails.
- Disposition: answered — the design decision recorded for this change
  (option 1; see `design.md` §Decisions, D1). The two reproduced archive
  failures in the Behave project are the evidence that option 3 costs a
  candidate per occurrence, and option 2 would make archive an author of
  project source.

### D2 — How broadly should a delta step definition be recognised?

- Question: Which files count as step-definition files, and which bindings
  count as step phrases, when the change and the permanent tree are compared?
- Options:
  1. **Path-and-binding scanner heuristic** — a file under a directory named
     `steps` or ending in `_steps`, or named `*_steps.<ext>`, counts, and its
     supported binding tokens (`#[given(`, `#[when(`, `#[then(`; `@given(`,
     `@when(`, `@then(`, `@step(`, `@Given(`, `@When(`, `@Then(`, `@And(`,
     `@But(`, `@Step(`; `Given(`, `When(`, `Then(`, `And(`, `But(`, `Step(`;
     `.Given(`, `.When(`, `.Then(`, `.Step(`) are read as whole expressions,
     yielding the first resolvable string, raw-string, or regex literal; gain:
     one implementation covers Behave, Cucumber-JVM, Cucumber-JS,
     Cucumber-Ruby, and Rust without new dependencies or a parser; cost: a
     false positive can block the archive, and a binding whose phrase is
     computed dynamically is skipped; in practice: a `@given("an imported
     catalog")` decorator and a multi-line decorator split across adjacent
     literals are both recognised, while a decorator built from a string
     variable is not.
  2. **Runner-specific parsing** — parse each language's binding AST; gain:
     fewer false positives and negatives; cost: a parser per framework, new
     dependencies, and a gate that changes with every runner feature; in
     practice: correct for Behave today and unmaintained for the next
     framework.
  3. **Text search for the phrase anywhere in the permanent tree** — gain:
     simplest possible check; cost: a phrase mentioned in a comment or a
     docstring satisfies the gate while the definition is still absent; in
     practice: the archive passes and the merged suite fails with undefined
     steps.
- Disposition: answered — the design decision recorded for this change
  (option 1; see `design.md` §Decisions, D2). The heuristic's bounded cost is
  recorded as a risk on the heuristic gate itself, not as a silent limitation.
