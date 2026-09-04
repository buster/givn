Feature: lint-command

  Scenario: No .feature files under givn/ reports clean and exits 0
    Given  a project root with givn/ initialised
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: no .feature files found under givn/"

  Scenario: All clean .feature files report a clean summary on stdout
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Bar
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  Scenario: A parse error is reported on stderr and exits 1
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/bad/bad.feature" with:
      """
      This is not gherkin
      """
    When  the user runs `givn lint`
    Then  the command exits 1
    And  stderr contains "[ERROR]"
    And  stderr contains "bad.feature"
    And  stderr contains "givn lint: 1 file(s) checked, 1 finding(s)"

  Scenario: A @wip-tagged scenario is reported on stderr and exits 2
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @wip
        Scenario: Not done yet
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 2
    And  stderr contains "[WIP  ]"
    And  stderr contains "scenario 'Not done yet' is tagged @wip"
    And  stderr contains "givn lint: 1 file(s) checked, 1 finding(s)"

  Scenario: A PENDING step is reported on stderr and exits 2
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Bar
          Given PENDING
      """
    When  the user runs `givn lint`
    Then  the command exits 2
    And  stderr contains "[WIP  ]"
    And  stderr contains "has a PENDING step"

  Scenario: Findings across multiple files are aggregated and files are processed in sorted order
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/zzz/zzz.feature" with:
      """
      Feature: Zzz
        @wip
        Scenario: Z
          Given x
      """
    And  a .feature file at "givn/specs/aaa/aaa.feature" with:
      """
      Feature: Aaa
        @wip
        Scenario: A
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 2
    And  stderr contains "givn lint: 2 file(s) checked, 2 finding(s)"
    And  the aaa finding appears before the zzz finding on stderr

  Scenario: --change scopes the check to that change's specs directory
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @wip
        Scenario: Ignored
          Given x
      """
    And  a .feature file at "givn/changes/some-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: S
          Given x
      """
    When  the user runs `givn lint --change some-change`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  Scenario: --change with no specs directory reports no files and exits 0
    Given  a project root with givn/ initialised
    And  an existing change named "empty-change"
    When  the user runs `givn lint --change empty-change`
    Then  the command exits 0
    And  stdout contains a line "givn lint: no .feature files found under givn/changes/empty-change/specs/"

  Scenario: A non-existent change id is not surfaced
    Given  a project root with givn/ initialised
    When  the user runs `givn lint --change does-not-exist`
    Then  the command exits 1
    And  stderr contains "does-not-exist"

  Scenario: Explicit paths lint exactly those files without recursive descent
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @wip
        Scenario: Ignored
          Given x
      """
    And  a .feature file at "givn/specs/target.feature" with:
      """
      Feature: Target
        Scenario: S
          Given x
      """
    When  the user runs `givn lint givn/specs/target.feature`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  Scenario: A parse error outranks a @wip finding
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/aaa/aaa.feature" with:
      """
      Feature: Aaa
        @wip
        Scenario: W
          Given x
      """
    And  a .feature file at "givn/specs/zzz/zzz.feature" with:
      """
      This is not gherkin
      """
    When  the user runs `givn lint`
    Then  the command exits 1
    And  stderr contains "[WIP  ]"
    And  stderr contains "[ERROR]"
    And  stderr contains "givn lint: 2 file(s) checked, 2 finding(s)"
  Scenario: A removed placeholder suppresses the @wip warning on itself
    Given  a project root with givn/ initialised
    And  a feature file with a removed-wip placeholder at "givn/specs/foo/foo.feature"
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  Scenario: A removed scenario does not suppress an unrelated later @wip scenario
    Given  a project root with givn/ initialised
    And  a feature file with a removed scenario and a separate wip scenario at "givn/specs/foo/foo.feature"
    When  the user runs `givn lint`
    Then  the command exits 2
    And  stderr contains "[WIP  ]"
    And  stderr contains "scenario 'NewNotDone' is tagged @wip"
  Scenario: The lint exit-code contract is preserved after refactor
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/clean/clean.feature" with:
      """
      Feature: Clean
        Scenario: S
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"
