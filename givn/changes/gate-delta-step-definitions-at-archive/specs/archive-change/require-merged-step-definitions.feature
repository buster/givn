@givn.delta @require-merged-step-definitions

Feature: Require Merged Step Definitions

  @givn.added @wip
  Scenario: Archive is blocked when a delta step definition is missing from the permanent test tree
    Given  a givn project with a completed change "unmerged-steps" ready to archive
    And  the change's spec tree binds "an imported catalog" in "steps/catalog_steps.py"
    And  no permanent step-definition file binds "an imported catalog"
    When  the user runs `givn archive --change unmerged-steps`
    Then  the command exits non-zero
    And  stderr names "specs/steps/catalog_steps.py"
    And  stderr names "an imported catalog"
    And  stderr says the definition must be merged into the permanent per-capability step file
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "unmerged-steps"

  @givn.added @wip
  Scenario: The refusal lists a bounded number of missing step phrases and a remainder count
    Given  a givn project with a completed change "many-unmerged-steps" ready to archive
    And  the change's spec tree binds 21 distinct step phrases in "steps/catalog_steps.py"
    And  no permanent step-definition file binds any of those step phrases
    When  the user runs `givn archive --change many-unmerged-steps`
    Then  the command exits non-zero
    And  stderr names the alphabetically first 20 missing step phrases
    And  stderr does not name the twenty-first missing step phrase
    And  stderr says "+1 more"
    And  stderr names "specs/steps/catalog_steps.py"

  @givn.added @wip
  Scenario: An unproven archive candidate with a missing step definition is not verified again
    Given  an unproven archive candidate "pending-steps" whose full suite failed
    And  the candidate's spec tree binds "an imported catalog" in "steps/catalog_steps.py"
    And  no permanent step-definition file binds "an imported catalog"
    And  the runner records every invocation with its scope and features path
    When  the user runs `givn archive --change pending-steps`
    Then  the command exits non-zero
    And  stderr names "an imported catalog"
    And  stderr names "specs/steps/catalog_steps.py"
    And  the candidate is still unproven
    And  no regular scope result exists for the candidate

  @givn.added @e2e @wip
  Scenario: Consumer archives a change whose delta step definitions are already in the permanent test tree
    Given  a givn project with a completed change "merged-steps" ready to archive
    And  the change's spec tree binds "an imported catalog" in "steps/catalog_steps.py"
    And  a permanent step-definition file "tests/steps/catalog_steps.py" binds "an imported catalog"
    When  the user invokes `givn archive --change merged-steps`
    Then  the command exits 0
    And  the change directory has been moved to givn/archive/
    And  the archived change still contains the step-definition file "specs/steps/catalog_steps.py"
