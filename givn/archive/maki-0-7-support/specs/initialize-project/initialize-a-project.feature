@givn.delta @initialize-a-project
Feature: Initialize a project

  @givn.added
  Scenario: The generated Maki permission manifest names the session read
    Given a project root without a givn/ directory
    When the consumer runs `givn init --agents maki`
    Then the project's ".maki/plugin.toml" grants "session_read" and revokes "session_write", "session_settings", and "input"

  @givn.added
  Scenario: Synchronizing an existing Maki manifest adds only the session read grant
    Given a project with a Maki permission manifest that lacks the session read grant
    When the user runs `givn skills sync`
    Then the project's ".maki/plugin.toml" grants "session_read"
    And the project's ".maki/plugin.toml" keeps the permission keys the user wrote
    And the project's ".maki/plugin.toml" adds no revocation
