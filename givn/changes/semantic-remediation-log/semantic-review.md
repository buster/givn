# Semantic Review
Report schema version: 3
Change: "semantic-remediation-log"
Status: BLOCKED
Candidate count: 12
Pair cache: 8 reused, 12 computed
E5 token-cap count: 1
e5_token_cap_count: 1
e5_token_cap_candidates: [{"status":"E5_TOKEN_CAP","file":"givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature","title":"A changed endpoint appends a candidate entry and receives the verified after state","content_hash":"270333a9a2979607","token_count":519,"limit":512,"model":"intfloat/multilingual-e5-large","tokenizer":"intfloat/multilingual-e5-large/tokenizer.json","refactoring_required":true,"guidance":"shorten or split the scenario, then rerun retrieval"}]

## E5 Token Cap Candidate 1
- status: E5_TOKEN_CAP
- classification: null
- rationale: null
- file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- title: "A changed endpoint appends a candidate entry and receives the verified after state"
- content_hash: 270333a9a2979607
- token_count: 519
- limit: 512
- model: intfloat/multilingual-e5-large
- tokenizer: intfloat/multilingual-e5-large/tokenizer.json
- refactoring_required: true
- guidance: "shorten or split the scenario, then rerun semantic review"
refactoring required: shorten or split the scenario, then rerun semantic review
refactoring required: shorten or split the scenarios, then rerun semantic review
BGE status: BGE_ENABLED
BGE reason: null
BGE model: rozgo/bge-reranker-v2-m3
BGE top_k: 5
BGE max unique pairs: 2048
BGE E5 raw count: 20
BGE recommended count: 0
BGE filtered count: 8
BGE unscored count: 0
BGE unique pair count: 20
BGE directional call count: 40
BGE model load attempts: 1
BGE runtime ms: 23698
NLI status: NLI_ENABLED
NLI reason: null
NLI model: MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
NLI top_k: 5
NLI max unique pairs: 2048
NLI E5 raw count: 20
NLI recommended count: 0
NLI filtered count: 8
NLI unscored count: 0
NLI unique pair count: 20
NLI directional call count: 40
NLI model load attempts: 1
NLI runtime ms: 28815

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

