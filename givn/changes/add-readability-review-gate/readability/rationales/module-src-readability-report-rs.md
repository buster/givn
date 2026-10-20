Unit: module:src/readability/report.rs
Classification: method-length
Finding: method-length
Evidence: `render_json` is 134 code lines; the schema is rendered from `src/readability/report.rs`.
Why this shape is right: the `report` module owns the one JSON document; keeping the keys in one renderer makes the schema shape and deterministic ordering visible in one place.
Alternatives considered: Split the JSON renderer by section; rejected: the sections share one classified model and one ordering rule, and splitting would hide the schema contract.
Owner: givn-maintainer--owner
