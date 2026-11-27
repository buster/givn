@clean-room-preparation
Feature: Clean-room preparation

  @clean-room-preparation
  Scenario: Preparation refuses when the declaration is missing
    Given  a Git project with ise enabled and no declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "givn/specification.yaml"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a malformed declaration
    Given  a Git project with ise enabled and a malformed declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "givn/specification.yaml" and the loader's reason
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses while the ise addon is off
    Given  a Git project with a declaration and ise off
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "ise"
    And  the declaration is not read
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses while architecture principles are off
    Given  a Git project with ise on, architecture principles off, and a malformed declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "architecture_principles"
    And  the declaration is not read
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a dirty checkout and names every dirty path
    Given  a Git project with ise enabled, a declaration, and one modified tracked file and one untracked file
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names the modified tracked file and the untracked file
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a project with no version-control backend
    Given  a directory with ise enabled, a declaration, and no version-control backend
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names the missing backend
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a project root nested inside a Git worktree
    Given  a Git worktree whose project root is nested inside the worktree top level
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names the nested project root
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a checkout carrying both Git and Fossil markers
    Given  a project root with both a Git worktree marker and a Fossil checkout marker
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names the ambiguous version control
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a path outside every source root
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer rebuilds the component path "docs/reference.md" into a spec copy
    Then  the command fails and exits 1, naming "docs/reference.md" and "rebuild-all"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses an omitted target
    Given  a Git project with ise enabled and a declaration
    When  the developer runs rebuild-all from "HEAD" with no target
    Then  the command fails and names "--into"
    And  the command exits 2
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a target inside the checkout
    Given  a Git project with ise enabled and a declaration
    When  the developer runs rebuild-all into a directory inside the checkout
    Then  the command fails and names the target
    And  the checkout holds no spec copy directory and no manifest

  @clean-room-preparation
  Scenario: Preparation refuses a repository link in the copy set
    Given  a Git project with ise enabled and a declaration whose "structure" role names "vendor/lib"
    And  a committed revision with a repository link at "vendor/lib"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "vendor/lib"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a nested repository marker in the copy set
    Given  a Git project with ise enabled and a declaration whose "structure" role names "nested/_FOSSIL_"
    And  a committed revision with "nested/_FOSSIL_"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "nested/_FOSSIL_"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation expands a directory path to the tracked files beneath it
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/keys/parser.rs", "src/keys/mod.rs", and "src/other.rs"
    When  the developer rebuilds the component path "src/keys" into a spec copy
    Then  the command succeeds and prints "Source code removed: 2" and "Files beside removed paths: 0"
    And  the spec copy omits "src/keys/parser.rs" and "src/keys/mod.rs"
    And  the spec copy holds "src/other.rs"

  @clean-room-preparation
  Scenario: Preparation collapses a repeated path
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/one.rs" and "src/two.rs"
    When  the developer rebuilds the component paths "src/one.rs" and "src/one.rs" into a spec copy
    Then  the command succeeds and prints "Source code removed: 1"
    And  the manifest records one removed path

  @clean-room-preparation
  Scenario: Preparation does not expand a glob-shaped path
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/one.rs" and "src/nested/two.rs"
    When  the developer rebuilds the component path "src/**" into a spec copy
    Then  the command succeeds and prints "Source code removed: 0"
    And  the spec copy holds "src/one.rs" and "src/nested/two.rs"

  @clean-room-preparation
  Scenario: Preparation preserves the executable bit and the symlink target
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with an executable script "src/run.sh" and a symlink "src/link" to "run.sh", a Specification file through the "structure" role
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds
    And  the spec copy's "src/run.sh" is executable
    And  the spec copy's "src/link" is a symlink to "run.sh"

  @clean-room-preparation
  Scenario: A named path absent from the revision removes nothing
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/keys/parser.rs"
    When  the developer rebuilds the component path "src/keys/parsr.rs" into a spec copy
    Then  the command succeeds and prints "Source code removed: 0"
    And  the spec copy holds "src/keys/parser.rs"

  @clean-room-preparation
  Scenario: Preparation reads an explicit revision and reports it
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/kept.rs", a Specification file through the "structure" role, and a later revision adding "src/later.rs"
    When  the developer rebuilds the whole product from the first revision into a spec copy
    Then  the command succeeds and prints "Backend: git" and "Revision:" with the first revision identifier
    And  the spec copy holds "src/kept.rs"
    And  the spec copy does not hold "src/later.rs"

  @clean-room-preparation
  Scenario: Preparation refuses an absent revision
    Given  a Git project with ise enabled and a declaration
    When  the developer rebuilds the whole product from "0000000000000000000000000000000000000000" into a spec copy
    Then  the command fails and names the revision
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses when a revision blob cannot be read
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision whose tracked blob is absent from the object store
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names the unreadable path and the revision
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation reads a Fossil revision and preserves an executable file and a symlink
    Given  a Fossil checkout with ise enabled, allow-symlinks on, and a declaration with one source root "src/**"
    And  a committed revision with a Specification file, a Source code kept file, a removed source file, an executable script, and a symlink to it
    When  the developer rebuilds the component path "src/keys/parser.rs" into a spec copy
    Then  the command succeeds and prints "Backend: fossil"
    And  the spec copy holds the Specification file and the Source code kept file
    And  the spec copy omits the removed source file
    And  the spec copy's executable script is executable
    And  the spec copy's symlink is a symlink to its target

  @clean-room-preparation
  Scenario: Preparation refuses a directory that contains the declaration
    Given  a Git project with ise enabled and a declaration whose source root is "givn/**"
    And  a committed revision with "givn/notes.md"
    When  the developer rebuilds the component path "givn" into a spec copy
    Then  the command fails and names "givn/specification.yaml" as a Specification file
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a revision that does not record the declaration
    Given  a Git project with ise enabled whose first revision predates the declaration
    When  the developer rebuilds the whole product from the first revision into a spec copy
    Then  the command fails and names "givn/specification.yaml", the revision, and "does not record"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a malformed declaration at the selected revision
    Given  a Git project with ise enabled, a malformed declaration in its first revision, and a valid declaration at "HEAD"
    When  the developer rebuilds the whole product from the first revision into a spec copy
    Then  the command fails and names "givn/specification.yaml" and the loader's reason
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a declaration that is not UTF-8
    Given  a Git project with ise enabled, a non-UTF-8 declaration in its first revision, and a valid declaration at "HEAD"
    When  the developer rebuilds the whole product from the first revision into a spec copy
    Then  the command fails and names "givn/specification.yaml" and the UTF-8 error
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: A role that matches the declaration counts it once
    Given  a Git project with ise enabled and a declaration whose "contracts" role names "givn/**"
    And  a committed revision with "givn/specs/a.md"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 2"
    And  the spec copy holds "givn/specification.yaml"

  @clean-room-preparation
  Scenario: Preparation refuses when the manifest cannot be written
    Given  a Git project with ise enabled and a declaration
    And  a manifest path whose parent directory is a regular file
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with that manifest
    Then  the command fails and names the manifest path
    And  the spec copy directory it created is removed

  @clean-room-preparation
  Scenario: Preparation refuses a declaration the revision records but cannot read
    Given  a Git project with ise enabled and a declaration
    And  the committed declaration's blob is absent from the object store
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "givn/specification.yaml", the revision, and "returned no blob"
    And  the spec copy path and the manifest path remain absent

  @e2e @clean-room-preparation
  Scenario: Developer rebuilds the whole product from its Specification
    Given  a Git project with ise enabled and a declaration whose "behavior" role names "givn/specs/**", whose "e2e" role names "tests/e2e", and whose source root is "src/**"
    And  a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "tests/e2e/steps/demo.rs", "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", and "notes.md"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds and prints "Backend: git", "Specification files: 2", "Source code kept: 0", "Source code removed: 3", "Outside the spec copy: 1", and "Files beside removed paths: 0"
    And  the report prints a "Revision:" line holding the resolved 40-hex commit
    And  the report prints "End-to-end tests: not copied (2 files)"
    And  the report prints a "Specification checksum:" line and an "End-to-end checksum:" line of 12 lowercase hex characters each
    And  the report prints the copy directory and the manifest path
    And  the spec copy holds "givn/specs/demo.feature" and "givn/specification.yaml"
    And  the spec copy omits "src/keys/parser.rs", "src/keys/mod.rs", "src/other.rs", "notes.md", "tests/e2e/Cargo.toml", and "tests/e2e/steps/demo.rs"
    And  the manifest records the scope, the revision, the removed paths, the five counts, the checksums, the copy directory, and the Givn version

  @clean-room-preparation
  Scenario: Developer rebuilds one component and leaves its neighbours
    Given  a Git project with ise enabled and a declaration whose "behavior" role names "givn/specs/**", whose "e2e" role names "tests/e2e", and whose source root is "src/**"
    And  a committed revision with "givn/specs/demo.feature", "tests/e2e/Cargo.toml", "src/keys/parser.rs", "src/keys/mod.rs", and "src/other.rs"
    When  the developer rebuilds the component path "src/keys/parser.rs" into a spec copy with a manifest
    Then  the command succeeds and prints "Backend: git", "Specification files: 2", "Source code kept: 2", "Source code removed: 1", "Outside the spec copy: 0", and "Files beside removed paths: 1"
    And  the spec copy holds "src/keys/mod.rs" and "src/other.rs"
    And  the spec copy omits "src/keys/parser.rs" and "tests/e2e/Cargo.toml"
    And  the manifest's removed paths are:
      | src/keys/parser.rs |

  @clean-room-preparation
  Scenario: Preparation refuses a Specification path missing at the revision
    Given  a Git project with ise enabled and a declaration whose "contracts" role names "docs/contract.md"
    And  a committed revision without "docs/contract.md"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and names "docs/contract.md" and the revision
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a path matched by a Specification role
    Given  a Git project with ise enabled and a declaration whose "behavior" role names "givn/specs/**"
    When  the developer rebuilds the component path "givn/specs/demo.feature" into a spec copy
    Then  the command fails and names "givn/specs/demo.feature" and the "behavior" role
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a directory path that contains a Specification path
    Given  a Git project with ise enabled, a declaration whose "behavior" role names "givn/specs/**" and whose source root is "givn/**"
    And  a committed revision with "givn/specs/demo.feature"
    When  the developer rebuilds the component path "givn/specs" into a spec copy
    Then  the command fails and names "givn/specs/demo.feature" and the "behavior" role
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation counts the files left in a removed path's directory
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision where "src/keys/parser.rs" sits beside "src/keys/mod.rs" and "src/keys/tests.rs"
    When  the developer rebuilds the component path "src/keys/parser.rs" into a spec copy
    Then  the command succeeds and prints "Files beside removed paths: 2"
    And  the spec copy holds "src/keys/mod.rs" and "src/keys/tests.rs"

  @clean-room-preparation
  Scenario: Preparation refuses an existing spec copy directory
    Given  a Git project with ise enabled and a declaration
    And  a target directory that already exists
    When  the developer runs rebuild-all into that target
    Then  the command fails and names the existing target
    And  the existing directory is left unchanged

  @clean-room-preparation
  Scenario: Preparation refuses an existing spec copy file
    Given  a Git project with ise enabled and a declaration
    And  a target path that exists as a regular file
    When  the developer runs rebuild-all into that target
    Then  the command fails and names the existing target
    And  the existing file is left unchanged

  @clean-room-preparation
  Scenario: Preparation refuses a spec copy target symlinked into the checkout
    Given  a Git project with ise enabled and a declaration
    And  a symlink outside the checkout that points into the checkout
    When  the developer runs rebuild-all through the symlinked target
    Then  the command fails and names the target
    And  the checkout holds no spec copy directory

  @clean-room-preparation
  Scenario: Preparation writes no version-control metadata into the spec copy
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/kept.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds
    And  the spec copy holds no ".git" entry and no ".fslckout" entry

  @clean-room-preparation
  Scenario: Preparation partitions with the declaration at the selected revision
    Given  a Git project with ise enabled and a declaration whose "structure" role names "docs/glossary.md"
    And  a committed revision with "docs/glossary.md" and "src/one.rs"
    And  a later revision that rewrites the declaration without the "structure" entry
    When  the developer rebuilds the whole product from the first revision into a spec copy
    Then  the command succeeds
    And  the spec copy holds "docs/glossary.md"
    And  the spec copy's "givn/specification.yaml" is the declaration committed at the selected revision

  @clean-room-preparation
  Scenario: A component rebuild refuses an empty path list
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer runs "givn ise rebuild-component" with no path and a target
    Then  the command exits 2 and names the missing path list
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation writes no manifest when no manifest path is given
    Given  a Git project with ise enabled and a declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy without a manifest
    Then  the command succeeds and prints no "Manifest:" line
    And  no manifest file is written
    And  the spec copy holds the declaration "givn/specification.yaml"

  @clean-room-preparation
  Scenario: A dry run writes no spec copy and no manifest
    Given  a Git project with ise enabled and a declaration
    When  the developer runs a dry run of the whole product from "HEAD" into a spec copy with a manifest
    Then  the dry run prints "Copy directory (would write):" and "Manifest (not written):"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: A dry run without a manifest prints no manifest line
    Given  a Git project with ise enabled and a declaration
    When  the developer runs a dry run of the whole product from "HEAD" into a spec copy without a manifest
    Then  the dry run prints "Copy directory (would write):" and no "Manifest (not written):" line
    And  the spec copy path remains absent

  @clean-room-preparation
  Scenario: Preparation leaves an ignored file out of the spec copy
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/kept.rs", a Specification file through the "structure" role
    And  an ignored file in the working tree
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds
    And  the spec copy holds "src/kept.rs"
    And  the spec copy does not hold the ignored file

  @clean-room-preparation
  Scenario: A whole-product rebuild removes every tracked file under the source roots
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/one.rs" and "src/nested/two.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 1", "Source code kept: 0", and "Source code removed: 2"
    And  the spec copy holds "givn/specification.yaml"
    And  the spec copy omits "src/one.rs" and "src/nested/two.rs"

  @clean-room-preparation
  Scenario: A whole-product rebuild keeps a Specification file under a source root
    Given  a Git project with ise enabled and a declaration whose "structure" role names "src/kept.rs" and "src/nested/notes.md" and whose source root is "src/**"
    And  a committed revision with "src/kept.rs", "src/nested/notes.md", and "src/other.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 3", "Source code kept: 0", "Source code removed: 1", and "Files beside removed paths: 1"
    And  the spec copy holds "src/kept.rs" and "src/nested/notes.md"
    And  the spec copy omits "src/other.rs"

  @clean-room-preparation
  Scenario: The spec copy carries the declaration when no role names it
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds
    And  the spec copy holds "givn/specification.yaml"

  @clean-room-preparation
  Scenario: The manifest lists the declaration among the Specification files
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds
    And  the manifest lists "givn/specification.yaml" among the Specification files

  @clean-room-preparation
  Scenario: The manifest lists the End-to-end tests it does not copy
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "tests/e2e"
    And  a committed revision with "tests/e2e/Cargo.toml" and "tests/e2e/steps/demo.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds and prints "End-to-end tests: not copied (2 files)"
    And  the manifest's End-to-end files hold "tests/e2e/Cargo.toml" and "tests/e2e/steps/demo.rs"
    And  the spec copy omits "tests/e2e/Cargo.toml" and "tests/e2e/steps/demo.rs"

  @clean-room-preparation
  Scenario: A declaration with no End-to-end role copies every Specification file
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds and prints "End-to-end tests: not copied (0 files)"
    And  the manifest's End-to-end files are empty

  @clean-room-preparation
  Scenario: An End-to-end role that covers the declaration keeps it among the Specification files
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "givn/**"
    And  a committed revision with "givn/specs/a.md" and "src/kept.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 1" and "End-to-end tests: not copied (1 file)"
    And  the spec copy holds "givn/specification.yaml"
    And  the spec copy omits "givn/specs/a.md"

  @clean-room-preparation
  Scenario: A project-root End-to-end entry covers the whole tracked tree
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "." and whose source root is "src/**"
    And  a committed revision with "src/kept.rs" and "notes.md"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 1", "Source code kept: 0", "Source code removed: 0", "Outside the spec copy: 0", and "End-to-end tests: not copied (2 files)"
    And  the spec copy holds "givn/specification.yaml"
    And  the spec copy omits "src/kept.rs" and "notes.md"

  @clean-room-preparation
  Scenario: An End-to-end entry that overlaps a source root wins over the removal
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "tests/e2e" and whose source root is "tests/**"
    And  a committed revision with "tests/e2e/runner.rs" and "tests/unit/helper.rs"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 1", "Source code kept: 0", "Source code removed: 1", "Outside the spec copy: 0", and "End-to-end tests: not copied (1 file)"
    And  the spec copy holds "givn/specification.yaml"
    And  the spec copy omits "tests/e2e/runner.rs" and "tests/unit/helper.rs"

  @clean-room-preparation
  Scenario: A whole-product rebuild keeps the declaration under a source root
    Given  a Git project with ise enabled and a declaration whose source root is "givn/**"
    And  a committed revision with "givn/notes.md"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 1" and "Source code removed: 1"
    And  the spec copy holds "givn/specification.yaml"
    And  the spec copy omits "givn/notes.md"

  @clean-room-preparation
  Scenario: Preparation refuses a manifest inside the spec copy
    Given  a Git project with ise enabled and a declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest inside the spec copy
    Then  the command fails and names the manifest
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses a manifest inside the checkout
    Given  a Git project with ise enabled and a declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest inside the checkout
    Then  the command fails and names the manifest
    And  the checkout holds no manifest

  @clean-room-preparation
  Scenario: Preparation refuses a manifest that is an ancestor of the spec copy
    Given  a Git project with ise enabled and a declaration
    When  the developer rebuilds the whole product from "HEAD" into a spec copy whose manifest path is an ancestor of the spec copy
    Then  the command fails and names the manifest
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses an existing manifest
    Given  a Git project with ise enabled and a declaration
    And  a manifest file that already exists
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with that manifest
    Then  the command fails and names the existing manifest
    And  the existing manifest is left unchanged

  @clean-room-preparation
  Scenario: Preparation refuses a symlink whose target resolves outside the spec copy
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with a symlink "src/out" to "../../outside"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and exits 1, naming "src/out" and "../../outside"
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation refuses an absolute symlink target
    Given  a Git project with ise enabled and a declaration with one source root "src/**"
    And  a committed revision with "src/run.sh" and a symlink "src/abs" to an absolute target inside the spec copy
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command fails and exits 1, naming "src/abs" and its absolute target
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation records the canonical checksums for a pinned fixture
    Given  the pinned canonical checksum fixture
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds
    And  the manifest's scenario checksum for "givn/specs/demo.feature" is "791f7b34866c710ad541e06fb1d354d3ed4fedd7c46404465183d0a68d88006c"
    And  the manifest's specification checksum is "35444598f1c4238a9db8c2cdea5b5cf5391604a3608aebd6fe2e9f91f105b917"
    And  the manifest's end-to-end checksum is "00580162d8a019161c476e2772c5a85e7506fc0c254cbc28424299343b479b07"
    And  the manifest's End-to-end files hold "givn/specs/demo.feature" and "tests/e2e/Cargo.toml"
    And  the manifest's specification checksums are:
      | path | checksum |
      | givn/specification.yaml | 592befb4303513eda36dd47519487011745c9710e7a56703677e630045c52e12 |
      | givn/specs/demo.feature | 95b647b4f91f9ceb15887db09ecfa08caaff14d5fb5228b25cf1aeab2889a540 |
      | docs/glossary.md | 42383f180e9413e0a02ff8ed07df04fd639d9b7274f1a8932d518656616ab40d |
      | docs/reference.md | 910604b25ba07632849d98cd3131ce26f016156a57082fdfce816d3a70714277 |
      | docs/arc42/02-architecture-constraints.md | 645638aef3715e807e8831dbb90b65b9b6183e6cbd1d8de4df16f7ab5abdd1c6 |
    And  the manifest's end-to-end checksums are:
      | path | checksum |
      | givn/specs/demo.feature | 95b647b4f91f9ceb15887db09ecfa08caaff14d5fb5228b25cf1aeab2889a540 |
      | tests/e2e/Cargo.toml | 7fb79f1e5eaf0e42fd94ebcd1044ac2fb64412680ea8ebbd0fb4fb443a7aea18 |
    And  the manifest's scenario checksums are:
      | path | checksum |
      | givn/specs/demo.feature | 791f7b34866c710ad541e06fb1d354d3ed4fedd7c46404465183d0a68d88006c |

  @clean-room-preparation
  Scenario: Preparation records the canonical checksums for a richer feature
    Given  the pinned richer checksum fixture
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds
    And  the manifest's scenario checksum for "givn/specs/demo.feature" is "d5e995848ed1491e131b5f13504c3295e79f23646909ff5d4fb3b548d03e0520"
    And  the manifest's specification checksum is "c1102713bbf0a24482eaf6e3b8ff5b787ffa32d9ff414dc1c9434cba7a192e0e"
    And  the manifest's end-to-end checksum is "2c16dfedf4203acf3a1d2f67340b4897d5e1c04c26a8d212fa95f1b8e0eb2844"

  @clean-room-preparation
  Scenario: Preparation checksums a repository link in the End-to-end set
    Given  a Git project with ise enabled and a declaration whose "e2e" role names "tests/e2e"
    And  a committed revision with "tests/e2e/Cargo.toml" and a repository link at "tests/e2e/vendor/dep"
    When  the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest
    Then  the command succeeds and prints "End-to-end tests: not copied (2 files)"
    And  the spec copy omits "tests/e2e/vendor/dep"
    And  the manifest's end-to-end checksums hold a 64-hex checksum for "tests/e2e/vendor/dep"

  @clean-room-preparation
  Scenario: The retired command names and flags exit 2
    Given  a Git project with ise enabled and a declaration
    When  the developer runs the retired command "givn ise replace" with a target
    Then  the command exits 2 and names the unknown command
    And  the spec copy path remains absent
    When  the developer runs the retired command "givn ise verify" with a manifest
    Then  the command exits 2 and names the unknown command
    When  the developer runs "givn ise rebuild-all --full" with a target
    Then  the command exits 2 and names the unknown flag
    And  the spec copy path remains absent

  @clean-room-preparation
  Scenario: Preparation refuses a component path that names the declaration
    Given  a Git project with ise enabled and a declaration whose source root is "givn/**"
    And  a committed revision with "src/kept.rs"
    When  the developer rebuilds the component path "givn/specification.yaml" into a spec copy
    Then  the command fails and names "givn/specification.yaml" as a Specification file
    And  the spec copy path and the manifest path remain absent

  @clean-room-preparation
  Scenario: Preparation carries every declared build input into the spec copy
    Given  a Git project with ise enabled and a declaration whose build inputs sit under the four content roles
    And  a committed revision with those build inputs, one source file, and an End-to-end manifest
    When  the developer rebuilds the whole product from "HEAD" into a spec copy
    Then  the command succeeds and prints "Specification files: 7"
    And  the spec copy holds "Cargo.toml" and "Cargo.lock"
    And  the spec copy holds "rust-toolchain.toml" and "assets/templates/glossary.md"
    And  the spec copy holds "assets/instructions/ise.md" and "givn/specs/demo.feature"
    And  the spec copy omits "src/main.rs" and "e2e/Cargo.toml"

  @clean-room-preparation
  Scenario: Preparation writes a runnable verification contract
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    Then  the command succeeds
    And  the spec copy holds "givn/ise-verification.yaml"
    And  the verification contract declares 10 component execution units and 1 End-to-end execution unit
