@givn.delta @archive-proof

Feature: Archive Proof

  @givn.added
  Scenario: Archive refuses a stale change verification receipt before merging
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change has a current verification receipt
    And  the delta specs changed after the receipt
    When  the user runs `givn archive --change verify-demo` without committing the fixture
    Then  the command exits non-zero
    And  stderr says the verification receipt is stale
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "verify-demo"

  @givn.added
  Scenario: Archive reuses the change receipt and runs only the merged suite
    Given  a clean committed givn project with a change whose delta specs pass
    And  the runner records every invocation with its scope and features path
    When  the user runs `givn archive --change verify-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the runner was invoked once for the regular scope with features path "givn/specs"
    And  the runner was never invoked with a change features path
