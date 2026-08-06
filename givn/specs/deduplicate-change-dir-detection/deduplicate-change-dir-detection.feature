Feature: Deduplicate Change Directory Detection

  Scenario: Integrity hook resolves the change directory from a relative features path inside givn/changes
    Given  a project root with givn/ initialised
    And  a change "detect-demo" exists with artifact "proposal.md"
    And  a change "detect-demo" exists with specs under "specs/"
    When  the user runs `givn check review --change detect-demo`
    Then  the command exits non-zero

  Scenario: Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
    Given  a project root with givn/ initialised
    And  the givn/config.yaml has an empty e2e_command
    And  a change "e2e-detect" exists with design.md containing a valid Step Definitions table
    And  a change "e2e-detect" exists with tasks.md where every COMMIT hash is a valid 40-character hex string
    And  a change "e2e-detect" exists with specs under "specs/" containing zero @e2e scenarios
    When  the user runs `givn check review --change e2e-detect`
    Then  the command exits 0

  @e2e
  Scenario: Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected
    Given  a givn project with a change "detect-int" that has design.md with a valid Step Definitions table and tasks.md where every COMMIT hash is a valid 40-character hex string
    When  the user invokes `givn check review --change detect-int`
    Then  the command succeeds
    And  stdout confirms integrity checks passed

  @e2e
  Scenario: Integrity check detects a placeholder COMMIT hash in tasks.md and fails
    Given  a givn project with a change "bad-hash" that has design.md with a valid Step Definitions table and tasks.md containing a placeholder COMMIT hash "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    When  the user invokes `givn check review --change bad-hash`
    Then  the command exits non-zero
    And  stderr reports the placeholder hash as invalid
