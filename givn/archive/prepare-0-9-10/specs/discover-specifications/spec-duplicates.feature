@givn.delta @spec-duplicates
Feature: Audit duplicate and related permanent scenarios

  @givn.modified @e2e
  Scenario: Exact duplicates in one feature are reported together
    Given a spec discovery project whose corpus contains one feature with two scenarios having literally equal supported executable content
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "exact_duplicate"
    And stdout contains "scanned permanent corpus"
    And stdout contains both audit endpoints
    And the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: Exact duplicates across features are reported together
    Given a spec discovery project whose corpus contains two features with one scenario each having literally equal supported executable content
    And the scenarios have different titles and file paths
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "exact_duplicate"
    And both distinct titles and file paths identify the match endpoints

  @givn.modified
  Scenario: Same assertion shape without exact behavior is reported as a structural match
    Given a spec discovery project whose corpus contains two scenarios with identical assertions but different actions
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "same_assertion_shape"
    And the pair is not labelled "exact_duplicate"
    And the report describes the structural match as advisory rather than proof of equivalent behavior

  @givn.added
  Scenario Outline: Normalized executable differences do not establish literal equality
    Given two permanent scenarios whose normalized structural shape matches
    And their supported executable content differs only in <difference>
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And the pair is reported as a structural match
    And the pair is not labelled "exact_duplicate"

    Examples:
      | difference                 |
      | a quoted step parameter    |
      | a numeric step parameter   |
      | a data-table argument cell |
      | a docstring argument       |
      | an Examples value          |

  @givn.added
  Scenario: Different inherited setup prevents an exact duplicate label
    Given two permanent scenarios with literally equal steps and step arguments
    And their supported Background setup differs
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And the pair is not labelled "exact_duplicate"

  @givn.added
  Scenario: Equal supported executable arguments remain exact duplicates
    Given two permanent Scenario Outlines with different titles and file paths
    And their supported Backgrounds, step keywords, step text, tables, docstrings, and Examples are literally equal
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And the pair is labelled "exact_duplicate"

  @givn.modified
  Scenario: Repeating an unchanged audit produces the same report
    Given a spec discovery project with several exact and structural matches across different files and titles
    When the user audits the unchanged corpus twice in text format and twice in JSON format
    Then the command exits 0
    And both text audit reports are identical
    And both JSON audit reports are identical
    And match endpoints and match ordering do not depend on filesystem enumeration order
