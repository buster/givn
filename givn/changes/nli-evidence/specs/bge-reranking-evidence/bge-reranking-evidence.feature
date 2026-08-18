@givn.delta @bge-reranking-evidence
Feature: BGE reranking evidence for bounded semantic search

  @givn.modified @nli
  Scenario: The independent NLI capability owns the legacy diagnostic selector
    Given a project with a permanent semantic candidate for second-stage evidence
    And the independent NLI evidence capability is enabled
    And the legacy NLI selector is explicitly selected
    When the user runs advisory change review through the NLI-capable CLI
    Then stdout contains diagnostic NLI evidence
    And stdout contains no BGE admission status
    And stdout reports NLI recommended and filtered candidate counts
    And the E5 candidate identities remain unchanged
