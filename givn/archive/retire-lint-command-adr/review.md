# Review: retire-lint-command-adr

## Fabrication Audit (do this first)

**Step definition scan** — one step definition was added
(`the final register has 24 accepted decisions and no proposed decisions`),
mirroring the existing count-step pattern; no scenario step is a stub.

**Commit cross-check** — one implementation commit touches the register and
specs:

| Scope | Commit hash | Touches the repaired artifacts? |
|---|---|---|
| ADR retirement | `f062c23f` | yes (deleted record, ADR-019, README, delta narratives, step definition) |

**Promised-architecture cross-check** — the change promises a rationale move
and a register retirement: the verify-change narrative delta exists, the
record is deleted, and no production module changed beyond the test step.

**Strict-mode proof** — inherited unchanged from `tests/features_runner.rs`
(`.fail_on_skipped()`).

**Downgraded e2e scenario scan** — N/A: no `@e2e` scenario is added or
changed.

**E2e scope check** — N/A.

**Duplicate-implementation check** — N/A.

**Local environment check** — N/A.

## Use-Case And Persona Conformance

The lint naming rule moves into `verify-change` Rules. No Persona was
invented; `developer--primary-user` is the CLI surface's lens. The retired
record's rationale is preserved in the specification, not lost.

## Retirements

RETIRE: ADR-027 from the active register (deleted, never archived); the
lint naming contract lives in `givn/specs/verify-change/`.

## Arc42 implementation conformance

Row 9 is Yes with the register change; the other rows are No. Direct check:
the README count (24), the removed row, the relationship line, and ADR-019's
references match `design.md`; the historical ADR-019 status stays because
the register-history scenario asserts it. No chapter is placeholder text.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the archive receipt. The delta carries one modified
scenario and one narrative document; the new step definition mirrors the
existing count pattern.

## Deterministic gate dispositions

The modified scenario keeps its title, so no overlap findings arise; the
permanent alignment keeps the full suite green during the change.

## E2e coverage

N/A — no `@e2e` scenario is added or changed; the e2e scope passes 88/88.

## Deferrals

None.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching the repaired
      artifacts.
- [x] Every promised artifact exists.
- [x] Strict-mode proof present and passing (inherited).
- [x] Scenario execution evidence recorded: full suite 539 passed, e2e 88
      passed; the archive receipt carries the measured coverage.
- [x] Coverage classified from inspection; no new gap.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No `@wip` tags.
- [x] Canonical E2E policy read; no new E2E action.
- [x] Local run command unchanged.
- [x] `verify.e2e_command` isolation unchanged.
- [x] Implementation matches design.md.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
