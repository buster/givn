# givn — Implementation Task Breakdown

Phase-by-phase plan. Each phase ends green (compiles + tests pass) and is
committed. References to architecture: see [`arc42/`](arc42/README.md), especially
chapter 8 (cross-cutting concepts) and chapter 9 (decisions).

Legend: `[ ]` pending · `[x]` done.

---

## Phase 0 — Repository bootstrap

- [x] 0.1 `cargo init --bin` in `~/projects/givn` (crate name `givn`).
- [x] 0.2 Add `Cargo.toml` deps: `clap` (derive), `serde` (derive),
      `serde_yaml_ng`, `serde_json`, `rust-embed`, `gherkin`, `anyhow`,
      `thiserror`. Pin versions (see arc42 ch.5.4).
- [x] 0.3 Add `.gitignore` (`/target`, editor cruft).
- [x] 0.4 Verify `cargo build` and `cargo test` succeed (empty).
- [x] 0.5 Run `scripts/check-dep-age.sh` (TC-13): every locked dep ≥ 30 days old.
      Fix violations by pinning older versions; only allowlist with justification.
- [x] 0.6 Commit: "chore: bootstrap cargo project".

**Done when:** empty binary builds, runs `givn --help`, and passes the dependency
age policy (`scripts/check-dep-age.sh`).

> The script `scripts/check-dep-age.sh` already exists. It needs a `Cargo.lock`
> (created by 0.1) to run. Wire it into CI in Phase 6.

---

## Phase 1 — Skeleton & embedded defaults

- [x] 1.1 Define `clap` CLI skeleton in `main.rs` with all subcommands
      (`init`, `new`, `instructions`, `lint`, `check`, `status`, `apply`,
      `archive`, `eject`, `skills sync`, `upgrade`, `missing-testrunner`) as stubs
      returning "not implemented".
- [x] 1.2 Create `assets/` and wire `rust-embed` in `config/embedded.rs`.
- [x] 1.3 Port the prototype `schema.yaml` → `assets/default-manifest.yaml`,
      renaming tags to `@givn.*` and paths to `givn/...`. Artifacts: proposal,
      specs, design, tasks, review; `apply.requires: [tasks, review]`;
      `apply.tracks: tasks.md`; `verify` block; default hooks
      `tasks.post: [verify]` and `review.post: [verify]` (ADR-028).
- [x] 1.4 Port the 5 templates into `assets/templates/`
      (`proposal.md`, `spec.feature`, `design.md`, `tasks.md`, `review.md`),
      with `@givn.*` tags and `GIVN_FEATURES` default `givn/specs`.
- [x] 1.5 Implement `config/manifest.rs` serde types; load + parse the embedded
      manifest; unit test it deserialises. Default `verify.command` is the
      sentinel `givn missing-testrunner` (ADR-021).
- [x] 1.6 Author `assets/config.yaml.tmpl`: the fully-commented, documented
      config scaffold (ADR-022) — every key present-but-commented with
      description/default/allowed-values, `verify.command` uncommented as the
      sentinel with a "CHANGE ME" note, and a `givn_config_version` marker
      (ADR-023). Cross-reference arc42 ch.8.10.
- [x] 1.7 Implement `cli::missing_testrunner`: print per-language setup guidance
      (cargo/mvn/cucumber-js/behave/go test/...) and exit non-zero (ADR-021).
- [x] 1.8 Implement `config/render.rs` (initial): render `config.yaml.tmpl` with
      the current binary version into `givn_config_version`.
- [x] 1.9 Implement `cli::init`: scaffold `givn/{specs,changes,archive}` and
      write the documented, versioned `givn/config.yaml`. Idempotent; `--force`.
- [x] 1.10 Test: every documented key in `config.yaml.tmpl` maps to a real
      manifest schema key and vice versa (no drift — ADR-022).
- [x] 1.11 Commit: "feat: embedded defaults, manifest model, documented config, givn init".

**Done when:** `givn init` scaffolds a project with a fully-documented, versioned
`givn/config.yaml`; `givn missing-testrunner` prints guidance and fails; manifest
loads from the binary.

---

## Phase 2 — Cascade & overlay

- [x] 2.1 Implement `config/cascade.rs`: `resolve_facet(artifact_id, facet)` —
      project override path under `givn/` wins, else embedded default.
- [x] 2.2 Decide & document the override path convention
      (e.g. `givn/artifacts/<id>/template.<ext>`, `.../instructions.md`).
