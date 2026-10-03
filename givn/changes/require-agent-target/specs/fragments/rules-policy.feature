@givn.delta @rules-policy

Feature: Rules policy

  @givn.modified @wip
  Scenario: A fresh project resolves the rules policy to local-only
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  the command exits 0
    And  the file "givn/config.yaml" contains "rules: local-only"
    And  stdout contains "rules: local-only"
