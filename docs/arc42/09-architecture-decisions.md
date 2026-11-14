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

## Current Amendments

An amendment changes an existing record's scope while preserving its original
rationale and identity. It does not create a parallel decision.

| Record | Current amendment |
|---|---|
| [ADR-010](adr/ADR-010-manifest-overlay.md) | Configuration authority is split: `givn/config.yaml` owns policy and graph, `givn/commands.yaml` owns executable `verify`/`coverage`/`run` settings through a dedicated schema. A policy key in the commands file and an executable key in the policy file are refused. |
| [ADR-013](adr/ADR-013-agent-integration.md) | The generated invocation surface is skills-only. `.opencode`, `.agents`, `.claude`, `.omp`, and `.maki` receive `<target>/skills/<name>/SKILL.md`; no target receives generated command files. Maki remains a full skills target and receives the development reporter and commit-attribution integration through merged user-owned configuration. |
| [ADR-023](adr/ADR-023-versioned-config-upgrade.md) | The managed upgrade runs on Git and Fossil through the SCM seam, captures a checkpoint, stages only the calculated allowlist, and reports actual publication and partial state. The supported migration baseline is 1.0 with an empty shipped catalog; retired 0.x keys require manual repair, not an alias or an embedded route. |
| [ADR-028](adr/ADR-028-hooks-and-mandatory-checks.md) | Mandatory review checks remain non-deactivatable: integrity, the run declaration, change-scoped lint, overlap/retirement/routing, the verification receipt, and scenario commits. The later conditional readability/outline review member is retired; the outline addon is advisory only and no `OUTLINE`/`READABILITY` verdict or waiver is required. |
| [ADR-070](adr/ADR-070-forward-only-archive-proof.md) | A change verification receipt binds a stable execution witness (revision, selected features, command, tested content, scope). A no-behavior-change declaration reports applicability instead of a fabricated green result. Successful archive scopes are persisted per scope and reused only when their inputs and results still match; changed inputs conservatively invalidate both scopes. |
| [ADR-072](adr/ADR-072-witness-bounded-publication.md) | The witness-bounded publication contract is unchanged. `givn commit` additionally uses explicit selected paths on both backends; witness-bounded archive publication remains the archive's own boundary. |

## Historical Amendments

These amendments are retained in the individual ADR files and the register:

- ADR-028 by `run-command` (run declaration), `visual-ui-review` (visual
  verdict/evidence), `web-ui-verification-hardening` (persona/state/deferral),
  and `persona-identity-reuse` (persona-ID resolution);
- ADR-013 by `require-agent-target` and `keep-agent-prompt-open` (explicit
  target selection), and `add-omp-agent-target`, `development-timeline` (target
  additions and the Maki reporter);
- ADR-070 by `review-friction-hardening` (tested-content digest and
  documentation-only reuse);
- ADR-071 by `maintain-project` (upgrade on both backends).

The earlier conditional outline review member and its `add-readability-*`
amendments are retained as historical rationale only. They do not describe the
current gate.

Individual ADRs preserve decision rationale and links. Current architecture
facts remain in the relevant Arc42 chapters or implementation; the register is
not a second copy of those facts.

The active register gained one decision in the `addon-artifact-report`
change: [ADR-076](adr/ADR-076-addon-materialization-declaration.md) places an
addon's filesystem effect on its registry entry, read by both the scaffold
routines and the `givn addons list` materialization report.

*[Architecture documentation index](README.md)*
