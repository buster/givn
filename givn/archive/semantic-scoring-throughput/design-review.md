# Design Review: semantic-scoring-throughput

## Phase 1: Grilling

A fresh-context grilling subagent walked all required branches against the
proposal, delta spec, design, arc42 assessment, ADR register, precedent
change, and `src/spec/retrieval.rs`. Outcomes:

- **Scope (A)**: spec matches proposal exactly; nothing extra, nothing
  missing. Caveat: the grouped-BGE scenario title overclaimed executable
  coverage.
- **Tech choices (B)**: stack valid (`std::thread::available_parallelism`,
  `BTreeMap` grouping via `SimilarityEndpoint: Ord`, pinned `fastembed`
  5.17.2 / `ort` 2.0.0-rc.12). Design's thread-site line references were
  stale; corrected to `retrieval.rs:8808` (E5), `:5151` (BGE), `:8040`
  (NLI).
- **Missing scenarios (C)**: malformed/zero/overflow/negative/whitespace
  collapse into the same parse-failure branch already covered; no missing
  error path. Brownian risk: the resolver must be **eager at
  `review_enabled` entry** so rejection fires before model work under the
  stub fixtures (the model sites are never reached on stub backends).
- **Testability (D)**: scenarios 1–5, 7 fail in RED on concrete observables
  (exit codes, `threads N`, stderr naming the setting, absence of status
  lines). Scenario 6 fails in RED on stdout disparity, but cannot exercise
  the grouped rerank path (stub keeps per-pair loops, and one-pair groups
  collapse to one call).
- **E2E fidelity (E)**: both scenarios were real-subprocess CLI bindings
  with env set and stdout/stderr/exit primary assertions.
- **Interaction coverage (F)**: the invalid-budget e2e was an error case of
  the configured-budget action — a separate inventory entry/@e2e violated
  the normalization policy.
- **Risk (G)**: unguarded grouped batching and a parity-sync claim that
  mismatched the code.
- **ADR qualification (H)**: ADR-064 independently re-derived as QUALIFIED /
  `NEW_ADR` (all five mandatory gates pass, existing-ADR search genuine;
  ADR-063 covers pair-count work, a distinct boundary). No missing or
  spurious ADR.
- **Arc42 (I)**: independent 12-row re-derivation matched `arc42.md`
  (Yes = 01, 08, 09, 11). All Yes chapters real and design-consistent; all
  12 chapter files present with substantive content; no ASCII-art diagrams.
  Chapter 09 has the qualified candidate's MADR; chapter 11 reflects its
  consequences.

## User decisions

1. Grouped-BGE verification: **rename only, no added real-model task**.
2. Tool parity: **defer Python tool sync** — the Python benchmark helpers
   are legacy and unused; the Rust code is the authoritative surface.

## Phase 2: Hardening

Applied:

- Spec: renamed scenario 6 to "BGE evidence identity is unchanged across
  thread settings" (thread-invariance guard, no grouped-batch overclaim);
  normalized the User Interaction Inventory to one entry; removed the
  redundant invalid-budget `@e2e` scenario (error case covered by the
  malformed/zero regular scenarios); kept exactly one `@e2e`.
- Design: corrected the three stale code-site references; rewrote "Parity
  sync" to state that no fingerprint/attestation/fixture change occurs (the
  fingerprinted descriptor has no thread field) and that Python tooling is
  legacy/unused and out of scope; removed Python tooling from Architecture
  Impact; pinned the embedder status line (`E5 status: E5_ENABLED,
  threads N`) and the append-only `, threads N` constraint on the BGE/NLI
  status lines so sibling `starts_with`/`contains` assertions stay valid;
  updated the Decisions bullet on grouped-batching verification.
- Arc42: no edits required — artifacts contained no Python/fingerprint
  claims and remain consistent with the decisions.

`givn lint --change semantic-scoring-throughput` exits `2` (expected WIP
status; 7 scenarios pending implementation in the next artifact).

## Notes

- All gated scenarios remain `@wip` until the implement step defines and
  greens them. They are not removed during hardening.
- The single `@e2e` scenario exercises the real compiled CLI with
  `GIVN_SPEC_THREADS=3` and asserts the stage status lines' thread count.

## Post-Review Scope Revision (user-directed, applied during implementation)

The design was narrowed during implementation by user decision, which does not
undo this review's core finding (PASS on the shared-budget decision):

- Status-line/thread-count reporting (`threads N`, embedder status line) was
  dropped as untestable: thread counts are not observable through the stub
  backends, and probing the real ONNX models is not viable. The scenarios
  asserting it were removed (zero scenarios remain).
- Grouped BGE batching was dropped; the rerank path changes only its thread
  count. NLI batched inference stays deferred per the decision section.
- The retained design — one `GIVN_SPEC_THREADS` setting defaulting to
  `available_parallelism()`, by-name validation (eager on the review path),
  applied at the three real-model sites, no fingerprint/attestation change,
  Python tooling out of scope — is exactly the part this review validated as
  low-risk and correct.

DESIGN-REVIEW: PASS
