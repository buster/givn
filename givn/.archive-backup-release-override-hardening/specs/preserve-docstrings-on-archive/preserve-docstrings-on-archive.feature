Feature: Preserve Docstrings on Archive

  Scenario: A modified scenario keeps its step docstring after archiving
    Given  a project root with givn/ initialised
    And  a permanent spec for capability "modify-cap" contains a scenario named "Existing behaviour"
    And  a completed change "modify-docstring" whose delta modifies scenario "Existing behaviour" within capability "modify-cap" and gives a step the docstring:
      """
      {
        "purpose": "docstring-survives-modified-merge",
        "count": 3
      }
      """
    When  the user runs `givn archive --change modify-docstring`
    Then  the command exits 0
    And  the permanent spec file for "modify-cap" contains the text "docstring-survives-modified-merge"

  Scenario: An added scenario keeps its step docstring after archiving
    Given  a project root with givn/ initialised
    And  a permanent spec for capability "add-cap" contains a scenario named "Existing behaviour"
    And  a completed change "add-docstring" whose delta adds scenario "New behaviour" within capability "add-cap" and gives a step the docstring:
      """
      payload:docstring-survives-added-merge
      """
    When  the user runs `givn archive --change add-docstring`
    Then  the command exits 0
    And  the permanent spec file for "add-cap" contains the text "docstring-survives-added-merge"

  Scenario: A modified scenario keeps its step data table after archiving
    Given  a project root with givn/ initialised
    And  a permanent spec for capability "table-cap" contains a scenario named "Existing behaviour"
    And  a completed change "keep-table" whose delta modifies scenario "Existing behaviour" within capability "table-cap" and gives a step a data table containing the text "table-survives-modified-merge"
    When  the user runs `givn archive --change keep-table`
    Then  the command exits 0
    And  the permanent spec file for "table-cap" contains the text "table-survives-modified-merge"

  @e2e
  Scenario: Consumer archives a modified scenario and the permanent spec keeps the docstring
    Given  a givn project with a completed change "real-modify" whose delta modifies scenario "Existing behaviour" within capability "real-cap" and gives a step the docstring:
      """
      payload:docstring-present-after-real-archive
      """
    When  the user invokes `givn archive --change real-modify`
    Then  the command succeeds
    And  the permanent spec file for "real-cap" contains the text "docstring-present-after-real-archive"
