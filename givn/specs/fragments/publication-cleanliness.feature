Feature: Publication cleanliness

  Scenario: The dedicated check names a tracked rule and exits non-zero
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn check rules`
    Then  the command exits non-zero
    And  stdout contains ".opencode/skills/givn-propose/SKILL.md"

  Scenario: The dedicated check passes on a clean project
    Given  a committed project root with givn/ initialised
    When  the user runs `givn check rules`
    Then  the command exits 0
    And  stdout contains "no tracked rule in the current revision"

  Scenario: The check names rules that survive in reachable history
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn check rules`
    Then  the command exits non-zero
    And  stdout contains "reachable revision(s) still contain a rule"
