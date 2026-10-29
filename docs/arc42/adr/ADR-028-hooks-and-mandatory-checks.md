---
status: accepted
date: 2026-07-16
decision-makers:
  - givn maintainers
consulted:
  - hook implementation and gate tests
informed:
  - contributors and users configuring lifecycle checks
---

# Separate configurable post hooks from mandatory gate checks

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - gate ordering and failure semantics affect every project lifecycle.
- [x] **Shapes quality attributes** - fail-closed verification and integrity checks protect correctness.
- [x] **Structural** - the binary's mandatory checks and manifest `post` hooks are distinct execution paths.
- [x] **Must be shared** - users must not mistake a removable hook for a non-deactivatable review requirement.

## Context and Problem Statement

The project needs to run project commands at lifecycle gates while preventing a configuration overlay from disabling core safety checks. The current implementation evolved from hook-based verification to a split: mandatory checks are in `cli::check`, and `post` is additive.

## Decision Drivers

- mandatory review safety
- project-specific additional commands
- explicit `GIVN_FEATURES` propagation
- useful failure and `allow_failure` semantics

## Considered Options

- Put every check in user-configurable `post` lists.
- Hardcode every project command and disallow customization.
- Keep binary mandatory checks and allow additive manifest hooks.

## Decision Outcome

Chosen option: **Keep mandatory checks in the binary and use `post` for additional hooks**. `givn check tasks` always runs WIP protection and change verification. `givn check review` always runs `integrity`, the run declaration, change-scoped lint, overlap/retirement/routing, the verification receipt, and scenario-commit checks. Scenario execution is proven by the receipt and by archive's merged scopes, not re-run in review. These mandatory checks are not disabled by removing or marking a `post` entry. Configured `post` hooks run separately; built-in names (`verify`, `verify-e2e`, `lint`, `integrity`) cannot be downgraded with `allow_failure`, while custom shell commands can.

The original record listed `verify` and `verify-e2e` as review members. The
verification split moved change-suite execution to `givn check tasks` and merged
suite execution to `givn archive`; review validates the receipt. The
non-deactivatable boundary and the hook-versus-check separation are unchanged.

The `run-command`, `visual-ui-review`, `web-ui-verification-hardening`, and
`persona-identity-reuse` changes each amended this decision by adding a member
to the integrity mandatory check — the run declaration, the visual verdict and
evidence, the persona disposition with state evidence and deferral pair, and
the persona-ID resolution respectively. The register in
[`../09-architecture-decisions.md`](../09-architecture-decisions.md) records
the amendments.

**Retired amendment.** The `add-readability-review-gate` change once added a
conditional readability/outline member with `READABILITY: FIXED` /
`READABILITY: JUSTIFIED` markers. That member is retired. `givn check review`
performs no readability or outline analysis, requires no `OUTLINE` or
`READABILITY` verdict, and accepts no metric waiver. The outline addon is
advisory only. The record's original non-deactivatable-check boundary remains
current; the retired member and its markers are historical.

The `prepare-0-9-10` change confirms the current members: integrity (including
the visual/persona checks), run declaration, change-scoped lint,
overlap/retirement/routing, verification receipt, and scenario commits. None can
be removed by a `post` entry.

### Consequences

- The embedded default manifest has empty `post` lists for `tasks` and `review`; older comments claiming default `post: [verify]` are not the effective runtime contract.
- `givn check` without an artifact runs only configured `post` hooks for applicable outputs; it does not invoke the artifact-specific mandatory path.
- `givn archive` runs configured hooks of `apply.requires`, then performs its own hard-coded structural gates, candidate publication, per-scope verification, optional coverage, and finalization stages.
- `verify.command` and coverage commands are read from `givn/commands.yaml` through its dedicated executable schema after the `givn/config.yaml` policy overlay.

### Confirmation

Hook and gate tests assert mandatory checks, non-degradable built-ins, custom `allow_failure`, feature-path propagation, archive hooks, and post-merge verification.

## Pros and Cons of the Options

### Mandatory checks plus additive hooks

- Good, because a project can extend checks without weakening the core gate.
- Bad, because users must understand two execution paths.

### All checks configurable

- Good, because the manifest is uniform.
- Bad, because an overlay could remove the checks that protect review integrity.

### All checks hardcoded

- Good, because behavior is simple to reason about.
- Bad, because projects cannot add their own validation or choose custom commands.

## More Information

The current execution split is documented in [chapter 8.12](../08-crosscutting-concepts.md#812-hooks-command-execution-and-mandatory-checks) and [chapter 6](../06-runtime-view.md).
