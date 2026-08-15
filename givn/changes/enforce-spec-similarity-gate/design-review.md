# Design Review: enforce-spec-similarity-gate

## Grilling Outcome

| Branch | Finding | Resolution |
|---|---|---|
| Scope | The original advisory retrieval behavior was too weak for the requested dogfood gate. | Keep interactive `spec search` advisory, but make review and archive preparation fail for every pair at or above the binary-fixed `0.95` threshold. |
| Threshold integrity | A project or agent must not raise the threshold to make its own change pass. | Use a private binary constant. No config, CLI flag, environment override, or disposition suppresses a candidate. Future threshold changes require a new versioned code/plan change and fresh dogfood evidence. |
| Similarity scope | A cross-feature-only scan would leave same-file duplicates and confusing variants unprotected. | Scan all distinct permanent pairs. Report same-file versus cross-file scope, but apply the same gate. |
| Scoring | Passage/query role asymmetry and self-matches could make the gate ambiguous. | Compare passage-role vectors on both sides; exclude the same logical identity; canonicalize endpoints and emit each pair once. Interactive search keeps its query-role behavior. |
| Delta identity | A normal `@givn.modified` scenario would collide with its own old permanent version. | Resolve modified targets by capability plus unchanged title and exclude only that own predecessor from the delta-to-baseline scan. A title rewrite is encoded as removed old title plus added new title. |
| Projection | A removal must be able to resolve a true duplicate before archive without mutating the tree. | Project add/modify/remove operations in memory using capability tags and the same ordered target semantics as archive merge; audit the projected tree. |
| Remediation | A `KEEP-AS-BOUNDARY` note must not become a permanent exception. | `MERGE` instructs deletion of a true duplicate. `KEEP-AS-BOUNDARY` instructs title/step rewriting until the score is below `0.95`. The score remains the only gate condition. |
| Baseline | The current Givn tree may already contain violations. | Add an implementation task and evidence artifact for a production-algorithm baseline scan. Resolve every existing candidate before archiving this change; no pair is grandfathered. |
| Archive | A `REVIEW: PASS` marker or stale hook must not bypass the new gate. | Re-run the strict similarity review after archive hooks and before any permanent-tree write. Refresh the index only after successful merge gates and archive move. |
| Feature gating | The default binary must not acquire ONNX Runtime or fail ordinary no-feature workflows. | Compile strict review and post-archive refresh behind `spec-retrieval`; no-feature check/archive paths remain unchanged while direct retrieval commands remain explicitly unavailable. |
| Output | A human and an agent need enough information to resolve a candidate. | Print scope, both files, both titles, score, fixed threshold, and remediation to stdout; print a deterministic non-zero gate error to stderr. Report all candidates, not only five. |
| E2E fidelity | The real boundary is the CLI, not an in-process retrieval call. | Cucumber invokes the compiled `givn` subprocess in a temporary project. The inventory and matrix cover permanent audit, delta audit, and archive refresh. |
| Arc42 | The change supersedes the advisory-only architectural statements. | All affected chapters and ADR-041 were updated; `arc42.md` contains an explicit 12-row assessment and `STATUS: DONE`. |

## Testability Decision

Every new behavior has a black-box scenario with deterministic stub embeddings.
RED is available before the production changes: the old advisory path does not
fail, does not scan all pairs, does not project removals, and does not refresh
the post-archive index. The runner remains strict through
`.fail_on_skipped()`; all new step bodies must perform real filesystem and CLI
assertions.

The implementation tasks additionally record:

- the current permanent-tree candidate inventory using the same passage-role
  production algorithm;
- remediation of every existing candidate;
- a final zero-candidate scan at `0.95`;
- no-feature compilation and workflow checks;
- archive pre-gate, projected-removal, modified-self-exclusion, and post-move
  refresh evidence.

## Status

DESIGN-REVIEW: PASS
