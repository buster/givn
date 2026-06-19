# givn — Implementation Task Breakdown

Phase-by-phase plan. Each phase ends green (compiles + tests pass) and is
committed. References to architecture: see [`arc42/`](arc42/README.md), especially
chapter 8 (cross-cutting concepts) and chapter 9 (decisions).

Legend: `[ ]` pending · `[x]` done.

---

## Phase 0 — Repository bootstrap

- [ ] 0.1 `cargo init --bin` in `~/projects/givn` (crate name `givn`).
- [ ] 0.2 Add `Cargo.toml` deps: `clap` (derive), `serde` (derive),
      `serde_yaml_ng`, `serde_json`, `rust-embed`, `gherkin`, `anyhow`,
      `thiserror`. Pin versions (see arc42 ch.5.4).
- [ ] 0.3 Add `.gitignore` (`/target`, editor cruft).
- [ ] 0.4 Verify `cargo build` and `cargo test` succeed (empty).
- [ ] 0.5 Commit: "chore: bootstrap cargo project".

**Done when:** empty binary builds and runs `givn --help`.

---

## Phase 1 — Skeleton & embedded defaults

- [ ] 1.1 Define `clap` CLI skeleton in `main.rs` with all subcommands
      (`init`, `new`, `instructions`, `lint`, `status`, `apply`, `archive`,
      `eject`, `skills sync`, `upgrade`, `missing-testrunner`) as stubs returning
      "not implemented".
- [ ] 1.2 Create `assets/` and wire `rust-embed` in `config/embedded.rs`.
- [ ] 1.3 Port the prototype `schema.yaml` → `assets/default-manifest.yaml`,
      renaming tags to `@givn.*` and paths to `givn/...`. Artifacts: proposal,
      specs, design, tasks, review; `apply.requires: [tasks, review]`;
      `apply.tracks: tasks.md`; `verify` block.
- [ ] 1.4 Port the 5 templates into `assets/templates/`
      (`proposal.md`, `spec.feature`, `design.md`, `tasks.md`, `review.md`),
      with `@givn.*` tags and `GIVN_FEATURES` default `givn/specs`.
- [ ] 1.5 Implement `config/manifest.rs` serde types; load + parse the embedded
      manifest; unit test it deserialises. Default `verify.command` is the
      sentinel `givn missing-testrunner` (ADR-021).
- [ ] 1.6 Author `assets/config.yaml.tmpl`: the fully-commented, documented
      config scaffold (ADR-022) — every key present-but-commented with
      description/default/allowed-values, `verify.command` uncommented as the
      sentinel with a "CHANGE ME" note, and a `givn_config_version` marker
      (ADR-023). Cross-reference arc42 ch.8.10.
- [ ] 1.7 Implement `cli::missing_testrunner`: print per-language setup guidance
      (cargo/mvn/cucumber-js/behave/go test/...) and exit non-zero (ADR-021).
- [ ] 1.8 Implement `config/render.rs` (initial): render `config.yaml.tmpl` with
      the current binary version into `givn_config_version`.
- [ ] 1.9 Implement `cli::init`: scaffold `givn/{specs,changes,archive}` and
      write the documented, versioned `givn/config.yaml`. Idempotent; `--force`.
- [ ] 1.10 Test: every documented key in `config.yaml.tmpl` maps to a real
      manifest schema key and vice versa (no drift — ADR-022).
- [ ] 1.11 Commit: "feat: embedded defaults, manifest model, documented config, givn init".

**Done when:** `givn init` scaffolds a project with a fully-documented, versioned
`givn/config.yaml`; `givn missing-testrunner` prints guidance and fails; manifest
loads from the binary.

---

## Phase 2 — Cascade & overlay

- [ ] 2.1 Implement `config/cascade.rs`: `resolve_facet(artifact_id, facet)` —
      project override path under `givn/` wins, else embedded default.
- [ ] 2.2 Decide & document the override path convention
      (e.g. `givn/artifacts/<id>/template.<ext>`, `.../instructions.md`).
- [ ] 2.3 Implement `config/overlay.rs`: merge `givn/config.yaml` onto the
      embedded default manifest (add by new id, deep-merge by existing id, `remove`
      directive, scalar replace incl. `verify.command`). Support `optional: bool`
      on artifacts. Unit tests for each case. (ADR-024)
