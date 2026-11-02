Feature: Outline report

  @e2e @outline-report
  Scenario: A developer runs a outline report and sees the classified units
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report"
    Then  the command exits with code 0
    And  the report classifies the unit as a god type
    And  the report labels the finding as advisory rather than a review or archive verdict
    And  the report records no recommendation
    And  the report writes no baseline file
    And  the terminal transcript is captured for this scenario

  Scenario: The outline report is unavailable when the experiment is disabled
    Given  a Givn project with the outline experiment disabled
    When  the developer runs "givn outline report"
    Then  the command exits non-zero
    And  stderr contains "outline experiment is disabled"

  Scenario: A data-only type is not treated as a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a data-only enum over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as data-only
    And  the report labels the unit as a hint
    And  the report records no recommendation

  Scenario: An interface is classified as an interface, not a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a trait over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  Scenario: A cohesive type is not treated as a god type
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with a cohesive type over the readable size
    When  the developer runs "givn outline report"
    Then  the report classifies the unit as a cohesive-large-type
    And  the report records no recommendation

  Scenario: A type with many public methods is an interface
    Given  a production source unit with more than 50 public methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as an interface
    And  the report records no recommendation

  Scenario: A long method is a method-length finding
    Given  a production source unit with a long method
    When  the outline report assesses the unit
    Then  the report classifies the unit as a method-length finding
    And  the report records no recommendation

  Scenario: Test sources are not scanned
    Given  a Givn project with the outline experiment enabled
    And  a production source unit and a test source unit
    When  the developer runs "givn outline report"
    Then  the report does not name the test source

  Scenario: A type at the readable size limit is within limits
    Given  a production source unit with exactly 500 code lines and cohesive methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as within-limits
    And  the report labels the unit as within limits
    And  the report records no recommendation

  Scenario: A type one line over the readable size limit is a finding
    Given  a production source unit with 501 code lines and cohesive methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report labels the unit as a hint
    And  the report records 501 code lines
    And  the report records no recommendation

  @outline-report
  Scenario: A report is available in machine-readable form
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --json"
    Then  stdout is valid JSON
    And  the JSON report classifies the unit
    And  the JSON report has no action or recommendation fields
    And  the JSON report does not present classifications as review or archive verdicts

  Scenario: A report is written to a file
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --out report.txt"
    Then  the report is written to "report.txt"
    And  stdout is empty

  Scenario: A report output path that cannot be written is an error
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    When  the developer runs "givn outline report --out missing/report.txt"
    Then  the command exits with code 1
    And  stderr names "missing/report.txt"

  Scenario: An unsupported construct is named and not invented
    Given  a Givn project with the outline experiment enabled
    And  a production source file with an item-position macro
    When  the developer runs "givn outline report"
    Then  the report names the unsupported construct
    And  the report does not classify the unsupported unit

  @outline-report
  Scenario: The JSON report carries the schema and rules versions
    Given  a Givn project with the outline experiment enabled
    And  a production source unit
    When  the developer runs "givn outline report --json"
    Then  the JSON report has schema version 5
    And  the JSON report has rules version 1

  @outline-report
  Scenario: The JSON report carries analyzer capabilities and limitations
    Given  a Givn project with the outline experiment enabled
    And  a production source unit
    When  the developer runs "givn outline report --json"
    Then  the JSON report names Rust and Python as the supported source languages
    And  the JSON report names the supported constructs
    And  the JSON report names the analyzer limitations
    And  the limitations state that structural call order is not a complete runtime trace
    And  the limitations state that uncertain or omitted call edges limit conclusions

  Scenario: Conditional code is counted as written
    Given  a production source unit with platform-conditional methods
    When  the outline report assesses the unit
    Then  the report counts both platform variants

  Scenario: The same source produces the same report through either analysis engine
    Given  a production source unit supported by two analysis engines
    When  the outline report is rendered from each engine
    Then  the two reports are identical

  Scenario: The report is deterministic
    Given  a production source unit
    When  the outline report is produced twice
    Then  the two reports are identical

  Scenario: A developer scopes the report to a workspace member
    Given  a Givn project with the outline experiment enabled
    And  a workspace member "crates/core" with a production source unit that is over the readable size
    When  the developer runs "givn outline report --path crates/core"
    Then  the report classifies the unit as a god type
    And  the report names the file "crates/core/src/engine.rs"

  @outline-report
  Scenario: The report fails when the scope has no production sources
    Given  a Givn project with the outline experiment enabled
    And  no production source files
    When  the developer runs "givn outline report --path ."
    Then  the command exits with code 1
    And  stderr names the requested scope
    And  stderr says there are no supported production sources
    And  stderr names Rust and Python as the supported source languages

  Scenario: An explicitly requested scope that does not exist is an error
    Given  a Givn project with the outline experiment enabled
    When  the developer runs "givn outline report --path missing"
    Then  the command exits with code 1
    And  stderr names "missing"

  Scenario: A file that cannot be analyzed is reported and the report continues
    Given  a Givn project with the outline experiment enabled
    And  a well-formed production source unit
    And  a malformed production source file
    When  the developer runs "givn outline report"
    Then  the report names the malformed file
    And  the report still classifies the well-formed unit

  Scenario: A large type in the inspect band is a cohesive large type
    Given  a production source unit with a large type in the inspect band
    When  the outline report assesses the unit
    Then  the report classifies the unit as a cohesive large type
    And  the report records no recommendation

  Scenario: A module over the plan limit is a module finding
    Given  a production source unit with a module over the plan limit
    When  the outline report assesses the unit
    Then  the report classifies the unit as a module
    And  the report labels the unit as a hint
    And  the report records no recommendation

  Scenario: The JSON hints count counts every non-within-limits unit
    Given  a Givn project with the outline experiment enabled
    And  a production source unit that is over the readable size
    And  a production source unit with a data-only enum over the readable size
    And  a production source unit within the readable size
    When  the developer runs "givn outline report --json"
    Then  the JSON hints count matches the non-within-limits units
    And  the JSON report classifies the data-only unit

  Scenario: Every analyzed unit has exactly one classification
    Given  the outline fixture corpus
    When  the outline report assesses every unit
    Then  every unit has exactly one classification
    And  no unit carries an action or recommendation

  Scenario: Units are sorted by classification and path
    Given  the outline fixture corpus
    When  the developer runs "givn outline report"
    Then  the report lists units by classification, then path, then unit id

  Scenario: A Python class is outlined with its methods and calls
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the Python class "OrderBook"
    And  the JSON report lists the class methods "add" and "fill"
    And  the JSON outline records the call from "fill" to "add"
    And  the JSON report records the file language "python"

  Scenario: A component outline is a call tree
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    When  the developer runs "givn outline report"
    Then  the outline of the class "OrderBook" shows "fill" calling "add"

  Scenario: A call to another component is a leaf
    Given  a Givn project with the outline experiment enabled
    And  a production source unit whose method calls a module function
    When  the developer runs "givn outline report"
    Then  the type outline calls the module function without expanding it

  Scenario: A Python module without classes is a module unit
    Given  a Givn project with the outline experiment enabled
    And  a Python module with top-level functions
    When  the developer runs "givn outline report --json"
    Then  the JSON report names one Python module unit
    And  the JSON report lists the module functions "load" and "save"

  Scenario: A Python project whose analyzer is unavailable still reports
    Given  a Givn project with the outline experiment enabled
    And  a Python module with a class and methods
    And  python3 is not on the PATH
    When  the developer runs the outline report
    Then  the command exits with code 0
    And  the report names the file "src/tracker.py"
    And  the report names the analyzer as unavailable

  Scenario: A Rust-only project never invokes python3
    Given  a Givn project with the outline experiment enabled
    And  a production source unit within the readable size
    And  a python3 recorder that records every invocation
    When  the developer runs the outline report with the recorder on the PATH
    Then  the command exits with code 0
    And  the python3 recorder records no invocation
    And  the report names only the rust language

  Scenario: A method call on an unknown receiver is a name-only guess
    Given  a production source unit whose method calls a receiver of unknown type
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the call "is_empty" as "name_only"
    And  the JSON call names "tier::TierReasoning::is_empty" as its candidate target

  Scenario: A name-only guess is not expanded as a same-unit call
    Given  a production source unit whose method calls a receiver of unknown type
    When  the developer runs "givn outline report"
    Then  the report prints the call "is_empty" as "-> [name_only, public, TierReasoning] is_empty"
    And  the name-only call "is_empty" is not expanded into a deeper-indented call line
    And  the report records "LCOM4 2" for the unit "type:src/tier.rs::TierReasoning"

  Scenario: A call on the unit's own receiver resolves within the unit
    Given  a production source unit whose method calls another method on itself
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the call "validate" as "same_file"
    And  the JSON call names "engine::Engine::validate" as its target
    And  the JSON report records the call "check" as "same_file"
    And  the JSON call names "engine::Engine::check" as its target

  Scenario: A method call on a named receiver resolves to that owner
    Given  a production source unit whose method calls an associated function on a project type
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the call "from" as "same_file"
    And  the JSON call names "line::Line::from" as its target
    And  the JSON report records 2 methods for the unit "type:src/line.rs::Line"

  Scenario: An ambiguous method name has no resolved target
    Given  a production source unit whose method calls an unknown receiver whose name two project types declare
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the call "refresh" as "ambiguous"
    And  the JSON call "refresh" names no target

  Scenario: An associated-function call whose qualifier contradicts the target is external
    Given  a production source unit that calls "Line::from" while the project declares "Error::from"
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the call "from" as "external"
    And  the JSON call "from" names no target

  Scenario: A god type found through cohesion reports only the cohesion evidence
    Given  a production source unit over the readable size whose methods are connected only by calls
    When  the outline report assesses the unit
    Then  the report classifies the unit as a god type
    And  the report records the evidence "type-tcc < 500"
    And  the report records no "type-lcom4" evidence

  Scenario: An interface found through its public methods reports only public-method evidence
    Given  a production source unit with more than 50 public methods
    When  the outline report assesses the unit
    Then  the report classifies the unit as an interface
    And  the report records the evidence "type-public-methods > 50"
    And  the report records no "type-kind-trait" evidence

  Scenario: A long method found through complexity reports only complexity evidence
    Given  a production source unit with a method over the complexity limit but within the length limit
    When  the outline report assesses the unit
    Then  the report classifies the unit as a method-length finding
    And  the report records the evidence "method-complexity > 50"
    And  the report records no "method-code-lines" evidence

  Scenario: A Rust module outline lists the types it declares
    Given  a production Rust module with three declared types and one free function
    When  the developer runs "givn outline report --json"
    Then  the JSON module outline lists the types "First", "Second", and "Third"

  Scenario: A module outline interleaves member types and free functions in source order
    Given  a production Rust module with a free function, a type, a free function, and a type in that order
    When  the developer runs "givn outline report --json"
    Then  the JSON module outline lists "alpha", "Beta", "gamma", and "Delta" in source order

  Scenario: A Python module outline lists its classes
    Given  a Python module with two classes and one function
    When  the developer runs "givn outline report --json"
    Then  the JSON module outline lists the classes "OrderBook" and "Ledger"

  Scenario: The JSON report names each call's resolved target
    Given  a production source unit whose method calls a module function
    When  the developer runs "givn outline report --json"
    Then  the JSON call "normalize" names the target "component::normalize"

  Scenario: A same-unit method implemented in another file is project_other_file
    Given  a production source unit declared in one file and implemented in another
    When  the developer runs "givn outline report"
    Then  the report prints the call "validate" as "-> [project_other_file, public, Engine] validate"
    And  the call "validate" is expanded into a deeper-indented call line
    And  the report records "LCOM4 1" for the unit "type:src/engine_core.rs::Engine"

  Scenario: A developer reads each call's resolution in the human report
    Given  a Givn project with the outline experiment enabled
    And  a production source unit whose method calls its own method, a cross-file module function, and a receiver of unknown type
    When  the developer runs "givn outline report"
    Then  the report prints the call "validate" as "-> [same_file, public, Engine] validate"
    And  the report prints the call "normalize" as "-> [project_other_file, public, module] normalize"
    And  the report prints the call "is_empty" as "-> [name_only, public, Engine] is_empty"
    And  the name-only call "is_empty" is not expanded into a deeper-indented call line

  Scenario: A file's code lines exclude inline test modules
    Given  a production source file with an inline test module
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the file "src/counted.rs" metric "code_lines" as 8
    And  the JSON report records the file "src/counted.rs" metric "physical_lines" as 18

  Scenario: A unit's physical lines cover only its own spans
    Given  a production source file with a small type and a large unrelated function
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the unit "type:src/scoped.rs::Scoped" metric "physical_lines" as 8
    And  the JSON report records the unit "type:src/scoped.rs::Scoped" metric "code_lines" as 8
    And  the JSON report records the file "src/scoped.rs" metric "physical_lines" as 32

  Scenario: An impl-level test method is excluded from the unit's lines
    Given  a production source unit with an impl-level test method
    When  the developer runs "givn outline report --json"
    Then  the JSON report records the unit "type:src/guarded.rs::Guarded" metric "code_lines" as 8
    And  the JSON report records the unit "type:src/guarded.rs::Guarded" metric "physical_lines" as 9
    And  the JSON report records 1 method for the unit "type:src/guarded.rs::Guarded"

  Scenario: A name-only guess does not connect methods for cohesion
    Given  a production source unit over the readable size whose only call edges are name-only guesses
    When  the developer runs "givn outline report"
    Then  the report records "LCOM4 4" for the unit "type:src/cohesion.rs::Engine"
    And  the report classifies the unit as a god type
    And  the report records the evidence "type-lcom4 > 2"

  Scenario: A method signature keeps arrays, qualified paths, and its full type detail
    Given  a production source unit with a method whose types use paths, generics, references, slices, tuples, arrays, pointers, and fully qualified paths
    When  the developer runs "givn outline report --json"
    Then  the JSON method signature contains "Result<Self, Error>"
    And  the JSON method signature contains "&[u8]"
    And  the JSON method signature contains "(u32, String)"
    And  the JSON method signature contains "*const Config"
    And  the JSON method signature contains "[u8; 4]"
    And  the JSON method signature contains "crate::config::Config"

  Scenario: The outline experiment is enabled by default
    Given  a Givn project
    When  the developer runs "givn addons list"
    Then  stdout contains "[enabled ]  outline"

  Scenario: A project can explicitly disable the outline experiment
    Given  a Givn project
    When  the user runs `givn addons disable outline`
    Then  the command exits 0
    When  the developer runs "givn addons list"
    Then  the outline addon is listed as disabled

  @outline-report
  Scenario Outline: Unavailable outline measurements are not reported as measured zero
    Given  a Givn project with the outline experiment enabled
    And  a supported production source unit for which <measurement> is unavailable
    When  the developer requests the outline report in text and JSON form
    Then  the text report marks <measurement> as unavailable
    And  the JSON report represents <measurement> as null
    And  neither report represents the unavailable measurement as zero
    Examples:
      | measurement |
      | tight class cohesion |
      | method complexity |
      | method nesting |

  @outline-report
  Scenario: Loose class cohesion is omitted when it is not independently measured
    Given  a Givn project with the outline experiment enabled
    And  a production source unit with measured tight class cohesion
    And  loose class cohesion is not independently measured
    When  the developer requests the outline report in text and JSON form
    Then  neither report includes an LCC measurement
    And  neither report presents tight class cohesion as loose class cohesion

  @outline-report
  Scenario: A Java production class is reported
    Given  a Givn project with the outline experiment enabled
    And  a Java production class
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the unit "Widget" with language "java"
    And  the JSON report gives the type kind "class" to "Widget"

  @outline-report
  Scenario: A TypeScript production class and function are reported
    Given  a Givn project with the outline experiment enabled
    And  a TypeScript production class and function
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the unit "Widget" with language "typescript"
    And  the JSON report names the function "makeWidget" with language "typescript"

  @outline-report
  Scenario: A TypeScript interface is reported with its own type kind
    Given  a Givn project with the outline experiment enabled
    And  a TypeScript production interface
    When  the developer runs "givn outline report --json"
    Then  the JSON report gives the type kind "interface" to "Drawable"

  @outline-report
  Scenario: A mixed-language report discloses every supported language
    Given  a Givn project with the outline experiment enabled
    And  a mixed Rust Python Java TypeScript and JavaScript production source set
    When  the developer runs "givn outline report --json"
    Then  the JSON report names all supported languages
    And  the JSON report names the unit "Widget" with language "java"
    And  the JSON report names the unit "Widget" with language "typescript"
    And  the JSON report names the unit "Widget" with language "javascript"

  @outline-report
  Scenario: A malformed Java file does not hide a valid TypeScript file
    Given  a Givn project with the outline experiment enabled
    And  a malformed Java production file beside a valid TypeScript production file
    When  the developer runs "givn outline report --json"
    Then  the JSON report records a parse failure for "src/Broken.java"
    And  the JSON report names the unit "Widget" with language "typescript"

  @outline-report
  Scenario: Java and TypeScript test and dependency sources are excluded
    Given  a Givn project with the outline experiment enabled
    And  Java and TypeScript test and dependency sources
    When  the developer runs "givn outline report --json"
    Then  stdout does not contain "IgnoredJava"
    And  stdout does not contain "IgnoredTypeScript"

  @outline-report
  Scenario: Unsupported-only Go production sources are not reported as absent
    Given  a Givn project with the outline experiment enabled
    And  the requested scope contains only production source files written in Go
    When  the developer runs "givn outline report --path ."
    Then  the command exits with code 1
    And  stderr names the requested scope
    And  stderr says there are no supported production sources
    And  stderr names all supported source languages
    And  stderr does not claim that no production sources exist

  @outline-report
  Scenario: A JavaScript production class and function are reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production class and function
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the unit "Widget" with language "javascript"
    And  the JSON report gives the type kind "class" to "Widget"
    And  the JSON report names the function "makeWidget" with language "javascript"

  @outline-report
  Scenario: A JavaScript class field is reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production class with a field
    When  the developer runs "givn outline report --json"
    Then  the JSON report records 1 field on "Widget"

  @outline-report
  Scenario Outline: JavaScript production modules are reported for every supported extension
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production module with the extension "<extension>"
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the function "drain" with language "javascript"
    Examples:
      | extension |
      | .js |
      | .mjs |
      | .cjs |

  @outline-report
  Scenario: A JavaScript arrow-function binding is reported
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production arrow-function binding
    When  the developer runs "givn outline report --json"
    Then  the JSON report names the function "clone" with language "javascript"

  @outline-report
  Scenario: A JavaScript arrow-function binding is named in the human report
    Given  a Givn project with the outline experiment enabled
    And  a JavaScript production arrow-function binding
    When  the developer runs "givn outline report"
    Then  stdout contains "clone = (value) =>"

  @outline-report
  Scenario: A malformed JavaScript file does not hide a valid TypeScript file
    Given  a Givn project with the outline experiment enabled
    And  a malformed JavaScript production file beside a valid TypeScript production file
    When  the developer runs "givn outline report --json"
    Then  the JSON report records a parse failure for "src/Broken.js"
    And  the JSON report names the unit "Widget" with language "typescript"

  @outline-report
  Scenario: JavaScript test and dependency sources are excluded
    Given  a Givn project with the outline experiment enabled
    And  JavaScript test and dependency sources
    When  the developer runs "givn outline report --json"
    Then  stdout does not contain "IgnoredJavaScript"
    And  stdout does not contain "IgnoredJavaScriptDependency"

  @outline-report
  Scenario: JSX-only production sources remain unsupported
    Given  a Givn project with the outline experiment enabled
    And  the requested scope contains only production source files written in JSX
    When  the developer runs "givn outline report --path ."
    Then  the command exits with code 1
    And  stderr names the requested scope
    And  stderr says there are no supported production sources
    And  stderr names all supported source languages
    And  stderr does not claim that no production sources exist
