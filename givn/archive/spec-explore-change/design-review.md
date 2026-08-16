# Design Review: spec-explore-change

## Scope

The proposal, executable scenarios, design, and Semantic Scenario Consolidation
Plan now describe one PR-2 slice: optional current-change exploration for
complete file and standard-input queries. The slice remains advisory and
read-only. It does not generate a semantic review report, classify candidates,
verify remediation, project archive state, invoke BGE/NLI evidence, or edit
feature files.

The output contract is intentionally additive. Existing PR-1 validation,
permanent candidate selection, ranking, and result fields/order remain stable;
all text results now identify source scope, and all JSON results append `scope`
and `change` metadata. This is documented as additive rather than byte-for-byte
compatibility.

## Decisions Resolved

### Scope

- Active `Added` and `Modified` delta scenarios join the request-local corpus.
- `Removed` scenarios are excluded.
- `--change --file` and `--change --stdin` are supported.
- Standard-input change-aware behavior is regular executable coverage, not a
  new interaction-inventory entry.

### E2E Fidelity and Interaction Coverage

The interface is the real CLI subprocess. The two inventory entries remain
separate `@e2e` scenarios because terminal-text consumption and
machine-readable-JSON consumption are distinct consumer interactions with
different output contracts. Both scenarios invoke the compiled `givn` binary
through `std::process::Command` and assert on exit status and returned stdout.
The change-aware stdin path is covered by a regular file/stdin parity scenario.
The design matrix maps every inventory entry to exactly one e2e scenario.

### Identity and Persistence

- Permanent, delta-source, projected-target, and query paths use one
  project-relative forward-slash normalization helper.
- Added source identity is excluded when it is the query identity.
- Modified source and projected permanent target identities are both excluded
  when they are the query identity.
- Each identity scenario retains another eligible candidate to prove that the
  filter is narrow.
- Delta vectors are request-local and never written to the permanent index.
- Change validation and delta parsing occur before permanent index refresh, so
  unsafe ids, missing changes/spec directories, and malformed deltas fail
  without changing the persisted index.

### Architecture Documentation

Arc42 is enabled. The change-level `arc42.md` contains the complete twelve-row
impact assessment and `STATUS: DONE`. Durable chapters, the Arc42 index, ADRs,
quality scenarios, risks, and glossary were updated for request-local
current-change exploration, additive source metadata, canonical path identity,
and rejection-before-persistence behavior. The chapter review found all twelve
chapter files substantive and no prohibited ASCII-art diagrams.

## Review Matrix

| Branch | Finding | Resolution |
| --- | --- | --- |
| Scope | The initial design was broader than the original PR-2 wording. | Proposal, plan, design, and feature now explicitly include file and stdin scope, while keeping review, remediation, and model-evidence layers out. |
| Technical choice | A second persisted delta index or new dependency would add unnecessary state. | Reuse the existing parser, permanent refresh, embedding backend, ranking, and JSON serializer; hold delta vectors only for the request. |
| Missing scenarios | Lifecycle identity, removed scenarios, rejected changes, path-form parity, stdin parity, and additive no-change metadata were initially under-covered. | Executable scenarios now cover each boundary, including unsafe id, missing change, missing specs directory, malformed delta, added self, modified source/target self, and another eligible candidate. |
| Testability | The runner could silently accept unfinished steps. | `features_runner` uses `.fail_on_skipped()`; new step bodies will begin with `unimplemented!()` during RED and no empty body is allowed. |
| E2E fidelity | The capability is CLI-only, so browser or HTTP substitutes would be invalid. | Both e2e scenarios spawn the compiled CLI and assert terminal output; no repository-only assertion replaces the interface assertion. |
| Interaction coverage | Text and JSON consumers require different result contracts by explicit product decision. | The two inventory entries have one-to-one matrix rows and one e2e scenario each; stdin is intentionally regular-only. |
| Persistence risk | A failed delta parse after refresh could rewrite the permanent index. | Change resolution and complete delta collection precede permanent refresh; rejection scenarios snapshot and compare the index. |
| Arc42 | Durable docs previously described current-change exploration as future work. | The change-level marker and affected durable chapters now describe the PR-2 contract and its ADRs, risks, and quality evidence. |

## Verification

The hardening pass ran:

```text
givn lint --change spec-explore-change
```

It exited `2` with 15 intentional `@wip` findings and no parse or structural
errors. `git diff --check` passed. The scenarios remain `@wip` until their
step definitions and production implementation are completed.

DESIGN-REVIEW: PASS
