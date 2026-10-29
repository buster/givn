@givn.delta @enable-optional-addons
Feature: Effective optional addon policy

  @givn.modified
  Scenario: givn init --no-addons produces no skill or command addon files
    Given a project root without a givn/ directory
    When the user runs `givn init --no-addons --agents opencode`
    Then all optional addons including communication_policy and challenge are explicitly disabled
    And no addon skills or generated commands exist
    And the mandatory dialog-policy startup obligation remains installed

  @givn.modified
  Scenario: givn addons enable generates skill and command files
    Given an initialized addon-free project selecting opencode
    When the user enables domain_modeling and arc42
    Then the selected target receives the domain-modeling skill
    And no generated command files are written
    And unselected targets receive no skills

  @givn.modified
  Scenario: givn addons disable removes generated command files
    Given an initialized project with challenge enabled
    And recognizable generated grill-me and grill-with-docs commands beside custom commands
    When the user runs `givn addons disable challenge`
    Then the generated givn-challenge skill and recognizable generated legacy commands are removed
    And custom commands remain byte-for-byte unchanged

  @givn.modified
  Scenario: givn skills sync includes skill and command addons when enabled
    Given a project with domain_modeling, arc42, dev_principles, and challenge enabled
    When the user runs `givn skills sync`
    Then the report names the domain-modeling, dev-principles, and givn-challenge skills
    And the report names no generated commands
    And no separate grill-me or grill-with-docs guidance is generated

  @givn.modified
  Scenario: givn skills sync excludes addon skills and commands when disabled
    Given a project with domain_modeling and challenge disabled
    When the user runs `givn skills sync`
    Then no generated domain-modeling or givn-challenge skill remains
    And no addon command is generated or reported as synchronized

  @givn.removed
  Scenario: The absolute-mode policy block carries the dialog context rule
    Given this scenario is removed

  @givn.modified
  Scenario: givn init enables every addon by default
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then every current addon including communication_policy and challenge is effectively enabled
    And the selected target receives addon skills without generated commands
    And no grill_me, grill_with_docs, or absolute_mode setting is generated

  @givn.modified
  Scenario: givn addons list reports every addon enabled by default
    Given an initialized project without an addons block
    When the user runs `givn addons list`
    Then every current addon including communication_policy and challenge is reported enabled
    And no retired addon name is listed

  @givn.modified
  Scenario: givn init --no-addons writes an explicit opt-out for every addon
    Given a project root without a givn/ directory
    When the user runs `givn init --no-addons --agents opencode`
    Then every current addon has an explicit false setting
    And `givn addons list` reports every addon disabled

  @givn.modified
  Scenario: a partial addons block keeps unnamed addons enabled
    Given an initialized project whose only explicit addon setting disables outline
    When the user synchronizes skills and lists addons
    Then outline is disabled in listing, guidance, artifacts, and addon execution
    And omitted addons including challenge and communication_policy use the same enabled defaults everywhere

  @givn.modified @e2e
  Scenario: Consumer enables the outline addon and it is listed as enabled
    Given an initialized addon-free project with outline disabled
    When the consumer runs `givn addons enable outline`
    Then the command exits 0 and reports that outline is enabled
    And `givn addons list` reports the effective outline state as enabled
    And unrelated addon opt-outs remain unchanged

  @givn.added
  Scenario: Communication policy replaces absolute mode without a compatibility alias
    Given an initialized addon-free project
    When the user runs `givn addons enable communication_policy`
    Then the managed communication block contains concrete presentation rules without cognitive-rebuilding rhetoric
    And the mandatory dialog policy remains independent of the communication addon
    And `givn addons enable absolute_mode` is refused as an unknown addon with manual rename guidance

  @givn.added
  Scenario Outline: Retired grill settings require manual transition
    Given an initialized project without legacy addon settings
    When the user runs `givn addons enable <retired>`
    Then the command refuses the unknown addon before writing
    And the diagnostic names challenge as the manual replacement without enabling it automatically

    Examples:
      | retired         |
      | grill_me        |
      | grill_with_docs |

  @givn.added
  Scenario: Challenge is optional and introduces no lifecycle gate
    Given a project with challenge explicitly disabled
    When the user requests the next lifecycle instruction and synchronizes skills
    Then the normal workflow remains available without a challenge verdict
    And no givn-challenge skill is installed

  @givn.added
  Scenario Outline: Challenge guidance researches the subject before asking unresolved decisions
    Given a project with challenge enabled
    When the author requests challenge guidance for "<subject>"
    Then givn-challenge directs the author to the canonical challenge instruction
    And that instruction requires research and evidence before questions
    And discoverable defects are reported as bounded actionable findings and hypotheses are labeled
    And questions are reserved for unresolved human decisions
    And evidence includes counterexamples, discriminating assertions, root causes, compatibility and failure recovery where relevant
    And performance claims require measurements
    And general engineering principles are referenced canonically rather than copied into a checklist
    And accepted modularity and fix-forward choices are not overridden by blanket compatibility or rollback rules

    Examples:
      | subject      |
      | proposal     |
      | tests        |
      | specs        |
      | design       |
      | code         |
      | whole change |

  @givn.added
  Scenario Outline: Addon edits preserve unrelated configuration or refuse unsupported YAML
    Given a project using "<presentation>" YAML with unrelated custom settings
    When the user runs `givn addons disable challenge`
    Then only challenge is disabled or unsupported YAML is refused before any write
    And no duplicate addons section or unrelated edit occurs
    And on refusal all configuration and generated files remain unchanged

    Examples:
      | presentation                |
      | block mapping with comments |
      | flow mapping                |
      | anchors and aliases         |

  @givn.added
  Scenario Outline: Configuration fields are enforced in their owning file
    Given an initialized project with "<field>" misplaced in "<file>"
    When the user requests effective addon state or executes the configured workflow
    Then the request fails clearly naming the field and its owning file "<owner>"
    And the misplaced setting is not silently ignored or applied

    Examples:
      | field          | file               | owner              |
      | addons         | givn/commands.yaml | givn/config.yaml   |
      | workflow       | givn/commands.yaml | givn/config.yaml   |
      | verify.command | givn/config.yaml   | givn/commands.yaml |
      | run.command    | givn/config.yaml   | givn/commands.yaml |
