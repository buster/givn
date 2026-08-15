# 7. Deployment View

## 7.1 Artifact

A single statically-linked executable named `givn`, with all defaults
(manifest, templates, skill templates) embedded. The default build has no
runtime dependency and no companion files required.

Spec retrieval is an opt-in authoring build (`--features spec-retrieval`). It
adds fastembed's native ONNX Runtime and uses a project-local model cache under
`givn/`; the first production index may download the configured model. This
feature build is not claimed as a portable musl release artifact until its
native link mode is proven. The distributed/default binary remains independent
of the model cache and preserves the single-binary contract.

## 7.2 Target environments

| Platform | Triple |
| --- | --- |
| Linux x64 | `x86_64-unknown-linux-gnu` (and/or `-musl` for full static) |
| Linux arm64 | `aarch64-unknown-linux-gnu` |
| macOS x64 | `x86_64-apple-darwin` |
| macOS arm64 | `aarch64-apple-darwin` |
| Windows x64 | `x86_64-pc-windows-msvc` |

> A musl target is preferred for Linux to maximise "no runtime" portability.

## 7.3 Distribution channels

| Channel | Mechanism | Audience |
| --- | --- | --- |
| GitHub release binaries | Pre-built archives per platform attached to a release. | Everyone. |
| Shell installer | `curl -fsSL <url>/install.sh \| sh` downloads the right binary to `~/.local/bin` (no root). | Anyone with a terminal. |
| Homebrew | A tap formula (`brew install <tap>/givn`). | macOS + Linux. |
| Cargo | `cargo install givn` from crates.io. | Rust developers. |

Explicitly **not** used: npm, system package managers requiring root, language
runtimes.

## 7.4 Release pipeline

- Built with **`cargo-dist`** (to be added in Phase 6) to produce multi-platform
  artifacts, checksums, the installer script, and the Homebrew formula from one
  config.
- Steps (CI, when a remote/tag exists later):
  1. Tag a version.
  2. `cargo-dist` builds all targets.
  3. Publish release with binaries + `install.sh` + checksums.
  4. Update the Homebrew tap.
  5. `cargo publish` to crates.io.

> OC-1: initially local `git init` only, no remote. The pipeline is designed now
> and wired up when a remote is added.

## 7.5 Installed footprint

```
~/.local/bin/givn            # the binary (curl|sh installer target)
# or  $(brew --prefix)/bin/givn
# or  ~/.cargo/bin/givn
```

Per-project footprint is created by `givn init` (see chapter 8.4 for the layout).
No global config directory is used (NFR-3: all config in the project `givn/`).
When retrieval is enabled, `givn/.spec-index.json`, the embedding model cache,
and the separate `givn/.spec-reranker-cache/` are project-local and generated.
Interactive search remains advisory; review additionally runs the local
multilingual pair reranker over top-K candidates. These artifacts are not
present in the default installation without the opt-in feature.

## 7.6 Upgrading an installed project

After upgrading the binary (via any channel), a project adopts new defaults and
documentation with:

```
givn upgrade        # refresh documented/versioned config scaffold (ch.8.10)
givn skills sync    # refresh generated skills (ch.8.7)
```

Both write into the project repo; the user reviews via `git diff`. User overrides
are preserved (ADR-005, ADR-023). `givn_config_version` in `givn/config.yaml`
records which binary version last upgraded the project.
