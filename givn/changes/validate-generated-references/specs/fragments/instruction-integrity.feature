@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The characterization guidance requests the reverse review from Givn
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-characterize guidance requests `givn instructions review-reverse`
    And  the generated givn-characterize command requests `givn instructions review-reverse`

  @givn.added
  Scenario: The generated guidance references resolve with the default configuration
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every guidance reference resolves in the consumer project or is owned by a named creation step

  @givn.added
  Scenario: The generated guidance references resolve without addons
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  every guidance reference resolves in the consumer project or is owned by a named creation step

  @givn.added
  Scenario: The printed instructions reference only resolvable files
    Given  a Givn project with an active change
    When  the author requests every instruction Givn prints
    Then  every printed guidance reference resolves in the consumer project or is owned by a named creation step

  @givn.added
  Scenario: An undeclared reference in an installed skill fails the check
    Given  a givn project with all agent skill targets enabled
    And  a givn-ideation skill override that names the missing file "givn/reference-that-resolves-nowhere.md"
    When  the user runs `givn skills sync`
    Then  the installed reference check fails and names "givn/reference-that-resolves-nowhere.md"

  @givn.added @wip
  Scenario: A printed reference that resolves nowhere fails the check
    Given  a Givn project with an active change
    And  a specs instruction override that names the missing file "givn/reference-that-resolves-nowhere.md"
    When  the author requests the specs instructions through the Givn CLI
    Then  the printed reference check fails and names "givn/reference-that-resolves-nowhere.md"
