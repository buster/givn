# Semantic Review
Report schema version: 3
Change: "domain-modeling-glossary-enforcement"
Status: PASS
Candidate count: 0
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 8
BGE recommended count: 1
BGE filtered count: 7
BGE unscored count: 0
BGE unique pair count: 8
BGE directional call count: 16
BGE model load attempts: 1
BGE runtime ms: 0
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 8
NLI recommended count: 1
NLI filtered count: 7
NLI unscored count: 0
NLI unique pair count: 8
NLI directional call count: 16
NLI model load attempts: 1
NLI runtime ms: 6446

## NLI Filtered Candidate
- candidate_id: 21b9064183df9189
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "The design-review skill checks that the ubiquitous language is followed"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The review skill checks that the ubiquitous language is followed"
- embedding_score: 0.9883
- nli_forward_entailment: 0.0253
- nli_forward_neutral: 0.9709
- nli_forward_contradiction: 0.0038
- nli_reverse_entailment: 0.4219
- nli_reverse_neutral: 0.4366
- nli_reverse_contradiction: 0.1415
- nli_symmetric_entailment: 0.1328
- nli_symmetric_neutral: 0.8373
- nli_symmetric_contradiction: 0.0299
- nli_admission: NLI_FILTERED

## BGE Filtered Candidate
- candidate_id: 7b8ec76fedf6a883
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "A project without the arc42 glossary is not pointed at the domain skill"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "An agent synchronizes skills and the domain skill is routed"
- embedding_score: 0.9732
- bge_forward_raw: 1.0213
- bge_reverse_raw: 1.0213
- bge_symmetric_raw: 1.0213
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: a420b823bd1e8c89
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "A project without the arc42 glossary is not pointed at the domain skill"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The skill requires the arc42 glossary for domain modeling"
- embedding_score: 0.9690
- bge_forward_raw: 0.5116
- bge_reverse_raw: 0.5116
- bge_symmetric_raw: 0.5116
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8e8d2c6723e3ea31
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "The review skill checks that the ubiquitous language is followed"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The ubiquitous language is recorded in the project's arc42 glossary"
- embedding_score: 0.9643
- bge_forward_raw: 0.9268
- bge_reverse_raw: 0.9268
- bge_symmetric_raw: 0.9268
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 87f80343df4d810e
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "A project without the arc42 glossary is not pointed at the domain skill"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The next-step guidance names the domain skill during later change phases"
- embedding_score: 0.9619
- bge_forward_raw: 1.0000
- bge_reverse_raw: 1.0000
- bge_symmetric_raw: 1.0000
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b7e89182ba095e8f
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "An agent synchronizes skills and the domain skill is routed"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The next-step guidance names the domain skill during later change phases"
- embedding_score: 0.9604
- bge_forward_raw: 0.8571
- bge_reverse_raw: 0.8571
- bge_symmetric_raw: 0.8571
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: f8943ba9248114e1
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "The skill requires the arc42 glossary for domain modeling"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The ubiquitous language is recorded in the project's arc42 glossary"
- embedding_score: 0.9598
- bge_forward_raw: 0.8889
- bge_reverse_raw: 0.8889
- bge_symmetric_raw: 0.8889
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b0c90e0f2f0f4fec
- left_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "The design-review skill checks that the ubiquitous language is followed"
- right_file: "givn/changes/domain-modeling-glossary-enforcement/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "The ubiquitous language is recorded in the project's arc42 glossary"
- embedding_score: 0.9597
- bge_forward_raw: 0.9268
- bge_reverse_raw: 0.9268
- bge_symmetric_raw: 0.9268
- bge_admission: BGE_FILTERED
