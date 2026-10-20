# Proposal: archive-transaction-safety

## Release note

> Required. One complete, user-facing sentence naming what this change does.

Make archive state transitions atomic and recoverable so a failed archive can be retried without manual cleanup or destructive repository resets.

## Use-Case Context

> Optional. When this change comes from ideation or an existing permanent
> contract, record the stable use-case ID, source topic, and confirmed Persona
> IDs. Do not copy Persona biographies into the proposal.

- Use-case ID: archive-change
- Ideation topic: none — derived from a real evaluation run
- Seed path: none
- Confirmed Personas: none — operator confirmed; archive is an operator workflow, not a rendered visual surface; the operator is a Gherkin actor, not a Persona.

## Problem / Opportunity

A failed archive can leave both `givn/changes/<id>` and `givn/archive/<id>` present. The next archive refuses to continue because the project is classified as corrupt. The operator must delete one side manually, and recovery reset operations can recreate the same state. In a real trial, this produced ten archive failures before success and forced destructive resets.

The operator also cannot tell whether a pending archive candidate is safely resumable or genuinely corrupt. The intended forward-only recovery path is not reliably available.

## Proposed Solution

Archive must maintain one unambiguous state at all times:

- active change only;
- pending archive candidate only;
- proven archive only.

A failed move or failed gate must leave a state the next archive can resume, repair or report precisely. Both directories must not be reported as generic corruption when the archive receipt is pending and the candidate is intact.

When the candidate is unproven, archive reports which state it is in and what the operator can do next. It must not require deleting the change or the archive as the normal recovery path.

Failed verification must roll forward: the candidate stays pending, the operator repairs the committed state, and rerunning archive verifies the repaired candidate.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| archive-proof | no signal — the top candidates tie at 13.00 | `EXTEND archive-proof` | The existing capability owns the archive receipt and the pending/proven state; this change adds the atomic transition, recoverable duplicate, evidence paths, and roll-forward repair to it. |

## Out of Scope

- Changing the verification scope contract or the meaning of a successful test run.
- Changing the order in which permanent specs are merged.
- Changing coverage gates.
- Adding a rollback command. The design remains forward-only.
- Repairing pre-existing uncommitted work automatically.

## Open Questions

> One entry per question, with a stable ID, the full question with its option
> treatment (context, per-option meaning, gain, cost, and a concrete example),
> and a Disposition: answered (with its source), carried (with an owner and
> the artifact that will decide it), or out of scope (with a reason).

### D1 — Recover both directories or reject them?

- Question: When `givn/changes/<id>` and `givn/archive/<id>` both exist and the archive has a pending receipt, should a rerun reconcile them, or should it continue to report corruption?
- Options:
  1. **Reconcile a pending candidate** — compare the archive candidate with the active change; resume when they agree, report corruption when they conflict. Gain: no manual deletion for an interrupted move. Cost: more state validation. In practice: a failed copy leaves both trees; archive compares them and resumes.
  2. **Keep rejecting both** — require an operator to remove one side. Gain: simpler. Cost: manual cleanup remains the normal failure path. In practice: the operator deletes the partial archive and loses the pending candidate.
- Disposition: answered — forward-only recovery requires option 1. The design decides the comparison and the conflict criteria.

### D2 — Atomic rename or transaction marker?

- Question: Should archive transition state with a filesystem rename or with a persistent transaction marker?
- Options:
  1. **Atomic rename** — move the change directory into the archive location after preconditions pass. Gain: both directories cannot coexist. Cost: requires same-filesystem rename and careful ordering of the pending receipt. In practice: `rename` replaces copy-then-delete.
  2. **Transaction marker** — keep both paths temporarily and record a transaction. Gain: works across filesystems. Cost: more state and recovery logic. In practice: a marker file records copy, pending and commit phases.
- Disposition: carried to `design.md` — choose based on the actual project-root filesystem and portable SCM behavior.

### D3 — Should a failed verification roll back?

- Question: When the merged-suite verification fails, should archive restore the active change, or keep the candidate pending?
- Options:
  1. **Keep pending and roll forward** — the candidate remains, the operator repairs the committed state, and rerunning archive verifies it again. Gain: no lost work and no rollback race. Cost: the repair happens in committed history. In practice: a regression in an old spec is fixed in code or permanent specs, then archive is rerun.
  2. **Roll back to active change** — restore the change directory and undo the candidate commit. Gain: the change is editable again. Cost: destructive history rewrite and a second archive attempt. In practice: `git reset` becomes part of the tool.
- Disposition: answered — roll forward, as already intended by the archive workflow. The design makes the pending state reliable.
