# Semantic Review
Report schema version: 3
Change: "semantic-remediation-log"
Status: PASS
Candidate count: 0
Pair cache: 18 reused, 2 computed
refactoring required: shorten or split the scenarios, then rerun semantic review
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 20
BGE recommended count: 0
BGE filtered count: 18
BGE unscored count: 0
BGE unique pair count: 20
BGE directional call count: 40
BGE model load attempts: 1
BGE runtime ms: 8179
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 20
NLI recommended count: 0
NLI filtered count: 20
NLI unscored count: 0
NLI unique pair count: 20
NLI directional call count: 40
NLI model load attempts: 0
NLI runtime ms: 0

## BGE Filtered Candidate
- candidate_id: 8e2bb18bc6bd4ed4
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- embedding_score: 0.9734
- bge_forward_raw: 1.2640
- bge_reverse_raw: 0.0641
- bge_symmetric_raw: 0.6640
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2a6ee67b082c8530
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A verified remediation whose log record was <log-state> still passes review"
- embedding_score: 0.9693
- bge_forward_raw: 0.7899
- bge_reverse_raw: 0.2084
- bge_symmetric_raw: 0.4992
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: 28e1116cba1665bf
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "An invalid remediation evidence state blocks the review"
- embedding_score: 0.9684
- nli_forward_entailment: 0.2557
- nli_forward_neutral: 0.3704
- nli_forward_contradiction: 0.3739
- nli_reverse_entailment: 0.2498
- nli_reverse_neutral: 0.3228
- nli_reverse_contradiction: 0.4274
- nli_symmetric_entailment: 0.2532
- nli_symmetric_neutral: 0.3464
- nli_symmetric_contradiction: 0.4005
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 9117086a18f1a514
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Real CLI archive retains the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Real CLI review completes the remediation log for a stamped duplicate remediation"
- embedding_score: 0.9675
- bge_forward_raw: -0.1217
- bge_reverse_raw: 0.1792
- bge_symmetric_raw: 0.0288
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f929265afe0a5a79
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A logged pair scored by BGE and NLI records their evidence states"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- embedding_score: 0.9662
- bge_forward_raw: 2.6470
- bge_reverse_raw: 1.5472
- bge_symmetric_raw: 2.0971
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8e0d3d4d515f01f7
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-review/semantic-review.feature"
- right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- embedding_score: 0.9654
- bge_forward_raw: 1.0502
- bge_reverse_raw: 0.1672
- bge_symmetric_raw: 0.6087
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 7ead621ce6588126
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "The first blocked review creates the remediation log with before evidence"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- embedding_score: 0.9650
- bge_forward_raw: -0.0032
- bge_reverse_raw: 0.5802
- bge_symmetric_raw: 0.2885
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6ff8f2dbd6dc358b
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A logged pair scored by BGE and NLI records their evidence states"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The first blocked review creates the remediation log with before evidence"
- embedding_score: 0.9649
- bge_forward_raw: 2.6624
- bge_reverse_raw: 0.0481
- bge_symmetric_raw: 1.3552
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ed43a91172e80e92
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The first blocked review creates the remediation log with before evidence"
- embedding_score: 0.9643
- bge_forward_raw: 0.9353
- bge_reverse_raw: -0.6910
- bge_symmetric_raw: 0.1222
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: 4fb206fa86438bed
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A verified remediation whose log record was <log-state> still passes review"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "An invalid remediation evidence state blocks the review"
- embedding_score: 0.9637
- nli_forward_entailment: 0.2519
- nli_forward_neutral: 0.3420
- nli_forward_contradiction: 0.4060
- nli_reverse_entailment: 0.2578
- nli_reverse_neutral: 0.2735
- nli_reverse_contradiction: 0.4687
- nli_symmetric_entailment: 0.2556
- nli_symmetric_neutral: 0.3068
- nli_symmetric_contradiction: 0.4376
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 14d931c4df166a6d
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Advisory spec review does not create the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Blocking review without its retrieval feature does not create the remediation log"
- embedding_score: 0.9635
- bge_forward_raw: -0.1593
- bge_reverse_raw: 0.5227
- bge_symmetric_raw: 0.1817
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ba2bd9d7bc9c7b2f
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A verified remediation whose log record was <log-state> still passes review"
- embedding_score: 0.9634
- bge_forward_raw: 0.2078
- bge_reverse_raw: 0.5020
- bge_symmetric_raw: 0.3549
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6497c4f154a2dcf8
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- embedding_score: 0.9626
- bge_forward_raw: 1.7524
- bge_reverse_raw: 2.0211
- bge_symmetric_raw: 1.8868
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 97fd564d5d60fbcd
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A verified remediation whose log record was <log-state> still passes review"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- embedding_score: 0.9625
- bge_forward_raw: 0.7325
- bge_reverse_raw: 0.8018
- bge_symmetric_raw: 0.7671
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: bf44de568ce57dcc
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- embedding_score: 0.9624
- bge_forward_raw: 2.0918
- bge_reverse_raw: 1.5475
- bge_symmetric_raw: 1.8197
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: da1b0f211fdeec9d
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A commit fixing two candidates carries one stamp line per candidate"
- embedding_score: 0.9619
- bge_forward_raw: 0.9931
- bge_reverse_raw: -0.3009
- bge_symmetric_raw: 0.3461
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 47d5f65db8e6f2b7
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-review/semantic-review.feature"
- right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- embedding_score: 0.9607
- bge_forward_raw: 0.8779
- bge_reverse_raw: -0.1722
- bge_symmetric_raw: 0.3529
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e45e0efe48b4c0f4
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A changed endpoint appends a candidate entry and receives the verified after state"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A surviving pair records its recomputed embedding score after remediation"
- embedding_score: 0.9601
- bge_forward_raw: 0.3850
- bge_reverse_raw: -0.6000
- bge_symmetric_raw: -0.1075
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 97bae1b6243967f6
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A surviving pair records its recomputed embedding score after remediation"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The first blocked review creates the remediation log with before evidence"
- embedding_score: 0.9599
- bge_forward_raw: -0.0178
- bge_reverse_raw: -0.3083
- bge_symmetric_raw: -0.1631
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 63ab46928e8c41ca
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The first blocked review creates the remediation log with before evidence"
- embedding_score: 0.9592
- bge_forward_raw: 2.4408
- bge_reverse_raw: 1.0084
- bge_symmetric_raw: 1.7246
- bge_admission: BGE_FILTERED
