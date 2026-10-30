@givn.delta @outline-report

Feature: Outline report

  @givn.removed
  Scenario Outline: Unsupported-only production sources are not reported as absent
    Given placeholder

  @givn.removed
  Scenario: The Tree-sitter evaluation leaves the current outline engine in place
    Given placeholder

  @givn.modified
  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the outline experiment enabled
    And  a production source unit
    When  the developer runs "givn outline report --json"
    Then  the JSON report has schema version 4
    And  the JSON report has rules version 1

  @givn.added
  Scenario: A Java production class is reported
    Given a Givn project with the outline experiment enabled
    And a Java production class
    When the developer runs "givn outline report --json"
    Then the JSON report names the unit "Widget" with language "java"
    And the JSON report gives the type kind "class" to "Widget"

  @givn.added
  Scenario: A TypeScript production class and function are reported
    Given a Givn project with the outline experiment enabled
    And a TypeScript production class and function
    When the developer runs "givn outline report --json"
    Then the JSON report names the unit "Widget" with language "typescript"
    And the JSON report names the function "makeWidget" with language "typescript"

  @givn.added
  Scenario: A TypeScript interface is reported with its own type kind
    Given a Givn project with the outline experiment enabled
    And a TypeScript production interface
    When the developer runs "givn outline report --json"
    Then the JSON report gives the type kind "interface" to "Drawable"

  @givn.added
  Scenario: A mixed-language report discloses every supported language
    Given a Givn project with the outline experiment enabled
    And a mixed Rust Python Java and TypeScript production source set
    When the developer runs "givn outline report --json"
    Then the JSON report names all supported languages
    And the JSON report names the unit "Widget" with language "java"
    And the JSON report names the unit "Widget" with language "typescript"

  @givn.added
  Scenario: A malformed Java file does not hide a valid TypeScript file
    Given a Givn project with the outline experiment enabled
    And a malformed Java production file beside a valid TypeScript production file
    When the developer runs "givn outline report --json"
    Then the JSON report records a parse failure for "src/Broken.java"
    And the JSON report names the unit "Widget" with language "typescript"

  @givn.added
  Scenario: Java and TypeScript test and dependency sources are excluded
    Given a Givn project with the outline experiment enabled
    And Java and TypeScript test and dependency sources
    When the developer runs "givn outline report --json"
    Then stdout does not contain "IgnoredJava"
    And stdout does not contain "IgnoredTypeScript"

  @givn.added
  Scenario: Unsupported-only Go production sources are not reported as absent
    Given a Givn project with the outline experiment enabled
    And the requested scope contains only production source files written in Go
    When the developer runs "givn outline report --path ."
    Then the command exits with code 1
    And stderr names the requested scope
    And stderr says there are no supported production sources
    And stderr names all supported source languages
    And stderr does not claim that no production sources exist
