@specification-manifest
Feature: specification-manifest

  @e2e @specification-manifest
  Scenario: The declared categories are read by the project-health pass
    Given  an ise-enabled project selecting opencode with a Specification Manifest naming the five categories and two source roots
    When  the consumer runs `givn lint`
    Then  the command exits 0
    And  stderr contains no manifest-schema error
    And  stderr contains the declared-path advisory for the contracts category
    And  the terminal transcript is captured for this scenario

  @specification-manifest
  Scenario Outline: A malformed Specification Manifest is refused by name and left unchanged
    Given  an ise-enabled project selecting opencode whose Specification Manifest <mutation> and whose bytes are recorded
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names <named>
    And  the Specification Manifest bytes equal the recorded bytes
    Examples:
      | mutation | named |
      | adds the key "components" | "components" |
      | misspells the category "behavior" as "behaviour" | "behaviour" |
      | repeats the category "e2e" | "e2e" |
      | omits the category "limits" | "limits" |
      | omits the source list | "source" |
      | declares the source root "../other/crates/**" | "../other/crates/**" |
      | declares the structure path "/opt/specs/**" | "/opt/specs/**" |

  @specification-manifest
  Scenario: A Specification Manifest that is not valid YAML is refused by name
    Given  an ise-enabled project selecting opencode whose Specification Manifest is not valid YAML
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "givn/specification.yaml"

  @specification-manifest
  Scenario: A category that is not a list is refused by name
    Given  an ise-enabled project selecting opencode whose Specification Manifest holds a scalar for the category "behavior"
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "behavior"

  @specification-manifest
  Scenario: An unreadable Specification Manifest is refused by name
    Given  an ise-enabled project selecting opencode whose Specification Manifest path is a directory
    When  the user runs `givn lint`
    Then  the command exits 1 and names "givn/specification.yaml"

  @specification-manifest
  Scenario: An unknown category is named before the category it replaces
    Given  an ise-enabled project selecting opencode whose Specification Manifest misspells the category "behavior" as "behaviour"
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "behaviour"
    And  stderr does not report the omitted category "behavior"

  @specification-manifest
  Scenario: A project with ise disabled reads no Specification Manifest
    Given  an initialized project selecting opencode with ise disabled and a malformed Specification Manifest
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains no manifest finding

  @specification-manifest
  Scenario: A Specification Manifest that names rebuild files under the four content categories is accepted
    Given  an ise-enabled project selecting opencode whose Specification Manifest places rebuild files under the four content categories
    When  the consumer runs `givn lint`
    Then  the command exits 0
    And  stderr contains no manifest-schema error
    And  stderr contains no declared-path advisory for the rebuild files
