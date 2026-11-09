@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.added
  Scenario: The communication block binds documentation to named artifacts
    Given  an initialized addon-free project
    When  the user runs `givn addons enable communication_policy`
    Then  the managed communication block requires tables for command, flag, and key mappings
    And  the managed communication block sends fallback and error-string rules to specs and help
    And  the managed communication block keeps quick start free of validation rules and exit codes
    And  the managed communication block names no documentation file or README line budget
    And  the managed communication block is at most 300 words
