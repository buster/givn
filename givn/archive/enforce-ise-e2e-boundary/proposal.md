# Proposal: enforce-ise-e2e-boundary

## Release note

With `ise` effective, a change check refuses a declared E2E package that is
missing, links production source, or is not named by the E2E runner command,
and binds that package, the runner command, and the running Givn version into
the change verification receipt; the canonical `@e2e` policy now states the
same package boundary.

## Use-Case Context

- Use-case IDs: `verify-change` (`givn-workflow-enforcement`) and
  `create-change` (`prose-consolidation`)
- Ideation topic: `ise-rebuildability`
- Seed path: `givn/ideation/ise-rebuildability/changes/enforce-ise-e2e-boundary.md`
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`,
  `givn-maintainer--owner`

## Review Context

- Review kind: independent
- Reason: major change. It turns the declared E2E package into a blocking
  acceptance boundary in the change check, adds a second runner execution for
  `ise`-effective changes, and extends the durable change verification receipt
  with a compatibility-bound identity that review and archive refuse to reuse
  when it is missing or changed. It also changes the canonical `@e2e` policy
  that every project reads.

## Problem / Opportunity

The `ise` declaration already names an `e2e` role and `givn lint` already
reports placement problems as advisory findings. Nothing prevents a change from
passing its check while that role names no package, names a package that links
production source, or leaves `verify.e2e_command` naming no declared root. The
verification receipt also records only the regular change run, so a later
reader cannot tell which black-box suite the accepted result was bound to.

The canonical `@e2e` policy still permits a library consumer's real public API
calls to run in process anywhere, and it permits a shared runner without naming
which package the E2E execution runs. Both sentences become false once the
package boundary is enforced. The archived predecessor deferred their
amendment to this change so that prose and enforcement land together.

Impact: without this change an `ise`-effective project can freeze an acceptance
authority that is missing, dependent on the code it is supposed to outlive, or
unnamed by its E2E runner command; the receipt cannot tell a reader which suite
the run executed; and the served policy contradicts the enforced boundary.

## Proposed Solution

Observable behaviour, from the developer's side:

- **Blocking package boundary.** With `ise` effective for the change's locked
  add-on set, `givn check tasks` requires the declared `e2e` role to resolve at
  least one package. The command refuses, names the condition, and writes no
  passed receipt when the role matches nothing, when a resolved package's package
  manifest cannot be read, when a resolved `e2e` path sits under a declared
  `source:` root or a resolved package's tree contains one, when a package reaches a
  `source` root through a direct, transitive, or workspace-inherited dependency
  edge, when a resolved path that resolves no package imports a package under a
  `source` root, or when `verify.e2e_command` does not name every resolved
  `e2e` root.
- **E2E execution is part of the change check.** When the change's delta
  contains at least one `@e2e` scenario, the same check runs the E2E command
  against the selected delta features before issuing a receipt. A failure, an
  unbound step, a skipped step, a missing or malformed result, a zero count, or
  a wrong scope fails the check and leaves no passed receipt. A delta without
  an `@e2e` scenario records the E2E scope as not applicable.
- **E2E identity in the receipt.** The change verification receipt binds the
  declared `e2e` role's package digests, the E2E runner command digest, the
  running Givn version, and the E2E scope outcome. Review and archive reuse the
  receipt only while those values still match. A receipt written before this
  change, one whose package or command changed, one written by another Givn
  version, or one with no green E2E outcome where the delta has `@e2e`
  scenarios is stale; the refusal names each stale reason.
- **One owner per condition.** The project-health check and the change check
  agree on one condition set. `givn lint` reports the project-level placement
  conditions as advisory findings with exit 0 and never blocks. `givn check
  tasks` refuses the change-level acceptance conditions. The runner-target and
  runtime-step conditions belong to the change check alone; the
  malformed-declaration contract stays a declaration error. A condition has one
  wording wherever it is reported.
- **One canonical policy.** The `@e2e` policy states that the declared package
  holds the E2E runner and every `@e2e` step definition, links no production
  source, is the target of `verify.e2e_command`, fails on an unbound or skipped
  step, and shares feature files. The in-process allowance applies only inside
  that package; a call that needs production source becomes a component
  scenario. The two pinned `prose-consolidation` scenarios and the two
  `create-change` rules change with the same wording.
- **`ise` off is unchanged.** A change whose locked add-on set has `ise` off
  keeps today's receipt shape, today's freshness rules, and today's single
  change-scoped runner.

## Capability Routing

Route reports one recommendation for the proposal's main change:
`EXTEND givn-workflow-enforcement (use case 'verify-change', score 17.00) —
confidence: signal`. It ranked no candidate for the policy wording. The table
records the recommendation where it applies and records `no signal` where it
does not; the decision column records the capability placement the use-case
contracts and the seed assign.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `givn-workflow-enforcement` | `EXTEND givn-workflow-enforcement` (17.00) | `EXTEND givn-workflow-enforcement` | Route agrees. The blocking boundary, the E2E execution, the receipt identity, and the freshness comparison belong to the change-scoped verification the capability owns. |
| `prose-consolidation` | `no signal` | `EXTEND prose-consolidation` | The canonical `@e2e` policy is owned by `create-change`; the seed assigns the two amendments, the two pinned scenarios, and the two rules to it. Route ranked no candidate because the proposal names the policy text, not a command. |

## Out of Scope

- The two `givn ise` commands, clean-room preparation, and trial-input
  verification (`add-ise-clean-room-commands`).
- The stack instruction split, `steps-rust`, and the instruction line budget
  (`split-stack-instructions`).
- Enabling `ise` on this repository, moving Givn's `@e2e` steps out of the
  production-linked target, and the self-rebuild
  (`move-givn-e2e-to-black-box-package`, `enable-ise-on-givn-and-run-a-dry-run`).
- The four advisory declaration findings themselves. Their project-level
  advisory behaviour, severity, messages, and exit code stay as they are.
- Per-step phrase uniqueness, a static import scan for a resolved package
  (references are read only for a declared path that resolves no package),
  unit-test inference, a `components:` key, and a per-component file list.
- A dependency reader for stacks other than Cargo, and a project-declared
  dependency command.
- Changing guidance for a project whose change has `ise` off.

## Open Questions

### D1 — Which proof scope carries the E2E identity

- Question: The existing receipt is `change-proof/v1`: it hashes the tested
  content, the delta features, and the change runner command, and reads the
  regular runner counts. The change adds the declared E2E package digests, the
  E2E runner command digest, and the running Givn version. Does that identity
  extend `change-proof/v1` with additive fields, or does it require a new proof
  scope version? The answer fixes whether an `ise`-effective change must refuse
  a receipt written before the identity existed, and by which name.
- Options:
  1. **Additive fields in `change-proof/v1`** — the receipt gains optional
     identity and E2E-outcome fields under the existing scope, and an
     `ise`-effective change refuses a receipt that lacks them; gain: no version
     bump, no consumer migration, and an old receipt stays readable for a
     project with `ise` off; cost: one scope version has two shapes, so every
     consumer must treat a missing identity as stale rather than optional; in
     practice: a receipt written today gains `e2e_identity` and `e2e_scope`
     fields, and the next `givn check tasks` on an `ise` change refuses the old
     receipt with a named missing-identity reason.
  2. **A new `change-proof/v2` scope** — an `ise`-effective change requires v2
     and a project with `ise` off keeps v1; gain: the shape is explicit and an
     old receipt is refused by its scope name; cost: two live scopes, and
     review, archive, and any external consumer must learn the new version and
     classify v1 receipts by mode; in practice: the receipt's `content_scope`
     reads `change-proof/v2`, and archive refuses a v1 receipt on an `ise`
     change as the wrong scope version rather than as an incomplete v1.
- Disposition: carried — the design's Decisions ledger decides it; the
  deciding artifact is `design.md`, and the recorded stale-reason names are
  the contract review and archive use.

### D2 — Which package manifests the E2E dependency-edge check reads

- Question: The blocking boundary and the existing advisory finding must say
  whether the declared `e2e` package reaches production source. Which package
  manifests does the first cut read, and what happens on a project whose
  package graph the reader cannot understand? The advisory reader and the
  blocking check share one answer.
- Options:
  1. **Cargo manifests only** — the check reads `Cargo.toml` package manifests
     statically and follows path and workspace-inherited dependencies; gain:
     one deterministic reader with no network, no project toolchain invocation,
     and coverage for the Cargo reference projects, including transitive and
     workspace-inherited edges; cost: a Java, Python, TypeScript, or JavaScript
     project receives no dependency-edge inference from a manifest graph, so a
     declared path that resolves no Cargo package is checked only by the
     references under it; in practice: an `e2e` crate that names the product
     crate as a path dependency is refused by name, while a `package.json`-only
     project is refused only when a reference under its `e2e` path resolves into
     a `source:` root.
  2. **Cargo plus the outline's other stacks** — read Cargo, `pyproject.toml`
     or `requirements.txt`, Maven or Gradle, and `package.json`; gain: the
     finding covers the stacks Givn already analyzes; cost: four graph readers
     and their failure modes, and the check must refuse where it cannot read;
     in practice: a Java project's `e2e` module is refused when its Maven
     dependency graph reaches production classes, and a malformed `pom.xml`
     must produce a named refusal rather than a silent pass.
  3. **A project-declared dependency command** — the declaration names a
     command that prints the `e2e` package's dependencies, and the check reads
     its output; gain: one contract for any stack; cost: a command surface and
     a trust boundary, a wrong command reports a false boundary, and `givn
     lint` stops being static; in practice: the declaration gains a
     `dependencies_command`, and a command that exits 0 with an empty list
     admits a package that links production source.
- Disposition: carried — the design's Decisions ledger decides it under the
  owner named there; the deciding artifact is `design.md`, and the limit is
  repeated in the `verify-change` use-case document and the reference
  documentation.
