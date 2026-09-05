@givn.delta @grouped-merge-for-new-capabilities

Feature: grouped-merge-for-new-capabilities
  The archive merge honors a new capability's declared group.

  @givn.added @e2e @wip
  Scenario: Consumer archives a new capability and the merged spec lands in its declared group
    Given a givn project with verify.e2e_command unconfigured in givn/config.yaml
    And a completed change "grouped-new" whose proposal declares the capability NEW in "some-group" and whose delta lives under specs/some-group/
    And the project declares a complete run declaration
    When the user invokes `givn archive --change grouped-new`
    Then the command exits 0
    And the file "givn/specs/some-group/grouped-new.feature" exists
    And the directory "givn/specs/grouped-new" does not exist
    And the change directory has been moved to givn/archive/

  @givn.added @wip
  Scenario: A new capability without a declared group keeps the flat merge layout
    Given a givn project with verify.e2e_command unconfigured in givn/config.yaml
    And a completed change "flat-new" whose proposal records no routing decision and whose delta lives under specs/flat-new/
    And the project declares a complete run declaration
    When the user invokes `givn archive --change flat-new`
    Then the command exits 0
    And the file "givn/specs/flat-new/flat-new.feature" exists
    And the change directory has been moved to givn/archive/
