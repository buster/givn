@givn.delta @initialize-a-project
Feature: Initialize skills-only project integrations

  @givn.modified @e2e
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given a project root without a givn/ directory
    And human-authored agent instructions outside the managed block
    When the consumer runs `givn init --no-addons --agents opencode`
    Then the command exits 0 and reports installed skills and disabled optional addons
    And the installed givn-propose skill directs the author to `givn instructions proposal`
    And no Givn command file is generated
    And the managed startup block requires loading the effective dialog policy before the first question in each context independently of addons
    And human-authored instructions remain unchanged
    And the terminal transcript is captured for this scenario

  @givn.modified @e2e
  Scenario: Consumer re-syncs skills after init and the selection is honoured
    Given an initialized project selecting opencode and claude
    And an older managed startup block beside human-authored instructions
    And recognizable Givn-generated legacy commands beside custom commands
    When the consumer runs `givn skills sync`
    Then the command exits 0 and reports the selected skill destinations
    And only selected targets receive skills
    And recognizable generated legacy commands are removed without generating replacements
    And custom commands and human-authored instructions remain byte-for-byte unchanged
    And the startup block is refreshed once without changing the persisted selection or unrelated configuration
    And the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: Skills and commands are not installed to an agent excluded from the selection
    Given an initialized project selecting only claude
    When the user runs `givn skills sync`
    Then only claude receives generated Givn skills
    And no target receives generated Givn commands

  @givn.modified
  Scenario: The omp target receives its own guidance and attribution integration
    Given a project root without a givn/ directory
    When the user runs `givn init --agents omp`
    Then the command exits 0
    And the omp givn-propose skill and attribution integration exist
    And no omp Givn command file exists
    And the guidance explains invocation as `/skill:givn-propose`
    And no unselected target receives skills

  @givn.modified
  Scenario: An omp project keeps the generated omp rules out of the repository
    Given a committed omp project with tracked recognizable generated legacy command files and generated skills
    When the user runs `givn skills sync`
    Then the command exits 0
    And recognizable generated legacy commands are removed
    And installed generated skills and attribution integration are ignored and not tracked
    And no replacement command files are generated

  @givn.added
  Scenario Outline: Every supported target discovers thin skills without generated commands
    Given an initialized project selecting only "<target>"
    When the user runs `givn skills sync`
    Then "<target>" can discover the givn-propose skill and its change argument through "<invocation>"
    And the skill contains trigger metadata and the direction to `givn instructions proposal`
    And the skill contains no duplicated rules, orchestration, delegation, or repeated dialog-policy block
    And no Givn command files are generated

    Examples:
      | target   | invocation                         |
      | opencode | supported native skill invocation  |
      | agents   | supported native skill discovery   |
      | claude   | supported native skill invocation  |
      | omp      | /skill:givn-propose                 |
      | maki     | explicit request                   |
      | maki     | agent-selected skill               |

  @givn.added
  Scenario: Maki guidance does not promise generated slash commands
    Given an initialized project selecting maki
    When the user runs `givn skills sync`
    Then the guidance explains explicit requests and agent-selected skills
    And the guidance does not promise a generated `/givn-propose` command

  @givn.added
  Scenario: A custom legacy command is preserved and its manual transition is reported
    Given an initialized project with a customized legacy givn-propose command
    When the user runs `givn skills sync`
    Then the custom command remains byte-for-byte unchanged
    And the report names the custom command requiring manual transition
    And a matching filename alone is not treated as evidence of Givn ownership

  @givn.added
  Scenario Outline: Init reentry preserves explicit addon opt-outs
    Given an initialized project with outline and challenge explicitly disabled and other addon settings preserved
    When the user runs `<command>`
    Then the command exits 0
    And outline and challenge remain disabled in configuration, installed guidance, and the initialization report
    And unrelated addon choices remain unchanged

    Examples:
      | command                                    |
      | givn init                                  |
      | givn init --force --agents opencode         |

  @givn.added
  Scenario: Required skill synchronization failure leaves an accurately reported partial initialization
    Given a writable new project whose required opencode skill destination cannot be written
    When the user runs `givn init --agents opencode`
    Then the command exits nonzero
    And the report identifies initialization as partial and names the failed destination
    And the report names `givn skills sync` as the retry after correcting that destination
    And the report does not declare the project Ready or claim rollback
    And successfully written scaffold files remain available for retry

  @givn.added
  Scenario: Retrying skill synchronization completes partial initialization without losing choices
    Given a partially initialized project after required skill synchronization failed
    And the destination has been made writable
    When the user runs `givn skills sync`
    Then the command exits 0 and reports installed skills
    And the persisted target selection and explicit addon opt-outs remain unchanged

  @givn.added
  Scenario: A new project declares verification runners as unconfigured
    Given a project root without a givn/ directory or test scripts
    When the user runs `givn init --agents opencode`
    Then the command exits 0
    And both verification scopes declare an explicit unconfigured runner sentinel
    And verification configuration does not assume `./run-tests.sh` or `./run-tests.sh --e2e` exists

  @givn.added
  Scenario: The served runner guidance states the complete adapter contract
    Given a project initialized with unconfigured verification runners
    When the user invokes the unconfigured test-runner sentinel
    Then the commands template requires selected GIVN_FEATURES and every GIVN_RESULT_SCOPE
    And the commands template requires fresh actual counts and fail-closed outcomes
    And the commands template points to the runner contract instead of a generic script
    And the sentinel names runner choices and the adapter contract
    And the sentinel exits non-zero

  @givn.added
  Scenario Outline: Target selection edits preserve unrelated YAML or refuse before writing
    Given an initialized project with "<presentation>" configuration and unrelated settings
    When the user runs `givn skills targets add claude`
    Then the target choice alone is updated or unsupported YAML is clearly refused before any write
    And unrelated settings and custom guidance are unchanged
    And no duplicate skills section is introduced

    Examples:
      | presentation                |
      | block mapping with comments |
      | flow mapping                |
      | anchors and aliases         |
