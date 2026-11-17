@givn.delta @trial-input-verification

Feature: Spec-copy verification

  @givn.removed
  Scenario: Verifier re-checks a prepared trial and receives the frozen-input report
    Given the retired scenario is replaced by "Verifier compares the spec copy with the manifest"

  @givn.removed
  Scenario: Verification names a retained file changed in the trial
    Given the retired scenario is replaced by "Comparison names a changed Specification file in the copy"

  @givn.removed
  Scenario: Verification names an e2e package file changed in the trial
    Given the retired scenario is replaced by "Comparison refuses a file added under an End-to-end role"

  @givn.removed
  Scenario: Verification names an e2e package file changed in the checkout
    Given the retired scenario is replaced by "Comparison names an End-to-end test changed in the checkout"

  @givn.removed
  Scenario: Verification names an e2e package path the record does not list
    Given the retired scenario is replaced by "Comparison names an added End-to-end file in the checkout"

  @givn.removed
  Scenario: Verification names an e2e scenario changed in the checkout
    Given the retired scenario is replaced by "Comparison names an @e2e scenario changed in the checkout"

  @givn.removed
  Scenario: Verification names an e2e scenario the record does not list
    Given the retired scenario is replaced by "Comparison names an added @e2e scenario file in the checkout"

  @givn.removed
  Scenario: Verification reports no drift after a kept source file changes
    Given the retired scenario is replaced by "Comparison reports no difference after a kept Source code file is edited"

  @givn.removed
  Scenario: Verification names a recorded path missing from the trial
    Given the retired scenario is replaced by "Comparison names a missing Specification file in the copy"

  @givn.removed
  Scenario: Verification refuses while the ise addon is off
    Given the retired scenario is replaced by "Comparison refuses while the ise addon is off"

  @givn.removed
  Scenario: Verification refuses while architecture principles are off
    Given the retired scenario is replaced by "Comparison refuses while architecture principles are off"

  @givn.removed
  Scenario: Verification refuses a record it cannot read
    Given the retired scenario is replaced by "Comparison refuses an unreadable manifest"

  @givn.removed
  Scenario: Verification refuses a record with another schema
    Given the retired scenario is replaced by "Comparison refuses a manifest whose scope is not ise-manifest/v1"

  @givn.removed
  Scenario: Verification refuses a record without a replaced digest
    Given the retired scenario is replaced by "Comparison refuses a manifest with a malformed checksum"

  @givn.removed
  Scenario: Verification refuses a record with an unknown field
    Given the retired scenario is replaced by "Comparison refuses a manifest with an unknown field"

  @givn.removed
  Scenario: Verification refuses a record with a missing field
    Given the retired scenario is replaced by "Comparison refuses a manifest with a missing field"

  @givn.removed
  Scenario: Verification refuses a record placed inside the trial
    Given the retired scenario is replaced by "Comparison refuses a manifest inside the copy"

  @givn.removed
  Scenario: Verification refuses a record whose written path is not the trial
    Given the retired scenario is replaced by "Comparison refuses a manifest whose copy directory is not the copy"

  @givn.removed
  Scenario: Verification refuses a record whose backend differs from the selected backend
    Given the retired scenario is replaced by "Comparison refuses a manifest with an unknown field"

  @givn.removed
  Scenario: Verification names every drifting path in one run
    Given the retired scenario is replaced by "Comparison names every drifting path in one run"

  @givn.removed
  Scenario: Verification names a declaration changed in the trial
    Given the retired scenario is replaced by "Comparison names a changed Specification file in the copy"

  @givn.removed
  Scenario: Verification names a declaration missing from the trial
    Given the retired scenario is replaced by "Comparison names a missing Specification file in the copy"

  @givn.removed
  Scenario: Verification reports no drift after only the checkout's structure role changes
    Given the retired scenario is replaced by "Comparison reports a Specification changed after the copy was written"

  @givn.removed
  Scenario: Verification names the e2e package a checkout declaration's added root brings
    Given the retired scenario is replaced by "Comparison names an added End-to-end root in the checkout declaration"

  @givn.removed
  Scenario: Verification bounds the frozen paths it refuses
    Given the retired scenario is replaced by "Comparison refuses a file added under a declared Specification path"

  @givn.removed
  Scenario: Verification keeps a trial prepared before the package was excluded re-checkable
    Given the retired scenario is replaced by "Comparison refuses a manifest whose scope is not ise-manifest/v1"

  @givn.removed
  Scenario: A frozen role that covers the declaration re-checks clean
    Given the retired scenario is replaced by "Verifier compares the spec copy with the manifest"

  @givn.added @e2e
  Scenario: Verifier compares the spec copy with the manifest
    Given  a prepared spec copy with a manifest
    When  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero
    And  comparison prints no line beginning "- "

  @givn.added
  Scenario: Comparison names a changed Specification file in the copy
    Given  a prepared spec copy with a manifest
    When  the agent edits the Specification file "givn/specs/demo/spec.feature" the spec copy was written with
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 changed"
    And  comparison prints "- givn/specs/demo/spec.feature"
    And  comparison prints "End-to-end tests: match"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names a missing Specification file in the copy
    Given  a prepared spec copy with a manifest
    When  the agent deletes the Specification file "givn/specs/demo/spec.feature" the spec copy was written with
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 missing"
    And  comparison prints "- givn/specs/demo/spec.feature"
    And  comparison prints "End-to-end tests: match"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison reports no difference after the agent adds Source code
    Given  a prepared spec copy with a manifest
    When  the agent adds a rebuild file at a removed path
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero

  @givn.added
  Scenario: Comparison reports no difference after a kept Source code file is edited
    Given  a prepared spec copy with a manifest
    When  the agent edits the kept Source code file "src/keys/steps.rs" the spec copy was written with
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero

  @givn.added
  Scenario: Comparison refuses a file added under a declared Specification path
    Given  a prepared spec copy with a manifest
    When  the agent adds a file under a declared Specification path
    And  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the added path as a Specification change
    And  comparison prints no "Specification files: match" line

  @givn.added
  Scenario: Comparison refuses a file added under an End-to-end role
    Given  a prepared spec copy with a manifest
    When  the agent adds a file under the declared End-to-end role
    And  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the added path as a Specification change
    And  comparison prints no "Specification files: match" line

  @givn.added
  Scenario: Comparison refuses a deleted kept Source code file
    Given  a prepared spec copy with a manifest
    When  the agent deletes the kept Source code file "src/keys/steps.rs" the spec copy was written with
    And  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming "src/keys/steps.rs"
    And  comparison prints no "Specification files:" line

  @givn.added
  Scenario: Comparison names an End-to-end test changed in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer edits the End-to-end (E2E) test "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "End-to-end tests: 1 changed"
    And  comparison prints "- tests/e2e/steps/keys.rs"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names an @e2e scenario changed in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer edits a scenario tagged "e2e" in "givn/specs/demo/spec.feature" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 changed"
    And  comparison prints "- givn/specs/demo/spec.feature"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names a content-only change to an @e2e feature in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer edits the feature description of "givn/specs/demo/spec.feature" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 changed"
    And  comparison prints "- givn/specs/demo/spec.feature"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names an added End-to-end file in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer adds the End-to-end (E2E) file "tests/e2e/steps/extra.rs" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 added"
    And  comparison prints "+ tests/e2e/steps/extra.rs"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names an added @e2e scenario file in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer adds the feature file "givn/specs/demo/extra.feature" with an "@e2e" scenario in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 added"
    And  comparison prints "+ givn/specs/demo/extra.feature"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names an added End-to-end root in the checkout declaration
    Given  a prepared spec copy with a manifest
    When  the developer adds the e2e root "tests/extra-e2e" to the checkout declaration with the file "tests/extra-e2e/Cargo.toml"
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 changed" and "End-to-end tests: 1 added"
    And  comparison prints "- givn/specification.yaml"
    And  comparison prints "+ tests/extra-e2e/Cargo.toml"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names an added Specification file in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer adds the contract file "docs/extra.md" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 added"
    And  comparison prints "+ docs/extra.md"
    And  comparison prints "End-to-end tests: match"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names a missing Specification file in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer deletes the Specification file "docs/reference.md" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 missing" and "End-to-end tests: match"
    And  comparison prints "- docs/reference.md"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names a missing End-to-end test in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer deletes the End-to-end (E2E) test "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 missing"
    And  comparison prints "- tests/e2e/steps/keys.rs"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names a missing @e2e feature in the checkout
    Given  a prepared spec copy with a manifest
    When  the developer deletes the Specification file "givn/specs/demo/spec.feature" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: 1 missing"
    And  comparison prints "- givn/specs/demo/spec.feature"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison names every drifting path in one run
    Given  a prepared spec copy with a manifest
    When  the agent edits the Specification file "givn/specs/demo/spec.feature" the spec copy was written with
    And  the developer edits the Specification file "docs/reference.md" in the checkout
    And  the developer deletes the Specification file "docs/arc42/02-architecture-constraints.md" in the checkout
    And  the developer adds the contract file "docs/extra.md" in the checkout
    And  the developer edits the End-to-end (E2E) test "tests/e2e/steps/keys.rs" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 2 changed, 1 missing, 1 added" and "End-to-end tests: 1 changed"
    And  comparison prints "- givn/specs/demo/spec.feature" and "- docs/reference.md"
    And  comparison prints "- docs/arc42/02-architecture-constraints.md"
    And  comparison prints "+ docs/extra.md"
    And  comparison prints "- tests/e2e/steps/keys.rs"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison prints one line for a path drifting in the copy and the checkout
    Given  a prepared spec copy with a manifest
    When  the agent edits the Specification file "docs/reference.md" the spec copy was written with
    And  the developer edits the Specification file "docs/reference.md" in the checkout
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 changed" and "End-to-end tests: match"
    And  comparison prints exactly one "- docs/reference.md" line under the "Specification files" subject
    And  comparison prints "Specification changed after the copy was written"
    And  comparison exits 1

  @givn.added
  Scenario: Comparison ignores an ignored file under the End-to-end package
    Given  a prepared spec copy with a manifest
    When  the developer adds an ignored file under the checkout's End-to-end (E2E) package
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "End-to-end tests: match"
    And  comparison exits zero

  @givn.added
  Scenario: Comparison reports a Specification changed after the copy was written
    Given  a prepared spec copy with a manifest
    When  the developer rewrites the checkout declaration's "structure" role to "config/kept.toml"
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: 1 changed, 1 added"
    And  comparison prints "- givn/specification.yaml"
    And  comparison prints "Specification changed after the copy was written"
    And  comparison prints "+ config/kept.toml"
    And  comparison exits 1

  @givn.added @wip
  Scenario: Comparison refuses while the ise addon is off
    Given  a prepared spec copy with a manifest
    And  the project disables ise after preparation
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming "ise"
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses while architecture principles are off
    Given  a prepared spec copy with a manifest
    And  the project disables architecture principles after preparation
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming "architecture_principles"
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest whose scope is not ise-manifest/v1
    Given  a prepared spec copy whose manifest names the scope "ise-trial/v1"
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and "ise-manifest/v1"
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest with an unknown field
    Given  a prepared spec copy whose manifest carries an unknown field
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and the unknown field
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest with a missing field
    Given  a prepared spec copy whose manifest is missing the "counts" field
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and the missing field
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest with a malformed checksum
    Given  a prepared spec copy whose manifest holds a malformed Specification checksum
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and the malformed checksum
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses an unreadable manifest
    Given  a prepared spec copy whose manifest is not readable YAML
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and the parse error
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest whose copy directory is not the copy
    Given  a prepared spec copy whose manifest names another copy directory
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest and the copy
    And  comparison prints no "Specification files:" line

  @givn.added @wip
  Scenario: Comparison refuses a manifest inside the copy
    Given  a prepared spec copy with its manifest inside the copy
    When  the verifier compares the spec copy with the manifest
    Then  comparison fails and exits 1, naming the manifest
    And  comparison prints no "Specification files:" line
