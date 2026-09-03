# Design Review: semantic-remediation-log

## Scope

The proposal, feature delta, design, arc42 marker, and ADR-068 describe one
slice: a change-local append-only remediation log beside the semantic report,
maintained by blocking `givn check review`, gate-validated against
`semantic-remediation:`-stamped commits, and retained by the archive. No new
CLI command, no report-grammar change, no automatic remediation, no evaluation
aggregation tooling.

## Grilling Results

A fresh-context grilling pass (independent subagent over all planning artifacts
plus `src/spec/retrieval.rs`, the test harness, `run-tests.sh`, and the arc42
chapters) returned ten ranked questions and a branch verdict table. All ten
questions have technically forced answers; none reopens a product decision.

| Branch | Result |
|---|---|
| Scope | FINDING → resolved. "Commits are reachable" and "complete entry" had no dedicated scenarios; unreachable-commit and pre-log backward-compatibility scenarios added. |
| Technology choices | PASS. Separate `src/spec/remediation_log.rs` for a 10.4k-line retrieval.rs; no new dependencies; report-schema extension and Git-history mining fail the spec's determinism and findability requirements. |
| Missing scenarios | FINDING → resolved. Seven scenarios added (pass-rerun dedup, archive-gate silence via log-unchanged assertion, after-field evidence matching, unreachable commit, unborn branch, BGE/NLI non-null evidence, multi-candidate stamp line, verbatim blank-lines fixture). |
| Testability | FINDING → resolved. The delta file only runs in gate-scoped E5-only groups, so BGE/NLI assertions would be vacuous; one `@rulebook`-tagged scenario now exercises non-null evidence states in the rulebook group. Duration steps must not assert `> 0`. |
| E2E fidelity | PASS with one hardening note. Two inventory entries ↔ two `@e2e` scenarios ↔ two matrix rows; real CLI subprocess driving; the e2e group is E5-only, so stage-duration assertions are null-tolerant there. |
| Interaction coverage | PASS. Inventory comment, `@e2e` scenarios, and design matrix are mutually consistent. |
| Risk | PASS after hardening. Top risks addressed: gate stranding of in-flight changes (backward-compatibility rule), fingerprint dedup target after PASS (compare against last run record of any kind), 0 ms legality (null only for non-executed stages), multiline verbatim blocks (4-space indent rule, block ends at deindented field/header), 16-hex commit grammar (full 40-hex SHAs). |
| ADR qualification | PASS. ADR-068 is NEW_ADR, not an amendment of ADR-046 (the regenerated-report contract is untouched); completeness wording aligned with chapter 8; anchors/unborn-branch/0-ms semantics recorded in the ADR. |
| Arc42 | FINDING → resolved. My inserted section duplicated the existing 8.5h number — renumbered to 8.5j; row 11 corrected to Yes with debt row D-11 (log reader joins the D-10 parser-debt class). |
| Ubiquitous language | FINDING → resolved. "Git anchor" was used without a glossary entry — added; the review-run-record entry now states the null/0 rule; design.md's contradiction about who fills classification/rationale is fixed (agent copies from the report; report remains authoritative). |

## Hardening Decisions

Adopted from the grilling pass (all technically forced, none a product fork):

1. **Log writes and validation are strictly inside `write_report = true`** —
   the archive similarity gate never reads or grows the log; the archive
   scenario additionally asserts the active log is byte-unchanged.
2. **Backward compatibility** — the gate demands entries only for candidates
   the log knows; a verified remediation predating the log passes without one;
   a missing log file behaves like an empty log.
3. **Fingerprint dedup** compares against the last run record of any kind, so a
   repeated passing rerun appends nothing.
4. **After fields land on the entry matching the verified evidence hashes**
   (the verifier's own matching), never first-entry-by-id.
5. **Full 40-character SHA anchors**; 16-hex stays reserved for candidate ids
   and the stamp line; unborn branch ≡ no Git history.
6. **Duration rule** restated: `null` only when a stage did not execute;
   measured 0 (including cache-only `Some(0)`) is valid.
7. **Verbatim block grammar** survives blank lines: interior blank lines render
   as the 4-space indent; the block ends at a deindented field or header; the
   wire-grammar scenario uses a blank-line + Examples-table fixture.
8. **classification/rationale** are agent-copied from the report into the
   entry; the report remains authoritative, no log-vs-report cross-check.
9. **Git helpers** are thin private fns in `remediation_log.rs`, not a
   visibility widening of migration.rs.
10. **Multi-candidate commits** carry one stamp line per candidate; the gate
    matches per entry.

## Arc42 Assessment

Independent 12-row walk agrees with the change marker after two corrections:
row 11 is Yes (D-11), and the new crosscutting section is 8.5j (8.5h already
exists). Chapters 5, 6, 8, 9, 11, 12 updated; ADR-068 registered as proposed.

## Sign-off

All questions resolved; artifacts hardened; lint clean of new severity-1
findings.

DESIGN-REVIEW: PASS
