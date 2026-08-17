# Design: fix-skill-instruction-integrity

## Architecture Impact

This change repairs the authoring-policy boundary without changing the
semantic retrieval algorithm or the lifecycle graph. The effective specs
instruction becomes the complete normative source for interaction and E2E
fidelity. Artifact-specific instructions retain only their own HOW and audit
requirements.

The change has four parts:

1. Complete the canonical specs policy and make the design, review, retrieval,
   override, and interface-taxonomy guidance consistent with it.
2. Add semantic-remediation evidence to the generic review instruction and
   review template.
3. Align commands and generated skills with the actual instruction JSON
   contract documented by the project: `id`, `generates`, `requires`, and
   `instruction`.
4. Add executable Gherkin coverage for the policy text, generated guidance,
   retrieval modes, semantic remediation evidence, and JSON output.

No external service or new runtime dependency is required.

## Source Assets

Update these embedded source assets:

- `assets/instructions/specs.md`: add the primary real-interface assertion,
  browser-driver fidelity, no HTTP/`fetch()` substitution for browser UI,
  design-gap handling, build-aware retrieval guidance, and the strict interface
  taxonomy.
- `assets/instructions/design.md`: restore the design-owned E2E runner,
  step-location, infrastructure, framework, strictness, and matrix fields;
  replace the cookie-aware HTTP workaround example with a browser-context or
  API-only distinction.
- `assets/instructions/review.md`: retain the fabrication checks and add the
  semantic remediation contracts and evidence receipt.
- `assets/templates/design.md`: align interface taxonomy and normalized-action
  terminology with the canonical policy.
- `assets/templates/tasks.md`: use interface-neutral evidence wording while
  retaining the design-specific driver and obstacle checks.
- `assets/templates/review.md`: add semantic classification/remediation tables
  and use interface-neutral assertion wording.
- `assets/templates/spec.feature`: use normalized inventory terminology.
- `assets/commands/` and `assets/skills/`: make command choreography consume
  `id`, `generates`, `requires`, and `instruction`; remove references to
  nonexistent `output`, `template`, `context`, `rules`, and `dependencies`
  response fields. Keep policy references thin and route policy ownership to
  the resolved instructions.

Regenerate the configured `.opencode`, `.agents`, and `.claude` skill targets
from the source templates. Slash commands are generated only for `.opencode`.
The repository's `harness/.opencode` fixtures are updated where they mirror
the command contract used by harness tests.

## Instruction Resolution

`givn instructions <artifact> --json` emits the documented machine-readable
object:

```json
{
  "id": "specs",
  "generates": ["..."],
  "requires": ["..."],
  "instruction": "..."
}
```

`generates` and `requires` come from the effective artifact graph after
project overlays. `instruction` comes from the instruction cascade, where a
project override wins over the embedded asset. Human-readable output remains
the resolved instruction text only.

The explicit `--change` value remains a context selector for the command
contract and is validated for safe kebab-case syntax. Artifact instructions
are global rather than change-specific, so the JSON does not claim that the
instruction text itself varies by change. Invalid path-like ids are rejected
before instruction output is emitted; a syntactically valid but unknown id is
accepted because the instruction text is global.

## Semantic Remediation Guidance

The generic review instruction and template describe the existing verifier
contract without duplicating implementation code:

| Classification | Required evidence |
|---|---|
| `DUPLIKAT` | Redundant projected endpoint is absent and the retained endpoint remains. |
| `VARIANTE` | A parameterized `Scenario Outline` with a non-empty `Examples` table covers the variant; the recomputed score is `< 0.9500`. |
| `VALID-BOUNDARY` | Both endpoints remain, observable evidence differs, and the recomputed score is `< 0.9500`. |
| Any stale, missing, or unresolved entry | Review remains blocked. |

The review records classification, rationale, projected-tree result, failure or
success evidence, and the `semantic remediation verified` receipt. The code
path remains read-only and the report remains atomic on failure.

## Retrieval Modes

Guidance distinguishes the feature modes already defined by the project:

- Feature-free builds report retrieval unavailability and must not silently
  pass the blocking semantic gate.
- `spec-retrieval` builds may index permanent scenarios and generate the
  semantic review worklist.
- `spec-similarity-gate` builds additionally enforce the strict similarity
  gate after the report path.

The authoring guidance names `givn spec index`, `givn spec search`, and
`givn spec explore` as advisory authoring tools, while `givn check review`
remains the blocking change-review path.

## Step Definition Locations

The change has one capability and two step files, with E2E bindings separated
from regular bindings:

| Capability | Regular steps | E2E steps |
|---|---|---|
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` | `tests/steps/instruction_integrity_e2e_steps.rs` |

Regular steps inspect source assets, rendered templates, and deterministic
metadata. E2E steps invoke the compiled Givn CLI as a real subprocess and
assert stdout, stderr, exit status, and JSON output. No production source is
changed by regular documentation assertions except the instruction CLI JSON
contract.

## Test Infrastructure

The repository has no external dependency for this change. The local run
command is:

```sh
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner
```

The configured full verification command is `./run-tests.sh`; it runs the
Gherkin feature runner through the project command. The configured E2E command
is `./run-tests.sh --e2e`, which filters to `@e2e` scenarios. A single
scenario is run with:

```sh
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner -- --name "<scenario title>"
```

The runner uses `fail_on_skipped()` and the new step definitions use
`unimplemented!()` during RED. The full runner and E2E runner are distinct;
the E2E count must be smaller than the full count for this change.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| request canonical interaction and E2E authoring guidance, including retrieval capability state | An author requests the canonical interaction and E2E guidance | CLI | Real compiled Givn subprocess; inspect stdout and exit status |
| request the technical design guidance for E2E setup | An author requests the technical E2E design guidance | CLI | Real compiled Givn subprocess; inspect stdout and exit status |
| complete a semantic review with class-specific remediation evidence | A reviewer requests remediation-aware review guidance | CLI | Real compiled Givn subprocess; inspect stdout and exit status |
| request machine-readable artifact guidance | An agent requests machine-readable artifact guidance | CLI | Real compiled Givn subprocess; parse stdout as JSON |

The retrieval capability is a variant of the canonical specs guidance action,
not a second E2E action. It is covered by the first E2E scenario and by
regular assertions for feature-free and feature-enabled text.

## Internal Primitives

No new domain or production primitives are required. The JSON response is
assembled from existing manifest and cascade data. Step definitions may use
test-only helpers for subprocess setup and asset assertions.

## Verify command

Unit/integration:
```sh
./run-tests.sh
```

E2E smoke tests:
```sh
./run-tests.sh --e2e
```
