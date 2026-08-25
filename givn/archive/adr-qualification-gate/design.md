# Design: ADR Qualification Gate

## Architecture Impact

This change modifies the authoring contract for Arc42 chapter 9 and the
associated design-review guidance. It does not change the Givn runtime or the
meaning of existing product specifications.

The resulting flow separates three decisions that are currently conflated:

1. whether a change affects an Arc42 chapter;
2. whether a design choice is a candidate for architectural rationale; and
3. whether that candidate passes the ADR qualification gate.

Only the third outcome creates or changes a MADR record. A chapter may be
affected while the candidate is documented in a feature, `design.md`, chapter
8, a process document, or ordinary project documentation.

The qualification/single-source policy is not an ADR. It is a default authoring
policy documented in Arc42 and the authoring/design-review guidance. A project
override can replace the effective policy through the normal asset cascade.
This policy is not runtime enforcement: Givn does not infer architectural
significance or reject an ADR from the register solely through a binary check.
The LLM, author, and design reviewer must apply the policy and record the
structured verdict.

The current ADR corpus will be reviewed in the same change using the
deterministic manifest below. Existing records are classified as independent
architecture decisions, refinements of a named governing ADR, genuinely
superseded architecture history, or non-ADR material. Non-ADR material is moved
once to its one named canonical home or deleted when already represented; it is
not copied into a second historical register. A record with no remaining
information value is removed from the ADR archive as well as the active index.

## Planned Artifacts

The implementation must align every live source that can expose this policy.
The complete MADR template and qualification procedure are owned by
`assets/addons/arc42-docs/instructions.md`. The change-level impact template
`assets/addons/arc42-docs/template.md` is only an Arc42 impact template; it is
not an ADR template and must not contain the complete qualification procedure.
The source inventory is:

- `assets/addons/arc42-docs/instructions.md` — complete MADR template,
  qualification procedure, strict Boolean gate, routing, and source ownership.
- `assets/addons/arc42-docs/template.md` — change-level impact assessment only,
  with the distinction between chapter impact and ADR requirement.
- `assets/addons/arc42-docs/chapters/README.md` — durable chapter and ADR
  storage contract, one-decision-per-ADR rule, and qualification summary.
- `assets/addons/arc42-docs/chapters/09-architecture-decisions.md` — source
  chapter guidance for the register/index and decision links.
- `assets/instructions/specs.md` — canonical interaction-inventory and E2E
  policy, including the lower-level source boundary where relevant.
- `assets/instructions/design-review.md` — effective review policy and strict
  qualification/routing check.
- `assets/commands/givn-design-review.md` — generated command choreography that
  references the canonical qualification contract without replacing it.
- `assets/templates/design-review.md` — review artifact structure and
  qualification evidence fields.
- `assets/skills/givn-design-review/SKILL.md.tmpl` — generated skill source;
  generated copies are snapshots, not independent policy authorities.
- `docs/TASKS.md` — historical task references to removed ADR identifiers are
  kept aligned with the canonical process documentation.

The implementation also updates the durable Arc42 chapters named in the impact
mapping below, `docs/arc42/09-architecture-decisions.md`, and
`docs/arc42/adr/README.md` as applicable. Those durable edits are already in
this working tree and are covered by the scenario commits and final review
evidence.

## Durable Arc42 Chapter Mapping

The change-level assessment marks a chapter Yes only when the implementation
updates a named section or register. The following mapping is the work contract,
and the listed durable chapter and register edits are present in this working
tree.

