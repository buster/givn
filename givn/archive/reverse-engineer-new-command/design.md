# Design — reverse-engineer-new-command

> **Amendment (post-archive):** The original test harness described below used a
> parallel `#[test]` file (`tests/new_command.rs`) with `verify.command:
> "cargo test"`. That approach was superseded so the `.feature` file is the
> **executable** spec, per givn's executable-spec invariant. The repo now uses
> `cucumber-rs`: `verify.command` is `cargo test --test features_runner`, the
> runner binary `tests/features_runner.rs` loads `givn/specs/**/*.feature`, and
> step definitions live in `tests/steps/mod.rs`. `tests/new_command.rs` was
> deleted. The black-box boundary (CLI subprocess in a temp dir) and the
> seam strategy (none) below are unchanged; only the test-execution mechanism
> changed. References to `tests/new_command.rs` and `cargo test` in the
> sections that follow describe the original (now-replaced) mechanism.

Characterization design for the `givn new <id>` capability. This is the HOW
layer for **testing strategy and seams** — no new production features are
built.

## Black-box boundary

The boundary is the CLI surface `givn new <id>`, exercised by invoking the
compiled `givn` binary as a **subprocess** and observing:

- the process exit code,
- the stdout stream,
- the stderr stream,
- the filesystem state inside a controlled working directory (directory tree
  created and the byte contents of every file written).

The working directory is a per-test temp directory; the binary is launched
with that directory as its current directory so that the command's
"project root" (derived from the current directory) is the temp directory.

Examples of inputs at this boundary:

- `givn new add-user-auth` (valid id, initialised project)
- `givn new ""` (empty id)
- `givn new HasUppercase` (invalid id)
- `givn new -- -foo` (id passed after the argument separator)
- `givn new dup` against a project where `givn/changes/dup/` already exists
- `givn new x` against a project where `givn/` does not exist
- `givn new x` against a project that has a custom
  `givn/artifacts/proposal/proposal.md` override

## Seam strategy

**No testability refactor is required.** The command reads its project root
from the process current directory and performs all filesystem operations
relative to that root. By launching the binary as a subprocess with a temp
directory as its current directory, every scenario's inputs are controlled
purely from the outside:

- "givn/ initialised" → run `givn init` in the temp dir first (the fixture
  MUST run `givn init` rather than hand-creating a minimal `givn/` subtree,
  so that scaffolding logic is not duplicated in test setup — see Test
  harness below).
- "givn/ absent" → use a bare temp dir (no `givn init`).
- "change already exists" → pre-create `givn/changes/<id>/` in the temp dir.
- "project override present" → write
  `givn/artifacts/proposal/proposal.md` in the temp dir before invoking.
- "built-in template" → read `assets/templates/proposal.md` at test compile
  time via `include_str!` to assert byte-for-byte equality.

Because the seam is the OS process boundary plus the current directory, no
production code changes are needed to characterise any scenario. This is the
ideal arrangement for reverse-engineering: behaviour is preserved exactly
because it is never touched.

### Testability refactor constraints

None applied. If a future scenario turns out to be unreachable through the
process boundary (none is currently expected), the **only** permitted
refactors are: extract a pure function from a side-effectful one, dependency
injection of the project root, or making a function public. Renames, logic
changes, error-message edits, bug fixes, and control-flow rewrites are
forbidden — they would change AS-IS behaviour.

## Test harness

- **Language / runner:** Rust, `cargo test`.
- **Test kind:** integration tests under `tests/`.
- **Binary under test:** located via the `CARGO_BIN_EXE_givn` environment
  variable (set by cargo for integration tests), invoked with
  `std::process::Command`.
- **`tempfile` dev-dependency:** `tempfile` is added as a dev-dependency in
  `Cargo.toml` (not deferred). Characterization is failure-heavy; the
  `tempfile::TempDir` handle cleans up its directory automatically when
  dropped, including on test panic, which prevents leaked temp dirs.
