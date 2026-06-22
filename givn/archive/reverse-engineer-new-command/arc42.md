# arc42-docs marker — reverse-engineer-new-command

This change is a reverse-engineering characterization of the `givn new`
command. It does not introduce new building blocks; it documents observed
AS-IS behaviour and flags suspected bugs as technical debt.

## Chapters updated

- **05 — Building Block View:** Refined the `cli::new` module responsibility
  row to describe the id-validation rules and the single-file (`proposal.md`
  only) output, derived from observed behaviour.
- **06 — Runtime View:** Rewrote section 6.2 (`givn new <change-id>`) with
  the actual step sequence observed by probing the binary — including the
  check ordering (already-exists before givn/-not-found), the verbatim
  template write without `{{CHANGE_ID}}` substitution, and the clap
  interception of leading-hyphen ids (exit 2) before validation.
- **11 — Risks and Technical Debt:** Added entries D-5 through D-8 recording
  the four suspected bugs/code smells surfaced by discovery
  (`{{CHANGE_ID}}` placeholder not substituted; check ordering; split
  start-hyphen behaviour; `givn init` not scaffolding `givn/artifacts/`).
  Each is marked as AS-IS-captured with a future `@givn.modified` repayment
  candidate.

No new chapters were created. The README status line was updated to note
the characterization in progress.

## Status

STATUS: DONE
