Unit: module:src/skills/render.rs
Classification: method-length
Finding: method-length
Evidence: the template renderer is 116 code lines with complexity 8; `src/skills/render.rs`.
Why this shape is right: the `render` module owns directive parsing and rendering in one pass; the conditional and variable state share the same scanner and line-removal rule.
Alternatives considered: Extract conditional and variable passes; rejected: the two passes share the same directive scanner and splitting would duplicate the line-removal rule.
Owner: givn-maintainer--owner
