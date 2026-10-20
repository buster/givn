Unit: module:src/readability/syn_adapter.rs
Classification: multi-type-module
Finding: existing-grown
Evidence: 794 -> 954 code lines; the growth is cross-file impl aggregation and the path-based unit ids in `src/readability/syn_adapter.rs`.
Why this shape is right: the adapter is the single `syn` boundary fixed by ADR-074; keeping `syn` handling in one adapter is what prevents parser types from leaking into metrics, classification, and the report.
Alternatives considered: Split the visitor and cross-file index into sibling modules; rejected because it would spread `syn` handling beyond the one adapter and weaken the ADR-074 boundary.
Owner: givn-maintainer--owner
