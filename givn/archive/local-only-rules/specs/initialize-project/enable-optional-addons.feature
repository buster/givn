@givn.delta @enable-optional-addons

Feature: Enable optional addons

  @givn.modified
  Scenario: givn skills sync excludes addon skills and commands when disabled
    Given  a project root with givn/ initialised
    And  givn addon "domain_modeling" is "false"
    And  givn addon "grill_me" is "false"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the directory ".opencode/skills/givn-domain-modeling" does not exist
    And  stdout does not contain "synced skill: givn-domain-modeling"
    And  stdout does not contain "synced command: /grill-me"
