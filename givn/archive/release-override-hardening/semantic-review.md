# Semantic Review
Report schema version: 3
Change: "release-override-hardening"
Status: PASS
Candidate count: 0
Pair cache: 9 reused, 0 computed
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 9
BGE recommended count: 0
BGE filtered count: 9
BGE unscored count: 0
BGE unique pair count: 9
BGE directional call count: 18
BGE model load attempts: 0
BGE runtime ms: 0
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 9
NLI recommended count: 0
NLI filtered count: 9
NLI unscored count: 0
NLI unique pair count: 9
NLI directional call count: 18
NLI model load attempts: 0
NLI runtime ms: 0

## BGE Filtered Candidate
- candidate_id: 23f850ee544371a8
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn announces each refused request by name"
- right_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn keeps the review gate closed when model evidence is unavailable"
- embedding_score: 0.9610
- bge_forward_raw: -0.8013
- bge_reverse_raw: 0.1119
- bge_symmetric_raw: -0.3447
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 100a426cea6055b7
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn announces each refused request by name"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn refuses a request for simulated embedder output"
- embedding_score: 0.9473
- bge_forward_raw: -2.0182
- bge_reverse_raw: -0.1054
- bge_symmetric_raw: -1.0618
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: ade359975a640115
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn keeps the review gate closed when model evidence is unavailable"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn refuses a request for simulated embedder output"
- embedding_score: 0.9447
- bge_forward_raw: -1.2418
- bge_reverse_raw: 0.3230
- bge_symmetric_raw: -0.4594
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 40bbfa0600ce455d
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn announces each refused request by name"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn ignores requests to disable or simulate the second-stage models"
- embedding_score: 0.9379
- bge_forward_raw: -2.6227
- bge_reverse_raw: -1.3259
- bge_symmetric_raw: -1.9743
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 09d0264e1b36ac3f
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn keeps the review gate closed when model evidence is unavailable"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn ignores requests to disable or simulate the second-stage models"
- embedding_score: 0.9350
- bge_forward_raw: -1.2517
- bge_reverse_raw: -0.4327
- bge_symmetric_raw: -0.8422
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 10ae42a4db38c7dd
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn announces each refused request by name"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "A development givn build honours a simulated-embedder request without an installed-build announcement"
- embedding_score: 0.9347
- bge_forward_raw: -2.7879
- bge_reverse_raw: -0.9732
- bge_symmetric_raw: -1.8806
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: e8e94f49dbda510b
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn keeps the review gate closed when model evidence is unavailable"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "A development givn build honours a simulated-embedder request without an installed-build announcement"
- embedding_score: 0.9315
- bge_forward_raw: -1.8268
- bge_reverse_raw: -0.5492
- bge_symmetric_raw: -1.1880
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b0d7eac7b4545e63
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn keeps the review gate closed when model evidence is unavailable"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn honours a model-location request that still uses the real models"
- embedding_score: 0.9292
- bge_forward_raw: -1.8183
- bge_reverse_raw: -0.6539
- bge_symmetric_raw: -1.2361
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 72a37b9eedce0825
- left_file: "givn/changes/release-override-hardening/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- left_title: "An installed givn announces each refused request by name"
- right_file: "givn/specs/release-strict-model-overrides/release-strict-model-overrides.feature"
- right_title: "An installed givn honours a model-location request that still uses the real models"
- embedding_score: 0.9290
- bge_forward_raw: -2.2358
- bge_reverse_raw: -1.4197
- bge_symmetric_raw: -1.8278
- bge_admission: BGE_FILTERED
