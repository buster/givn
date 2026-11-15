@givn.delta @trial-input-verification

Feature: Trial input verification

  @givn.added @wip
  Scenario: Verification names a declaration changed in the trial
    Given  a prepared Git trial with a receipt
    When  the verifier changes the retained file "givn/specification.yaml" in the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specification.yaml"

  @givn.added @wip
  Scenario: Verification names a declaration missing from the trial
    Given  a prepared Git trial with a receipt
    When  the trial loses the retained file "givn/specification.yaml"
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specification.yaml"

  @givn.added @wip
  Scenario: Verification reports no drift after only the checkout's structure role changes
    Given  a prepared Git trial with a receipt
    When  the developer rewrites the checkout declaration's "structure" role to "docs/glossary.md"
    And  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero

  @givn.added @wip
  Scenario: Verification names the e2e package a checkout declaration's added root brings
    Given  a prepared Git trial with a receipt
    When  the developer adds the e2e root "tests/extra-e2e" to the checkout declaration
    And  the developer adds the e2e file "tests/extra-e2e/Cargo.toml" to the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/extra-e2e/Cargo.toml"
    And  verification reports the e2e package gained the path
