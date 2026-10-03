# Semantic Remediation Log
Log schema version: 1
Change: "semantic-remediation-log"

## Run 1
- kind: before
- commit: fbe1773413b6bb71eadf55d2a36a438fa979b3b0
- e5_runtime_ms: 20450
- bge_runtime_ms: 20206
- nli_runtime_ms: 17264

## Run 2
- kind: before
- commit: fbe1773413b6bb71eadf55d2a36a438fa979b3b0
- e5_runtime_ms: 18458
- bge_runtime_ms: 30051
- nli_runtime_ms: 23630

## Run 3
- kind: before
- commit: fbe1773413b6bb71eadf55d2a36a438fa979b3b0
- e5_runtime_ms: 18177
- bge_runtime_ms: 24692
- nli_runtime_ms: 17479

## Candidate 28e1116cba1665bf
- run: 1
- before_embedding_score: 0.9754
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "An invalid remediation evidence state blocks the review"
- before_left_content_hash: b683f2725dccdc9a
- before_right_content_hash: beaea9de28de27a7
- before_left_evidence_hash: 1bbc0b1112bad481
- before_right_evidence_hash: c994ca4d220f3f5a
- before_left_gherkin: |
      Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "took the surplus out" and the commits ["3c4d5e6f"]
        And the doubled scenario is withdrawn
        Then the log records a null before run anchor
        And the gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | repository-state                  |
          | without Git history               |
          | inside an unborn Git repository   |
- before_right_gherkin: |
      Scenario Outline: An invalid remediation evidence state blocks the review
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the doubled scenario is withdrawn
        And the remediation evidence is left <evidence-state>
        And the gate reruns
        Then the run fails
        And the diagnostic names "<expected-diagnostic>"
    
        Examples:
          | repository-state                  | evidence-state                           | expected-diagnostic  |
          | without Git history               | without commit evidence                  | remediation log      |
          | inside a committed Git repository | without the stamp                        | semantic-remediation |
          | inside a committed Git repository | with a stamp naming another candidate    | semantic-remediation |
          | inside a committed Git repository | with its commit removed from the history | reachable            |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 2a6ee67b082c8530
- run: 1
- before_embedding_score: 0.9727
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "A verified remediation whose log record was <log-state> still passes review"
- before_left_content_hash: b683f2725dccdc9a
- before_right_content_hash: ff451cce0cf19b0c
- before_left_evidence_hash: 1bbc0b1112bad481
- before_right_evidence_hash: 3a8f9075430e729f
- before_left_gherkin: |
      Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "took the surplus out" and the commits ["3c4d5e6f"]
        And the doubled scenario is withdrawn
        Then the log records a null before run anchor
        And the gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | repository-state                  |
          | without Git history               |
          | inside an unborn Git repository   |
- before_right_gherkin: |
      Scenario Outline: A verified remediation whose log record was <log-state> still passes review
        Given a duplicate-log project contains a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the operator records the fix "removed the twin" with commits ["4d5e6f70"]
        And the remediation log is <log-state>
        And the redundant scenario is deleted from the delta
        And the review gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | log-state                         |
          | deleted entirely                  |
          | stripped of its candidate entries |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 4fb206fa86438bed
- run: 1
- before_embedding_score: 0.9705
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A verified remediation whose log record was <log-state> still passes review"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "An invalid remediation evidence state blocks the review"
- before_left_content_hash: ff451cce0cf19b0c
- before_right_content_hash: beaea9de28de27a7
- before_left_evidence_hash: 3a8f9075430e729f
- before_right_evidence_hash: c994ca4d220f3f5a
- before_left_gherkin: |
      Scenario Outline: A verified remediation whose log record was <log-state> still passes review
        Given a duplicate-log project contains a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the operator records the fix "removed the twin" with commits ["4d5e6f70"]
        And the remediation log is <log-state>
        And the redundant scenario is deleted from the delta
        And the review gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | log-state                         |
          | deleted entirely                  |
          | stripped of its candidate entries |
