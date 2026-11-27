@givn.delta @clean-room-preparation
Feature: Clean-room preparation

  @givn.added
  Scenario: Preparation carries every declared build input into the spec copy
    Given  a Git project with ise enabled and a declaration whose build inputs sit under the four content roles
    And  a committed revision with those build inputs, one source file, and an End-to-end manifest
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 7"
    And  the spec copy holds "Cargo.toml" and "Cargo.lock"
    And  the spec copy holds "rust-toolchain.toml" and "assets/templates/glossary.md"
    And  the spec copy holds "assets/instructions/ise.md" and "givn/specs/demo.feature"
    And  the spec copy omits "src/main.rs" and "e2e/Cargo.toml"
