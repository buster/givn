Feature: Readability review gate

  @e2e
  Scenario: A developer runs the review with accounted readability findings and it passes
    Given  a Givn project with the readability experiment enabled
    And  a committed base with a god type of 620 code lines split across files in "src/engine.rs"
    And  a committed change directory for "test-change"
    And  the change shrinks the type to 480 code lines and adds a cohesive type of 650 code lines split across files in "src/cohesive.rs"
    And  the change has complete step files, design, tasks, README, and routing preconditions
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: JUSTIFIED type:src/cohesive.rs::Cohesive"
    And  a rationale for "type:src/cohesive.rs::Cohesive" has every required section
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

  Scenario: A new type over the readable size with no decision fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a god type of 620 code lines split across files in "src/engine.rs" at head
    And  the change's review record exists
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/engine.rs::Engine"
    And  the review says "readability review: FAIL (1 finding without a decision)"
    And  the review says "add 'READABILITY: FIXED type:src/engine.rs::Engine' or 'READABILITY: JUSTIFIED type:src/engine.rs::Engine'"
    And  the review says "givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md"

  Scenario: A new module over the readable size with no decision fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no module over the readable size
    And  the change adds a module of 1200 code lines in "src/history.rs" at head
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING module:src/history.rs"
    And  the review says "new module: 1200 code lines; classification module"

  Scenario: A fixed marker for a unit still over its threshold fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change keeps the type at 620 code lines at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 620 code lines (base 620)"

  Scenario: A fixed marker for a unit under its threshold passes
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change shrinks the type to 480 code lines at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "FIXED type:src/engine.rs::Engine"

  Scenario: A justified marker with a complete rationale passes
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "JUSTIFIED type:src/engine.rs::Engine"

  Scenario: A concrete rationale with a named alternative passes
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a god type of 620 code lines split across files in "src/engine.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" names the rejected alternative "Extract the renderer" and cites "620 code lines" in its evidence
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "JUSTIFIED type:src/engine.rs::Engine"

  Scenario: A justified marker without a rationale fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "JUSTIFIED marker for type:src/engine.rs::Engine has no rationale at givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md"

  Scenario: A justified marker for a unit under its threshold fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change shrinks the type to 480 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "JUSTIFIED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is not a finding at head; use READABILITY: FIXED type:src/engine.rs::Engine"

  Scenario: A rationale whose unit does not match the finding fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" names the unit "type:src/other.rs::Other"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "Unit: type:src/other.rs::Other does not match the finding type:src/engine.rs::Engine"

  Scenario: Two units with the same rationale slug fail
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a god type "C" of 620 code lines split across "src/a_b.rs" and "src/a_b/parts.rs" at head
    And  the change adds a god type "C" of 620 code lines split across "src/a-b.rs" and "src/a-b/parts.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/a_b.rs::C"
    And  the change's review record records "READABILITY: JUSTIFIED type:src/a-b.rs::C"
    And  a rationale for "type:src/a_b.rs::C" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "rationale slug collision for type:src/a-b.rs::C and type:src/a_b.rs::C"

  Scenario: A rationale whose classification does not match the finding fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" records the classification "cohesive-large-type"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "Classification: cohesive-large-type does not match god-type"

  Scenario: A rationale without an owner fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" omits the owner
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "missing Owner:"

  Scenario: A rationale from an unconfirmed owner fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" records "someone-else" as its owner
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "Owner: 'someone-else' is not a confirmed owner; use givn-maintainer--owner"

  Scenario: A justification for "not my code" is rejected
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" says "not my code" as its reason
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "banned reason 'not my code'"

  Scenario: A justification for "out of scope" is rejected
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" says "out of scope" as its reason
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "banned reason 'out of scope'"

  Scenario: A boilerplate rationale is rejected
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" records "none" as its alternative
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "Alternatives considered: must name at least one alternative and why it was rejected"

  Scenario: A type rationale does not account for the module finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no module over the readable size
    And  the change adds a god type of 620 code lines in one file "src/engine.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING module:src/engine.rs"

  Scenario: Two types in one file need two rationales
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds two god types in one file "src/engine.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::First"
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Second"
    And  a rationale for "type:src/engine.rs::First" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/engine.rs::Second"

  Scenario: A fixed finding without a fixed marker fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change shrinks the type to 480 code lines at head
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/engine.rs::Engine"

  Scenario: An existing non-growing finding without a decision fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change touches the type without growing it
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/engine.rs::Engine"
    And  the review says "existing finding did not grow: 620 -> 620 code lines; classification god-type"

  Scenario: An existing growing finding passes with a justified marker
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review says "existing finding grew: 620 -> 700 code lines; classification god-type"

  Scenario: A new cohesive large type passes with a justified marker
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a cohesive type of 600 code lines split across files in "src/cohesive.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/cohesive.rs::Cohesive"
    And  a rationale for "type:src/cohesive.rs::Cohesive" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review says "classification cohesive-large-type"

  Scenario: An existing data-only type is exempt from the size gate
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a data-only enum of 620 code lines in one file "src/status.rs"
    And  the change touches the enum without growing it
    And  the change's review record records "READABILITY: JUSTIFIED module:src/status.rs"
    And  a rationale for "module:src/status.rs" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports no finding for "type:src/status.rs::Status"

  Scenario: A data-only type with a long method is a finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a data-only enum of 300 code lines in one file "src/status.rs"
    And  the enum has a method of 120 code lines
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/status.rs::Status"
    And  the review says "long method: 120 code lines"

  Scenario: A new interface under the readable size with more than 50 public methods is a finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 420 code lines with 51 public methods in "src/api.rs"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/api.rs::Api"
    And  the review says "classification interface"

  Scenario: A module of exactly 1000 code lines is a finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a module of exactly 1000 code lines in "src/history.rs"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING module:src/history.rs"
    And  the review says "new module: 1000 code lines; classification module"

  Scenario: A type at exactly 500 code lines is not a gate finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a type of exactly 500 code lines with cohesive methods split across "src/engine.rs" and "src/engine/parts.rs" at head
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports no finding for "type:src/engine.rs::Engine"

  Scenario: A long method on a type within the size limit is a finding
    Given  a Givn project with the readability experiment enabled
    And  a committed change that adds a type of 300 code lines in "src/engine.rs"
    And  the type has a method of 120 code lines
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "MISSING type:src/engine.rs::Engine"
    And  the review says "long method: 120 code lines"

  Scenario: The review requires a readability verdict when a finding exists
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "readability review: FAIL (missing 'READABILITY: PASS')"

  Scenario: A fail marker fails the review even when every finding is justified
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    And  the change's review record records "FAIL the type has no single responsibility"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "the type has no single responsibility"

  Scenario: A missing review record with a finding fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a god type of 620 code lines split across files in "src/engine.rs" at head
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "readability record is missing; create givn/changes/test-change/review.md and record 'READABILITY: PASS'"

  Scenario: A touched production file that cannot be parsed fails closed
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change writes malformed Rust at head in "src/engine.rs"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "cannot analyze src/engine.rs at head"

  Scenario: A changed Rust file the analyzer skips fails closed
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a Rust file in the analyzer-excluded test directory "src/tests/support.rs" at head
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "changed production file 'src/tests/support.rs' was not analyzed; the analyzer scope or extension filter is wrong"

  Scenario: A temporary rationale without a follow-up fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a temporary rationale for "type:src/engine.rs::Engine" omits "Follow-up:"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "missing Follow-up:"

  Scenario: A temporary rationale with a follow-up and expiry passes
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" carries "Follow-up:" and "Expiry:"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "JUSTIFIED type:src/engine.rs::Engine"

  Scenario: Conflicting decisions for one unit fail
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "conflicting decisions for type:src/engine.rs::Engine"

  Scenario: An unknown readability marker fails
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    And  the change's review record records "READABILITY: DEFERRED type:src/engine.rs::Engine"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "readability review: FAIL (unknown readability marker: READABILITY: DEFERRED type:src/engine.rs::Engine)"

  Scenario: Several failures are reported together
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines and adds a god type of 650 code lines split across "src/cohesive.rs" and "src/cohesive/parts.rs" at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review fails
    And  the review reports "FAILURE  invalid-fixed  FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 700 code lines (base 620)"
    And  the review reports "FAILURE  unaccounted  MISSING type:src/cohesive.rs::Cohesive"
    And  the review says "readability review: FAIL (2 failures; first: FIXED marker for type:src/engine.rs::Engine, but type:src/engine.rs::Engine is still a finding: god-type 700 code lines (base 620))"

  Scenario: A symlink in place of a touched production file fails closed
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a type in "src/engine.rs"
    And  the change replaces "src/engine.rs" with a symlink at head
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "cannot analyze src/engine.rs at head: symlink"

  Scenario: A base with no production sources makes every head unit new
    Given  a Givn project with the readability experiment enabled
    And  a committed change directory with no production sources at its parent
    And  the change adds a god type of 620 code lines split across files in "src/engine.rs" at head
    And  the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And  a rationale for "type:src/engine.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review says "new type: 620 code lines"

  Scenario: A head with no production sources passes with no findings
    Given  a Givn project with the readability experiment enabled
    And  a committed change directory with no production sources at base or head
    When  the readability review assesses the change
    Then  the review passes
    And  the review says "touched 0 unit(s), 0 finding(s)"
    And  the review says "readability review: PASS"

  Scenario: A production commit before the change directory is part of the base
    Given  a Givn project with the readability experiment enabled
    And  a commit that grows a type to 620 code lines before the change directory exists
    And  the first change commit adds the change directory and grows the type to 700 code lines
    When  the readability review assesses the change
    Then  the review reports the base size 620
    And  the review reports the head size 700

  Scenario: A root commit whose change directory is the first commit has an empty base
    Given  a Givn project with the readability experiment enabled
    And  the change directory and a god type of 620 code lines are added in the root commit
    When  the readability review assesses the change
    Then  the review says "base none (root change commit; empty base tree)"
    And  the review reports "MISSING type:src/engine.rs::Engine"

  Scenario: A merge commit uses its first parent as the change base
    Given  a Givn project with the readability experiment enabled
    And  a change directory commit on the branch grows a type from 450 to 620 code lines
    And  the branch is merged from main with a merge commit
    When  the readability review assesses the change
    Then  the review reports the base size 450
    And  the review reports the head size 620

  Scenario: A rebase recomputes the change base
    Given  a Givn project with the readability experiment enabled
    And  a change directory commit grows a type from 450 to 620 code lines
    And  the branch is rebased onto a commit that grows the type to 500 code lines
    When  the readability review assesses the change
    Then  the review reports the base size 500
    And  the review reports the head size 620

  Scenario: A change directory without a reachable commit fails closed
    Given  a Givn project with the readability experiment enabled
    And  a change directory "givn/changes/test-change/" that no commit touched
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "change directory 'givn/changes/test-change/' has no reachable commit; commit the change directory before review"

  Scenario: A shallow clone fails closed
    Given  a Givn project with the readability experiment enabled
    And  a shallow clone whose first change commit is beyond the fetched history
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "cannot resolve change base: repository is shallow; fetch the full history before review"

  Scenario: A sibling Fossil timeline is not the change base
    Given  a Fossil project with the readability experiment enabled
    And  a sibling timeline that touches the change directory
    And  an ancestor commit that does not touch the change directory
    When  the readability review assesses the change
    Then  the review uses the ancestor commit as the first touch
    And  the review ignores the sibling timeline

  Scenario: A Fossil merge check-in uses its parent field as the change base
    Given  a Fossil project with the readability experiment enabled
    And  a change directory check-in on the branch grows a type from 450 to 620 code lines
    And  a merged-in check-in grows the type to 700 code lines and is named by "merged-from:"
    When  the readability review assesses the change
    Then  the review reports the base size 450
    And  the review reports the head size 620

  Scenario: Dirty production paths fail closed before analysis
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the working tree has an uncommitted change in "src/engine.rs"
    When  the readability review assesses the change
    Then  the review fails
    And  the review says "production paths are dirty; commit or discard before review: src/engine.rs"

  Scenario: A file move closes the old unit and needs a decision for the new unit
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change moves the type and its impl to "src/core.rs" and "src/core/parts.rs" without changing its name
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: JUSTIFIED type:src/core.rs::Engine"
    And  a rationale for "type:src/core.rs::Engine" has every required section
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "FIXED type:src/engine.rs::Engine"
    And  the review reports "JUSTIFIED type:src/core.rs::Engine"

  Scenario: Deleting a finding requires a fixed marker
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change deletes the type at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "FIXED type:src/engine.rs::Engine"

  Scenario: Deleting a production file needs a decision for its removed unit
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change deletes "src/engine.rs" and "src/engine/parts.rs" at head
    And  the change's review record records "READABILITY: FIXED type:src/engine.rs::Engine"
    And  the change's review record records "READABILITY: PASS"
    When  the readability review assesses the change
    Then  the review passes
    And  the review reports "removed: base 620 code lines; finding no longer fires"

  Scenario: The readability review is deterministic
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And  the change grows the type to 700 code lines at head
    When  the readability review assesses the change twice
    Then  both results are identical

  Scenario: The readability review stores no classification baseline
    Given  a Givn project with the readability experiment enabled
    And  a committed change whose base has no type over the readable size
    And  the change adds a god type of 620 code lines split across files in "src/engine.rs" at head
    When  the readability review assesses the change
    Then  the review writes no readability baseline file

  Scenario: The review guidance carries the fixed readability vocabulary
    Given  a Givn project with the readability experiment enabled
    When  the reviewer reads the review guidance
    Then  the guidance names "READABILITY: FIXED"
    And  the guidance names "READABILITY: JUSTIFIED"
    And  the guidance names "READABILITY: PASS"
    And  the guidance names "FAIL <reason>"
    And  the guidance says a justification cannot use ownership or scope as a reason
    And  the guidance says to run "givn check review"
