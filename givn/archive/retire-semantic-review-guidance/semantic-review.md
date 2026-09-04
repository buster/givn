# Semantic Review
Report schema version: 3
Change: "retire-semantic-review-guidance"
Status: PASS
Candidate count: 0
Pair cache: 9 reused, 0 computed
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 9
BGE recommended count: 1
BGE filtered count: 8
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

## NLI Filtered Candidate
- candidate_id: bcb87e67f4493de8
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "A reviewer requests disposition-aware review guidance"
- right_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- right_title: "Review guidance names the deterministic disposition gates"
- embedding_score: 0.9581
- nli_forward_entailment: 0.2349
- nli_forward_neutral: 0.5234
- nli_forward_contradiction: 0.2417
- nli_reverse_entailment: 0.2458
- nli_reverse_neutral: 0.5281
- nli_reverse_contradiction: 0.2261
- nli_symmetric_entailment: 0.2404
- nli_symmetric_neutral: 0.5258
- nli_symmetric_contradiction: 0.2338
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: f37c15a0df00e59d
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "A reviewer requests disposition-aware review guidance"
- right_file: "givn/specs/instruction-integrity/instruction-integrity.feature"
- right_title: "An author requests the technical E2E design guidance"
- embedding_score: 0.9427
- bge_forward_raw: 1.9691
- bge_reverse_raw: 1.7294
- bge_symmetric_raw: 1.8493
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0d2908639538b7eb
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "A reviewer requests disposition-aware review guidance"
- right_file: "givn/specs/instruction-integrity/instruction-integrity.feature"
- right_title: "An agent requests machine-readable artifact guidance"
- embedding_score: 0.9407
- bge_forward_raw: 1.0893
- bge_reverse_raw: 1.6200
- bge_symmetric_raw: 1.3546
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: beca85d21e182608
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "A reviewer requests disposition-aware review guidance"
- right_file: "givn/specs/skill-rewrites/skill-rewrites.feature"
- right_title: "Review instructions require deterministic gate dispositions"
- embedding_score: 0.9398
- bge_forward_raw: -1.2107
- bge_reverse_raw: -1.0729
- bge_symmetric_raw: -1.1418
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3a97972477988b5d
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "A reviewer requests disposition-aware review guidance"
- right_file: "givn/specs/instruction-integrity/instruction-integrity.feature"
- right_title: "An author requests the canonical interaction and E2E guidance"
- embedding_score: 0.9355
- bge_forward_raw: 2.3269
- bge_reverse_raw: 1.2260
- bge_symmetric_raw: 1.7765
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: c90a7cecbf0ed566
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "Review guidance names the deterministic disposition gates"
- right_file: "givn/specs/skill-rewrites/skill-rewrites.feature"
- right_title: "Review instructions require deterministic gate dispositions"
- embedding_score: 0.9349
- bge_forward_raw: -1.1719
- bge_reverse_raw: -0.9377
- bge_symmetric_raw: -1.0548
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 0c583bf5d59a7555
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "Review guidance names the deterministic disposition gates"
- right_file: "givn/specs/skill-rewrites/skill-rewrites.feature"
- right_title: "Synchronized guidance explains how to resolve the deterministic gates"
- embedding_score: 0.9073
- bge_forward_raw: -2.6113
- bge_reverse_raw: -1.0817
- bge_symmetric_raw: -1.8465
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 66df360286315e73
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "Review guidance names the deterministic disposition gates"
- right_file: "givn/specs/skill-rewrites/skill-rewrites.feature"
- right_title: "Spec instructions use deterministic gates before retrieval exists"
- embedding_score: 0.9072
- bge_forward_raw: -2.6319
- bge_reverse_raw: -1.1954
- bge_symmetric_raw: -1.9136
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 950eb90038903224
- left_file: "givn/changes/retire-semantic-review-guidance/specs/instruction-integrity/instruction-integrity.feature"
- left_title: "Review guidance names the deterministic disposition gates"
- right_file: "givn/specs/instruction-integrity/instruction-integrity.feature"
- right_title: "Project instruction overrides survive skill synchronization"
- embedding_score: 0.9011
- bge_forward_raw: 2.1742
- bge_reverse_raw: 1.7707
- bge_symmetric_raw: 1.9725
- bge_admission: BGE_FILTERED
