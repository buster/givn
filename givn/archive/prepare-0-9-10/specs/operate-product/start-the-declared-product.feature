@givn.delta @start-the-declared-product
Feature: Bounded declared product readiness

  @givn.modified @e2e
  Scenario: The started server reports readiness and forwards a clean exit
    Given a server declaration with a readiness deadline whose server becomes ready before that deadline and exits cleanly
    When the consumer runs `givn run`
    Then the command exits 0
    And stdout reports readiness at the declared port and readiness path
    And the reported ready outcome follows a successful response through the declared boundary
    And the terminal transcript is captured for this scenario

  @givn.modified
  Scenario: A readiness timeout abandons the start
    Given a server declaration whose readiness path never succeeds within its declared deadline
    When the user runs `givn run`
    Then the command exits 2 within the readiness deadline plus bounded process cleanup time
    And the output names the readiness timeout without reporting ready
    And the started product is stopped and reaped

  @givn.modified
  Scenario: The readiness probe gives up after the timeout
    Given a readiness endpoint whose failing responses consume almost all the declared readiness time
    When the readiness deadline elapses
    Then no additional connection attempt receives a fresh full timeout
    And the probe reports a timeout within the original total deadline

  @givn.added
  Scenario Outline: A stalled readiness exchange cannot outlive the total deadline
    Given a server declaration with a finite readiness deadline
    And the readiness endpoint "<stall>"
    When the consumer runs `givn run`
    Then the command exits 2 within that deadline plus bounded process cleanup time
    And no ready line is printed
    And the started product is stopped and reaped

    Examples:
      | stall                                                     |
      | does not complete a connection                            |
      | accepts a connection but does not read the request         |
      | reads the request but never responds                      |
      | sends a partial response and leaves the connection open   |

  @givn.added
  Scenario: Readiness retries share one deadline across connection and exchange
    Given a readiness endpoint that delays connection, request consumption, and response across repeated attempts
    When the consumer starts the product with a declared readiness timeout
    Then the combined connection, write, read, and retry waits consume no more than the total readiness deadline
    And an expired deadline triggers the existing product cleanup rather than another full probe wait