- **Reusable fixture:** `tests/common/mod.rs` exposes a reusable fixture
  (e.g. a `TestEnv` struct or `temp_givn_root()` function) that:
  1. creates a fresh temp dir via `tempfile::TempDir` (auto-cleanup on drop);
  2. runs `givn init` with that temp dir as the current directory (this
     scaffolds `givn/specs`, `givn/changes`, `givn/archive`, config, and
     skills exactly as a real initialised project — no duplicated logic);
  3. returns a handle carrying the `TempDir` (so it lives as long as the
     test needs it) plus convenience helpers.
  The fixture is reusable by all scenarios in this change and by future
  characterization changes.
- **Project setup helpers** exposed alongside the fixture:
  - `make_bare_root() -> TestEnv` — a `tempfile::TempDir` with no `givn/`
    subtree (used only for the uninitialised-project scenario).
  - `make_init_root() -> TestEnv` — a `tempfile::TempDir` with `givn init`
    already run (the default fixture).
  - `run_givn(env: &TestEnv, args: &[&str]) -> Output` — invoke the binary
    with the env's temp dir as current directory and return captured
    stdout/stderr/exit code.
  - `write_override(env: &TestEnv, artifact: &str, file: &str, content: &str)`
    — create `givn/artifacts/<artifact>/<file>`. Note: `givn init` does not
    scaffold `givn/artifacts/` (SMELL-3), so this helper must
    `create_dir_all` the `givn/artifacts/proposal/` subtree itself.

Note: running `givn init` in the temp dir writes the sentinel
`verify.command: "givn missing-testrunner"` into the temp dir's
`givn/config.yaml`, NOT the project's `cargo test`. This is harmless because
`src/cli/new.rs` does not read `givn/config.yaml`. The project-level
`givn/config.yaml` (updated separately — see verify.command below) is what
`givn lint` / the gate hooks consult.

## verify.command

`givn/config.yaml` currently has `verify.command: "givn missing-testrunner"`
(the sentinel). It is updated in this change to:

```yaml
verify:
  command: "cargo test"
```

This is the runner that confirms GREEN at gate points and during
characterization. `givn lint` remains the static Gherkin check (syntax,
`@wip`, delta tags) and runs no tests; the two are complementary.

## Step definition / test function locations

This project does not bind Gherkin directly to Rust (no `cucumber`-style
runner). Instead, each `.feature` scenario maps to **one Rust integration
test function** that exercises the same inputs and asserts the same outcomes.
The mapping is by scenario title: each test function is named after its
scenario and carries a `// Scenario: <title>` line for traceability.

- Feature file (spec, checked by `givn lint`):
  `givn/changes/reverse-engineer-new-command/specs/new-command/new-command.feature`
- Test functions (executable harness, run by `cargo test`):
  `tests/new_command.rs`
- Shared helpers: `tests/common/mod.rs`

After archive, the `.feature` merges into the permanent
`givn/specs/new-command/new-command.feature` (clean Gherkin, `@givn.*` tags
stripped). The Rust test functions remain under `tests/` as the standing
characterization harness.

### Why this binding approach over a Gherkin-to-Rust runner

- **Minimum footprint.** Reverse-engineering must not restructure the
  project. Adding a `cucumber` runtime dependency and a step-binding layer
  is a larger architectural decision than this change is chartered to make.
- **Zero new runtime deps.** Subprocess + `std::fs` uses only the standard
  library. The project's published binary dependencies are unchanged.
- **True black box.** Subprocess invocation is the most faithful
  characterization surface: it exercises argument parsing, exit codes, and
  stdout/stderr exactly as a real user does — including the clap-intercepted
  start-hyphen case (exit 2) that a unit-level call could not reproduce.
- **Trade-off.** The link between a `.feature` scenario and its test
  function is maintained by convention (title + comment), not enforced by a
  framework. `givn lint` + `givn apply` track coverage and `@wip` state via
  `tasks.md` checkboxes, which is the givn-native mechanism. Adopting a
  direct Gherkin binding is recorded as a future decision (see proposal.md
  Open Questions), not a blocker.
