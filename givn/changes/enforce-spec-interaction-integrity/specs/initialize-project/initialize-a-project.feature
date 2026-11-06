@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.modified @initialize-a-project @e2e
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
