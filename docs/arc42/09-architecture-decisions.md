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

ADR-028 is amended again by the `visual-ui-review` change. The integrity
mandatory check gains the visual review gate: when a change's `design.md`
declares a visual interface, `givn check review` requires the visual verdict
(`VISUAL-REVIEW: PASS`, with `VISUAL-REVIEW: UNVERIFIED` blocking until a
human signs off) and complete per-viewport evidence. The check lives inside
the existing integrity member, so it cannot be disabled by a `post` hook
entry. This is a further extension of the same boundary, not a new record.

ADR-028 is amended a third time by the `web-ui-verification-hardening`
change. The same integrity member gains the persona disposition, the
per-state evidence, and the deferral pair: a declared visual interface
requires a `PERSONAS:` line naming at least one confirmed Persona, every
declared UI state requires committed `<viewport>--<state>.png` evidence, and
a `DEFERRAL:` line requires a matching `OPERATOR-DECISION:` record. The
classification reasoning and the quality of a persona disposition stay
guidance-level. This remains an extension of the same boundary, not a new
record.

ADR-071 is accepted by the `scm-backend` change: repository-state commands ask
one selection seam which version-control system owns the project root. Detection
uses the project's own markers (Git worktree first, then a Fossil checkout) and
refuses ambiguity, absence, a nested root, or a missing executable by name; no
configuration key declares the backend. The seam is the prerequisite for the
Fossil backend and for moving the archive publication onto the same contract.
The mixed-setup and missing-executable consequences are carried by R-63 and
R-64 in chapter 11.

ADR-072 is accepted by the `archive-change` change: publication through the SCM
seam is witness-bounded. Givn resolves the publication paths, validates the
exact dirty set against them, and then records only that set — Git stages the
candidate paths, Fossil records all changes and commits the witnessed dirty set
because it has no index and cannot name a directory that contains missing
files. The archive consumes the Fossil backend, and the resume path republishes
a candidate whose publication a backend hook rejected. The witness-bounded
consequence is carried by R-65 in chapter 11.

Individual ADRs preserve decision rationale and links. Current architecture
facts remain in the relevant Arc42 chapters or implementation; the register is
not a second copy of those facts.

ADR-023 is amended by the 0.3.0 release migration protocol. The amendment keeps
config scaffold upgrades preservation-oriented and adds a clean-worktree,
allowlisted managed commit plus a visible aggregate maintenance change when the
embedded migration catalog declares a route. It does not make Givn responsible
for semantic project decisions.

ADR-069 supersedes the fifteen semantic-engine records: the semantic retrieval
engine (E5 retrieval, BGE/NLI evidence, model-cache acquisition, and the
semantic gates) was removed by the `drop-embedding-features` change, leaving a
single feature-free binary. The register in [`adr/README.md`](adr/README.md)
is the authoritative list of active decisions, statuses, and relationships.

ADR-067 is accepted by the `run-command` change: the same tool that declares
the product start (`run:` in `givn/commands.yaml`) also executes it
(`givn run`) under one spec'd machine contract — ready line, exit-code table,
port hand-off, and signal forwarding. The record was proposed with the
change's design and left `proposed` until the full suite and the @e2e scope
stayed green; the compatibility obligation is carried by R-45 in chapter 11.

ADR-070 is accepted by the `archive-proof` change: publication becomes
forward-only and Git-backed — the merged archive candidate is committed before
verification, each verification scope runs once with a fail-closed runner
result, and the verification receipt is committed as the evidence-only
finalization. Unlike ADR-067, this record is accepted before the archive runs:
the permanent ADR register scenario asserts that no register record is left
`proposed`, and that scenario is itself verified by the archive, so a
`proposed` record could never satisfy the merged corpus. The recovery and
review-evidence consequences are carried by R-53 through R-55 in chapter 11.
