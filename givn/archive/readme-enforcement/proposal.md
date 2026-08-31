# Proposal: readme-enforcement

## Problem / Opportunity

The readme addon is a one-time scaffolder: it writes an initial README and then
never touches it again. Nothing in the workflow requires anyone to fill the
placeholder sections, and nothing notices when the README drifts from the
project it describes.

The observable result: a project can run the full givn lifecycle — including a
successful first iteration with green gates — and still ship a README whose
description, quick start, usage, and license sections contain only scaffold
requests-for-content ("TBD" license, comment-only sections). Real case: a
first-iteration project ended with an essentially empty README because no gate,
no check, and no instruction ever pointed at it.

Two distinct gaps:

1. **Initial completeness** — the scaffold's open placeholders are never
   required to be filled.
2. **Ongoing currency** — nothing records whether a change touched the
   project's public face, and the parts of the README givn could keep
   synchronous (the development commands) drift silently.

## Proposed Solution

When the readme addon is enabled:

1. **Marked placeholders.** A scaffolded README marks every section that still
   awaits real content with a machine-recognizable open placeholder. A user
   reading the README sees the same request-for-content as before; a check can
   now see it too.

2. **Readable health check at any time.** `givn readme check` reports every
   open placeholder by section and what is expected there. It passes only when
   none remain. It also reports damage to the development managed block, and
   it refuses a README that carries no givn markers at all: with the addon
   enabled, the project owner either adapts the README to the marked form or
   disables the addon. A missing README is reported the same way.

3. **Recorded documentation decision per change.** The review artifact records
   an explicit README impact decision: either the README needed no update for
   this change, or it was updated (naming the sections). The review step
   refuses sign-off without that recorded decision. This makes "we forgot the
   README" impossible to do silently — the decision must be made and written
   down every change.

4. **Archive blocks on an unfinished README.** Archiving a change fails while
   the README still carries open placeholders. The failure names each
   unfinished section and its expected content, so the fix is immediate.

5. **Owned parts stay synchronous.** At archive time, givn refreshes the
   README's development-commands section from the project's configured
   workflow, so what the README claims and what the workflow runs cannot
   diverge. The same refresh runs when the project is first scaffolded.

6. **Migration for existing projects.** A project scaffolded before this
   change still has the old-style README, and a project that adopted the addon
   onto a hand-written README has no markers at all — both fail the health
   check. givn recognizes the previous scaffold's placeholder texts exactly —
   no guessing about hand-written content, so there are no false positives —
   and reports them as unfinished sections. The README check and the session
   instructions name the migration as the remediation: rewrite the existing
   README into the new marked form while preserving all real content. The
   same health check verifies the migration before the project can archive
   again.

7. **Guidance at the right moments.** The session-entry instructions surface
   README state (open placeholders) and the archive/review skills carry the
   matching steps, so an agent encounters the requirement where it acts — not
   only at the failing gate. A missing README impact decision needs no extra
   detector in the session entry: the review gate already refuses sign-off
   without it.

Projects without the readme addon observe none of this.

## Out of Scope

- **Prose quality.** givn enforces structure, markers, and recorded decisions.
  Whether a usage example is actually correct remains the reviewer's
  responsibility.
- **README as a change artifact.** The README stays a project-level document;
  no new artifact enters the workflow graph.
- **Documentation beyond the README.** arc42 chapters and their maintenance
  are unchanged.
- **Auto-generated prose.** givn fills only what it can derive (development
  commands); description, usage, and configuration text stay human/agent
  written.

## Open Questions

None. Decisions already taken with the product owner:

- Completeness enforcement is **blocking** (archive gate), mirroring the
  coverage gate — not warning-only.
- The per-change decision is a **machine-checkable sign-off line** in the
  review artifact, not prose guidance.
- Existing projects migrate via an **agent-executed rewrite** whose acceptance
  is the deterministic health check — givn never trusts the rewrite itself,
  only verifies the outcome.