- **Drift mitigation.** Because the convention-based mapping has no
  automated enforcement, `tasks.md` carries an explicit DONE checklist item
  requiring a manual confirmation that each `// Scenario:` comment in
  `tests/new_command.rs` corresponds to exactly one scenario in the
  `.feature` (and vice versa). For this change the scenario count is small
  (17), so a hand audit is sufficient; a future change may introduce
  automated enforcement if drift becomes a problem.

## Scenario → seam mapping

Every scenario is exercised through the same seam (subprocess + temp-dir
current directory). The mapping below records the per-scenario setup
particulars and the expected outcome to assert.

- **Valid multi-word kebab-case id creates the change scaffold** — init
  root; assert exit 0, dir + file exist, stdout lines (id quoted, absolute
  proposal path, next-steps line).
- **Single-word id is accepted** — init root; assert exit 0, dir + file.
- **Id containing digits is accepted** — init root; assert exit 0, dir.
- **Empty id is rejected** — init root; the empty string reaches
  `validate_id` (clap does not intercept an empty positional argument), which
  rejects it before the `givn/`-not-found check. Assert exit code **1** and
  stderr contains "change id must not be empty"; no change directory created.
  (Observed AS-IS during characterization: `validate_id` runs, not clap, so
  exit is 1, not 2.)
- **Id with uppercase letters is rejected** — init root; assert non-zero,
  stderr contains "kebab-case", no dir.
- **Id with underscores is rejected** — init root; assert non-zero,
  stderr "kebab-case".
- **Id with spaces is rejected** — init root; assert non-zero,
  stderr "kebab-case".
- **Id with non-ASCII letters is rejected** — init root; assert non-zero,
  stderr "kebab-case".
- **Id with a dot is rejected** — init root; assert non-zero,
  stderr "kebab-case".
- **Id ending with a hyphen is rejected** — init root; assert non-zero,
  stderr "must not start or end with a hyphen".
- **Id beginning with a hyphen is rejected by argument parsing** — init
  root; invoke `givn new -foo` (no `--`); assert exit code **2**, stderr
  mentions an unexpected argument, no dir. (This is the AS-IS path: clap
  intercepts the leading hyphen before the id validator runs.)
- **Id beginning with a hyphen passed after the argument separator is
  rejected by validation** — init root; invoke `givn new -- -foo`; assert
  non-zero, stderr "must not start or end with a hyphen".
- **Uninitialised project is rejected** — bare root (no `givn/`); assert
  non-zero, stderr "givn/ not found" + "init", no dir.
- **Already-existing change is rejected** — init root with
  `givn/changes/dup-change/` pre-created; assert non-zero, stderr
  "already exists" + the absolute path to `givn/changes/dup-change`.
- **Only proposal.md is created in the change directory** — init root;
  after success, assert the change directory's entry list is exactly
  `proposal.md` and contains no `specs/` subdirectory.
- **proposal.md content equals the built-in template verbatim** — init
  root; assert `givn/changes/template-check/proposal.md` equals
  `include_str!("../assets/templates/proposal.md")` byte for byte, and that
  its heading contains the literal text `{{CHANGE_ID}}`. (This characterizes
  BUG-1 from the proposal — the unsubstituted placeholder — as AS-IS.)
- **Project override at givn/artifacts/proposal/proposal.md is used instead
  of the built-in template** — init root plus a written override file;
  assert the created `proposal.md` equals the override content byte for
  byte and does **not** contain `{{CHANGE_ID}}`.

## Provisional behaviour note

The behaviour in the spec was inferred by probing the real binary. During
characterization each test function is written to assert the **observed**
behaviour. If a test fails, the diagnosis is either (a) a test-plumbing
problem — fix the test only, or (b) a spec/implementation mismatch — correct
the spec (and this design's seam notes, if needed), never the production
code. Suspected bugs (BUG-1, SMELL-1, SMELL-2, SMELL-3) are characterized
as AS-IS and recorded as follow-up `@givn.modified` candidates in
`proposal.md` and `review.md`.
