Unit: module:src/readability/classify.rs
Classification: method-length
Finding: method-length
Evidence: `decide` is 135 code lines with complexity 23; the precedence table and evidence rows are in `src/readability/classify.rs`.
Why this shape is right: `decide` is the single ordered precedence table for classifications; keeping it in one function makes the first-match order and evidence selection auditable in one place.
Alternatives considered: Extract each rule into its own function; rejected because the order would be scattered across helpers and the first-match contract would be harder to verify.
Owner: givn-maintainer--owner
