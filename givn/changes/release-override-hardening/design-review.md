# Design Review: release-override-hardening

## Grilling (2026-08-31)

Fresh-context grilling subagent; findings across scope, the completeness net's
tech choices, missing scenarios, testability, E2E fidelity, interaction
coverage, risk, ADR qualification, arc42, and ubiquitous language. Key code
grounding: `GIVN_SPEC_` appears in exactly one src module today; the net's
single-file scan was true but unenforced; the honoured model-location and
HF_* channels share one accepted exception, not a "verdict-irrelevant" class.

## Decisions (product owner, one question at a time)

1. **The completeness net walks all of `src/`** (not retrieval.rs only) — the
   headline claim "mechanically complete" must be true as written; the walk
   is a few lines and covers channels read outside the retrieval module.
2. **Fail-safe is outcome-level with two cases**: primary model artifacts
   missing → the gate refuses (exit non-zero, never a silent pass);
   second-stage evidence missing → the E5 admission verdict stands (pinned by
   the permanent second-stage unavailability scenarios; an installed-build e2e
   for this case needs real E5 artifacts and is not runnable offline).
3. **README claim narrowed**: environment manipulation cannot substitute
   simulated output, disable an evidence stage, or silence a refusal — the
   model-location/mirror exception makes "cannot change a verdict" false.
4. **The net runs in the standard gate**: `cargo test --lib -- release_strict`
   added to the default and `--non-e2e` branches of `run-tests.sh` (seconds).

## Hardening applied

- design.md: EXEMPT_CHANNELS description corrected (threads + honoured model
  locations + test probe), mermaid label updated, `include_str!` replaced by
  the src-wide walk in the description, HF_* justification corrected to the
  accepted-exception class, REQUEST_REPRESENTATIVE wording made precise,
  GREEN-first documented with mutation evidence.
- src/spec/retrieval.rs: completeness test walks all `src/*.rs`; trust
  comments at the BGE/NLI model-dir sites.
- docs/arc42/08-crosscutting-concepts.md: exemption sentence names the
  download mirror; the fail-safe row split into primary (refusal) and
  second-stage (E5 verdict stands) cases.
- README.md: enforcement note narrowed to the honest guarantee.
- run-tests.sh: lib net added to default and `--non-e2e` branches.
- arc42.md: ch. 09 justification cites the predecessor's CANONICAL_ARTIFACT
  routing (not ADR-028, which owns hook/mandatory-check separability only).

## Mutation evidence (RED proof for the pinning scenarios)

1. Unregistered read probe (`spec_env("GIVN_SPEC_UNREGISTERED_PROBE")`) →
   completeness test FAILED ("neither in REQUEST_CHANNELS nor
   EXEMPT_CHANNELS"); reverted → green.
2. Sweep disabled (`report_ignored_release_overrides` early-return) →
   "An installed givn announces each refused request by name" failed (exit
   101, missing pair-budget announcement); reverted → green.
3. The review-gate-closed scenario pins the refusal observable already proven
   by the permanent release-strict scenarios (same then-binding).

## Lint

`givn lint --change release-override-hardening` clean (2 @wip removed after
implementation; both scenarios GREEN).

DESIGN-REVIEW: PASS
