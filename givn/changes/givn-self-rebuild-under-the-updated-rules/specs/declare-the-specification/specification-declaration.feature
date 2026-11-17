@givn.delta @specification-declaration
Feature: specification-declaration

  @givn.added
  Scenario: A declaration that names build inputs under the four content roles is accepted
    Given  an ise-enabled project selecting opencode whose declaration places build inputs under the four content roles
    When  the consumer runs `givn lint`
    Then  the command exits 0
    And  stderr contains no declaration-schema error
    And  stderr contains no declared-path advisory for the build inputs