- before_right_gherkin: |
      Scenario Outline: An invalid remediation evidence state blocks the review
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the doubled scenario is withdrawn
        And the remediation evidence is left <evidence-state>
        And the gate reruns
        Then the run fails
        And the diagnostic names "<expected-diagnostic>"
    
        Examples:
          | repository-state                  | evidence-state                           | expected-diagnostic  |
          | without Git history               | without commit evidence                  | remediation log      |
          | inside a committed Git repository | without the stamp                        | semantic-remediation |
          | inside a committed Git repository | with a stamp naming another candidate    | semantic-remediation |
          | inside a committed Git repository | with its commit removed from the history | reachable            |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 4ac750f855575c43
- run: 1
- before_embedding_score: 0.9585
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A commit fixing two candidates carries one stamp line per candidate"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "An invalid remediation evidence state blocks the review"
- before_left_content_hash: bb87cef96f9f6efa
- before_right_content_hash: beaea9de28de27a7
- before_left_evidence_hash: a953834690a41300
- before_right_evidence_hash: c994ca4d220f3f5a
- before_left_gherkin: |
      @givn.added
      Scenario: A commit fixing two candidates carries one stamp line per candidate
        Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And both classified duplicates are removed
        And the user commits the remediation with one matching stamp per candidate
        And both fixes are recorded in the log
        And the review gate reruns
        Then the review passes with the remediation receipt
- before_right_gherkin: |
      Scenario Outline: An invalid remediation evidence state blocks the review
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the doubled scenario is withdrawn
        And the remediation evidence is left <evidence-state>
        And the gate reruns
        Then the run fails
        And the diagnostic names "<expected-diagnostic>"
    
        Examples:
          | repository-state                  | evidence-state                           | expected-diagnostic  |
          | without Git history               | without commit evidence                  | remediation log      |
          | inside a committed Git repository | without the stamp                        | semantic-remediation |
          | inside a committed Git repository | with a stamp naming another candidate    | semantic-remediation |
          | inside a committed Git repository | with its commit removed from the history | reachable            |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 8e2bb18bc6bd4ed4
