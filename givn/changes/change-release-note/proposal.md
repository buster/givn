# Proposal: change-release-note

## Release note

Give every change one release note, shared as the subject of all its
revisions.

## Use-Case Context

- Use-case ID: create-change
- Ideation topic: none
- Seed path: none
- Confirmed Personas: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Problem / Opportunity

Givn's generated implementation guidance requires one revision per scenario
whose message subject is the scenario title. A change that modifies ten
scenarios therefore records ten subjects; a changelog derived from those
revisions lists ten lines for one feature. A change that deviates with a
shared summary instead produces incomplete, unreviewed sentences.

Observed in a consumer project, release 0.5.0: the changelog lists 34 lines
for a single feature, eleven of them exact duplicates; a later change that
deviated to shared summaries produced fragments such as "the amount in both
views, silence, and the guards". The scenario title is the implementation's
unit of work, not the release reader's unit of meaning. No artifact currently
holds one authored, reviewable sentence that names what a change does.

## Proposed Solution

A change owns one release note: a complete, user-facing sentence that names
what the change does. The proposal carries the release note as a required
field, so it is authored and reviewed before any implementation.

Every revision a change records carries that release note as the subject of
its revision message; the scenario title is carried in the message body. The
rule is the same for feature, characterization, and end-to-end revisions, and
for both SCM backends.

The generated implementation guidance states the rule, so every revision of a
multi-scenario change records the same subject. A changelog derived from
revision subjects then lists exactly one entry per change: the release note.
Example: instead of 34 lines, release 0.5.0 carries one line — "Explain an
existing shell command without executing it".

## Capability Routing

`givn spec route` reports no signal; the decision is recorded with its
rationale.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `change-release-note` | no signal | `NEW in create-change` | The release note and the shared-subject rule are generated guidance about a change's artifacts; create-change owns the canonical guidance set. This is a new behavior, not another pointer in prose-consolidation. A project's changelog generator stays project-owned and out of scope. |

## Out of Scope

- Changelog generators and their configuration in consumer projects; Givn
  states the rule, it does not render a project's changelog.
- Rewriting existing revision history or existing changelogs.
- Givn's own release notes and migration bundles (`publish-a-release`).
- Previously archived changes; the rule applies to changes authored after the
  guidance refresh.

## Open Questions

### D1 — Where does a change's release note live?

- Question: Which artifact records the change's release note, and when is it
  authored?
- Options:
  1. **In the proposal** — the release note is a required proposal field,
     authored before specs; gain: reviewed before any implementation; cost:
     it can drift when the change's scope shifts late; in practice: the
     proposal carries `Release note: <sentence>`.
  2. **In the tasks plan** — the release note heads the change's task list;
     gain: authored when the work is understood; cost: reviewed later, and
     the proposal stays silent about the user-visible outcome; in practice:
     tasks.md carries `Release note: <sentence>` above the task sections.
  3. **In a dedicated change file** — `release-note.md` authored at archive;
     gain: always current at release time; cost: a late, separate review
     surface; in practice: `givn/archive/<id>/release-note.md`.
- Disposition: answered — this proposal chooses option 1: the note is
  authored and reviewed before implementation, where a missing or drifting
  sentence is visible at the proposal gate instead of at release time.

### D2 — Does a one-scenario change author a release note?

- Question: Does the rule also apply when the change records exactly one
  scenario?
- Options:
  1. **Always** — every change authors a release note; a one-scenario change
     may reuse the scenario title as the release note; gain: one authoring
     rule and one changelog entry per change; cost: authors repeat themselves
     for trivial changes; in practice: a bug-fix change carries
     `Release note: <scenario sentence>`.
  2. **Only multi-scenario changes** — a one-scenario change keeps the
     scenario title as the subject; gain: no extra writing; cost: two rules
     to remember and to review; in practice: the same change type has two
     message shapes.
- Disposition: answered — this proposal chooses option 1: one rule, one
  changelog entry per change, no special case to review.

### D3 — Does `givn commit` act on the release note?

- Question: Is the shared subject guidance only, or does the commit command
  compose or check the subject from the recorded release note?
- Options:
  1. **Guidance only** — the implementation agent composes subject and body;
     `givn commit` is unchanged; gain: no command change; cost: a deviating
     subject is caught by review, not by the tool; in practice: the agent
     records a message whose subject is the release note and whose body names
     the scenario.
  2. **Compose** — `givn commit` reads the recorded release note and builds
     the subject itself; gain: the rule cannot be violated; cost: the command
     must know the release note, and a human's message handling changes; in
     practice: the agent passes the scenario title and the command emits
     subject `<type>(<capability>): <release note>`.
  3. **Warn** — `givn commit` warns when the subject differs from the
     recorded release note; gain: drift is visible without blocking; cost: a
     warning path to test on both backends; in practice: the same command
     prints `warning: subject differs from the change's release note`.
- Disposition: carried — design decides; owner: design.
