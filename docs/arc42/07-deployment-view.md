# 7. Deployment View

## 7.1 Product Artifact

The product target is a native executable named `givn` with embedded manifests,
templates, instructions, and skill assets. The default Cargo install enables
all declared retrieval features; a feature-free minimal build remains available
with `--no-default-features`.

The current source-of-truth installation path is a Rust build from this
checkout:

```sh
cargo install --path .
```

The planned release, shell-installer, Homebrew, and Cargo publication channels
are not yet wired as release infrastructure in this checkout.

## 7.2 Default Retrieval Artifact

The default feature set adds native ONNX Runtime and local E5 model/tokenizer
support. The default E5 cache is `givn/.spec-model-cache`; the persistent index
is `givn/.spec-index.json`. A feature-free build reports retrieval as
unavailable rather than passing silently.

`cross-encoder-reranker` adds local BGE evidence and is enabled by default. Its default artifact directory
is `givn/.spec-reranker-cache` and the current production loader preflights
`model.onnx`, optional referenced `model.onnx.data`, `tokenizer.json`,
`config.json`, `special_tokens_map.json`, and `tokenizer_config.json` before
using FastEmbed's user-defined loader. An approval file is checked for the
runtime metadata required by the BGE admission path.

`nli-evidence` adds local NLI evidence and is enabled by default. Its default artifact directory is
`givn/.spec-nli-cache`, with `model.onnx`, `config.json`, `tokenizer.json`, and
`tokenizer_config.json` preflighted before session construction.

Regular and E2E tests use deterministic stubs and do not download or construct
multi-gigabyte model artifacts. Production model paths remain local and
feature-gated.

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
```

The Arc42 addon currently enabled by default during `init` scaffolds the fixed
`docs/arc42/` directory. It does not create a configurable output location.
Retrieval-enabled projects may additionally contain `.spec-index.json` and the
local model caches above.

Generated skills are written only to the persisted target selection. Givn may
maintain its own fenced blocks in root `AGENTS.md`; all content outside those
blocks remains user-owned.

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

The managed route requires Git and a clean worktree. Non-Git and dirty projects
receive no partial update; the public migration guide describes the
evidence-preserving recovery path. A migration catalog route may span multiple
versions, but all of its bundle phases are delivered in one ordered aggregate
maintenance change.
