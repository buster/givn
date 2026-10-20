@givn.delta @archive-proof

Feature: Archive Proof

  @givn.added
  Scenario: CLI interaction transcripts are declared evidence
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change has an uncommitted evidence file "evidence/cli/session.txt"
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change verify-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the archived change contains the evidence file "evidence/cli/session.txt"