- [ ] 2.3a Implement effective-graph validation: acyclic, no dangling `requires`,
      `apply.requires` may not reference a removed non-optional artifact; clear
      errors. Derive order from `requires`. Tests incl. remove-review and
      add-refactor reconfigurations. (ADR-024)
- [ ] 2.4 Implement `cli::eject` for `artifact:facet`, `skill:<name>`, `manifest`;
      refuse to overwrite without `--force`.
- [ ] 2.5 Implement `cli::instructions` (text + `--json`) using cascade + overlay.
- [ ] 2.6 Commit: "feat: cascade resolution, manifest overlay, eject, instructions".

**Done when:** ejecting a facet then editing it changes resolution; overlay tests
green; `givn instructions <a> --json` works.

---

## Phase 2.5 — Config versioning & upgrade

- [ ] 2.5.1 Define the managed-vs-user region convention for `givn/config.yaml`
      (commented lines = givn-managed scaffold; uncommented key/values = user-owned).
- [ ] 2.5.2 Extend `config/render.rs`: given an existing `config.yaml`, parse user
      overrides + `givn_config_version`, re-render the managed comment scaffold from
      the current template, preserve user overrides verbatim, keep removed/renamed
      keys with a note, bump the version.
- [ ] 2.5.3 Implement `cli::upgrade`: perform the above as ordinary file writes;
      print a summary; advise `git diff` review and `givn skills sync`.
- [ ] 2.5.4 Tests: idempotency (second run = no diff), override preservation,
      version bump, removed-key retention. (ADR-023)
- [ ] 2.5.5 Commit: "feat: versioned config with givn upgrade".

**Done when:** a 1.0.0-style config can be upgraded in place; user overrides are
preserved; re-running produces no diff; changes are reviewable via `git diff`.

---

## Phase 3 — Gherkin core

- [ ] 3.1 Implement `gherkin/parse.rs` wrapping the `gherkin` crate; surface
      parse errors and `@wip`/PENDING detection.
- [ ] 3.2 Implement `cli::lint` (the `givn lint` command — static `.feature`
      check; runs no tests): collect targets, classify findings, exit
      **0/1/2** (parse error > wip > clean). Tests for each exit code.
- [ ] 3.3 Implement `gherkin/delta.rs`: detect `@givn.delta`, capability tag, and
      `@givn.added/modified/removed` (default added); handle `@givn.removed`
      placeholder step; split features/scenarios by title.
- [ ] 3.4 Implement `gherkin/merge.rs`: apply added/modified/removed to a
      permanent `.feature` by scenario title; strip all `@givn.*` tags;
      all-or-nothing (validate before write). Round-trip integration tests
      (added/modified/removed, error cases: add-existing, modify-missing,
      remove-missing).
- [ ] 3.5 Commit: "feat: gherkin parse/lint and delta merge engine".

**Done when:** merge round-trip tests pass; `givn lint` exit codes correct;
failures abort without partial writes.

---

## Phase 4 — Lifecycle

- [ ] 4.1 Implement `state.rs`: derive change state from file presence
      (proposal/spec/design/tasks/review/archived); parse `tasks.md` checkboxes.
- [ ] 4.2 Implement `cli::new <id>`: validate kebab-case, scaffold
      `givn/changes/<id>/` with a proposal stub from cascade.
- [ ] 4.3 Implement `cli::status`: report derived state (+ task progress).
- [ ] 4.4 Implement `cli::apply --change <id>`: enforce `apply.requires`, report
      progress and next pending task group (givn tracks, does not execute).
- [ ] 4.5 Implement `cli::archive --change <id>`: check `apply.requires`
      (tasks complete AND review PASS), run merge into `givn/specs/`, move change
      to `givn/archive/<id>/`. Refuse + clear error on unmet gate or merge failure.
- [ ] 4.6 Decide review-PASS detection (string marker vs structured field) and
      implement.
- [ ] 4.7 Integration test: full lifecycle on a temp project.
- [ ] 4.8 Commit: "feat: lifecycle — new, status, apply, archive".

**Done when:** a change can go new → … → archive in a temp project with correct
state and merged specs.

---

## Phase 5 — Skills & AGENTS.md generation

- [ ] 5.1 Author default skill templates in `assets/skills/` (one per action:
      `givn-propose`, `givn-spec`, `givn-design`, `givn-tasks`, `givn-implement`,
      `givn-review`, `givn-archive`) and an `AGENTS.md` rules template. Port the
      proven SKILL.md content from the gherkinspec prototype.
