# Semantic Review
Report schema version: 1
Change: "semantic-review"
Status: PASS
Candidate count: 1

## Candidate 1
- candidate_id: 7903de1e3a6eea6d
- classification: VARIANTE
- rationale: "These candidates cover the same invalidation contract but exercise different endpoint lifecycles: one changes the active candidate scenario, while the other changes the permanent endpoint. The evidence and remediation paths are distinct variants."
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-review/specs/semantic-review/semantic-review.feature"
- left_title: "Changing a candidate scenario invalidates its previous classification"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-review/semantic-review.feature","Changing a candidate scenario invalidates its previous classification"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-review"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a project prepared for semantic review with a similar active delta","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLIKAT\"","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"When the active semantic candidate is changed","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"And the changed endpoint has a different content hash and evidence hash","docstring":null,"data_table":null},{"text":"And the semantic review report contains classification \"UNCERTAIN\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: a5961550eba08504
- left_evidence_hash: ba877405544a63e3
- right_scope: current-change
- right_file: "givn/changes/semantic-review/specs/semantic-review/semantic-review.feature"
- right_title: "Changing a permanent endpoint invalidates its prior classification"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-review/semantic-review.feature","Changing a permanent endpoint invalidates its prior classification"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-review"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given a project prepared for semantic review with a similar active delta","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLIKAT\"","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"When the permanent semantic endpoint is changed","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"And the changed permanent endpoint has a different content hash and evidence hash","docstring":null,"data_table":null},{"text":"And the semantic review report contains classification \"UNCERTAIN\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: a2eaaff81dc4eb91
- right_evidence_hash: fb239a57f6c88d2d
- embedding_score: 0.9727
- fact_contradiction: false
