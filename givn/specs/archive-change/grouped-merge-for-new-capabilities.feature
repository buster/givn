Feature: grouped-merge-for-new-capabilities

  @e2e @grouped-merge-for-new-capabilities
  Scenario: Consumer archives a new capability and the merged spec lands in its declared use case
    Given  a givn project with a completed change declaring a capability NEW in "some-group"
    And  its supported delta feature has Background setup, inherited execution tags and a Scenario Outline with a docstring, table and Examples
    And  all applicable archive verification passes
    When  the consumer invokes `givn archive --change grouped-new`
    Then  the command succeeds and terminal output reports a proven archive
    And  the feature is published under "givn/specs/some-group" rather than a capability-named use-case directory
    And  the published feature retains its complete supported executable content and tag inheritance
    And  the terminal transcript is captured for this scenario

  @grouped-merge-for-new-capabilities
  Scenario: Archive is blocked when a new capability has no declared routing decision
    Given  a completed change introduces a new capability without a declared routing decision
    When  the developer runs `givn archive --change flat-new`
    Then  the command exits non-zero and explains the missing NEW routing decision
    And  the permanent capability is not created and the active change is not moved

  @grouped-merge-for-new-capabilities
  Scenario: Nothing is merged when one delta file of a change has no routing decision
    Given  a completed change has two deltas with one routed NEW in "mixed-group" and one unrouted
    When  the developer runs `givn archive --change mixed-routing`
    Then  the command exits non-zero and names the missing routing decision
    And  neither delta is merged and the active change is not moved

  Scenario: Retiring a use case's last capability removes the use-case directory and its narrative
    Given  a givn project with a permanent use case "retire-home" whose only capability is "only-cap"
    And  a completed change "retire-last" whose delta retires capability "only-cap"
    And  the project declares a complete run declaration
    When  the user invokes `givn archive --change retire-last`
    Then  the command exits 0
    And  the directory "givn/specs/retire-home" does not exist
    And  the tombstone "givn/specs/.retired.yaml" records capability "only-cap"
    And  the change directory has been moved to givn/archive/

  @grouped-merge-for-new-capabilities
  Scenario Outline: Review and archive resolve existing capability owners consistently
    Given  an otherwise valid completed change has a delta for capability "shared-capability"
    And  "shared-capability" has <existing owners>
    And  the change declares <routing> for "shared-capability"
    And  all other applicable review and archive evidence passes
    When  the developer runs <command>
    Then  the command <outcome> and reports <owner resolution>
    And  owner resolution completes before any permanent-spec mutation or active-change move
    And  a rejected command leaves permanent specs and the active change unchanged
    Examples:
      | command | existing owners | routing | outcome | owner resolution |
      | `givn check review --change owner-demo` | one use-case owner | EXTEND | succeeds | the existing use-case owner |
      | `givn archive --change owner-demo` | one use-case owner | EXTEND | succeeds | the existing use-case owner |
      | `givn check review --change owner-demo` | one fragment owner | EXTEND | succeeds | the existing fragment owner |
      | `givn archive --change owner-demo` | one fragment owner | EXTEND | succeeds | the existing fragment owner |
      | `givn check review --change owner-demo` | one use-case owner | NEW | exits non-zero | a NEW conflict with the existing use-case owner |
      | `givn archive --change owner-demo` | one use-case owner | NEW | exits non-zero | a NEW conflict with the existing use-case owner |
      | `givn check review --change owner-demo` | one fragment owner | NEW | exits non-zero | a NEW conflict with the existing fragment owner |
      | `givn archive --change owner-demo` | one fragment owner | NEW | exits non-zero | a NEW conflict with the existing fragment owner |
      | `givn check review --change owner-demo` | both a use-case owner and a fragment owner | EXTEND | exits non-zero | ambiguous capability owners |
      | `givn archive --change owner-demo` | both a use-case owner and a fragment owner | EXTEND | exits non-zero | ambiguous capability owners |
      | `givn check review --change owner-demo` | both a use-case owner and a fragment owner | NEW | exits non-zero | ambiguous capability owners |
      | `givn archive --change owner-demo` | both a use-case owner and a fragment owner | NEW | exits non-zero | ambiguous capability owners |

  @grouped-merge-for-new-capabilities
  Scenario: A new capability preserves its supported feature structure
    Given  a completed change routes a new capability into its declared use case
    And  the supported feature contains Background setup, inherited tags, docstrings, data tables and multiple tagged Examples blocks
    When  the developer archives the change
    Then  the new permanent feature retains the same supported setup and execution scope
    And  every step argument and Examples value remains attached to its original executable element
    And  only delta-operation tags are removed

  @grouped-merge-for-new-capabilities
  Scenario: An unsupported Rule in one new capability prevents every delta mutation
    Given  the supported Gherkin dialect does not support Rule
    And  a completed change has two correctly routed new capabilities
    And  one delta is supported while the other contains a Rule
    When  the developer archives the change
    Then  the command exits non-zero and names the unsupported Rule and feature path
    And  neither delta is merged and neither permanent capability is created
    And  the active change is not moved and no archive receipt is written
