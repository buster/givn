Feature: grouped-merge-for-new-capabilities

  @e2e
  Scenario: Consumer archives a new capability and the merged spec lands in its declared use case
    Given  a givn project with verify.e2e_command unconfigured in givn/commands.yaml
    And  a completed change "grouped-new" whose proposal declares the capability NEW in "some-group" and whose delta lives under specs/some-group/
    And  the project declares a complete run declaration
    When  the user invokes `givn archive --change grouped-new`
    Then  the command exits 0
    And  the file "givn/specs/some-group/grouped-new.feature" exists
    And  the directory "givn/specs/grouped-new" does not exist
    And  the change directory has been moved to givn/archive/

  @e2e
  Scenario: Archive is blocked when a new capability has no declared routing decision
    Given  a givn project with verify.e2e_command unconfigured in givn/commands.yaml
    And  a completed change "flat-new" whose proposal records no routing decision and whose delta lives under specs/flat-new/
    And  the project declares a complete run declaration
    When  the user invokes `givn archive --change flat-new`
    Then  the command exits non-zero
    And  stderr contains "records no 'NEW in"
    And  the directory "givn/specs/flat-new" does not exist
    And  the change directory still exists under givn/changes/

  @e2e
  Scenario: Nothing is merged when one delta file of a change has no routing decision
    Given  a givn project with verify.e2e_command unconfigured in givn/commands.yaml
    And  a completed change "mixed-routing" with two deltas: one routed NEW in "mixed-group" and one unrouted
    And  the project declares a complete run declaration
    When  the user invokes `givn archive --change mixed-routing`
    Then  the command exits non-zero
    And  stderr contains "records no 'NEW in"
    And  the file "givn/specs/mixed-group/mixed-routing.feature" does not exist
    And  the change directory still exists under givn/changes/

  Scenario: Retiring a use case's last capability removes the use-case directory and its narrative
    Given  a givn project with a permanent use case "retire-home" whose only capability is "only-cap"
    And  a completed change "retire-last" whose delta retires capability "only-cap"
    And  the project declares a complete run declaration
    When  the user invokes `givn archive --change retire-last`
    Then  the command exits 0
    And  the directory "givn/specs/retire-home" does not exist
    And  the tombstone "givn/specs/.retired.yaml" records capability "only-cap"
    And  the change directory has been moved to givn/archive/
