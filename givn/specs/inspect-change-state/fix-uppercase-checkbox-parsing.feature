Feature: Uppercase Checkbox Parsing

  Scenario: Lowercase [x] checkboxes are counted as done in text mode
    Given a project root with givn/ initialised
    And a change "checkbox-demo" exists with artifact "tasks.md" containing "- [x] done task\n- [ ] pending task"
    When the user runs `givn status --change checkbox-demo`
    Then the command exits 0
    And stdout mentions "1/2 tasks"

  Scenario: Uppercase [X] checkboxes are counted as done in text mode
    Given a project root with givn/ initialised
    And a change "checkbox-upper" exists with artifact "tasks.md" containing "- [X] first task\n- [ ] second task"
    When the user runs `givn status --change checkbox-upper`
    Then the command exits 0
    And stdout mentions "1/2 tasks"

  Scenario: Mixed-case checkboxes are both counted as done in JSON mode
    Given a project root with givn/ initialised
    And a change "checkbox-mixed" exists with artifact "tasks.md" containing "- [x] task one\n- [X] task two\n- [ ] task three"
    When the user runs `givn status --change checkbox-mixed --json`
    Then the command exits 0
    And stdout mentions "\"tasks_done\": 2"
    And stdout mentions "\"tasks_total\": 3"

  Scenario: Uppercase checkboxes satisfy the archive gate
    Given a project root with givn/ initialised
    And a change "checkbox-archive" exists with all required artifacts complete and tasks.md containing "- [X] all done" with a valid COMMIT hash
    And the project declares a complete run declaration
    When the user runs `givn archive --change checkbox-archive`
    Then the command exits 0
    And the change directory has been moved to givn/archive/

  @e2e
  Scenario: Consumer runs givn status on a change with uppercase checkboxes
    Given a givn project with a change "e2e-upper-check" containing tasks.md with content "- [X] hello world\n- [ ] not done"
    When the user invokes `givn status --change e2e-upper-check`
    Then stdout contains "1/2 tasks"
    And the command exits 0
