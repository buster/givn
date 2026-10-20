# Review: apply-readability-independent-review

Status: implementation complete. The five requested long methods are
extracted, the syn adapter is split into a directory module with a syn-free
impl index, and the target registry keeps its justified shape. The gate
decisions are recorded. The branch also carried a stale validator test
fixture; fixing it required touching `src/readability/gate/rationale.rs`, so
its 107-line `validate_text` was extracted as well and is recorded as a sixth
fixed unit. The independent reviewer accepted the `targets` rationale.

README-IMPACT: none

## Fabrication audit

- No new step-definition file is introduced. The single delta scenario reuses
  the existing gate steps in `tests/steps/readability_review_gate_steps.rs`.
- No existing test, scenario, threshold, or gate rule is weakened.
- The source changes are behavior-preserving extractions; each extraction is
  covered by the existing unit tests and the existing gate scenarios.
- The stale fixture in `a_complete_rationale_passes_and_each_floor_fails`
  lacked the independent reviewer fields; adding them restores the test
  without weakening any assertion.
- The gate's base is the branch state before the change directory commit, so
  all seven findings are measured against their pre-change sizes.

## Coverage classification

1. Dead code: none introduced; every extracted helper is called by its former
   function.
2. Missing coverage: the existing unit and gate suites cover the extracted
   paths. The new scenario pins the invalid-rationale message so the applied
   rationale review names the offending file.
3. Hard to test: none beyond the independent reviewer's judgement of the
   remaining structural rationale for the target registry.

## Overlap dispositions

`givn lint --change apply-readability-independent-review` reports the delta
clean and lists the shape matches below. Each existing scenario pins a distinct
fixture or failure state; the added scenario pins the full invalid-rationale
message including the rationale path.

| Scenario A | Scenario B | Disposition | Rationale |
|---|---|---|---|
| A fixed marker for a unit still over its threshold fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A justified marker without a rationale fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A justified marker for a unit under its threshold fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale whose unit does not match the finding fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| Two units with the same rationale slug fail | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale whose classification does not match the finding fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale without an owner fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale from an unconfirmed owner fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A justification for "not my code" is rejected | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A justification for "out of scope" is rejected | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A boilerplate rationale is rejected | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| The review requires a readability verdict when a finding exists | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A fail marker fails the review even when every finding is justified | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A missing review record with a finding fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A touched production file that cannot be parsed fails closed | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A changed Rust file the analyzer skips fails closed | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A temporary rationale without a follow-up fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| Conflicting decisions for one unit fail | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| An unknown readability marker fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A symlink in place of a touched production file fails closed | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A change directory without a reachable commit fails closed | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A shallow clone fails closed | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| Dirty production paths fail closed before analysis | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale without a reviewer fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale reviewed by its author fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
| A rationale with a reject verdict fails | An incomplete rationale names the rationale path | boundary | The existing scenario pins a different state or failure; the added scenario pins the full invalid-rationale message with the rationale path. |
## Use case and Personas

`verify-change` owns `readability-review-gate`. The Developer, Authoring agent,
and Givn maintainer are review lenses, not Gherkin actors.

READABILITY: FIXED module:src/readability/classify.rs
READABILITY: FIXED module:src/readability/gate.rs
READABILITY: FIXED module:src/readability/gate/findings.rs
READABILITY: FIXED module:src/readability/report.rs
READABILITY: FIXED module:src/skills/render.rs
READABILITY: FIXED module:src/readability/gate/rationale.rs
READABILITY: FIXED module:src/readability/syn_adapter.rs
READABILITY: JUSTIFIED module:src/skills/targets.rs
READABILITY: PASS
