Feature: Installed builds always evaluate the semantic gates with the real models

  @e2e @release-proof
  Scenario: An installed givn refuses a request for simulated embedder output
    Given  a project prepared for semantic indexing
    And  a request for simulated embedder output
    When  a consumer runs `givn spec index` with an installed givn build
    Then  the installed givn announces that the simulated-embedder request is ignored
    And  the installed givn refuses to proceed without the real embedder model

  @release-proof
  Scenario: An installed givn ignores requests to disable or simulate the second-stage models
    Given  a project prepared for semantic indexing
    And  a request to disable the reranker evidence stage
    And  a request for simulated entailment evidence
    When  a consumer runs `givn spec index` with an installed givn build
    Then  the installed givn announces that the disable-reranker request is ignored
    And  the installed givn announces that the simulated-entailment request is ignored
    And  the installed givn refuses to proceed without the real model evidence

  @e2e @release-proof
  Scenario: An installed givn honours a model-location request that still uses the real models
    Given  a project prepared for semantic indexing
    And  a request for the real models from a chosen cache location
    When  a consumer runs `givn spec index` with an installed givn build
    Then  the installed givn does not announce that any model request is ignored

  Scenario: A development givn build honours a simulated-embedder request without an installed-build announcement
    Given  a project prepared for semantic indexing
    And  a request for simulated embedder output
    When  a consumer runs `givn spec index` with a development givn build
    Then  the index is built
    And  no installed-build announcement is emitted
  @e2e @release-proof
  Scenario: An installed givn announces each refused request by name
    Given  a project prepared for semantic indexing
    And  a request for simulated embedder output
    And  a request to resize the second-stage pair budget
    When  a consumer runs `givn spec index` with an installed givn build
    Then  the installed givn announces that the simulated-embedder request is ignored
    And  the installed givn announces that the pair-budget request is ignored
    And  the installed givn refuses to proceed without the real embedder model

  @e2e @release-proof
  Scenario: An installed givn keeps the review gate closed when model evidence is unavailable
    Given  a project prepared for semantic indexing
    And  a change whose review the gate must evaluate
    When  a consumer runs the change review gate with an installed givn build
    Then  the installed givn refuses to proceed without the real embedder model
