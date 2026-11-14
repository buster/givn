@givn.delta @enable-optional-addons
Feature: enable-optional-addons

  @givn.added @e2e @wip
  Scenario: Consumer lists addons and sees each addon's materialized files reported
    Given an initialized project with arc42 enabled and its chapters present
    When the consumer runs `givn addons list`
    Then the command exits 0
    And the listing reports arc42 as enabled with artifacts present

  @givn.added @wip
  Scenario: An enabled addon with missing files is reported and the exit status is non-zero
    Given an initialized project with arc42 enabled
    And the directory "docs/arc42" does not exist
    When the user runs `givn addons list`
    Then the command exits 1
    And the listing reports arc42 as enabled with artifacts missing
    And the listing suggests `givn addons enable arc42`

  @givn.added @wip
  Scenario: Files present for a disabled addon are reported without failing the command
    Given an initialized project with the readme addon disabled
    And the file "README.md" exists
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports readme as disabled with artifacts present
    And the listing suggests `givn addons disable readme`

  @givn.added @wip
  Scenario: An enabled skill addon missing its skill from a selected target is reported as partial
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, agents"
    And givn addon "domain_modeling" is "true"
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    When the user runs `givn addons list`
    Then the command exits 1
    And the listing reports domain_modeling as enabled with artifacts partial
    And the listing names "agents" as a selected target missing the skill
    And the listing suggests `givn skills sync`

  @givn.added @wip
  Scenario: A retired addon key in the configuration is reported with its replacement
    Given an initialized project whose givn/config.yaml contains the retired key "grill_me"
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports "grill_me" as retired
    And the listing names "challenge" as the replacement
    And the listing performs no configuration change

  @givn.added @wip
  Scenario: An enabled addon whose effect is a managed block reports the block state
    Given an initialized project with communication_policy enabled
    And the managed communication block in "AGENTS.md" is intact
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports communication_policy as enabled with artifacts present

  @givn.added @wip
  Scenario: A damaged managed block for an enabled addon fails the command
    Given an initialized project with communication_policy enabled
    And the managed communication block in "AGENTS.md" is damaged
    When the user runs `givn addons list`
    Then the command exits 1
    And the listing reports communication_policy as enabled with artifacts missing
    And the listing names the damaged managed block

  @givn.added @wip
  Scenario: The listing writes nothing to the project
    Given an initialized project with arc42 enabled
    And the directory "docs/arc42" does not exist
    When the user runs `givn addons list`
    Then the project files are unchanged
    And givn config keeps addon "arc42" enabled

  @givn.added @wip
  Scenario: Disabling domain_modeling keeps the shared glossary from a partial report
    Given an initialized project with arc42 enabled
    And givn addon "domain_modeling" is "false"
    And the file "docs/glossary.md" exists
    And the directory ".opencode/skills/givn-domain-modeling" does not exist
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports domain_modeling as disabled with artifacts none
    And the listing reports arc42 as enabled with artifacts present

  @givn.added @wip
  Scenario: A locked change adds the lock to each suggestion
    Given an initialized project with the readme addon disabled
    And the file "README.md" exists
    When the user runs `givn new addon-lock`
    And the user runs `givn addons list`
    Then the command exits 0
    And the listing suggests `givn addons disable readme`
    And the suggestion names "addon-lock" as the change holding the add-on lock
    And givn config keeps addon "readme" disabled

  @givn.added @wip
  Scenario: An addon-free project listing reports every addon explicitly disabled
    Given a project root with givn/ initialised without addons
    When the user runs `givn addons list`
    Then the command exits 0
    And the listing reports every registered addon as disabled
