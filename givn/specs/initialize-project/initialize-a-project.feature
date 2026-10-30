Feature: initialize-a-project

  Scenario: Interactive init installs only the agents the user selects
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "opencode, .claude"
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

  Scenario: The selected agents are persisted in givn/config.yaml
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "opencode, .claude"
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, claude"

  Scenario: givn skills sync installs to the persisted selection only
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

  @initialize-a-project
  Scenario: Skills and commands are not installed to an agent excluded from the selection
    Given  an initialized project selecting only claude
    When  the user runs `givn skills sync`
    Then  only claude receives generated Givn skills
    And  no target receives generated Givn commands

  Scenario: givn skills targets add installs the new agent without touching anything else
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add claude`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists

  Scenario: givn skills targets add accepts "all" as a shorthand for every target
    Given  a project root with givn/ initialised
    And  givn skills targets in givn/config.yaml equal "opencode"
    When  the user runs `givn skills targets add all`
    Then  the command exits 0
    And  givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp, maki"
    And  the file ".agents/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the file ".omp/skills/givn-propose/SKILL.md" exists

  Scenario: givn skills targets add drops an unknown name with a hint
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add claude,nope`
    Then the command exits 0
    And stdout mentions ".claude"
    And givn skills targets in givn/config.yaml equal "opencode, claude"

  Scenario: givn skills targets add errors when every given name is unknown
    Given a project root with givn/ initialised
    When the user runs `givn skills targets add nope`
    Then the command exits non-zero

  Scenario: givn skills targets add errors when givn/ is not initialised
    Given a project root without a givn/ directory
    When the user runs `givn skills targets add claude`
    Then the command exits non-zero

  Scenario: Re-running init keeps the persisted selection
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "claude"
    When the user runs `givn init`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "claude"

  Scenario: An unknown agent name in the answer is dropped with a hint listing the valid agents
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "claude, nope"
    Then the command exits 0
    And stdout mentions "opencode"
    And stdout mentions ".agents"
    And stdout mentions ".claude"
    And givn skills targets in givn/config.yaml equal "claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists

  Scenario: Interactive init can select the .agents target
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "opencode, .agents"
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, agents"
    And the file ".agents/skills/givn-propose/SKILL.md" exists

  Scenario: givn skills sync defaults to opencode when the config has no skills block
    Given a project root without a givn/ directory
    And a givn/config.yaml without a skills block
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".claude/skills/givn-propose" does not exist

  @e2e
  Scenario: Consumer selects agent targets during interactive init
    Given  a project root without a givn/ directory
    When  the consumer selects opencode and claude in the real terminal init prompt
    Then  the terminal displays a labeled agent-target selection and confirms the selected integrations
    And  the command exits 0
    And  givn skills targets in givn/config.yaml equal "opencode, claude"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the directory ".agents/skills/givn-propose" does not exist

  @initialize-a-project
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given  a project root without a givn/ directory
    And  human-authored agent instructions outside the managed block
    When  the consumer runs `givn init --no-addons --agents opencode`
    Then  the command exits 0 and reports installed skills and disabled optional addons
    And  the installed givn-propose skill directs the author to `givn instructions proposal`
    And  no Givn command file is generated
    And  the managed startup block requires loading the effective dialog policy before the first question in each context independently of addons
    And  the file ".opencode/skills/givn-start/SKILL.md" exists
    And  the file ".opencode/skills/givn-explore/SKILL.md" does not exist

  @e2e @initialize-a-project
  Scenario: Consumer re-syncs skills after init and the selection is honoured
    Given  an initialized project selecting opencode and claude
    And  an older managed startup block beside human-authored instructions
    And  recognizable Givn-generated legacy commands beside custom commands
    When  the consumer runs `givn skills sync`
    Then  the command exits 0 and reports the selected skill destinations
    And  only selected targets receive skills
    And  recognizable generated legacy commands are removed without generating replacements
    And  custom commands and human-authored instructions remain byte-for-byte unchanged
    And  the startup block is refreshed once without changing the persisted selection or unrelated configuration
    And  the terminal transcript is captured for this scenario

  Scenario: The --agents flag selects targets non-interactively, no prompt needed
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode,claude`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
    And givn skills targets in givn/config.yaml equal "opencode, claude"

  Scenario: The --agents flag accepts the dotted directory form
    Given a project root without a givn/ directory
    When the user runs `givn init --agents .claude,.agents`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "claude, agents"

  Scenario: The --agents flag accepts "all" as a shorthand for every target
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents all`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the file ".agents/skills/givn-propose/SKILL.md" exists
    And  the file ".omp/skills/givn-propose/SKILL.md" exists
    And  givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp, maki"

  Scenario: An unknown agent name in the --agents flag is dropped with a hint
    Given a project root without a givn/ directory
    When the user runs `givn init --agents claude,nope`
    Then the command exits 0
    And stdout mentions "opencode"
    And stdout mentions ".agents"
    And stdout mentions ".claude"
    And givn skills targets in givn/config.yaml equal "claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists

  Scenario: Initialization keeps the generated rules out of the repository
    Given  a committed project root with givn/ initialised
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-propose/SKILL.md"
    And  the tracked ignore block contains ".opencode/skills/givn-*/"
    And  the tracked ignore block masks the rule names
    And  the local exclude contains ".opencode/skills/givn-*/"

  Scenario: Initialization keeps a rule from a disabled addon out of the repository
    Given  a committed project root with givn/ initialised
    And  a tracked orphan rule at ".opencode/skills/givn-stale/SKILL.md"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-stale/SKILL.md"
    And  the file ".opencode/skills/givn-stale/SKILL.md" exists on disk

  Scenario: Non-interactive init without an agent target refuses initialization
    Given  a project root without a givn/ directory
    When  the user runs `givn init`
    Then  the command exits non-zero
    And  stderr contains "no agent target selected"
    And  stderr contains ".opencode"
    And  stderr contains ".claude"
    And  stderr contains "--agents"
    And  the directory "givn" does not exist

  Scenario: An empty answer refuses initialization
    Given  a project root without a givn/ directory
    When  the user runs `givn init` and answers the agent prompt with ""
    Then  the command exits non-zero
    And  stderr contains "no agent target selected"
    And  the directory "givn" does not exist

  Scenario: An answer without any known agent refuses initialization
    Given  a project root without a givn/ directory
    When  the user runs `givn init` and answers the agent prompt with "nope"
    Then  the command exits non-zero
    And  stderr contains "no agent target selected"
    And  stderr contains ".agents"
    And  the directory "givn" does not exist

  Scenario: An unknown-only --agents value refuses initialization
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents nope`
    Then  the command exits non-zero
    And  stderr contains "no agent target selected"
    And  the directory "givn" does not exist

  Scenario: givn init --force re-applies the explicitly given selection
    Given  a project root with givn/ initialised
    And  givn skills targets in givn/config.yaml equal "opencode, claude"
    When  the user runs `givn init --force --agents opencode`
    Then  the command exits 0
    And  givn skills targets in givn/config.yaml equal "opencode"

  Scenario: givn init --force without a selection keeps the persisted selection
    Given  a project root with givn/ initialised
    And  givn skills targets in givn/config.yaml equal "opencode, claude"
    When  the user runs `givn init --force`
    Then  the command exits non-zero
    And  stderr contains "no agent target selected"
    And  givn skills targets in givn/config.yaml equal "opencode, claude"

  Scenario: The interactive prompt accepts the pre-marked OpenCode default
    Given  a project root without a givn/ directory
    When  the user runs `givn init` in a terminal and confirms the prompt
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the directory ".agents/skills/givn-propose" does not exist
    And  the directory ".claude/skills/givn-propose" does not exist

  Scenario: The interactive prompt keeps asking until a target is marked
    Given  a project root without a givn/ directory
    When  the user runs `givn init` in a terminal, unmarks every target, and confirms twice
    Then  the command exits 0
    And  stderr contains "mark at least one agent target"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the directory ".agents/skills/givn-propose" does not exist

  Scenario: Fossil initialization names the supported attribution path without Git plumbing
    Given  a Fossil checkout without a givn/ directory
    When  the consumer runs `givn init --agents opencode`
    Then  the command exits 0
    And  stdout names the givn commit attribution path
    And  no .githooks directory exists
    And  the generated guidance names the commit path of the active backend
    And  the generated guidance states that a raw fossil commit carries no attribution

  Scenario: Initialization without SCM preserves guidance and names the later archive refusal
    Given  a project root without a Git worktree or a Fossil checkout
    When  the consumer runs `givn init --agents opencode`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  stdout warns that archive and managed upgrade are unavailable
    And  a later archive is refused by name

  @initialize-a-project
  Scenario: The omp target receives its own guidance and attribution integration
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents omp`
    Then  the command exits 0
    And  the omp givn-propose skill and attribution integration exist
    And  no omp Givn command file exists
    And  the guidance explains invocation as `/skill:givn-propose`
    And  no unselected target receives skills

  @initialize-a-project
  Scenario: An omp project keeps the generated omp rules out of the repository
    Given  a committed omp project with tracked recognizable generated legacy command files and generated skills
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  recognizable generated legacy commands are removed
    And  installed generated skills and attribution integration are ignored and not tracked
    And  no replacement command files are generated

  Scenario: The --agents flag accepts the dotted directory form for the omp target
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents .omp`
    Then  the command exits 0
    And  givn skills targets in givn/config.yaml equal "omp"
    And  the file ".omp/skills/givn-propose/SKILL.md" exists

  Scenario: A full copy installed by an earlier version is reconciled to the direction
    Given  a project root with givn/ initialised
    And  a full-copy givn-propose skill installed by an earlier version
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" directs the author to the proposal instruction
    And  the file ".opencode/skills/givn-propose/SKILL.md" carries no built-in rule text

  Scenario: The project-start block renders its resident rules from the tasks instruction
    Given  a Givn project with an active change
    When  the user runs `givn skills sync`
    Then  the project-start block carries the resident rules section of the tasks instruction
    And  the project-start block requires the effective dialog policy

  @initialize-a-project
  Scenario Outline: Every supported target discovers thin skills without generated commands
    Given  an initialized project selecting only "<target>"
    When  the user runs `givn skills sync`
    Then  "<target>" can discover the givn-propose skill and its change argument through "<invocation>"
    And  the skill contains trigger metadata and the direction to `givn instructions proposal`
    And  the skill contains no duplicated rules, orchestration, delegation, or repeated dialog-policy block
    And  no Givn command files are generated
    Examples:
      | target | invocation |
      | opencode | supported native skill invocation |
      | agents | supported native skill discovery |
      | claude | supported native skill invocation |
      | omp | /skill:givn-propose |
      | maki | explicit request |
      | maki | agent-selected skill |

  @initialize-a-project
  Scenario: Maki guidance does not promise generated slash commands
    Given  an initialized project selecting maki
    When  the user runs `givn skills sync`
    Then  the guidance explains explicit requests and agent-selected skills
    And  the guidance does not promise a generated `/givn-propose` command

  @initialize-a-project
  Scenario: A custom legacy command is preserved and its manual transition is reported
    Given  an initialized project with a customized legacy givn-propose command
    When  the user runs `givn skills sync`
    Then  the custom command remains byte-for-byte unchanged
    And  the report names the custom command requiring manual transition
    And  a matching filename alone is not treated as evidence of Givn ownership

  @initialize-a-project
  Scenario Outline: Init reentry preserves explicit addon opt-outs
    Given  an initialized project with outline and challenge explicitly disabled and other addon settings preserved
    When  the user runs `<command>`
    Then  the command exits 0
    And  outline and challenge remain disabled in configuration, installed guidance, and the initialization report
    And  unrelated addon choices remain unchanged
    Examples:
      | command |
      | givn init |
      | givn init --force --agents opencode |

  @initialize-a-project
  Scenario: Required skill synchronization failure leaves an accurately reported partial initialization
    Given  a writable new project whose required opencode skill destination cannot be written
    When  the user runs `givn init --agents opencode`
    Then  the command exits nonzero
    And  the report identifies initialization as partial and names the failed destination
    And  the report names `givn skills sync` as the retry after correcting that destination
    And  the report does not declare the project Ready or claim rollback
    And  successfully written scaffold files remain available for retry

  @initialize-a-project
  Scenario: Retrying skill synchronization completes partial initialization without losing choices
    Given  a partially initialized project after required skill synchronization failed
    And  the destination has been made writable
    When  the user runs `givn skills sync`
    Then  the command exits 0 and reports installed skills
    And  the persisted target selection and explicit addon opt-outs remain unchanged

  @initialize-a-project
  Scenario: A new project declares verification runners as unconfigured
    Given  a project root without a givn/ directory or test scripts
    When  the user runs `givn init --agents opencode`
    Then  the command exits 0
    And  both verification scopes declare an explicit unconfigured runner sentinel
    And  verification configuration does not assume `./run-tests.sh` or `./run-tests.sh --e2e` exists

  @initialize-a-project
  Scenario: The served runner guidance states the complete adapter contract
    Given  a project initialized with unconfigured verification runners
    When  the user invokes the unconfigured test-runner sentinel
    Then  the commands template requires selected GIVN_FEATURES and every GIVN_RESULT_SCOPE
    And  the commands template requires fresh actual counts and fail-closed outcomes
    And  the commands template points to the runner contract instead of a generic script
    And  the sentinel names runner choices and the adapter contract
    And  the sentinel exits non-zero

  @initialize-a-project
  Scenario Outline: Target selection edits preserve unrelated YAML or refuse before writing
    Given  an initialized project with "<presentation>" configuration and unrelated settings
    When  the user runs `givn skills targets add claude`
    Then  the target choice alone is updated or unsupported YAML is clearly refused before any write
    And  unrelated settings and custom guidance are unchanged
    And  no duplicate skills section is introduced
    Examples:
      | presentation |
      | block mapping with comments |
      | flow mapping |
      | anchors and aliases |

  @initialize-a-project
  Scenario: Initialization onboarding names the proportionate entry point
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  stdout contains "givn-start"
    And  stdout contains "direct"
    And  stdout does not contain "givn-explore"
