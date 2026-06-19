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
      (`init`, `new`, `instructions`, `verify`, `status`, `apply`, `archive`,
      `eject`, `skills sync`) as stubs returning "not implemented".
- [ ] 1.2 Create `assets/` and wire `rust-embed` in `config/embedded.rs`.
- [ ] 1.3 Port the prototype `schema.yaml` → `assets/default-manifest.yaml`,
      renaming tags to `@givn.*` and paths to `givn/...`. Artifacts: proposal,
      specs, design, tasks, review; `apply.requires: [tasks, review]`;
      `apply.tracks: tasks.md`; `verify` block.
- [ ] 1.4 Port the 5 templates into `assets/templates/`
      (`proposal.md`, `spec.feature`, `design.md`, `tasks.md`, `review.md`),
      with `@givn.*` tags and `GIVN_FEATURES` default `givn/specs`.
- [ ] 1.5 Implement `config/manifest.rs` serde types; load + parse the embedded
      manifest; unit test it deserialises.
- [ ] 1.6 Implement `cli::init`: scaffold `givn/{specs,changes,archive}` and a
      minimal `givn/config.yaml` (with `verify.command` placeholder). Idempotent;
      `--force` to overwrite.
- [ ] 1.7 Commit: "feat: embedded defaults, manifest model, givn init".

**Done when:** `givn init` scaffolds a project; manifest loads from the binary.

---

## Phase 2 — Cascade & overlay

- [ ] 2.1 Implement `config/cascade.rs`: `resolve_facet(artifact_id, facet)` —
      project override path under `givn/` wins, else embedded default.
- [ ] 2.2 Decide & document the override path convention
      (e.g. `givn/artifacts/<id>/template.<ext>`, `.../instructions.md`).
- [ ] 2.3 Implement `config/overlay.rs`: merge `givn/config.yaml` onto the
      embedded default manifest (add by new id, deep-merge by existing id, remove
      directive, scalar replace incl. `verify.command`). Unit tests for each case.
- [ ] 2.4 Implement `cli::eject` for `artifact:facet`, `skill:<name>`, `manifest`;
      refuse to overwrite without `--force`.
- [ ] 2.5 Implement `cli::instructions` (text + `--json`) using cascade + overlay.
- [ ] 2.6 Commit: "feat: cascade resolution, manifest overlay, eject, instructions".

**Done when:** ejecting a facet then editing it changes resolution; overlay tests
green; `givn instructions <a> --json` works.

---

## Phase 3 — Gherkin core

- [ ] 3.1 Implement `gherkin/parse.rs` wrapping the `gherkin` crate; surface
      parse errors and `@wip`/PENDING detection.
- [ ] 3.2 Implement `cli::verify`: collect targets, classify findings, exit
      **0/1/2** (parse error > wip > clean). Tests for each exit code.
- [ ] 3.3 Implement `gherkin/delta.rs`: detect `@givn.delta`, capability tag, and
      `@givn.added/modified/removed` (default added); handle `@givn.removed`
      placeholder step; split features/scenarios by title.
- [ ] 3.4 Implement `gherkin/merge.rs`: apply added/modified/removed to a
      permanent `.feature` by scenario title; strip all `@givn.*` tags;
      all-or-nothing (validate before write). Round-trip integration tests
      (added/modified/removed, error cases: add-existing, modify-missing,
      remove-missing).
- [ ] 3.5 Commit: "feat: gherkin parse/verify and delta merge engine".

**Done when:** merge round-trip tests pass; verify exit codes correct; failures
abort without partial writes.

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
- [ ] 5.5 Make skill templates ejectable (`givn eject skill:<name>`) — verify
      cascade picks the override.
- [ ] 5.6 Commit: "feat: AGENTS.md + multi-target skill generation".

**Done when:** `givn init` and `givn skills sync` produce the three skill targets
+ AGENTS.md; ejected templates override defaults.

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
      verify → apply → review → archive`.
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