| # | Chapter | Affected? | Durable update required |
|---:|---|:---:|---|
| 1 | `docs/arc42/01-introduction-and-goals.md` | Yes | Add the documentation-quality goal: the active architecture contract remains small, current, and free of duplicate/non-ADR records; update acceptance ownership if needed. |
| 2 | `docs/arc42/02-architecture-constraints.md` | Yes | Add the repository authoring constraints: strict five-dimension qualification, one ADR per independent boundary, canonical single source, and no runtime enforcement claim. |
| 3 | `docs/arc42/03-context-and-scope.md` | Yes | Update the existing `givn instructions` CLI guidance surface and identify effective instruction sources and project overrides; no new runtime interface is introduced. |
| 4 | `docs/arc42/04-solution-strategy.md` | No | No product implementation strategy changes; the source-policy implementation remains documentation work. |
| 5 | `docs/arc42/05-building-block-view.md` | No | No production building block or responsibility boundary changes. |
| 6 | `docs/arc42/06-runtime-view.md` | No | No runtime sequence or command execution flow changes; guidance evaluation remains human/LLM authoring work. |
| 7 | `docs/arc42/07-deployment-view.md` | No | No binary, dependency, host, model, or deployment topology changes. |
| 8 | `docs/arc42/08-crosscutting-concepts.md` | Yes | Document strict qualification/routing, supporting-only `Must be shared`, canonical source ownership, project override behavior, and the non-runtime-enforcement boundary. |
| 9 | `docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/README.md` | Yes | Apply the ID-by-ID register migration, preserve only genuinely architectural superseded history, relocate non-ADR behavior to its canonical source, amend the named refinements, and leave ADR-063 proposed and owned by its active change. |
| 10 | `docs/arc42/10-quality-requirements.md` | Yes | Add quality scenarios for every mandatory gate, strict AND semantics, structured verdicts, routing, source/register integrity, and effective/generated guidance parity. |
| 11 | `docs/arc42/11-risks-and-technical-debt.md` | Yes | Record guidance drift, migration loss, one-decision merge risk, and the limitation that current integrity hooks do not semantically validate the ADR register. |
| 12 | `docs/arc42/12-glossary.md` | Yes | Define qualification verdict, mandatory dimension, canonical artifact, supporting-only evidence, amendment, supersession, and active register terms. |

Rows 1-3 are intentional Yes decisions despite no product runtime change: row
1 changes the documentation-quality goal, row 2 changes the repository's
authoring constraints, and row 3 changes existing user-facing CLI guidance
returned by `givn instructions`. Rows 4-7 are No because no implementation
strategy, component, runtime flow, or deployment topology changes.

## Integrity And Enforcement Boundary

The current integrity hooks mechanically check promised files, hashes, and
resolvable affected-file edits; they do not semantically validate whether an
ADR satisfies the five mandatory dimensions, whether the register is
canonical, or whether independent decisions were merged correctly. The
implementation must therefore add explicit guidance, executable scenarios,
source-parity checks, and human design/conformance review evidence. It must not
claim that the runtime or an integrity hook enforces architectural
qualification. A project override may replace the default authoring guidance;
that override is an authoring choice and must be visible in the effective
guidance output.

No Rust runtime behavior, public CLI command semantics, persisted runtime schema,
or external dependency is changed; two source comments are corrected to remove
stale ADR references. Givn does not evaluate the semantic truth of
an ADR candidate at runtime; the structured verdict is an LLM/authoring
protocol whose wording is verified through the guidance and CLI-output
contracts.

## Internal Primitives

None. This change introduces no production Rust struct, function, enum, schema,
or runtime abstraction. The structured verdict is a guidance-level JSON
contract, and the test bindings use the existing `GivnWorld` and subprocess
helpers.

## ADR Qualification Model

The implementation will make the following checks explicit in the guidance.
Qualification is a strict Boolean AND: every mandatory dimension below must
pass. The MADR checklist supplies evidence for the dimensions; it is not an
at-least-one escape hatch.

The mandatory dimensions are:

- `alternatives` — one real choice between materially different alternatives;
- `architectural_impact` — a boundary, contract, dependency direction,
  system-wide quality constraint, deployment/data boundary, or compatibility
  consequence;
