# User Interaction Inventory:
# - invoke `givn spec duplicates` and consume the complete advisory audit, including its JSON output variant

@givn.delta @spec-duplicates

Feature: Audit duplicate and related permanent scenarios

  @givn.added @e2e
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
    The boundary under test is title-only variation within one source-file identity.
    Given the permanent feature file "givn/specs/provider/provider.feature" contains two separately named scenarios with the same canonical body
    And the corpus cardinality is one source file with two scenario nodes
    When the user runs `givn spec duplicates`
    Then the one-file audit contains exactly one candidate
    And the one-file result is classified as `exact_duplicate`
    And the one-file result has left identity ("givn/specs/provider/provider.feature", "Provider is rejected")
    And the one-file result has right identity ("givn/specs/provider/provider.feature", "Unknown provider is rejected")
    And the one-file result has relation `identical_behavior`
    And candidate rank 1 has an intra-file identity boundary
    And title-only variation is collapsed under one source-file identity
    And both scenario names remain distinct under that identity
    And sibling scenarios remain co-located after title normalization
    And the one-file result has score `null`

  @givn.added
  Scenario: Exact duplicates across features are reported together
    The boundary under test is source-file variation between two endpoint identities.
    Given the permanent feature files "givn/specs/auth/auth.feature" and "givn/specs/provider/provider.feature" each contain one separately named scenario with the same canonical body
    And the corpus cardinality is two source files with one scenario node per file
    When the user runs `givn spec duplicates`
    Then the two-file audit contains exactly one candidate
    And the two-file result is classified as `exact_duplicate`
    And the two-file result has left identity ("givn/specs/auth/auth.feature", "Unknown provider is rejected")
    And the two-file result has right identity ("givn/specs/provider/provider.feature", "Provider is rejected")
    And the two-file result has relation `identical_behavior`
    And candidate rank 1 has an inter-file identity boundary
    And source-file variation remains visible despite identical scenario bodies
    And each source file supplies one named endpoint
    And disjoint source paths remain distinct after body normalization
    And the two-file result has score `null`

  @givn.added
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

  @givn.added
  Scenario: A structural subset is reported as an overlap
    Given the permanent scenario "Short provider assertion" at "givn/specs/a/a.feature" has the assertion sequence of "Long provider assertion" at "givn/specs/b/b.feature"
    When the user runs `givn spec duplicates`
    Then candidate rank 1 is classified as `structural_overlap`
    And candidate rank 1 has left identity ("givn/specs/a/a.feature", "Short provider assertion")
    And candidate rank 1 has right identity ("givn/specs/b/b.feature", "Long provider assertion")
    And candidate rank 1 has relation `left_assertions_subset_of_right`
    And candidate rank 1 has assertion shapes `{"left":["the command exits *"],"right":["the command exits *","the output contains *"]}`
    And candidate rank 1 has score `null`

  @givn.added
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

  @givn.added
  Scenario: The audit reports candidates beyond the interactive top five
    Given the permanent corpus contains more than five duplicate and related-scenario candidates
    When the user runs `givn spec duplicates`
    Then the audit reports candidate count 6
    And the audit includes the candidate at rank 6
    And candidate ranks are exactly `1,2,3,4,5,6`
    And the highest rank equals the candidate count

  @givn.added
  Scenario: Repeating an unchanged audit produces the same report
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates` twice without changing the corpus
    Then both audit reports are byte-for-byte identical
    And both reports have the same scanned scenario count, candidate count, ranks, and evidence

  @givn.added
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

  @givn.added
  Scenario: Auditing leaves permanent specifications unchanged
    Given the permanent-specification audit corpus contains duplicate and related scenarios
    And the canonical permanent feature bytes are snapshotted before auditing
    When the user runs `givn spec duplicates`
    Then the canonical permanent feature bytes are unchanged byte-for-byte

  @givn.added
  Scenario: Auditing leaves the persistent retrieval index unchanged
    Given the persistent-index audit corpus contains duplicate and related scenarios
    And the built `.spec-index.json` bytes are snapshotted before auditing
    When the user runs `givn spec duplicates`
    Then the built `.spec-index.json` bytes are unchanged byte-for-byte

  @givn.added
  Scenario: Active change scenarios are not part of the permanent audit
    Given a permanent scenario corpus with one related scenario
    And an active change contains another scenario with related behaviour
    When the user runs `givn spec duplicates`
    Then the audit reports only relationships between permanent scenarios
    And the active change scenario identity is absent from every candidate
    And every candidate endpoint file starts with `givn/specs/`

  @givn.added
  Scenario: An audit with no candidates reports an empty result
    Given a permanent scenario corpus with no duplicate or related behaviour
    When the user runs `givn spec duplicates`
    Then the command exits 0
    And the text report is exactly `Givn spec duplicates\nScanned scenarios: 2\nCandidate count: 0\n`
    And the JSON report has scanned_scenarios 2, candidate_count 0, and an empty candidates array

  @givn.added
  Scenario: The audit reports retrieval unavailable without the retrieval feature
    Given a permanent corpus is passed to a binary compiled without the spec-retrieval capability
    When the user runs `givn spec duplicates` through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"

  @givn.added
  Scenario: The audit reports an unavailable retrieval model
    Given a feature-enabled permanent corpus requires runtime E5 model loading
    And runtime E5 model loading fails before candidate scoring
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains "retrieval model is unavailable"

  @givn.added
  Scenario: A malformed permanent feature reports its path
    Given a permanent feature with invalid Gherkin syntax at "givn/specs/broken/broken.feature"
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains the project-relative path "givn/specs/broken/broken.feature"
    And stdout is empty

  @givn.added
  Scenario: An unreadable permanent feature reports its path
    Given a permanent feature that exists but cannot be read at "givn/specs/private/private.feature"
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains the project-relative path "givn/specs/private/private.feature"
    And stdout is empty

  @givn.added
  Scenario: A missing permanent specifications directory is an empty corpus
    Given a project without a `givn/specs/` directory
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the JSON report has scanned_scenarios 0, candidate_count 0, and an empty candidates array
    And the persistent retrieval index remains absent

  @givn.added
  Scenario: Archived scenarios are excluded from the audit
    Given a permanent corpus with one candidate
    And the archive contains a matching scenario at "givn/archive/old-change/specs/old/old.feature"
    When the user runs `givn spec duplicates --format json`
    Then every JSON candidate endpoint file starts with `givn/specs/`
    And no JSON candidate identity contains `givn/archive/old-change/`
    And the archived matching scenario does not change candidate_count

  @givn.added
  Scenario: Auditing leaves active changes and archive contents unchanged
    Given the audit has separate active-change and archive trees alongside permanent candidate specifications
    And the active-change and archive trees are snapshotted before the read-only audit
    When the user runs `givn spec duplicates`
    Then active-change files are unchanged byte-for-byte
    And archive files are unchanged byte-for-byte

  @givn.added
  Scenario: An absent persistent index is ignored and remains absent
    Given an audit corpus with an absent persistent-index file
    And the persistent retrieval index file is absent before auditing
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the audit uses current permanent files and transient embeddings
    And the persistent retrieval index remains nonexistent

  @givn.added
  Scenario: A malformed persistent index is ignored and unchanged
    Given an audit corpus with malformed persistent-index bytes
    And the persistent retrieval index file contains invalid JSON bytes "not-json"
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the audit report is valid JSON with the permanent candidates
    And the malformed index bytes remain exactly "not-json"

  @givn.added
  Scenario: A semantic candidate at the inclusive threshold is reported
    Given permanent scenarios exercise inclusive semantic admission with no exact or structural relationship
    And the normalized endpoint pair is assigned the inclusive admission score `0.9500`
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And candidate rank 1 is admitted as a `semantic_candidate`
    And candidate rank 1 has relation `semantic_similarity`
    And candidate rank 1 has score `0.9500`

  @givn.added
  Scenario: A semantic pair below the threshold is not reported
    Given permanent scenarios exercise below-boundary semantic exclusion with no exact or structural relationship
    And the normalized endpoint pair is assigned the exclusion score `0.9499`
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the JSON report excludes all semantic candidates
    And no candidate has score `0.9499`

  @givn.added
  Scenario: Candidate classes take precedence and equal semantic scores use endpoint order
    Given the corpus contains one exact duplicate, one structural overlap, and two semantic candidates
    And the two semantic candidates have equal test score `0.9600`
    When the user runs `givn spec duplicates --format json`
    Then candidate rank 1 is the exact duplicate
    And candidate rank 2 is the structural overlap
    And candidate ranks 3 and 4 are semantic candidates with score `0.9600`
    And equal-score semantic candidates are ordered by left file, left title, right file, and right title
    And candidate ranks are exactly `1,2,3,4`

  @givn.added
  Scenario: Rule-nested permanent scenarios are included in the audit
    Given a permanent feature has two identical scenarios nested under a Rule
    When the user runs `givn spec duplicates --format json`
    Then the JSON report scans 2 eligible scenarios
    And the Rule-nested scenarios produce one exact duplicate
    And the Rule-nested candidate retains both scenario identities

  @givn.added
  Scenario: A blocked permanent specifications path reports its I/O error
    Given the permanent specifications path is blocked by a non-directory
    When the user runs `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains the project-relative path "givn/specs"
    And stdout is empty

  @givn.added
  Scenario: Removed permanent scenarios are excluded from the audit
    Given a permanent feature contains a removed duplicate beside a retained scenario
    When the user runs `givn spec duplicates --format json`
    Then the JSON report scans 1 eligible scenario
    And the removed scenario does not create a candidate
    And no candidate identity contains "Removed provider is rejected"

  @givn.added
  Scenario: Structural relations are ordered by relation class
    Given a permanent corpus contains same-shape, left-subset, and right-subset overlaps
    When the user runs `givn spec duplicates --format json`
    Then structural candidates are ordered `same_assertion_shape,left_assertions_subset_of_right,right_assertions_subset_of_left`
    And every structural candidate has a null semantic score

  @givn.added
  Scenario: Equal semantic scores use right endpoint order
    Given two semantic candidates share one left endpoint and have distinct right endpoints
    And both right-endpoint candidates have equal test score `0.9600`
    When the user runs `givn spec duplicates --format json`
    Then equal-score candidates with one left endpoint are ordered by right endpoint
    And both right-endpoint candidates retain score `0.9600`

  @givn.added
  Scenario: Invalid duplicate score overrides are rejected
    Given a permanent corpus for duplicate score validation
    And the duplicate score seam has these invalid entries:
      | case | raw | expected |
      | empty input |  | must not be empty |
      | invalid UTF-8 | __INVALID_UTF8__ | not valid UTF-8 |
      | empty entry | givn/specs/score/a.feature\|A::givn/specs/score/b.feature\|B=0.9600; | empty entry |
      | missing equals | givn/specs/score/a.feature\|A::givn/specs/score/b.feature\|B | expected <endpoint>::<endpoint>=<score> |
      | missing pair separator | givn/specs/score/a.feature\|A=0.9600 | expected two endpoint identities |
      | equal endpoints | givn/specs/score/a.feature\|A::givn/specs/score/a.feature\|A=0.9600 | two distinct endpoints |
      | duplicate pair | givn/specs/score/a.feature\|A::givn/specs/score/b.feature\|B=0.9600;givn/specs/score/b.feature\|B::givn/specs/score/a.feature\|A=0.9500 | pair is repeated |
      | malformed endpoint | givn/specs/score/a.feature\|A::broken=0.9600 | expected <file>\|<title> |
      | empty title | givn/specs/score/a.feature\|::givn/specs/score/b.feature\|B=0.9600 | invalid duplicate endpoint identity |
      | invalid file | project/spec.feature\|A::givn/specs/score/b.feature\|B=0.9600 | invalid duplicate endpoint identity |
      | invalid score shape | givn/specs/score/a.feature\|A::givn/specs/score/b.feature\|B=0.960 | expected four decimal places |
      | out-of-range score | givn/specs/score/a.feature\|A::givn/specs/score/b.feature\|B=1.0001 | outside the cosine range |
    When the user validates each duplicate score override
    Then every invalid duplicate score override fails with its expected diagnostic

  @givn.added
  Scenario: Reversed score override endpoint pairs normalize canonically
    Given two permanent semantic scenarios have a reversed score override pair
    When the user runs `givn spec duplicates --format json`
    Then the reversed score override reports one semantic candidate at `0.9600`
    And the reversed score candidate uses canonical endpoint order
