# arc42 Documentation Update: semantic-review

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a blocking semantic-review completeness goal, a fixed E5 boundary, and a user-facing change-local report retained as archive evidence. | Added FR-28, semantic-review integrity as a secondary quality goal, and stakeholder expectations for complete evidence and agent classification. |
| 2 | 02 architecture-constraints | Yes | Blocking semantic review now requires an explicit retrieval-enabled local build while feature-free builds fail clearly. | Added TC-17 and the dogfood/build requirement; default no-feature behavior remains an explicit non-PASS path. |
| 3 | 03 context-and-scope | Yes | `givn check review --change <id>` gains a dedicated report-producing interface and a change-local evidence boundary that is retained when the existing archive move runs. | Documented the dedicated entrypoint, candidate scopes, exact E5 boundary, agent ownership, no-feature failure, index boundary, and excluded classification/remediation/full-corpus audit. |
| 4 | 04 solution-strategy | Yes | The change adds an architectural strategy for the fixed `0.9500` candidate boundary, complete evidence hashes, strict report wire grammar, deterministic ordering, atomic report replacement, and non-mutation. | Added the semantic review gate strategy, exact report grammar and JSON serialization, permanent-only index recovery rule, no-feature behavior, and unchanged advisory review. |
| 5 | 05 building-block-view | Yes | Existing `cli::check` and `spec::retrieval` building blocks gain separate report-generation and classification-gate responsibilities, with a compiled-CLI acceptance boundary and no-feature path. | Updated the Mermaid decomposition, module responsibilities, candidate/evidence model, permanent-only index boundary, and semantic report additions. |
| 6 | 06 runtime-view | Yes | The blocking check-review sequence gains a dedicated E5 report gate between structural overlap and existing retrieval behavior, including strict wire parsing, complete evidence order, malformed-index recovery, atomic blocked output, no-feature failure, and archive evidence retention. | Added the gate order, Mermaid sequence, candidate/report lifecycle, deterministic ordering, invalidation, blocking, permanent-only index rebuild, non-mutation, and archive flow. |
| 7 | 07 deployment-view | No | The report is a project-local change-local Markdown file retained by the existing change-directory archive move; there is no new service, deployment target, runtime dependency, or distribution artifact. | |
| 8 | 08 crosscutting-concepts | Yes | Candidate selection, the `0.9500` E5 boundary, complete source-ordered evidence, strict report wire grammar, index recovery/permanence, classification vocabulary, no-feature exit behavior, atomic writes, archive retention, and non-mutation are cross-cutting contracts. | Added the semantic report concept, Mermaid flow, exact serialization and parser rules, dedicated entrypoint/error semantics, permanent-only rebuild boundary, and historical retention while preserving advisory retrieval. |
| 9 | 09 architecture-decisions | Yes | The change introduces trade-offs for a dedicated blocking report, fixed E5 candidate universe, full source-ordered evidence, exact wire grammar, full-evidence invalidation, strict prior parsing, permanent-only index recovery, no-feature failure, and retained historical evidence instead of terminal-only output or automatic edits. | Updated MADR ADR-046 and linked it to the existing strict similarity, retrieval, identity, index, and archive decisions. |
| 10 | 10 quality-requirements | Yes | The change introduces quality scenarios for exact boundary, candidate-id and field order, stable bytes, complete source-ordered evidence invalidation, strict prior grammar and atomic blocked output, malformed-index rebuild, dedicated entrypoint/no-feature behavior, permanent-only index structure, non-mutation, archive retention, and real CLI acceptance. | Added and expanded semantic-review quality scenarios covering Q-52 onward. |
| 11 | 11 risks-and-technical-debt | Yes | The report adds risks around E5 recall and threshold drift, incomplete source-ordered evidence invalidation, malformed prior reports and unknown fields, false no-feature passes, malformed-index contamination, stale scope assumptions, automatic-edit assumptions, and historical evidence loss. | Updated the semantic-review risks and mitigations with executable evidence for the fixed boundary, exact wire grammar, full evidence hash, parser, gate, index rebuild, no-feature, and archive contracts. |
| 12 | 12 glossary | Yes | The change introduces domain terms for the change-local retained report, exact wire grammar, candidate universe/order, complete source-ordered endpoint evidence, evidence hash, report/parser versions, dedicated gate, permanent-only index recovery, and no-feature behavior. | Added and refined the semantic-review vocabulary, serialization, index-rebuild, and historical-retention terminology. |

## Notes

The durable docs distinguish the new blocking `givn check review --change <id>`
report entrypoint from unchanged advisory `givn spec review --change <id>`
behavior and the existing structural overlap, coverage, permanent-index, and
similarity contracts. The strict similarity gate requires
`spec-similarity-gate`; compiling `spec-retrieval` alone leaves the old strict
similarity gate advisory, while the new semantic report gate is blocking. The
report uses the existing E5 boundary `0.9500` and
deterministic Fingerprint facts only; it excludes BGE/NLI, removed scenarios,
and permanent-permanent pairs. Complete source-ordered endpoint evidence and
fixed-format evidence hashes control classification carry-forward. The strict
report grammar fixes headers, field order, compact JSON serialization, `null`
absence, the final newline, and the allowed hash/score formats; unknown fields
are rejected. A malformed or incompatible permanent index is treated as stale,
rebuilt from permanent specifications, and followed by semantic review; the
rebuilt index contains no `Delta` state. A strict prior parser discards invalid
state, and a complete report is atomically replaced before a blocked result.
Every resolved classification requires an agent rationale. A candidate that
appears unrelated but remains above the E5 boundary stays unresolved until its
domain vocabulary and invariant are sharpened, or it is resolved as a duplicate,
variant, or explicit valid boundary.
Without `spec-retrieval`, check review returns the existing unavailable error
and creates no report. Givn never edits feature files, permanent specs, or
review dispositions; the permanent index may refresh but remains structurally
permanent-only. The generated change-local report is retained as historical
evidence under `givn/archive/<id>/` by the existing archive move. The regular
scenarios cover the exact `0.9500` boundary, candidate-id and field order,
Rule-nested and Modified identity handling, complete source-ordered evidence,
empty/removed input, strict parser and unknown-field rejection, full-evidence
invalidation including stale version metadata, malformed-index recovery,
deterministic and equal-score ordering, archive retention, no-feature/model
errors, and complete non-mutation. Exactly one CLI interaction inventory entry
and one `@e2e` scenario use the compiled CLI subprocess. The dogfood install
enables `spec-retrieval` before `givn skills sync`; a feature-free build is
tested separately for explicit unavailability.

---

## Status

STATUS: DONE
