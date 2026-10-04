@givn.delta @initialize-a-project
Feature: Unconditional dialog obligations in project startup guidance

  # Interaction inventory: specs/initialize-project/usecase.md.
  # Initialize non-interactively -> Consumer runs init non-interactively and gets opencode skills
  # Resynchronize selected targets -> Consumer re-syncs skills after init and the selection is honoured
  # Addon-disabled startup guidance and existing human text are invariant fixtures, not new actions.
  # Interactive target selection -> Consumer selects agent targets during interactive init
  # Fossil and no-SCM are init preconditions; remove their redundant E2E identities explicitly.

  @givn.modified @e2e @wip
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given a project root without a givn/ directory
    And the root contains human-authored agent instructions to preserve
    When the consumer runs `givn init --no-addons --agents opencode`
    Then the command exits 0
    And the initialization report names opencode and states that optional addons are disabled
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".claude/skills/givn-propose" does not exist
    And the generated project-start guidance requires the effective dialog policy before every context's first user-facing question
    And the obligation covers questions outside named skills and requires explicit handoff to delegated agents
    And the project-start guidance contains a policy reference rather than the worked dialog examples
    And the human-authored agent instructions remain unchanged
    And no optional tone addon is required for the obligation

  @givn.modified @e2e @wip
  Scenario: Consumer re-syncs skills after init and the selection is honoured
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    And the project contains an older managed startup block beside human-authored agent instructions
    When the consumer runs `givn skills sync`
    Then the command exits 0
    And the synchronization report identifies the selected guidance destinations
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
    And the managed project-start block now requires the effective dialog policy for every user-facing question
    And the managed block is not duplicated
    And the human-authored agent instructions remain unchanged
    And the persisted agent selection and unrelated configuration remain unchanged

  @givn.modified @e2e @wip
  Scenario: Consumer selects agent targets during interactive init
    Given a project root without a givn/ directory
    When the consumer selects opencode and claude in the real terminal init prompt
    Then the terminal displays a labeled agent-target selection and confirms the selected integrations
    And the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist

  @givn.removed @e2e @wip
  Scenario: Consumer initializes a Fossil checkout and receives backend-aware guidance
    Given the Fossil-precondition contract is preserved as a regular initialization variant

  @givn.added @wip
  Scenario: Fossil initialization names the supported attribution path without Git plumbing
    Given a Fossil checkout without a givn/ directory
    When the consumer runs `givn init --agents opencode`
    Then the command exits 0
    And stdout names the givn commit attribution path
    And no .githooks directory exists
    And the generated guidance names the commit path of the active backend
    And the generated guidance states that a raw fossil commit carries no attribution

  @givn.removed @e2e @wip
  Scenario: Consumer initializes a plain directory and receives a named warning
    Given the no-SCM contract is preserved as a regular initialization variant

  @givn.added @wip
  Scenario: Initialization without SCM preserves guidance and names the later archive refusal
    Given a project root without a Git worktree or a Fossil checkout
    When the consumer runs `givn init --agents opencode`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And stdout warns that archive and managed upgrade are unavailable
    And a later archive is refused by name