- `durable_consequence` — a lasting effect such as migration cost, broad code or
  interface change, cross-command behavior, or a fundamental invariant;
- `lower_level_artifact` — evidence that a feature specification, technical
  design, cross-cutting Arc42 section, process instruction, ordinary project
  documentation, or code is not the correct complete home; and
- `existing_adr_check` — a completed search for the same or related decision,
  with an explicit `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, or
  `CANONICAL_ARTIFACT` route.

`Must be shared` is supporting evidence only. It is represented as
`SUPPORTING` or `NOT_SUPPORTING`, never as a mandatory gate, and cannot qualify
a decision by itself.

### Structured verdict contract

Before drafting an ADR, the LLM emits one structured verdict with these fields:

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "NEW_ADR",
  "canonical_artifact": null,
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [],
    "architectural_impact": [],
    "durable_consequence": [],
    "lower_level_artifact": [],
    "existing_adr_check": []
  }
}
```

The five mandatory fields are `PASS` only when their own decision-specific
evidence is present. `qualification` is `QUALIFIED` only when all five are
`PASS`; otherwise it is `NOT_QUALIFIED`. The evidence arrays contain only facts
and alternatives supplied by the author. Missing evidence stays empty and
causes the corresponding gate to fail. The guidance must not invent options,
impact, consequences, participants, or other evidence. `routing` is
`CANONICAL_ARTIFACT` for a non-qualifying candidate, `NEW_ADR` for a qualified
independent decision with no existing record, `AMEND_ADR` for a refinement, and
`SUPERSEDE_ADR` for a qualified replacement. `canonical_artifact` is required
for `CANONICAL_ARTIFACT`; `target_adr` is required for `AMEND_ADR`; and
`replacement_adr` is required for `SUPERSEDE_ADR`. The unused route-target
fields are `null`. A project may override this default authoring policy through
the cascade; no runtime validator enforces the JSON verdict or routing choice.
The schema is a prompt-level contract, not a new persisted Givn artifact.

Every MADR still evaluates all four checklist items with a concrete,
decision-specific reason. The checklist records evidence for the five
mandatory dimensions; checking `Must be shared` records supporting context and
does not alter the Boolean result.

## Consolidation Scope

The audit uses the following deterministic ID-by-ID migration manifest. Every
record has exactly one disposition. `retain` means that the record remains one
independent ADR. `absorb/amend into ADR-<id>` is reserved for a refinement of
that one governing decision, not for an unrelated decision. `superseded/historical`
means that a genuinely architectural old decision stays in the ADR archive with
an explicit replacement. `delete as redundant` removes a non-ADR record whose
useful information already exists in its named
canonical source. Reserved IDs have no record and therefore have no migration
action. `ADR-063` is an explicit exclusion, not a migration disposition.

