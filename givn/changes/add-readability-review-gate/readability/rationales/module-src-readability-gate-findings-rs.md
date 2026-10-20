Unit: module:src/readability/gate/findings.rs
Classification: method-length
Finding: new-module
Evidence: the new module is 434 code lines; the finding computation is 108 code lines; `src/readability/gate/findings.rs`.
Why this shape is right: the finding classifier owns the creation and ratchet table; one function keeps base/head reasons and decisions together.
Alternatives considered: Split each finding reason into a separate function; rejected because the shared base/head comparison and touched-unit expansion would be duplicated.
Owner: givn-maintainer--owner
