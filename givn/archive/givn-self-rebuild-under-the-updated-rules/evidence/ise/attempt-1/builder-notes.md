# Ise rebuild — builder notes (attempt 1)

## Sandbox build command

Every build used exactly this command (only the trailing command after `--chdir /work` changed):

```sh
bwrap \
  --unshare-all --die-with-parent \
  --ro-bind /usr /usr --ro-bind /bin /bin --ro-bind /lib /lib \
  --ro-bind /lib64 /lib64 --ro-bind /etc /etc \
  --dev /dev --proc /proc \
  --bind /tmp/givn-ise-self-rebuild-20261117T204903Z /work \
  --tmpfs /home \
  --ro-bind "$HOME/.cargo/bin/rustup" /home/builder/.cargo/bin/rustup \
  --ro-bind "$HOME/.cargo/bin/cargo" /home/builder/.cargo/bin/cargo \
  --ro-bind "$HOME/.cargo/bin/rustc" /home/builder/.cargo/bin/rustc \
  --ro-bind "$HOME/.rustup" /home/builder/.rustup \
  --ro-bind "$HOME/.cargo/registry" /home/builder/.cargo/registry \
  --ro-bind "$HOME/.cargo/git" /home/builder/.cargo/git \
  --tmpfs /tmp \
  --setenv HOME /home/builder \
  --setenv RUSTUP_HOME /home/builder/.rustup \
  --setenv CARGO_HOME /home/builder/.cargo \
  --setenv PATH "/home/builder/.cargo/bin:/usr/bin:/bin" \
  --chdir /work cargo build --release --offline
```

No build ran outside this sandbox.

## Inputs read

Specification and contracts
- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`
- `givn/specification.yaml`
- `docs/reference.md`, `docs/glossary.md`, `docs/domain-model.md`, `docs/arc42/02-architecture-constraints.md`

Behavior corpus (`givn/specs/**`)
- `initialize-project/initialize-a-project.feature`, `initialize-project/enable-optional-addons.feature`
- `create-change/create-a-change.feature`, `create-change/change-strategy.feature`, `create-change/change-release-note.feature`
- `inspect-change-state/*.feature` (status, task progress, change ids, skipped artifacts)
- `obtain-effective-guidance/serve-extended-rules.feature`, `extend-an-instruction/instruction-extension.feature`
- `verify-change/check-specification-health.feature`, `verify-change/givn-workflow-enforcement.feature` (selected scenarios), `verify-change/review-disposition-gates.feature`, `verify-change/spec-overlap-gates.feature`, `verify-change/validate-artifact-ids.feature`, `verify-change/visual-review-gate.feature` (scenario titles)
- `discover-specifications/spec-duplicates.feature`, `discover-specifications/keep-audit-output-clean.feature`, `discover-specifications/usecase.md`
- `declare-the-specification/specification-declaration.feature`
- `prepare-an-ise-test/clean-room-preparation.feature`, `prepare-an-ise-test/trial-input-verification.feature`
- `archive-change/archive-proof.feature`, `archive-change/enforce-post-merge-e2e-gate.feature`, `archive-change/extract-coverage-gate.feature`, `archive-change/grouped-merge-for-new-capabilities.feature`, `archive-change/preserve-scenario-docstrings.feature`, `archive-change/require-durable-retained-paths.feature`, `archive-change/require-merged-step-definitions.feature`
- `assess-code-structure/outline-report.feature`, `upgrade-a-project/apply-a-managed-upgrade.feature`, `operate-product/start-the-declared-product.feature`, `operate-product/declare-the-product-start.feature`, `keep-documentation-current/readme-enforcement.feature`
- `inspect-development-history/*.feature` (scenario titles and selected details)
- use-case narratives in `author-change`, `create-change`, `inspect-change-state`, `initialize-project`, `obtain-effective-guidance`, `prepare-an-ise-test`, `verify-change`, `discover-specifications`

Served and structural assets (`assets/**`)
- `instructions/*.md` (served text, addon conditionals, resident-rules markers)
- `skills/*/SKILL.md.tmpl`
- `agent-integrations/{opencode,claude,omp,maki}/**`
- `hooks/commit-msg`, `hooks/git-commit-msg-launcher`
- `templates/*.md`, `templates/spec.feature`
- `addons/arc42-docs/**`, `addons/communication-policy/policy.md`, `addons/coverage/**`, `addons/readme/README.md.tmpl`
- `config.yaml.tmpl`, `commands.yaml.tmpl`, `run.yaml.tmpl`, `default-manifest.yaml`, `specification.yaml.tmpl`, `agents-project-hint.md`
- `timeline/report.html`
- `migrations/catalog.yaml`, `migrations/fixtures/synthetic/**`

## Authored files

All content is under `src/**` and `tests/**`.

- `src/lib.rs` — crate module map and version.
- `src/main.rs` — process entry, SIGPIPE handling.
- `src/cli.rs` — clap command surface, dispatch, exit codes.
- `src/util.rs` — errors with contractual exit codes, sha256, path walking, id validation, target names.
- `src/assets.rs` — rust-embed over `assets/` and the `{{#if addons.*}}` renderer.
- `src/config.rs` — project discovery, config/commands/declaration authority, addon and target text edits, SCM detection, JSON printing.
- `src/manifest.rs` — embedded artifact graph, addon-driven arc42 artifact, config overlay, archive gates.
- `src/gherkin.rs` — Gherkin parser, canonical executable payload, shape/assertion normalization, checksums.
- `src/declaration.rs` — `givn/specification.yaml` loader/refusals, glob matching, declaration advisories, e2e package resolution.
- `src/lint.rs` — feature corpus scan, parse/WIP/duplicate/shape findings, seed and persona advisories, change-scoped projected inventory, exit precedence, JSON.
- `src/spec.rs` — `spec coverage`, `spec duplicates`, `spec tree`, `spec find`, `spec route`.
- `src/init.rs` — `init`, agent selection, scaffolding, config/commands writes, skills sync, target add, AGENTS managed blocks, ignore state, integrations.
- `src/instructions.rs` — instruction/policy serving, addon conditionals, project extensions, overrides, JSON envelope.
- `src/newcmd.rs` — `new` validation, proposal template (byte-for-byte or project override), config lock, skip file.
- `src/status.rs` — stage/artifact checklist, task progress, verification freshness, archive proof state, text and JSON.
- `src/check.rs` — `check tasks` runner contract and receipt, `check review` integrity/run/README/lint/lock/receipt/use-case schema gates, revision and input witnesses.
- `src/addons.rs` — addon list/materialization state, enable/disable prerequisites, retired ids, locked-change override.
- `src/archive.rs` — archive preflight, routing, delta merge, candidate move, regular/e2e scopes, proven/pending receipt.
- `src/ise.rs` — `ise rebuild-all`, `ise rebuild-component`, `ise compare`, tracked-file partition, checksums, manifest, refusals.
- `src/history.rs` — record store, `history` read/ingest/harness-hook/settings, timeline HTML rendering.
- `src/outline.rs` — structure report for Rust/Python/Java/TypeScript/JavaScript, JSON schema 5.
- `src/misc.rs` — `graph`, `eject`, `upgrade`, `readme check`, `run`, `commit`, `completions`, `test-result`, sentinels, attribution stamp.
- `tests/features_runner.rs` — minimal `fn main() {}` manifest target.
- `tests/ready_fixture.rs` — minimal `fn main() {}` manifest target.

Authored file count: 24 (22 under `src/`, 2 under `tests/`).

## Crates

From `Cargo.lock`; only the crates the implementation relies on are listed:

- `clap 4.6.7`
- `clap_complete 4.6.11`
- `serde 1.0.229`
- `serde_yaml_ng 0.10.0`
- `serde_json 1.0.151`
- `rust-embed 8.12.0`
- `anyhow 1.0.104`
- `sha2 0.11.0`
- `toml 1.2.1+spec-1.1.0`
- `libc 0.2.189` (unix SIGPIPE handling)

Transitive crates were resolved from the pinned lockfile; no dependency was added or updated.

## Commands

Every command below ran inside the sandbox with the tail shown (repeated runs collapsed).

- `cargo build --release --offline` (first build and every rebuild)
- `./target/release/givn --version`
- `./target/release/givn --help`
- `./target/release/givn init --agents opencode`
- `./target/release/givn init --no-addons --agents opencode`
- `./target/release/givn init --agents .claude,.agents` (and `--agents all`, `--agents claude,nope`)
- `./target/release/givn lint`, `givn lint --change empty-change`
- `./target/release/givn instructions tasks`, `instructions proposal --change add-user-auth`, `instructions --json`, `instructions proposal --change feature --json`, `instructions arc42-docs`, `instructions ise`
- `./target/release/givn status`, `status --change add-user-auth`, `status --change feature --json`, `status --change old`
- `./target/release/givn new add-user-auth`, `new feature`, `new -foo`, `new -- -foo`, `new ""`, `new empty-change`, `new verify-demo`, `new demo`
- `./target/release/givn addons list`
- `./target/release/givn addons enable ise`, `addons enable architecture_principles`, `addons disable architecture_principles`, `addons disable domain_modeling`
- `./target/release/givn skills targets add claude`, `skills targets add claude,nope`, `skills targets add all`
- `./target/release/givn spec duplicates`, `spec coverage`, `spec tree`, `spec find order`
- `./target/release/givn readme check`
- `./target/release/givn graph`, `graph --json`
- `./target/release/givn eject design:template`, `eject nonexistent:instruction`
- `./target/release/givn check tasks --change verify-demo`, `check review --change verify-demo`, `check review --change nope`
- `./target/release/givn archive --change demo`
- `./target/release/givn ise rebuild-all --into /tmp/ise-copy-test --manifest /tmp/ise-manifest-test.json --dry-run`
- `./target/release/givn ise rebuild-all --into /tmp/ise-copy-test --manifest /tmp/ise-manifest-test.json`
- `./target/release/givn ise compare --manifest /tmp/ise-manifest-test.json --copy /tmp/ise-copy-test`
- `./target/release/givn outline report --json`
- Fixture setup commands inside the sandbox: `mktemp -d`, `git init`, `git config user.email/user.name`, `mkdir`, `printf`, `cat > ... <<"EOS"`, `chmod +x`, `git add -A`, `git commit -qm <message>`.

## First build

The first build exited `0`. Before it, per the contract, `tests/features_runner.rs` and
`tests/ready_fixture.rs` were created, each containing exactly `fn main() {}`, so Cargo
accepted the two `harness = false` `[[test]]` targets and no manifest-parse failure
(exit 101) occurred. The first build produced `target/release/givn` reporting
`givn 0.9.20`.

## Clean-room statement

I read no path in the checkout, no `e2e/` file, and no network source. My reads were
confined to the spec copy (`/work`), and my writes were confined to `/work/src/**`,
`/work/tests/**`, and this notes file. The build ran only inside the recorded bwrap
sandbox, using the read-only toolchain and lockfile-resolved registry cache supplied by
the sandbox.
