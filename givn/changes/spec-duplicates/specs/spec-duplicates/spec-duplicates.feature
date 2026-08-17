# User Interaction Inventory:
# - invoke `givn spec duplicates` and consume the complete advisory audit, including its JSON output variant

@givn.delta @spec-duplicates

Feature: Audit duplicate and related permanent scenarios

  @givn.added @e2e @wip
  Scenario: Maintainer inspects the complete permanent scenario audit
    Given an end-to-end project with permanent scenarios containing duplicate and related behaviour
    When the end-to-end user runs `givn spec duplicates`
    Then the command exits 0
    And stdout starts with the exact duplicate-audit header and counts
    And stdout uses candidate fields in the exact order `rank,kind,left,right,relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score`
    And stdout identifies rank 1 as the exact duplicate between `givn/specs/auth/auth.feature` scenario "Unknown provider is rejected" and `givn/specs/provider/provider.feature` scenario "Provider is rejected"
    And stdout identifies rank 2 as the structural overlap between `givn/specs/provider/provider.feature` scenario "Provider exits successfully" and `givn/specs/provider/provider.feature` scenario "Provider exits with a warning"
    And stdout identifies rank 3 as the semantic candidate between `givn/specs/auth/auth.feature` scenario "Token is required" and `givn/specs/token/token.feature` scenario "Authentication needs a token"
    And every text candidate rank is contiguous from 1 through the reported candidate count

  @givn.added
  Scenario: Exact duplicates in one feature are reported together
    Given a permanent feature contains two scenarios with identical behaviour
    When the user runs `givn spec duplicates`
    Then the audit reports candidate count 1
    And candidate rank 1 is classified as `exact_duplicate`
    And candidate rank 1 has left identity ("givn/specs/provider/provider.feature", "Provider is rejected")
    And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Unknown provider is rejected")
    And candidate rank 1 has relation `identical_behavior`
    And candidate rank 1 has score `null`

  @givn.added
  Scenario: Exact duplicates across features are reported together
    Given two permanent features contain scenarios with identical behaviour
    When the user runs `givn spec duplicates`
    Then the audit reports candidate count 1
    And candidate rank 1 is classified as `exact_duplicate`
    And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Unknown provider is rejected")
    And candidate rank 1 has right identity ("givn/specs/provider/provider.feature", "Provider is rejected")
    And candidate rank 1 has relation `identical_behavior`
    And candidate rank 1 has score `null`

  @givn.added @wip
  Scenario: Structurally overlapping scenarios retain their distinguishing facts
    Given permanent scenarios share the same masked assertion shape but have different fingerprint facts
    When the user runs `givn spec duplicates`
    Then candidate rank 1 is classified as `structural_overlap`
    And candidate rank 1 has relation `same_assertion_shape`
    And candidate rank 1 has left fingerprint facts `{"exit_code":"0","contains_polarity":{"0":"positive"},"tables":{"provider":"aws"},"done":false,"tty":false}`
    And candidate rank 1 has right fingerprint facts `{"exit_code":"2","contains_polarity":{"0":"negative"},"tables":{"provider":"gcp"},"done":true,"tty":true}`
    And candidate rank 1 has distinguishing facts `[{"field":"exit_code","left":"0","right":"2"},{"field":"contains_polarity[0]","left":"positive","right":"negative"},{"field":"tables[provider]","left":"aws","right":"gcp"},{"field":"done","left":false,"right":true},{"field":"tty","left":false,"right":true}]`
    And candidate rank 1 has fact contradiction `true`
    And candidate rank 1 has no Given/When action-structure evidence

  @givn.added @wip
  Scenario: A structural subset is reported as an overlap
    Given the permanent scenario "Short provider assertion" at "givn/specs/a/a.feature" has the assertion sequence of "Long provider assertion" at "givn/specs/b/b.feature"
    When the user runs `givn spec duplicates`
    Then candidate rank 1 is classified as `structural_overlap`
    And candidate rank 1 has left identity ("givn/specs/a/a.feature", "Short provider assertion")
    And candidate rank 1 has right identity ("givn/specs/b/b.feature", "Long provider assertion")
    And candidate rank 1 has relation `left_assertions_subset_of_right`
    And candidate rank 1 has assertion shapes `{"left":["the command exits *"],"right":["the command exits *","the output contains *"]}`
    And candidate rank 1 has score `null`

  @givn.added @wip
  Scenario: Semantically related scenarios are reported as candidates
    Given the permanent scenario "Token is required" at "givn/specs/auth/auth.feature" and "Authentication needs a token" at "givn/specs/token/token.feature" describe closely related behaviour with different wording
    When the user runs `givn spec duplicates`
    Then the audit reports candidate count 1
    And candidate rank 1 is classified as `semantic_candidate`
    And candidate rank 1 has left identity ("givn/specs/auth/auth.feature", "Token is required")
    And candidate rank 1 has right identity ("givn/specs/token/token.feature", "Authentication needs a token")
    And candidate rank 1 has relation `semantic_similarity`
    And candidate rank 1 has score `0.9600`
    And candidate rank 1 has null assertion shape and null fingerprint facts
    And candidate rank 1 has empty distinguishing facts and fact contradiction `false`
    And the semantic candidate is advisory and is not an automatic duplicate decision

  @givn.added @wip
  Scenario: The audit reports candidates beyond the interactive top five
    Given the permanent corpus contains more than five duplicate and related-scenario candidates
    When the user runs `givn spec duplicates`
    Then the audit reports candidate count 6
    And the audit includes the candidate at rank 6
    And candidate ranks are exactly `1,2,3,4,5,6`
    And the highest rank equals the candidate count

  @givn.added @wip
  Scenario: Repeating an unchanged audit produces the same report
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates` twice without changing the corpus
    Then both audit reports are byte-for-byte identical
    And both reports have the same scanned scenario count, candidate count, ranks, and evidence

  @givn.added @wip
  Scenario: JSON output identifies every audit candidate
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates --format json`
    Then stdout is a valid JSON audit document
    And the JSON schema_version is exactly 1
    And the JSON top-level keys are exactly `schema_version,scanned_scenarios,candidate_count,candidates` in that order
    And every JSON candidate has keys exactly `rank,kind,left,right,evidence` in that order
    And every JSON endpoint has keys exactly `file,title` in that order
    And every JSON evidence object has keys exactly `relation,assertion_shape,facts,distinguishing_facts,fact_contradiction,score` in that order
    And every JSON candidate has an exact classification, both scenario identities, a relation, and a rank
    And the JSON candidates follow the deterministic audit ordering
    And JSON ranks are contiguous from 1 through candidate_count
    And JSON scanned_scenarios equals the number of eligible non-removed permanent scenarios

  @givn.added @wip
  Scenario: Auditing leaves permanent specifications unchanged
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And the permanent specifications are snapshotted
    When the user runs `givn spec duplicates`
    Then the permanent specifications are unchanged byte-for-byte

  @givn.added @wip
  Scenario: Auditing leaves the persistent retrieval index unchanged
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And the persistent retrieval index is built and snapshotted
    When the user runs `givn spec duplicates`
    Then the persistent retrieval index is unchanged byte-for-byte

  @givn.added @wip
  Scenario: Active change scenarios are not part of the permanent audit
    Given a permanent scenario corpus with one related scenario
    And an active change contains another scenario with related behaviour
    When the user runs `givn spec duplicates`
    Then the audit reports only relationships between permanent scenarios
    And the active change scenario identity is absent from every candidate
    And every candidate endpoint file starts with `givn/specs/`

  @givn.added @wip
  Scenario: An audit with no candidates reports an empty result
    Given a permanent scenario corpus with no duplicate or related behaviour
    When the user runs `givn spec duplicates`
    Then the command exits 0
    And the text report is exactly `Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n`
    And the JSON report has scanned_scenarios 2, candidate_count 0, and an empty candidates array

  @givn.added @wip
  Scenario: The audit reports retrieval unavailable without the retrieval feature
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates` through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"

  @givn.added @wip
  Scenario: The audit reports an unavailable retrieval model
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And the retrieval model is unavailable
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains "retrieval model is unavailable"

  @givn.added @wip
  Scenario: A malformed permanent feature reports its path
    Given a malformed permanent feature at "givn/specs/broken/broken.feature"
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains the project-relative path "givn/specs/broken/broken.feature"
    And stdout is empty

  @givn.added @wip
  Scenario: An unreadable permanent feature reports its path
    Given an unreadable permanent feature at "givn/specs/private/private.feature"
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains the project-relative path "givn/specs/private/private.feature"
    And stdout is empty

  @givn.added @wip
  Scenario: A missing permanent specifications directory is an empty corpus
    Given a project without a `givn/specs/` directory
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the JSON report has scanned_scenarios 0, candidate_count 0, and an empty candidates array
    And the persistent retrieval index remains absent

  @givn.added @wip
  Scenario: Archived scenarios are excluded from the audit
    Given a permanent corpus with one candidate
    And the archive contains a matching scenario at "givn/archive/old-change/specs/old/old.feature"
    When the user runs `givn spec duplicates --format json`
    Then every JSON candidate endpoint file starts with `givn/specs/`
    And no JSON candidate identity contains `givn/archive/old-change/`
    And the archived matching scenario does not change candidate_count

  @givn.added @wip
  Scenario: Auditing leaves active changes and archive contents unchanged
    Given a permanent corpus with duplicate and related-scenario candidates
    And active change files and archive contents are snapshotted
    When the user runs `givn spec duplicates`
    Then active change files are unchanged byte-for-byte
    And archive contents are unchanged byte-for-byte

  @givn.added @wip
  Scenario: An absent persistent index is ignored and remains absent
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And no persistent retrieval index exists
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the audit uses current permanent files and transient embeddings
    And the persistent retrieval index remains absent

  @givn.added @wip
  Scenario: A malformed persistent index is ignored and unchanged
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And the persistent retrieval index contains the bytes "not-json"
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the audit report is valid JSON with the permanent candidates
    And the persistent retrieval index still contains the bytes "not-json"

  @givn.added @wip
  Scenario: A semantic candidate at the inclusive threshold is reported
    Given permanent scenarios have no exact or structural relationship
    And the test score for their normalized endpoint pair is `0.9500`
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And candidate rank 1 is classified as `semantic_candidate`
    And candidate rank 1 has relation `semantic_similarity`
    And candidate rank 1 has score `0.9500`

  @givn.added @wip
  Scenario: A semantic pair below the threshold is not reported
    Given permanent scenarios have no exact or structural relationship
    And the test score for their normalized endpoint pair is `0.9499`
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the JSON report has candidate_count 0
    And no candidate has score `0.9499`

  @givn.added @wip
  Scenario: Candidate classes take precedence and equal semantic scores use endpoint order
    Given the corpus contains one exact duplicate, one structural overlap, and two semantic candidates
    And the two semantic candidates have equal test score `0.9600`
    When the user runs `givn spec duplicates --format json`
    Then candidate rank 1 is the exact duplicate
    And candidate rank 2 is the structural overlap
    And candidate ranks 3 and 4 are semantic candidates with score `0.9600`
    And equal-score semantic candidates are ordered by left file, left title, right file, and right title
    And candidate ranks are exactly `1,2,3,4`