| ID | Action | Target ADR / canonical artifact | Rationale |
|---|---|---|---|
| ADR-001 | retain | ADR-001 | Standalone product and project boundary is an independent durable architecture decision. |
| ADR-002 | retain | ADR-002 | Production language and native implementation boundary are independent of the product boundary. |
| ADR-003 | delete as redundant | `Cargo.toml` | Product naming is not architecture and the current name is already represented by package metadata; no historical ADR copy is needed. |
| ADR-004 | retain | ADR-004 | Native executable and runtime-prerequisite deployment boundary is independent of language selection. |
| ADR-005 | retain | ADR-005 | Cascade precedence is an independent ownership and upgrade boundary. |
| ADR-006 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.1 | Eject behavior is already the canonical cascade documentation; the materialization command is not an independent architecture decision. |
| ADR-007 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.1 | Facet addressability is already the canonical cascade documentation; it is not an independent durable trade-off. |
| ADR-008 | retain | ADR-008 | Visible `givn/` filesystem ownership is an independent project-layout boundary. |
| ADR-009 | retain | ADR-009 | YAML manifest representation is an independent durable format decision. |
| ADR-010 | retain | ADR-010 | Overlay patch semantics are independent of the YAML representation and graph topology. |
| ADR-011 | retain | ADR-011 | The executable `.feature` specification boundary is an independent durable contract. |
| ADR-012 | delete as redundant | `assets/addons/coverage/instructions.md` | Coverage classification is governance already represented in the canonical coverage guidance, not an architecture decision. |
| ADR-013 | retain | ADR-013 | Generated-agent and user-owned-file boundaries are independent of target selection. |
| ADR-014 | retain | ADR-014 | Filesystem-derived lifecycle state is an independent state boundary. |
| ADR-015 | retain | ADR-015 | Explicit project test-runner declaration is an independent public runner contract. |
| ADR-016 | delete as redundant | `README.md` | Repository location and early version-control setup contain no durable product architecture information; contributor guidance is the sole relevant project home. |
| ADR-017 | retain | ADR-017 | The clean OpenSpec compatibility boundary is an independent parser and layout decision. |
| ADR-018 | delete as redundant | `PLAN.md` | MVP scope is product planning and belongs in the project plan. |
| ADR-019 | superseded/historical | ADR-027; `docs/arc42/adr/archive/ADR-019-lint-contract.md` | The original static-check contract is genuinely architectural history and is explicitly superseded by the final `givn lint` contract. |
| ADR-020 | retain | ADR-020 | Delta tag namespace and merge-operation parsing are an independent executable-spec contract. |
| ADR-021 | absorb/amend into ADR-015 | ADR-015 | Fail-closed missing-runner behavior refines the explicit runner contract and does not warrant a second ADR. |
| ADR-022 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.10 | The documented scaffold behavior already has its canonical configuration section; comments and discoverability do not form an independent ADR. |
| ADR-023 | retain | ADR-023 | Versioned upgrade and override preservation are an independent migration-safety boundary. |
| ADR-024 | retain | ADR-024 | Configurable lifecycle graph topology is an independent workflow architecture boundary. |
| ADR-025 | absorb/amend into ADR-011 | ADR-011 | Specification/implementation separation is the authoring consequence of the executable feature specification boundary, not an independent product decision. |
| ADR-026 | retain | ADR-026 | Arc42 addon lifecycle and fixed durable-output boundary are independent from conformance review procedure. |
| ADR-027 | retain | ADR-027 | `givn lint` naming and public exit contract are an independent CLI interface decision. |
| ADR-028 | retain | ADR-028 | Binary mandatory checks versus additive hooks are an independent gate-execution boundary. |
| ADR-029 | delete as redundant | `assets/commands/givn-tasks.md` | Carrying design constraints into tasks is implementation guidance already expressed in the canonical task instruction, not product architecture. |
| ADR-030 | delete as redundant | `assets/default-manifest.yaml` | A required planning checkpoint is workflow governance already represented by the canonical lifecycle graph, not a runtime architecture decision. |
| ADR-031 | delete as redundant | `assets/commands/givn-explore.md` | Keeping Explore outside lifecycle state is a workflow rule already represented by the canonical command guidance. |
| ADR-032 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.13 | The superseded reverse-engineering experiment adds no current architecture contract; its historical rationale is not retained as an ADR copy. |
| ADR-033 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.13 | The peer flow is a lifecycle/process behavior already represented by the canonical cross-cutting chapter, not a separate ADR. |
| ADR-034 | delete as redundant | `Cargo.toml` license metadata | Licensing is legal governance, and the actual license choice is already recorded as package metadata; do not retain a non-ADR archive copy. |
| ADR-035 | delete as redundant | `docs/arc42/08-crosscutting-concepts.md` section 8.14 | Post-implementation Arc42 conformance is already the canonical cross-cutting review procedure, not an ADR. |
| ADR-036 | absorb/amend into ADR-013 | ADR-013 | Persisted agent-target selection is a detail of the generated-agent ownership boundary, not an independent architecture decision. |
| ADR-037 | retain | ADR-037 | Deterministic structural overlap evidence is independent of semantic retrieval and its strict gate. |
| ADR-038 | delete as redundant | `assets/instructions/specs.md` | The normative interaction-authoring policy is already in the canonical specs instruction; no ADR copy is needed. |
| ADR-039 | retain | ADR-039 | Local E5 retrieval and persistent-index boundary are independent from each command surface. |
| ADR-040 | delete as redundant | `docs/arc42/05-building-block-view.md` | Repository-owned compiled-CLI fixtures are test support already represented by the canonical acceptance building-block documentation; the record adds no unique architecture rationale. |
| ADR-041 | retain | ADR-041 | Binary-fixed semantic uniqueness is an independent quality gate. |
| ADR-042 | superseded/historical | ADR-052; `docs/arc42/adr/archive/ADR-042-bge-experiment.md` | The experimental reranker interpretation is genuine semantic architecture history and is explicitly superseded by advisory BGE evidence. |
| ADR-043 | delete as redundant | `givn/specs/spec-explore-permanent/spec-explore-permanent.feature` | Permanent Explore's observable query contract is already executable in the permanent specification; no ADR copy is needed. |
| ADR-044 | retain | ADR-044 | Request-local active-change corpus and persistence boundary are independent from permanent Explore. |
| ADR-045 | retain | ADR-045 | Project-relative scenario identity is a shared identity boundary used by multiple retrieval surfaces. |
| ADR-046 | retain | ADR-046 | Change-local blocking semantic report and carry-forward contract are independent from remediation verification. |
| ADR-047 | retain | ADR-047 | Projected-tree remediation verification is an independent evidence and safety boundary. |
| ADR-048 | retain | ADR-048 | Machine-readable instruction metadata is an independent agent interface contract; the prose policy itself is relocated from ADR-038. |
| ADR-049 | delete as redundant | `givn/specs/spec-duplicates/spec-duplicates.feature` | The permanent audit's observable behavior is already executable in the permanent specification; no ADR copy is needed. |
| ADR-050 | reserved/unassigned (no record) | none | Reserved identifier; no decision exists to migrate or archive. |
| ADR-051 | reserved/unassigned (no record) | none | Reserved identifier; no decision exists to migrate or archive. |
| ADR-052 | retain | ADR-052 | Advisory BGE admission evidence is an independent second-stage model boundary. |
| ADR-053 | superseded/historical | ADR-063; `docs/arc42/adr/archive/ADR-053-bge-pair-cap.md` | Fixed pair-cap refusal is genuine architecture history superseded by the proposed budget decision; keep it archived while ADR-063 remains incomplete. |
| ADR-054 | retain | ADR-054 | Versioned opt-in evidence output is an independent machine-consumer compatibility boundary. |
| ADR-055 | retain | ADR-055 | Independent retrieval/model feature selection is an independent Cargo/runtime capability boundary. |
| ADR-056 | retain | ADR-056 | Matching benchmark approval for product BGE is an independent admission boundary. |
| ADR-057 | retain | ADR-057 | Preflighted local BGE artifact loading is an independent deployment and no-download boundary. |
| ADR-058 | retain | ADR-058 | Independent NLI evidence and bidirectional output are an independent model boundary. |
| ADR-059 | retain | ADR-059 | Complete semantic serialization and exact hash are the governing shared model-input boundary. |
| ADR-060 | retain | ADR-060 | Actual-tokenizer cap handling is an independent inference-safety boundary. |
| ADR-061 | retain | ADR-061 | Surface-specific independent BGE/NLI pools are an independent candidate-selection boundary. |
| ADR-062 | absorb/amend into ADR-059 | ADR-059 | Empty optional-field omission refines the exact complete serializer; it does not define a separate independent boundary. Amend ADR-059 and remove the parallel ADR-062 record. |
| ADR-063 | excluded from this migration | `docs/arc42/adr/ADR-063-reranker-pair-budget.md`; owner `semantic-reranker-budget` | The change is incomplete. Leave ADR-063 `proposed`, do not amend or archive it here, and let its owning change decide acceptance. |
| ADR-064 | delete as redundant | `givn/commands.yaml` | The later duplicate label records coverage command governance already represented by the project command file; remove the non-ADR archive copy. |
| ADR-065 | delete as redundant | `README.md` | The later duplicate label records managed README coverage blocks already represented by the README; remove the non-ADR archive copy. |