- [ ] 5.2 Implement `skills/render.rs`: render templates with project context
      (cascade-resolved; ejectable).
- [ ] 5.3 Implement `skills/targets.rs`: write to `.agents/skills/<name>/SKILL.md`
      (primary) + mirrors `.claude/skills/`, `.opencode/skills/`, and root
      `AGENTS.md`. Validate SKILL.md frontmatter (name matches dir, description
      length).
- [ ] 5.4 Wire generation into `cli::init`; implement `cli::skills sync` for
      regeneration after upgrades.
- [ ] 5.5 Make skill templates ejectable (`givn eject skill:<name>`) — confirm
      cascade picks the override.
- [ ] 5.6 Commit: "feat: AGENTS.md + multi-target skill generation".

**Done when:** `givn init` and `givn skills sync` produce the three skill targets
+ AGENTS.md; ejected templates override defaults.

---

## Phase 5.5 — Addons & arc42 documentation artifact

- [ ] 5.5.1 Implement the `addons:` overlay sugar in `config/overlay.rs`: an
      enabled addon expands into its artifact definition(s). (ADR-026)
- [ ] 5.5.2 Author embedded `assets/addons/arc42/`: 12 arc42 chapter templates +
      an index template + the `arc42-docs` artifact instruction (create missing
      chapters, update only affected ones; durable project-level docs).
- [ ] 5.5.3 Define the `arc42-docs` artifact default: `requires: [design]`,
      `optional: true`, output `docs/arc42/` (configurable `arc42.output_dir`,
      `arc42.chapters`, `arc42.require_for_archive`).
- [ ] 5.5.4 Document the addon + arc42 keys in `config.yaml.tmpl` (commented).
- [ ] 5.5.5 Ensure filesystem-derived state observes the change-level marker for
      arc42 maintenance (durable docs themselves are not per-change artifacts).
- [ ] 5.5.6 Make addon templates/instruction ejectable; test that enabling the
      addon inserts the artifact after `design` and (optionally) gates archive.
- [ ] 5.5.7 Commit: "feat: addon mechanism + arc42 documentation artifact".

**Done when:** enabling `addons: { arc42: true }` adds an `arc42-docs` step after
design that creates/maintains durable `docs/arc42/` docs; everything configurable
and ejectable; graph validates.

---

## Phase 6 — Distribution

- [ ] 6.1 Add `cargo-dist` config; targets: linux x64+arm64 (prefer musl),
      macOS x64+arm64, windows x64.
- [ ] 6.2 Produce the `curl | sh` installer (installs to `~/.local/bin`, no root).
- [ ] 6.3 Generate the Homebrew tap formula via `cargo-dist`.
- [ ] 6.4 Prepare crates.io publish metadata (`cargo publish --dry-run`).
- [ ] 6.5 Write the install matrix into the user `README.md` (already drafted).
- [ ] 6.6 (When a remote exists) wire the release CI pipeline.
- [ ] 6.7 Commit: "build: cargo-dist multi-platform release + installers".

**Done when:** binaries build for all targets; installer works on a clean machine.

---

## Phase 7 — Dogfood

- [ ] 7.1 Copy/convert a sample (e.g. gherkintest) to use `givn/` and
      `GIVN_FEATURES` default `givn/specs`.
- [ ] 7.2 Run the full loop: `init → new → instructions → (author specs) →
      lint → run tests (verify.command) → apply → review → archive`.
- [ ] 7.3 Confirm merged specs are clean Gherkin and the runner passes against
      `givn/specs`.
- [ ] 7.4 Capture any rough edges as issues/follow-ups.
- [ ] 7.5 Commit: "test: end-to-end dogfood validation".

**Done when:** the full workflow works end-to-end on a real project.

---

## Deferred (post-v1)

- [ ] `givn migrate` (read legacy `openspec/` + `@openspec.*` → `givn/` + `@givn.*`).
- [ ] Additional built-in skill targets (cursor, gemini, …).
- [ ] Release CI once a git remote is added.
- [ ] Richer overlay deep-merge if needs surface.

---

## Cross-references

- Architecture & rationale: [`arc42/README.md`](arc42/README.md).
- Every binding decision: [`arc42/09-architecture-decisions.md`](arc42/09-architecture-decisions.md).
- Core mechanics: [`arc42/08-crosscutting-concepts.md`](arc42/08-crosscutting-concepts.md).
