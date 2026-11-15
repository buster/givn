@givn.delta @enable-optional-addons
Feature: enable-optional-addons

  @givn.added @e2e @wip
  Scenario: Consumer lists addons and sees each addon's materialized files reported
    Given an initialized project with scaffolding materialized
    When the consumer lists the addons
    Then the command exits 0
    And the listing reports arc42 as enabled with artifacts present

  @givn.added @wip
  Scenario: An enabled addon with missing files is reported and the exit status is non-zero
    Given an initialized project with scaffolding materialized
    And the directory "docs/arc42" no longer exists
    When the user lists the addons
    Then the command exits 1
    And the listing reports arc42 as enabled with artifacts missing docs/arc42
    And the listing suggests for arc42 `givn addons enable arc42`

  @givn.added @wip
  Scenario: Files present for a disabled addon are reported without failing the command
    Given an initialized project with scaffolding materialized
    And the readme addon is disabled
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports readme as disabled with artifacts present
    And the listing suggests `givn addons disable readme`

  @givn.added @wip
  Scenario: An enabled skill addon missing its skill from a selected target is reported as partial
    Given an initialized project with scaffolding materialized
    And givn skills targets in givn/config.yaml equal "opencode, agents"
    And the user runs skills sync
    And the skill "givn-domain-modeling" is removed from the "agents" target only
    When the user runs `givn addons list`
    Then the command exits 1
    And the listing reports domain_modeling as enabled with artifacts partial (.agents/skills/givn-domain-modeling missing)
    And the listing suggests `givn skills sync`

  @givn.added @wip
  Scenario: A retired addon key in the configuration is refused with its replacement
    Given an initialized project with scaffolding materialized
    And the givn config contains the retired key "grill_me"
    When the user runs `givn addons list`
    Then the command exits non-zero
    And stderr names "grill_me" as retired
    And stderr names "challenge" as the replacement

  @givn.added @wip
  Scenario: An enabled addon whose effect is a managed block reports the block state
    Given an initialized project with scaffolding materialized
    When the user lists the addons
    Then the command exits 0
    And the listing reports communication_policy as enabled with artifacts present

  @givn.added @wip
  Scenario: A damaged managed block for an enabled addon fails the command
    Given an initialized project with scaffolding materialized
    And the managed communication block in "AGENTS.md" is damaged
    When the user lists the addons
    Then the command exits 1
    And the listing reports communication_policy as enabled with artifacts missing AGENTS.md
    And the listing suggests for communication_policy `givn addons enable communication_policy`

  @givn.added @wip
  Scenario: The listing writes nothing to the project
    Given an initialized project with scaffolding materialized
    And the directory "docs/arc42" no longer exists
    When the user lists the addons
    Then the project files are unchanged
    And the givn config keeps addon "arc42" enabled

  @givn.added @wip
  Scenario: Disabling domain_modeling keeps the shared glossary from a partial report
    Given an initialized project with scaffolding materialized
    And givn addon "domain_modeling" is "false"
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports domain_modeling as disabled with artifacts present
    And the listing suggests for domain_modeling `givn addons disable domain_modeling`
    And the listing reports arc42 as enabled with artifacts present

  @givn.added @wip
  Scenario: A locked change adds the lock to each suggestion
    Given an initialized project with scaffolding materialized
    And the readme addon is disabled
    When the user runs `givn new addon-lock`
    And the user runs `givn addons list`
    Then the command exits 0
    And the listing suggests `givn addons disable readme`
    And the suggestion names "addon-lock" as the change holding the add-on lock
    And the givn config keeps addon "readme" disabled

  @givn.added @wip
  Scenario: An addon-free project listing reports every addon explicitly disabled
    Given a project root with givn/ initialised without addons
    When the user runs `givn addons list`
    Then the command exits 0
    And `givn addons list` reports every addon disabled
