@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.modified @e2e @wip
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given  a project root without a givn/ directory
    And  the root contains human-authored agent instructions to preserve
    When  the consumer runs `givn init --no-addons --agents opencode`
    Then  the command exits 0
    And  the initialization report names opencode and states that optional addons are disabled
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the installed givn-propose guidance directs the author to the proposal instruction
    And  the directory ".claude/skills/givn-propose" does not exist
    And  the generated project-start guidance requires the effective dialog policy before every context's first user-facing question
    And  the obligation covers questions outside named skills and requires explicit handoff to delegated agents
    And  the project-start guidance contains a policy reference rather than the worked dialog examples
    And  the human-authored agent instructions remain unchanged
    And  no optional tone addon is required for the obligation

  @givn.added @wip
  Scenario: A full copy installed by an earlier version is reconciled to the direction
    Given a project root with givn/ initialised
    And a full-copy givn-propose skill installed by an earlier version
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" directs the author to the proposal instruction
    And the file ".opencode/skills/givn-propose/SKILL.md" carries no built-in rule text

  @givn.added @wip
  Scenario: The project-start block renders its resident rules from the tasks instruction
    Given a Givn project with an active change
    When the user runs `givn skills sync`
    Then the project-start block carries the resident rules section of the tasks instruction
    And the project-start block requires the effective dialog policy
