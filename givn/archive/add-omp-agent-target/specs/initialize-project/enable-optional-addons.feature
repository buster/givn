@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.modified
  Scenario: givn addons disable removes generated command files
    Given a project root with givn/ initialised
    And givn addon "grill_me" is "true"
    And givn skills targets in givn/config.yaml equal "opencode, omp"
    And the command "grill-me" exists in .opencode/commands
    And the command "grill-me" exists in .omp/commands
    When the user runs `givn addons disable grill_me`
    Then the command exits 0
    And the file ".opencode/commands/grill-me.md" does not exist
    And the file ".omp/commands/grill-me.md" does not exist
