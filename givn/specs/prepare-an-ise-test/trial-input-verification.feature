@trial-input-verification
Feature: Trial input verification

  @e2e @trial-input-verification
  Scenario: Verifier re-checks a prepared trial and receives the frozen-input report
    Given  a prepared Git trial with a receipt
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero

  @trial-input-verification
  Scenario: Verification names a retained file changed in the trial
    Given  a prepared Git trial with a receipt
    When  the verifier changes the retained file "givn/specs/demo/spec.feature" in the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specs/demo/spec.feature"

  @trial-input-verification
  Scenario: Verification names an e2e package file changed in the trial
    Given  a prepared Git trial with a receipt
    When  the verifier copies the e2e file "tests/e2e/steps/keys.rs" into the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/e2e/steps/keys.rs"
    And  verification reports the trial holds a path the frozen package covers

  @trial-input-verification
  Scenario: Verification names an e2e package file changed in the checkout
    Given  a prepared Git trial with a receipt
    When  the developer changes the e2e file "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/e2e/steps/keys.rs"

  @trial-input-verification
  Scenario: Verification names an e2e package path the record does not list
    Given  a prepared Git trial with a receipt
    When  the developer adds the e2e file "tests/e2e/steps/extra.rs" to the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/e2e/steps/extra.rs"
    And  verification reports the e2e package gained the path

  @trial-input-verification
  Scenario: Verification names an e2e scenario changed in the checkout
    Given  a prepared Git trial with a receipt
    When  the developer changes a scenario tagged "e2e" in "givn/specs/demo/spec.feature"
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specs/demo/spec.feature"

  @trial-input-verification
  Scenario: Verification names an e2e scenario the record does not list
    Given  a prepared Git trial with a receipt
    When  the developer adds the e2e scenario file "givn/specs/demo/extra.feature" to the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specs/demo/extra.feature"
    And  verification reports the selected e2e scenarios differ

  @trial-input-verification
  Scenario: Verification reports no drift after a kept source file changes
    Given  a prepared Git trial with a receipt
    When  the verifier changes the kept source file "src/keys/steps.rs" in the trial
    And  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero

  @trial-input-verification
  Scenario: Verification names a recorded path missing from the trial
    Given  a prepared Git trial with a receipt
    When  the trial loses the retained file "givn/specs/demo/spec.feature"
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specs/demo/spec.feature"

  @trial-input-verification
  Scenario: Verification refuses while the ise addon is off
    Given  a prepared Git trial with a corrupted record
    And  the project disables ise after preparation
    When  the verifier re-checks the trial against the record
    Then  verification fails and names "ise"
    And  verification reports no record error

  @trial-input-verification
  Scenario: Verification refuses while architecture principles are off
    Given  a prepared Git trial with a corrupted record
    And  the project disables architecture principles after preparation
    When  the verifier re-checks the trial against the record
    Then  verification fails and names "architecture_principles"
    And  verification reports no record error

  @trial-input-verification
  Scenario: Verification refuses a record it cannot read
    Given  a prepared Git trial with a record that is not a trial record
    When  the verifier re-checks the trial against the record
    Then  verification fails and names the record
    And  verification names the expected record scope

  @trial-input-verification
  Scenario: Verification refuses a record with another schema
    Given  a prepared Git trial with a receipt carrying the schema 2
    When  the verifier re-checks the trial against the record
    Then  verification fails and names the record and the schema
    And  verification reports the expected record schema

  @trial-input-verification
  Scenario: Verification refuses a record without a replaced digest
    Given  a prepared Git trial with a receipt whose replaced digest is not a SHA-256 value
    When  the verifier re-checks the trial against the record
    Then  verification fails and names the record and the replaced digest
    And  verification reports the record is no trial evidence

  @trial-input-verification
  Scenario: Verification refuses a record with an unknown field
    Given  a prepared Git trial with a receipt carrying an unknown field
    When  the verifier re-checks the trial against the record
    Then  verification fails and names the record and the unknown field

  @trial-input-verification
  Scenario: Verification refuses a record with a missing field
    Given  a prepared Git trial with a receipt missing the counts field
    When  the verifier re-checks the trial against the record
    Then  verification fails and names the record and the missing field

  @trial-input-verification
  Scenario: Verification refuses a record placed inside the trial
    Given  a prepared Git trial with a receipt
    When  the verifier copies the record inside the trial and re-checks it there
    Then  verification fails and names the record path
    And  verification reports the record is inside the trial

  @trial-input-verification
  Scenario: Verification refuses a record whose written path is not the trial
    Given  a prepared Git trial with a receipt
    When  the verifier rewrites the record's written path and re-checks it against the trial
    Then  verification fails and names the record and the trial path
    And  verification reports the written path mismatch

  @trial-input-verification
  Scenario: Verification refuses a record whose backend differs from the selected backend
    Given  a prepared Git trial with a receipt
    When  the verifier rewrites the record's backend to "fossil" and re-checks it
    Then  verification fails and names the record and the backend
    And  verification reports the backend mismatch

  @trial-input-verification
  Scenario: Verification names every drifting path in one run
    Given  a prepared Git trial with a receipt
    When  the verifier changes the retained files "givn/specs/demo/spec.feature" and "docs/reference.md" in the trial
    And  the developer changes the e2e file "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names all three paths

  @trial-input-verification
  Scenario: Verification names a declaration changed in the trial
    Given  a prepared Git trial with a receipt
    When  the verifier changes the retained file "givn/specification.yaml" in the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specification.yaml"

  @trial-input-verification
  Scenario: Verification names a declaration missing from the trial
    Given  a prepared Git trial with a receipt
    When  the trial loses the retained file "givn/specification.yaml"
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "givn/specification.yaml"

  @trial-input-verification
  Scenario: Verification reports no drift after only the checkout's structure role changes
    Given  a prepared Git trial with a receipt
    When  the developer rewrites the checkout declaration's "structure" role to "docs/glossary.md"
    And  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero

  @trial-input-verification
  Scenario: Verification names the e2e package a checkout declaration's added root brings
    Given  a prepared Git trial with a receipt
    When  the developer adds the e2e root "tests/extra-e2e" to the checkout declaration
    And  the developer adds the e2e file "tests/extra-e2e/Cargo.toml" to the checkout
    And  the verifier re-checks the trial against the record
    Then  verification fails and names "tests/extra-e2e/Cargo.toml"
    And  verification reports the e2e package gained the path

  @trial-input-verification
  Scenario: Verification bounds the frozen paths it refuses
    Given  a prepared Git trial with a receipt whose frozen package holds twenty-five files
    When  the verifier copies the whole e2e package "tests/e2e" into the trial
    And  the verifier re-checks the trial against the record
    Then  verification fails and names the first twenty frozen paths in sorted order
    And  verification reports the count of the remaining frozen paths

  @trial-input-verification
  Scenario: Verification keeps a trial prepared before the package was excluded re-checkable
    Given  a prepared Git trial whose record and trial were written before the frozen package was excluded
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  the record names no excluded role
    And  the trial holds "tests/e2e/steps/keys.rs"
    And  verification exits zero

  @trial-input-verification
  Scenario: A frozen role that covers the declaration re-checks clean
    Given  a prepared Git trial whose frozen role covers the declaration
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification exits zero
