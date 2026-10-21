@givn.delta @outline-report

Feature: Outline report

  @givn.added
  Scenario: A method call on an unknown receiver is a name-only guess
    Given a production source unit whose method calls a receiver of unknown type
    When the developer runs "givn outline report --json"
    Then the JSON report records the call "is_empty" as "name_only"
    And the JSON call names "tier::TierReasoning::is_empty" as its candidate target

  @givn.added
  Scenario: A name-only guess is not expanded as a same-unit call
    Given a production source unit whose method calls a receiver of unknown type
    When the developer runs "givn outline report"
    Then the report prints the call "is_empty" as "-> [name_only, public, TierReasoning] is_empty"
    And the name-only call "is_empty" is not expanded into a deeper-indented call line
    And the report records "LCOM4 2" for the unit "type:src/tier.rs::TierReasoning"

  @givn.added
  Scenario: A call on the unit's own receiver resolves within the unit
    Given a production source unit whose method calls another method on itself
    When the developer runs "givn outline report --json"
    Then the JSON report records the call "validate" as "same_file"
    And the JSON call names "engine::Engine::validate" as its target
    And the JSON report records the call "check" as "same_file"
    And the JSON call names "engine::Engine::check" as its target

  @givn.added
  Scenario: A method call on a named receiver resolves to that owner
    Given a production source unit whose method calls an associated function on a project type
    When the developer runs "givn outline report --json"
    Then the JSON report records the call "from" as "same_file"
    And the JSON call names "line::Line::from" as its target
    And the JSON report records 2 methods for the unit "type:src/line.rs::Line"

  @givn.added @wip
  Scenario: An ambiguous method name has no resolved target
    Given a production source unit whose method calls an unknown receiver whose name two project types declare
    When the developer runs "givn outline report --json"
    Then the JSON report records the call "refresh" as "ambiguous"
    And the JSON call "refresh" names no target

  @givn.added @wip
  Scenario: An associated-function call whose qualifier contradicts the target is external
    Given a production source unit that calls "Line::from" while the project declares "Error::from"
    When the developer runs "givn outline report --json"
    Then the JSON report records the call "from" as "external"
    And the JSON call "from" names no target

  @givn.added @wip
  Scenario: A god type found through cohesion reports only the cohesion evidence
    Given a production source unit over the readable size whose methods are connected only by calls
    When the outline report assesses the unit
    Then the report classifies the unit as a god type
    And the report records the evidence "type-tcc < 500"
    And the report records no "type-lcom4" evidence

  @givn.added @wip
  Scenario: An interface found through its public methods reports only public-method evidence
    Given a production source unit with more than 50 public methods
    When the outline report assesses the unit
    Then the report classifies the unit as an interface
    And the report records the evidence "type-public-methods > 50"
    And the report records no "type-kind-trait" evidence

  @givn.added @wip
  Scenario: A long method found through complexity reports only complexity evidence
    Given a production source unit with a method over the complexity limit but within the length limit
    When the outline report assesses the unit
    Then the report classifies the unit as a method-length finding
    And the report records the evidence "method-complexity > 50"
    And the report records no "method-code-lines" evidence

  @givn.added @wip
  Scenario: A Rust module outline lists the types it declares
    Given a production Rust module with three declared types and one free function
    When the developer runs "givn outline report --json"
    Then the JSON module outline lists the types "First", "Second", and "Third"

  @givn.added @wip
  Scenario: A module outline interleaves member types and free functions in source order
    Given a production Rust module with a free function, a type, a free function, and a type in that order
    When the developer runs "givn outline report --json"
    Then the JSON module outline lists "alpha", "Beta", "gamma", and "Delta" in source order

  @givn.added @wip
  Scenario: A Python module outline lists its classes
    Given a Python module with two classes and one function
    When the developer runs "givn outline report --json"
    Then the JSON module outline lists the classes "OrderBook" and "Ledger"

  @givn.added @wip
  Scenario: The JSON report names each call's resolved target
    Given a production source unit whose method calls a module function
    When the developer runs "givn outline report --json"
    Then the JSON call "normalize" names the target "component::normalize"

  @givn.added @wip
  Scenario: A same-unit method implemented in another file is project_other_file
    Given a production source unit declared in one file and implemented in another
    When the developer runs "givn outline report"
    Then the report prints the call "validate" as "-> [project_other_file, public, Engine] validate"
    And the call "validate" is expanded into a deeper-indented call line
    And the report records "LCOM4 1" for the unit "type:src/engine_core.rs::Engine"

  @givn.added @wip
  Scenario: A developer reads each call's resolution in the human report
    Given a Givn project with the outline experiment enabled
    And a production source unit whose method calls its own method, a cross-file module function, and a receiver of unknown type
    When the developer runs "givn outline report"
    Then the report prints the call "validate" as "-> [same_file, public, Engine] validate"
    And the report prints the call "normalize" as "-> [project_other_file, public, module] normalize"
    And the report prints the call "is_empty" as "-> [name_only, public, Engine] is_empty"
    And the name-only call "is_empty" is not expanded into a deeper-indented call line

  @givn.added @wip
  Scenario: A file's code lines exclude inline test modules
    Given a production source file with an inline test module
    When the developer runs "givn outline report --json"
    Then the JSON report records the file "src/counted.rs" metric "code_lines" as 8
    And the JSON report records the file "src/counted.rs" metric "physical_lines" as 18

  @givn.added @wip
  Scenario: A unit's physical lines cover only its own spans
    Given a production source file with a small type and a large unrelated function
    When the developer runs "givn outline report --json"
    Then the JSON report records the unit "type:src/scoped.rs::Scoped" metric "physical_lines" as 8
    And the JSON report records the unit "type:src/scoped.rs::Scoped" metric "code_lines" as 8
    And the JSON report records the file "src/scoped.rs" metric "physical_lines" as 32

  @givn.added @wip
  Scenario: An impl-level test method is excluded from the unit's lines
    Given a production source unit with an impl-level test method
    When the developer runs "givn outline report --json"
    Then the JSON report records the unit "type:src/guarded.rs::Guarded" metric "code_lines" as 8
    And the JSON report records the unit "type:src/guarded.rs::Guarded" metric "physical_lines" as 9
    And the JSON report records 1 method for the unit "type:src/guarded.rs::Guarded"

  @givn.added @wip
  Scenario: A name-only guess does not connect methods for cohesion
    Given a production source unit over the readable size whose only call edges are name-only guesses
    When the developer runs "givn outline report"
    Then the report records "LCOM4 4" for the unit "type:src/cohesion.rs::Engine"
    And the report classifies the unit as a god type
    And the report records the evidence "type-lcom4 > 2"

  @givn.added @wip
  Scenario: A method signature keeps arrays, qualified paths, and its full type detail
    Given a production source unit with a method whose types use paths, generics, references, slices, tuples, arrays, pointers, and fully qualified paths
    When the developer runs "givn outline report --json"
    Then the JSON method signature contains "Result<Self, Error>"
    And the JSON method signature contains "&[u8]"
    And the JSON method signature contains "(u32, String)"
    And the JSON method signature contains "*const Config"
    And the JSON method signature contains "[u8; 4]"
    And the JSON method signature contains "crate::config::Config"
