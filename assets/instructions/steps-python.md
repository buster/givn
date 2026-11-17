# Instruction: steps-python

`behave` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition file: `features/steps/<capability>_steps.py`, one per capability.
- Binding form: `@given(`, `@when(`, `@then(`, or `@step(`.
- Unimplemented body: `raise NotImplementedError("TODO")`.
- Strict mode: every new step body starts with `raise`; a bare `pass` reports as passed.
- Single scenario: `behave -n "<title>"`.
