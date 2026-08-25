# 9. Architecture Decisions

Chapter 9 is maintained as a one-file-per-decision ADR register. The active
register, status rules, qualification gate, evidence, relationship map, ID
history, and archive index are in [`adr/README.md`](adr/README.md).

Each accepted, non-superseded decision is a separate MADR file under
[`adr/`](adr/). Superseded, deprecated, and non-architectural historical records
remain under [`adr/archive/`](adr/archive/) and are not current architecture.
`ADR-050` and `ADR-051` are reserved and unassigned. The register explains the
historical duplicate-ID migration and the later removal of records that do not
meet the strict qualification gate.

Individual ADRs preserve decision rationale and links. Current architecture
facts remain in the relevant Arc42 chapters or implementation; the register is
not a second copy of those facts.

ADR-023 is amended by the 0.3.0 release migration protocol. The amendment keeps
config scaffold upgrades preservation-oriented and adds a clean-worktree,
allowlisted managed commit plus a visible aggregate maintenance change when the
embedded migration catalog declares a route. It does not make Givn responsible
for semantic project decisions.

ADR-055 is also amended by the release's Cargo installation contract. The four
semantic capabilities remain independent, but all declared features are enabled
by the normal installation; `--no-default-features` remains the explicit
feature-free path.

ADR-057 is amended by the `semantic-model-download` change. The BGE local
artifact boundary remains validated and admission-controlled, but an explicit
semantic operation may acquire missing artifacts into the shared XDG user cache
before the unchanged local loader is invoked. The change does not create a new
parallel ADR for this refinement.

ADR-039 is amended by the same change. E5 model-cache ownership moves to the
shared XDG user cache while the permanent index remains project-local; this is a
retrieval-boundary refinement, not a new decision.
