@givn.delta @trial-input-verification

Feature: Trial input verification

  @givn.added @e2e @wip
  Scenario: Verifier re-checks a prepared trial and receives the frozen-input report
    Given a prepared Git trial with a receipt
    When the verifier re-checks the trial against the record
    Then verification reports the retained and e2e identities clean
    And verification exits zero

  @givn.added @wip
  Scenario: Verification names a retained file changed in the trial
    Given a prepared Git trial with a receipt
    When the verifier changes the retained file "givn/specs/demo/spec.feature" in the trial
    And the verifier re-checks the trial against the record
    Then verification fails and names "givn/specs/demo/spec.feature"

  @givn.added @wip
  Scenario: Verification names an e2e package file changed in the trial
    Given a prepared Git trial with a receipt
    When the verifier changes the e2e file "tests/e2e/steps/keys.rs" in the trial
    And the verifier re-checks the trial against the record
    Then verification fails and names "tests/e2e/steps/keys.rs"

  @givn.added @wip
  Scenario: Verification names an e2e package file changed in the checkout
    Given a prepared Git trial with a receipt
    When the developer changes the e2e file "tests/e2e/steps/keys.rs" in the checkout
    And the verifier re-checks the trial against the record
    Then verification fails and names "tests/e2e/steps/keys.rs"

  @givn.added @wip
  Scenario: Verification names an e2e scenario changed in the checkout
    Given a prepared Git trial with a receipt
    When the developer changes a scenario tagged "e2e" in "givn/specs/demo/spec.feature"
    And the verifier re-checks the trial against the record
    Then verification fails and names "givn/specs/demo/spec.feature"

  @givn.added @wip
  Scenario: Verification reports no drift after a kept source file changes
    Given a prepared Git trial with a receipt
    When the verifier changes the kept source file "src/keys/steps.rs" in the trial
    And the verifier re-checks the trial against the record
    Then verification reports the retained and e2e identities clean
    And verification exits zero

  @givn.added @wip
  Scenario: Verification names a recorded path missing from the trial
    Given a prepared Git trial with a receipt
    When the trial loses the retained file "givn/specs/demo/spec.feature"
    And the verifier re-checks the trial against the record
    Then verification fails and names "givn/specs/demo/spec.feature"

  @givn.added @wip
  Scenario: Verification refuses while the ise addon is off
    Given a prepared Git trial with a corrupted record
    And the project disables ise after preparation
    When the verifier re-checks the trial against the record
    Then verification fails and names "ise"
    And verification reports no record error

  @givn.added @wip
  Scenario: Verification refuses while architecture principles are off
    Given a prepared Git trial with a receipt
    And the project disables architecture principles after preparation
    When the verifier re-checks the trial against the record
    Then verification fails and names "architecture_principles"

  @givn.added @wip
  Scenario: Verification refuses a record it cannot read
    Given a prepared Git trial with a record that is not a trial record
    When the verifier re-checks the trial against the record
    Then verification fails and names the record
    And verification names the expected record scope

  @givn.added @wip
  Scenario: Verification refuses a record with an unknown field
    Given a prepared Git trial with a receipt carrying an unknown field
    When the verifier re-checks the trial against the record
    Then verification fails and names the record and the unknown field

  @givn.added @wip
  Scenario: Verification refuses a record with a missing field
    Given a prepared Git trial with a receipt missing the counts field
    When the verifier re-checks the trial against the record
    Then verification fails and names the record and the missing field

  @givn.added @wip
  Scenario: Verification refuses a record placed inside the trial
    Given a prepared Git trial with a receipt
    When the verifier copies the record inside the trial and re-checks it there
    Then verification fails and names the record path
    And verification reports the record is inside the trial

  @givn.added @wip
  Scenario: Verification refuses a record whose written path is not the trial
    Given a prepared Git trial with a receipt
    When the verifier rewrites the record's written path and re-checks it against the trial
    Then verification fails and names the record and the trial path
    And verification reports the written path mismatch

  @givn.added @wip
  Scenario: Verification refuses a record whose backend differs from the selected backend
    Given a prepared Git trial with a receipt
    When the verifier rewrites the record's backend to "fossil" and re-checks it
    Then verification fails and names the record and the backend
    And verification reports the backend mismatch

  @givn.added @wip
  Scenario: Verification names every drifting path in one run
    Given a prepared Git trial with a receipt
    When the verifier changes the retained files "givn/specs/demo/spec.feature" and "docs/reference.md" and the e2e file "tests/e2e/steps/keys.rs" in the trial
    And the verifier re-checks the trial against the record
    Then verification fails and names all three paths
