Feature: Outline report

  @e2e
  Scenario: A developer runs a outline report and sees the classified units
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as a god type
    And  the report labels the unit as a hint
    And  the report records no recommendation
    And  the report writes no baseline file

  Scenario: The outline experiment is disabled by default
    Given  a Givn project with the outline experiment disabled
    When  the developer runs "givn addons list"
    Then  the outline addon is listed as disabled

  Scenario: The outline report is unavailable when the experiment is disabled
    Given  a Givn project with the outline experiment disabled
    When  the developer runs "givn outline report"
    Then  the command exits non-zero
    And  stderr contains "outline experiment is disabled"

  Scenario: A data-only type is not treated as a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a data-only enum over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as data-only
    And  the report labels the unit as a hint
    And  the report records no recommendation

  Scenario: An interface is classified as an interface, not a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a trait over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  Scenario: A cohesive type is not treated as a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a cohesive type over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as a cohesive-large-type
    And  the report records no recommendation

  Scenario: A type with many public methods is an interface
    Given  a production source unit with more than 50 public methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  Scenario: A long method is a method-length finding
    Given  a production source unit with a long method
    When  the outline report assesses the unit
    Then  the report classifies the unit as a method-length finding
    And  the report records no recommendation

  Scenario: Test sources are not scanned
    Given  a Givn project with the outline experiment enabled
    And  a production source unit and a test source unit
    When  the developer runs "givn outline report"
    Then  the report does not name the test source

  Scenario: A type at the readable size limit is within limits
    Given  a production source unit with exactly 500 code lines and cohesive methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as within-limits
    And  the report labels the unit as within limits
    And  the report records no recommendation

  Scenario: A type one line over the readable size limit is a finding
    Given  a production source unit with 501 code lines and cohesive methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report labels the unit as a hint
    And  the report records 501 code lines
    And  the report records no recommendation

  Scenario: A report is available in machine-readable form
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --json"
    Then  stdout is valid JSON
    And  the JSON report classifies the unit
    And  the JSON report has no action or recommendation fields

  Scenario: A report is written to a file
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --out report.txt"
    Then  the report is written to "report.txt"
    And  stdout is empty

  Scenario: A report output path that cannot be written is an error
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --out missing/report.txt"
    Then  the command exits with code 1
    And  stderr names "missing/report.txt"

  Scenario: An unsupported construct is named and not invented
    Given  a Givn project with the outline experiment enabled
    And  a production source file with an item-position macro
    When  the developer runs "givn outline report"
    Then  the report names the unsupported construct
    And  the report does not classify the unsupported unit

  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the outline experiment enabled
    And  a production source unit
    When  the developer runs "givn outline report --json"
    Then  the JSON report has schema version 3
    And  the JSON report has rules version 1

  Scenario: The JSON report carries analyzer capabilities and limitations
    Given  a Givn project with the outline experiment enabled
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the supported constructs
    And  the JSON report names the analyzer limitations

  Scenario: Conditional code is counted as written
    Given  a production source unit with platform-conditional methods
    When  the outline report assesses the unit
    Then  the report counts both platform variants

  Scenario: The same source produces the same report through either analysis engine
    Given  a production source unit supported by two analysis engines
    When  the outline report is rendered from each engine
    Then  the two reports are identical

  Scenario: The report is deterministic
    Given  a production source unit
    When  the outline report is produced twice
    Then  the two reports are identical

  Scenario: A developer scopes the report to a workspace member
    Given  a Givn project with the outline experiment enabled
    And  a workspace member "crates/core" with a production source unit that is over the readable size
    When  the developer runs "givn outline report --path crates/core"
    Then  the report classifies the unit as a god type
    And  the report names the file "crates/core/src/engine.rs"

  Scenario: The report says when the scope has no production sources
    Given  a Givn project with the outline experiment enabled
    And  no production source files
    When  the developer runs "givn outline report"
    Then  the report says there are no production sources
    And  the command exits with code 0

  Scenario: An explicitly requested scope that does not exist is an error
    Given  a Givn project with the outline experiment enabled
    When  the developer runs "givn outline report --path missing"
    Then  the command exits with code 1
    And  stderr names "missing"

  Scenario: A file that cannot be analyzed is reported and the report continues
    Given  a Givn project with the outline experiment enabled
    And  a well-formed production source unit
    And  a malformed production source file
    When  the developer runs "givn outline report"
    Then  the report names the malformed file
    And  the report still classifies the well-formed unit
  Scenario: A large type in the inspect band is a cohesive large type
    Given  a production source unit with a large type in the inspect band
    When  the outline report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report records no recommendation

  Scenario: A module over the plan limit is a module finding
    Given  a production source unit with a module over the plan limit
    When  the outline report assesses the unit
    Then  the report classifies the unit as a module
    And  the report labels the unit as a hint
    And  the report records no recommendation

  Scenario: The JSON hints count counts every non-within-limits unit
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    And  a production source unit with a data-only enum over the readable size
    And  a production source unit within the readable size
    When  the developer runs "givn outline report --json"
    Then  the JSON hints count matches the non-within-limits units
    And  the JSON report classifies the data-only unit

  Scenario: Every analyzed unit has exactly one classification
    Given  the outline fixture corpus
    When  the outline report assesses every unit
    Then  every unit has exactly one classification
    And  no unit carries an action or recommendation

  Scenario: Units are sorted by classification and path
    Given  the outline fixture corpus
    When  the developer runs "givn outline report"
    Then  the report lists units by classification, then path, then unit id

  Scenario: A Python class is outlined with its methods and calls
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the Python class "OrderBook"
    And  the JSON report lists the class methods "add" and "fill"
    And  the JSON outline records the call from "fill" to "add"
    And  the JSON report records the file language "python"

  Scenario: A component outline is a call tree
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    When  the developer runs "givn outline report"
    Then  the outline of the class "OrderBook" shows "fill" calling "add"

  Scenario: A call to another component is a leaf
    Given  a Givn project with the outline experiment enabled
    And  a production source unit whose method calls a module function
    When  the developer runs "givn outline report"
    Then  the type outline calls the module function without expanding it

  Scenario: A Python module without classes is a module unit
    Given  a Givn project with the outline experiment enabled
    And  a Python module with top-level functions
    When  the developer runs "givn outline report --json"
    Then  the JSON report names one Python module unit
    And  the JSON report lists the module functions "load" and "save"

  Scenario: A Python project whose analyzer is unavailable still reports
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    And  python3 is not on the PATH
    When  the developer runs the outline report
    Then  the command exits with code 0
    And  the report names the file "src/tracker.py"
    And  the report names the analyzer as unavailable

  Scenario: A Rust-only project never invokes python3
    Given  a Givn project with the outline experiment enabled
    And  a production source unit within the readable size
    And  a python3 recorder that records every invocation
    When  the developer runs the outline report with the recorder on the PATH
    Then  the command exits with code 0
    And  the python3 recorder records no invocation
    And  the report names only the rust language
