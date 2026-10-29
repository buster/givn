@givn.delta @outline-report
Feature: Outline report

  @givn.modified @e2e
  Scenario: A developer runs a outline report and sees the classified units
    Given a Givn project with the outline experiment enabled
    And a production source unit that is over the readable size
    When the developer runs "givn outline report"
    Then the command exits with code 0
    And the report classifies the unit as a god type
    And the report labels the finding as advisory rather than a review or archive verdict
    And the report records no recommendation
    And the report writes no baseline file
    And the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: The report fails when the scope has no production sources
    Given a Givn project with the outline experiment enabled
    And no production source files
    When the developer runs "givn outline report --path ."
    Then the command exits with code 1
    And stderr names the requested scope
    And stderr says there are no supported production sources
    And stderr names Rust and Python as the supported source languages

  @givn.added
  Scenario Outline: Unsupported-only production sources are not reported as absent
    Given a Givn project with the outline experiment enabled
    And the requested scope contains only production source files written in <language>
    When the developer runs "givn outline report --path ."
    Then the command exits with code 1
    And stderr names the requested scope
    And stderr says there are no supported production sources
    And stderr names Rust and Python as the supported source languages
    And stderr does not claim that no production sources exist

    Examples:
      | language |
      | Java     |
      | Go       |

  @givn.modified
  Scenario: The JSON report carries analyzer capabilities and limitations
    Given a Givn project with the outline experiment enabled
    And a production source unit
    When the developer runs "givn outline report --json"
    Then the JSON report names Rust and Python as the supported source languages
    And the JSON report names the supported constructs
    And the JSON report names the analyzer limitations
    And the limitations state that structural call order is not a complete runtime trace
    And the limitations state that uncertain or omitted call edges limit conclusions

  @givn.modified
  Scenario: A report is available in machine-readable form
    Given a Givn project with the outline experiment enabled
    And a production source unit that is over the readable size
    When the developer runs "givn outline report --json"
    Then stdout is valid JSON
    And the JSON report classifies the unit
    And the JSON report has no action or recommendation fields
    And the JSON report does not present classifications as review or archive verdicts

  @givn.added
  Scenario Outline: Unavailable outline measurements are not reported as measured zero
    Given a Givn project with the outline experiment enabled
    And a supported production source unit for which <measurement> is unavailable
    When the developer requests the outline report in text and JSON form
    Then the text report marks <measurement> as unavailable
    And the JSON report represents <measurement> as null
    And neither report represents the unavailable measurement as zero

    Examples:
      | measurement          |
      | tight class cohesion |
      | method complexity    |
      | method nesting       |

  @givn.added
  Scenario: Loose class cohesion is omitted when it is not independently measured
    Given a Givn project with the outline experiment enabled
    And a production source unit with measured tight class cohesion
    And loose class cohesion is not independently measured
    When the developer requests the outline report in text and JSON form
    Then neither report includes an LCC measurement
    And neither report presents tight class cohesion as loose class cohesion
