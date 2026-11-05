@givn.delta @initialize-a-project
Feature: Initialize a project

  @givn.modified
  Scenario: The generated Maki permission manifest names the session read
    Given  a project root without a givn/ directory
    When  the consumer runs `givn init --agents maki`
    Then  the project's ".maki/plugin.toml" grants "session_read"
    And  the project's ".maki/plugin.toml" grants "session_write"
    And  the project's ".maki/plugin.toml" revokes "net"
    And  the project's ".maki/plugin.toml" revokes "env"
    And  the project's ".maki/plugin.toml" revokes "agent"
    And  the project's ".maki/plugin.toml" revokes "session_settings"
    And  the project's ".maki/plugin.toml" revokes "input"

  @givn.added
  Scenario: Initializing a Maki target installs the attribution integration
    Given  a project root without a givn/ directory
    When  the consumer runs `givn init --agents maki`
    Then  the file ".maki/lua/givn-attribution.lua" exists
    And  the file ".maki/init.lua" contains the attribution fence

  @givn.removed
  Scenario: Synchronizing an existing Maki manifest adds only the session read grant
    Given  the scenario is retired in favor of the reporter and attribution grant scenario

  @givn.added
  Scenario: Synchronizing an existing Maki manifest adds the reporter and attribution grants
    Given  a project with a Maki permission manifest that lacks the session read grant
    When  the user runs `givn skills sync`
    Then  the project's ".maki/plugin.toml" grants "session_read"
    And  the project's ".maki/plugin.toml" grants "session_write"
    And  the project's ".maki/plugin.toml" keeps the permission keys the user wrote
    And  the project's ".maki/plugin.toml" adds no revocation

  @givn.added
  Scenario: Synchronizing an unmodified generated Maki manifest grants attribution
    Given  a project with the previous generated Maki permission manifest
    When  the user runs `givn skills sync`
    Then  the project's ".maki/plugin.toml" grants "session_write"
    And  the file ".maki/lua/givn-attribution.lua" exists

  @givn.added
  Scenario: Synchronizing an edited Maki manifest preserves its attribution revocation and names the needed grant
    Given  a project with an edited Maki permission manifest that revokes the attribution grant
    When  the user runs `givn skills sync`
    Then  the project's ".maki/plugin.toml" revokes "session_write"
    And  stderr names ".maki/plugin.toml"
    And  stderr names "session_write"