- [x] 2.3 Implement `config/overlay.rs`: merge `givn/config.yaml` onto the
      embedded default manifest (add by new id, deep-merge by existing id, `remove`
      directive, scalar replace incl. `verify.command`). Support `optional: bool`
      on artifacts. Unit tests for each case. (ADR-024)
- [x] 2.3a Implement effective-graph validation: acyclic, no dangling `requires`,
      `apply.requires` may not reference a removed non-optional artifact; clear
      errors. Derive order from `requires`. Tests incl. remove-review and
      add-refactor reconfigurations. (ADR-024)
- [x] 2.4 Implement `cli::eject` for `artifact:facet`, `skill:<name>`, `manifest`;
      refuse to overwrite without `--force`.
- [x] 2.5 Implement `cli::instructions` (text + `--json`) using cascade + overlay.
- [x] 2.6 Commit: "feat: cascade resolution, manifest overlay, eject, instructions".

**Done when:** ejecting a facet then editing it changes resolution; overlay tests
green; `givn instructions <a> --json` works.

---

## Phase 2.5 — Config versioning & upgrade

- [x] 2.5.1 Define the managed-vs-user region convention for `givn/config.yaml`
      (commented lines = givn-managed scaffold; uncommented key/values = user-owned).
- [x] 2.5.2 Extend `config/render.rs`: given an existing `config.yaml`, parse user
      overrides + `givn_config_version`, re-render the managed comment scaffold from
      the current template, preserve user overrides verbatim, keep removed/renamed
      keys with a note, bump the version.
- [x] 2.5.3 Implement `cli::upgrade`: perform the above as ordinary file writes;
      print a summary; advise `git diff` review and `givn skills sync`.
- [x] 2.5.4 Tests: idempotency (second run = no diff), override preservation,
      version bump, removed-key retention. (ADR-023)
- [x] 2.5.5 Commit: "feat: versioned config with givn upgrade".

**Done when:** a 1.0.0-style config can be upgraded in place; user overrides are
preserved; re-running produces no diff; changes are reviewable via `git diff`.

---

## Phase 3 — Gherkin core

- [x] 3.1 Implement `gherkin/parse.rs` wrapping the `gherkin` crate; surface
      parse errors and `@wip`/PENDING detection.
- [x] 3.2 Implement `cli::lint` (the `givn lint` command — static `.feature`
      check; runs no tests): collect targets, classify findings, exit
      **0/1/2** (parse error > wip > clean). Tests for each exit code.
- [x] 3.3 Implement `gherkin/delta.rs`: detect `@givn.delta`, capability tag, and
      `@givn.added/modified/removed` (default added); handle `@givn.removed`
      placeholder step; split features/scenarios by title.
- [x] 3.4 Implement `gherkin/merge.rs`: apply added/modified/removed to a
      permanent `.feature` by scenario title; strip all `@givn.*` tags;
      all-or-nothing (validate before write). Round-trip integration tests
      (added/modified/removed, error cases: add-existing, modify-missing,
      remove-missing).
- [x] 3.5 Commit: "feat: gherkin parse/lint and delta merge engine".

**Done when:** merge round-trip tests pass; `givn lint` exit codes correct;
failures abort without partial writes.

---

## Phase 4 — Lifecycle

- [x] 4.1 Implement `state.rs`: derive change state from file presence
      (proposal/spec/design/tasks/review/archived); parse `tasks.md` checkboxes.
- [x] 4.2 Implement `cli::new <id>`: validate kebab-case, scaffold
      `givn/changes/<id>/` with a proposal stub from cascade.
- [x] 4.3 Implement `cli::status`: report derived state (+ task progress).
- [x] 4.4 Implement `cli::apply --change <id>`: enforce `apply.requires`, report
      progress and next pending task group (givn tracks, does not execute).
- [x] 4.5 Implement `hooks.rs` (ADR-028): parse artifact `post` entries
      (built-in `verify` → run `verify.command`; `lint` → run `givn lint`; or a
      shell string; object form with `allow_failure`/`name`). Execute with
      `GIVN_FEATURES` set (active change in dev; `givn/specs` after archive),
      stream output, map failures to non-zero unless `allow_failure`.
- [x] 4.6 Implement `cli::check [<artifact>] [--change <id>]`: run an artifact's
      `post` hooks (or all artifacts with existing output); exit non-zero if a
      required hook fails. Tests: green passes, red blocks, sentinel fails loudly.