No independent decision is merged merely to reduce the register count. The
independent records remain separate for product/distribution, manifest format
and overlay, runner/gate contracts, semantic evidence layers, and model-input
boundaries. The absorb/amend actions are limited to ADR-021 into ADR-015,
ADR-025 into ADR-011, ADR-036 into ADR-013, and ADR-062 into ADR-059. Each is a
documented refinement of its target decision. `ADR-043` and `ADR-049` are not
absorbed into `ADR-039`; their observable contracts remain only in their
permanent specifications. Process-only records are deleted or represented by
their existing canonical artifact instead of being merged into an ADR.

Genuinely architectural superseded records remain only under
`docs/arc42/adr/archive/`. Non-ADR records in the delete rows leave both the
active index and the ADR archive. The implementation must apply this manifest
without touching ADR-063.

## Test Runner And Strictness

The existing runner remains `./run-tests.sh`. It executes the Cucumber binary
over the feature files selected through `GIVN_FEATURES`; the change feature is
run with:

```sh
GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
  cargo test --features spec-retrieval --test features_runner -- \
  --name '^The guidance defines strict qualification as an all-gates contract$'
```

The strict-mode mechanism is `GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`. Cucumber must fail on skipped or undefined steps;
new step definitions start with `unimplemented!()` until their scenario enters
GREEN, and no empty or silently passing step body is permitted. The exact
scenario command uses an anchored `--name '^...$'` regex so it runs only the
named scenario.

