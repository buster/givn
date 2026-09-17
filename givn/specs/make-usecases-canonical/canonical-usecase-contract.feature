Feature: Canonical use-case contract

  @e2e
  Scenario: Use-case tree reports explicit use-case ownership
    Given  a spec discovery project with two use cases and a usecase.md narrative
    When  the user inspects the corpus with `givn spec tree`
    Then  the command exits 0
    And  stdout contains both use-case names
    And  stdout contains the use-case actor line
  Scenario: The durable use-case model publishes the canonical definition and rules
    Given  the durable use-case model document is inspected
    When  a maintainer reads the definition and the scoping rules
    Then  the document defines a use case as a contract between one primary actor and the system
    And  the document carries the promotion rule for a leaf with three or more actor goals
    And  the document carries the placement test for a capability
    And  the document carries the ten review rejection criteria
    And  the document carries the worked split of an observing mega-leaf
    And  the document carries the promotion of a conjunction-goal leaf into a summary and its split
    And  the worked observing case is tied to the mutation-containment and component-name criteria
    And  the worked conjunction case is tied to the goal-conjunction and oversized-leaf criteria
    And  the workflow document points at the use-case model
    And  the reference document points at the use-case model

  Scenario: The generated use-case document template carries the authoring guidance
    Given  the canonical use-case document template is inspected
    When  a maintainer reads the template guidance
    Then  the template names the primary-actor convention
    And  the template shows the extension format
    And  the template numbers the main-flow example
    And  the template carries the promotion note
    And  the template preserves the fixture placeholders
