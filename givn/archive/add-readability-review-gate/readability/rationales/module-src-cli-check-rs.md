# Rationale: module:src/cli/check.rs

Unit: module:src/cli/check.rs
Classification: module
Finding: crossed-threshold
Evidence: 477 code lines at the change base (611 physical lines), 13 production methods, the largest 84 code lines at complexity 25; the growth is the readability gate call and its `GateOutcome` rendering in `src/cli/check.rs`, documented in docs/arc42/06-runtime-view.md section 6.20; the head count is refreshed from `givn readability report --json --path src/cli` at review time.
Why this shape is right: The module owns the review check's ordering and its single entry point: integrity, run declaration, overlap, receipt, and readability must run in that order in one invocation, so no caller can skip a mandatory member or see a readability verdict before a structural failure.
Alternatives considered: (a) Put the gate in a new `givn readability review` command. Rejected: a separate command lets `givn check review` pass without the gate and contradicts ADR-028's mandatory-check member. (b) Move `run_mandatory_checks` into `src/readability/gate.rs`. Rejected: it would make the readability module own integrity, run-declaration, overlap, and receipt checks that exist without the addon, inverting the dependency and changing the report-only configuration.
Owner: givn-maintainer--owner