Regular scenario bindings read the canonical guidance, source assets, durable
register, and generated snapshots and assert their observable text, fields,
links, statuses, and ownership rules. They do not implement or simulate an ADR
qualification evaluator. The E2E bindings assert the same policy through the
real CLI output.

The E2E command remains `./run-tests.sh --e2e`, which is a strict subset of
the full runner through its `@e2e` tag filters. E2E bindings are separate from
regular bindings in `tests/steps/adr_qualification_gate_e2e_steps.rs`; regular
bindings live in `tests/steps/adr_qualification_gate_steps.rs`.

For the scenario-by-scenario E2E loop, the exact anchored single-scenario
commands are:

```sh
GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
  cargo test --features spec-retrieval --test features_runner -- \
  --name '^An author requests the strict ADR guidance through the Givn CLI$'

GIVN_FEATURES=givn/changes/adr-qualification-gate/specs \
  cargo test --features spec-retrieval --test features_runner -- \
  --name '^A reviewer requests design-review guidance through the Givn CLI$'
```

The full `verify.e2e_command` remains the configured gate; these direct
anchored invocations are only the required single-scenario evidence commands.

The capability is CLI/documentation based. Local execution uses temporary,
initialized Givn projects and the compiled Givn subprocess. The E2E setup uses
the unique change id `adr-qualification-gate` and a capability-specific fixture
step; it does not reuse the hardcoded `instruction-integrity` fixture or its
generic step phrase. There are no servers, databases, queues, browsers,
external services, or digital twins. The local run command is `./run-tests.sh`;
no additional service startup is required.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| request the Arc42 ADR guidance through the Givn CLI | An author requests the strict ADR guidance through the Givn CLI | CLI / terminal | Cucumber E2E binding creates an initialized `adr-qualification-gate` fixture and starts the compiled `givn instructions arc42-docs --change adr-qualification-gate` subprocess; the primary assertion checks stdout and exit status |
| request the design-review guidance through the Givn CLI | A reviewer requests design-review guidance through the Givn CLI | CLI / terminal | The same capability-specific E2E binding uses the initialized `adr-qualification-gate` fixture and starts the compiled `givn instructions design-review --change adr-qualification-gate` subprocess; the primary assertion checks stdout and exit status |

