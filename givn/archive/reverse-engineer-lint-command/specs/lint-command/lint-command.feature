@givn.delta @lint-command
Feature: lint-command

  The black-box boundary for this capability is: the `givn lint` CLI command,
  invoked as a subprocess. Observable outputs are stdout, stderr, and the
  process exit code. The command statically checks `.feature` files for parse
  errors, `@wip`-tagged scenarios, and `PENDING` steps. It runs NO tests.

  Exit code contract (as observed in the code today):
    0 = clean (no findings)
    1 = parse error(s) only
    2 = any @wip or PENDING finding present

  Suspected bugs flagged for follow-up `@givn.modified` candidates — NOT
  addressed in this change; the scenarios below capture the AS-IS behaviour:

  1. Severity ranking is inverted versus the source doc comment. The doc
     comment on `run()` states "Parse errors outrank wip/PENDING", but the
     code assigns `ParseError -> 1u8` and `Wip -> 2u8` and takes the max, so
     a @wip finding forces exit code 2 even when a parse error is also
     present across files. (See "A @wip finding outranks a parse error".)

  2. `@givn.removed` @wip suppression is over-broad. The check is
     `s.position.line <= wip_line` against ANY removed scenario in the
     feature, so a removed scenario near the top of the file suppresses an
     unrelated `@wip` scenario anywhere later in the same file. (See
     "A @givn.removed scenario suppresses an unrelated later @wip scenario".)

  3. `--change <id>` with a non-existent change id silently reports
     "no .feature files found" and exits 0, giving no signal that the change
     itself does not exist. (See "A non-existent change id is not surfaced".)

  4. The source comment in `run()` claims "actual process exit is handled by
     mapping to a non-zero return in main", but `lint` calls
     `std::process::exit(1)` and `std::process::exit(2)` directly, bypassing
     main's anyhow error handler. (Architectural note — no scenario; the
     observable exit codes are documented above.)

  @givn.added
  Scenario: No .feature files under givn/ reports clean and exits 0
    Given  a project root with givn/ initialised
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: no .feature files found under givn/"

  @givn.added
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

  @givn.added
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

  @givn.added
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

  @givn.added
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

  @givn.added
  Scenario: A @wip finding outranks a parse error
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
    Then  the command exits 2
    And  stderr contains "[WIP  ]"
    And  stderr contains "[ERROR]"
    And  stderr contains "givn lint: 2 file(s) checked, 2 finding(s)"

  @givn.added
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

  @givn.added
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

  @givn.added
  Scenario: --change with no specs directory reports no files and exits 0
    Given  a project root with givn/ initialised
    And  an existing change named "empty-change"
    When  the user runs `givn lint --change empty-change`
    Then  the command exits 0
    And  stdout contains a line "givn lint: no .feature files found under givn/changes/empty-change/specs/"

  @givn.added
  Scenario: A non-existent change id is not surfaced
    Given  a project root with givn/ initialised
    When  the user runs `givn lint --change does-not-exist`
    Then  the command exits 0
    And  stdout contains a line "givn lint: no .feature files found under givn/changes/does-not-exist/specs/"

  @givn.added
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

  @givn.added
  Scenario: A @givn.removed placeholder suppresses the @wip warning on itself
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      @givn.delta @foo
      Feature: Foo

        @givn.removed @wip
        Scenario: Old
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  @givn.added
  Scenario: A @givn.removed scenario suppresses an unrelated later @wip scenario
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      @givn.delta @foo
      Feature: Foo

        @givn.removed
        Scenario: Old
          Given x

        @wip
        Scenario: NewNotDone
          Given y
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"
