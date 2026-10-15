# 02 — seed-artifact-lifecycle

Status: ready for `/givn-propose`
Source: change-seed schema (T1 B, seeds part) and seed lifecycle (M2 option A)
Depends on: 01 (dialog policy)

## Problem

A change seed is the handoff artifact between ideation and `/givn-propose`.
Today it has no content schema and no lifecycle state. Consumed seeds still
read "ready for propose"; seeds contradicted by later decisions keep their
original instructions with no pointer to the contradiction; the propose flow
never writes back. Tooling is blind: `givn new`, `givn status`, and
`givn instructions` contain no seed handling.

## Evidence

kiekste side:

- `givn/ideation/fossil-scm-integration/changes/create-project-with-scm-choice.md`
  ends with "First change of the topic; foundation for all others… Suggested
  start: immediately." No status field, no backlink. The replacement lives
  only in the other topic: `interchangeable-building-blocks/questions.md` Q14
  and the seed `create-project-with-port-bindings.md` ("Supersedes the fossil
  topic's seed…"). A grep for `supersed|refram|interchangeable|connection`
  across the fossil topic returns zero hits.
- Direct contradiction without a link: fossil
  `use-cases/host-project-forge.md` ("git projects … gain no forge") vs the
  interchangeable-building-blocks topic doc ("git repository with fossil
  tracker" as free composition).
- All six `run-authority` seeds were consumed by archived changes
  (`authorize-build-start`, `change-preview`, `publication-cas`,
  `stop-and-report`) or closed as already delivered; none names its consuming
  change.
- The interchangeable-building-blocks topic invented an ad-hoc fix
  (`## Status — Superseded on 2026-09-17…` headers and session addenda),
  which is evidence the need is real and the convention absent.
- Open questions in seeds are bare references: `act-from-a-port.md` lists
  "(Q16)" and "(Q16/Q5)".

givn side:

- `assets/skills/givn-ideation/SKILL.md.tmpl` lines 96, 129–146: seed
  contents described in one sentence; no schema, no status.
- `assets/commands/givn-propose.md` lines 129–132: reads a seed when
  available; no write-back.
- `assets/instructions/proposal.md` lines 22–25: records use-case ID, source
  topic, Personas; no seed path or stamping.
- No seed handling in `src/cli/new.rs`, `src/cli/status.rs`, `src/state.rs`
  (grep for `ideation`/`seed`).

## Decided scope

1. Fixed content schema for change seeds. (T1 B, seeds part)
2. Lifecycle status header with stamping at propose and archive time. (M2 A)
3. One advisory lint check for stale seed status. (M2 A)

## Required changes

### 1. Seed content schema

Add the schema to the ideation skill's Handoff section (fenced block,
consistent with the use-case schema precedent in the same file). A seed must
contain:

- Status header (see 2) as the first line after the title.
- Stable use-case ID and topic source.
- Confirmed Persona IDs.
- Accepted decisions, each as: decision / context / options considered, each
  with meaning, gain, cost, and one concrete example / chosen option /
  rationale. Never a bare "decision A".
- Open questions as full standalone text with the option treatment from the
  dialog policy (context, per-option meaning, gain, cost, concrete example).
  Never "(Q16)".
- Exact handoff status and suggested start order.

### 2. Status header convention

```text
Status: ready | proposed by <change-id> | implemented in <change-id> | superseded by <reference> | withdrawn
```

Rules:

- Ideation Handoff writes every seed with `Status: ready`.
- When a proposal is created from a seed, the seed becomes
  `proposed by <change-id>`, and the proposal's Use-Case Context records the
  seed path.
- When the change archives, the seed becomes `implemented in <change-id>` or
  `withdrawn` with a one-line reason. This stamp is required by the archive
  guidance; it is prose, not runtime code, in this change.
- A superseding decision updates the seed to `superseded by <reference>`
  (another seed, a change, or a permanent use case). The superseded seed keeps
  its original text; the status line is the only edit.

### 3. Propose-time stamping

Update `assets/commands/givn-propose.md` and
`assets/instructions/proposal.md`: when the change originates from an ideation
seed, the proposal step must (a) read the seed, (b) record its path in the
Use-Case Context, and (c) set the seed status to `proposed by <change-id>`.

### 4. Archive-time stamping

Update `assets/commands/givn-archive.md` (and the archive instruction surface
if one exists) to require setting `implemented in <change-id>` or `withdrawn`
on the originating seed. If no seed is recorded, the archive step records
"no seed" explicitly.

### 5. Advisory seed-status check

Add an advisory finding to `givn lint` for seed files under
`givn/ideation/*/changes/*.md`:

- missing status header;
- `proposed by <change-id>` where the change id does not exist;
- `proposed by <change-id>` where that change is already archived (stale;
  should be `implemented in` or `withdrawn`).

Behavior: rendered on stderr with a distinct level label; advisory only —
do not change the existing exit-code contract in this change. Change 08
introduces the severity classification that this finding adopts.

## Files to change

| Path | Change |
|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` | Seed schema; Handoff writes `Status: ready` |
| `assets/commands/givn-ideation.md` | Guardrail: seed status on handoff |
| `assets/commands/givn-propose.md` | Stamp seed `proposed by <change-id>` |
| `assets/instructions/proposal.md` | Record seed path; stamping rule |
| `assets/templates/proposal.md` | Use-Case Context field for the seed path |
| `assets/commands/givn-archive.md` | Stamp `implemented`/`withdrawn` |
| `src/cli/lint.rs` | Advisory `seed_status` finding |

## Executable-spec impact

- `givn/specs/fragments/ideation-skill.feature`: new scenario asserting the
  generated guidance contains the status values and the seed schema phrases
  (keep the existing "change seed" phrase).
- `givn/specs/verify-change/lint-command.feature`: scenarios for the three
  advisory cases and for a clean seed.
- Check `givn/specs/archive-change/` for the archive guidance contract; add
  a scenario only if an existing spec asserts archive-step prose.
- If the seed scan needs a documented scan root, update
  `docs/workflow.md` §3.

## Acceptance criteria

- [ ] Generated ideation guidance carries the seed schema and the status
      values.
- [ ] The propose flow stamps a seed and records its path.
- [ ] The archive guidance stamps `implemented`/`withdrawn`.
- [ ] `givn lint` reports the three stale/missing cases as advisories and
      leaves the exit-code contract unchanged.
- [ ] A seed in the shape of the fossil `create-project-with-scm-choice`
      case (consumed, contradicted, unstamped) is reported by the check.

## Out of scope

- M2 option B (`givn new --from-seed` scaffolding and atomic stamping).
- Automatic status transitions beyond the propose and archive stamps.
- Rewriting existing kiekste seeds or fossil topic artifacts.
- Any new runtime artifact or manifest entry for seeds.

## Risks and notes

- Seeds are human-readable markdown; the lint check must tolerate free-form
  prose around the header and must not parse beyond the status line.
- Keep the check advisory. A hard failure on ideation state would contradict
  the posture that ideation state is convention, not machine-enforced.
- `givn/ideation/` may not exist in a project; the scan must be a no-op then.
