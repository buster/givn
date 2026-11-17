# Repair notes

## Context

The initial clean-room builder produced the binary recorded in
`builder-notes.md`: 12 release builds, `givn 0.9.10`, SHA-256
`ccf0332307a0d67acd96d5dfc68a68c0bbf3ce24ea2c00401b9063c820caccb3`,
frozen run 1 at 120 total, 35 passed, 85 failed, 0 skipped.

The operator directed a bounded implementation-defect repair round after the
first triage. The clean-room builder is not the repair actor: the builder's
queued repair prompt applied a partial source batch before it was canceled, and
the trial executor completed the repair. The executor read the frozen failure
log and the product's use-case/step assertions to fix implementation defects.
This is the design's D2 `implementation defect` route; the actor is recorded as
an operator-directed deviation from "return to the builder" because the builder
was canceled. The initial rebuild's clean-room claim is unchanged: the fresh
builder authored the first binary from the Specification and the disclosed
inputs without reading the checkout or the frozen `e2e/` package.

## Builds and frozen results

| Round | Binary SHA-256 | Size | Total | Passed | Failed | Skipped |
|---|---|---|---|---|---|---|
| Initial builder, 12 builds | `ccf0332307a0d67acd96d5dfc68a68c0bbf3ce24ea2c00401b9063c820caccb3` | 3,266,384 | 120 | 35 | 85 | 0 |
| Repair build 1, 15:06 | `8ba8f36dc13ecb5fcc0850a3ffe032525f3396f1a93778ecb2134cd1fa0148f8` | 3,291,976 | 120 | 49 | 71 | 0 |
| Repair build 2, 15:16 | `f7e808d4b65f1ae8ccb43c19b66719d3e127470089fe204665644e40f9e6738f` | 3,299,216 | 120 | 51 | 69 | 0 |

Attempt 1 evidence: `e2e-result-attempt1.json`, `e2e-run-attempt1.log`,
`e2e-failures-attempt1.md`. Repair build 1 evidence:
`e2e-result-attempt2.json`, `e2e-run-attempt2.log`,
`rebuilt-version-attempt2.txt`. The final attempt overwrites
`e2e-result.json` and `rebuilt-version.txt` and keeps its own log.

## Fixes applied

Repair build 1:

- `src/init.rs`: default `--agents` selection and preservation of existing
  target lists; normal-case reports; `refreshed commit attribution plumbing`.
- `src/archive.rs`: receipt `tested_revision`, `candidate_revision`, and
  `merge_completed`; archive scope and README/coverage refresh.
- `src/check.rs`: `receipt_is_green` for `status: passed`, `head` acceptance,
  the receipt `status`/`head` fields, placeholder-hash handling.
- `src/instructions.rs`: `coverage` id served from the existing `COVERAGE`
  corpus; review/design/Arc42/extension phrases.
- `src/status.rs`, `src/spec.rs`, `src/history.rs`, `src/outline.rs`,
  `src/config.rs`, `src/attribution.rs`, `src/scm.rs`, `src/skills.rs`: status
  wording, spec-route evidence, JSON array output, `god-type`, README markers,
  attribution reports, revision reports, skill destinations.

Repair build 2:

- `src/init.rs`: `givn eject <artifact>` writes
  `givn/artifacts/<artifact>/<artifact>.md` for the template facet.
- `src/config.rs`: README description, license, and run markers match the
  product contract.
- `src/check.rs`: receipt revision accepts `head`; `status: passed` accepted.
- `src/spec.rs`: the route evidence path uses the ranked capability's feature
  file.
- `src/instructions.rs`: the `coverage` instruction is served.

The repair did not modify the 90 copied Specification files. `givn ise
compare` still reports `Specification files: match` and `End-to-end tests:
match` after both builds.

## Residuals

The final residual differences and their one outcome each are in
`spec-gaps.md`. The `attribution_asset` exact-asset rows and the product-version
row remain `specification defect`; they are not repair targets. Remaining
implementation defects are recorded with the rebuilt file that is still wrong.
