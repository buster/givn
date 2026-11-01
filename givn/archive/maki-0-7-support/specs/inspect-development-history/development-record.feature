@givn.delta @development-record
Feature: Development Record

  @givn.added
  Scenario: A reported session point entry is recorded with its title
    Given a project root with givn/ initialised
    When the agent reports a session named "Maki session"
    Then the record holds one session entry with that title

  @givn.added
  Scenario: A reported event may name the session it belongs to
    Given a project root with givn/ initialised
    When the agent reports an event for the session "child" in a payload for "main"
    Then the record holds that entry under the session "child"

  @givn.added
  Scenario: A tool the agent moved to the background is marked
    Given a project root with givn/ initialised
    When the agent reports a completed tool call marked as moved to the background
    Then the record marks that tool call as backgrounded

  @givn.added
  Scenario: A reported usage carries its cost
    Given a project root with givn/ initialised
    When the agent reports a subagent span with a usage cost
    Then the record holds a usage entry carrying that cost

  @givn.added
  Scenario: A reported session entry without a title is rejected
    Given a project root with givn/ initialised
    When the agent reports a session point entry with no title
    Then the intake names the rejected session event
    And the record holds no session entry

  @givn.added
  Scenario: A reported usage cost that is not a number is rejected
    Given a project root with givn/ initialised
    When the agent reports a subagent span with a non-numeric usage cost
    Then the intake names the rejected usage event
    And the record holds no usage entry for that subagent
