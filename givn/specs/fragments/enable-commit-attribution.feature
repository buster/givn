Feature: Enable commit attribution

  @e2e
  Scenario: Consumer sees the commit attribution installation report
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  stdout reports the shared Git commit attribution hook
    And  stdout reports the OpenCode attribution integration
    And  stdout reports the Claude Code attribution integration
    And  stdout reports the omp attribution integration

  @e2e
  Scenario Outline: Consumer upgrades an existing project with attribution plumbing missing
    Given  a clean initialized Git project with "<prior integration>" attribution plumbing recorded in its history
    When  the consumer runs `givn upgrade`
    Then  the command exits 0
    And  stdout reports that commit attribution plumbing was refreshed
    And  the managed upgrade commit contains the refreshed attribution plumbing
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | prior integration | release |
      | missing | V1 floor |
      | missing | V1 latest |
      | missing | V2 latest |
      | V1 only | V1 floor |
      | V1 only | V1 latest |
      | V1 only | V2 latest |

  @e2e
  Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook
    Given  a Fossil checkout without a givn/ directory
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  stdout names the givn commit attribution path
    And  no .githooks directory exists
  @e2e
  Scenario Outline: Consumer synchronizes skills and retains compatible OpenCode attribution
    Given  an initialized project selecting OpenCode with a V1-only attribution integration
    When  the consumer runs `givn skills sync`
    Then  the command exits 0
    And  stdout reports the OpenCode attribution integration
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | release |
      | V1 floor |
      | V1 latest |
      | V2 latest |

  @e2e
  Scenario Outline: Consumer adds an agent target and receives compatible OpenCode attribution
    Given  an initialized project selecting "<targets>" without addons
    When  the consumer runs "<command>"
    Then  the command exits 0
    And  stdout reports the OpenCode attribution integration
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | targets | command | release |
      | claude | givn skills targets add opencode | V1 floor |
      | claude | givn skills targets add opencode | V1 latest |
      | claude | givn skills targets add opencode | V2 latest |
      | opencode | givn skills targets add claude | V2 latest |

  @e2e
  Scenario Outline: Consumer enables an addon and retains compatible OpenCode attribution
    Given  an initialized project selecting OpenCode with a V1-only attribution integration
    And  the "dev_principles" addon is disabled
    When  the consumer runs `givn addons enable dev_principles`
    Then  the command exits 0
    And  stdout reports the OpenCode attribution integration
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | release |
      | V1 floor |
      | V1 latest |
      | V2 latest |

  @e2e
  Scenario Outline: Consumer disables an addon and retains compatible OpenCode attribution
    Given  an initialized project selecting OpenCode with a V1-only attribution integration
    And  the "dev_principles" addon is enabled
    When  the consumer runs `givn addons disable dev_principles`
    Then  the command exits 0
    And  stdout reports the OpenCode attribution integration
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | release |
      | V1 floor |
      | V1 latest |
      | V2 latest |

  Scenario: Reinitialization without force refreshes attribution without replacing project configuration
    Given  an initialized project selecting OpenCode with a V1-only attribution integration
    And  the project configuration is snapshotted
    When  the consumer runs `givn init --no-addons`
    Then  the command exits 0
    And  the project configuration is unchanged
    And  the generated OpenCode attribution integration matches the supplied attribution behavior across supported generations

  Scenario: Initialization without addons still enables compatible OpenCode attribution
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode --no-addons`
    Then  the command exits 0
    And  stdout contains "addons:  disabled (--no-addons)"
    And  the generated OpenCode attribution integration matches the supplied attribution behavior across supported generations

  Scenario: Synchronization without the OpenCode target does not install its attribution integration
    Given  an initialized project selecting "claude" without addons
    When  the consumer runs `givn skills sync`
    Then  the command exits 0
    And  no OpenCode attribution integration exists
    And  stdout does not contain ".opencode/plugins/givn-attribution.js"
