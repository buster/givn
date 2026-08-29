# Semantic Review
Report schema version: 3
Change: "release-strict-model-overrides"
Status: PASS
Candidate count: 0
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 6
BGE recommended count: 1
BGE filtered count: 5
BGE unscored count: 0
BGE unique pair count: 6
BGE directional call count: 12
BGE model load attempts: 1
BGE runtime ms: 10420
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 6
NLI recommended count: 0
NLI filtered count: 6
NLI unscored count: 0
NLI unique pair count: 6
NLI directional call count: 12
NLI model load attempts: 1
NLI runtime ms: 7426

## NLI Filtered Candidate
- candidate_id: f9b2c1579a00599e
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "A development givn build honours a simulated-embedder request without an installed-build announcement"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn refuses a request for simulated embedder output"
- embedding_score: 0.9873
- nli_forward_entailment: 0.0015
- nli_forward_neutral: 0.0128
- nli_forward_contradiction: 0.9857
- nli_reverse_entailment: 0.0020
- nli_reverse_neutral: 0.0170
- nli_reverse_contradiction: 0.9809
- nli_symmetric_entailment: 0.0018
- nli_symmetric_neutral: 0.0148
- nli_symmetric_contradiction: 0.9835
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 092ada6c3b9acb56
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn honours a model-location request that still uses the real models"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn refuses a request for simulated embedder output"
- embedding_score: 0.9818
- bge_forward_raw: 2.9977
- bge_reverse_raw: 2.9499
- bge_symmetric_raw: 2.9738
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 943228068b8829ca
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "A development givn build honours a simulated-embedder request without an installed-build announcement"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn honours a model-location request that still uses the real models"
- embedding_score: 0.9783
- bge_forward_raw: 1.7975
- bge_reverse_raw: 2.2909
- bge_symmetric_raw: 2.0442
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 9745380aea565e3e
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn ignores requests to disable or simulate the second-stage models"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn refuses a request for simulated embedder output"
- embedding_score: 0.9781
- bge_forward_raw: 0.7102
- bge_reverse_raw: 2.7579
- bge_symmetric_raw: 1.7341
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b48289efd141ea60
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn honours a model-location request that still uses the real models"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn ignores requests to disable or simulate the second-stage models"
- embedding_score: 0.9719
- bge_forward_raw: 2.4230
- bge_reverse_raw: 1.8276
- bge_symmetric_raw: 2.1253
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a9a82a9f84856ed8
- left_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "A development givn build honours a simulated-embedder request without an installed-build announcement"
- right_file: "givn/changes/release-strict-model-overrides/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn ignores requests to disable or simulate the second-stage models"
- embedding_score: 0.9683
- bge_forward_raw: 1.9086
- bge_reverse_raw: 0.4364
- bge_symmetric_raw: 1.1725
- bge_admission: BGE_FILTERED
