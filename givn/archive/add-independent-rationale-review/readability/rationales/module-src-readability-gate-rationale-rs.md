Unit: module:src/readability/gate/rationale.rs
Classification: method-length
Finding: method-length
Evidence: `validate_text` is 107 code lines with complexity 24; the field parsing and all rationale checks are in `src/readability/gate/rationale.rs`.
Why this shape is right: the `rationale` module owns one ordered validation floor; keeping the field parsing, banned-reason scan, evidence/responsibility/alternative checks, and the independent-review check in one function makes the failure order and the exact message contract auditable.
Alternatives considered: Split each check into its own helper; rejected: the checks share one parsed field map and one ordered error contract, and splitting would scatter the order and duplicate the field parsing.
Owner: givn-maintainer--owner
Author: agent
Reviewed-by: fresh-context-reviewer-2026-10-20
Review verdict: accept
