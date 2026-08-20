# Semantic Review
Report schema version: 1
Change: "semantic-compact-serialization"
Status: BLOCKED
Candidate count: 3
E5 token-cap count: 2
e5_token_cap_count: 2
e5_token_cap_candidates: [{"status":"E5_TOKEN_CAP","file":"givn/specs/spec-duplicates/spec-duplicates.feature","title":"Invalid duplicate score overrides are rejected","content_hash":"c2ca17f37c15f630","token_count":800,"limit":512,"model":"intfloat/multilingual-e5-large","tokenizer":"intfloat/multilingual-e5-large/tokenizer.json","refactoring_required":true,"guidance":"shorten or split the scenario, then rerun retrieval"},{"status":"E5_TOKEN_CAP","file":"givn/specs/spec-duplicates/spec-duplicates.feature","title":"Structurally overlapping scenarios retain their distinguishing facts","content_hash":"0aae8551d346e564","token_count":516,"limit":512,"model":"intfloat/multilingual-e5-large","tokenizer":"intfloat/multilingual-e5-large/tokenizer.json","refactoring_required":true,"guidance":"shorten or split the scenario, then rerun retrieval"}]

## E5 Token Cap Candidate 1
- status: E5_TOKEN_CAP
- classification: null
- rationale: null
- file: "givn/specs/spec-duplicates/spec-duplicates.feature"
- title: "Invalid duplicate score overrides are rejected"
- content_hash: c2ca17f37c15f630
- token_count: 800
- limit: 512
- model: intfloat/multilingual-e5-large
- tokenizer: intfloat/multilingual-e5-large/tokenizer.json
- refactoring_required: true
- guidance: "shorten or split the scenario, then rerun semantic review"

## E5 Token Cap Candidate 2
- status: E5_TOKEN_CAP
- classification: null
- rationale: null
- file: "givn/specs/spec-duplicates/spec-duplicates.feature"
- title: "Structurally overlapping scenarios retain their distinguishing facts"
- content_hash: 0aae8551d346e564
- token_count: 516
- limit: 512
- model: intfloat/multilingual-e5-large
- tokenizer: intfloat/multilingual-e5-large/tokenizer.json
- refactoring_required: true
- guidance: "shorten or split the scenario, then rerun semantic review"
refactoring required: shorten or split the scenario, then rerun semantic review

## Candidate 1
- candidate_id: 4c3764d96b05e230
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- left_title: "Empty optional sections do not add retrieval content"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","Empty optional sections do not add retrieval content"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent scenario with only required feature, scenario, and step content","docstring":null,"data_table":null},{"text":"When the user rebuilds the permanent scenario retrieval index","docstring":null,"data_table":null},{"text":"Then the index contains the minimal scenario with the expected compact content hash","docstring":null,"data_table":null},{"text":"And the index contains no additional scenario vector","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 7076caac8f181d80
- left_evidence_hash: eeb21d568a2079c5
- right_scope: current-change
- right_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- right_title: "Non-empty optional sections remain complete retrieval content"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","Non-empty optional sections remain complete retrieval content"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent scenario with non-empty descriptions, backgrounds, tables, docstrings, outline, and examples","docstring":null,"data_table":null},{"text":"When the user rebuilds the permanent scenario retrieval index","docstring":null,"data_table":null},{"text":"Then the index contains the rich scenario with the expected complete compact content hash","docstring":null,"data_table":null},{"text":"And the rich scenario has a non-empty indexed vector","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 6f7cfda00e0803f7
- right_evidence_hash: 4e9e7d1f7d3edaac
- embedding_score: 0.9823
- fact_contradiction: false

## Candidate 2
- candidate_id: 947ab561eff4714e
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- left_title: "A maintainer can index scenarios with complete compact passages"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","A maintainer can index scenarios with complete compact passages"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent corpus with required and non-empty optional scenario content","docstring":null,"data_table":null},{"text":"When the end-to-end user rebuilds the compact retrieval index through the givn CLI","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout reports that the permanent scenarios were indexed","docstring":null,"data_table":null},{"text":"And the persistent retrieval index contains the indexed scenarios","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 13aeec575798a168
- left_evidence_hash: 27a66f6b8e39396e
- right_scope: current-change
- right_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- right_title: "Non-empty optional sections remain complete retrieval content"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","Non-empty optional sections remain complete retrieval content"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent scenario with non-empty descriptions, backgrounds, tables, docstrings, outline, and examples","docstring":null,"data_table":null},{"text":"When the user rebuilds the permanent scenario retrieval index","docstring":null,"data_table":null},{"text":"Then the index contains the rich scenario with the expected complete compact content hash","docstring":null,"data_table":null},{"text":"And the rich scenario has a non-empty indexed vector","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 6f7cfda00e0803f7
- right_evidence_hash: 4e9e7d1f7d3edaac
- embedding_score: 0.9653
- fact_contradiction: false

## Candidate 3
- candidate_id: c794c4b0295c752c
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- left_title: "A maintainer can index scenarios with complete compact passages"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","A maintainer can index scenarios with complete compact passages"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","e2e"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent corpus with required and non-empty optional scenario content","docstring":null,"data_table":null},{"text":"When the end-to-end user rebuilds the compact retrieval index through the givn CLI","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout reports that the permanent scenarios were indexed","docstring":null,"data_table":null},{"text":"And the persistent retrieval index contains the indexed scenarios","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 13aeec575798a168
- left_evidence_hash: 27a66f6b8e39396e
- right_scope: current-change
- right_file: "givn/changes/semantic-compact-serialization/specs/semantic-compact-serialization/semantic-compact-serialization.feature"
- right_title: "Empty optional sections do not add retrieval content"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-compact-serialization/semantic-compact-serialization.feature","Empty optional sections do not add retrieval content"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-compact-serialization"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a permanent scenario with only required feature, scenario, and step content","docstring":null,"data_table":null},{"text":"When the user rebuilds the permanent scenario retrieval index","docstring":null,"data_table":null},{"text":"Then the index contains the minimal scenario with the expected compact content hash","docstring":null,"data_table":null},{"text":"And the index contains no additional scenario vector","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 7076caac8f181d80
- right_evidence_hash: eeb21d568a2079c5
- embedding_score: 0.9651
- fact_contradiction: false