- [x] 4.7 Implement `cli::archive --change <id>`: check `apply.requires`
      (tasks complete AND review PASS); **run hooks of every `apply.requires`
      artifact as the gate** (refuse on failure); then merge into `givn/specs/`,
      move change to `givn/archive/<id>/`. Refuse + clear error on unmet gate,
      hook failure, or merge failure.
- [x] 4.8 Decide review-PASS detection (string marker vs structured field) and
      implement.
- [ ] 4.9 Integration test: full lifecycle on a temp project, incl. a red-suite
      run that blocks archive and a green run that passes the gate.
      (Deferred: unit + hook tests cover the individual gates; end-to-end
      validation will happen in Phase 7 dogfood.)
- [x] 4.10 Commit: "feat: lifecycle + post hooks (new, status, apply, check, archive)".

**Done when:** a change can go new → … → archive in a temp project; `givn check`
runs `verify.command`; archive is gated on the test suite being green (and fails
loudly under the sentinel default).

---

## Phase 5 — Skills & AGENTS.md generation

- [x] 5.1 Author default skill templates in `assets/skills/` (one per action:
      `givn-propose`, `givn-spec`, `givn-design`, `givn-tasks`, `givn-implement`,
      `givn-review`, `givn-archive`) and an `AGENTS.md` rules template. Port the
      proven SKILL.md content from the gherkinspec prototype.
- [x] 5.2 Implement `skills/render.rs`: render templates with project context
      (cascade-resolved; ejectable).
- [x] 5.3 Implement `skills/targets.rs`: write to `.agents/skills/<name>/SKILL.md`
      (primary) + mirrors `.claude/skills/`, `.opencode/skills/`, and root
      `AGENTS.md`. Validate SKILL.md frontmatter (name matches dir, description
      length).
- [x] 5.4 Wire generation into `cli::init`; implement `cli::skills sync` for
      regeneration after upgrades.
- [x] 5.5 Make skill templates ejectable (`givn eject skill:<name>`) — confirm
      cascade picks the override.
- [x] 5.6 Commit: "feat: AGENTS.md + multi-target skill generation".

**Done when:** `givn init` and `givn skills sync` produce the three skill targets
+ AGENTS.md; ejected templates override defaults.

---

## Phase 5.5 — Addons & arc42 documentation artifact

- [x] 5.5.1 Implement the `addons:` overlay sugar in `config/overlay.rs`: an
      enabled addon expands into its artifact definition(s). (ADR-026)
- [x] 5.5.2 Author embedded `assets/addons/arc42-docs/`: 12 arc42 chapter templates +
      an index template + the `arc42-docs` artifact instruction (create missing
      chapters, update only affected ones; durable project-level docs).
- [x] 5.5.3 Define the `arc42-docs` artifact default: `requires: [design]`,
      `optional: true`, change-level marker `arc42.md` with `STATUS: DONE`.
- [x] 5.5.4 Document the addon + arc42 keys in `config.yaml.tmpl` (commented).
- [x] 5.5.5 Ensure filesystem-derived state observes the change-level `arc42.md`
      marker (`STATUS: DONE` detection, parallel to `REVIEW: PASS`).
- [x] 5.5.6 Make addon templates/instruction ejectable via cascade fallback
      (`addons/<id>/`); eject uses effective manifest so addon artifacts work.
- [x] 5.5.7 Implement `givn completions <shell>`: add `clap_complete` dep; emit
      shell completion scripts for bash/zsh/fish/elvish/powershell to stdout.
      Visible in `--help` with one-line install instructions per shell.
- [x] 5.5.8 Implement `givn addons list`: list all known addons with name,
      description, and enabled/disabled status for the current project.
- [x] 5.5.9 Implement `givn addons enable <name>` / `givn addons disable <name>`:
      toggle the addon switch in `givn/config.yaml`; print confirmation + next step.
- [x] 5.5.10 Commit: "feat: addon mechanism + arc42 documentation artifact".

**Done when:** enabling `addons: { arc42: true }` adds an `arc42-docs` step after
design that creates/maintains durable `docs/arc42/` docs; everything configurable
and ejectable; graph validates; `givn completions <shell>` emits a usable
completion script.

---

## Phase 5.6 — Prompt fixes (ADR-029)

- [x] 5.6.1 Fix `assets/commands/givn-tasks.md`: add mandatory "Design constraints"
      block to the scenario task template, populated from design.md per scenario.
      Update GREEN instruction: "minimum code following the design constraints
      exactly". Add guardrail: a task without a populated constraints block is
      incomplete. (ADR-029)