- run: 1
- before_embedding_score: 0.9579
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_left_content_hash: 79cf1fa69148868e
- before_right_content_hash: f3f67014f117a5b1
- before_left_evidence_hash: f1470e8e09a9653b
- before_right_evidence_hash: 01106f5b3642cc51
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the fixture carries the orchard corpus:
          """
- before_right_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the fixture carries the ferry corpus:
          """
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 1de68484dfbf3b9c
- run: 1
- before_embedding_score: 0.9551
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A surviving pair records its recomputed embedding score after remediation"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- before_left_content_hash: 15c69636bb7c9c13
- before_right_content_hash: 29df3ccef6da95d1
- before_left_evidence_hash: 68b8f3a35fab06c4
- before_right_evidence_hash: 3735d18208d45d22
- before_left_gherkin: |
      @givn.added
      Scenario: A surviving pair records its recomputed embedding score after remediation
        Given the fixture carries the beehive corpus:
          """
- before_right_gherkin: |
      @givn.added
      Scenario: Rerunning review with unchanged endpoints does not grow the remediation log
        Given the fixture carries the bakery corpus:
          """
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate d97b56c16f480050
- run: 1
- before_embedding_score: 0.9527
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A surviving pair records its recomputed embedding score after remediation"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_left_content_hash: 15c69636bb7c9c13
- before_right_content_hash: f3f67014f117a5b1
- before_left_evidence_hash: 68b8f3a35fab06c4
- before_right_evidence_hash: 01106f5b3642cc51
- before_left_gherkin: |
      @givn.added
      Scenario: A surviving pair records its recomputed embedding score after remediation
        Given the fixture carries the beehive corpus:
          """
- before_right_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the fixture carries the ferry corpus:
          """
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 97bae1b6243967f6
- run: 1
- before_embedding_score: 0.9519
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A surviving pair records its recomputed embedding score after remediation"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The first blocked review creates the remediation log with before evidence"
- before_left_content_hash: 15c69636bb7c9c13
- before_right_content_hash: 2e997f6f931af21f
- before_left_evidence_hash: 68b8f3a35fab06c4
- before_right_evidence_hash: b185aff85892efd3
- before_left_gherkin: |
      @givn.added
      Scenario: A surviving pair records its recomputed embedding score after remediation
        Given the fixture carries the beehive corpus:
          """
- before_right_gherkin: |
      @givn.added
      Scenario: The first blocked review creates the remediation log with before evidence
        Given the fixture carries the telescope corpus:
          """
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 8e2bb18bc6bd4ed4
- run: 2
- before_embedding_score: 0.9756
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: 93d15acc59bffa08
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: 259f86fa2c55838c
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the user records the fix "removed the duplicate" and commits ["1a2b3c4d"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the log contains an after run record
        And the removed entry records after state "endpoint-absent" with a null after score
        And the log bytes are snapshotted
        And the review gate reruns
        Then the log bytes equal the snapshot
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 7ead621ce6588126
- run: 2
- before_embedding_score: 0.9668
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "The first blocked review creates the remediation log with before evidence"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_left_content_hash: 520b72e1135fe93a
- before_right_content_hash: 93d15acc59bffa08
- before_left_evidence_hash: 144cce5f42283d76
- before_right_evidence_hash: 259f86fa2c55838c
- before_left_gherkin: |
      @givn.added
      Scenario: The first blocked review creates the remediation log with before evidence
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        Then the command exits non-zero
        And the log exists
        And the log declares schema version 1 and the change id
        And the log records the before run stage durations
        And the log records the candidate id and embedding score
        And the entry records the BGE and NLI states from the report
        And the entry records the scenario text verbatim
        And the entry records the endpoint hashes
- before_right_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the user records the fix "removed the duplicate" and commits ["1a2b3c4d"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the log contains an after run record
        And the removed entry records after state "endpoint-absent" with a null after score
        And the log bytes are snapshotted
        And the review gate reruns
        Then the log bytes equal the snapshot
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate ed43a91172e80e92
- run: 2
- before_embedding_score: 0.9645
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The first blocked review creates the remediation log with before evidence"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: 520b72e1135fe93a
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: 144cce5f42283d76
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: The first blocked review creates the remediation log with before evidence
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        Then the command exits non-zero
        And the log exists
        And the log declares schema version 1 and the change id
        And the log records the before run stage durations
        And the log records the candidate id and embedding score
        And the entry records the BGE and NLI states from the report
        And the entry records the scenario text verbatim
        And the entry records the endpoint hashes
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate ba2bd9d7bc9c7b2f
- run: 2
- before_embedding_score: 0.9639
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "A verified remediation whose log record was <log-state> still passes review"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: ff451cce0cf19b0c
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: 3a8f9075430e729f
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      Scenario Outline: A verified remediation whose log record was <log-state> still passes review
        Given a duplicate-log project contains a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the operator records the fix "removed the twin" with commits ["4d5e6f70"]
        And the remediation log is <log-state>
        And the redundant scenario is deleted from the delta
        And the review gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | log-state                         |
          | deleted entirely                  |
          | stripped of its candidate entries |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 47d5f65db8e6f2b7
- run: 2
- before_embedding_score: 0.9638
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-review/semantic-review.feature"
- before_right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- before_left_content_hash: 93d15acc59bffa08
- before_right_content_hash: 32a79ccdc6bcb650
- before_left_evidence_hash: 259f86fa2c55838c
- before_right_evidence_hash: 6ec338705bce9f0b
- before_left_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the user records the fix "removed the duplicate" and commits ["1a2b3c4d"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the log contains an after run record
        And the removed entry records after state "endpoint-absent" with a null after score
        And the log bytes are snapshotted
        And the review gate reruns
        Then the log bytes equal the snapshot
- before_right_gherkin: |
      @givn.modified
      Scenario: Removing a DUPLICATE candidate verifies the projected remediation
        Given the remediation fixture stages a "DUPLICATE" case
        And the permanent specs are snapshotted
        When the review gate runs
        Then the command exits non-zero
        When the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed" and the commits ["0a1b2c40"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the permanent specs are unchanged
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate da1b0f211fdeec9d
- run: 2
- before_embedding_score: 0.9628
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "A commit fixing two candidates carries one stamp line per candidate"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: bb87cef96f9f6efa
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: a953834690a41300
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: A commit fixing two candidates carries one stamp line per candidate
        Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And both classified duplicates are removed
        And the user commits the remediation with one matching stamp per candidate
        And both fixes are recorded in the log
        And the review gate reruns
        Then the review passes with the remediation receipt
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 8e0d3d4d515f01f7
- run: 2
- before_embedding_score: 0.9628
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-review/semantic-review.feature"
- before_right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: 32a79ccdc6bcb650
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: 6ec338705bce9f0b
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.modified
      Scenario: Removing a DUPLICATE candidate verifies the projected remediation
        Given the remediation fixture stages a "DUPLICATE" case
        And the permanent specs are snapshotted
        When the review gate runs
        Then the command exits non-zero
        When the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed" and the commits ["0a1b2c40"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the permanent specs are unchanged
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate e45e0efe48b4c0f4
- run: 2
- before_embedding_score: 0.9605
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "A surviving pair records its recomputed embedding score after remediation"
- before_left_content_hash: a0f8444a99c69b6e
- before_right_content_hash: f9e965d65fbe5cce
- before_left_evidence_hash: b746a8b70b8cbe91
- before_right_evidence_hash: a055f9b0a90c0b15
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the assessor resolves the candidate to "DUPLICATE"
        And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: A surviving pair records its recomputed embedding score after remediation
        Given the remediation fixture stages a "VARIANT" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "VARIANT"
        And the user records the fix "consolidated" and commits ["2b3c4d5e"]
        And the corpus variant is consolidated
        And the review gate reruns
        Then the command exits 0
        And the log records the "outline-consolidated" after state with a recomputed score below the boundary
        And the candidate entry records the consolidated outline text verbatim from the active delta
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 8e2bb18bc6bd4ed4
- run: 3
- before_embedding_score: 0.9747
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- before_left_content_hash: f7f247165188002b
- before_right_content_hash: b42b6bc87250ec7c
- before_left_evidence_hash: 2ac94b3869cbd261
- before_right_evidence_hash: 51467e8d237cb983
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the resolved duplicate "removed the superseded twin" is recorded ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the resolved duplicate "removed the duplicate" is recorded ["1a2b3c4d"]
        And the classified duplicate is removed
        And the review gate reruns
        Then the review passes with the remediation receipt
        And the log records the after run and the "endpoint-absent" state with a null after score
        And the log is unchanged after a further review
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate ed43a91172e80e92
- run: 3
- before_embedding_score: 0.9677
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "The first blocked review creates the remediation log with before evidence"
- before_left_content_hash: f7f247165188002b
- before_right_content_hash: 520b72e1135fe93a
- before_left_evidence_hash: 2ac94b3869cbd261
- before_right_evidence_hash: 144cce5f42283d76
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the resolved duplicate "removed the superseded twin" is recorded ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: The first blocked review creates the remediation log with before evidence
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        Then the command exits non-zero
        And the log exists
        And the log declares schema version 1 and the change id
        And the log records the before run stage durations
        And the log records the candidate id and embedding score
        And the entry records the BGE and NLI states from the report
        And the entry records the scenario text verbatim
        And the entry records the endpoint hashes
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 28e1116cba1665bf
- run: 3
- before_embedding_score: 0.9655
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "An invalid remediation evidence state blocks the review"
- before_left_content_hash: 2effba96dd65ed3c
- before_right_content_hash: f846b9dd6b25aa0f
- before_left_evidence_hash: d943a068c0f0fb0b
- before_right_evidence_hash: 3af473db541dd556
- before_left_gherkin: |
      Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the resolved duplicate "took the surplus out" is recorded ["3c4d5e6f"]
        And the doubled scenario is withdrawn
        Then the log records a null before run anchor
        And the gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | repository-state |
          | no git           |
          | unborn           |
- before_right_gherkin: |
      Scenario Outline: An invalid remediation evidence state blocks the review
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the resolved duplicate is stamped and withdrawn
        And the remediation evidence is left <evidence-state>
        And the gate reruns
        Then the run fails
        And the diagnostic names "<expected-diagnostic>"
    
        Examples:
          | repository-state | evidence-state     | expected-diagnostic  |
          | no git           | no evidence        | remediation log      |
          | committed        | no stamp           | semantic-remediation |
          | committed        | foreign stamp      | semantic-remediation |
          | committed        | unreachable commit | reachable            |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate 4fb206fa86438bed
- run: 3
- before_embedding_score: 0.9644
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A verified remediation whose log record was <log-state> still passes review"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "An invalid remediation evidence state blocks the review"
- before_left_content_hash: 58e62ce5c67db95e
- before_right_content_hash: f846b9dd6b25aa0f
- before_left_evidence_hash: 8aec14abf3570d5b
- before_right_evidence_hash: 3af473db541dd556
- before_left_gherkin: |
      Scenario Outline: A verified remediation whose log record was <log-state> still passes review
        Given a duplicate-log project contains a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the resolved duplicate "removed the twin" is recorded ["4d5e6f70"]
        And the remediation log is <log-state>
        And the redundant scenario is deleted from the delta
        And the review gate reruns
        Then the review passes with the remediation receipt
    
        Examples:
          | log-state |
          | deleted   |
          | empty     |
- before_right_gherkin: |
      Scenario Outline: An invalid remediation evidence state blocks the review
        Given a remediation-log project with one "DUPLICATE" finding <repository-state>
        And the run declaration is complete
        When the review gate runs
        And the resolved duplicate is stamped and withdrawn
        And the remediation evidence is left <evidence-state>
        And the gate reruns
        Then the run fails
        And the diagnostic names "<expected-diagnostic>"
    
        Examples:
          | repository-state | evidence-state     | expected-diagnostic  |
          | no git           | no evidence        | remediation log      |
          | committed        | no stamp           | semantic-remediation |
          | committed        | foreign stamp      | semantic-remediation |
          | committed        | unreachable commit | reachable            |
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null

## Candidate e45e0efe48b4c0f4
- run: 3
- before_embedding_score: 0.9639
- before_bge_admission: BGE_TOKEN_CAP
- before_bge_symmetric_raw: null
- before_nli_admission: NLI_TOKEN_CAP
- before_nli_symmetric_entailment: null
- before_left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- before_right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- before_right_title: "A surviving pair records its recomputed embedding score after remediation"
- before_left_content_hash: f7f247165188002b
- before_right_content_hash: f9e965d65fbe5cce
- before_left_evidence_hash: 2ac94b3869cbd261
- before_right_evidence_hash: a055f9b0a90c0b15
- before_left_gherkin: |
      @givn.added
      Scenario: A changed endpoint appends a candidate entry and receives the verified after state
        Given the remediation fixture stages a "DUPLICATE" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "DUPLICATE"
        And the classified endpoint changes
        And the gate reruns
        Then the run fails
        And a second entry records different endpoint hashes
        And the prior candidate entry is retained unchanged
        And the resolved duplicate "removed the superseded twin" is recorded ["708192a3"]
        And the doubled scenario is withdrawn
        And the gate reruns
        Then the review passes with the remediation receipt
        And the second entry records its after state
- before_right_gherkin: |
      @givn.added
      Scenario: A surviving pair records its recomputed embedding score after remediation
        Given the remediation fixture stages a "VARIANT" case
        And the run declaration is complete
        When the review gate runs
        And the assessor resolves the candidate to "VARIANT"
        And the user records the fix "consolidated" and commits ["2b3c4d5e"]
        And the corpus variant is consolidated
        And the review gate reruns
        Then the command exits 0
        And the log records the "outline-consolidated" after state with a recomputed score below the boundary
        And the candidate entry records the consolidated outline text verbatim from the active delta
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null
