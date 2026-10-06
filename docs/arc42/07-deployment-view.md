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
`fossil` installed.

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
.claude/settings.local.json          # merged Givn hook entry when claude is selected
```

The Arc42 addon currently enabled by default during `init` scaffolds the fixed
`docs/arc42/` directory. It does not create a configurable output location.

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

### Planned OpenCode compatibility footprint

[The compatibility design](../../givn/changes/fix-opencode-v2-attribution/design.md)
retains exactly one generated `.opencode/plugins/givn-attribution.js` when
OpenCode is selected. The same embedded asset is intended for V1 1.18.29+
within V1 and V2. A rebuilt Givn binary supplies it through the existing
generation path; no V1 sibling, runtime-version selection, or production npm
installation is added. Regeneration still overwrites the canonical generated
file, without backups, merges, or preservation of arbitrary manual edits.

The planned acceptance environment preprovisions real OpenCode releases and
Node through the Nix development closure. Each example owns an isolated
temporary project and V2 server where applicable, plus a deterministic loopback
provider twin and a refusing forward proxy. The proxy answers `403` to every
request, records the destination (the pinned runtimes try `models.opencode.ai:443`
and `registry.npmjs.org:443`; any other destination fails the run) and never
forwards; it is not a TLS-intercepting catalog twin and no fixture CA exists.
The fixed fixture command uses the absolute pinned Node path because V2's
`/bin/zsh` login shell resets `PATH`. Darwin Seatbelt or a Linux loopback-only network
namespace denies other traffic; missing isolation fails setup. These are
test-only processes, not deployed Givn services. Provisioning and compatibility
tests remain unexecuted; the detailed lifecycle belongs to the design.

The planned `tests/fixtures/opencode/resolve-runtime.sh` validates pinned runtime
paths exported by the development shell defined only in `flake.nix`. It is used
by the suite wrapper, direct `run-tests.sh`, and runtime fixtures used in
cargo/nextest and coverage; the shell scripts never invoke Nix or Fish. Canonical
archive/self-dogfood commands require the development environment to be active
but do not depend on wrapper-private runtime discovery or a globally installed
OpenCode. Missing prerequisites and actual tool-selected shell diagnostics are
test-only concerns.
The separate `tests/opencode_supervisor.rs` target owns fixture process groups
and monitors a parent lease so runner signals/process exit trigger bounded
termination and reaping; `run-suite.sh --setup-smoke` holds that lease with a
FIFO writer so a closed caller stdin (CI) is not read as a closed lease. The
test-only negative-control overrides (substitute plugin files) are refused
unless `GIVN_OPENCODE_NEGATIVE_CONTROL=1` is set, so no exported variable can
turn the suite green against a substitute. Admission and interruption probes remain unexecuted;
RAII alone is not an interruption-cleanup guarantee.

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
generated guidance. For a clean project whose marker selects the 0.2.0-to-0.3.0
route, the binary performs this synchronization as the first of two narrowly
scoped Git commits and creates a second commit containing the LLM migration
plan. For a release without a selected route, it creates one managed commit
when files changed and no migration plan. Project overrides, durable Arc42 chapters, and user-owned
`AGENTS.md` content outside givn blocks remain under project control.

For the pending question-policy update, changed defaults require the rebuilt
binary and normal guidance synchronization. Project-owned instruction, skill,
and command replacements remain authoritative and untouched; their owners
must review stale question guidance. Sync must not be described as upgrading
an override or proving that an agent follows the referenced policy.

The managed route requires a version-controlled project and a clean checkout.
Projects without a usable backend and dirty projects receive no partial update;
the public migration guide describes the evidence-preserving recovery path. A
migration catalog route may span multiple versions, but all of its bundle
phases are inventoried in one ordered aggregate maintenance change; a bundle
may direct that the repairs run as named follow-up changes.
