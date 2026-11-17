# Instruction: steps-java

`cucumber-jvm` mechanics. The universal rules stay in `givn instructions steps`.

- Step definition file: `src/test/java/.../steps/<Capability>Steps.java`, one per capability.
- Binding form: `@Given(`, `@When(`, `@Then(`, `@And(`, `@But(`, or `@Step(`.
- Unimplemented body: `throw new io.cucumber.java.PendingException("TODO");`.
- Strict mode (CLI): `--strict`.
- Strict mode (JUnit Platform Suite): no native flag; never leave a step body empty and name a plugin or listener that fails the build on UNDEFINED or PENDING.
- Single scenario: `mvn test -Dcucumber.filter.name="<title>"`.
