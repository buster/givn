@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.removed
  Scenario: givn init enables every addon by default
    Given a placeholder step

  @givn.added
  Scenario: givn init writes the nine default-on addons and omits ise
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the command exits 0
    And givn/config.yaml contains the line "  arc42: true"
    And givn/config.yaml contains the line "  outline: true"
    And givn/config.yaml does not contain the line "  ise:"
    And the file "givn/specification.yaml" does not exist

  @givn.added @wip
  Scenario: givn init --no-addons writes an explicit false for every addon including ise
    Given a project root without a givn/ directory
    When the user runs `givn init --no-addons --agents opencode`
    Then the command exits 0
    And the active addon settings disable exactly "arc42, communication_policy, readme, coverage, domain_modeling, dev_principles, architecture_principles, challenge, outline, ise"
    And the file "givn/specification.yaml" does not exist

  @givn.removed
  Scenario: givn addons list reports every addon enabled by default
    Given a placeholder step

  @givn.added @e2e @wip
  Scenario: A consumer lists addons and each declared default is reported
    Given a project root without a givn/ directory
    When the consumer runs `givn init --agents opencode`
    And the consumer runs `givn addons list`
    Then the command exits 0
    And stdout contains "[disabled]  ise"
    And stdout contains "[enabled ]  outline"
    And stdout contains "each omitted key resolves that addon's declared default"
    And the terminal transcript is captured for this scenario

  @givn.removed
  Scenario: a partial addons block keeps unnamed addons enabled
    Given a placeholder step

  @givn.added
  Scenario: a partial addons block resolves each omitted addon to its declared default
    Given an initialized project whose only explicit addon setting disables outline
    When the user synchronizes skills and lists addons
    Then outline is disabled in listing, guidance, artifacts, and addon execution
    And the listing reports ise as disabled
    And omitted default-on addons including challenge and communication_policy use the same enabled defaults everywhere

  @givn.added @wip
  Scenario: Enabling ise writes the setting and scaffolds the declaration
    Given an initialized project selecting opencode
    When the user runs `givn addons enable ise`
    Then the command exits 0
    And stdout names "givn/specification.yaml"
    And the active addon settings keep "ise" enabled
    And the file "givn/specification.yaml" exists
    And the declaration names exactly the five roles and one source list

  @givn.added
  Scenario: Enabling ise without its prerequisite is refused by name
    Given an initialized project selecting opencode with architecture_principles disabled
    When the user runs `givn addons enable ise`
    Then the command exits non-zero
    And stderr contains "addon 'ise' requires 'architecture_principles'; enable it first"
    And the active addon settings keep "ise" disabled
    And the active addon settings keep "architecture_principles" disabled
    And the file "givn/specification.yaml" does not exist

  @givn.added @e2e @wip
  Scenario: A consumer disables a required addon and the refusal names ise
    Given an ise-enabled project selecting opencode
    When the consumer runs `givn addons disable architecture_principles`
    Then the command exits non-zero
    And stderr contains "addon 'architecture_principles' is required by 'ise'; disable 'ise' first"
    And the active addon settings keep "architecture_principles" enabled
    And the active addon settings keep "ise" enabled
    And the terminal transcript is captured for this scenario

  @givn.added @wip
  Scenario: Enabling ise preserves an edited declaration
    Given an initialized project selecting opencode
    And a declaration at "givn/specification.yaml" whose bytes are recorded
    When the user runs `givn addons enable ise`
    Then the command exits 0
    And stdout contains "content preserved"
    And the declaration bytes equal the recorded bytes

  @givn.added @wip
  Scenario: Repeating the enable changes nothing
    Given an ise-enabled project selecting opencode whose declaration bytes are recorded
    When the user runs `givn addons enable ise`
    Then the command exits 0
    And the declaration bytes equal the recorded bytes

  @givn.added @wip
  Scenario: A forced re-initialization preserves an effective ise and its declaration
    Given an ise-enabled project selecting opencode whose declaration bytes are recorded
    When the user runs `givn init --force --agents opencode`
    Then the command exits 0
    And the active addon settings keep "ise" enabled
    And the declaration bytes equal the recorded bytes

  @givn.added @wip
  Scenario: Disabling ise preserves the declaration and reports it project-owned
    Given an ise-enabled project selecting opencode whose declaration bytes are recorded
    When the user runs `givn addons disable ise`
    Then the command exits 0
    And stdout contains "project-owned"
    And the active addon settings keep "ise" disabled
    And the declaration bytes equal the recorded bytes

  @givn.added @wip
  Scenario: A change locked before ise existed accepts the enabled addon
    Given an ise-enabled project selecting opencode
    And an active change whose addon lock lists the nine default-on addons and omits "ise"
    When the user runs `givn check review --change add-declaration-check`
    Then stderr does not contain "add-on configuration changed during change"

  @givn.added @wip
  Scenario: A declaration scaffold failure names the path and the applied setting
    Given an initialized project selecting opencode
    And the path "givn/specification.yaml" is a directory
    When the user runs `givn addons enable ise`
    Then the command exits non-zero
    And stderr names "givn/specification.yaml"
    And the active addon settings keep "ise" enabled
