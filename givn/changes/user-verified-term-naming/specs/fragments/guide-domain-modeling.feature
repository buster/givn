@givn.delta @guide-domain-modeling

Feature: Guide domain modeling

  @givn.added @wip
  Scenario: The challenge bullet hands the name choice to the user
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body no longer tells the agent to choose the name alone
    And the body describes the concept before proposing names
    And the body offers up to three candidate names
    And the body states what each candidate evokes and any collision it carries
    And the body accepts the user's own name

  @givn.added @wip
  Scenario: The challenge bullet shows a naming question with candidates and a collision
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body shows the naming question for "ShoppingList"
    And the body shows the colliding candidate "Basket"
    And the body asks which name the record should use
