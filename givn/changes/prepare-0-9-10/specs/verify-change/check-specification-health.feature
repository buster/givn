@givn.delta @check-specification-health
Feature: Check specification health

  @givn.modified @e2e @wip
  Scenario: All clean .feature files report a clean summary on stdout
    Given a project root with givn/ initialised
    And a structurally valid specification corpus with one clean feature
    When the user invokes `givn lint`
    Then the command exits 0
    And stdout contains a line "givn lint: 1 file(s) checked — clean"
    And the terminal transcript is captured for this scenario

  @givn.added @wip
  Scenario Outline: Explicit relationship namespaces disambiguate equal identifiers
    Given a structurally valid corpus with a use case named "support" and a fragment named "support"
    And another use case Includes "<target>"
    When the user runs `givn lint`
    Then the relationship resolves to the <owner> named "support"
    And no ambiguous relationship target finding is reported

    Examples:
      | target            | owner    |
      | usecase: support  | use case |
      | fragment: support | fragment |

  @givn.added @wip
  Scenario Outline: A relationship cannot resolve through the other namespace
    Given a structurally valid corpus containing only a <existing owner> named "support"
    And another use case Includes "<target>"
    When the user runs `givn lint`
    Then the command exits 1
    And the findings name the unresolved typed target "<target>"

    Examples:
      | existing owner | target            |
      | fragment       | usecase: support  |
      | use case       | fragment: support |

  @givn.added @wip
  Scenario: A fragment cannot be an extension target even when a use case has the same identifier
    Given a structurally valid corpus with a use case named "support" and a fragment named "support"
    And another use case Extends "fragment: support"
    When the user runs `givn lint`
    Then the command exits 1
    And the findings say that Extends cannot target a fragment

  @givn.added @wip
  Scenario Outline: Cycles follow use-case includes and extends relationships
    Given structurally valid use cases "first" and "second"
    And "first" <relationship> "usecase: second"
    And "second" Includes "usecase: first"
    When the user runs `givn lint`
    Then the command exits 1
    And a use-case cycle finding names "first" and "second"

    Examples:
      | relationship |
      | Includes     |
      | Extends      |

  @givn.added @wip
  Scenario: A fragment inclusion does not invent an edge to a same-named use case
    Given structurally valid use cases "first" and "support" and a fragment named "support"
    And "first" Includes "fragment: support"
    And the use case "support" Includes "usecase: first"
    When the user runs `givn lint`
    Then no use-case cycle finding is reported

  @givn.added @wip
  Scenario Outline: Interaction evidence cannot be credited from another capability
    Given a structurally valid <owner> declaring capabilities "place-order" and "cancel-order"
    And its interaction inventory maps "place-order" to "Consumer submits an order"
    And only "cancel-order" has an E2E scenario titled "Consumer submits an order"
    When the user runs `givn lint`
    Then the command exits 1
    And the findings report missing E2E evidence for "place-order" and "Consumer submits an order"
    And the scenario in "cancel-order" does not satisfy that interaction

    Examples:
      | owner    |
      | use case |
      | fragment |

  @givn.added @wip
  Scenario: Fragment interactions are checked in their declaring fragment
    Given a structurally valid fragment declaring capability "show-help"
    And its interaction inventory maps "show-help" to "Consumer requests help"
    And "show-help" has an E2E scenario titled "Consumer requests help"
    And two use cases include that fragment without copying its interaction inventory
    When the user runs `givn lint`
    Then no missing or unlisted interaction finding is reported for "Consumer requests help"

  @givn.added @wip
  Scenario Outline: A malformed Unicode use-case title produces a diagnostic instead of a panic
    Given an otherwise valid use-case document whose first heading is "<heading>"
    When the user runs `givn lint`
    Then the command exits 1
    And the findings identify the malformed use-case title and its document
    And stderr does not contain "panicked"

    Examples:
      | heading                  |
      | # Use Case: Étude         |
      | # Use case: 注文          |
      | # 使用例: 注文            |
