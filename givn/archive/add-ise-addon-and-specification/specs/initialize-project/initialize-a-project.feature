@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.modified @e2e
  Scenario: Consumer selects agent targets during interactive init
    Given  a project root without a givn/ directory
    When  the consumer selects opencode and claude in the real terminal init prompt
    Then  the terminal displays a labeled agent-target selection and confirms the selected integrations
    And  the command exits 0
    And  givn skills targets in givn/config.yaml equal "opencode, claude"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the directory ".agents/skills/givn-propose" does not exist
    And  the active addon settings enable exactly "arc42, communication_policy, readme, coverage, domain_modeling, dev_principles, architecture_principles, challenge, outline"
    And  the active addon settings do not name "ise"
    And  the file "givn/specification.yaml" does not exist
    And  the terminal transcript is captured for this scenario
