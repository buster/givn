# 5. Building Block View

## 5.1 Level 0 — System

`givn` is a single published binary crate. It reads embedded defaults and the
project's `givn/` directory, and writes artifacts and agent files.

## 5.2 Level 1 — Crate structure

```
givn/  (single published binary crate)
├── src/
│   ├── main.rs                 # clap dispatch / entrypoint
│   ├── cli/                    # one module per subcommand
│   │   ├── init.rs
│   │   ├── new.rs
│   │   ├── instructions.rs
│   │   ├── verify.rs
│   │   ├── status.rs
│   │   ├── apply.rs
│   │   ├── archive.rs
│   │   ├── eject.rs
│   │   ├── skills.rs           # `skills sync`
│   │   ├── upgrade.rs          # re-render managed config scaffold, bump version
│   │   └── missing_testrunner.rs  # sentinel default verify.command
│   ├── config/
│   │   ├── embedded.rs         # access to baked-in assets (rust-embed)
│   │   ├── manifest.rs         # serde types for the YAML manifest
│   │   ├── overlay.rs          # merge project overlay onto embedded graph
│   │   ├── cascade.rs          # per-facet resolution (project ← embedded)
│   │   └── render.rs           # render documented, versioned config.yaml scaffold
│   ├── gherkin/
│   │   ├── parse.rs            # wrap the `gherkin` crate
│   │   ├── delta.rs            # tag detection, split feature/scenario
│   │   └── merge.rs            # added/modified/removed merge + tag stripping
│   ├── skills/
│   │   ├── render.rs           # template → SKILL.md / AGENTS.md
│   │   └── targets.rs          # emit to .agents, .claude, .opencode
│   ├── state.rs                # derive lifecycle state from the filesystem
│   └── error.rs                # error types (thiserror) + exit-code mapping
├── assets/                     # embedded via rust-embed
│   ├── default-manifest.yaml   # the default artifact graph
│   ├── templates/              # proposal.md, spec.feature, design.md, tasks.md, review.md
│   └── skills/                 # default SKILL.md templates per workflow step
└── tests/                      # integration tests
```

## 5.3 Module responsibilities

| Module | Responsibility | Key collaborators |
| --- | --- | --- |
| `main.rs` | Parse args, dispatch to `cli::*`, map errors to exit codes. | `cli`, `error` |
| `cli::init` | Scaffold `givn/`, write minimal `config.yaml`, generate AGENTS.md + skills. | `config`, `skills` |
| `cli::new` | Create `givn/changes/<id>/` scaffold (proposal stub). | `config`, `state` |
| `cli::instructions` | Resolve an artifact's instruction (cascade) and emit (optionally `--json`). | `config::cascade`, `config::manifest` |
| `cli::verify` | Lint `.feature` files; produce exit codes 0/1/2. | `gherkin::parse` |
| `cli::status` | Report lifecycle state derived from filesystem. | `state` |
| `cli::apply` | Parse `tasks.md` checkboxes; report/track progress. | `state`, `config::manifest` |
| `cli::archive` | Merge deltas into `givn/specs/`, strip tags, move change to `archive/`. | `gherkin::merge`, `config::manifest` |
| `cli::eject` | Write an embedded default facet into the project `givn/`. | `config::embedded`, `config::cascade` |
| `cli::skills` | Regenerate AGENTS.md + skills from current templates. | `skills` |
| `cli::upgrade` | Re-render managed config scaffolding, preserve user overrides, bump `givn_config_version`. | `config::manifest`, `config::embedded` |
| `cli::missing_testrunner` | Default sentinel `verify.command`: print per-language setup guidance, exit non-zero. | — |
| `config::embedded` | Read-only access to compiled-in assets. | `rust-embed` |
| `config::manifest` | Serde model of the manifest (artifacts, deps, apply, verify). | `serde_yaml_ng` |
| `config::overlay` | Apply the project overlay to the embedded default graph. | `manifest` |
| `config::cascade` | For `(artifact, facet)`, return project file or embedded default. | `embedded`, project FS |
| `config::render` | Render the fully-commented, versioned `givn/config.yaml`; merge-preserve user overrides on upgrade. | `embedded`, `manifest` |
| `gherkin::parse` | Parse `.feature` into a typed AST; classify lint findings. | `gherkin` crate |
| `gherkin::delta` | Detect `@givn.*` tags; split features/scenarios. | `parse` |
| `gherkin::merge` | Apply added/modified/removed to permanent spec; strip tags. | `delta`, `parse` |
| `skills::render` | Render templates with context into SKILL.md / AGENTS.md. | `config::cascade` |
| `skills::targets` | Write rendered output to the three skill locations + root. | `render` |
| `state` | Compute change state from which files exist. | project FS, `manifest` |
| `error` | Typed errors; map to process exit codes. | `thiserror` |

## 5.4 Dependencies (crates)

| Crate | Use | Version (verified) |
| --- | --- | --- |
| `clap` | CLI parsing | 4.6 |
| `serde` + `serde_yaml_ng` | Manifest (de)serialisation | serde 1, serde_yaml_ng 0.10 |
| `rust-embed` | Embed default assets | 8.11 |
| `gherkin` | Parse `.feature` files | 0.16 |
| `anyhow` | Application error context | 1.0 |
| `thiserror` | Library error types | 2.0 |
| `serde_json` | `--json` output | 1 |

(Exact versions to be pinned in `Cargo.toml` during Phase 1.)
