Feature: Remediation log for semantic review

  Scenario: The first blocked review creates the remediation log with before evidence
    Given  a staged "DUPLICATE" case with a complete run declaration
    When  the review gate runs
    Then  the command exits non-zero
    And  the log exists
    And  the log declares schema version 1 and the change id
    And  the log records the before run stage durations
    And  the log records the candidate id and embedding score
    And  the entry records the BGE and NLI states from the report
    And  the entry records the scenario text verbatim
    And  the entry records the endpoint hashes

  Scenario: Rerunning review with unchanged endpoints does not grow the remediation log
    Given  a staged "DUPLICATE" case with a complete run declaration
    When  the review gate runs
    And  the review gate reruns
    Then  the log still holds one candidate entry and one run record

  Scenario Outline: An invalid remediation evidence state blocks the review
    Given  a remediation-log project with one "DUPLICATE" finding <repository-state> and a complete run declaration
    When  the review gate runs
    And  the resolved duplicate is stamped and withdrawn
    And  the remediation evidence is left <evidence-state>
    And  the gate reruns
    Then  the run fails
    And  the diagnostic names "<expected-diagnostic>"
    Examples:
      | repository-state | evidence-state | expected-diagnostic |
      | no git | no evidence | remediation log |
      | committed | no stamp | semantic-remediation |
      | committed | foreign stamp | semantic-remediation |
      | committed | unreachable commit | reachable |

  Scenario Outline: A verified remediation whose log record was <log-state> still passes review
    Given  a duplicate-log project with a complete run declaration
    When  the review gate runs
    And  the resolved duplicate "removed the twin" is recorded ["4d5e6f70"]
    And  the remediation log is <log-state>
    And  the redundant scenario is deleted from the delta
    And  the review gate reruns
    Then  the review passes with the remediation receipt
    Examples:
      | log-state |
      | deleted |
      | empty |

  Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
    Given  a remediation-log project with one "DUPLICATE" finding <repository-state> and a complete run declaration
    When  the review gate runs
    And  the resolved duplicate "took the surplus out" is recorded ["3c4d5e6f"]
    And  the doubled scenario is withdrawn
    Then  the log records a null before run anchor
    And  the gate reruns
    Then  the review passes with the remediation receipt
    Examples:
      | repository-state |
      | no git |
      | unborn |

  Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
    Given  a staged "DUPLICATE" case with a complete run declaration
    When  the review gate runs
    And  the resolved duplicate "removed the duplicate" is recorded ["1a2b3c4d"]
    And  the classified duplicate is removed
    And  the review gate reruns
    Then  the review passes with the remediation receipt
    And  the log records the after run and the "endpoint-absent" state with a null after score
    And  the log is unchanged after a further review

  Scenario: A changed endpoint appends a candidate entry and receives the verified after state
    Given  a staged "DUPLICATE" case with a complete run declaration
    When  the review gate runs
    And  the assessor resolves the candidate to "DUPLICATE"
    And  the classified endpoint changes
    And  the gate reruns
    Then  the run fails
    And  a second entry is appended and the prior entry is retained
    And  the resolved duplicate "removed the superseded twin" is recorded ["708192a3"]
    And  the doubled scenario is withdrawn
    And  the gate reruns
    Then  the review passes with the remediation receipt
    And  the second entry records its after state

  Scenario: A surviving pair records its recomputed embedding score after remediation
    Given  a staged "VARIANT" case with a complete run declaration
    When  the review gate runs
    And  the assessor resolves the candidate to "VARIANT"
    And  the user records the fix "consolidated" and commits ["2b3c4d5e"]
    And  the corpus variant is consolidated
    And  the review gate reruns
    Then  the command exits 0
    And  the log records the "outline-consolidated" after state with a recomputed score below the boundary
    And  the candidate entry records the consolidated outline text verbatim from the active delta

  @rulebook
  Scenario: A logged pair scored by BGE and NLI records their evidence states
    Given  a staged "DUPLICATE" case with a complete run declaration
    When  the review gate runs
    Then  the command exits non-zero
    And  the log records non-null BGE and NLI states

  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given  a project prepared for semantic review with two permanent-duplicate deltas in a Git repository
    And  the run declaration is complete
    When  the review gate runs
    And  the assessor resolves the candidate to "DUPLICATE"
    And  both classified duplicates are removed
    And  the user commits the remediation with one matching stamp per candidate
    And  both fixes are recorded in the log
    And  the review gate reruns
    Then  the review passes with the remediation receipt

  Scenario: The remediation log obeys its strict wire grammar
    Given  the remediation case carries the distinguishing marker "cobalt elm locomotive lumen piston ridge summit wharf"
    Given  a project prepared for semantic remediation whose delta scenario contains blank lines and an Examples table
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the remediation log has the exact headers and entry field order
    And  the remediation log contains only allowed fields and no unknown fields
    And  absent values are represented only by "null"
    And  the remediation log has no timestamp or environment fields
    And  the remediation log ends with exactly one final newline
    And  the verbatim scenario text preserves interior blank lines and the Examples table

  Scenario: Failed review before the semantic stage does not create the remediation log
    Given  the remediation case carries the distinguishing marker "eucalyptus fennel inlet moss needle nutmeg xylophone yacht"
    Given  a project prepared for semantic review with a malformed delta feature
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the remediation log does not exist

  Scenario: The remediation log survives the archive move
    Given  the remediation case carries the distinguishing marker "accordion acorn jackal knoll lattice lumen orchard umber"
    Given  a completed semantic remediation change with a generated log and archive prerequisites
    And  the active remediation log bytes are snapshotted
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change running-change`
    Then  the command exits 0
    And  `givn/archive/running-change/semantic-remediation.md` exists
    And  the archived remediation log bytes equal the active log snapshot

  Scenario: Advisory spec review does not create the remediation log
    Given  the remediation case carries the distinguishing marker "clover kiln nettle nutmeg rivet trowel tulip wharf"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs the advisory semantic review command
    Then  the command exits 0
    And  stdout contains "retrieval advisory"
    And  the remediation log does not exist

  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given  the remediation case carries the distinguishing marker "beacon fern heather oarlock oboe thistle trowel vest"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs change review through a build without spec retrieval
    Then  the command exits non-zero
    And  stderr contains "spec retrieval is unavailable"
    And  the remediation log does not exist

  @e2e
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given  the remediation case carries the distinguishing marker "abacus anchor apricot cleat domino dune ember glacier"
    Given  an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When  the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then  the review passes with the remediation receipt
    And  the remediation log records the before and after run records with stage durations and the stamped commit

  @e2e
  Scenario: Real CLI archive retains the remediation log
    Given  the remediation case carries the distinguishing marker "abacus atlas domino eddy needle piston valve zinnia"
    Given  an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When  the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then  the command exits 0
    And  the archived remediation log exists next to the archived semantic report