- [x] 5.6.2 Fix `assets/commands/givn-implement.md`: step 4 requires full
      design.md passed to every subagent (not "relevant sections"); if a task
      lacks a constraints block, add it before spawning. Subagent prompt gains
      pre-RED "read design.md in full" step. GREEN sharpened: "minimum lines,
      not simpler architecture". Orchestrator guardrail: reject GREEN that
      deviates from the design and re-spawn. (ADR-029)
- [x] 5.6.3 Fix `assets/commands/givn-propose.md`: architect's tasks section
      makes "Design constraints" block mandatory per scenario task, with example
      structure and explicit rationale. (ADR-029)
- [x] 5.6.4 Record ADR-029 in `docs/arc42/09-architecture-decisions.md`.
- [x] 5.6.5 Commit: "fix(prompts): embed design constraints in tasks; enforce
      design adherence in implement".

**Done when:** every scenario task generated by `/givn-propose` or `/givn-tasks`
includes a populated "Design constraints" block; coding subagents read design.md
before writing code; GREEN means scenario passes AND design is followed.

---

## Phase 5.7 — Design-review artifact (ADR-030)

- [x] 5.7.1 Add `design-review` artifact to `assets/default-manifest.yaml`:
      `requires: [design]`, `optional: false`, `post: []`. Update `tasks` to
      `requires: [design-review]`. `review` stays `requires: [tasks]`.
      `apply.requires` unchanged: `[tasks, review]`. (ADR-030)
- [x] 5.7.2 Write `assets/instructions/design-review.md`: two mandatory phases
      (grilling + hardening); one-question-at-a-time constraint; required branches
      (scope, tech, missing scenarios, testability, risk); hardening edits;
      sign-off via `DESIGN-REVIEW: PASS`.
- [x] 5.7.3 Write `assets/templates/design-review.md`: grilling log table,
      codebase-findings table, open questions table, hardening changes table,
      sign-off checklist, `DESIGN-REVIEW: PASS` marker.
- [x] 5.7.4 Write `assets/commands/givn-design-review.md`: orchestrator that
      spawns a grilling subagent (clean context, returns ranked questions with
      recommended answers), runs interactive one-question-at-a-time loop with
      the user, then spawns a hardening subagent (applies decisions, writes
      design-review.md).
- [x] 5.7.5 Write `assets/skills/givn-design-review/SKILL.md.tmpl`: skill
      summarising both phases.
- [x] 5.7.6 Add `detect_design_review_pass()` to `src/state.rs` (parallel to
      `detect_review_pass()`); wire into `artifact_status()`; add unit test;
      update `topological_sort_respects_requires` test for new order
      (design < design-review < tasks < review).
- [x] 5.7.7 Update `src/cli/archive.rs` error message to cover both sign-off
      markers (`DESIGN-REVIEW: PASS` and `REVIEW: PASS`).
- [x] 5.7.8 Update `docs/arc32/08-crosscutting-concepts.md`: state table,
      marker conventions, project layout, apply.requires note.
- [x] 5.7.9 Record ADR-030 in `docs/arc42/09-architecture-decisions.md`;
      update ADR-024 sequence reference.
- [x] 5.7.10 Update `docs/arc42/01-introduction-and-goals.md` and
      `docs/arc42/03-context-and-scope.md` sequence references.
- [x] 5.7.11 Update `README.md` workflow sequence and quickstart.
- [x] 5.7.12 Commit.

**Done when:** after design is written, `/givn-design-review` spawns a grilling
subagent, the orchestrator presents questions one at a time, a hardening
subagent applies decisions and writes `design-review.md` with
`DESIGN-REVIEW: PASS`; tasks cannot start until the marker is present; 48 tests
pass.

---

## Phase 6 — Distribution

- [ ] 6.1 Add `cargo-dist` config; targets: linux x64+arm64 (prefer musl),
      macOS x64+arm64, windows x64.
- [ ] 6.2 Produce the `curl | sh` installer (installs to `~/.local/bin`, no root).
- [ ] 6.3 Generate the Homebrew tap formula via `cargo-dist`.
- [ ] 6.4 Prepare crates.io publish metadata (`cargo publish --dry-run`).
- [ ] 6.5 Write the install matrix into the user `README.md` (already drafted).
- [ ] 6.6 (When a remote exists) wire the release CI pipeline.
- [ ] 6.7 Add a CI job running `scripts/check-dep-age.sh` (TC-13) on every PR and
      before release; fail the build on policy violation.
- [ ] 6.8 Commit: "build: cargo-dist multi-platform release + installers + dep-age CI".

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
