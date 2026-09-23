# Persona: External contributor (open-source consumer)

New: external-contributor--open-source-consumer — authority boundary: may clone, build, test, fork, and contribute, but owns no workflow state and never runs the Givn lifecycle

## Stable core

- Role: A person who encounters a project created with Givn as a published
  open-source repository and works in it without Givn. Two merged segments —
  the builder who only runs the project and its tests, and the contributor
  who sends changes — share the authority boundary of a bare clone.
- Authority boundary: May clone, build, run tests, fork, and contribute; owns
  no Givn workflow state, decides nothing inside the lifecycle, and never
  runs a Givn command. May write and commit their own rules and notes without
  Givn interference.
- Evaluation capability: Can judge whether a bare clone builds, tests, and
  runs with no Givn installed, and whether the repository carries
  tool-specific artifacts or dangling references; cannot judge the Givn
  lifecycle, provenance, or archive mechanics.
- Own vocabulary: "clone", "build", "test", "dependency", "contribution",
  "pull request"; does not know or need "change", "archive", "scenario tag",
  or "givn instructions".
- Success criterion: Clone the repository, run the declared build and test
  path, and get a green result with `givn` absent from PATH; nothing in the
  repository requires or references a Givn command to work.

## Running record

- 2026-09-23 — Need: the project builds, runs, and tests with no Givn
  installed. (Frame)
- 2026-09-23 — Need: no generated Givn rule files, markers, or history
  entries in the published repository. (Frame, user decision)
- 2026-09-23 — Need: hand-written rules and files are never ignored,
  untracked, or deleted by Givn. (Frame, gap G8)
- 2026-09-23 — Tolerated: the publisher may commit `AGENTS.md` including its
  Givn fence; the contribution path must not depend on it. (Q3)

## Topic stances

- 2026-09-23 — local-only-integrations: publishability is the success
  condition; a dangling reference to a local-only file is acceptable only if
  the clone still builds, tests, and runs.