## Candidate 1
- candidate_id: 2a6ee67b082c8530
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A <repository-state> records null run anchors and reviews without stamp checks"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":"A <repository-state> records null run anchors and reviews without stamp checks","examples":[{"name":null,"description":null,"tags":[],"table":[["repository-state"],["without Git history"],["inside an unborn Git repository"]]}],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"jackdaw jetty mandrel mist needle urn vane yoke\"","docstring":null,"data_table":null},{"text":"Given a remediation-log project with one \"DUPLICATE\" finding <repository-state>","docstring":null,"data_table":null},{"text":"And the run block of the project is fully declared","docstring":null,"data_table":null},{"text":"When the change review gate runs for running-change","docstring":null,"data_table":null},{"text":"And the reviewer marks each semantic finding as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the reviewer logs the fix \"took the surplus out\" and the commits [\"3c4d5e6f\"]","docstring":null,"data_table":null},{"text":"And the doubled scenario is withdrawn from the change specs","docstring":null,"data_table":null},{"text":"Then the remediation log records the before run anchor as null","docstring":null,"data_table":null},{"text":"And the gate is evaluated a second time","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 6a348c5d0f563b3b
- left_evidence_hash: 6015c0bf20324279
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A verified remediation whose log record was <log-state> still passes review"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A verified remediation whose log record was <log-state> still passes review"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":"A verified remediation whose log record was <log-state> still passes review","examples":[{"name":null,"description":null,"tags":[],"table":[["log-state"],["deleted entirely"],["stripped of its candidate entries"]]}],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"dinghy fern harness heather nozzle whip wren zinnia\"","docstring":null,"data_table":null},{"text":"Given a duplicate-log project contains a \"DUPLICATE\" case","docstring":null,"data_table":null},{"text":"And the run declaration of the project is complete","docstring":null,"data_table":null},{"text":"When the blocking review is executed for running-change","docstring":null,"data_table":null},{"text":"And the operator labels every review candidate \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the operator records the fix \"removed the twin\" with commits [\"4d5e6f70\"]","docstring":null,"data_table":null},{"text":"And the remediation log is <log-state>","docstring":null,"data_table":null},{"text":"And the redundant scenario is deleted from the delta","docstring":null,"data_table":null},{"text":"And the review command reruns for running-change","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 9bb34735aab4ad18
- right_evidence_hash: fce3d93d9f45e582
- embedding_score: 0.9621
- fact_contradiction: true
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 367
- bge_right_tokens: 343
- bge_pair_tokens: 710
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 347
- nli_right_tokens: 325
- nli_pair_tokens: 671
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 2
- candidate_id: abdf3ab06efcab85
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- left_delta_operation: modified
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","Removing a DUPLICATE candidate verifies the projected remediation"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.modified"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"cinder gondola locomotive rivet timber trowel tundra umber\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the observatory corpus:","docstring":"\nFeature: Observatory night log\n\n  Scenario: The seeing conditions are noted twice\n    Given a clear night at the observatory\n    When the astronomer notes the seeing\n    Then the night log holds one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Observatory night log change\n\n  @givn.added\n  Scenario: The seeing conditions are noted twice\n    Given a clear night at the observatory\n    When the astronomer notes the seeing\n    Then the night log holds one entry\n","data_table":null},{"text":"And the permanent specifications are snapshotted","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the gate operator marks the candidate \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the gate operator files the fix \"removed\" with the record [\"0a1b2c40\"]","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null},{"text":"And the semantic review report status is \"PASS\"","docstring":null,"data_table":null},{"text":"And the permanent specifications are unchanged","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 9adcaa38cbaab3ac
- left_evidence_hash: 0ce54f6da4407bf5
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","The passing rerun completes the after state, and a further rerun leaves the log unchanged"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"cleat domino lantern mist oarlock sandal thistle wren\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the ferry corpus:","docstring":"\nFeature: Ferry manifest\n\n  Scenario: The car deck count is noted twice\n    Given a ferry at the loading ramp\n    When the steward counts the cars\n    Then the manifest shows one deck count\n      --- delta ---\n@givn.delta @running-change\nFeature: Ferry manifest change\n\n  @givn.added\n  Scenario: The car deck count is noted twice\n    Given a ferry at the loading ramp\n    When the steward counts the cars\n    Then the manifest shows one deck count\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the user records the fix \"removed the duplicate\" and commits [\"1a2b3c4d\"]","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null},{"text":"And the remediation log contains an after run record","docstring":null,"data_table":null},{"text":"And the removed candidate entry records after state \"endpoint-absent\" with a null after score","docstring":null,"data_table":null},{"text":"And the active remediation log bytes are snapshotted","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change` again without edits","docstring":null,"data_table":null},{"text":"Then the remediation log bytes equal the active log snapshot","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: c5eca3a2e84a1eaf
- right_evidence_hash: c92974ea2adedc0a
- embedding_score: 0.9590
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 417
- bge_right_tokens: 501
- bge_pair_tokens: 918
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 424
- nli_right_tokens: 510
- nli_pair_tokens: 933
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 3
- candidate_id: 28e1116cba1665bf
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A <repository-state> records null run anchors and reviews without stamp checks"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":"A <repository-state> records null run anchors and reviews without stamp checks","examples":[{"name":null,"description":null,"tags":[],"table":[["repository-state"],["without Git history"],["inside an unborn Git repository"]]}],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"jackdaw jetty mandrel mist needle urn vane yoke\"","docstring":null,"data_table":null},{"text":"Given a remediation-log project with one \"DUPLICATE\" finding <repository-state>","docstring":null,"data_table":null},{"text":"And the run block of the project is fully declared","docstring":null,"data_table":null},{"text":"When the change review gate runs for running-change","docstring":null,"data_table":null},{"text":"And the reviewer marks each semantic finding as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the reviewer logs the fix \"took the surplus out\" and the commits [\"3c4d5e6f\"]","docstring":null,"data_table":null},{"text":"And the doubled scenario is withdrawn from the change specs","docstring":null,"data_table":null},{"text":"Then the remediation log records the before run anchor as null","docstring":null,"data_table":null},{"text":"And the gate is evaluated a second time","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 6a348c5d0f563b3b
- left_evidence_hash: 6015c0bf20324279
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "An invalid remediation evidence state blocks the review"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","An invalid remediation evidence state blocks the review"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":"An invalid remediation evidence state blocks the review","examples":[{"name":null,"description":null,"tags":[],"table":[["repository-state","evidence-state","expected-diagnostic"],["without Git history","without commit evidence","remediation log"],["inside a committed Git repository","without the stamp","semantic-remediation"],["inside a committed Git repository","with a stamp naming another candidate","semantic-remediation"],["inside a committed Git repository","with its commit removed from the history","reachable"]]}],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"accordion apricot grapple keel quill saffron scythe vest\"","docstring":null,"data_table":null},{"text":"Given a remediation-log project with one \"DUPLICATE\" finding <repository-state>","docstring":null,"data_table":null},{"text":"And the run block of the project is fully declared","docstring":null,"data_table":null},{"text":"When the change review gate runs for running-change","docstring":null,"data_table":null},{"text":"And the reviewer marks each semantic finding as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the doubled scenario is withdrawn from the change specs","docstring":null,"data_table":null},{"text":"And the remediation evidence is left <evidence-state>","docstring":null,"data_table":null},{"text":"And the gate is evaluated a second time","docstring":null,"data_table":null},{"text":"Then the run fails","docstring":null,"data_table":null},{"text":"And the diagnostic names \"<expected-diagnostic>\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 6437ed90cc53d44c
- right_evidence_hash: d55a41f9849237a8
- embedding_score: 0.9577
- fact_contradiction: true
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 367
- bge_right_tokens: 423
- bge_pair_tokens: 790
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 347
- nli_right_tokens: 380
- nli_pair_tokens: 726
- nli_limit: 512
- nli_refactoring_required: true

## BGE Filtered Candidate
- candidate_id: 1b695a8f04a5038a
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Failed review before the semantic stage does not create the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The remediation log obeys its strict wire grammar"
- embedding_score: 0.9572
- bge_forward_raw: 1.4167
- bge_reverse_raw: 1.1460
- bge_symmetric_raw: 1.2814
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: b409c3d4c2f8b82c
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "The remediation log obeys its strict wire grammar"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The remediation log survives the archive move"
- embedding_score: 0.9568
- bge_forward_raw: 0.4162
- bge_reverse_raw: 0.3832
- bge_symmetric_raw: 0.3997
- bge_admission: BGE_FILTERED

## Candidate 4
- candidate_id: 947fc749d7eecdda
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A surviving pair records its recomputed embedding score after remediation"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A surviving pair records its recomputed embedding score after remediation"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"abacus lumen lyre moss nickel quadrant scythe xylophone\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the beehive corpus:","docstring":"\nFeature: Beehive inspection\n\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Beehive inspection change\n\n  @givn.added\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"VARIANT\"","docstring":null,"data_table":null},{"text":"And the user records the fix \"consolidated\" and commits [\"2b3c4d5e\"]","docstring":null,"data_table":null},{"text":"And the active corpus variant is consolidated with an explicit parameter","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And the remediation log records after state \"outline-consolidated\" with the recomputed embedding score below the boundary","docstring":null,"data_table":null},{"text":"And the candidate entry records the consolidated outline text verbatim from the active delta","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: e575c90598275cfc
- left_evidence_hash: 5f342fdd95bfab3a
- right_scope: permanent
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Folding the variant body into a Scenario Outline passes the first-change gate"
- right_delta_operation: permanent
- right_projected_target: null
- right_evidence_json: {"feature_description":null,"feature_tags":[],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"the beekeepers cooperative lists the same hive inspection stamp on both the spring inspection sheet and the autumn inspection sheet while the running change carries two first-time delta scenarios\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic remediation with a delta-to-delta \"VARIANT\" candidate on change \"rc-var-consolidated\"","docstring":null,"data_table":null},{"text":"Given the remediation notes record that \"the beekeepers ran the spring and autumn hive inspections on the same calendar day because only one inspector held the service key, pressing the self-same inspection stamp onto both record sheets, with the same brood-frame count and the same queen condition noted twice, so the cooperative's ledger contained two entries differing in nothing but the heading of the sheet\"","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change rc-var-consolidated`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"VARIANT\"","docstring":null,"data_table":null},{"text":"And the delta-to-delta variant is consolidated into a parameterized Scenario Outline","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change rc-var-consolidated`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout contains \"semantic remediation verified\"","docstring":null,"data_table":null},{"text":"And the semantic review report status is \"PASS\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: ca8f1efdee54edf0
- right_evidence_hash: 242886f84191c632
- embedding_score: 0.9566
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 452
- bge_right_tokens: 448
- bge_pair_tokens: 900
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 460
- nli_right_tokens: 455
- nli_pair_tokens: 914
- nli_limit: 512
- nli_refactoring_required: true

## BGE Filtered Candidate
- candidate_id: 600aa6aab0472ec8
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Failed review before the semantic stage does not create the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The remediation log survives the archive move"
- embedding_score: 0.9564
- bge_forward_raw: 1.5891
- bge_reverse_raw: 1.7795
- bge_symmetric_raw: 1.6843
- bge_admission: BGE_FILTERED

## Candidate 5
- candidate_id: bdd91b8d54fcb3f2
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A commit fixing two candidates carries one stamp line per candidate"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A commit fixing two candidates carries one stamp line per candidate"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"beacon cobalt fennel iris nettle pumice quill vane\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidates are removed from the active deltas","docstring":null,"data_table":null},{"text":"And the user commits the remediation with one matching stamp per candidate","docstring":null,"data_table":null},{"text":"And the user records both remediation fixes and the commit in the remediation log","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 229e4912bb93ed6c
- left_evidence_hash: b74d49af29f16b61
- right_scope: permanent
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- right_delta_operation: permanent
- right_projected_target: null
- right_evidence_json: {"feature_description":null,"feature_tags":[],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"duplicate projected removal success beta canyon copper ivory juniper kelp mosaic nickel opal prairie ruby saffron topaz umber violet willow xylophone yarrow zenith\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic remediation with a \"DUPLICATE\" candidate","docstring":null,"data_table":null},{"text":"And the permanent specifications are snapshotted","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout contains \"semantic remediation verified\"","docstring":null,"data_table":null},{"text":"And the semantic review report status is \"PASS\"","docstring":null,"data_table":null},{"text":"And the permanent specifications are unchanged","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: c8e3f02590536302
- right_evidence_hash: dde135010853f8f4
- embedding_score: 0.9560
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 288
- bge_right_tokens: 351
- bge_pair_tokens: 639
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 280
- nli_right_tokens: 347
- nli_pair_tokens: 626
- nli_limit: 512
- nli_refactoring_required: true

## BGE Filtered Candidate
- candidate_id: 5b3c82e66ea751d7
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Real CLI archive retains the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The remediation log survives the archive move"
- embedding_score: 0.9544
- bge_forward_raw: 1.3473
- bge_reverse_raw: 0.7786
- bge_symmetric_raw: 1.0630
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 3a51adf8c78f8137
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Real CLI review completes the remediation log for a stamped duplicate remediation"
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Real CLI review reports successful duplicate remediation"
- embedding_score: 0.9530
- bge_forward_raw: -0.0883
- bge_reverse_raw: 0.8275
- bge_symmetric_raw: 0.3696
- bge_admission: BGE_FILTERED

## BGE Filtered Candidate
- candidate_id: 8a7d5c679fa5adc9
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "Advisory spec review does not create the remediation log"
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Failed review before the semantic stage does not create the remediation log"
- embedding_score: 0.9527
- bge_forward_raw: 0.3966
- bge_reverse_raw: 0.6355
- bge_symmetric_raw: 0.5160
- bge_admission: BGE_FILTERED

## Candidate 6
- candidate_id: 3c7a6c4d914d3da2
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A <repository-state> records null run anchors and reviews without stamp checks"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A <repository-state> records null run anchors and reviews without stamp checks"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":"A <repository-state> records null run anchors and reviews without stamp checks","examples":[{"name":null,"description":null,"tags":[],"table":[["repository-state"],["without Git history"],["inside an unborn Git repository"]]}],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"jackdaw jetty mandrel mist needle urn vane yoke\"","docstring":null,"data_table":null},{"text":"Given a remediation-log project with one \"DUPLICATE\" finding <repository-state>","docstring":null,"data_table":null},{"text":"And the run block of the project is fully declared","docstring":null,"data_table":null},{"text":"When the change review gate runs for running-change","docstring":null,"data_table":null},{"text":"And the reviewer marks each semantic finding as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the reviewer logs the fix \"took the surplus out\" and the commits [\"3c4d5e6f\"]","docstring":null,"data_table":null},{"text":"And the doubled scenario is withdrawn from the change specs","docstring":null,"data_table":null},{"text":"Then the remediation log records the before run anchor as null","docstring":null,"data_table":null},{"text":"And the gate is evaluated a second time","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 6a348c5d0f563b3b
- left_evidence_hash: 6015c0bf20324279
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "A commit fixing two candidates carries one stamp line per candidate"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A commit fixing two candidates carries one stamp line per candidate"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"beacon cobalt fennel iris nettle pumice quill vane\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidates are removed from the active deltas","docstring":null,"data_table":null},{"text":"And the user commits the remediation with one matching stamp per candidate","docstring":null,"data_table":null},{"text":"And the user records both remediation fixes and the commit in the remediation log","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 229e4912bb93ed6c
- right_evidence_hash: b74d49af29f16b61
- embedding_score: 0.9517
- fact_contradiction: true
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 367
- bge_right_tokens: 288
- bge_pair_tokens: 655
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 347
- nli_right_tokens: 280
- nli_pair_tokens: 626
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 7
- candidate_id: 2f784eac42865a54
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A commit fixing two candidates carries one stamp line per candidate"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A commit fixing two candidates carries one stamp line per candidate"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"beacon cobalt fennel iris nettle pumice quill vane\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidates are removed from the active deltas","docstring":null,"data_table":null},{"text":"And the user commits the remediation with one matching stamp per candidate","docstring":null,"data_table":null},{"text":"And the user records both remediation fixes and the commit in the remediation log","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 229e4912bb93ed6c
- left_evidence_hash: b74d49af29f16b61
- right_scope: permanent
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "A DUPLICATE classification without removal remains blocked"
- right_delta_operation: permanent
- right_projected_target: null
- right_evidence_json: {"feature_description":null,"feature_tags":[],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"duplicate removal blocking contract alpha crimson lantern meadow orbit quartz river summit velvet walnut xenon yellow zephyr atlas beacon cobalt delta ember\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic remediation with a \"DUPLICATE\" candidate","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"And stderr contains \"DUPLICATE remediation requires removing the redundant scenario\"","docstring":null,"data_table":null},{"text":"And the semantic review report status is \"BLOCKED\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: d1883f6774f9e69f
- right_evidence_hash: 7e5aa23e3dfbdef1
- embedding_score: 0.9509
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 288
- bge_right_tokens: 295
- bge_pair_tokens: 583
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 280
- nli_right_tokens: 292
- nli_pair_tokens: 571
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 8
- candidate_id: 0783c80984d10537
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A surviving pair records its recomputed embedding score after remediation"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A surviving pair records its recomputed embedding score after remediation"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"abacus lumen lyre moss nickel quadrant scythe xylophone\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the beehive corpus:","docstring":"\nFeature: Beehive inspection\n\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Beehive inspection change\n\n  @givn.added\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"VARIANT\"","docstring":null,"data_table":null},{"text":"And the user records the fix \"consolidated\" and commits [\"2b3c4d5e\"]","docstring":null,"data_table":null},{"text":"And the active corpus variant is consolidated with an explicit parameter","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And the remediation log records after state \"outline-consolidated\" with the recomputed embedding score below the boundary","docstring":null,"data_table":null},{"text":"And the candidate entry records the consolidated outline text verbatim from the active delta","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: e575c90598275cfc
- left_evidence_hash: 5f342fdd95bfab3a
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Removing a DUPLICATE candidate verifies the projected remediation"
- right_delta_operation: modified
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","Removing a DUPLICATE candidate verifies the projected remediation"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.modified"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"cinder gondola locomotive rivet timber trowel tundra umber\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the observatory corpus:","docstring":"\nFeature: Observatory night log\n\n  Scenario: The seeing conditions are noted twice\n    Given a clear night at the observatory\n    When the astronomer notes the seeing\n    Then the night log holds one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Observatory night log change\n\n  @givn.added\n  Scenario: The seeing conditions are noted twice\n    Given a clear night at the observatory\n    When the astronomer notes the seeing\n    Then the night log holds one entry\n","data_table":null},{"text":"And the permanent specifications are snapshotted","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the gate operator marks the candidate \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the gate operator files the fix \"removed\" with the record [\"0a1b2c40\"]","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null},{"text":"And the semantic review report status is \"PASS\"","docstring":null,"data_table":null},{"text":"And the permanent specifications are unchanged","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 9adcaa38cbaab3ac
- right_evidence_hash: 0ce54f6da4407bf5
- embedding_score: 0.9507
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 452
- bge_right_tokens: 417
- bge_pair_tokens: 869
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 460
- nli_right_tokens: 424
- nli_pair_tokens: 883
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 9
- candidate_id: 729a2d93791f3583
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A commit fixing two candidates carries one stamp line per candidate"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A commit fixing two candidates carries one stamp line per candidate"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"beacon cobalt fennel iris nettle pumice quill vane\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidates are removed from the active deltas","docstring":null,"data_table":null},{"text":"And the user commits the remediation with one matching stamp per candidate","docstring":null,"data_table":null},{"text":"And the user records both remediation fixes and the commit in the remediation log","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 229e4912bb93ed6c
- left_evidence_hash: b74d49af29f16b61
- right_scope: permanent
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Changing a candidate scenario invalidates its previous classification"
- right_delta_operation: permanent
- right_projected_target: null
- right_evidence_json: {"feature_description":null,"feature_tags":[],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"regression active candidate invalidation sigma texture vector antler birch comet dune ember fjord granite harbor island juniper kiln lagoon marble nebula olive prism quartz rocket sandal timber umber\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic remediation with a \"DUPLICATE\" candidate","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout contains \"semantic remediation verified\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 2ab40294d1c847b9
- right_evidence_hash: 3974a32ee818d465
- embedding_score: 0.9501
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 288
- bge_right_tokens: 292
- bge_pair_tokens: 580
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 280
- nli_right_tokens: 295
- nli_pair_tokens: 574
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 10
- candidate_id: 7a586045eeecc1ea
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-permanent
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A commit fixing two candidates carries one stamp line per candidate"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A commit fixing two candidates carries one stamp line per candidate"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"beacon cobalt fennel iris nettle pumice quill vane\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidates are removed from the active deltas","docstring":null,"data_table":null},{"text":"And the user commits the remediation with one matching stamp per candidate","docstring":null,"data_table":null},{"text":"And the user records both remediation fixes and the commit in the remediation log","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 229e4912bb93ed6c
- left_evidence_hash: b74d49af29f16b61
- right_scope: permanent
- right_file: "givn/specs/semantic-review/semantic-review.feature"
- right_title: "Changing a permanent endpoint invalidates its prior classification"
- right_delta_operation: permanent
- right_projected_target: null
- right_evidence_json: {"feature_description":null,"feature_tags":[],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":[],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"regression permanent endpoint invalidation tau unmodified baseline acorn beacon copper delta engine frost grove hickory iris jade kettle lemon mosaic nickel opal pebble ruby stone thistle\"","docstring":null,"data_table":null},{"text":"Given a project prepared for semantic remediation with a \"DUPLICATE\" candidate","docstring":null,"data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"When the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And stdout contains \"semantic remediation verified\"","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 5a2647f3e12b8c7d
- right_evidence_hash: 7088475269a8682e
- embedding_score: 0.9499
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 288
- bge_right_tokens: 290
- bge_pair_tokens: 578
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 280
- nli_right_tokens: 288
- nli_pair_tokens: 567
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 11
- candidate_id: f929265afe0a5a79
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A logged pair scored by BGE and NLI records their evidence states"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A logged pair scored by BGE and NLI records their evidence states"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added","rulebook"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"accordion almanac bone hammock heather rivet spar zephyr\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the greenhouse corpus:","docstring":"\nFeature: Greenhouse watering plan\n\n  Scenario: The orchid watering is scheduled twice\n    Given an orchid bench with a moisture probe\n    When the gardener plans the watering\n    Then the plan shows one schedule entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Greenhouse watering plan change\n\n  @givn.added\n  Scenario Outline: The orchid watering is scheduled twice\n    Given an orchid bench with a moisture probe\n    When the gardener plans the watering\n    Then the plan shows one schedule entry\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits non-zero","docstring":null,"data_table":null},{"text":"And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: 80e99e00b6e96e65
- left_evidence_hash: 68f315a632c33b6c
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "Rerunning review with unchanged endpoints does not grow the remediation log"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","Rerunning review with unchanged endpoints does not grow the remediation log"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"elm ibis javelin latch orchard parapet plum summit\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the bakery corpus:","docstring":"\nFeature: Bakery ledger\n\n  Scenario: The sourdough batch is logged twice\n    Given a proofing schedule for the night\n    When the baker records the sourdough batch\n    Then the ledger shows one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Bakery ledger change\n\n  @givn.added\n  Scenario: The sourdough batch is logged twice\n    Given a proofing schedule for the night\n    When the baker records the sourdough batch\n    Then the ledger shows one entry\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change` again without edits","docstring":null,"data_table":null},{"text":"Then the remediation log still contains exactly one candidate entry and one run record","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: 25250c1cf34d6111
- right_evidence_hash: 01383fed84b4e70f
- embedding_score: 0.9495
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 314
- bge_right_tokens: 312
- bge_pair_tokens: 626
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 324
- nli_right_tokens: 325
- nli_pair_tokens: 648
- nli_limit: 512
- nli_refactoring_required: true

## Candidate 12
- candidate_id: d97b56c16f480050
- classification: UNCERTAIN
- rationale: null
- scope: delta-to-delta
- left_scope: current-change
- left_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- left_title: "A surviving pair records its recomputed embedding score after remediation"
- left_delta_operation: added
- left_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","A surviving pair records its recomputed embedding score after remediation"]
- left_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"abacus lumen lyre moss nickel quadrant scythe xylophone\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the beehive corpus:","docstring":"\nFeature: Beehive inspection\n\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n      --- delta ---\n@givn.delta @running-change\nFeature: Beehive inspection change\n\n  @givn.added\n  Scenario Outline: The queen sighting is logged twice\n    Given an inspected beehive in summer\n    When the beekeeper logs the queen sighting\n    Then the inspection card holds one entry\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"VARIANT\"","docstring":null,"data_table":null},{"text":"And the user records the fix \"consolidated\" and commits [\"2b3c4d5e\"]","docstring":null,"data_table":null},{"text":"And the active corpus variant is consolidated with an explicit parameter","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the command exits 0","docstring":null,"data_table":null},{"text":"And the remediation log records after state \"outline-consolidated\" with the recomputed embedding score below the boundary","docstring":null,"data_table":null},{"text":"And the candidate entry records the consolidated outline text verbatim from the active delta","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- left_content_hash: e575c90598275cfc
- left_evidence_hash: 5f342fdd95bfab3a
- right_scope: current-change
- right_file: "givn/changes/semantic-remediation-log/specs/semantic-remediation-log/semantic-remediation-log.feature"
- right_title: "The passing rerun completes the after state, and a further rerun leaves the log unchanged"
- right_delta_operation: added
- right_projected_target: ["givn/specs/semantic-remediation-log/semantic-remediation-log.feature","The passing rerun completes the after state, and a further rerun leaves the log unchanged"]
- right_evidence_json: {"feature_description":null,"feature_tags":["givn.delta","semantic-remediation-log"],"rule_description":null,"rule_tags":[],"scenario_description":null,"scenario_tags":["givn.added"],"outline_name":null,"examples":[],"feature_background_steps":[],"rule_background_steps":[],"normalized_steps":[{"text":"Given the remediation case carries the distinguishing marker \"cleat domino lantern mist oarlock sandal thistle wren\"","docstring":null,"data_table":null},{"text":"Given the remediation fixture files carry the ferry corpus:","docstring":"\nFeature: Ferry manifest\n\n  Scenario: The car deck count is noted twice\n    Given a ferry at the loading ramp\n    When the steward counts the cars\n    Then the manifest shows one deck count\n      --- delta ---\n@givn.delta @running-change\nFeature: Ferry manifest change\n\n  @givn.added\n  Scenario: The car deck count is noted twice\n    Given a ferry at the loading ramp\n    When the steward counts the cars\n    Then the manifest shows one deck count\n","data_table":null},{"text":"And the project declares a complete run declaration","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"And the user classifies every semantic review candidate as \"DUPLICATE\"","docstring":null,"data_table":null},{"text":"And the user records the fix \"removed the duplicate\" and commits [\"1a2b3c4d\"]","docstring":null,"data_table":null},{"text":"And the classified duplicate candidate is removed from the active delta","docstring":null,"data_table":null},{"text":"And the user runs `givn check review --change running-change`","docstring":null,"data_table":null},{"text":"Then the review passes with the remediation receipt","docstring":null,"data_table":null},{"text":"And the remediation log contains an after run record","docstring":null,"data_table":null},{"text":"And the removed candidate entry records after state \"endpoint-absent\" with a null after score","docstring":null,"data_table":null},{"text":"And the active remediation log bytes are snapshotted","docstring":null,"data_table":null},{"text":"When the user runs `givn check review --change running-change` again without edits","docstring":null,"data_table":null},{"text":"Then the remediation log bytes equal the active log snapshot","docstring":null,"data_table":null}],"model_id":"intfloat/multilingual-e5-large","index_version":1,"fingerprint_schema_version":1,"evidence_schema_version":1,"report_schema_version":1}
- right_content_hash: c5eca3a2e84a1eaf
- right_evidence_hash: c92974ea2adedc0a
- embedding_score: 0.9492
- fact_contradiction: false
- bge_forward_raw: null
- bge_reverse_raw: null
- bge_symmetric_raw: null
- bge_admission: BGE_TOKEN_CAP
- bge_left_tokens: 452
- bge_right_tokens: 501
- bge_pair_tokens: 953
- bge_limit: 512
- bge_refactoring_required: true
- nli_forward_entailment: null
- nli_forward_neutral: null
- nli_forward_contradiction: null
- nli_reverse_entailment: null
- nli_reverse_neutral: null
- nli_reverse_contradiction: null
- nli_symmetric_entailment: null
- nli_symmetric_neutral: null
- nli_symmetric_contradiction: null
- nli_admission: NLI_TOKEN_CAP
- nli_left_tokens: 460
- nli_right_tokens: 510
- nli_pair_tokens: 969
- nli_limit: 512
- nli_refactoring_required: true
