# Change seed: author-change

- **Stable use-case ID:** `author-change` (existing, modified), including the
  `support` / `llm-attribution` fragment capability
- **Topic source:** `givn/ideation/multi-scm-support/`
  (`use-cases/author-change.md`, `use-cases/support--llm-attribution.md`,
  eventstorming A2-A4, B7)
- **Confirmed Persona IDs:** `developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`
- **Accepted decisions:**
  - Q2: `givn commit` is the SCM-neutral commit path; Git keeps the chained
    `commit-msg` hook as a fallback for raw commits; Fossil attribution exists
    only through `givn commit`; givn stamps its own archive/upgrade revisions
    directly; the trailer format is unchanged; Fossil is trailer-only (no
    committer override).
  - Q7: `givn init` without an SCM scaffolds and exits 0 with a named warning
    that archive and managed upgrade are unavailable.
- **Open questions:** none blocking; the unregistered
  `llm-attribution-installation` capability must be registered or folded during
  spec authoring.
- **Handoff status:** ready for `/givn-propose` after seed 1.
- **Scope note:** backend-aware initialization report, conditional attribution
  plumbing, guidance that names only commands valid for the active backend,
  and the `givn commit` wrapper. Modifies the init and guidance scenarios in
  `givn/specs/author-change/` and `givn/specs/fragments/llm-attribution.feature`.
