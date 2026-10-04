# Proposal: release-truth

## Release note

One repository workflow prepares and publishes every Givn release, and its changelog lists each user-visible change exactly once, taken from the change's release note.

## Use-Case Context

- Use-case ID: publish-a-release
- Ideation topic: none
- Seed path: none
- Confirmed Personas: givn-maintainer--owner

The maintainer prepares the release and is the reader of the published release
notes; the persona's truth-keeper segment owns the check that a release claim
matches the implementation.

## Problem / Opportunity

Preparing a release is one operation split across two workflows: the
preparation workflow bumps the version, writes the changelog, and pushes the
tag; the tag push then starts the release workflow, which validates the tag
again and creates the GitHub release. Both halves run the whole repository
gate suite, and a maintainer resuming a half-finished release must know which
half to dispatch.

The changelog's selection rules do not deliver what a reader needs:

- The range since the last version tag (`v0.4.0`) holds 912 revisions. 539 of
  them are documentation, refactoring, test, chore, archive, CI, or planning
  revisions, and the current configuration still groups those into
  "Documentation", "Refactoring", and "Other Changes" sections.
- Repeated revisions of one change all carry the change's release note as
  their subject, and the configuration does not collapse them: the canonical
  dialog policy change alone carries 13 feature revisions with an identical
  note (ten under one capability, three under another) plus seven test
  revisions; the agent-target change carries eleven revisions.
- Every unmatched revision lands in "Other Changes", so a range without any
  user-visible change still produces a release section instead of stopping.

The repository's own history shows the split flow is easy to bypass: only
`v0.4.0` carries a version tag, and the 0.7.0 and 0.8.0 sections were written
by hand instead of generated, while the file header claims every release
section is generated with git-cliff. The next run of the current preparation
workflow would start its range at `v0.4.0` and repeat the changes that those
hand-written sections already describe.

## Proposed Solution

The maintainer runs one release operation: a single repository workflow
dispatch naming the release version.

1. The operation checks that the version is newer than the package manifest's,
   that no version tag exists for it, and that the version the repository
   declares current already carries a matching version tag; otherwise it names
   the missing tag and stops.
2. It bumps the package manifest and the lockfile, generates the release
   section from the revisions since the last version tag, and stops before
   committing, tagging, or publishing when that section has no user-visible
   change.
3. It runs the repository gates on the prepared state, records one release
   revision, creates the annotated version tag, and publishes a GitHub release
   whose notes are the generated release section.
4. A dispatch naming an existing annotated version tag revalidates that tag
   and publishes its GitHub release when it is missing, without regenerating
   or committing anything.

A release section lists only user-visible change: breaking changes, features,
bug fixes, performance changes, and reverts. Each change appears exactly once,
with the change's release note as its entry text. Documentation, refactoring,
tests, test-harness fixes, chores, cleanup, and merges never appear.

The GitHub release is the only publication channel: the crate is not published
to crates.io, and no release binaries are attached.

## Capability Routing

`givn spec route --change release-truth` reports `EXTEND apply-a-managed-upgrade`
(use case `upgrade-a-project`, score 17.00, confidence: signal). That signal is
driven by this proposal's out-of-scope mention of `upgrade-a-project`; no
release capability appears in the ranked list. The decision stays with the
release's own use case and is recorded here.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| describe-the-release | `EXTEND apply-a-managed-upgrade` (use case `upgrade-a-project`, score 17.00) | `EXTEND describe-the-release` | The proposed behavior is release metadata and changelog truth. The use case `publish-a-release` already owns the changelog contract — its capability scenario "The changelog describes the complete 0.3.0 release" — and its success guarantee "The release describes itself truthfully". Route's top candidate matches only through the Out of Scope sentence about consumer-project releases. |

## Out of Scope

- Rewriting, renumbering, or re-publishing already published release sections
  (0.2.0 through 0.8.0 stay as they are).
- crates.io publication, release binaries, a shell installer, or Homebrew.
- The migration catalog and its bundles.
- Releasing consumer projects (`upgrade-a-project`).
- Retroactive releases for versions that were documented without a tag; the
  range guard only requires the one-time baseline tag named in the design
  rollout.

## Open Questions

### D1 — Does a pushed version tag still publish a release?

- Question: Should pushing a version tag to the repository still validate the
  tag and publish its release, or is a manual dispatch the only release entry
  point?
- Context: today the preparation workflow pushes the tag, and that tag push
  starts the release workflow; anyone with push access also reaches a release
  by pushing a version tag directly, and that path skips the preparation
  state. The watn-style flow has one entry point: the release job runs in the
  same workflow run as the preparation, and an existing tag is revalidated and
  published only when a maintainer dispatches with it.
- Options:
  1. **Dispatch only** — a pushed tag starts nothing; the same run that
     prepared the release publishes it, and a dispatch with the tag serves
     recovery; gain: exactly one gated path, and no way to reach a release
     without the preparation gates; cost: a hand-pushed tag needs an explicit
     dispatch before it becomes a release; in practice: pushing `v0.9.0`
     manually changes no release until `release.yml` is dispatched with
     `tag=v0.9.0`.
  2. **Keep the tag-push trigger as well** — a pushed version tag starts the
     validation and publishes the release; gain: the current behaviour is
     preserved, and a hand-pushed tag needs no dispatch; cost: two entry
     points, one of which knows nothing about the preparation state, so the
     workflow must keep supporting a release from raw tag content; in
     practice: pushing `v0.9.0` from a local checkout publishes it directly.
- Disposition: carried — design decides; owner: design.

### D2 — Which range does the next release section cover?

- Question: When the newest version tag is older than the version the package
  manifest declares (today `v0.4.0` versus `0.8.0`), does the release refuse
  to prepare until a matching tag exists, or generate the range from the older
  tag?
- Context: the release section is generated from `last-tag..HEAD`. The
  repository documents 0.7.0 and 0.8.0 without version tags, so the newest
  tag is `v0.4.0` and a generated range would span 912 revisions, repeating
  what the hand-written sections already describe.
- Options:
  1. **Refuse until the current version is tagged** — the operation names the
     missing tag and stops; gain: every generated section covers exactly the
     unreleased work, and the tag set becomes complete again; cost: the
     maintainer creates the one-time annotated `v0.8.0` tag on the 0.8.0
     release revision before the next release, which also publishes that
     version as a tagged revision; in practice: the next prepare run fails
     with "no version tag matches 0.8.0" until that tag exists.
  2. **Generate from the older tag anyway** — the range starts at `v0.4.0`;
     gain: no preparatory action; cost: 0.9.0's notes repeat the dialog-policy
     and agent-target changes that 0.8.0's section already describes, and the
     reader cannot tell which release introduced what; in practice: 0.9.0's
     section lists changes from five weeks of work.
  3. **Record a baseline revision per release in the workflow** — the range
     starts at a hand-maintained commit; gain: no new tag; cost: a second
     hand-maintained value that drifts silently and is invisible to readers;
     in practice: the workflow carries a baseline hash that the maintainer
     updates at every release.
- Disposition: carried — design decides; owner: design.
