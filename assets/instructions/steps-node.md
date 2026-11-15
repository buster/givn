# Instruction: steps-node

`cucumber-js` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition file: `features/steps/<capability>.steps.js`, one per capability.
- Binding form: `.Given(`, `.When(`, `.Then(`, or `.Step(`.
- Unimplemented body: `return "pending";` or `throw new Error("TODO")`.
- Strict mode: `--strict`, pinned explicitly in `cucumber.js`.
- Single scenario: `cucumber-js --name "<title>"`.
