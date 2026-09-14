# Persona: Primary user (developer / maintainer)

## Role

The human who uses givn in their own projects. Develops features, maintains
the project, runs managed upgrades, and crosses release gaps. Works in Git
projects, Fossil projects, or both. This persona merges the former developer
and project-maintainer segments by user decision; the distinct interests stay
listed and are not averaged away.

## Interests

- I1: Git projects keep behaving exactly as today. No regression, no new
  mandatory tooling, no vocabulary churn in existing guidance.
- I2: Fossil projects get the full lifecycle, not a reduced one: archive with
  a proven receipt, managed upgrade, forward repair.
- I3: One mental model across both SCMs: the same commands, the same failure
  messages, the same repair path. No per-SCM flags to remember.
- I4: Managed upgrade and migration commits stay reviewable and atomic in
  whichever SCM is active.
- I5: Project documentation, tickets, and wiki stay available offline and in
  the same artifact as the code (the reason for choosing Fossil). Context
  only: the hosting and the work-item features are kiekste's domain, so this
  interest never becomes a givn specification obligation.

I1 and I2 pull against each other wherever Fossil's mechanics differ; the
persona expects the difference to be handled inside givn, not pushed onto
them.

## Goals

- Archive a completed change and receive a proven archive.
- Upgrade givn without hand-fixing the scaffold.
- Cross a release gap with reviewable migration evidence.
- Read project documentation and work items without a running server.

## Pain points

- Being told "this project is not a Git repository" when it is a valid Fossil
  checkout.
- Git-specific vocabulary in specs, skills, and error messages.
- SCM flags or per-SCM command variants in the normal workflow.
- Partial automatic migration or force-staged files they did not approve.
- Capabilities that silently do nothing on one SCM.

## Success criterion

A Git project's behavior is unchanged, and a Fossil project completes the same
lifecycle with the same guarantees, using the same commands.

## Vocabulary

- "my project", "check in" / "commit" (used interchangeably), "upgrade",
  "migration", "clean", "history", "timeline".
- Fossil user: "wiki", "tickets", "checkout".
- Git user: "branch", "commit", "worktree".
- The collision "commit" (Git) vs. "check-in" (Fossil) is their normal speech;
  the domain must not force either word on them.

## Decision, need, and demand log

| Date | Entry | Source |
|---|---|---|
| 2026-09-14 | Decision: givn natively supports Git and Fossil; a Git project uses Git, otherwise Fossil. | User |
| 2026-09-14 | Need: a named refusal when neither SCM is present. | Frame |
| 2026-09-14 | Demand: attribution should work in Fossil projects if feasible. | User |
| 2026-09-14 | Need: no Git-only claim in generated guidance for a Fossil project. | Frame |
