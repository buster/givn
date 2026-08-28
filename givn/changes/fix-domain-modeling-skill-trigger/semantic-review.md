# Semantic Review
Report schema version: 2
Change: "fix-domain-modeling-skill-trigger"
Status: BLOCKED
Candidate count: 3
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 3
BGE recommended count: 3
BGE filtered count: 0
BGE unscored count: 0
BGE unique pair count: 3
BGE directional call count: 6
BGE model load attempts: 1
BGE runtime ms: 130917

## Candidate 1
- candidate_id: 3ced702199695192
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "An agent requests the design guidance and is routed to the domain skill"
- left_delta_operation: added
- left_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests the design guidance and is routed to the domain skill"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the design instructions through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 0240556b323fcc03
- left_evidence_hash: 15817e7bab509ec3
- right_scope: current-change
- right_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "An agent requests the specs guidance and is routed to the domain skill"
- right_delta_operation: added
- right_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests the specs guidance and is routed to the domain skill"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the specs instructions through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 2bc99d9fc8d3ca51
- right_evidence_hash: aac79ee0656143af
- embedding_score: 0.9700
- fact_contradiction: false
- bge_forward_raw: 4.6081
- bge_reverse_raw: 4.7110
- bge_symmetric_raw: 4.6596
- bge_admission: BGE_RECOMMENDED
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_UNSCORED

## Candidate 2
- candidate_id: d0cbb7010611f33d
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "An agent requests next-step guidance and is routed to the domain skill"
- left_delta_operation: added
- left_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests next-step guidance and is routed to the domain skill"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the next-step guidance through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: f01a9fef2df45339
- left_evidence_hash: b2584d8ceb7df952
- right_scope: current-change
- right_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "An agent requests the specs guidance and is routed to the domain skill"
- right_delta_operation: added
- right_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests the specs guidance and is routed to the domain skill"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the specs instructions through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 2bc99d9fc8d3ca51
- right_evidence_hash: aac79ee0656143af
- embedding_score: 0.9585
- fact_contradiction: false
- bge_forward_raw: 4.8190
- bge_reverse_raw: 4.6836
- bge_symmetric_raw: 4.7513
- bge_admission: BGE_RECOMMENDED
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_UNSCORED

## Candidate 3
- candidate_id: 206d30baadfa8b63
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- left_title: "An agent requests next-step guidance and is routed to the domain skill"
- left_delta_operation: added
- left_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests next-step guidance and is routed to the domain skill"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the next-step guidance through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: f01a9fef2df45339
- left_evidence_hash: b2584d8ceb7df952
- right_scope: current-change
- right_file: "givn/changes/fix-domain-modeling-skill-trigger/specs/domain-skill-trigger/domain-skill-trigger.feature"
- right_title: "An agent requests the design guidance and is routed to the domain skill"
- right_delta_operation: added
- right_projected_target: ["givn/specs/domain-skill-trigger/domain-skill-trigger.feature","An agent requests the design guidance and is routed to the domain skill"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","domain-skill-trigger"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a Givn project with the domain model addon enabled and a change awaiting specs","docstring":null,"data_table":null},{"text":"When the agent requests the design instructions through the Givn CLI","docstring":null,"data_table":null},{"text":"Then the guidance says to load the domain-modeling skill now","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 0240556b323fcc03
- right_evidence_hash: 15817e7bab509ec3
- embedding_score: 0.9571
- fact_contradiction: false
- bge_forward_raw: 4.6513
- bge_reverse_raw: 4.4817
- bge_symmetric_raw: 4.5665
- bge_admission: BGE_RECOMMENDED
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_UNSCORED
