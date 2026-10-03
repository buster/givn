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

  Scenario: Skills and commands are not installed to an agent excluded from the selection
    Given a project root without a givn/ directory
    And a givn/config.yaml with skills targets "claude"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".opencode/skills/givn-propose" does not exist
    And the file ".opencode/commands/givn-propose.md" does not exist

  Scenario: givn skills targets add installs the new agent without touching anything else
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add claude`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists

  Scenario: givn skills targets add accepts "all" as a shorthand for every target
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add all`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude"
    And the file ".agents/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists

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
    Given a project root without a givn/ directory
    When the consumer runs `givn init` and answers the agent prompt with "opencode, .claude"
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

  @e2e
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given  a project root without a givn/ directory
    When  the consumer runs `givn init --agents opencode`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the directory ".claude/skills/givn-propose" does not exist

  @e2e
  Scenario: Consumer re-syncs skills after init and the selection is honoured
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the consumer runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

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
    Given a project root without a givn/ directory
    When the user runs `givn init --agents all`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the file ".agents/skills/givn-propose/SKILL.md" exists
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude"

  Scenario: An unknown agent name in the --agents flag is dropped with a hint
    Given a project root without a givn/ directory
    When the user runs `givn init --agents claude,nope`
    Then the command exits 0
    And stdout mentions "opencode"
    And stdout mentions ".agents"
    And stdout mentions ".claude"
    And givn skills targets in givn/config.yaml equal "claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists
  @e2e
  Scenario: Consumer initializes a Fossil checkout and receives backend-aware guidance
    Given  a Fossil checkout without a givn/ directory
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  the generated guidance names the commit path of the active backend

  @e2e
  Scenario: Consumer initializes a plain directory and receives a named warning
    Given  a project root without a Git worktree or a Fossil checkout
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  stdout warns that archive and managed upgrade are unavailable
    And  a later archive is refused by name
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
