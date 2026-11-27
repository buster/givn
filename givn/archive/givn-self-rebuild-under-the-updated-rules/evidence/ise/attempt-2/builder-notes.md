# Ise rebuild — builder notes (attempt 2)

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

Re-read or consulted from the spec copy (no checkout, no `e2e/`, no network):
- `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `givn/specification.yaml`
- `docs/reference.md`, `docs/glossary.md`, `docs/domain-model.md`, `docs/arc42/02-architecture-constraints.md`
- `givn/specs/**` use cases and features, including this round's target areas: `verify-change/*`, `archive-change/*`, `initialize-project/*`, `obtain-effective-guidance/*`, `extend-an-instruction/*`, `inspect-development-history/*`, `discover-specifications/*`, `assess-code-structure/*`, `prepare-an-ise-test/*`, `operate-product/*`, `keep-documentation-current/*`, and `fragments/*` (notably `guide-domain-modeling.feature`)
- `assets/instructions/**`, `assets/skills/**`, `assets/agent-integrations/**`, `assets/hooks/**`, `assets/templates/**`, `assets/addons/**`, `assets/timeline/report.html`, `assets/*.tmpl`, `assets/default-manifest.yaml`, `assets/agents-project-hint.md`, `assets/migrations/**`

## Authored files

All content remains under `src/**` and `tests/**`; no new files were added this round.

Modified in attempt 2:
- `src/archive.rs` — receipt-clean archive preflight, candidate/finalization publication checks, scope-evidence reuse by digest, e2e-unconfigured refusal, durable retained-path gate, merged step-definition gate.
- `src/check.rs` — `unknown artifact` wording, non-executing guidance for bare `check`, invalid-placeholder wording, lifecycle recording, aggregated review findings.
- `src/init.rs` — explicit installed-skill/destination/disabled-addon report, attribution-installed report, labeled interactive target selection on stdout, run open placeholder in the README scaffold.
- `src/instructions.rs` — coverage instruction id, domain-modeling routing sentence for start/specs/design, project-state route.
- `src/cli.rs` — JSON project-state `next_skill` domain route.
- `src/misc.rs` — upgrade run/attribution backfill, managed revision staging, fill-in report, run ready port, signal exit mapping, `committed revision` report.
- `src/history.rs` — JSON entry sequence, entry ids, linked ideation, full static development-report view.
- `src/newcmd.rs` — lifecycle recording.
- `src/lint.rs` — duplicate-title paths, JSON finding kinds.
- `src/spec.rs` — use-case scenario/@e2e totals and `actors:` line, `ranked candidates` routing line, relative `spec find` paths.
- `src/status.rs` — `proof-pending` archive labels.
- `src/ise.rs` — copy only declaration/Specification/kept-source paths; e2e directory-prefix matching; strict manifest checks added earlier retained.
- `src/outline.rs` — corpus classification labels (`god-type`, `data-only`, `cohesive-large-type`, `method-length`, `module`, `within-limits`), impl method/field association, HINT labels.

Unchanged authored files (all still present): `src/lib.rs`, `src/main.rs`, `src/util.rs`, `src/assets.rs`, `src/config.rs`, `src/manifest.rs`, `src/gherkin.rs`, `src/declaration.rs`, `src/addons.rs`, `tests/features_runner.rs`, `tests/ready_fixture.rs`.

Authored file count: 24 (22 `src/*.rs`, 2 `tests/*.rs`).

Repair list addressed: items 1–12 as far as the notes below state.

Not completed or only partially addressed:
- **Archive proof reuse** is digest-based over the merged spec corpus plus runner command; it does not model every input class (coverage, helper sources, runtime settings) named by the Specification.
- **Visual review gate** is not a separate gate; a change passes when its other gates pass, rather than validating visual evidence completeness.
- **Merged step-definition enforcement** searches only step-location source files and only literal phrase text, not each runner's binding syntax.
- **Development report** renders the requested static headings, phase cards, lanes, zoom control, details panel, linked ideation group, and empty state; it does not populate the template's full JavaScript workflow model.
- **Commit attribution** writes and reports the plumbing and generic `Givn-Attribution` trailer; it does not synthesize per-harness `Co-Authored-By` values from environment.
- **Fossil-specific** archive/Ise publication still delegates to the Git-shaped paths (Fossil commands are not implemented beyond detection and refusal paths).
- **Interactive init** remains a labeled line prompt rather than an arrow-key multi-select.
- **Outline classification** remains heuristic; bands such as multi-type modules and exact 500/1,000-line boundaries are approximated.

## Crates

- `clap 4.6.7`
- `clap_complete 4.6.11`
- `serde 1.0.229`
- `serde_yaml_ng 0.10.0`
- `serde_json 1.0.151`
- `rust-embed 8.12.0`
- `anyhow 1.0.104`
- `sha2 0.11.0`
- `toml 1.2.1+spec-1.1.0`
- `libc 0.2.189`

No dependency changed; all resolved from the pinned lockfile.

## Commands

Repeated: `cargo build --release --offline`, `./target/release/givn --help`, `--version`.

Smoke and repair probes (inside the sandbox):
- `givn init --agents opencode`, `--no-addons`, `--agents <dotted list>`
- `givn addons list`
- `givn new demo`
- `givn check`, `givn check bogus`, `givn check tasks --change demo`, `givn check review --change diag`
- `givn archive --change demo` (after `givn check tasks` with an uncommitted receipt)
- `givn status`, `givn status | head -4`
- `givn history --json`, `givn history --change demo --json`
- `givn timeline --out rep.html`, `givn timeline --change demo --out rep.html`
- `givn instructions coverage`, `givn instructions specs`
- `givn outline report --json`
- `givn run --print`
- `givn ise rebuild-all --into /tmp/copy-notes --dry-run`
- `givn ise rebuild-all --into /tmp/copy-notes`
- Fixture setup: `mktemp -d`, `git init`, `git config user.email/user.name`, `mkdir`, `printf`, heredoc `cat`, `chmod +x`, `git add -A`, `git commit -qm`.

## First build

The first build of this repair round exited `0`; `tests/features_runner.rs` and
`tests/ready_fixture.rs` already held `fn main() {}`. Final rebuild exited `0`,
and the binary reports `givn 0.9.20`.

## Clean-room statement

I read no checkout path, no `e2e/` file, and no network source during this repair
round. Reads stayed inside the spec copy (`/work`); writes stayed inside
`/work/src/**`, `/work/tests/**`, and this notes file. Builds used only the
recorded bwrap sandbox and its read-only toolchain/registry binds.