## Design Decisions Requiring User Input

### Decision 1: Qualification gate semantics

Decision: **STRICT**. A candidate needs real materially different alternatives,
architectural impact, durable consequence, correct lower-level artifact
classification, and a completed existing-ADR/duplicate check. All five are
mandatory and combine with Boolean AND semantics. The MADR checklist is
evidence for these dimensions, not an at-least-one escape hatch. `Must be
shared` is supporting evidence only and can never qualify a candidate on its
own.

Status: **DECIDED**

### Decision 2: Historical storage of reclassified records

Decision: **CANONICAL SINGLE SOURCE**. A record that is not an ADR is not kept
as a historical ADR. Its useful information is moved once to the relevant
Arc42 chapter, specification, project documentation, or code. If that
information already exists there, the redundant record is removed. No
`non-ADR historical record` copy or compatibility stub is created in the ADR
archive.

Status: **DECIDED**

### Decision 3: Consolidation target

Decision: **AGGRESSIVE DRY**. Keep one ADR per independent durable architecture
boundary or trade-off. Merge only a closely coupled refinement into its named
governing ADR, remove process/design/detail records, and do not preserve a
separate ADR just to retain its historical file name. Do not combine unrelated
decisions merely to reach an arbitrary count. Each retained ADR contains
rationale and links only; current architecture facts remain in Arc42 and
implementation details remain in code. The migration manifest has four
absorb/amend actions: ADR-021 into ADR-015, ADR-025 into ADR-011, ADR-036 into
ADR-013, and ADR-062 into ADR-059.

The manifest above replaces the earlier broad merge proposal. It retains
independent decisions even when they touch one subsystem: ADR-005/008,
ADR-009/010/024, ADR-015/027/028, ADR-013/048, ADR-037/041,
ADR-039/044/045, ADR-046/047, and ADR-052/054/055/056/057/058/059/060/061
remain separate. ADR-021, ADR-025, ADR-036, and ADR-062 are amended into their
named governing ADRs because each is a refinement rather than an independent
trade-off. ADR-019, ADR-042, and ADR-053 remain historical only when their
explicit superseding ADR exists. ADR-043 and ADR-049 are removed as redundant
because their behavior is already owned by permanent specifications and Arc42
context. No action in this change changes the status or ownership of ADR-063.

Status: **DECIDED**

## Implementation Work Remaining

The qualification procedure, derived guidance, durable Arc42 chapters, ADR
register, retained ADR verdicts, source comments, generated mirrors, and
historical task references are already aligned in this working tree. The
scenario task list must still add the regular and E2E step definitions, verify
the canonical guidance/source and register contracts, and record the evidence
without presenting the integrity hooks as a runtime qualification validator.

- Add regular and E2E step definitions under the capability-specific test
  paths. The E2E fixture must initialize a project, create
  `adr-qualification-gate`, invoke the compiled CLI as a subprocess, and assert
  stdout and exit status before any secondary filesystem checks.
- Add semantic source-parity and register-review evidence without presenting
  the current integrity hooks as a runtime qualification validator.

### Decision 4: ADR status of this policy change

Decision: **NO ADR**. The new qualification and canonical-source policy is a
documentation and authoring governance change, not a new product architecture
boundary. It is documented in the applicable Arc42 chapters and Givn authoring
and design-review guidance. It is the default policy, can be replaced by a
project override through the asset cascade, and is not enforced by the runtime.
The change does not create an ADR for itself.

Status: **DECIDED**
