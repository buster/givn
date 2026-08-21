# Design Review: semantic-reranker-budget

## Fresh-context Findings

The fresh review identified the following blockers before implementation:

- The unset-setting behavior contradicted the default. The contract is now
  explicit: unset means `2048`; explicit empty, zero, negative, malformed,
  overflowing, or non-UTF-8 values fail with the setting named.
- The Arc42 addon is enabled. `arc42.md` now contains all 12 chapter decisions,
  and the affected durable chapters include the ordered-budget ADR, quality
  requirements, risks, and glossary terms.
- Parity and approval depend on the pair budget. The design now includes
  `rust-production-v2`, active-budget fingerprints, matching benchmark runtime
  configuration, and the affected approval fixtures/tooling.
- The old pair-cap refusal scenarios are removed. Their retained 512/513
  scenarios continue to describe the separate model-token boundary, not the
  operational pair budget.
- The design now distinguishes budget-omitted `*_UNSCORED` candidates from
  selected token-cap candidates and model-filtered candidates.
- The new executable scenarios cover the default, descending E5 selection for
  BGE and NLI, independent combined selection, invalid values, and the real
  CLI boundary. The implementation applies the same selection contract to
  Explore, advisory review, blocking review, and duplicate audit.

## Scope And Architecture Decisions

The scope matches the proposal: replace all-or-nothing second-stage overflow
with a configurable E5-ranked work budget. E5 threshold, complete compact
serialization, deterministic exclusions, BGE/NLI thresholds, and 512-token
preflight remain unchanged. The CLI remains the real E2E interface; there is
one normalized inventory action and one matching E2E scenario.

The implementation uses one fallible setting parser and deterministic ordered
prefix selectors rather than changing model APIs or adding a new dependency.
The existing test-only stub seams remain isolated from the production local
artifact paths.

## Arc42 Cross-check

`arc42.md` independently assesses all 12 chapters. Chapter 7 is explicitly
marked unaffected because the setting changes runtime selection and reporting,
not packaging or deployment topology. Chapters 1, 2, 3, 4, 5, 6, 8, 9, 10,
11, and 12 are updated consistently with the design. ADR-063 records the
tradeoff between bounded CPU work and an explicit lower-ranked recall tail;
risks R-74 and R-75 record its operational consequences.

## Verification

The post-hardening structural check reported only the seven expected `@wip`
scenarios in the active delta and no parse, duplicate-title, or inventory
finding:

```text
givn lint --change semantic-reranker-budget
givn lint: 3 file(s) checked, 7 finding(s)
```

The repository Cucumber runner uses `.fail_on_skipped()` in
`tests/features_runner.rs`. The active-change targeted command is documented in
`design.md`; the configured full and E2E commands remain `./run-tests.sh` and
`./run-tests.sh --e2e`.

DESIGN-REVIEW: PASS
