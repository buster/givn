@givn.delta @release-0-3-0
Feature: Givn 0.3.0 release and legacy project migration

  @givn.modified
  Scenario: A project outside Git is protected from a partial automatic migration
    Given  a Givn 0.2.0 project that is not a Git repository
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  the command explains that a version-controlled project is required for the automatic migration
