# Design Review: template-engine-conditional-rendering

## Grilling questions and resolutions

1. **Permanent regression: arc42-review-alignment asserts the N/A string.**
   The permanent e2e "Generated review guidance stays aligned after skill sync"
   asserts the generated review guidance contains `N/A when the arc42 addon is
   disabled`. Removing the N/A prologue breaks it.
   Resolved: this change modifies that permanent scenario — the N/A-line
   assertion is dropped; the remaining assertions ("Arc42 implementation
   conformance", design.md, tasks.md) still hold with the arc42-enabled
   default fixture. Verified by codebase search that no other permanent
   scenario or step asserts a string the template table touches (the remaining
   matches read raw template assets, whose text is retained inside
   conditionals, or CLI stdout).

2. **Invariant scope: the proposal claims "no reference to an un-enabled
   addon" universally, but the first plan gated only a few sections.**
   Resolved by user decision: **universal removal**. Every addon-owned section,
   row, and bullet is gated; incidental pointers are reworded. The complete
   per-asset edit table is in design.md. The residual machine check (spec
   scenario "No generated skill or command mentions a disabled addon") makes
   the invariant executable, not prose.

3. **Stale-file removal in two places (design contradiction).**
   Resolved: one removal path only — inside `skills sync` (self-reconciliation
   of managed files no longer valid for the current config). `addons.rs` keeps
   its existing skill/command-file removal for skill/command addons, which is
   now idempotent with sync; no special-case removal on `disable arc42`.

4. **Dangling "Otherwise, …" / "If it is enabled, …" prose when only the
   disabled-branch prologue is cut.**
   Resolved: the edit table explicitly removes both clauses of each prologue
   and drops the "(conditional)" heading suffix, so the kept branch reads as
   clean, unconditional text.

5. **Fail-closed directives are observable but had no scenario.**
   Resolved: two scenarios added — unknown addon name in a project-override
   skill template and an unbalanced conditional in a command template; both
   assert `skills sync` fails non-zero and names the template. Balance/
   nesting remain unit-tested in `render.rs`.

6. **The coupling cleanup (disable arc42 → previously generated
   domain-modeling skill disappears) was untested.**
   Resolved: a regular scenario added (a variant of the disable action, so not
   a separate e2e): "Disabling the Arc42 addon removes an existing generated
   domain-modeling skill."

7. **Unit/e2e duplication risk.** Non-e2e fixtures now set addon state by
   editing `givn/config.yaml` directly then syncing, so auto-regeneration is
   exercised only by the e2e Whens. The distinct per-file invariants are
   listed in design.md Black-Box-First.

8. **Marker strings must match real heading casing.**
   Resolved: spec markers corrected to the real template text
   ("Ubiquitous language conformance" — no hyphen; "Architecture
   documentation (arc42)" for the skill branch; "Architecture documentation
   (arc42) check" for the command grid).

9. **Inclusion-side scenarios cannot fail in RED.**
   Accepted (documented in design.md): current rendering includes everything,
   so presence-when-enabled scenarios are GREEN-only regression guards; the
   deletion-side scenarios drive RED.

10. **UL section requires both addons.** The review skill's UBL check writes
    into `docs/arc42/12-glossary.md`, so it must render only when
    `domain_modeling && arc42`. Resolved with nested conditionals (no grammar
    extension). Same for the design-review skill's UBL row.

11. **Multiple un-conditional arc42 references beyond the original table.**
    Resolved by the universal table: givn-tasks skill/command parentheticals,
    the design-review command input bullet, the sign-off N/A checkbox, the
    archive-skill coverage section, and the grill-with-docs source list are
    all gated or reworded.

12. **Unexpected sync multiplication in `givn init`.** `init` enables addons
    via `run_enable` per addon; if every enable re-synced, init would sync ~8x.
    Resolved: `run_enable`/`run_disable` split into a scaffolding body plus a
    syncing caller. Standalone toggles sync once after scaffolding; `givn init`
    calls the scaffolding-only variants for each addon and keeps its single
    end-of-init sync (design.md Architecture impact).

13. **No ADR required.** All three decisions (engine choice, coupling,
    toggle-regeneration) are design decisions. Canonical documentation:
    engine → design.md Technology decisions + glossary "Conditional section"
    row; coupling → design.md + the `givn-domain-modeling` skill's
    "Requires the arc42 addon" line (kept); toggle-regeneration → design.md.

14. **arc42 chapter re-derivation.** Independent 12-row walk agrees with
    `arc42.md`: Yes on 08 crosscutting-concepts and 12 glossary (rows added),
    No elsewhere. Borderline rows documented: 04 solution-strategy (the engine
    decision lives in design.md as a tech choice, no strategy change) and 11
    risks (change-level mitigations, no architectural debt).

## E2E fidelity check

Inventory (3 entries: `skills sync`, `addons enable`, `addons disable`) maps
1:1 to the design's Interaction Coverage Matrix; each e2e scenario drives the
real CLI binary via `run_givn` subprocess and asserts on generated files —
the observable boundary of a CLI generator. No @e2e tags removed.

## Sign-off

All fourteen questions resolved; design.md and specs updated accordingly.

DESIGN-REVIEW: PASS
