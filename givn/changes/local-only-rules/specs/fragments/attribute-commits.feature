@givn.delta @attribute-commits

Feature: Attribute commits

  @givn.added
  Scenario: The stage-all path excludes AGENTS.md and generated rules under local-only
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    And  a pending AGENTS.md user change
    And  an untracked project file at "src/lib.rs"
    When  the user runs `givn commit -m "feat: fixture content"`
    Then  the command exits 0
    And  the revision tracks "src/lib.rs"
    And  the latest revision does not touch "AGENTS.md"

  @givn.added
  Scenario: The versioned stage-all path still excludes AGENTS.md
    Given  a committed project root with givn/ initialised
    And  a givn/config.yaml with rules "versioned"
    And  a pending AGENTS.md user change
    And  an untracked project file at "src/lib.rs"
    When  the user runs `givn commit -m "feat: fixture content"`
    Then  the command exits 0
    And  the revision tracks "src/lib.rs"
    And  the latest revision does not touch "AGENTS.md"
