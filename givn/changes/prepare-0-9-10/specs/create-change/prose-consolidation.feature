@givn.delta @prose-consolidation
Feature: prose-consolidation

  @givn.modified @e2e
  Scenario: Generated guidance points to the canonical E2E policy
    Given a Givn project with all agent skill targets and optional guidance enabled
    When the consumer synchronizes the selected agent guidance through the Givn CLI
    Then the generated givn-spec skill directs the author to the canonical specs instruction
    And managed bootstrap guidance owns loading the effective dialog policy before the first question once per context
    And thin skills contain no repeated dialog-loading or delegation boilerplate
    And detailed dialog examples remain available through the policy's companion index
    And synchronization reports skills without separately generated commands

  @givn.modified
  Scenario: Canonical policy requires a rendered-state assertion
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then browser E2E guidance accepts a real browser rendering static content as interface evidence
    And it requires state changes only when the product interaction actually changes state
    And it does not require artificial clocks counters or asynchronous content

  @givn.modified
  Scenario: The propose and archive commands carry the seed stamping instructions
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then the served proposal instruction preserves the proposed-by seed stamp
    And the served archive instruction preserves the implemented-in seed stamp
    And generated propose and archive skills direct authors to those instructions
    And no separately generated propose or archive command is installed

  @givn.modified
  Scenario: Disabling optional addons does not disable the dialog obligation
    Given a Givn project with all agent skill targets selected and every optional addon disabled
    When the author synchronizes the selected guidance through the Givn CLI
    Then the managed bootstrap still requires loading the effective dialog policy before the context's first question
    And guidance for disabled addons is not installed
    And core skills contain no repeated policy-loading boilerplate
    And the selected targets and addon choices remain unchanged

  @givn.removed
  Scenario: Delegated question preparation carries the policy without assuming inherited context
    Given this scenario is removed

  @givn.modified
  Scenario: Successful project skill and command replacements are preserved without policy injection
    Given a Givn project with all agent skill targets selected
    And valid project-owned skill replacements intentionally omit bundled dialog-loading text
    When the author synchronizes the selected guidance through the Givn CLI
    Then project-owned skill source bytes remain unchanged
    And every selected skill consumer receives the authoritative replacement without a bundled prefix
    And user-owned legacy command files remain unchanged without being generated or synchronized
    And managed project-start guidance still references the effective dialog policy

  @givn.added
  Scenario: Canonical policy accepts the library public API as its real interface
    Given a project whose product is a library without a service or browser
    When the author reads the canonical E2E guidance
    Then consumer calls to the public library API may run in process as real-interface evidence
    And the policy does not require artificial services containers or browser infrastructure

  @givn.added
  Scenario: Canonical policy selects E2E scope explicitly rather than by command shape
    Given regular and E2E verification share one runner command
    When the author reads the canonical verification guidance
    Then explicit scope selection such as GIVN_RESULT_SCOPE distinguishes the executions
    And equal test counts or command strings do not invalidate honestly isolated scopes
    And required executions and zero failed or skipped tests remain required without an extra isolation run

  @givn.added
  Scenario: Justified E2E reclassification preserves real-interface coverage
    Given an existing E2E scenario is shown to be a variant rather than a distinct action
    When the author reads the canonical reclassification guidance
    Then reclassification requires a reason and aligned interaction inventory tasks and review records
    And every retained real user action still has exactly one E2E smoke scenario
    And tags cannot be removed merely to evade verification
    And regular scenarios protect requirements distinct from the retained smoke
