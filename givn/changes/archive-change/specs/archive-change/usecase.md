# Use case: archive-change

## Level

!

## Actors

- Developer

## Goal

Publish a completed change into the permanent specification corpus and prove
the published state, without losing behavior or leaving a partial archive.

## Trigger

The developer archives a reviewed change.

## Preconditions

- The change has passed its required workflow gates.
- The active SCM backend is selected and its clean-state witness is available.
- The backend can publish a subset of paths as a revision.
- The permanent corpus is available for update.

## Main flow

1. Run the archive gates and confirm the project state is clean in the active
   backend; only resolved evidence paths may be dirty.
2. Merge the accepted behavior and use-case documents (delta merge with
   base-revision match, retirement and grouping rules).
3. Resolve the evidence scope: the archive directory, the README when the
   readme addon is on, and the configured coverage outputs.
4. Publish the merged state as the archive candidate revision of the active
   backend, containing exactly the merged corpus paths and the pending receipt
   (unbound, marking the archive unproven).
5. Run each verification scope once against the candidate and check the
   reported scenario counts.
6. Measure coverage once and publish it.
7. Check the revision binding (tested revision equals candidate revision),
   record the proven receipt, refresh the README managed blocks, and publish
   the evidence-only finalization revision.
8. Report the archive as proven.

## Extensions

- Uncommitted work outside the resolved evidence blocks the archive and names the offending files.
- A failed scope leaves an unproven candidate for forward repair.
- Finalization refuses a tested tree that changed outside the resolved evidence.
- A backend hook rejects the candidate revision: the archive stays unproven and repairable; the exact intermediate state is backend-specific.
- No SCM backend is selected: the archive is refused by name before any mutation.
- An already proven archive refuses another archive run.
- A legacy archive without a receipt refuses another archive run.
- A change present as both active and archived is refused as corrupt.

## Rules

- A use-case replacement must match its recorded base revision.
- Feature behavior is never silently dropped.
- Only a proven archive is complete; an unproven candidate is never reported as proven.
- A rerun resumes an unproven candidate and refuses a proven archive.
- The candidate revision contains exactly the merged corpus paths.
- The finalization revision contains only resolved evidence paths.
- The tested revision equals the published candidate revision.
- A file the backend would not publish still counts as dirty and blocks the archive: Git untracked files and Fossil `EXTRA` files both block.
- Givn publishes the candidate revision and the finalization revision on both backends; the developer's per-scenario revisions remain theirs.
- The hook-rejection promise is "unproven and repairable", not an identical internal state: Git leaves the merge staged, Fossil leaves the merged state uncommitted in the checkout.

## Examples

- Git candidate: a change with merged specs and a clean worktree publishes
  candidate `a1b2c3d4e5f6...`; both scopes pass; the finalization revision
  contains the archive directory and the README; the proven receipt records
  `tested_revision: a1b2c3d4e5f6...`.
- Fossil candidate: the same change in a Fossil checkout whose `EXTRA` files
  are clean publishes a candidate check-in; both scopes pass; the finalization
  check-in contains the archive directory; the proven receipt binds the
  candidate check-in.
- Failed scope: the candidate stays published; `givn status` reports the
  unproven candidate; the developer repairs with a new revision; the rerun
  proves the archive without repeating the merge.
- Hook rejection, Git: a commit hook exits non-zero; the merge stays staged;
  the archive is unproven.
- Hook rejection, Fossil: a commit hook exits non-zero; the merged state stays
  in the checkout uncommitted; the archive is unproven.
- No SCM: `givn archive --change multi-scm-support` in a plain directory exits
  non-zero with a named refusal and mutates nothing.

## Minimal guarantee

A failed archive is never reported as proven, and its candidate remains
repairable in place.

## Success guarantee

The permanent corpus contains the accepted change, and a verification receipt
binds the proven archive to the backend revision that passed every
verification scope.

## Personas

- developer--primary-user
- agent--technical-role
- givn-maintainer--owner

## Capabilities

- archive-proof
- grouped-merge-for-new-capabilities
- preserve-docstrings-on-archive
- extract-coverage-gate
- enforce-post-merge-e2e-gate

## Interactions

| Capability | Consumer action | E2E scenario |
|---|---|---|
| archive-proof | archive a completed change | Consumer archives a completed change and receives a proven archive |
| archive-proof | repair a blocked archive | Consumer repairs a blocked archive and the rerun proves it |
| archive-proof | inspect an unproven archive candidate | Consumer inspects an unproven archive candidate |
| grouped-merge-for-new-capabilities | archive a new capability | Consumer archives a new capability and the merged spec lands in its declared use case |
| grouped-merge-for-new-capabilities | archive without routing | Archive is blocked when a new capability has no declared routing decision |
| grouped-merge-for-new-capabilities | archive a multi-file change | Nothing is merged when one delta file of a change has no routing decision |
| preserve-docstrings-on-archive | archive a modified scenario | Consumer archives a modified scenario and the permanent spec keeps the docstring |
| extract-coverage-gate | archive with coverage | Consumer archives a change with coverage enabled and the coverage summary appears in README |
| enforce-post-merge-e2e-gate | archive an E2E delta | Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner |

## Includes

- usecase: verify-change
- fragment: support

## Extends

- none

## Out of scope

Reviewing implementation design or inventing new behavior during archive.

## Diagram

```mermaid
flowchart LR
  Actor((Developer)) --> Gate[[Run archive gates]]
  Gate --> Merge[[Publish specifications]]
  Merge --> Candidate[[Publish candidate]]
  Candidate --> Proof[[Run scopes and record receipt]]
  Proof --> Archive[[Finalize archive]]
```

<!-- givn:base-sha256:d1c37c620dab545da96087bff955f6b13d214806906c4a86c67e2a97803d1e51 -->
