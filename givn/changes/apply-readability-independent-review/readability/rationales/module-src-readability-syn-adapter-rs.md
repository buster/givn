Unit: module:src/readability/syn_adapter.rs
Classification: multi-type-module
Finding: existing-grown
Evidence: 954 -> 968 code lines; the growth is the pending-impl and trait-relation phase split while the module keeps cross-file impl aggregation and the path-based unit ids in `src/readability/syn_adapter.rs`.
Why this shape is right: the `syn_adapter` module is the single `syn` boundary fixed by ADR-074; keeping parser handling in one adapter is what prevents parser types from leaking into metrics, classification, and the report.
Alternatives considered: Split the visitor and cross-file index into sibling modules; rejected: it would spread `syn` handling beyond the one adapter and weaken the ADR-074 boundary.
Owner: givn-maintainer--owner
Author: agent
