@givn.delta @apply-a-managed-upgrade

Feature: Managed upgrade and legacy project migration

  @givn.added
  Scenario: A local-only upgrade publishes rule removals without AGENTS.md
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    And  the AGENTS.md file has a user change
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-propose/SKILL.md"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists on disk
    And  the latest revision does not touch "AGENTS.md"

  @givn.added
  Scenario: The upgrade reports rules that survive in reachable history
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  stdout contains "reachable revision(s) still contain a rule"
