# Proposal: guard-retained-paths-across-archive

## Release note

A project with `ise` effective can no longer archive a change while a declared retained path points inside the moving change directory: the archive names the entry, the moved path, and the archived path to declare instead, and the served `ise` guidance states that retained paths must survive archiving.

## Use-Case Context

- Use-case IDs: `archive-change` (`require-durable-retained-paths`) and
  fragment `support` (`instruction-integrity`)
- Ideation topic: none — the change comes from a reproduced benchmark failure,
  not from an ideation topic.
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner` — all three are recorded on the two owning
  documents; the operator confirmed them for this change.

## Review Context

- Review kind: independent
- Reason: major change. It adds a new pre-mutation refusal to the shared
  archive path that can block every `ise`-effective project, and it changes the
  durable archive contract (a main-flow gate, a new capability, a new
  interaction row) plus the served `ise` obligations. The archive is a two-run
  state machine (fresh move and pending resume), so the refusal placement and
  its effect on a resumed candidate must be checked independently.

## Problem / Opportunity

A project declares which paths a rebuild keeps in `givn/specification.yaml`.
`givn archive` moves `givn/changes/<id>/` to `givn/archive/<id>/` and never
inspects that declaration. A project that declares a change artifact in a
retained role therefore has its declaration silently invalidated by an
unrelated command.

The failure was reproduced in a live benchmark run: an agent declared
`limits: givn/changes/catopia-certification-authority/design.md`, then ran
`givn archive`, which moved that artifact to
`givn/archive/catopia-certification-authority/`. At the recorded revision the
retained entry matched nothing, so `givn ise replace` refused with
`the declared limits entry '...' matches nothing at revision '...'` before
printing any count, and the rebuild arm could not start.

Two repository facts make the state invisible until it is too late:

- `assets/instructions/ise.md` says nothing about retained paths surviving
  archive, so the guidance neither warns nor explains the constraint.
- `givn archive` moves the change directory without reading
  `givn/specification.yaml`, so the invalidation is silent.

Impact: a project can archive itself into an unseedable state, discover it only
when a rebuild fails, and repair it with an unrelated follow-up revision. The
declaration is project-owned and the archive is the only command that can break
it, so the archive is where the refusal belongs.

## Proposed Solution

Observable behavior, from the project's side:

- **Archive-time refusal.** With `ise` effective and a declaration present,
  `givn archive` refuses before any mutation when a declared retained role
  entry matches a path that this archive will move and the entry resolves
  nowhere after the move: it matches no tracked path that stays and none of the
  moved paths' archived locations. The refusal names the role and the entry, the
  moved path, and the archived path to declare instead; the change stays under
  `givn/changes/`, no candidate is published, and nothing is moved.
- **Unaffected declarations.** An entry that matches nothing under the moving
  change directory is unaffected. An entry broad enough to still resolve after
  the move (for example `givn/**`) is also unaffected, as is a directory that
  still holds another active change. A durable document such as
  `docs/arc42/02-architecture-constraints.md` is unaffected.
- **No read without the addon.** A project with `ise` off archives exactly as
  before and never reads a declaration that happens to be present.
- **Guidance.** `givn instructions ise` states that a retained role names a
  path that survives `givn archive`, that a change's artifacts move from
  `givn/changes/<id>/` to `givn/archive/<id>/`, and that a declaration must
  name the archived or another durable path.
- **The rebuild refusal stays.** `givn ise replace` keeps refusing a declared
  retained entry that matches nothing, with the same message. The new guard only
  makes it impossible to create that state through archive.

## Capability Routing

Route reports one recommendation for the whole proposal:
`EXTEND givn-workflow-enforcement (use case 'verify-change', score 17.00) —
confidence: signal`. It matched the proposal's mentions of `givn check`,
`givn check review`, and `route`, not the behavior. The table records that
recommendation where it applies and `no signal` where route ranked no candidate
for the capability.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `require-durable-retained-paths` | `EXTEND givn-workflow-enforcement` (use case `verify-change`, score 17.00) | `NEW in archive-change` | Route matched the words `givn check` and `route`. The observable behavior is not a `verify-change` check; it is a pre-mutation refusal at the archive move boundary. Archive owns whether the project's declaration still resolves after the move, and the refusal protects the archive's success guarantee. The step-definition gate took the same placement (`NEW in archive-change`) for the same reason. |
| `instruction-integrity` | no signal | `EXTEND instruction-integrity` | `instruction-integrity` is declared in `givn/specs/fragments/fragment.md`; the `support` fragment owns the `ise` obligations' content. The serving mechanics of `obtain-effective-guidance` are unchanged, so no capability there changes. |

## Out of Scope

- Rewriting, moving, or deleting an entry in `givn/specification.yaml`.
  The declaration is project-owned text.
- Changing the archive layout, the move transition, the merge, or the receipt.
- Any change to `givn ise replace` or `givn ise verify`: their behavior,
  refusals, messages, and exit codes stay as they are.
- Refusing on a `source:` root that points into the moving change tree
  (proposal D1): the reproduced defect and this change's boundary concern
  retained roles.
- Refusing a pending archive candidate whose declaration still names the
  already-moved change path (proposal D3): a guarded archive cannot create that
  state, and `givn ise replace` already names it.
- Reading `givn/specification.yaml` while `ise` is off.
- Changing the declaration schema, the one-time scaffold, or the four
  project-health findings.

## Open Questions

### D1 — Does the guard cover the declared `source:` roots as well as the retained roles?

- Question: `givn/specification.yaml` holds five retained roles and one
  `source:` list of replaceable roots. Should the archive-time guard refuse a
  `source:` root that points inside the moving change directory, or only a
  retained role entry? The reported defect concerns a retained entry, but a
  moved `source:` root is equally invalid after the move.
- Options:
  1. **Retained roles only** — the guard reads the five roles and ignores
     `source:`; gain: the refusal matches the reproduced defect and the
     operator's stated boundary, and `source:` semantics (replaceable roots,
     not retained paths) stay with their own reader; cost: a project whose
     `source:` root names `givn/changes/<id>/` can still archive that change and
     then see `--path` fall outside every root; in practice: `limits:
     givn/changes/<id>/design.md` is refused, while `source:
     givn/changes/<id>/` archives silently.
  2. **Every declared entry** — the guard checks all six keys; gain: one rule
     for the whole declaration; cost: the change grows into `source:` root
     semantics and its repair text, and a `source:` root inside a change tree
     may be an intentional throwaway during a rebuild; in practice: the same
     archive refuses twice for a declaration that names the change in both keys.
  3. **Retained roles now, `source:` as a separate proposal question** — gain:
     the boundary stays explicit; cost: a second change is required; in
     practice: the same reproduced defect is fixed now, and the `source:` case
     becomes a named future question.
- Disposition: out of scope — the operator's change boundary names retained
  role entries, and `source:` roots have no retained-role guarantee to lose.
  Option 3 records the deferred question without widening this change.

### D2 — What does the archive do when the declaration cannot be loaded?

- Question: With `ise` effective, `givn/specification.yaml` may be absent,
  malformed, or unreadable at archive time. The guard cannot compare entries it
  cannot read. Should the archive refuse, skip the guard, or treat the file as
  empty?
- Options:
  1. **Refuse, naming the declaration** — the archive reuses the loader's
     message and refuses before mutation; gain: the refusal is never silent and
     the same loader owns the message; cost: a project with a malformed
     declaration cannot archive that change until it repairs the declaration;
     in practice: `givn archive` prints
     `archive blocked: the declared retained paths cannot be read: givn/specification.yaml: ...`
     and moves nothing.
  2. **Skip the guard** — archive proceeds and the declaration stays broken;
     gain: archive never blocks on the declaration; cost: the unseedable state
     remains reachable, which defeats the change; in practice: a declaration
     with an unknown key archives and the next trial refuses.
  3. **Treat an absent or unreadable file as no declaration** — gain: no
     refusal; cost: silently conflates "no declaration" with "broken
     declaration", and the loader's strictness is bypassed at the one command
     that can invalidate it; in practice: `givn archive` proceeds while the
     declaration is a directory.
- Disposition: answered — option 1, recorded in `design.md` §Decisions D2.
  The archive receipt gate runs first, so in the ordinary flow a broken
  declaration is already a stale-receipt refusal; the guard's branch is
  defensive and reuses the one loader.

### D3 — Does the guard run on the pending-candidate resume path?

- Question: The archive has a fresh path (move the active change) and a resume
  path (an unproven candidate already sits under `givn/archive/<id>/`). Should
  the retained-path guard run again on the resume path, and against which
  directory?
- Options:
  1. **Fresh move only, against the active change directory** — the guard runs
     once before the receipt write and the rename; gain: it guards the only
     transition that can create the state, and a resume is never blocked by a
     declaration that already names the broken location; cost: a resume created
     by a pre-guard binary is not corrected by the guard; in practice: an
     interrupted archive resumes normally, and `givn ise replace` names the
     stale entry.
  2. **Both paths, against the active change directory** — gain: a rerun of the
     guard on resume; cost: on resume the active directory is absent, so the
     check is vacuous or refuses for a missing directory; in practice: resume
     either does nothing or fails for an unrelated reason.
  3. **Both paths, refusing when the declaration still names the archived
     change** — gain: a pre-guard broken state is caught; cost: the state is
     already unseedable and can only be repaired by editing the declaration
     first, so a resume adds a second refusal for a condition the existing
     `ise replace` refusal names more precisely; in practice: an interrupted
     archive refuses until the declaration is edited and committed.
- Disposition: answered — option 1, recorded in `design.md` §Decisions D3. A
  guarded archive cannot create the resume state, so the fresh path is the
  complete guard; option 3 is deferred rather than designed for.
