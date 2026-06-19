# 2. Architecture Constraints

These constraints are fixed decisions from the design Q&A. They are not open for
relitigation during implementation (see chapter 9 for rationale).

## 2.1 Technical Constraints

| ID | Constraint | Rationale (short) |
| --- | --- | --- |
| TC-1 | Implementation language is **Rust**. | Small static binaries, strong type safety for merge/state logic, author familiarity. |
| TC-2 | Ships as a **single static binary**. | Frictionless distribution; no runtime dependency. |
| TC-3 | **No npm**, **no root/sudo**, **no language runtime** required to install or run. | Direct fix of prior-tool pain points. |
| TC-4 | Project configuration lives in **one visible `givn/` directory**. | "All config in one place"; discoverable. |
| TC-5 | Defaults are **embedded in the binary**; project files override via cascade. | Upgrades never clobber user files. |
| TC-6 | Manifest format is **YAML**, parsed with **`serde_yaml_ng`**. | `serde_yaml` is deprecated; `serde_yaml_ng` is the maintained drop-in fork. |
| TC-7 | Gherkin parsing uses the **`gherkin` crate** (same parser cucumber-rs uses). | Proven on the gherkintest project; consistency with cucumber-rs runners. |
| TC-8 | CLI built with **`clap`**. | De-facto standard, derive API, help/usage generation. |
| TC-9 | Embedded assets via **`rust-embed`** (or `include_dir`). | Bake defaults into the binary. |
| TC-10 | Lifecycle state is **derived from the filesystem**; no separate state file. | No drift, git-friendly, stateless. |
| TC-11 | Delta tags use the `@givn.*` namespace. | Clean break from `@openspec.*`. |
| TC-12 | `verify` exit codes: **0** GREEN, **1** parse error, **2** `@wip`/PENDING. | Preserved contract from the prototype; CI-friendly. |

## 2.2 Distribution Constraints

| ID | Constraint |
| --- | --- |
| DC-1 | Must be installable via: GitHub release binary, `curl \| sh` installer, Homebrew tap, and `cargo install givn`. |
| DC-2 | Target platforms: linux x64+arm64, macOS x64+arm64, windows x64. |
| DC-3 | Crate/binary name is `givn` (verified available on crates.io and Homebrew). |
| DC-4 | Release pipeline uses `cargo-dist` (or equivalent) for multi-platform artifacts. |

## 2.3 Organisational Constraints

| ID | Constraint |
| --- | --- |
| OC-1 | Repository lives at `~/projects/givn`, local `git init` only (no remote yet). |
| OC-2 | Clean break from OpenSpec: no backward-compat read of `openspec/` dirs. A `givn migrate` command is a deferred nice-to-have. |
| OC-3 | MVP scope includes the full stack: core engine + lifecycle + skills + install. |
| OC-4 | Intended to be open source for the broader community (license TBD). |

## 2.4 Conventions

| ID | Convention |
| --- | --- |
| CV-1 | Agent rules emitted to `AGENTS.md` (Linux-Foundation-stewarded standard). |
| CV-2 | Agent skills emitted to neutral `.agents/skills/<name>/SKILL.md` (primary) with `.claude/skills` and `.opencode/skills` mirrors. |
| CV-3 | Capability names use kebab-case (e.g. `user-auth`). |
| CV-4 | Test-runner feature path overridable via `GIVN_FEATURES` env var, default `givn/specs`. |
| CV-5 | Default `verify.command` is the sentinel `givn missing-testrunner` (prints setup guidance, exits non-zero) until the project sets a real runner. |
| CV-6 | `givn/config.yaml` ships fully commented with documented defaults; every key is discoverable in-file. |
| CV-7 | `givn/config.yaml` carries `givn_config_version`; `givn upgrade` provides a version-to-version upgrade path, with git as the diff/merge surface. |
