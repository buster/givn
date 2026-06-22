@givn.delta @new-command
Feature: new-command
  Characterize the existing `givn new <id>` CLI capability.

  The black-box boundary is the CLI surface `givn new <id>`, observed through
  its exit code, stdout, and the filesystem effects (the created directory and
  the contents of every file written inside it).

  All scenarios are provisional — inferred from probing the real binary — and
  will be validated during characterization. If a scenario's assertion is
  proven wrong against the live code, the scenario is corrected to match
  AS-IS behaviour; production code is never changed.

  @givn.added
  Scenario: Valid multi-word kebab-case id creates the change scaffold
    Given a project root with givn/ initialised
    And no existing change named "add-user-auth"
    When the user runs `givn new add-user-auth`
    Then the command exits 0
    And the directory givn/changes/add-user-auth/ exists
    And the file givn/changes/add-user-auth/proposal.md exists
    And stdout contains a line "Created change 'add-user-auth'"
    And stdout contains the absolute path to givn/changes/add-user-auth/proposal.md
    And stdout contains a line beginning with "Next:"
    And stdout mentions "givn instructions proposal --change add-user-auth"

  @givn.added
  Scenario: Single-word id is accepted
    Given a project root with givn/ initialised
    When the user runs `givn new feature`
    Then the command exits 0
    And the directory givn/changes/feature/ exists
    And the file givn/changes/feature/proposal.md exists

  @givn.added
  Scenario: Id containing digits is accepted
    Given a project root with givn/ initialised
    When the user runs `givn new fix-123`
    Then the command exits 0
    And the directory givn/changes/fix-123/ exists

  @givn.added
  Scenario: Empty id is rejected
    # The empty string reaches validate_id (clap does not intercept it);
    # the validator rejects it before the givn/-not-found check, so the
    # init-root fixture yields the empty-id error.
    Given a project root with givn/ initialised
    When the user runs `givn new ""`
    Then the command exits 1
    And stderr contains "change id must not be empty"
    And no change directory has been created

  @givn.added
  Scenario: Id with uppercase letters is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new HasUppercase`
    Then the command exits non-zero
    And stderr contains "kebab-case"
    And no directory givn/changes/HasUppercase/ has been created

  @givn.added
  Scenario: Id with underscores is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new has_underscores`
    Then the command exits non-zero
    And stderr contains "kebab-case"
    And no directory givn/changes/has_underscores/ has been created

  @givn.added
  Scenario: Id with spaces is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new "has spaces"`
    Then the command exits non-zero
    And stderr contains "kebab-case"

  @givn.added
  Scenario: Id with non-ASCII letters is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new café`
    Then the command exits non-zero
    And stderr contains "kebab-case"

  @givn.added
  Scenario: Id with a dot is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new a.b`
    Then the command exits non-zero
    And stderr contains "kebab-case"

  @givn.added
  Scenario: Id ending with a hyphen is rejected
    Given a project root with givn/ initialised
    When the user runs `givn new foo-`
    Then the command exits non-zero
    And stderr contains "must not start or end with a hyphen"

  @givn.added
  Scenario: Id beginning with a hyphen is rejected by argument parsing
    Given a project root with givn/ initialised
    When the user runs `givn new -foo`
    Then the command exits with the argument-parsing exit code 2
    And stderr mentions an unexpected argument
    And no change directory has been created

  @givn.added
  Scenario: Id beginning with a hyphen passed after the argument separator is rejected by validation
    Given a project root with givn/ initialised
    When the user runs `givn new -- -foo`
    Then the command exits non-zero
    And stderr contains "must not start or end with a hyphen"

  @givn.added
  Scenario: Uninitialised project is rejected
    Given a project root without a givn/ directory
    When the user runs `givn new some-change`
    Then the command exits non-zero
    And stderr contains "givn/ not found"
    And stderr mentions running init first
    And no change directory has been created

  @givn.added
  Scenario: Already-existing change is rejected
    Given a project root with givn/ initialised
    And an existing change named "dup-change"
    When the user runs `givn new dup-change`
    Then the command exits non-zero
    And stderr contains "already exists"
    And stderr contains the absolute path to givn/changes/dup-change

  @givn.added
  Scenario: Only proposal.md is created in the change directory
    Given a project root with givn/ initialised
    When the user runs `givn new only-proposal`
    Then the command exits 0
    And the change directory givn/changes/only-proposal/ contains exactly one file: proposal.md
    And no subdirectory named specs/ exists inside givn/changes/only-proposal/

  @givn.added
  Scenario: proposal.md content equals the built-in template verbatim
    Given a project root with givn/ initialised
    When the user runs `givn new template-check`
    Then the command exits 0
    And the file givn/changes/template-check/proposal.md equals the built-in proposal template byte for byte
    And the proposal.md heading contains the literal text "{{CHANGE_ID}}"

  @givn.added
  Scenario: Project override at givn/artifacts/proposal/proposal.md is used instead of the built-in template
    Given a project root with givn/ initialised
    And a project override file at givn/artifacts/proposal/proposal.md with custom content
    When the user runs `givn new with-override`
    Then the command exits 0
    And the file givn/changes/with-override/proposal.md equals the project override content byte for byte
    And the file givn/changes/with-override/proposal.md does not contain "{{CHANGE_ID}}"
