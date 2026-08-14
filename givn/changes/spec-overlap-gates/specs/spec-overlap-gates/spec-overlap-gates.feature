# User Interaction Inventory:
# - Run `givn lint` across the whole active tree (default scope)
# - Run `givn lint --json` for machine-readable findings

@givn.delta @spec-overlap-gates

Feature: spec-overlap-gates

  @givn.added @wip @e2e
  Scenario: A duplicate title across two permanent spec files is a hard error
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/alpha/alpha.feature" with:
      """
      Feature: Alpha
        Scenario: The same behaviour twice
          Given x
          When y
          Then z
      """
    And  a .feature file at "givn/specs/beta/beta.feature" with:
      """
      Feature: Beta
        Scenario: The same behaviour twice
          Given a
          When b
          Then c
      """
    When  the user runs `givn lint`
    Then  the command exits 3
    And  stderr contains "duplicate scenario title"
    And  stderr contains "alpha.feature"
    And  stderr contains "beta.feature"
    And  stderr contains "The same behaviour twice"

  @givn.added
  Scenario: An added delta duplicating a permanent spec title is a hard error
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/alpha/alpha.feature" with:
      """
      Feature: Alpha
        Scenario: Shared title
          Given x
      """
    And  a .feature file at "givn/changes/own/specs/beta/beta.feature" with:
      """
      Feature: Beta
        Scenario: Shared title
          Given a
      """
    When  the user runs `givn lint`
    Then  the command exits 3
    And  stderr contains "Shared title"

  @givn.added
  Scenario: A delta of another active change duplicating a permanent spec title is a hard error
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/alpha/alpha.feature" with:
      """
      Feature: Alpha
        Scenario: Borrowed title
          Given x
      """
    And  a .feature file at "givn/changes/other/specs/gamma/gamma.feature" with:
      """
      Feature: Gamma
        Scenario: Borrowed title
          Given a
      """
    When  the user runs `givn lint`
    Then  the command exits 3
    And  stderr contains "Borrowed title"
    And  stderr contains "gamma.feature"

  @givn.added
  Scenario: Deltas of two active changes sharing a title are a hard error
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/changes/one/specs/a/a.feature" with:
      """
      Feature: A
        Scenario: Contested title
          Given x
      """
    And  a .feature file at "givn/changes/two/specs/b/b.feature" with:
      """
      Feature: B
        Scenario: Contested title
          Given a
      """
    When  the user runs `givn lint`
    Then  the command exits 3
    And  stderr contains "Contested title"
    And  stderr contains "a.feature"
    And  stderr contains "b.feature"

  @givn.added
  Scenario: A modified delta matching its merge target in the same capability is allowed
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Being modified
          Given x
      """
    And  a .feature file at "givn/changes/one/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @givn.modified
        Scenario: Being modified
          Given x
          When y
      """
    When  the user runs `givn lint`
    Then  the command exits 0

  @givn.added
  Scenario: A modified delta whose title exists in a different capability is a hard error
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Wrong home
          Given x
      """
    And  a .feature file at "givn/changes/one/specs/bar/bar.feature" with:
      """
      Feature: Bar
        @givn.modified
        Scenario: Wrong home
          Given a
      """
    When  the user runs `givn lint`
    Then  the command exits 3
    And  stderr contains "Wrong home"

  @givn.added
  Scenario: A removed delta referencing its target by title is allowed
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Going away
          Given x
      """
    And  a .feature file at "givn/changes/one/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @givn.removed
        Scenario: Going away
          Given removed placeholder
      """
    When  the user runs `givn lint`
    Then  the command exits 0

  @givn.added
  Scenario: Two scenarios with the same shape but different titles produce a warning
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: First sighting
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          And stdout contains "clean"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Second sighting
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          And stdout contains "tidy"
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "First sighting"
    And  stderr contains "Second sighting"
    And  stderr contains "merging"

  @givn.added
  Scenario: The same shape with contradicting facts produces a related-scenario hint
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Exits cleanly
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          And stdout contains "clean"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Exits with a finding
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 2
          And stdout contains "clean"
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "related_scenario"
    And  stderr contains "Exits cleanly"
    And  stderr contains "Exits with a finding"

  @givn.added
  Scenario: A scenario whose assertion sequence is a subset of another produces a subset warning
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: The full check
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          And stdout contains "clean"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: The partial check
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "subset"
    And  stderr contains "The partial check"

  @givn.added
  Scenario: A scenario with more than 14 steps produces a long-scenario warning
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: The marathon
          Given a project root with givn/ initialised
          And step one
          And step two
          And step three
          And step four
          And step five
          And step six
          And step seven
          And step eight
          And step nine
          And step ten
          And step eleven
          And step twelve
          When the user runs `givn lint`
          Then the command exits 0
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "long scenario"
    And  stderr contains "The marathon"

  @givn.added
  Scenario: The default scope excludes the archive tree
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Current
          Given x
      """
    And  a .feature file at "givn/archive/old/old.feature" with:
      """
      This is not gherkin
      """
    When  the user runs `givn lint`
    Then  the command exits 0

  @givn.added
  Scenario: An explicit archive path still lints its file
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/archive/old/old.feature" with:
      """
      Feature: Old
        @wip
        Scenario: Past
          Given x
      """
    When  the user runs `givn lint givn/archive/old/old.feature`
    Then  the command exits 2
    And  stderr contains "scenario 'Past' is tagged @wip"

  @givn.added @wip @e2e
  Scenario: The json output reports duplicate-title findings machine-readably
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/alpha/alpha.feature" with:
      """
      Feature: Alpha
        Scenario: Twin title
          Given x
      """
    And  a .feature file at "givn/specs/beta/beta.feature" with:
      """
      Feature: Beta
        Scenario: Twin title
          Given a
      """
    When  the user runs `givn lint --json`
    Then  the command exits 3
    And  stdout is valid JSON
    And  stdout contains "\"kind\": \"duplicate_title\""
    And  stdout contains "\"titles\""
    And  stdout contains "Twin title"

  @givn.added @wip
  Scenario: The json output reports a parse error as a machine-readable finding
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/bad/bad.feature" with:
      """
      This is not gherkin
      """
    When  the user runs `givn lint --json`
    Then  the command exits 1
    And  stdout is valid JSON
    And  stdout contains "\"kind\": \"parse_error\""
    And  stdout contains "bad.feature"

  @givn.added @wip
  Scenario: Consecutive And and But steps normalise to the same shape as plain Then steps
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Written with conjunctions
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          And stdout contains "clean"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Written with plain steps
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
          Then stdout contains "clean"
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "Written with conjunctions"
    And  stderr contains "Written with plain steps"

  @givn.added @wip
  Scenario: Quoted strings are masked in the shape comparison
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Looking for alpha
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then stdout contains "alpha"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Looking for beta
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then stdout contains "beta"
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "Looking for alpha"
    And  stderr contains "Looking for beta"

  @givn.added @wip
  Scenario: Numbers are masked in the shape comparison but exit codes stay facts
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Exit zero
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Exit two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 2
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "related_scenario"
    And  stderr contains "Exit zero"
    And  stderr contains "Exit two"

  @givn.added @wip
  Scenario: Scenario outlines sharing a template produce a shape-match warning
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario Outline: Paying with <method>
          Given a project root with givn/ initialised
          When the user pays with <method>
          Then the payment succeeds

          Examples:
            | method |
            | card   |

        Scenario Outline: Paying again with <method>
          Given a project root with givn/ initialised
          When the user pays with <method>
          Then the payment succeeds

          Examples:
            | method |
            | cash   |
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "Paying with"
    And  stderr contains "Paying again with"

  @givn.added @wip
  Scenario: Docstrings are ignored in the shape comparison
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: With a docstring
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the output matches
            \"\"\"
            one docstring
            \"\"\"
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: With another docstring
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the output matches
            \"\"\"
            a different docstring
            \"\"\"
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "With a docstring"
    And  stderr contains "With another docstring"

  @givn.added @wip
  Scenario: Background steps are prepended before the comparison
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Background:
          Given a project root with givn/ initialised

        Scenario: With background context
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Without background context
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "With background context"
    And  stderr contains "Without background context"

  @givn.added @wip
  Scenario: Data table contents are compared as facts and can contradict
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Permission for admin
          Given a project root with givn/ initialised
          When the user checks the role
          Then the decision is recorded
            | role  | allowed |
            | admin | yes     |
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Permission for guest
          Given a project root with givn/ initialised
          When the user checks the role
          Then the decision is recorded
            | role  | allowed |
            | guest | no      |
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "related_scenario"
    And  stderr contains "Permission for admin"
    And  stderr contains "Permission for guest"

  @givn.added @wip
  Scenario: The same shape without any extractable facts suggests merging
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Plain steps one
          Given a blank slate
          When something happens
          Then nothing is asserted
      """
    And  a .feature file at "givn/specs/bar/bar.feature" with:
      """
      Feature: Bar
        Scenario: Plain steps two
          Given a blank slate
          When something happens
          Then nothing is asserted
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stderr contains "merging"
    And  stderr contains "Plain steps one"
    And  stderr contains "Plain steps two"
