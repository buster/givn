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

The planned release, shell-installer, Homebrew, and Cargo publication channels
are not yet wired as release infrastructure in this checkout.

## 7.3 Target Environments

The package is intended to support the usual Linux, macOS, and Windows targets.
Static musl linking and the exact release matrix are deployment work still to be
proven; this documentation does not claim that all target artifacts currently
build or ship.

## 7.4 Project Footprint

`givn init` creates:

```text
givn/config.yaml
givn/commands.yaml
givn/specs/
givn/changes/
givn/archive/
.githooks/commit-msg                 # checked-in shared commit hook
.opencode/plugins/givn-attribution.js # when opencode is selected
.claude/hooks/givn-attribution.sh    # when claude is selected
.claude/settings.json                # merged Givn hook entry when claude is selected
```

The Arc42 addon currently enabled by default during `init` scaffolds the fixed
`docs/arc42/` directory. It does not create a configurable output location.

Generated skills are written only to the persisted target selection. Givn may
maintain its own fenced blocks in root `AGENTS.md`; all content outside those
blocks remains user-owned.

The checked-in `.githooks/commit-msg` is shared through normal version control.
The local dispatcher under the Git hooks directory is machine-local and is
recreated by `givn init` or `givn skills sync`; Git clones do not activate a
project hook merely because the source file is checked in.

## 7.5 Distribution Work

The intended release pipeline uses release binaries, a shell installer,
Homebrew, and `cargo publish`, potentially generated with `cargo-dist`. No
remote release pipeline or target artifact should be treated as current checkout
evidence until it is present and verified.

## 7.6 Upgrade

After a binary upgrade, `givn upgrade` refreshes the managed config scaffold and
generated guidance. For a clean project whose marker selects the 0.2.0-to-0.3.0
route, the binary performs this synchronization as the first of two narrowly
scoped Git commits and creates a second commit containing the LLM migration
plan. For a release without a selected route, it creates one managed commit
when files changed and no migration plan. Project overrides, durable Arc42 chapters, and user-owned
`AGENTS.md` content outside givn blocks remain under project control.

The managed route requires a version-controlled project and a clean checkout.
Projects without a usable backend and dirty projects receive no partial update;
the public migration guide describes the evidence-preserving recovery path. A
migration catalog route may span multiple versions, but all of its bundle
phases are delivered in one ordered aggregate maintenance change.
