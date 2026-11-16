@givn.delta @trial-delivery-identity

Feature: Trial delivery identity

  @givn.added @e2e @wip
  Scenario: Developer records a trial's delivery identity and receives the delivery report
    Given  a prepared Git trial with a receipt
    And  the builder wrote the replacement "src/keys/parser.rs" in the trial
    When  the developer records the trial's delivery identity
    Then  the delivery report prints the trial path, the trial revision, the backend, "Retained: clean", "E2E: clean", and the delivery record path
    And  the delivery report prints "Delivered: 2" and an abbreviated delivered digest
    And  the delivery record lists "src/keys/parser.rs" and "src/keys/steps.rs" in its delivered set
    And  the delivery record lists no retained path
    And  the delivery record keeps the trial record's replaced digest
    And  the trial record is unchanged

  @givn.added @e2e @wip
  Scenario: Verifier re-checks a delivered trial and receives the delivery report
    Given  a prepared Git trial with a delivery record
    When  the verifier re-checks the trial in delivery mode
    Then  verification reports the retained, e2e, and delivery identities clean
    And  verification exits zero

  @givn.added @wip
  Scenario: Verification names every delivered file changed after the delivery was recorded
    Given  a prepared Git trial with a delivery record
    When  the verifier changes the delivered file "src/keys/parser.rs" in the trial
    And  the verifier changes the kept source file "src/keys/steps.rs" in the trial
    And  the verifier re-checks the trial in delivery mode
    Then  verification fails and names "src/keys/parser.rs" and "src/keys/steps.rs"
    And  verification reports the delivered content differs

  @givn.added @wip
  Scenario: Verification names a path the builder added after the delivery was recorded
    Given  a prepared Git trial with a delivery record
    When  the builder adds "src/keys/views.rs" to the trial
    And  the verifier re-checks the trial in delivery mode
    Then  verification fails and names "src/keys/views.rs"
    And  verification reports the trial gained the path

  @givn.added @wip
  Scenario: Verification names a delivered path missing from the trial
    Given  a prepared Git trial with a delivery record
    When  the trial loses the delivered file "src/keys/parser.rs"
    And  the verifier re-checks the trial in delivery mode
    Then  verification fails and names "src/keys/parser.rs"
    And  verification reports the delivered path is missing from the trial

  @givn.added @wip
  Scenario: Verification refuses a record that holds no delivery identity
    Given  a prepared Git trial with a receipt
    When  the verifier re-checks the trial in delivery mode
    Then  verification fails and names the record
    And  verification reports the record holds no delivery identity

  @givn.added @wip
  Scenario: Verification reports the delivery identity as not checked without the delivery flag
    Given  a prepared Git trial with a delivery record
    When  the verifier re-checks the trial against the record
    Then  verification reports the retained and e2e identities clean
    And  verification reports the delivery identity is not checked
    And  verification exits zero

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a trial whose frozen inputs drifted
    Given  a prepared Git trial with a receipt
    And  the verifier changes the retained file "givn/specs/demo/spec.feature" in the trial
    When  the developer records the trial's delivery identity
    Then  recording fails and names "givn/specs/demo/spec.feature"
    And  the delivery record path remains absent

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a record that already holds one
    Given  a prepared Git trial with a delivery record
    When  the developer records the trial's delivery identity again
    Then  recording fails and names the delivery record
    And  recording reports the record already holds a delivery identity
    And  the second delivery record path remains absent

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a record that names another trial
    Given  a prepared Git trial with a receipt
    And  the record's written path is rewritten to another directory
    When  the developer records the trial's delivery identity
    Then  recording fails and names the written path and the trial
    And  the delivery record path remains absent

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a receipt inside the trial
    Given  a prepared Git trial with a receipt
    When  the developer records the trial's delivery identity with a receipt inside the trial
    Then  recording fails and names the receipt
    And  the delivery record path remains absent

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a receipt inside the checkout
    Given  a prepared Git trial with a receipt
    When  the developer records the trial's delivery identity with a receipt inside the checkout
    Then  recording fails and names the receipt
    And  the checkout holds no delivery record

  @givn.added @wip
  Scenario: Recording a delivery identity refuses an existing receipt
    Given  a prepared Git trial with a receipt
    And  a delivery receipt path that already exists
    When  the developer records the trial's delivery identity
    Then  recording fails and names the existing receipt
    And  the existing receipt is left unchanged

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a missing trial directory
    Given  a prepared Git trial with a receipt
    And  the trial directory is removed after preparation
    When  the developer records the trial's delivery identity
    Then  recording fails and names the trial directory
    And  the delivery record path remains absent

  @givn.added @wip
  Scenario: Recording a delivery identity refuses a record it cannot read
    Given  a prepared Git trial with a record that is not a trial record
    When  the developer records the trial's delivery identity
    Then  recording fails and names the record
    And  the delivery record path remains absent

  @givn.added @wip
  Scenario: Verification refuses a delivery identity that is not a SHA-256 value
    Given  a prepared Git trial with a delivery record whose delivered digest is not a SHA-256 value
    When  the verifier re-checks the trial in delivery mode
    Then  verification fails and names the record and the delivered digest
