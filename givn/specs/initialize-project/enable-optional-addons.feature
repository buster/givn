Feature: enable-optional-addons

  @enable-optional-addons
  Scenario: givn init --no-addons produces no skill or command addon files
    Given  a project root without a givn/ directory
    When  the user runs `givn init --no-addons --agents opencode`
    Then  all optional addons including communication_policy and challenge are explicitly disabled
    And  no addon skills or generated commands exist
    And  the mandatory dialog-policy startup obligation remains installed
    And  the file "docs/glossary.md" does not exist
    And  the file "docs/domain-model.md" does not exist

  Scenario: givn addons exposes architecture principles as opt-in
    Given a project root with givn/ initialised without addons
    When the user runs `givn addons list`
    Then stdout contains "[disabled]  architecture_principles"
    When the user runs `givn addons enable architecture_principles`
    Then the command exits 0
    And givn addon "architecture_principles" is "true"

  @enable-optional-addons
  Scenario: givn addons enable generates skill and command files
    Given  an initialized addon-free project selecting opencode
    When  the user enables domain_modeling and arc42
    Then  the selected target receives the domain-modeling skill
    And  no generated command files are written
    And  unselected targets receive no skills

  Scenario: givn addons disable removes generated skill files
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "true"
    And the skill "givn-domain-modeling" exists in all targets
    When the user runs `givn addons disable domain_modeling`
    Then the command exits 0
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".claude/skills/givn-domain-modeling" does not exist
    And the directory ".opencode/skills/givn-domain-modeling" does not exist

  @enable-optional-addons
  Scenario: givn addons disable removes generated command files
    Given  an initialized project with challenge enabled
    And  recognizable generated grill-me and grill-with-docs commands beside custom commands
    When  the user runs `givn addons disable challenge`
    Then  the generated givn-challenge skill and recognizable generated legacy commands are removed
    And  custom commands remain byte-for-byte unchanged

  Scenario: givn addons disable when already disabled is idempotent
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "false"
    When the user runs `givn addons disable domain_modeling`
    Then the command exits 0

  @enable-optional-addons
  Scenario: givn skills sync includes skill and command addons when enabled
    Given  a project with domain_modeling, arc42, dev_principles, and challenge enabled
    When  the user runs `givn skills sync`
    Then  the report names the domain-modeling, dev-principles, and givn-challenge skills
    And  the report names no generated commands
    And  no separate grill-me or grill-with-docs guidance is generated

  @enable-optional-addons
  Scenario: givn skills sync excludes addon skills and commands when disabled
    Given  a project with domain_modeling and challenge disabled
    When  the user runs `givn skills sync`
    Then  no generated domain-modeling or givn-challenge skill remains
    And  no addon command is generated or reported as synchronized

  Scenario: givn addons enable errors when givn/ is not initialised
    Given a project root without a givn/ directory
    When the user runs `givn addons enable domain_modeling`
    Then the command exits non-zero
    And stderr contains "givn/config.yaml not found"

  Scenario: givn addons disable errors when givn/ is not initialised
    Given a project root without a givn/ directory
    When the user runs `givn addons disable domain_modeling`
    Then the command exits non-zero
    And stderr contains "givn/config.yaml not found"

  Scenario: addon skills are written to the default agent target after sync
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "true"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".claude/skills/givn-domain-modeling" does not exist

  Scenario: addon skills are written to every configured agent target after sync
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude"
    And givn addon "domain_modeling" is "true"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".claude/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists

  @e2e @enable-optional-addons
  Scenario: Consumer enables the outline addon and it is listed as enabled
    Given  an initialized addon-free project with outline disabled
    When  the consumer runs `givn addons enable outline`
    Then  the command exits 0 and reports that outline is enabled
    And  `givn addons list` reports the effective outline state as enabled
    And  unrelated addon opt-outs remain unchanged
    And  the terminal transcript is captured for this scenario

  @enable-optional-addons
  Scenario: givn init enables every addon by default
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  every current addon including communication_policy and challenge is effectively enabled
    And  the selected target receives addon skills without generated commands
    And  no grill_me, grill_with_docs, or absolute_mode setting is generated
    And  the file "docs/glossary.md" exists
    And  the file "docs/domain-model.md" exists

  @enable-optional-addons
  Scenario: givn addons list reports every addon enabled by default
    Given  an initialized project without an addons block
    When  the user runs `givn addons list`
    Then  every current addon including communication_policy and challenge is reported enabled
    And  no retired addon name is listed

  @enable-optional-addons
  Scenario: givn init --no-addons writes an explicit opt-out for every addon
    Given  a project root without a givn/ directory
    When  the user runs `givn init --no-addons --agents opencode`
    Then  every current addon has an explicit false setting
    And  `givn addons list` reports every addon disabled

  @enable-optional-addons
  Scenario: a partial addons block keeps unnamed addons enabled
    Given  an initialized project whose only explicit addon setting disables outline
    When  the user synchronizes skills and lists addons
    Then  outline is disabled in listing, guidance, artifacts, and addon execution
    And  omitted addons including challenge and communication_policy use the same enabled defaults everywhere

  @enable-optional-addons
  Scenario: Communication policy replaces absolute mode without a compatibility alias
    Given  an initialized addon-free project
    When  the user runs `givn addons enable communication_policy`
    Then  the managed communication block contains concrete presentation rules without cognitive-rebuilding rhetoric
    And  the mandatory dialog policy remains independent of the communication addon
    And  `givn addons enable absolute_mode` is refused as an unknown addon with manual rename guidance

  @enable-optional-addons
  Scenario Outline: Retired grill settings require manual transition
    Given  an initialized project without legacy addon settings
    When  the user runs `givn addons enable <retired>`
    Then  the command refuses the unknown addon before writing
    And  the diagnostic names challenge as the manual replacement without enabling it automatically
    Examples:
      | retired |
      | grill_me |
      | grill_with_docs |

  @enable-optional-addons
  Scenario: Challenge is optional and introduces no lifecycle gate
    Given  a project with challenge explicitly disabled
    When  the user requests the next lifecycle instruction and synchronizes skills
    Then  the normal workflow remains available without a challenge verdict
    And  no givn-challenge skill is installed

  @enable-optional-addons
  Scenario Outline: Challenge guidance researches the subject before asking unresolved decisions
    Given  a project with challenge enabled
    When  the author requests challenge guidance for "<subject>"
    Then  givn-challenge directs the author to the canonical challenge instruction
    And  that instruction requires research and evidence before questions
    And  discoverable defects are reported as bounded actionable findings and hypotheses are labeled
    And  questions are reserved for unresolved human decisions
    And  evidence includes counterexamples, discriminating assertions, root causes, compatibility and failure recovery where relevant
    And  performance claims require measurements
    And  the challenge instruction names the canonical engineering principles
    And  the challenge instruction includes the condensed engineering principles
    And  the challenge instruction requires the quality lenses
    And  accepted modularity and fix-forward choices are not overridden by blanket compatibility or rollback rules
    Examples:
      | subject |
      | proposal |
      | tests |
      | specs |
      | design |
      | code |
      | whole change |

  @enable-optional-addons
  Scenario: Challenge guidance omits disabled principle sections
    Given  a project with challenge enabled and dev_principles and architecture_principles disabled
    When  the author requests challenge guidance for "code"
    Then  the challenge instruction omits the disabled engineering principles

  @enable-optional-addons
  Scenario Outline: Addon edits preserve unrelated configuration or refuse unsupported YAML
    Given  a project using "<presentation>" YAML with unrelated custom settings
    When  the user runs `givn addons disable challenge`
    Then  only challenge is disabled or unsupported YAML is refused before any write
    And  no duplicate addons section or unrelated edit occurs
    And  on refusal all configuration and generated files remain unchanged
    Examples:
      | presentation |
      | block mapping with comments |
      | flow mapping |
      | anchors and aliases |

  @enable-optional-addons
  Scenario Outline: Configuration fields are enforced in their owning file
    Given  an initialized project with "<field>" misplaced in "<file>"
    When  the user requests effective addon state or executes the configured workflow
    Then  the request fails clearly naming the field and its owning file "<owner>"
    And  the misplaced setting is not silently ignored or applied
    Examples:
      | field | file | owner |
      | addons | givn/commands.yaml | givn/config.yaml |
      | workflow | givn/commands.yaml | givn/config.yaml |
      | verify.command | givn/config.yaml | givn/commands.yaml |
      | run.command | givn/config.yaml | givn/commands.yaml |

  @enable-optional-addons
  Scenario: The scaffolded glossary is the standalone vocabulary authority
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  the file "docs/glossary.md" contains "| Term | Definition |"
    And  the file "docs/glossary.md" contains "sole vocabulary authority"

  @enable-optional-addons
  Scenario: The scaffolded domain model carries the notation legend
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  the file "docs/domain-model.md" contains "Collapsed consistency boundary"
    And  the file "docs/domain-model.md" contains "External system"
    And  the file "docs/domain-model.md" contains "layout: elk"
    And  the file "docs/domain-model.md" contains "This is notation only, not part of the domain model"

  @enable-optional-addons
  Scenario: Existing domain documents survive initialization
    Given  a project root with givn/ initialised
    And  a project file "docs/glossary.md" with content "# Project glossary"
    And  a project file "docs/domain-model.md" with content "# Project model"
    When  the user runs `givn init --agents opencode`
    Then  the file "docs/glossary.md" contains "# Project glossary"
    And  the file "docs/domain-model.md" contains "# Project model"

  @enable-optional-addons
  Scenario: Enabling arc42 scaffolds only the standalone glossary
    Given  an initialized addon-free project selecting opencode
    When  the user runs `givn addons enable arc42`
    Then  the file "docs/glossary.md" exists
    And  the file "docs/domain-model.md" does not exist
    And  the file "docs/arc42/12-glossary.md" contains "../glossary.md"
    And  the file "docs/arc42/12-glossary.md" does not contain "| Term | Definition |"

  @enable-optional-addons
  Scenario: Enabling domain modeling scaffolds both durable documents
    Given  an initialized addon-free project selecting opencode
    When  the user runs `givn addons enable domain_modeling`
    Then  the file "docs/glossary.md" exists
    And  the file "docs/domain-model.md" exists

  @enable-optional-addons
  Scenario: Enabling an addon preserves an existing glossary
    Given  an initialized addon-free project selecting opencode
    And  a project file "docs/glossary.md" with content "# Project glossary"
    When  the user runs `givn addons enable domain_modeling`
    Then  the file "docs/glossary.md" contains "# Project glossary"

  @enable-optional-addons
  Scenario: Disabling domain modeling leaves the durable documents in place
    Given  an initialized addon-free project selecting opencode
    When  the user runs `givn addons enable domain_modeling`
    And  the user runs `givn addons disable domain_modeling`
    Then  the file "docs/glossary.md" exists
    And  the file "docs/domain-model.md" exists

  @enable-optional-addons
  Scenario: A new change records the operator add-on configuration
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    Then  the file "givn/changes/addon-lock/config-lock.yaml" exists
    And  the file "givn/changes/addon-lock/config-lock.yaml" contains "coverage: true"

  @enable-optional-addons
  Scenario: An active change refuses add-on mutation
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the user runs `givn addons disable coverage`
    Then  the command exits non-zero
    And  stderr contains "operator-owned"
    And  stderr contains "coverage"
    And  stderr contains "environment"
    And  the givn config keeps addon "coverage" enabled

  @enable-optional-addons
  Scenario: A non-interactive operator override is refused
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the user runs `givn addons disable coverage --operator --reason "missing dependency"`
    Then  the command exits non-zero
    And  stderr contains "interactive terminal"
    And  the givn config keeps addon "coverage" enabled

  @enable-optional-addons
  Scenario: A forced init cannot change add-ons during a locked change
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the user runs `givn init --force --no-addons --agents opencode`
    Then  the command exits non-zero
    And  stderr contains "add-on configuration is locked"
    And  stderr contains "coverage"
    And  the givn config keeps addon "coverage" enabled

  @enable-optional-addons
  Scenario: A forced init that preserves add-ons is allowed during a locked change
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the user runs `givn init --force --agents opencode`
    Then  the command exits 0
    And  the givn config keeps addon "coverage" enabled

  @enable-optional-addons
  Scenario: An operator override on a real terminal is recorded
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the operator disables coverage with an override in the real terminal
    Then  the command exits 0
    And  the givn config keeps addon "coverage" disabled
    And  the change lock records the operator override reason

  @enable-optional-addons
  Scenario: Add-ons remain changeable without an active change
    Given  a project root with givn/ initialised
    When  the user runs `givn addons disable coverage`
    Then  the command exits 0
    And  the givn config keeps addon "coverage" disabled

  @enable-optional-addons
  Scenario: The communication block binds documentation to named artifacts
    Given  an initialized addon-free project
    When  the user runs `givn addons enable communication_policy`
    Then  the managed communication block requires tables for command, flag, and key mappings
    And  the managed communication block sends fallback and error-string rules to specs and help
    And  the managed communication block keeps quick start free of validation rules and exit codes
    And  the managed communication block names no documentation file or README line budget
    And  the managed communication block is at most 300 words

  @e2e @enable-optional-addons
  Scenario: Consumer lists addons and sees each addon's materialized files reported
    Given  an initialized project with scaffolding materialized
    When  the consumer lists the addons
    Then  the command exits 0
    And  the listing reports arc42 as enabled with artifacts present

  @enable-optional-addons
  Scenario: An enabled addon with missing files is reported and the exit status is non-zero
    Given  an initialized project with scaffolding materialized
    And  the directory "docs/arc42" no longer exists
    When  the user lists the addons
    Then  the command exits 1
    And  the listing reports arc42 as enabled with artifacts missing docs/arc42
    And  the listing suggests for arc42 `givn addons enable arc42`

  @enable-optional-addons
  Scenario: Files present for a disabled addon are reported without failing the command
    Given  an initialized project with scaffolding materialized
    And  the readme addon is disabled
    When  the user runs `givn addons list`
    Then  the command exits 0
    And  the listing reports readme as disabled with artifacts present
    And  the listing suggests `givn addons disable readme`

  @enable-optional-addons
  Scenario: An enabled skill addon missing its skill from a selected target is reported as partial
    Given  an initialized project with scaffolding materialized
    And  givn skills targets in givn/config.yaml equal "opencode, agents"
    And  the user runs skills sync
    And  the skill "givn-domain-modeling" is removed from the "agents" target only
    When  the user runs `givn addons list`
    Then  the command exits 1
    And  the listing reports domain_modeling as enabled with artifacts partial (.agents/skills/givn-domain-modeling missing)
    And  the listing suggests `givn skills sync`

  @enable-optional-addons
  Scenario: A retired addon key in the configuration is refused with its replacement
    Given  an initialized project with scaffolding materialized
    And  the givn config contains the retired key "grill_me"
    When  the user runs `givn addons list`
    Then  the command exits non-zero
    And  stderr names "grill_me" as retired
    And  stderr names "challenge" as the replacement

  @enable-optional-addons
  Scenario: An enabled addon whose effect is a managed block reports the block state
    Given  an initialized project with scaffolding materialized
    When  the user lists the addons
    Then  the command exits 0
    And  the listing reports communication_policy as enabled with artifacts present

  @enable-optional-addons
  Scenario: A damaged managed block for an enabled addon fails the command
    Given  an initialized project with scaffolding materialized
    And  the managed communication block in "AGENTS.md" is damaged
    When  the user lists the addons
    Then  the command exits 1
    And  the listing reports communication_policy as enabled with artifacts missing AGENTS.md
    And  the listing suggests for communication_policy `givn addons enable communication_policy`

  @enable-optional-addons
  Scenario: The listing writes nothing to the project
    Given  an initialized project with scaffolding materialized
    And  the directory "docs/arc42" no longer exists
    When  the user lists the addons
    Then  the project files are unchanged
    And  the givn config keeps addon "arc42" enabled

  @enable-optional-addons
  Scenario: Disabling domain_modeling keeps the shared glossary from a partial report
    Given  an initialized project with scaffolding materialized
    And  givn addon "domain_modeling" is "false"
    When  the user runs `givn addons list`
    Then  the command exits 0
    And  the listing reports domain_modeling as disabled with artifacts present
    And  the listing suggests for domain_modeling `givn addons disable domain_modeling`
    And  the listing reports arc42 as enabled with artifacts present

  @enable-optional-addons
  Scenario: A locked change adds the lock to each suggestion
    Given  an initialized project with scaffolding materialized
    And  the readme addon is disabled
    When  the user runs `givn new addon-lock`
    And  the user runs `givn addons list`
    Then  the command exits 0
    And  the listing suggests `givn addons disable readme`
    And  the suggestion names "addon-lock" as the change holding the add-on lock
    And  the givn config keeps addon "readme" disabled

  @enable-optional-addons
  Scenario: An addon-free project listing reports every addon explicitly disabled
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn addons list`
    Then  the command exits 0
    And  `givn addons list` reports every addon disabled
