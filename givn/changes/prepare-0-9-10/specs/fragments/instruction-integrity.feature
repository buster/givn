@givn.delta @instruction-integrity
Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.modified
  Scenario: Design guidance requires the complete E2E setup record
    Given a Givn project with an active change
    When the author requests the design instructions through the Givn CLI
    Then the guidance requires explicit regular and E2E scope selection and the appropriate real interface driver
    And the setup example executes the selected Gherkin features supplied through GIVN_FEATURES rather than a parallel hand-maintained test suite
    And a shared runner selects regular or E2E scenarios explicitly through GIVN_RESULT_SCOPE
    And every invocation writes fresh results to GIVN_RESULT_FILE with total passed failed and skipped counts
    And total equals passed plus failed plus skipped and successful verification requires a nonzero total with zero failures and skips
    And undefined pending ambiguous or failed steps and skipped scenarios cannot produce passing verification
    And runner failures exit nonzero and missing or malformed results cannot imply success
    And the guidance supplies executable runner and matching verification configuration examples
    And runner strictness is proved at initial setup and after relevant runner version invocation or failure-configuration changes
    And local infrastructure is required only for dependencies the product actually has
    And the guidance retains framework justification interaction coverage and applicable visual evidence

  @givn.modified
  Scenario: Review guidance names the deterministic disposition gates
    Given a Givn project with an active change
    When the reviewer requests the review instructions through the Givn CLI
    Then the guidance requires split-or-keep for each reported long scenario with a brief keep reason
    And a chosen split must be completed before sign-off
    And shape-match pairs are advisory discovery rather than mandatory dispositions
    And advisory findings are not completed verification evidence

  @givn.modified
  Scenario: The canonical dialog policy demonstrates the full question record
    Given an initialized Givn project
    When the author requests the dialog policy through the Givn CLI
    Then the concise policy requires decision-relevant context alternatives consequences and resolution
    And chosen and rejected alternatives retain their reasons
    And routine clarification does not require invented alternatives
    And confirmations explain acceptance and revision consequences without making consent a rule waiver
    And the policy indexes the companion regression examples by question type

  @givn.modified
  Scenario: Dialog teaching contrasts show complete decisions rather than bare labels
    Given an initialized Givn project
    When the author follows the dialog policy index to its companion regression examples
    Then the companion retains the scope-approval target-selection behavior-choice persistent-format-commitment and discoverable-diagnosis examples
    And the examples distinguish hypothetical teaching records from current product behavior
    And each good example supplies its decision record and concrete answer consequences
    And the diagnostic example reports the researched assertion defect without asking the person to inspect unseen scenarios
    And no answer violates a confirmed one-answer constraint

  @givn.modified
  Scenario: Migration consent does not imply an unverified recovery guarantee
    Given an initialized Givn project
    When the author reads the persistent-format-commitment example linked from the dialog policy
    Then the example distinguishes recovery requiring verification from recovery already supported
    And the less-committing answer leaves saved projects unchanged without promising a reversible upgrade
    And unknown compatibility facts must be established or consent narrowed before rollout approval

  @givn.added
  Scenario: Relevant dialog examples are required before the first question of their type
    Given the effective dialog policy has been loaded in the current context
    When the author prepares the first question of a type indexed by the policy
    Then the policy requires reading that type's companion examples before presenting the question
    And repeated questions of that type do not require rereading examples retained in context
    And questions remain one at a time

  @givn.modified @e2e
  Scenario: Author starts a session with the universal dialog obligation
    Given an initialized Givn project without an active change
    When the author requests the normal Givn startup instructions through the Givn CLI
    Then the startup guidance requires loading the effective dialog policy before the context's first user-facing question
    And the bootstrap owns that obligation independently of optional addons and named skills
    And the policy is loaded once per context and reloaded if lost
    And the startup guidance still explains how to begin a change
    And the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: The characterization guidance requests the reverse review from Givn
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then every selected target receives a givn-characterize skill directing the author to `givn instructions review-reverse`
    And no separately generated characterization command is installed

  @givn.modified
  Scenario: Every instruction id named by generated guidance resolves through the Givn CLI
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then every generated skill names a served instruction
    And every named instruction id resolves through the Givn CLI

  @givn.modified
  Scenario: Every generated skill and command directs to exactly one served instruction id
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then the command succeeds
    And every generated skill directs to exactly one served instruction id
    And no separately authored agent command is generated

  @givn.added
  Scenario Outline: Givn shipped templates render parser-compatible documents and examples
    Given Givn's shipped "<asset>" template with its documented example values
    When a Givn maintainer renders the template for its documented use
    Then frontmatter is valid wherever the document requires it
    And Markdown fences are balanced and preserve the intended example boundaries
    And ordinary parsers for the documented formats accept the step tables and configuration samples without Givn-specific repairs

    Examples:
      | asset                       |
      | artifact documents          |
      | agent skills                |
      | step tables                 |
      | verification configuration  |

  @givn.added
  Scenario Outline: Current shipped guidance and help exclude harness-project leakage without rewriting history
    Given Givn's current "<surface>" and preserved historical evidence
    When a Givn maintainer inspects the release's consumer-facing content
    Then harness-project paths fixture names and recording content are absent from current consumer guidance
    And legitimate generic examples remain usable outside Givn's own repository
    And historical decisions and evidence remain unchanged and are clearly distinguished from current guidance

    Examples:
      | surface                 |
      | shipped guidance assets |
      | command help            |

  @givn.added
  Scenario: Givn release distribution excludes local recordings and harness fixtures
    Given a Givn release checkout containing local runtime recordings and harness fixtures
    When a Givn maintainer prepares the release distribution
    Then the distributed package contains neither runtime recordings nor harness fixtures
    And the checkout's historical evidence remains unchanged
    And this Givn distribution requirement installs no release gate in consumer projects
