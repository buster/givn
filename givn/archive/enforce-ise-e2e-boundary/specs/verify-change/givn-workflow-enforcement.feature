@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.added
  Scenario Outline: An ise-effective change whose declared e2e role resolves no readable package fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose e2e role "<state>"
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the declared e2e package and "<condition>"
    And no change verification receipt is written

    Examples:
      | state                                    | condition                    |
      | matches no project path                  | matches no path              |
      | resolves a package with a malformed manifest | cannot be read           |
      | lists no path                            | the 'e2e' role lists no path |

  @givn.added
  Scenario: An ise-effective change with no behavior change still needs a readable e2e declaration
    Given an ise-effective change with no behavior change and no readable e2e declaration
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the absent e2e declaration
    When the developer runs `givn check review --change ise-boundary`
    Then the command exits non-zero
    And stderr names the absent e2e declaration

  @givn.added
  Scenario Outline: An ise-effective change whose e2e package reaches a source package fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose e2e package reaches a source package through a "<edge>" dependency
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the e2e package and the dependency edge into "source"
    And no change verification receipt is written

    Examples:
      | edge                |
      | direct              |
      | transitive          |
      | workspace-inherited |
      | target-specific     |

  @givn.added
  Scenario: An ise-effective change whose e2e path sits under a source root fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose e2e path sits under a source root
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the e2e path and the source root
    And no change verification receipt is written

  @givn.added
  Scenario: An ise-effective change whose e2e package contains a source root fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose e2e package contains a source root
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the e2e package and the source root
    And no change verification receipt is written

  @givn.added
  Scenario: An ise-effective change whose e2e path resolves no package and imports a source package fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose e2e path resolves no package and imports a package under a source root
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the e2e path and the imported source package
    And no change verification receipt is written

  @givn.added
  Scenario Outline: An ise-effective change whose E2E runner command names no declared e2e root fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose E2E runner command is "<command>"
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the declared e2e root and the E2E runner command
    And no change verification receipt is written

    Examples:
      | command                           |
      | ./run-tests.sh --bin other-target |
      | (empty)                           |
      | givn missing-e2e-runner           |

  @givn.added
  Scenario: An ise-effective change whose E2E run leaves a step unbound fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose delta has one E2E scenario with an unbound step
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the failing E2E run and reports 1 failed test
    And no change verification receipt is written

  @givn.added
  Scenario: An ise-effective change whose E2E run reports a skipped step fails the tasks check
    Given an ise-effective change ready for change-scoped verification whose delta has one E2E scenario with a skipped step
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names the failing E2E run and reports 1 skipped test
    And no change verification receipt is written

  @givn.added
  Scenario: An ise-effective change receipt binds the declared e2e package, the E2E runner command, and the Givn version
    Given an ise-effective change ready for change-scoped verification with a passing E2E run over two declared e2e packages
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And the change verification receipt records a digest of every resolved e2e package
    And the change verification receipt records the E2E runner command digest
    And the change verification receipt records the running Givn version
    And the change verification receipt records a passed E2E scope

  @givn.added
  Scenario: An ise-effective change whose delta has no E2E scenario records the E2E scope as not applicable
    Given an ise-effective change ready for change-scoped verification with no E2E scenario in its delta
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And the change verification receipt records a digest of every resolved e2e package
    And the change verification receipt records the E2E scope as not applicable

  @givn.added
  Scenario Outline: An ise-effective change receipt whose E2E identity no longer matches the project is stale for review
    Given an ise-effective change with a current change verification receipt
    And the receipt's E2E identity no longer matches the project because <input>
    When the developer runs `givn check review --change ise-boundary`
    Then the command exits non-zero
    And stderr says the change verification receipt is stale
    And stderr reports the stale reason "<reason>"

    Examples:
      | input                          | reason                          |
      | the e2e package changed        | e2e package digest changed      |
      | the E2E runner command changed | E2E runner command changed      |
      | the Givn version changed       | Givn version changed            |
      | the receipt lacks the identity | receipt lacks the E2E identity  |
      | the declaration cannot be read | the E2E identity cannot be read |

  @givn.added
  Scenario Outline: An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review
    Given an ise-effective change with a current change verification receipt whose delta "<delta>"
    And the receipt's E2E scope records "<scope>"
    When the developer runs `givn check review --change ise-boundary`
    Then the command exits non-zero
    And stderr says the change verification receipt is stale
    And stderr reports the stale reason "<reason>"

    Examples:
      | delta               | scope          | reason                                        |
      | has an E2E scenario | not_applicable | the E2E scope is missing                      |
      | has no E2E scenario | passed         | the E2E scope is not applicable to this delta |

  @givn.added
  Scenario: A change without ise keeps today's receipt without an E2E identity
    Given a change ready for change-scoped verification with ise off
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And the change verification receipt contains no E2E identity and no E2E scope
