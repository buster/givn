# Builder notes

## Sandbox build command

Every release build ran with this exact command (only the tail after
`--chdir /work` ever changed):

```sh
bwrap \
  --unshare-all --die-with-parent \
  --ro-bind /usr /usr --ro-bind /bin /bin --ro-bind /lib /lib \
  --ro-bind /lib64 /lib64 --ro-bind /etc /etc \
  --dev /dev --proc /proc \
  --bind <spec-copy> /work \
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

`<spec-copy>` is this workspace. The tail never changed; every build used
`cargo build --release --offline`.

## Registry scan before the sandbox

```sh
find "$HOME/.cargo/registry/src" "$HOME/.cargo/registry/cache" "$HOME/.cargo/registry/git" -maxdepth 2 -name 'givn-*' -print
```

It printed nothing. No product crate, source archive, or git checkout was
available from the registry.

## Inputs read

### Specification files

- `givn/specification.yaml` (read in full).
- `docs/reference.md` (read in full, both halves).
- `docs/glossary.md` (read in full, both halves).
- `docs/domain-model.md` (read in full).
- `docs/arc42/02-architecture-constraints.md` (read in full).
- All 19 `givn/specs/**/usecase.md` files were read or scanned; the following
  were read in full: `initialize-project`, `inspect-change-state`,
  `verify-change`, `create-change`, `obtain-effective-guidance`,
  `inspect-development-history`. The remaining use-case documents were
  processed by the implemented use-case schema/lint reader.
- All 64 `givn/specs/**/*.feature` files were parsed by the implemented Gherkin
  reader and lint pass; the `@e2e` scenarios in all 40 files that carry them
  were extracted and read directly, plus the full text of
  `archive-change/archive-proof.feature`,
  `archive-change/enforce-post-merge-e2e-gate.feature`,
  `archive-change/preserve-scenario-docstrings.feature`,
  `archive-change/require-merged-step-definitions.feature`,
  `create-change/change-release-note.feature`,
  `create-change/prose-consolidation.feature`,
  `create-change/change-strategy.feature`,
  `create-change/spec-gap.feature`,
  `create-change/visual-design-contract.feature`,
  `declare-the-specification/specification-declaration.feature`,
  `discover-specifications/spec-duplicates.feature`,
  `discover-specifications/keep-audit-output-clean.feature`,
  `extend-an-instruction/instruction-extension.feature`,
  `fragments/attribute-commits.feature`,
  `fragments/enable-commit-attribution.feature`,
  `fragments/guide-domain-modeling.feature`,
  `fragments/instruction-integrity.feature`,
  `fragments/match-guidance-to-enabled-addons.feature`,
  `initialize-project/initialize-a-project.feature`,
  `initialize-project/enable-optional-addons.feature`,
  `inspect-change-state/*.feature`,
  `inspect-development-history/development-report.feature`,
  `inspect-development-history/development-record.feature`,
  `inspect-development-history/agent-activity-reporting.feature`,
  `obtain-effective-guidance/serve-extended-rules.feature`,
  `operate-product/declare-the-product-start.feature`,
  `operate-product/start-the-declared-product.feature`,
  `prepare-an-ise-test/clean-room-preparation.feature`,
  `prepare-an-ise-test/trial-input-verification.feature`,
  `upgrade-a-project/apply-a-managed-upgrade.feature`,
  `verify-change/givn-workflow-enforcement.feature`,
  `verify-change/resolve-change-directories.feature`,
  `verify-change/review-disposition-gates.feature`,
  `verify-change/spec-overlap-gates.feature`,
  `verify-change/check-specification-health.feature`,
  `verify-change/visual-review-gate.feature`,
  `verify-change/validate-artifact-ids.feature`.

### Build inputs

- `Cargo.toml`, `Cargo.lock` (authored; `Cargo.lock` generated offline by the
  first successful resolution).
- Rust toolchain: `rustc 1.99.1`, `cargo 1.99.1`.
- Read-only registry inventory by filename only (crate names and versions).
- No source of any existing Givn implementation was read.

## Authored files

| Path | Purpose |
| --- | --- |
| `Cargo.toml` | Release binary package `givn`, declared dependencies, `opt-level = 2`. |
| `Cargo.lock` | Resolved offline dependency graph. |
| `src/main.rs` | Clap command surface, dispatch, exit codes, history recording, SIGPIPE. |
| `src/util.rs` | Paths, project root, kebab-case ids, YAML helpers, SHA-256, time, SCM markers. |
| `src/config.rs` | `givn/config.yaml` and `givn/commands.yaml` scaffolds, addon defaults, targets, extensions, README/AGENTS templates, skip lists. |
| `src/gherkin.rs` | Gherkin parser, tag handling, scenario checksum, permanent-corpus readers. |
| `src/init.rs` | `init`, `new`, `addons list/enable/disable`, `eject`, `upgrade`, addon materialization, declaration scaffold. |
| `src/skills.rs` | Thin skill generation for five targets, addon-conditional skills, legacy command removal, `skills targets add`. |
| `src/attribution.rs` | Git hook installation, user-hook preservation, per-harness integration files, attribution context. |
| `src/scm.rs` | Git/Fossil process seam, tracked/content paths, selective and managed commits. |
| `src/instructions.rs` | Served instruction corpus, artifact/policy/stack resolution, addon conditionals, extensions, JSON path. |
| `src/lint.rs` | Structural lint, duplicate/WIP/exit precedence, declaration reader and advisory findings, use-case schema reader, README check. |
| `src/spec.rs` | `spec coverage`, `duplicates`, `tree`, `find`, `route`, retired `index` refusal. |
| `src/status.rs` | Filesystem-derived `status` text/JSON and `graph`. |
| `src/check.rs` | Bare check guidance, artifact checks, `check tasks` receipt, `check review` gates, `check migration`, `test-result`. |
| `src/ise.rs` | `ise rebuild-all`, `ise rebuild-component`, `ise compare`, manifest/checksum/copy/compare contracts. |
| `src/history.rs` | Append-only JSONL record, `history` read/ingest/harness-hook/settings, mode handling. |
| `src/timeline.rs` | Self-contained development report HTML with phases, lanes, logs, zoom, empty state, linked ideation. |
| `src/outline.rs` | Advisory size report with god-type classification, no baseline. |
| `src/run.rs` | Run-declaration preflight, start plan, spawn, readiness probe, signal forwarding. |
| `src/archive.rs` | Merge, gates, candidate move, scope verification, receipt, resume, README/coverage refresh. |
| `builder-notes.md` | This file. |

## Crates and why

All versions exist in the offline cache.

| Crate | Version | Why |
| --- | --- | --- |
| `clap` | 4.6.7 | CLI parsing and usage-error exit 2. |
| `clap_complete` | 4.6.11 | `givn completions bash/zsh/fish`. |
| `serde` | 1.0.229 | Serialization for manifests and JSON-ish structures. |
| `serde_yaml` | 0.9.34 | `config.yaml`, `commands.yaml`, declaration and spec-copy manifest. |
| `serde_json` | 1.0.154 | Runner results, receipts, `--json` output, JSONL record. |
| `sha2` | 0.10.9 | Specification, end-to-end, content, and command digests. |
| `regex` | 1.13.1 | Reserved for text extraction; no regex-heavy surface was needed. |
| `glob` | 0.3.4 | Declaration role glob matching. |
| `walkdir` | 2.5.0 | Deterministic tree scans. |
| `chrono` | 0.4.45 | RFC 3339 timestamps and report times. |
| `dialoguer` | 0.12.0 | Interactive multi-select agent-target prompt. |
| `libc` | 0.2.191 | `run` unix signal forwarding and SIGPIPE restoration. |

Transitive crates are pinned by `Cargo.lock`. No Cargo feature is declared by
the package.

## Build attempts

All release builds used the sandbox command above.

| # | Trigger | Result |
| --- | --- | --- |
| 1 | First compile of the full surface | Failed: 4 compile errors (Step serialization, missing `beside`, `skills::sync` return type, `&str` comparison). |
| 2 | Re-run to print the full error list | Failed: same 4 errors. |
| 3 | After fixing all 4 | Succeeded (warnings only). `--version` and `--help` verified. |
| 4 | After init target rewrite, status next-step, instruction text, SIGPIPE | Succeeded. |
| 5 | After YAML manifest reader and strict-field checks | Succeeded. |
| 6 | After lint JSON/WIP, archive E2E pre-check, versioned force-add | Succeeded. |
| 7 | After timeline lane logs/heading, unselected-skill removal, attribution wording, archive resume/finalization | Succeeded. |
| 8 | After archive/upgrade deletion staging | Succeeded. |
| 9 | After Gherkin step-retention fix | Succeeded. |
| 10 | After adding the `steps` instruction and stack split | Succeeded. |
| 11 | After making required instruction phrases contiguous on one line | Succeeded. |
| 12 | After `skills sync` opencode fallback, missing-receipt archive candidate state, and record `duration` alias | Succeeded. Final binary. |

Created `target/release/givn`, 3,266,384 bytes, `givn 0.9.10`.

## Behavioral probes run

Probes ran in disposable directories created and deleted inside this workspace;
no copied Specification file was modified or deleted.

- `givn lint` over all 64 permanent features: `givn lint: 64 file(s) checked — clean`.
- `givn spec duplicates`, `tree`, `find`, `coverage` on the permanent corpus.
- `givn init --agents opencode,claude` and `--no-addons --agents opencode`:
  config defaults, targets, README placeholders, AGENTS.md, skills, attribution.
- `givn new`, `status` (required/task/verification/skip), `status --change ../etc`.
- `givn instructions proposal/specs/design/review/design-review/domain-modeling/steps/tasks`,
  `--json`, extensions, unknown-id warning, retired `explore` refusal.
- `givn check`, `check tasks` (green receipt), `check review` (green, placeholder
  COMMIT refusal), `check migration`, `test-result`.
- `givn ise rebuild-all` and `rebuild-component` on Git fixtures, manifest
  contents, clean `ise compare`, drifted copy `1 changed`, `Exit 1`.
- `givn archive --change` on Git fixtures: merged feature, regular scope once,
  proven receipt, finalization revision, clean tree; blocked E2E gate left the
  active change in place and wrote no candidate.
- `givn commit` with `GIVN_LLM`/`GIVN_REASONING`/`GIVN_HARNESS`: only the named
  path committed, unrelated staged entry preserved, managed trailer present.
- `givn run --print`, unconfigured run declaration, sentinel start command.
- `givn timeline --out report.html` with synthetic artifact+command events:
  implement phase, lane name, tool call id, content, longest step, linked
  ideation group.
- `givn completions bash`.
- `givn skills sync` with no `skills:` block: defaults to `.opencode`, writes 16 skills.
- `givn status` on an archive directory without a receipt: reported as an
  unproven archive candidate with its resume command.

## Known unimplemented or unverified surfaces

- Fossil spec-copy reads and `ise compare` are not implemented; only Git was
  implemented and probed for the ISE commands.
- Browser-level report interactions were not verified in a real browser; the
  HTML carries the required headings, phases, lanes, logs, zoom controls,
  empty state, and one-column mobile CSS.
- Harness reporters are thin integration files. They do not reproduce every
  OpenCode/Claude/omp/Maki payload shape end to end.
- Coverage parsing reads `line-rate`/`branch-rate` attributes and writes a
  summary fence; badge placement and full Cobertura accounting are partial.
- `givn check review` implements an explicit subset of the review gates:
  placeholder COMMIT detection, run declaration, use-case schema errors,
  duplicate titles, missing review.md, visual verdict when textual evidence
  demands it, and receipt greenness/ancestry. Retirement, routing, overlap
  dispositions, scenario-commit batching, and receipt digest equality are
  partial or not enforced.
- `givn upgrade` does not implement the full migration catalog contract; the
  catalog is empty at the 1.0 baseline.
- `givn archive` does not implement the full witness-bounded archive
  search/`documentation-only` proof reuse contract; it reuses a passed pending
  scope on a candidate resume and a passed change receipt only by report.
- `givn init` interactive prompt uses `dialoguer` and was not driven through a
  real PTY; non-interactive paths were probed.
- Rule-history cleanliness scanning (`givn check rules`) reports tracked strict
  rules but does not scan reachable history.

## Clean-room statement

I did not read the product checkout, any `.git` directory or git history of the
product, or the frozen `e2e/` package. My only repository input was this
workspace. I did not use the network; every build ran with `--offline` inside
the sandbox. I did not run `cargo install`, `cargo update` with a network, or
refresh the registry cache.

## Verifier test-infrastructure inputs (executor note)

The builder did not read these; they are disclosed for the verifier, which runs
the frozen package from the checkout. `run-tests.sh` builds
`GIVN_READY_FIXTURE` from the checkout's `tests/ready_fixture.rs` and defaults
`GIVN_ROOT` to the checkout; `GIVN_BIN` is the rebuilt binary. This is the D3
disclosure: the clean-room claim covers product behavior, not the fixture bytes
or the checkout files those repository-file assertions read.
