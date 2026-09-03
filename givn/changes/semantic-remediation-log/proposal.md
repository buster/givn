# Proposal: semantic-remediation-log

## Problem / Opportunity

When the semantic review admits a candidate, the agent classifies it, applies the
projected remediation, and reruns the review until the gate verifies the fix.
Afterwards only two traces survive: the final semantic review report (regenerated,
overwriting every earlier state) and commit hashes in the change's task list.

The remediation story itself is lost. Nobody can later answer:

- What did the embedding recall and the evidence models (BGE, NLI) recommend for
  this candidate, and how long did each review stage run to find it?
- What did the reviewed scenario look like before the fix, and what does it look
  like after?
- Which human decision and which rationale led to which concrete fix?
- Which commits carried the fix — findable from Git history, not only from the
  change's task list?

Without this record the archive cannot be used as evaluation material: there is no
labelled set of model recommendation, human decision, fix, and outcome to judge
how well the semantic review pipeline works.

## Proposed Solution

Every change gains a second review artifact next to the semantic review report:
the **remediation log**. It is written and maintained by the review tooling, is
append-only, and records one entry per admitted candidate:

- **Before**: the review run that admitted the candidate — the embedding score,
  the BGE and NLI evidence states, the scenario text of both endpoints verbatim
  as it appeared in the specification files, and the durations of the review
  stages (embedding recall, BGE, NLI).
- **Decision**: the classification and the rationale recorded during review.
- **Fix**: a short description of the remediation performed and the commits that
  carried it.
- **After**: the verified end state after the passing rerun — the recomputed
  score, the remaining scenario text verbatim (or the absence of a removed
  endpoint), and the durations of the verifying review run.

Every remediation commit is additionally marked in Git history with a
`semantic-remediation: <change>/<candidate>` footer line, so the commits are
searchable with plain Git tools (`git log --grep`), independent of any givn
artifact. The review gate refuses to pass while a resolved candidate lacks a
complete log entry whose commits are reachable and carry the matching stamp.

Archiving retains the remediation log next to the semantic review report. Across
all archived changes the logs form the labelled evaluation material: what the
models recommended, what the human decided, what was fixed, in which commits,
and what it cost in review-run time.

## Out of Scope

- The semantic review report itself keeps its existing grammar; the log does not
  replace or extend the report.
- No automatic remediation: givn still never selects classifications or edits
  specification files; the agent writes decision and fix text, the tool records
  and validates.
- No new commit-message hook: the stamp is validated by the review gate, not
  enforced at commit time.
- No aggregation command over archived logs yet — the log format is the
  foundation; evaluation tooling follows later.
- Projections onto permanent specifications and the existing remediation
  verification stay exactly as they are.

## Open Questions

None — the artifact grammar, gate rules, and Git stamp are decided; details go
to specs and design.
