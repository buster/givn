@givn.delta @clean-room-preparation

Feature: Clean-room preparation

  @givn.modified @e2e @wip
  Scenario: Developer prepares a clean-room trial from a recorded revision
    Given  a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**"
    And  a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", and "notes.md"
    When  the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs" and a receipt
    Then  preparation succeeds
    And  the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit
    And  the report prints "Retained: 2", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"
    And  the report prints "Frozen package: e2e, not copied (2 files)"
    And  the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis
    And  the report prints the written trial path and the receipt path
    And  the trial holds the retained and kept source files, including "src/keys/mod.rs"
    And  the trial holds the declaration "givn/specification.yaml"
    And  the trial omits "src/keys/parser.rs", "notes.md", "tests/e2e/Cargo.toml", and "tests/e2e/steps/demo.rs"
    And  the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version

  @givn.added
  Scenario: The trial record names the frozen role its trial omits
    Given  a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**"
    And  a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", and "src/kept.rs"
    When  the developer prepares a trial from "HEAD" with a full replacement and a receipt
    Then  preparation succeeds
    And  the receipt's excluded roles hold "e2e"
    And  the receipt's retained set holds no path under "tests/e2e"
    And  the receipt holds one e2e identity per resolved package path

  @givn.added
  Scenario: A declaration with no frozen package excludes no role
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer prepares a trial from "HEAD" with a full replacement and a receipt
    Then  preparation succeeds
    And  the report holds no "Frozen package" line
    And  the receipt's excluded roles are empty

  @givn.added
  Scenario: A frozen package root that covers the declaration keeps it in the trial
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "givn/**"
    And  a committed revision with "givn/specs/a.md" and "src/kept.rs"
    When  the developer prepares a trial from "HEAD" with a full replacement
    Then  preparation succeeds and prints "Retained: 1" and "Frozen package: e2e, not copied (1 file)"
    And  the trial holds "givn/specification.yaml"
    And  the trial omits "givn/specs/a.md"

  @givn.modified
  Scenario: A dry run performs every read and writes no trial or record
    Given  a Git project with ise enabled and a declaration
    When  the developer runs a dry run from "HEAD" with a full replacement and a receipt
    Then  the dry run prints the fixed report lines, the receipt path line, the frozen line, and a "Would write:" block
    And  the "Would write:" block lists the sorted paths the trial would hold
    And  the "Would write:" block lists no path under "tests/e2e"
    And  the trial path and the receipt path remain absent
