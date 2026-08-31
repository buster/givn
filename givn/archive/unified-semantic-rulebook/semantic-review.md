# Semantic Review
Report schema version: 3
Change: "unified-semantic-rulebook"
Status: PASS
Candidate count: 0
Pair cache: 0 reused, 20 computed
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 20
BGE recommended count: 2
BGE filtered count: 18
BGE unscored count: 0
BGE unique pair count: 20
BGE directional call count: 40
BGE model load attempts: 1
BGE runtime ms: 30723
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
NLI model load attempts: 1
NLI runtime ms: 18237

## BGE Filtered Candidate
- candidate_id: 8c11f82a54122172
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "A changed permanent endpoint re-judges its recorded pair"
- embedding_score: 0.9970
- bge_forward_raw: 2.2133
- bge_reverse_raw: 2.2029
- bge_symmetric_raw: 2.2081
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 433e06cdd80e74eb
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer rerun reuses recorded pair judgements"
- embedding_score: 0.9915
- bge_forward_raw: 2.1186
- bge_reverse_raw: 2.9084
- bge_symmetric_raw: 2.5135
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 232a272613acdef1
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed permanent endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer rerun reuses recorded pair judgements"
- embedding_score: 0.9878
- bge_forward_raw: 1.8646
- bge_reverse_raw: 2.9044
- bge_symmetric_raw: 2.3845
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: 692ecdb06fd508b8
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A change without candidates reports empty cache accounting"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "A changed endpoint re-judges its recorded pair"
- embedding_score: 0.9806
- nli_forward_entailment: 0.0406
- nli_forward_neutral: 0.2664
- nli_forward_contradiction: 0.6930
- nli_reverse_entailment: 0.0203
- nli_reverse_neutral: 0.1228
- nli_reverse_contradiction: 0.8569
- nli_symmetric_entailment: 0.0293
- nli_symmetric_neutral: 0.1845
- nli_symmetric_contradiction: 0.7862
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 037b8173e3707fc3
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A change without candidates reports empty cache accounting"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer rerun reuses recorded pair judgements"
- embedding_score: 0.9804
- bge_forward_raw: 2.8663
- bge_reverse_raw: 2.1380
- bge_symmetric_raw: 2.5022
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 45b03a9de501fd64
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer review caps the judged pairs at the pair cap"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The pair cap fills deterministically when scores tie"
- embedding_score: 0.9803
- bge_forward_raw: 1.2504
- bge_reverse_raw: 0.2077
- bge_symmetric_raw: 0.7291
- bge_admission: BGE_FILTERED

## NLI Filtered Candidate
- candidate_id: de19afafd51e285a
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A change without candidates reports empty cache accounting"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "A changed permanent endpoint re-judges its recorded pair"
- embedding_score: 0.9777
- nli_forward_entailment: 0.0066
- nli_forward_neutral: 0.1005
- nli_forward_contradiction: 0.8929
- nli_reverse_entailment: 0.0055
- nli_reverse_neutral: 0.0540
- nli_reverse_contradiction: 0.9405
- nli_symmetric_entailment: 0.0060
- nli_symmetric_neutral: 0.0740
- nli_symmetric_contradiction: 0.9200
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 6c960308b181aca4
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The pair cap fills deterministically when scores tie"
- embedding_score: 0.9771
- bge_forward_raw: 0.7078
- bge_reverse_raw: 0.6844
- bge_symmetric_raw: 0.6961
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1b2bc7cc06235d59
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer rerun reuses recorded pair judgements"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The pair cap fills deterministically when scores tie"
- embedding_score: 0.9767
- bge_forward_raw: 1.9764
- bge_reverse_raw: 0.4944
- bge_symmetric_raw: 1.2354
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 89283a60c9c72f35
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer review caps the judged pairs at the pair cap"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer review judges the selected pairs with both evidence models"
- embedding_score: 0.9744
- bge_forward_raw: 1.4545
- bge_reverse_raw: 0.0247
- bge_symmetric_raw: 0.7396
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1c8eae39372a98b7
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer review caps the judged pairs at the pair cap"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The neighbour count cuts per-scenario pairs before the pair cap"
- embedding_score: 0.9744
- bge_forward_raw: 1.9331
- bge_reverse_raw: 1.4972
- bge_symmetric_raw: 1.7151
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 1228a4ce0c581f6e
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer rerun reuses recorded pair judgements"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer review judges the selected pairs with both evidence models"
- embedding_score: 0.9741
- bge_forward_raw: 0.7453
- bge_reverse_raw: -0.6658
- bge_symmetric_raw: 0.0397
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9428a05e7d3b7415
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer rerun reuses recorded pair judgements"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer review caps the judged pairs at the pair cap"
- embedding_score: 0.9729
- bge_forward_raw: 1.4439
- bge_reverse_raw: 0.7795
- bge_symmetric_raw: 1.1117
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 19debb04ea2ada06
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed permanent endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The pair cap fills deterministically when scores tie"
- embedding_score: 0.9722
- bge_forward_raw: 0.9633
- bge_reverse_raw: 0.6247
- bge_symmetric_raw: 0.7940
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 67747a8b1dfae5fa
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Consumer archive applies the rulebook to the current permanent tree"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer rerun reuses recorded pair judgements"
- embedding_score: 0.9718
- bge_forward_raw: 0.8863
- bge_reverse_raw: 2.0486
- bge_symmetric_raw: 1.4674
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a734de01e9343ed4
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "Emergency budget truncation keeps outside candidates unscored and blocking"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The neighbour count cuts per-scenario pairs before the pair cap"
- embedding_score: 0.9718
- bge_forward_raw: 1.8210
- bge_reverse_raw: 1.3305
- bge_symmetric_raw: 1.5758
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8f40ce25484ac0b7
- left_file: "givn/changes/unified-semantic-rulebook/specs/spec-retrieval/spec-retrieval.feature"
- left_title: "Review gate runs the unified rulebook and stays green without candidates"
- right_file: "givn/specs/spec-retrieval/spec-retrieval.feature"
- right_title: "Review gate keeps retrieval advisory"
- embedding_score: 0.9704
- bge_forward_raw: -1.1608
- bge_reverse_raw: -2.4501
- bge_symmetric_raw: -1.8055
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 6ea79601cf75c675
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer review judges the selected pairs with both evidence models"
- embedding_score: 0.9690
- bge_forward_raw: 0.9202
- bge_reverse_raw: 0.3141
- bge_symmetric_raw: 0.6171
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ed5502c818c2511e
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A changed endpoint re-judges its recorded pair"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "Consumer review caps the judged pairs at the pair cap"
- embedding_score: 0.9686
- bge_forward_raw: 0.2560
- bge_reverse_raw: 1.0386
- bge_symmetric_raw: 0.6473
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 2113ea517817a6dc
- left_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- left_title: "A change without candidates reports empty cache accounting"
- right_file: "givn/changes/unified-semantic-rulebook/specs/unified-semantic-rulebook/unified-semantic-rulebook.feature"
- right_title: "The pair cap fills deterministically when scores tie"
- embedding_score: 0.9682
- bge_forward_raw: 2.2852
- bge_reverse_raw: 0.4794
- bge_symmetric_raw: 1.3823
- bge_admission: BGE_FILTERED
