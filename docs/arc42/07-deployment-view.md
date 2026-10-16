# 7. Deployment View

## 7.1 Product Artifact

The product target is a native executable named `givn` with embedded manifests,
templates, instructions, and skill assets. The build contract is a single
feature-free binary: no Cargo features are declared and no model dependencies
exist.

The current source-of-truth installation path is a Rust build from this
checkout:

```sh
cargo install --path .
```

The planned shell-installer, Homebrew, binary, and Cargo publication channels
are not yet wired as release infrastructure in this checkout; the GitHub
release channel is wired by the release workflow described in 7.5.

## 7.3 Target Environments

The package is intended to support the usual Linux, macOS, and Windows targets.
Static musl linking and the exact release matrix are deployment work still to be
proven; this documentation does not claim that all target artifacts currently
build or ship.

Operating a project additionally requires the executable of its selected SCM
backend on `PATH`: `git` for a Git worktree, `fossil` for a Fossil checkout. A
project whose marker is present but whose executable is missing is refused by
name before any mutation. The repository acceptance suite needs both `git` and
`fossil` installed. Its browser tier additionally needs Node.js (the current
LTS) and the Chromium build bundled with the resolved Playwright version, both
installed by the one-time `./setup-e2e-browser.sh`; that script is the only
step in the suite that touches the network, and no scenario needs a server, a
port, or a database.

## 7.4 Project Footprint

`givn init` creates:

```text
givn/config.yaml
givn/commands.yaml
givn/specs/
givn/changes/
givn/archive/
.githooks/commit-msg                 # checked-in shared commit hook
.opencode/plugins/givn-attribution.js # when opencode is selected (Git and Fossil)
.claude/hooks/givn-attribution.sh    # when claude is selected
.claude/settings.json                # merged Givn hook entry when claude is selected
```

The Arc42 addon currently enabled by default during `init` scaffolds the fixed
`docs/arc42/` directory. It does not create a configurable output location.

The development record adds one project-local directory, `givn/history/`: the
append-only `events.jsonl` record, the generated reports under
`givn/history/reports/`, and the small pending-state files under
`givn/history/pending/` that a process-per-hook harness needs for its start
timestamps. Givn keeps all of it out of repository history independently of the
`rules` policy, because the record is development data rather than generated
rules, and the recorder never writes a versioned file: it re-asserts the record
block in `.git/info/exclude` for a Git worktree, writes Fossil's repository-level
`ignore-glob` setting only while no managed file overrides it, and uses
`.gitignore` only before a repository exists. A Fossil checkout that already has
`.fossil-settings/ignore-glob` receives the entry `givn/history/*` in that file's
managed entry list through `givn init`, `givn skills sync`, and `givn upgrade`.

When `omp` is a selected target, initialization and synchronization also install
`.omp/hooks/pre/givn-timeline.js` next to the attribution hook. The other two
targets that execute an agent get their own reporter the same way:
`.opencode/plugins/givn-timeline.js` for `opencode`, and
`.claude/hooks/givn-timeline.sh` plus one registered event per hook entry in
`.claude/settings.local.json` for `claude`. The `agents` target is a skills
mirror that no harness runs, so it receives no reporter. The browser tier's Node
project under `tests/e2e/` is repository content and is not installed into a
consumer project.

Generated skills are written only to the persisted target selection. Givn may
maintain its own fenced blocks in root `AGENTS.md`; all content outside those
blocks remains user-owned.

The pending decision-ready-question contract adds a short effective-dialog
reference to the existing managed project block and all bundled available
guidance. It creates no additional deployed service or generated-file type.

The checked-in `.githooks/commit-msg` is shared through normal version control.
The local dispatcher under the Git hooks directory is machine-local and is
recreated by `givn init` or `givn skills sync`; Git clones do not activate a
project hook merely because the source file is checked in.

## 7.5 Distribution Work

The release pipeline is one dispatch-triggered workflow,
`.github/workflows/release.yml`. A dispatch naming a release version prepares
the version bump, generates the release section from the revisions since the
last version tag, runs the repository gates, records the release revision and
the annotated version tag, and publishes a GitHub release whose notes are the
generated section. A dispatch naming an existing annotated version tag
revalidates that tag and publishes its release when it is missing. The base
check (`scripts/release-guard.sh base`) refuses a version whose package
version has no matching version tag, and the section check refuses a release
section without a user-visible group.

The changelog selection rules live in `cliff.toml`: only breaking changes,
features, bug fixes, performance changes, and reverts; one entry per change,
taken from the change's release note. The GitHub release is the only
publication channel. Release binaries, a shell installer, Homebrew, the exact
target matrix, and `cargo publish` remain unwired; no such artifact should be
treated as current checkout evidence until it is present and verified.

## 7.6 Upgrade

After a binary upgrade, `givn upgrade` refreshes the managed config scaffold and
generated guidance. For a clean project whose marker selects a declared
migration route, the binary performs this synchronization as the first of two
narrowly scoped revisions on the selected backend and creates a second
revision containing the LLM migration plan. For a release without a selected
route — including any version below the oldest source the migration catalog
declares (0.7.0) — it
creates one managed revision when files changed and no migration plan. Project
overrides, durable Arc42 chapters, and user-owned
`AGENTS.md` content outside givn blocks remain under project control.

For the pending question-policy update, changed defaults require the rebuilt
binary and normal guidance synchronization. Project-owned instruction, skill,
and command replacements remain authoritative and untouched; their owners
must review stale question guidance. Sync must not be described as upgrading
an override or proving that an agent follows the referenced policy.

The managed route requires a version-controlled project and a clean checkout.
Projects without a usable backend and dirty projects receive no partial update;
the retained migration guides describe the evidence-preserving recovery path. A
migration catalog route may span multiple versions, but all of its bundle
phases are inventoried in one ordered aggregate maintenance change; a bundle
may direct that the repairs run as named follow-up changes.

*[Architecture documentation index](README.md)*
