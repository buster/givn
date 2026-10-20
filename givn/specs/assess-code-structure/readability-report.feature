Feature: Readability report

  @e2e
  Scenario: A developer runs a readability report and sees the classified units
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as a god type
    And  the report recommends a plan
    And  the report writes no baseline file

  Scenario: The readability experiment is disabled by default
    Given  a Givn project with the readability experiment disabled
    When  the developer runs "givn addons list"
    Then  the readability addon is listed as disabled

  Scenario: The readability report is unavailable when the experiment is disabled
    Given  a Givn project with the readability experiment disabled
    When  the developer runs "givn readability report"
    Then  the command exits non-zero
    And  stderr contains "readability experiment is disabled"

  Scenario: A data-only type is not treated as a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a data-only enum over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as data-only
    And  the report recommends an advisory action
    And  the report recommends no plan for the unit

  Scenario: An interface is classified as an interface, not a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a trait over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as an interface
    And  the report recommends a trait-composition plan

  Scenario: A cohesive type is not treated as a god type
    Given  a Givn project with the readability experiment enabled
    And  a production source unit with a cohesive type over the readable size
    When  the developer runs "givn readability report"
    Then  the report classifies the unit as a cohesive-large-type
    And  the report recommends an advisory action

  Scenario: A type with many public methods is an interface
    Given  a production source unit with more than 50 public methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as an interface
    And  the report recommends a trait-composition plan

  Scenario: A large type in the inspect band needs an exception
    Given  a production source unit with a large type in the inspect band
    When  the readability report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report recommends an exception action

  Scenario: A module over the plan limit needs an exception
    Given  a production source unit with a module over the plan limit
    When  the readability report assesses the unit
    Then  the report classifies the unit as a module
    And  the report recommends an exception action

  Scenario: A long method is a method-length finding
    Given  a production source unit with a long method
    When  the readability report assesses the unit
    Then  the report classifies the unit as a method-length finding
    And  the report recommends a watch action

  Scenario: Test sources are not scanned
    Given  a Givn project with the readability experiment enabled
    And  a production source unit and a test source unit
    When  the developer runs "givn readability report"
    Then  the report does not name the test source

  Scenario: A type at the readable size limit is within limits
    Given  a production source unit with exactly 500 code lines and cohesive methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as within-limits
    And  the report recommends no action

  Scenario: A type one line over the readable size limit is a finding
    Given  a production source unit with 501 code lines and cohesive methods
    When  the readability report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report recommends an advisory action
    And  the report records 501 code lines

  Scenario: A report is available in machine-readable form
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report --json"
    Then  stdout is valid JSON
    And  the JSON report classifies the unit

  Scenario: A report is written to a file
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report --out report.txt"
    Then  the report is written to "report.txt"
    And  stdout is empty

  Scenario: A report output path that cannot be written is an error
    Given  a Givn project with the readability experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn readability report --out missing/report.txt"
    Then  the command exits with code 1
    And  stderr names "missing/report.txt"

  Scenario: An unsupported construct is named and not invented
    Given  a Givn project with the readability experiment enabled
    And  a production source file with an item-position macro
    When  the developer runs "givn readability report"
    Then  the report names the unsupported construct
    And  the report does not classify the unsupported unit

  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the readability experiment enabled
    And  a production source unit
    When  the developer runs "givn readability report --json"
    Then  the JSON report has schema version 1
    And  the JSON report has rules version 1

  Scenario: The JSON report carries analyzer capabilities and limitations
    Given  a Givn project with the readability experiment enabled
    When  the developer runs "givn readability report --json"
    Then  the JSON report names the supported constructs
    And  the JSON report names the analyzer limitations

  Scenario: Conditional code is counted as written
    Given  a production source unit with platform-conditional methods
    When  the readability report assesses the unit
    Then  the report counts both platform variants

  Scenario: The same source produces the same report through either analysis engine
    Given  a production source unit supported by two analysis engines
    When  the readability report is rendered from each engine
    Then  the two reports are identical

  Scenario: The report is deterministic
    Given  a production source unit
    When  the readability report is produced twice
    Then  the two reports are identical

  Scenario: Every analyzed unit has one classification and one action
    Given  the readability fixture corpus
    When  the readability report assesses every unit
    Then  every unit has exactly one classification and one action

  Scenario: A developer scopes the report to a workspace member
    Given  a Givn project with the readability experiment enabled
    And  a workspace member "crates/core" with a production source unit that is over the readable size
    When  the developer runs "givn readability report --path crates/core"
    Then  the report classifies the unit as a god type
    And  the report names the file "crates/core/src/engine.rs"

  Scenario: The report says when the scope has no production sources
    Given  a Givn project with the readability experiment enabled
    And  no production source files
    When  the developer runs "givn readability report"
    Then  the report says there are no production sources
    And  the command exits with code 0

  Scenario: An explicitly requested scope that does not exist is an error
    Given  a Givn project with the readability experiment enabled
    When  the developer runs "givn readability report --path missing"
    Then  the command exits with code 1
    And  stderr names "missing"

  Scenario: A file that cannot be analyzed is reported and the report continues
    Given  a Givn project with the readability experiment enabled
    And  a well-formed production source unit
    And  a malformed production source file
    When  the developer runs "givn readability report"
    Then  the report names the malformed file
    And  the report still classifies the well-formed unit
