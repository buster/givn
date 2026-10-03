# Semantic Remediation Log
Log schema version: 1
Change: "rebuild-deterministic-spec-discovery"

## Run 1
- kind: before
- commit: 37d59a7adfd8d6492f00ad571fd55570ae5dc3d2
- e5_runtime_ms: 14299
- bge_runtime_ms: 20347
- nli_runtime_ms: 12235

## Candidate eb91d84e3818da22
- run: 1
- before_embedding_score: 0.9776
- before_bge_admission: BGE_RECOMMENDED
- before_bge_symmetric_raw: 3.4716
- before_nli_admission: NLI_RECOMMENDED
- before_nli_symmetric_entailment: 0.6647
- before_left_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- before_left_title: "Auditing leaves permanent specifications unchanged"
- before_right_file: "givn/changes/rebuild-deterministic-spec-discovery/specs/spec-duplicates/spec-duplicates.feature"
- before_right_title: "Repeating an unchanged audit produces the same report"
- before_left_content_hash: 3dcda2843993c466
- before_right_content_hash: 9a85408daae3b1dd
- before_left_evidence_hash: b3f43511b60ac422
- before_right_evidence_hash: fce5dec90473af88
- before_left_gherkin: |
      @givn.removed
      Scenario: Auditing leaves permanent specifications unchanged
- before_right_gherkin: |
      @givn.removed
      Scenario: Repeating an unchanged audit produces the same report
- classification: UNCERTAIN
- rationale: null
- remediation: null
- commits: []
- after_state: null
- after_embedding_score: null
- after_commit: null
- after_left_gherkin: null
