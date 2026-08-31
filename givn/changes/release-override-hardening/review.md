# Review: release-override-hardening

README-IMPACT: none

## Coverage gaps

| Gap | Classification | Reason |
|---|---|---|
| release-binary refusal inside debug-profile scenarios | legitimately hard to test | the debug binary honours fixture requests by design; the refusal is exercised by the @release-proof scenarios through the release binary (GIVN_RELEASE_BIN resolution or lazy `cargo build --release --bin givn`) |
| second-stage-missing fail-safe on the installed build | legitimately hard to test | requires real E5 artifacts in the release fixture cache; the mechanism (E5 admission verdict stands) is pinned by the permanent second-stage unavailability scenarios in the stub suites |

## Evidence

- Completeness net: src-wide `GIVN_SPEC_*` scan, both directions, both
  profiles; mutation-proven (unregistered read probe fails the net; sweep
  disabled fails the announcement scenario).
- Release-proof scenarios: pair-budget announcement + review-gate refusal on
  the release binary (isolated cache, fixture artifact server).
- Full suite exit 0 (549 scenarios); e2e exit 0 (104 scenarios);
  clippy `-D warnings` clean; fmt clean.
- Real-model review run: pair cache 0 reused, 9 computed — see the generated
  semantic review report.

## Ubiquitous language

Fixture request, installed build, development build, model-location request,
refusal, fail-safe — glossary terms reused verbatim across proposal, design,
specs, ch. 08 § 8.5a, and the README enforcement note.

REVIEW: PASS
