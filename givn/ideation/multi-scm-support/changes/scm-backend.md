# Change seed: scm-backend

- **Stable use-case ID:** fragment `scm-backend` (new)
- **Topic source:** `givn/ideation/multi-scm-support/` (frame, eventstorming A1
  and the refusal events, `use-cases/scm-backend.md`)
- **Confirmed Persona IDs:** `developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`
- **Accepted decisions:**
  - Q1: detection only; no backend declaration in `givn/config.yaml` in v1.
  - Q4: the project-boundary rule is the Git worktree top level or the Fossil
    checkout root; the repository file location is irrelevant.
  - Q6: detection probes the project root; both markers there is an ambiguity
    refusal; ancestor Fossil markers are ignored; Git may resolve to an
    enclosing worktree (nested case, TC-32).
  - Q14: a selected backend whose executable is missing is refused by name at
    selection time, before any mutation.
  - Q9: kiekste hosting stays out of scope.
  - Frame detection rule: Git worktree, then Fossil checkout, then a named
    refusal; `init` and `status` continue without a backend, archive and
    managed upgrade refuse.
- **Open questions:** Q10 (vocabulary — resolve in this change's design/spec),
  Q12 (test strategy — resolve in design).
- **Handoff status:** ready for `/givn-propose`.
- **Scope note:** introduces the SCM port and the Git backend, and moves the
  duplicated Git logic in `src/cli/migration.rs` onto the same seam without
  behavior change. Adds `givn/specs/fragments/scm-backend.feature` and its
  fragment document. This seed is the prerequisite for seeds 2-4.
