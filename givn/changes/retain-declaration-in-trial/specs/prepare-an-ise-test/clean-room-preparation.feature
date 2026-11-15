@givn.delta @clean-room-preparation

Feature: Clean-room preparation

  @givn.modified @e2e @wip
  Scenario: Developer prepares a clean-room trial from a recorded revision
    Given  a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**"
    And  a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", and "notes.md"
    When  the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs" and a receipt
    Then  preparation succeeds
    And  the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit
    And  the report prints "Retained: 4", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"
    And  the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis
    And  the report prints the written trial path and the receipt path
    And  the trial holds the retained and kept source files, including "src/keys/mod.rs"
    And  the trial holds the declaration "givn/specification.yaml"
    And  the trial omits "src/keys/parser.rs" and "notes.md"
    And  the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version

  @givn.added @wip
  Scenario: The trial carries the declaration when no role names it
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer prepares a trial from "HEAD" with a full replacement
    Then  preparation succeeds
    And  the trial holds "givn/specification.yaml"

  @givn.added @wip
  Scenario: The trial record lists the declaration in its retained set
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer prepares a trial from "HEAD" with a full replacement and a receipt
    Then  preparation succeeds
    And  the receipt lists "givn/specification.yaml" in its retained set

  @givn.added @wip
  Scenario: Preparation refuses a replacement path that names the declaration
    Given  a Git project with ise enabled and a declaration that sources "givn/**"
    And  a committed revision with "src/kept.rs"
    When  the developer prepares a trial from "HEAD" with the path "givn/specification.yaml"
    Then  preparation fails and names "givn/specification.yaml" as retained
    And  the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a directory that contains the declaration
    Given  a Git project with ise enabled and a declaration that sources "givn/**"
    And  a committed revision with "givn/notes.md"
    When  the developer prepares a trial from "HEAD" with the directory path "givn"
    Then  preparation fails and names "givn/specification.yaml" as retained
    And  the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation partitions with the declaration recorded at the selected revision
    Given  a Git project with ise enabled and a declaration whose "structure" role names "docs/glossary.md"
    And  a committed revision with "docs/glossary.md" and "src/one.rs"
    And  a later revision that rewrites the declaration without the "structure" entry
    When  the developer prepares a trial from the first revision with a full replacement
    Then  preparation succeeds
    And  the trial holds "docs/glossary.md"
    And  the trial's "givn/specification.yaml" is the declaration committed at the selected revision

  @givn.added @wip
  Scenario: Preparation refuses a revision that does not record the declaration
    Given  a Git project with ise enabled whose first revision predates the declaration
    When  the developer prepares a trial from the first revision with a full replacement
    Then  preparation fails and names "givn/specification.yaml" and the revision
    And  the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a malformed declaration at the selected revision
    Given  a Git project with ise enabled, a malformed declaration in its first revision, and a valid declaration at "HEAD"
    When  the developer prepares a trial from the first revision with a full replacement
    Then  preparation fails and names "givn/specification.yaml" and the loader's reason
    And  the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: A full replacement keeps the declaration under a source root
    Given  a Git project with ise enabled and a declaration that sources "givn/**"
    And  a committed revision with "givn/notes.md"
    When  the developer prepares a trial from "HEAD" with a full replacement
    Then  preparation succeeds and prints "Replaced: 3"
    And  the trial holds "givn/specification.yaml"
    And  the trial omits "givn/notes.md"

  @givn.added @wip
  Scenario: A role that matches the declaration counts it once
    Given  a Git project with ise enabled and a declaration whose "contracts" role names "givn/**"
    And  a committed revision with "givn/specs/a.md"
    When  the developer prepares a trial from "HEAD" with a full replacement
    Then  preparation succeeds and prints "Retained: 2"
    And  the trial holds "givn/specification.yaml"
