@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The ise instruction makes the declaration cover every build input
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names the build manifest, the lockfile, the toolchain pin, a structural template, a schema or migration asset, and served instruction text as build inputs
    And  the served instruction places the build manifest and the lockfile under contracts, the toolchain pin under limits, a structural template under structure, a schema or migration asset under contracts, and served instruction text under behavior
    And  the served instruction states that a build input the declaration leaves outside the Specification roles is a specification defect, and a neighbor when it lies outside the replaced unit

  @givn.added
  Scenario: The ise instruction accepts an asset the rebuild generates inside the replaced unit
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction treats an asset the rebuild generates inside the replaced unit whose bytes no frozen test fixes as an acceptable undocumented change
    And  the served instruction states that a generated asset is not a neighbor

  @givn.added
  Scenario: The ise instruction keeps the dependency cache in the environment
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names the dependency cache and the installed toolchain as environment inputs, never declared files
    And  the served instruction states that a locked dependency the cache cannot resolve blocks the rebuild
    And  the served instruction states that the blocked rebuild is a setup failure, not a Spec gap
    And  the served instruction states that the blocked rebuild is not a specification defect

  @givn.added
  Scenario: The ise instruction requires a relied-on contract to enumerate its fixed values
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction states that a contract the frozen tests rely on enumerates every value it fixes

  @givn.added
  Scenario: The ise instruction keeps a published copy out of the clean room
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction requires the builder to read no product source outside the spec copy and the declared inputs
    And  the served instruction requires a scan of the local package registry's source, cache, and git stores
    And  the served instruction requires every registry hit to be quarantined outside the builder's reach
    And  the served instruction states that a store that does not exist has no hit
    And  the served instruction records the registry scan result in the neighbor record

  @givn.added
  Scenario: The ise instruction records a rebuild's neighbors and consolidations
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction records every file changed outside the declared Specification or the replaced unit
    And  the served instruction records every consolidation with its reason
    And  the served instruction states that the trial review records the neighbor record

  @givn.added
  Scenario: The ise instruction keeps a dry run out of the product
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction states that a Dry-Run Ise trial promotes nothing
    And  the served instruction states that the rebuilt work stays in the copy

  @givn.modified
  Scenario: The review guidance asks the neighboring-components question without a gate
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the review guidance
    Then  the command exits 0
    And  the served review guidance asks for the files changed outside the declared Specification or the replaced unit
    And  the served review guidance asks for every consolidation the rebuild made
    And  the served review guidance asks for the reason for each
    And  the served review guidance states that the answer is a review question, never a gate
