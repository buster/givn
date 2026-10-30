Feature: Managed upgrade and legacy project migration

  @e2e @apply-a-managed-upgrade
  Scenario: A clean project without migration requirements receives one managed upgrade commit
    Given  a clean Git project with stale generated integration and a committed checkpoint
    And  its current-format configuration was prepared manually with the last 0.x version marker
    And  the installed Givn version is 0.9.10
    When  the consumer runs `givn upgrade`
    Then  the command exits successfully and reports the managed upgrade revision
    And  the project config identifies the current Givn version
    And  exactly one allowlist-only managed revision exists and no migration change is created
    And  no automatic configuration migration or embedded 0.x migration route is applied
    And  project specifications, extensions, and custom replacements are unchanged
    And  the terminal transcript is captured for this scenario

  @apply-a-managed-upgrade
  Scenario: An unclean project is protected from automatic migration commits
    Given  a Git project with uncommitted user changes
    When  the consumer runs `givn upgrade`
    Then  the command fails before changing project files and names the dirty paths
    And  no revision is created and user changes remain byte-for-byte unchanged

  @apply-a-managed-upgrade
  Scenario: Repeating the upgrade does not duplicate the migration
    Given  a disposable project fixture already prepared by the synthetic migration route from 1.0 to the simulated target 1.1
    And  the same synthetic route and target remain supplied through the test-only seam
    When  the consumer runs `givn upgrade` again
    Then  the command does not overwrite the migration change
    And  no second migration commit is created
    And  the command reports that the project is already prepared for migration

  @apply-a-managed-upgrade
  Scenario: A project outside Git is protected from a partial automatic migration
    Given  a project without either a Git worktree or a Fossil checkout
    When  the consumer runs `givn upgrade`
    Then  the command fails before changing project files
    And  the diagnostic explains that a version-controlled checkpoint is required

  @apply-a-managed-upgrade
  Scenario: An existing migration plan is never overwritten
    Given  a clean disposable project fixture with a synthetic 1.0 version marker
    And  a synthetic migration route to the simulated later target 1.1 is supplied through the test-only seam
    And  an existing migration proposal and design for that synthetic route
    When  the consumer runs `givn upgrade`
    Then  the existing proposal and design remain byte-for-byte unchanged
    And  the report identifies the existing migration change

  @apply-a-managed-upgrade
  Scenario: The migration change remains honest until the project review passes
    Given  a disposable project fixture with an automatically created migration change for the synthetic route from 1.0 to the simulated target 1.1
    And  that route and target were supplied through the test-only seam
    When  an LLM opens the migration change
    Then  the change asks for project-specific task derivation
    And  it does not contain a fabricated design-review pass
    And  it does not contain a fabricated implementation commit
    And  the normal Givn review and archive gates remain required

  @e2e @apply-a-managed-upgrade
  Scenario: A completed maintenance migration can be archived without a product specification
    Given  a completed no-behavior-change maintenance migration change prepared manually in a disposable project
    And  its review and archive gates are satisfied without a product specification
    When  the consumer archives the maintenance migration
    Then  the command reports that the maintenance migration was archived successfully
    And  the permanent specification tree contains no migration feature
    And  the terminal transcript is captured for this scenario

  @e2e
  Scenario: A consumer invoking a retired engine command is told it does not exist
    Given  an installed Givn binary
    When  the consumer invokes the retired `givn spec index` command
    Then  the command exits non-zero
    And  the error reports "spec index" is not an available command
    And  the error output stays free of "embedding" and "download" wording

  @apply-a-managed-upgrade
  Scenario: A clean Fossil project receives the same managed upgrade revision shape
    Given  a clean Fossil project with stale generated integration and a committed checkpoint
    And  its current-format configuration was prepared manually with the last 0.x version marker
    And  the installed Givn version is 0.9.10
    When  the consumer runs `givn upgrade`
    Then  the command exits successfully and reports one allowlist-only managed check-in
    And  no migration change is created
    And  project specifications, extensions, and custom replacements are unchanged

  @apply-a-managed-upgrade
  Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
    Given  a Fossil project with uncommitted user changes
    When  the consumer runs `givn upgrade`
    Then  the command fails before changing project files and names the dirty paths
    And  no revision is created and user changes remain byte-for-byte unchanged

  @apply-a-managed-upgrade
  Scenario: A local-only upgrade publishes rule removals without AGENTS.md
    Given  a clean checkpointed project using local-only rules and manually prepared current-format configuration
    And  a tracked generated skill and a committed human-authored AGENTS.md
    When  the consumer runs `givn upgrade`
    Then  the command exits 0
    And  the managed revision removes the generated skill from tracking while leaving its refreshed skill on disk
    And  the revision does not modify AGENTS.md or human-authored instructions

  Scenario: The upgrade reports rules that survive in reachable history
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  stdout contains "reachable revision(s) still contain a rule"

  Scenario: An upgrade reconciles full-copy guidance to the direction
    Given  a committed project root with givn/ initialised
    And  a tracked full-copy generated rule at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  the revision shows ".opencode/skills/givn-propose/SKILL.md" rewritten to the direction

  Scenario: An upgrade leaves the project's extensions untouched
    Given  a committed project root with givn/ initialised
    And  the project records an extension for the proposal instruction
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  the upgrade revision carries no extension content
    And  the recorded extension is unchanged

  Scenario: A project-owned replacement stays byte-for-byte across an upgrade
    Given  a committed project root with givn/ initialised
    And  a project-owned replacement for the proposal instruction
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  the project-owned replacement is byte-for-byte unchanged
    And  the upgrade revision carries no injected text for the replacement

  @e2e
  Scenario: A consumer invoking the retired history rewrite command is told it does not exist
    Given  an installed Givn binary
    When  the consumer invokes the retired `givn rewrite-history` command
    Then  the command exits non-zero
    And  the error reports "rewrite-history" is not an available command

  @apply-a-managed-upgrade
  Scenario Outline: Pre-1.0 projects require manual transition instead of embedded migration plans
    Given  a clean "<backend>" project created with Givn "<version>"
    And  the project retains recognized retired configuration keys requiring manual transition
    When  the consumer runs `givn upgrade`
    Then  the report names 1.0 as the supported migration baseline and explains manual transition
    And  the report does not treat the 0.x version marker alone as an unsupported configuration format
    And  no embedded pre-1.0 migration route or aggregate plan is applied
    And  no automatic compatibility aliases convert absolute_mode, grill_me, or grill_with_docs
    And  project specifications and user-owned files are not destroyed
    Examples:
      | backend | version |
      | Git | 0.7.0 |
      | Fossil | 0.7.0 |
      | Git | 0.2.0 |

  @apply-a-managed-upgrade
  Scenario Outline: Unsupported old configuration is refused before upgrade writes
    Given  a clean checkpointed project with a 0.x version marker and "<configuration defect>"
    When  the consumer runs `givn upgrade`
    Then  the command refuses before changing project files because of "<configuration defect>", not the version marker
    And  the diagnostic identifies the malformed configuration or recognized retired keys
    And  the diagnostic explains manual communication_policy, challenge, and skills-only transitions where applicable
    And  no automatic configuration migration or compatibility alias is applied
    Examples:
      | configuration defect |
      | malformed configuration |
      | the recognized retired absolute_mode key |
      | the recognized retired grill_me key |
      | the recognized retired grill_with_docs key |

  @apply-a-managed-upgrade
  Scenario Outline: Upgrade failures report actual partial state and operator recovery
    Given  a clean checkpointed "<backend>" project ready for a managed upgrade
    And  the upgrade will fail "<failure>"
    When  the consumer runs `givn upgrade`
    Then  the command exits nonzero and names the failed operation
    And  the report identifies the checkpoint, completed writes, and revisions actually published
    And  the report accurately describes any best-effort restoration and the remaining partial state
    And  the report distinguishes pending work from completed upgrade work
    And  recovery guidance offers operator-controlled restoration or fix-forward from that checkpoint
    And  no automatic version-control reset, clean, or history rewrite is performed
    And  no complete rollback is promised
    Examples:
      | backend | failure |
      | Git | after some managed files are written |
      | Git | while publishing the managed revision |
      | Fossil | after some managed files are written |
      | Fossil | while publishing the managed revision |

  @apply-a-managed-upgrade
  Scenario Outline: Future supported migration routes remain reviewable
    Given  a clean checkpointed disposable "<backend>" project fixture with a synthetic 1.0 version marker
    And  a synthetic migration route to the simulated later target 1.1 is supplied through the test-only seam
    When  the consumer requests the managed upgrade to the simulated target
    Then  version detection selects the synthetic route from 1.0 to 1.1
    And  the migration plan records the actual managed upgrade revision
    And  no removed pre-1.0 route is needed to produce the plan
    Examples:
      | backend |
      | Git |
      | Fossil |

  @apply-a-managed-upgrade
  Scenario: Upgrade reconciles only recognizable generated commands to skills
    Given  a clean checkpointed project with recognizable generated legacy commands beside customized command replacements
    When  the consumer runs `givn upgrade`
    Then  recognizable generated legacy commands are removed without generating new command files
    And  selected targets receive thin skills directing authors to served instructions
    And  customized commands remain byte-for-byte unchanged and are reported for manual transition
