Feature: init-agent-target-selection

  Scenario: Non-interactive init installs skills to opencode only
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".opencode/commands/givn-propose.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
    And the directory ".claude/skills/givn-propose" does not exist

  Scenario: Interactive init installs only the agents the user selects
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "opencode, .claude"
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

  Scenario: Interactive init accepts the default selection with an empty answer
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with ""
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".claude/skills/givn-propose" does not exist
    And the directory ".agents/skills/givn-propose" does not exist

  Scenario: The selected agents are persisted in givn/config.yaml
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "opencode, .claude"
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, claude"

  Scenario: Non-interactive init persists opencode as the default target
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode"

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

  Scenario: Re-running init keeps the persisted selection
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "claude"
    When the user runs `givn init`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "claude"

  Scenario: givn init --force re-applies the default selection
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the user runs `givn init --force`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode"

  Scenario: An unknown agent name in the answer is dropped with a hint listing the valid agents
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "claude, nope"
    Then the command exits 0
    And stdout mentions "opencode"
    And stdout mentions ".agents"
    And stdout mentions ".claude"
    And givn skills targets in givn/config.yaml equal "claude"
    And the file ".claude/skills/givn-propose/SKILL.md" exists

  Scenario: An answer without any known agent falls back to the default
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "nope"
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode"

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
    Given a project root without a givn/ directory
    When the consumer invokes `givn init` without answering a prompt
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".claude/skills/givn-propose" does not exist

  @e2e
  Scenario: Consumer re-syncs skills after init and the selection is honoured
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the consumer runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
