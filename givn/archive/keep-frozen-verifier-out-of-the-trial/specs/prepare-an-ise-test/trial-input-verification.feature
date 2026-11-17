@givn.delta @trial-input-verification

Feature: Trial input verification

  @givn.modified
  Scenario: Verification names an e2e package file changed in the trial
    Given  a prepared Git trial with a receipt
    When  the verifier copies the e2e file "tests/e2e/steps/keys.rs" into the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/e2e/steps/keys.rs"
    And  verification reports the trial holds a path the frozen package covers

  @givn.modified
  Scenario: Verification names every drifting path in one run
    Given  a prepared Git trial with a receipt
    When  the verifier changes the retained files "givn/specs/demo/spec.feature" and "docs/reference.md" in the trial
    And  the developer changes the e2e file "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names all three paths

  @givn.added
  Scenario: Verification bounds the frozen paths it refuses
    Given  a prepared Git trial with a receipt whose frozen package holds twenty-five files
    When  the verifier copies the whole e2e package "tests/e2e" into the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names the first twenty frozen paths in sorted order
    And  verification reports the count of the remaining frozen paths

  @givn.added
  Scenario: Verification keeps a trial prepared before the package was excluded re-checkable
    Given  a prepared Git trial whose record and trial were written before the frozen package was excluded
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  the record names no excluded role
    And  the trial holds "tests/e2e/steps/keys.rs"
    And  verification exits zero

  @givn.added
  Scenario: A frozen role that covers the declaration re-checks clean
    Given  a prepared Git trial whose frozen role covers the declaration
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero
