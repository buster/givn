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
| TC-12 | `givn lint` exit codes: **0** clean, **1** parse error, **2** `@wip`/PENDING. | Static Gherkin check; preserved CI-friendly contract. Distinct from the test runner (`verify.command`). |
| TC-13 | **Minimum dependency age:** every crate version in `Cargo.lock` (direct *and* transitive) must have been published on crates.io **≥ 30 days** ago. | Avoid brand-new releases that have had no time to surface bugs or supply-chain compromise. See 2.5. |
| TC-14 | Spec retrieval is an **opt-in Cargo feature**. The default release and musl builds must not require its native ONNX Runtime or model cache. | Preserve TC-2/TC-3/DC-2 while permitting local authoring retrieval with an explicit feature build. |
| TC-15 | The semantic similarity gate uses the private binary constant `0.95`; no project config, CLI flag, environment variable, or review note may raise or lower it. | Prevent an authoring agent from weakening the uniqueness gate to make its own review pass. |
| TC-16 | The reranker experiment uses a local multilingual ONNX pair model behind the same opt-in retrieval feature and a private raw-logit boundary; no hosted inference is allowed. | Keep review reproducible, local, and independent of external services. |
| TC-17 | Blocking semantic review requires a retrieval-enabled local build; the default feature-free build must fail explicitly rather than claim semantic completeness. | Keep the opt-in native retrieval dependency explicit while ensuring the dogfood binary and review/archive workflow use `--features spec-retrieval`. |

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
| OC-4 | Open source under dual MIT OR Apache-2.0 license (see ADR-034). |

## 2.4 Conventions

| ID | Convention |
| --- | --- |
| CV-1 | `AGENTS.md` at repo root is user-owned; givn never writes or overwrites it. |
| CV-2 | Agent skills emitted to neutral `.agents/skills/<name>/SKILL.md` (primary) with `.claude/skills` and `.opencode/skills` mirrors. |
| CV-3 | Capability names use kebab-case (e.g. `user-auth`). |
| CV-4 | Test-runner feature path overridable via `GIVN_FEATURES` env var, default `givn/specs`. |
| CV-5 | Default `verify.command` is the sentinel `givn missing-testrunner` (prints setup guidance, exits non-zero) until the project sets a real runner. |
| CV-6 | `givn/config.yaml` ships fully commented with documented defaults; every key is discoverable in-file. |
| CV-7 | `givn/config.yaml` carries `givn_config_version`; `givn upgrade` provides a version-to-version upgrade path, with git as the diff/merge surface. |
| CV-8 | Artifacts may declare `post` hooks (built-in `verify`/`lint` or shell). Defaults: `tasks.post: [verify]`, `review.post: [verify]`. `givn check` runs them; `givn archive` runs the hooks of `apply.requires` artifacts as the gate. Failing hooks block (unless `allow_failure`). |

## 2.5 Dependency Age Policy (TC-13)

**Rule.** Every crate version pinned in `Cargo.lock` — direct *and* transitive —
must have been published on crates.io at least **30 days** ago.

**Why.** Cargo has **no native minimum-dependency-age feature** (no `--max-age`
flag; `-Z minimal-versions` is about minimum *version*, not *age*). Brand-new
releases are the highest-risk window for both ordinary regressions and
supply-chain compromise. A 30-day soak requirement lets issues surface before we
adopt a release.

**Enforcement.** A checked-in script, [`scripts/check-dep-age.sh`](../../scripts/check-dep-age.sh),
parses `Cargo.lock`, queries each crate version's `created_at` from the crates.io
API, and fails (exit 1) if any registry dependency is younger than the threshold.

- Threshold configurable via `MIN_AGE_DAYS` (default 30).
- Local `path`/`git` dependencies and the workspace's own crates are skipped
  (only `registry+…` sources are checked).
- Rare, justified exceptions go in `GIVN_DEP_AGE_ALLOWLIST` (`name@version` list).
- Intended to run as: (a) a CI gate, and (b) optionally a givn `post` hook on a
  dev/release artifact. Failure blocks the gate.

**Remediation when it fails.** Pin an older version
(`cargo update -p <crate> --precise <older>`), wait until the release ages past
the threshold, or allowlist with explicit justification.

## 2.6 Acceptance-boundary constraints

Repository-owned workflow acceptance runs through the compiled Givn CLI in an
isolated temporary project. Fixtures use only local deterministic commands and
must not require a consumer checkout, provider credential, network service, or
shared filesystem. The active change path is supplied through `GIVN_FEATURES`
so the same Gherkin runner can execute the delta during lifecycle gates.
