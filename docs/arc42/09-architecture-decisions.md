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

ADR-028 is amended by the `run-command` change. The mandatory-binary-check
boundary it established gains one more non-deactivatable member: the
run-declaration gate. It runs in `givn check review` and in `givn archive`
before the README completeness gate, independently of the readme addon, so a
project without a README addon is still held to declaring how its product
starts. This is an extension of ADR-028's existing boundary, not a new record.

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

ADR-063 is proposed by the open `semantic-reranker-budget` change (bounded
second-stage work). ADR-064 is proposed by the open
`semantic-scoring-throughput` change: one shared `GIVN_SPEC_THREADS` budget
controls the compute threads of E5, BGE, and NLI. ADR-066 is proposed by the
open `unified-semantic-rulebook` change: one change-scoped comparison sequence
(single `0.9000` admission, hybrid selection, pair-score cache) behind all
three review surfaces. All three are listed as `proposed`
in the register until their changes pass their remaining gates.

ADR-067 is accepted by the `run-command` change: the same tool that declares
the product start (`run:` in `givn/commands.yaml`) also executes it
(`givn run`) under one spec'd machine contract — ready line, exit-code table,
port hand-off, and signal forwarding. The record was proposed with the
change's design and left `proposed` until the full suite and the @e2e scope
stayed green; the compatibility obligation is carried by R-45 in chapter 11.

ADR-068 is proposed by the open `semantic-remediation-log` change: remediation
evidence for semantic candidates lives in an append-only
`semantic-remediation.md` beside the regenerated report, with per-run stage
durations, verbatim before/after scenario text, and a
`semantic-remediation: <change>/<candidate>` Git stamp that the review gate
cross-validates. The record stays `proposed` until the change passes its
remaining gates.
