# Design Review: fix-skill-instruction-integrity

## Grilling

### Scope

The proposal asks for complete authoring guidance, retrieval-aware guidance,
semantic-remediation evidence, effective-policy documentation, and a
machine-readable instruction contract. The spec covers each item with regular
observable scenarios and one E2E scenario for each normalized CLI interaction:

- canonical specs guidance, including retrieval capability state;
- technical design guidance;
- remediation-aware review guidance;
- machine-readable artifact guidance.

The separate retrieval scenario is a regular variant of the canonical specs
request, not a second E2E action. The explicit unknown-change scenario is a
negative variant of the same machine request. No scope gap remains.

### Technical choices

The change uses the existing Rust CLI, the existing cascade and manifest data,
and the existing repository-owned Cucumber runner. This is the smallest
implementation that satisfies the observable contract. No new dependency,
runtime service, renderer, or metadata side-channel is introduced.

The JSON shape follows the already documented architecture contract in
`docs/arc42/06-runtime-view.md:71-83` and quality scenario Q-10:
`{id, generates, requires, instruction}`. Commands will consume these fields
instead of inventing undocumented metadata.

### Missing scenarios

The plan covers positive and negative boundaries:

- complete canonical E2E policy;
- complete design setup fields;
- complete semantic classification/remediation evidence;
- valid JSON metadata;
- invalid/unknown change context;
- retrieval-enabled and feature-free guidance;
- project override and generated snapshot semantics through regular asset
  assertions.

No live third-party service or browser is required because the changed public
interface is the local CLI and filesystem. The E2E scenarios invoke the
compiled CLI as a real subprocess.

### Testability

Every scenario has concrete observable assertions on text, exit status, JSON
fields, or rendered/generated files. RED uses `unimplemented!()` and the
runner's `.fail_on_skipped()` strict mode. GREEN and REFACTOR target one
scenario with `--name`. The E2E scenarios use the same compiled CLI boundary
and are isolated through `./run-tests.sh --e2e`.

The E2E inventory has exactly one scenario per distinct action. Retrieval mode
is a variant of the canonical instruction request, not another user action.

### Risk and mitigation

The primary risk is repairing embedded source assets while leaving generated
targets or command consumers stale. Mitigation is to update source assets,
run `givn skills sync`, inspect all configured targets, update the harness
mirrors used by acceptance tests, and assert both source and generated output
in the feature suite.

The JSON contract risk is handled by following the existing documented shape,
preserving human-readable output, validating explicit `--change` context, and
asserting that stdout is one parseable JSON document.

The override risk is handled by testing the effective cascade and documenting
that project overrides replace embedded facets while generated snapshots only
refresh through `givn skills sync`.

### Architecture documentation

The 12-row `arc42.md` assessment was independently checked against the
proposal, specs, and design:

| Chapter | Independent result | Design alignment |
|---|---|---|
| 01 | Yes | FR-30 and FR-31 cover policy integrity and JSON metadata. |
| 02 | Yes | Existing local-first, single-binary, feature-gated constraints remain. |
| 03 | Yes | CLI JSON and effective guidance are external interfaces. |
| 04 | Yes | Cascade plus generated snapshots are the strategy. |
| 05 | Yes | `cli::instructions`, cascade, and targets are affected. |
| 06 | Yes | Instruction resolution and JSON emission are runtime flows. |
| 07 | No | No deployment or distribution change. |
| 08 | Yes | Policy ownership, overrides, snapshots, and E2E fidelity are cross-cutting. |
| 09 | Yes | ADR-048 records the contract and policy decision. |
| 10 | Yes | Q-71 through Q-74 cover the new acceptance signals. |
| 11 | Yes | R-46 through R-49 cover drift and contract risks. |
| 12 | Yes | Effective instruction, JSON contract, and derived guidance are new terms. |

All 12 chapter files exist and contain non-placeholder project content. The
affected chapters contain decision-specific updates. Existing diagrams remain
Mermaid or plain directory listings; no new ASCII diagram is introduced.

## Hardening decisions

- Keep the JSON contract at the documented four fields; do not add a second
  template/context metadata protocol.
- Validate explicit `--change` before emitting artifact guidance.
- Validate explicit `--change` syntax for path safety, while preserving the
  existing behavior that instruction text is global and syntactically valid
  unknown or archived contexts remain acceptable.
- Treat database state as primary E2E evidence only when the database itself is
  the public consumer interface. Browser UI requires a real browser.
- Keep remediation verification in the generic review guidance as derived
  class-specific evidence, while the canonical specs instruction owns the
  cross-interface E2E policy.

## Sign-off

All grilling branches are resolved. No user decision remains open in the
design. The plan is ready for scenario-by-scenario tasks.

## Re-review after compatibility hardening

Implementation initially rejected a syntactically valid but unknown active
change. Existing global-instruction consumers and archived-change guidance
showed that `--change` is a context selector, not an existence requirement.
The design was corrected to validate only kebab-case/path safety. Existing
instruction scenarios and the full repository suite pass with this contract.

DESIGN-REVIEW: PASS
