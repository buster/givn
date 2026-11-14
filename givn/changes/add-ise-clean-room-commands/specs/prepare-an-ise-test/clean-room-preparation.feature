@givn.delta @clean-room-preparation

Feature: Clean-room preparation

  @givn.added @e2e @wip
  Scenario: Developer prepares a clean-room trial from a recorded revision
    Given a Git project with ise enabled and a declaration that retains "givn/specs/**", declares "tests/e2e", and replaces "src/**"
    And a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", and "notes.md"
    When the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs" and a receipt
    Then preparation succeeds
    And the report prints "Backend: git" and a "Revision:" line holding the resolved 40-hex commit
    And the report prints "Retained: 3", "Kept source: 2", "Replaced: 1", "Outside the declaration: 1", and "Not replaced: 1"
    And the report abbreviates the retained digest and the e2e digest to 12 lowercase hex characters and an ellipsis
    And the report prints the written trial path and the receipt path
    And the trial holds the retained and kept source files, including "src/keys/mod.rs"
    And the trial omits "src/keys/parser.rs" and "notes.md"
    And the receipt records the replacement revision, the backend, the replaced paths, the counts, the replaced digest, the written path, and the Givn version

  @givn.added
  Scenario: Preparation refuses when the declaration is missing
    Given a Git project with ise enabled and no declaration
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "givn/specification.yaml"
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses a malformed declaration
    Given a Git project with ise enabled and a malformed declaration
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "givn/specification.yaml" and the loader's reason
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses while the ise addon is off
    Given a Git project with a declaration and ise off
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "ise"
    And the declaration is not read
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses while architecture principles are off
    Given a Git project with ise on, architecture principles off, and a declaration
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "architecture_principles"
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses a dirty checkout and names every dirty path
    Given a Git project with ise enabled, a declaration, and one modified tracked file and one untracked file
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names the modified tracked file and the untracked file
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses a project with no version-control backend
    Given a directory with ise enabled, a declaration, and no version-control backend
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names the missing backend
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses a project root nested inside a Git worktree
    Given a Git worktree whose project root is nested inside the worktree top level
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names the nested project root
    And the trial path and the receipt path remain absent

  @givn.added
  Scenario: Preparation refuses a checkout carrying both Git and Fossil markers
    Given a project root with both a Git worktree marker and a Fossil checkout marker
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names the ambiguous version control
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a retained path missing at the revision
    Given a Git project with ise enabled and a declaration whose "contracts" role names "docs/contract.md"
    And a committed revision without "docs/contract.md"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "docs/contract.md" and the revision
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a path outside every source root
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    When the developer prepares a trial from "HEAD" with the path "docs/reference.md"
    Then preparation fails and names "docs/reference.md"
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a path matched by a retained role
    Given a Git project with ise enabled and a declaration that retains "givn/specs/**"
    When the developer prepares a trial from "HEAD" with the path "givn/specs/demo.feature"
    Then preparation fails and names "givn/specs/demo.feature" and the "behavior" role
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a directory path that contains a retained path
    Given a Git project with ise enabled, a declaration that retains "givn/specs/**", and one source root "givn/**"
    And a committed revision with "givn/specs/demo.feature"
    When the developer prepares a trial from "HEAD" with the directory path "givn/specs"
    Then preparation fails and names "givn/specs/demo.feature" and the "behavior" role
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses when neither replacement form is given
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial from "HEAD" with neither a full replacement nor a path
    Then preparation fails and names "--full" and "--path"
    And the command exits 2
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses when both replacement forms are given
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial from "HEAD" with both a full replacement and a path
    Then preparation fails and names "--full" and "--path" as mutually exclusive
    And the command exits 2
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses an omitted target
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial from "HEAD" with a full replacement and no target
    Then preparation fails and names "--into"
    And the command exits 2
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a target inside the checkout
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial into a directory inside the checkout
    Then preparation fails and names the target
    And the checkout holds no trial directory and no receipt

  @givn.added @wip
  Scenario: Preparation refuses an existing target directory
    Given a Git project with ise enabled and a declaration
    And a target directory that already exists
    When the developer prepares a trial into that target
    Then preparation fails and names the existing target
    And the existing directory is left unchanged

  @givn.added @wip
  Scenario: Preparation refuses an existing target file
    Given a Git project with ise enabled and a declaration
    And a target path that exists as a regular file
    When the developer prepares a trial into that target
    Then preparation fails and names the existing target
    And the existing file is left unchanged

  @givn.added @wip
  Scenario: Preparation refuses a receipt inside the trial directory
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial with a receipt inside the trial directory
    Then preparation fails and names the receipt
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a receipt inside the checkout
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial with a receipt inside the checkout
    Then preparation fails and names the receipt
    And the checkout holds no receipt

  @givn.added @wip
  Scenario: Preparation refuses a receipt that is an ancestor of the trial directory
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial whose receipt path is an ancestor of the trial directory
    Then preparation fails and names the receipt
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a symlinked target that resolves inside the checkout
    Given a Git project with ise enabled and a declaration
    And a symlink outside the checkout that points into the checkout
    When the developer prepares a trial through the symlinked target
    Then preparation fails and names the target
    And the checkout holds no trial directory

  @givn.added @wip
  Scenario: Preparation refuses an existing receipt
    Given a Git project with ise enabled and a declaration
    And a receipt file that already exists
    When the developer prepares a trial with that receipt
    Then preparation fails and names the existing receipt
    And the existing receipt is left unchanged

  @givn.added @wip
  Scenario: Preparation refuses a repository link in the copy set
    Given a Git project with ise enabled and a declaration that retains "vendor/lib"
    And a committed revision with a repository link at "vendor/lib"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "vendor/lib"
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses a nested repository marker in the copy set
    Given a Git project with ise enabled and a declaration that retains "nested/_FOSSIL_"
    And a committed revision with "nested/_FOSSIL_"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names "nested/_FOSSIL_"
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation counts the files left in a replaced path's directory
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision where "src/keys/parser.rs" sits beside "src/keys/mod.rs" and "src/keys/tests.rs"
    When the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs"
    Then preparation succeeds and prints "Not replaced: 2"
    And the trial holds "src/keys/mod.rs" and "src/keys/tests.rs"

  @givn.added @wip
  Scenario: Preparation expands a directory path to the tracked files beneath it
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/keys/parser.rs", "src/keys/mod.rs", and "src/other.rs"
    When the developer prepares a trial from "HEAD" with the directory path "src/keys"
    Then preparation succeeds and prints "Replaced: 2" and "Not replaced: 0"
    And the trial omits "src/keys/parser.rs" and "src/keys/mod.rs"
    And the trial holds "src/other.rs"

  @givn.added @wip
  Scenario: A full replacement removes every tracked file under the source roots
    Given a Git project with ise enabled, a declaration that retains "givn/specs/**", and one source root "src/**"
    And a committed revision with "src/one.rs", "src/nested/two.rs", and "givn/specs/demo.feature"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation succeeds and prints "Replaced: 2" and "Not replaced: 0"
    And the trial holds "givn/specs/demo.feature"
    And the trial holds no file under "src"

  @givn.added @wip
  Scenario: Preparation collapses a repeated path
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/one.rs" and "src/two.rs"
    When the developer prepares a trial from "HEAD" with the paths "src/one.rs" and "src/one.rs"
    Then preparation succeeds and prints "Replaced: 1"
    And the receipt records one named path and one replaced path

  @givn.added @wip
  Scenario: Preparation does not expand a glob-shaped path
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/one.rs" and "src/nested/two.rs"
    When the developer prepares a trial from "HEAD" with the path "src/**"
    Then preparation succeeds and prints "Replaced: 0"
    And the trial holds "src/one.rs" and "src/nested/two.rs"

  @givn.added @wip
  Scenario: Preparation reads the revision and leaves an ignored file out of the trial
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/kept.rs"
    And an ignored file in the working tree
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation succeeds
    And the trial holds "src/kept.rs"
    And the trial does not hold the ignored file

  @givn.added @wip
  Scenario: Preparation preserves the executable bit and the symlink target
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with an executable script "src/run.sh" and a symlink "src/link" to "run.sh"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation succeeds
    And the trial's "src/run.sh" is executable
    And the trial's "src/link" is a symlink to "run.sh"

  @givn.added @wip
  Scenario: Preparation writes no version-control metadata into the trial
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/kept.rs"
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation succeeds
    And the trial holds no ".git" entry and no ".fslckout" entry

  @givn.added @wip
  Scenario: Preparation prints the trial record to stdout when no receipt path is given
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial from "HEAD" with a full replacement and no receipt
    Then preparation succeeds and prints the trial record to stdout
    And no record file is written
    And the report holds no "Receipt path" line

  @givn.added @wip
  Scenario: A dry run performs every read and writes no trial or record
    Given a Git project with ise enabled and a declaration
    When the developer runs a dry run from "HEAD" with a full replacement and a receipt
    Then the dry run prints the ten fixed report lines, the receipt path line, and a "Would write:" block
    And the "Would write:" block lists the sorted paths the trial would hold
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: A named path absent from the revision removes nothing
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/keys/parser.rs"
    When the developer prepares a trial from "HEAD" with the path "src/keys/parsr.rs"
    Then preparation succeeds and prints "Replaced: 0"
    And the trial holds "src/keys/parser.rs"

  @givn.added @wip
  Scenario: Preparation reads an explicit revision and reports it
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision with "src/kept.rs" and a later revision adding "src/later.rs"
    When the developer prepares a trial from the first revision with a full replacement
    Then preparation succeeds and prints "Revision:" with the first revision identifier
    And the trial holds "src/kept.rs"
    And the trial does not hold "src/later.rs"

  @givn.added
  Scenario: Preparation refuses an absent revision
    Given a Git project with ise enabled and a declaration
    When the developer prepares a trial from "0000000000000000000000000000000000000000" with a full replacement
    Then preparation fails and names the revision
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses when a revision blob cannot be read
    Given a Git project with ise enabled and a declaration with one source root "src/**"
    And a committed revision whose tracked blob is absent from the object store
    When the developer prepares a trial from "HEAD" with a full replacement
    Then preparation fails and names the unreadable path and the revision
    And the trial path and the receipt path remain absent

  @givn.added @wip
  Scenario: Preparation refuses when the record cannot be written
    Given a Git project with ise enabled and a declaration
    And a receipt path whose parent directory is a regular file
    When the developer prepares a trial from "HEAD" with a full replacement and that receipt
    Then preparation fails and names the receipt path
    And the trial directory it created is removed

  @givn.added @wip
  Scenario: Preparation reads a Fossil revision and preserves an executable file and a symlink
    Given a Fossil checkout with ise enabled, allow-symlinks on, and a declaration with one source root "src/**"
    And a committed revision with a retained file, a kept source file, a replaced source file, an executable script, and a symlink to it
    When the developer prepares a trial from "HEAD" with the path "src/keys/parser.rs"
    Then preparation succeeds and prints "Backend: fossil"
    And the trial holds the retained file and the kept source file
    And the trial omits the replaced source file
    And the trial's executable script is executable
    And the trial's symlink is a symlink to its target
