Unit: module:src/skills/targets.rs
Classification: multi-type-module
Finding: existing-grown
Evidence: 672 -> 673 code lines; the growth is the one-line `readability` addon flag in `src/skills/targets.rs`.
Why this shape is right: the `targets` module owns the single list of agent targets and their managed paths; the addon flag belongs in that registry so the target list and ownership rules stay in one contract.
Alternatives considered: Split the registry by target; rejected: the shared path and ownership rules and the sync order are one contract and would be duplicated across files.
Owner: givn-maintainer--owner
