Feature: Readability review gate

  @e2e
  Scenario: A developer runs the review with readable outlines and it passes
    Given  a Givn project with the readability experiment enabled
    And  a committed base with a god type of 620 code lines split across files in "src/engine.rs"
    And  a committed change directory for "test-change"
    And  the change shrinks the type to 480 code lines and adds a cohesive type of 650 code lines split across files in "src/cohesive.rs"
    And  the change has complete step files, design, tasks, README, and routing preconditions
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | builds an Engine; validate then compile"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/cohesive.rs::Cohesive | one cohesive responsibility; calls follow the data flow"
    And  the change's review record records "READABILITY: PASS"
    And  the change's verification receipt is current
    When  the developer runs "givn check review --change test-change"
    Then  the command exits 0
    And  stdout contains "readability review: PASS"

  Scenario: The readability review does not run when the experiment is disabled
    Given  a Givn project with the readability experiment disabled
    And  a committed change that adds a type over the readable size
    When  the developer runs "givn check review --change test-change"
    Then  the command exits 0
    And  stdout does not contain "readability review:"

  Scenario: A readable outline passes while a metric flags the code
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | one responsibility; the methods call in a clear order"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports an outline for "type:src/engine.rs::Engine"
    And  the review reports the outline check "PASS" for "type:src/engine.rs::Engine"
    And  the review reports a supporting metric finding for "type:src/engine.rs::Engine"

  Scenario: An unreadable outline fails while the metrics are clean
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: OUTLINE FAIL type:src/engine.rs::Engine | the call order is not explainable"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "outline review failed for type:src/engine.rs::Engine: the call order is not explainable"

  Scenario: A missing outline verdict fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "MISSING outline type:src/engine.rs::Engine"
    And  the review says "1 outline without a verdict"

  Scenario: A PASS without a responsibility and call-order explanation fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | "
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "outline PASS for type:src/engine.rs::Engine has no responsibility/call-order explanation"

  Scenario: A missing reviewer fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "missing 'READABILITY: REVIEWER <id>'"

  Scenario: A reviewer cannot be the author
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER agent"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "outline reviewer 'agent' must differ from the author 'agent'"

  Scenario: Conflicting outline verdicts fail
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the change's review record records "READABILITY: AUTHOR agent"
    And  the change's review record records "READABILITY: REVIEWER fresh-context-reviewer"
    And  the change's review record records "READABILITY: OUTLINE PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And  the change's review record records "READABILITY: OUTLINE FAIL type:src/engine.rs::Engine | the calls are not clear"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "conflicting outline verdicts for type:src/engine.rs::Engine"

  Scenario: A change with no production sources passes without outlines
    Given  a Givn project with the readability experiment enabled
    And  a committed change directory with no production sources at base or head
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes

  Scenario: Dirty production paths fail closed before analysis
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 40 code lines in "src/engine.rs"
    And  the working tree has an uncommitted change in "src/engine.rs"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "production paths are dirty; commit or discard before review: src/engine.rs"

  Scenario: A change directory without a reachable commit fails closed
    Given  a Givn project with the readability experiment enabled
    And  a change directory "givn/changes/test-change/" that no commit touched
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "has no reachable commit"

  Scenario: A shallow clone fails closed
    Given  a Givn project with the readability experiment enabled
    And  a shallow clone whose first change commit is beyond the fetched history
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "repository is shallow"

  Scenario: The review guidance carries the outline vocabulary
    Given  a Givn project with the readability experiment enabled
    When  the reviewer reads the review guidance
    Then  the guidance names "READABILITY: OUTLINE PASS"
    And  the guidance names "READABILITY: OUTLINE FAIL"
    And  the guidance names "READABILITY: AUTHOR"
    And  the guidance names "READABILITY: REVIEWER"
    And  the guidance names "READABILITY: PASS"
    And  the guidance names "no justification path for an unreadable outline"
    And  the guidance says to run "givn check review"
