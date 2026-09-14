---

status: accepted
date: 2026-09-14
decision-makers: buster (project owner)
consulted: none
informed: none

# Select the SCM backend from project markers through one seam

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

Evidence: the ideation topic `multi-scm-support` confirmed detection-only
selection (Q1) after weighing a declared backend in `givn/config.yaml`; the
`scm-backend-selection` fragment specifies the observable refusals but not the
component boundary, dependency direction, or the seam-versus-config choice, so
the lower-level artifacts do not fully own this decision; no active or archived
register record owns backend selection, so the route is a new record.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: every repository-state command (archive, managed upgrade, and
      the later Fossil backend) consumes the seam; removing it means touching
      each command, its specs, and its step definitions again.
- [x] **Shapes quality attributes** - It materially affects maintainability
      and testability because: one detection rule replaces duplicated Git
      checks, and every consumer shares the same named refusals.
- [x] **Structural** - It defines a boundary, contract, and dependency
      direction because: commands depend on `scm::select` and a `Backend`
      handle; no command derives the backend on its own.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: adding a new lifecycle command without
      asking the seam would reintroduce Git assumptions.

## Context and Problem Statement

givn's repository-state handling was Git-only and duplicated: the archive
boundary (`src/git.rs`) and the managed upgrade (`src/cli/migration.rs`) each
ran their own Git checks. The `multi-scm-support` topic commits givn to Git and
Fossil, so every command needs one answer to "which version-control system owns
this project root?" — and a project with no usable answer must be refused by
name before anything is mutated.

## Decision Drivers

- One detection rule for every repository-state command.
- No silent guessing when the project root is ambiguous.
- No configuration migration: existing projects keep working unchanged.
- Refusals name the missing prerequisite before mutation.
- Room for a second backend without rewriting the consumers.

## Considered Options

- Keep per-command Git checks (status quo).
- Declare the backend in `givn/config.yaml`.
- One seam that detects the backend from project markers, in a fixed order,
  and refuses by name otherwise.

## Decision Outcome

Chosen option: "One seam that detects the backend from project markers, in a
fixed order, and refuses by name otherwise", because the project's own markers
are the only source that cannot drift from reality, and one seam gives every
command the same refusals without a config migration.

### Consequences

- Good, because a Git project keeps its current behavior while the refusal
  wording becomes backend-neutral and shared.
- Good, because adding the Fossil backend later means adding a `Backend`
  variant and its operations, not editing every consumer's Git checks.
- Bad, because a project root that carries both a Git marker and a Fossil
  marker is refused with no escape hatch in this version; removing one marker
  is the only way to proceed (recorded as R-63).
- Bad, because detection depends on external executables: a missing `git`
  without a `.git` entry at the root is indistinguishable from "no version
  control" (recorded as R-64).

### Confirmation

The `scm-backend-selection` fragment scenarios assert Git selection, the
no-version-control refusal, the Fossil refusal, the ambiguity refusal, the
nested-root refusal, and the missing-executable refusal through the compiled
CLI; the existing release-migration scenarios keep the managed upgrade
behavior green after the refactor.

## Pros and Cons of the Options

### Keep per-command Git checks

- Good, because no new abstraction is introduced.
- Bad, because the same assumption exists twice and can drift.
- Bad, because a Fossil project sees a Git-specific error instead of a named
  backend refusal.

### Declare the backend in `givn/config.yaml`

- Good, because an ambiguous or mixed project could force its backend.
- Bad, because the declaration can contradict the checkout on disk.
- Bad, because every existing project would need a config migration for a
  value the filesystem already answers.

### One seam with marker detection

- Good, because one place answers the question and produces the refusals.
- Good, because no config key is required and existing projects are untouched.
- Bad, because mixed setups have no override until a later change adds one.
- Neutral, because detection order (Git first, then Fossil) is a fixed rule
  that the fragment documents.

## More Information

- Ideation topic: `givn/ideation/multi-scm-support/` (frame, Q1, Q4, Q6, Q14).
- Fragment narrative: `givn/specs/fragments/fragment.md` and
  `givn/specs/fragments/scm-backend-selection.feature`.
- Related records: ADR-023 (versioned config upgrade), ADR-070 (forward-only
  archive proof).
