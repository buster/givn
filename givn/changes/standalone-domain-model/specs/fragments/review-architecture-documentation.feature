@givn.delta @review-architecture-documentation

Feature: Review architecture documentation

  @givn.added
  Scenario: Review guidance checks vocabulary and structure at their owners
    Given  a project root with givn/ initialised
    And  an existing change named "review-domain-owners"
    Then  the served review instruction contains "docs/glossary.md"
    And  the served review instruction contains "docs/domain-model.md"
    And  the served review instruction contains "structural impact"

  @givn.added
  Scenario: Design-review guidance checks vocabulary and structure at their owners
    Given  a project root with givn/ initialised
    And  an existing change named "design-review-domain-owners"
    Then  the served design-review instruction contains "docs/glossary.md"
    And  the served design-review instruction contains "docs/domain-model.md"
    And  the served design-review instruction contains "structural impact"

  @givn.added
  Scenario: Review does not accept a no-op model edit as maintenance evidence
    Given  a project root with givn/ initialised
    And  an existing change named "review-no-op-model"
    Then  the served review instruction contains "no-op document edit"
    And  the served review instruction contains "not evidence"
