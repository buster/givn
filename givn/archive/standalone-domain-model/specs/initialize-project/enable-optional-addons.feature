@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.modified
  Scenario: givn init --no-addons produces no skill or command addon files
    Given  a project root without a givn/ directory
    When  the user runs `givn init --no-addons --agents opencode`
    Then  all optional addons including communication_policy and challenge are explicitly disabled
    And  no addon skills or generated commands exist
    And  the mandatory dialog-policy startup obligation remains installed
    And  the file "docs/glossary.md" does not exist
    And  the file "docs/domain-model.md" does not exist

  @givn.modified
  Scenario: givn init enables every addon by default
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  every current addon including communication_policy and challenge is effectively enabled
    And  the selected target receives addon skills without generated commands
    And  no grill_me, grill_with_docs, or absolute_mode setting is generated
    And  the file "docs/glossary.md" exists
    And  the file "docs/domain-model.md" exists

  @givn.added
  Scenario: The scaffolded glossary is the standalone vocabulary authority
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the file "docs/glossary.md" contains "| Term | Definition |"
    And the file "docs/glossary.md" contains "sole vocabulary authority"

  @givn.added
  Scenario: The scaffolded domain model carries the notation legend
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the file "docs/domain-model.md" contains "Collapsed consistency boundary"
    And the file "docs/domain-model.md" contains "External system"
    And the file "docs/domain-model.md" contains "layout: elk"
    And the file "docs/domain-model.md" contains "This is notation only, not part of the domain model"

  @givn.added
  Scenario: Existing domain documents survive initialization
    Given a project root with givn/ initialised
    And a project file "docs/glossary.md" with content "# Project glossary"
    And a project file "docs/domain-model.md" with content "# Project model"
    When the user runs `givn init --agents opencode`
    Then the file "docs/glossary.md" contains "# Project glossary"
    And the file "docs/domain-model.md" contains "# Project model"

  @givn.added
  Scenario: Enabling arc42 scaffolds only the standalone glossary
    Given an initialized addon-free project selecting opencode
    When the user runs `givn addons enable arc42`
    Then the file "docs/glossary.md" exists
    And the file "docs/domain-model.md" does not exist
    And the file "docs/arc42/12-glossary.md" contains "../glossary.md"
    And the file "docs/arc42/12-glossary.md" does not contain "| Term | Definition |"

  @givn.added
  Scenario: Enabling domain modeling scaffolds both durable documents
    Given an initialized addon-free project selecting opencode
    When the user runs `givn addons enable domain_modeling`
    Then the file "docs/glossary.md" exists
    And the file "docs/domain-model.md" exists

  @givn.added
  Scenario: Enabling an addon preserves an existing glossary
    Given an initialized addon-free project selecting opencode
    And a project file "docs/glossary.md" with content "# Project glossary"
    When the user runs `givn addons enable domain_modeling`
    Then the file "docs/glossary.md" contains "# Project glossary"

  @givn.added
  Scenario: Disabling domain modeling leaves the durable documents in place
    Given an initialized addon-free project selecting opencode
    When the user runs `givn addons enable domain_modeling`
    And the user runs `givn addons disable domain_modeling`
    Then the file "docs/glossary.md" exists
    And the file "docs/domain-model.md" exists
