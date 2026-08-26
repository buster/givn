Feature: Semantic review output leads with decision-relevant evidence

  @review-output
  Scenario: Default output reports only recommended second-stage pairs
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout lists every recommended second-stage pair exactly once under its stage with its admission status
    And  stdout contains no detail line for a filtered or unscored second-stage pair
    And  token-capped pairs keep their guidance lines visible with pair and limit token counts
    And  the stage status lines, accounting counters, and label distributions remain printed

  @review-output
  Scenario: The raw E5 list is verbose-only detail
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout contains no raw E5 similarity-candidate detail line
    And  stdout still reports the E5 evaluated-pairs count
    When  the user runs the advisory semantic review for "running-change" again with verbose output
    Then  stdout lists every E5 similarity candidate above the threshold in its established format

  @review-output
  Scenario: E5-only hits without a recommendation are not problems
    Given  a project prepared for semantic review whose second-stage evidence filters every candidate
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits 0
    And  stdout reports the recommendation accounting for both stages with zero recommended pairs

  @review-output
  Scenario: A recommended pair from either stage is the problem signal
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stderr names the flagged recommended pair count
    And  stderr names the persisted findings path under the change directory
    And  givn/changes/running-change/review-findings.json exists

  @review-output
  Scenario: A BGE-only recommendation flags the review without NLI participation
    Given  a project prepared for semantic review whose BGE evidence recommends one pair while NLI stays disabled
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stderr names the single recommended candidate as the failure reason

  @review-output
  Scenario: An NLI-only recommendation flags the review without BGE participation
    Given  a project prepared for semantic review whose NLI evidence recommends one pair while BGE stays disabled
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stderr names the single recommended candidate as the failure reason

  @review-output
  Scenario: Without participating second-stage layers the review stays advisory
    Given  a project prepared for semantic review with a similar active delta and second-stage evidence disabled
    When  the user runs the advisory semantic review for "running-change" with second-stage evidence disabled
    Then  the command exits 0
    And  stdout lists every E5 similarity candidate above the threshold in its established format
    And  stderr names no similarity gate failure

  @review-output
  Scenario: Recommended pairs are ordered by model confidence before embedding score
    Given  a project prepared for semantic review whose recommended pairs rank differently by confidence than by embedding score
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  the recommended NLI pairs are listed strongest symmetric entailment first
    And  the recommended BGE pairs are listed highest symmetric raw score first
    And  neither listing follows embedding-score order

  @review-output
  Scenario: Retained text lines keep their established wording
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout keeps the exact stage status line naming the model, pair budget, considered pairs, and diagnostic-only disclaimer
    And  stdout keeps the exact accounting line naming raw, maximum, recommended, filtered, and unscored counts
    And  stdout keeps the exact distribution line naming entailment, neutral, and contradiction counts
    And  each reported pair keeps the exact established detail line including its advisory disclaimer

  @review-output
  Scenario: Verbose output lists every scored second-stage pair
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change" with verbose output
    Then  the command exits non-zero
    And  stdout lists every filtered pair in embedding-score order after the recommended pairs
    And  stdout lists any unscored or token-capped pair with its admission status
    And  no listing limit applies to any stage

  @review-output
  Scenario: Default output collapses artifact resolution chatter to one line per stage
    Given  a project prepared for semantic review whose stubbed stages emit repeated resolution notices
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stderr shows exactly one artifact resolution summary line per participating stage
    And  stderr contains no repeated resolution lines for the same stage

  @review-output
  Scenario: Verbose output restores full artifact resolution detail
    Given  a project prepared for semantic review whose stubbed stages emit repeated resolution notices
    When  the user runs the advisory semantic review for "running-change" with verbose output
    Then  the command exits non-zero
    And  stderr contains the full repeated resolution lines for every participating stage

  @review-output
  Scenario: The truncation disclaimer appears only for a truncated listing
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout contains no truncation disclaimer for either second-stage stage
    When  the user runs the advisory semantic review for "running-change" again with verbose output
    Then  stdout still contains no truncation disclaimer

  @review-output
  Scenario: A deprecated report-limit setting is accepted silently without effect
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    And  the deprecated report-limit environment variable is set to "1"
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  every recommended second-stage pair is still listed
    And  every line except elapsed-runtime values matches the same run without the variable
    And  no warning or error names the deprecated variable

  @review-output
  Scenario: Structured JSON output is one complete review document
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change" with structured JSON output
    Then  the command exits non-zero
    And  stdout parses as exactly one JSON document
    And  the document declares schema version 1 and the reviewed change id
    And  the document contains no log lines outside the JSON document

  @review-output
  Scenario: Structured JSON verdict and stage summaries record run accounting
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change" with structured JSON output
    Then  the command exits non-zero
    And  the verdict records the flagged recommended pair count, whether the threshold fallback applied, the threshold, and the E5 candidate count
    And  each participating stage summary records its status, model identity, pair counters, label distribution where applicable, and runtime

  @review-output
  Scenario: Structured JSON candidates record endpoints, labels, and evidence totals
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change" with structured JSON output
    Then  the command exits non-zero
    And  every candidate records both endpoint titles and files, the embedding score, the admission labels of every participating stage, and its evidence scores
    And  the document reports the combined recommendation total across participating stages
    And  the document records the index refresh outcome instead of printing banner lines

  @review-output
  Scenario: JSON counters match the text counters of the same run
    Given  a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    When  the user runs the advisory semantic review for "running-change" once as text and once as structured JSON
    Then  both runs reach the same verdict
    And  every count printed by the text run has an equal field value in the JSON document
    And  both runs agree on the candidate identities and their admission labels

  @review-output
  Scenario: Structured JSON preserves exit codes and failure messages
    Given  a project prepared for semantic review whose second-stage evidence recommends at least one pair
    When  the user runs the advisory semantic review for "running-change" as text
    Then  the command exits non-zero and stderr names the flagged recommendation failure
    When  the user runs the advisory semantic review for "running-change" with structured JSON output
    Then  the command exits non-zero and stderr names the identical failure message unchanged

  @review-output
  Scenario: Flagged recommendations persist their findings beside the change
    Given  a project prepared for semantic review whose second-stage evidence recommends at least one pair
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  the failure message names the persisted findings path under the change directory
    And  givn/changes/running-change/review-findings.json exists
    And  the file parses as the same schema-version-1 document emitted on stdout
    And  a stale findings document left by an earlier run is replaced

  @review-output
  Scenario: Successful reviews write no findings document
    Given  a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits 0
    And  no findings document exists under the change directory

  @review-output
  Scenario: An unwritable findings location warns without masking the flagged failure
    Given  a project prepared for semantic review whose second-stage evidence recommends at least one pair
    And  the findings path cannot be written
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero and stderr names the flagged recommendation failure
    And  stderr warns that the findings document could not be written and names the attempted path

  @review-output
  Scenario: Token-capped BGE pairs keep their guidance lines visible
    Given  a project prepared for semantic review whose BGE evidence token-caps some pairs while the second stage advertises one recommendation
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout shows the BGE token-cap guidance line naming the capped pair and its pair and limit token counts
    And  stdout still shows the NLI token-cap guidance line

  @review-output
  Scenario: E5 token-cap evidence prints in the advisory review output
    Given  a project prepared for semantic review whose E5 evidence token-caps some scenarios in the advisory output
    When  the user runs the advisory semantic review for "running-change"
    Then  the command exits non-zero
    And  stdout shows the E5 token-cap advisory count and candidate lines

  @review-output
  Scenario: A clean corpus below the threshold reports no problem in fallback mode
    Given  a project prepared for semantic review with a dissimilar active delta and second-stage evidence disabled
    When  the user runs the advisory semantic review for "running-change" with second-stage evidence disabled
    Then  the command exits 0
    And  stdout reports no scenario pair at or above the fixed threshold
    And  no findings document exists under the change directory

  @e2e @review-output
  Scenario: Real CLI review surfaces only recommended second-stage pairs
    Given  an end-to-end project whose semantic review recommends one pair and filters many
    When  the end-to-end user runs the advisory semantic review for "running-change" through the givn CLI
    Then  the command exits non-zero
    And  stdout shows the recommended pair's detail line
    And  stdout shows no FILTERED detail line

  @e2e @review-output
  Scenario: Real CLI verbose review lists all second-stage evidence
    Given  an end-to-end project whose semantic review recommends one pair and filters many
    When  the end-to-end user runs the advisory semantic review with verbose output through the givn CLI
    Then  the command exits non-zero
    And  stdout shows the FILTERED detail rows
    And  stdout shows no truncation disclaimer

  @e2e @review-output
  Scenario: Real CLI JSON review emits one structured document
    Given  an end-to-end project whose semantic review recommends one pair and filters many
    When  the end-to-end user runs the advisory semantic review with structured JSON output through the givn CLI
    Then  the command exits non-zero
    And  stdout parses as a schema-version-1 review document
    And  the JSON counters equal those of an equivalent text run

  @e2e @review-output
  Scenario: Real CLI persists findings beside the change after flagged recommendations
    Given  an end-to-end project whose second-stage evidence recommends at least one pair
    When  the end-to-end user runs the advisory semantic review for "running-change" through the givn CLI
    Then  the command exits non-zero
    And  givn/changes/running-change/review-findings.json exists beside the change
    And  the persisted document matches the document emitted on stdout

  @e2e @review-output
  Scenario: Real CLI blocking review flags recommendations through the workflow gate
    Given  an end-to-end project whose second-stage evidence recommends at least one pair
    When  the end-to-end user runs the blocking workflow review for "running-change" through the givn CLI
    Then  the command exits non-zero because the recommended candidate is unresolved
    And  the semantic review report exists
    And  the persisted semantic review report records the blocked status
    And  no findings document exists under the change directory
    And  the command stdout names the complete saved semantic-review report path
