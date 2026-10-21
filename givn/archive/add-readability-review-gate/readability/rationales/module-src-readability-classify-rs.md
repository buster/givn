Unit: module:src/readability/classify.rs
Classification: method-length
Finding: method-length
Evidence: `decide` is 135 code lines with complexity 23; the precedence table and evidence rows are in `src/readability/classify.rs`.
Why this shape is right: the `classify` module owns the one ordered precedence table for classifications; keeping `decide` in one function makes the first-match order auditable in one place and is the responsibility of this module.
Alternatives considered: Extract each rule into its own function; rejected: the order would be scattered across helpers and the first-match contract would be harder to verify.
Owner: givn-maintainer--owner
