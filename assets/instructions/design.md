# Instruction: design

Write the technical design for this change. This is the HOW layer — implementation
details are welcome and required here.

**Scope (what belongs in design):**
- Technology decisions: language, framework, libraries, patterns.
- Architecture impact: which modules/components are affected, what is new.
- Data model changes: schema, structs, database changes.
- Step definition locations: where Cucumber/Gherkin step definitions live.
- Test runner command: confirm the `verify.command` in givn/config.yaml.
- Any other technical decision the implementer needs before writing tasks.

**Rules:**
- Do not duplicate the scenario text from the spec. Reference scenarios by title.
- Do not change the WHAT — if you discover the proposal/spec is wrong, update
  those artifacts instead.
- Justify non-obvious decisions (why this approach over alternatives?).
