# User Interaction Inventory:

@givn.delta @nli-evidence
Feature: Independent NLI duplicate-audit pool

  @givn.modified @nli
  Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit
    Given a permanent corpus with deterministic semantic candidates for an independent NLI audit
    When the user runs the permanent NLI duplicate audit
    Then NLI receives the full E5 semantic candidate pool independently of BGE
    And the NLI audit retains the E5 candidate identities and ordering
