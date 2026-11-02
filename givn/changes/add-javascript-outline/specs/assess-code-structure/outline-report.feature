@givn.delta @outline-report

Feature: Outline report

  @givn.modified
  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the outline experiment enabled
    And  a production source unit
    When  the developer runs "givn outline report --json"
    Then  the JSON report has schema version 5
    And  the JSON report has rules version 1

  @givn.modified
  Scenario: A mixed-language report discloses every supported language
    Given  a Givn project with the outline experiment enabled
    And  a mixed Rust Python Java TypeScript and JavaScript production source set
    When  the developer runs "givn outline report --json"
    Then  the JSON report names all supported languages
    And  the JSON report names the unit "Widget" with language "java"
    And  the JSON report names the unit "Widget" with language "typescript"
    And  the JSON report names the unit "Widget" with language "javascript"

  @givn.added
  Scenario: A JavaScript production class and function are reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production class and function
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the unit "Widget" with language "javascript"
    And  the JSON report gives the type kind "class" to "Widget"
    And  the JSON report names the function "makeWidget" with language "javascript"

  @givn.added
  Scenario: A JavaScript class field is reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production class with a field
    When  the developer runs "givn outline report --json"
    Then  the JSON report records 1 field on "Widget"

  @givn.added
  Scenario Outline: JavaScript production modules are reported for every supported extension
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production module with the extension "<extension>"
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the function "drain" with language "javascript"

    Examples:
      | extension |
      | .js       |
      | .mjs      |
      | .cjs      |

  @givn.added
  Scenario: A JavaScript arrow-function binding is reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production arrow-function binding
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the function "clone" with language "javascript"

  @givn.added
  Scenario: A malformed JavaScript file does not hide a valid TypeScript file
    Given  a Givn project with the outline experiment enabled
    And  a malformed JavaScript production file beside a valid TypeScript production file
    When  the developer runs "givn outline report --json"
    Then  the JSON report records a parse failure for "src/Broken.js"
    And  the JSON report names the unit "Widget" with language "typescript"

  @givn.added
  Scenario: JavaScript test and dependency sources are excluded
    Given  a Givn project with the outline experiment enabled
    And  JavaScript test and dependency sources
    When  the developer runs "givn outline report --json"
    Then  stdout does not contain "IgnoredJavaScript"
    And  stdout does not contain "IgnoredJavaScriptDependency"

  @givn.added
  Scenario: JSX-only production sources remain unsupported
    Given  a Givn project with the outline experiment enabled
    And  the requested scope contains only production source files written in JSX
    When  the developer runs "givn outline report --path ."
    Then  the command exits with code 1
    And  stderr names the requested scope
    And  stderr says there are no supported production sources
    And  stderr names all supported source languages
    And  stderr does not claim that no production sources exist
