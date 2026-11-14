@specification-declaration
Feature: specification-declaration

  @e2e @specification-declaration
  Scenario: The declared roles are read by the project-health pass
    Given  an ise-enabled project selecting opencode with a declaration naming the five roles and two source roots
    When  the consumer runs `givn lint`
    Then  the command exits 0
    And  stderr contains no declaration-schema error
    And  stderr contains the declared-path advisory for the contracts role
    And  the terminal transcript is captured for this scenario

  @specification-declaration
  Scenario Outline: A malformed declaration is refused by name and left unchanged
    Given  an ise-enabled project selecting opencode whose declaration <mutation> and whose bytes are recorded
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names <named>
    And  the declaration bytes equal the recorded bytes
    Examples:
      | mutation | named |
      | adds the key "components" | "components" |
      | misspells the role "behavior" as "behaviour" | "behaviour" |
      | repeats the role "e2e" | "e2e" |
      | omits the role "limits" | "limits" |
      | omits the source list | "source" |
      | declares the source root "../other/crates/**" | "../other/crates/**" |
      | declares the structure path "/opt/specs/**" | "/opt/specs/**" |

  @specification-declaration
  Scenario: A declaration that is not valid YAML is refused by name
    Given  an ise-enabled project selecting opencode whose declaration is not valid YAML
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "givn/specification.yaml"

  @specification-declaration
  Scenario: A role that is not a list is refused by name
    Given  an ise-enabled project selecting opencode whose declaration holds a scalar for the role "behavior"
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "behavior"

  @specification-declaration
  Scenario: An unreadable declaration is refused by name
    Given  an ise-enabled project selecting opencode whose declaration path is a directory
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "givn/specification.yaml"

  @specification-declaration
  Scenario: An unknown role is named before the role it replaces
    Given  an ise-enabled project selecting opencode whose declaration misspells the role "behavior" as "behaviour"
    When  the user runs `givn lint`
    Then  the command exits non-zero
    And  stderr names "behaviour"
    And  stderr does not report the omitted role "behavior"

  @specification-declaration
  Scenario: A project with ise disabled reads no declaration
    Given  an initialized project selecting opencode with ise disabled and a malformed declaration
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains no declaration finding
