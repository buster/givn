# Design Review: nli-evidence

## Grilling Outcome

The fresh adversarial review checked the proposal, delta specification, design,
existing retrieval/BGE implementation, canonical specs guidance, and all Arc42
chapters. It identified the following gaps before implementation:

| Finding | Resolution |
|---|---|
| Change-aware exploration was conflated with permanent exploration. | Added a separate inventory entry, E2E scenario, matrix row, and assertions for the active-change union and source/projected-target identity exclusions. |
| Combined BGE/NLI unavailable or unscored evidence had no blocking-safety scenario. | Added a scenario requiring an unresolved E5 candidate to remain blocking without a false filtered status. |
| NLI pair-cap refusal and the 513-pair unavailable state were untested. | Added a bounded-pool overflow scenario covering reason, zero scoring/load counts, and E5 unscored fallback. |
| Partial bounded NLI coverage was untested. | Added a JSON scenario covering `NLI_UNSCORED`, null probabilities, ordering, and exclusion from filtered evidence. |
| NLI JSON/schema-v3 output was untested. | Added a JSON scenario covering schema version, additive raw-result fields, model/probability/count/runtime evidence, and E5 order. |
| Compiled-but-disabled NLI behavior was untested. | Added a disabled-capability scenario asserting the unchanged E5-only contract and absence of NLI claims. |
| The legacy `@nli-future` scenario was claimed as modified but had no delta. | Added a `@givn.modified` scenario that exercises the real NLI reduction while retaining diagnostic wording and E5 identity assertions. |
| An NLI-recommended unresolved candidate had no blocking non-zero scenario. | Added a scenario requiring a non-zero blocking review and an NLI-recommended report block. |
| New scenarios lacked `@wip` during the pre-implementation phase. | Tagged all sixteen scenarios `@wip`; each will be cleared only after its own RED/GREEN/REFACTOR cycle. |
| The existing diagnostic output could make a basic probability-only scenario pass in RED. | Kept endpoint-order independence as a concrete assertion; implementation must bind it to the new bounded NLI path rather than the legacy early return. |

## Scope

The proposal, feature delta, and design now agree on five E5-backed search
surfaces: permanent exploration, change-aware exploration, advisory review,
blocking semantic review, and permanent duplicate audit. NLI remains advisory
evidence, reduces the recommended worklist, preserves the E5 raw set, and never
assigns a Givn classification or edits specifications.

## Technical Choices

The existing Rust retrieval module, ONNX Runtime/tokenizer path, local artifact
preflight, deterministic stubs, and `cucumber-rs` subprocess fixtures are the
smallest stack that satisfies the observable contracts. Reusing the existing
bounded E5 pool avoids a second candidate generator and makes E5+BGE, E5+NLI,
and combined measurements comparable. No external service or new runtime
dependency is required.

## Testability And E2E

Every delta scenario has a concrete output, count, status, report, or fallback
assertion and is currently marked `@wip`, so `givn lint --change nli-evidence`
returns the expected WIP exit code `2`. The five inventory entries for the
permanent exploration, change-aware exploration, advisory review, blocking
review, and duplicate audit are mapped to five `@e2e` scenarios in
`design.md`; all use the real compiled CLI subprocess and assert CLI output or
exit status as the primary interface. The blocking report is secondary evidence
because it is the command's generated artifact.

Strict mode is `.fail_on_skipped()` in `tests/features_runner.rs`. New step
bindings begin with `unimplemented!()` during RED; no empty or silent step body
is permitted.

## Arc42 Verification

The fresh review independently marked all twelve Arc42 chapter rows affected.
`givn/changes/nli-evidence/arc42.md` contains all twelve rows and
`STATUS: DONE`. Every durable chapter contains NLI-specific content. ADR-058
documents the independent bounded NLI decision, and risks R-62 through R-65
cover its false-negative, calibration, pool-parity, and artifact consequences.
No prohibited ASCII diagram was introduced; diagrams remain Mermaid blocks.

## Risk Decision

The highest risk is filtering a true duplicate or variant before the blocking
worklist. The implementation must preserve E5 raw/unscored candidates, keep
unavailable/unscored candidates blocking, and expose directional NLI evidence.
The bounded deterministic pool and later labeled quality measurement are the
mitigation; NLI statuses remain non-authoritative evidence.

## Post-Review Clarification

The coverage workflow was rechecked after implementation exposed a matrix
detail: `measure-coverage.sh` now detects the active `@nli-evidence` feature
and uses the combined retrieval feature set, while permanent BGE coverage uses
the BGE-only feature set so the existing NLI-isolation scenario remains valid.
This is an explicit test-infrastructure clarification only; the CLI product
boundary, five interaction rows, strict runner, and Arc42 assessment are
unchanged. The active NLI coverage run produced fresh non-E2E and E2E reports,
and both feature matrices passed.

## Status

DESIGN-REVIEW: PASS
