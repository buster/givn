@givn.delta @readability-report

Feature: Readability report

  @givn.modified @e2e
  Scenario: A developer runs a readability report and sees the classified units
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as a god type
    And  the report labels the unit as a finding
    And  the report records no recommendation
    And  the report writes no baseline file

  @givn.modified
  Scenario: A data-only type is not treated as a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a data-only enum over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as data-only
    And  the report labels the unit as a finding
    And  the report records no recommendation

  @givn.modified
  Scenario: An interface is classified as an interface, not a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a trait over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  @givn.modified
  Scenario: A cohesive type is not treated as a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a cohesive type over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as a cohesive-large-type
    And  the report records no recommendation

  @givn.modified
  Scenario: A type with many public methods is an interface
    Given  a production source unit with more than 50 public methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  @givn.removed @wip
  Scenario: A large type in the inspect band needs an exception
    Given a placeholder step

  @givn.added
  Scenario: A large type in the inspect band is a cohesive large type
    Given a production source unit with a large type in the inspect band
    When the readability report assesses the unit
    Then the report classifies the unit as a cohesive large type
    And the report records no recommendation

  @givn.removed @wip
  Scenario: A module over the plan limit needs an exception
    Given a placeholder step

  @givn.added
  Scenario: A module over the plan limit is a module finding
    Given a production source unit with a module over the plan limit
    When the readability report assesses the unit
    Then the report classifies the unit as a module
    And the report labels the unit as a finding
    And the report records no recommendation

  @givn.modified
  Scenario: A long method is a method-length finding
    Given  a production source unit with a long method
    When  the readability report assesses the unit
    Then  the report classifies the unit as a method-length finding
    And  the report records no recommendation

  @givn.modified
  Scenario: A type at the readable size limit is within limits
    Given  a production source unit with exactly 500 code lines and cohesive methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as within-limits
    And  the report labels the unit as within limits
    And  the report records no recommendation

  @givn.modified
  Scenario: A type one line over the readable size limit is a finding
    Given  a production source unit with 501 code lines and cohesive methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report labels the unit as a finding
    And  the report records 501 code lines
    And  the report records no recommendation

  @givn.modified
  Scenario: A report is available in machine-readable form
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report --json"
    Then  stdout is valid JSON
    And  the JSON report classifies the unit
    And  the JSON report has no action or recommendation fields

  @givn.added
  Scenario: The JSON findings count counts every non-within-limits unit
    Given a Givn project with the readability experiment enabled
    And a production source unit that is over the readable size
    And a production source unit with a data-only enum over the readable size
    And a production source unit within the readable size
    When the developer runs "givn readability report --json"
    Then the JSON findings count matches the non-within-limits units
    And the JSON report classifies the data-only unit

  @givn.modified
  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the readability experiment enabled
    And  a production source unit
    When  the developer runs "givn readability report --json"
    Then  the JSON report has schema version 2
    And  the JSON report has rules version 1

  @givn.removed @wip
  Scenario: Every analyzed unit has one classification and one action
    Given a placeholder step

  @givn.added
  Scenario: Every analyzed unit has exactly one classification
    Given  the readability fixture corpus
    When  the readability report assesses every unit
    Then  every unit has exactly one classification
    And  no unit carries an action or recommendation

  @givn.added
  Scenario: Units are sorted by classification and path
    Given  the readability fixture corpus
    When  the developer runs "givn readability report"
    Then  the report lists units by classification, then path, then unit id
