# User Interaction Inventory:
# - run `givn check <artifact>` with a valid artifact id
# - run `givn check <artifact>` with an unknown artifact id
# - run `givn eject <artifact>:instruction` with a valid artifact id
# - run `givn eject <artifact>:instruction` with an unknown artifact id

@givn.delta @validate-artifact-id-input

Feature: Validate Artifact ID Input

  @givn.added @wip
  Scenario: givn check with unknown artifact validates against manifest and provides helpful error
    Given a project root with givn/ initialised
    When the user runs `givn check nonexistent`
    Then the command exits non-zero
    And stderr contains "unknown artifact"
    And stderr lists known artifact ids

  @givn.added @wip
  Scenario: givn check with known artifact proceeds to run hooks
    Given a project root with givn/ initialised
    And a change "check-demo" exists with artifact "proposal.md"
    When the user runs `givn check proposal --change check-demo`
    Then the command exits 0

  @givn.added @wip
  Scenario: givn eject instruction with unknown artifact validates against manifest and provides helpful error
    Given a project root with givn/ initialised
    When the user runs `givn eject nonexistent:instruction`
    Then the command exits non-zero
    And stderr says the artifact is not in the effective manifest
    And stderr lists known artifact ids

  @givn.added @e2e @wip
  Scenario: Consumer invokes givn check with an unknown artifact and gets a helpful error
    Given a givn project with a standard manifest
    When the user invokes `givn check bogus-artifact`
    Then the command fails
    And stderr contains "unknown artifact"
