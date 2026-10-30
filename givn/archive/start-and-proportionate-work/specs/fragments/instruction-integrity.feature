@givn.delta @instruction-integrity

Feature: Instruction integrity

  @givn.modified @e2e
  Scenario: Author starts a session with the universal dialog obligation
    Given  an initialized Givn project without an active change
    When  the author requests the normal Givn startup instructions through the Givn CLI
    Then  the startup guidance requires loading the effective dialog policy before the context's first user-facing question
    And  the bootstrap owns that obligation independently of optional addons and named skills
    And  the policy is loaded once per context and reloaded if lost
    And  the startup guidance says to load givn-start for new or unclassified work
    And  the startup guidance names givn new as the direct alternative
    And  the terminal startup output recommends givn-start and the direct change command
    And  the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change
    Given  an initialized Givn project with an active change awaiting specifications
    When  the author requests the normal Givn startup instructions as JSON
    Then  the response preserves the active change identity and next required artifact
    And  the response includes the same universal effective-dialog loading obligation in its guidance
    And  the response does not embed a second complete dialog policy
    And  the response says the reported step applies when continuing that change
    And  the response says the reported step does not authorize unrelated work

  @givn.added
  Scenario: Machine-readable no-active startup guidance carries the start route
    Given  an initialized Givn project without an active change
    When  the author requests the normal Givn startup instructions as JSON
    Then  the response names givn-start as the next skill
    And  the response keeps givn new as the direct command
    And  the response presents the direct command as optional

  @givn.added
  Scenario: The active-change startup guidance qualifies its next step
    Given  an initialized Givn project with an active change awaiting specifications
    When  the author requests the normal Givn startup instructions through the Givn CLI
    Then  the startup guidance says the reported step applies when continuing that change
    And  the startup guidance says the reported step does not authorize unrelated work
