# Design: template-engine-conditional-rendering

## Overview

The generated agent guidance (skills under `.opencode/skills/`, `.agents/skills/`,
`.claude/skills/`, and commands under `.opencode/commands/`) is currently
written two ways: skills go through a 4-variable string substitution
(`src/skills/render.rs::render_skill`), commands are copied verbatim from
`assets/commands/*.md`. Neither is aware of which addons the project has
enabled. Templates compensate by embedding two-state prose
("If `addons.arc42` is disabled, record `N/A …`"), so a project without the
Addon still reads guidance about it.

This change makes generation configuration-aware: templates mark the sections
that only apply when an addon is enabled, the renderer resolves those marks
against the effective addon set, and generated files contain only matching
content. Toggling an addon regenerates the affected files so generated
guidance always matches the current configuration.

**Universal invariant (user decision):** with an addon disabled, no generated
skill or command file may contain a reference to that addon — neither its
name/artifacts (`arc42`, `docs/arc42/`, `arc42.md`, `addons.arc42`,
`coverage.measure_command`, …) nor a workflow step that exists only because of
it. Two moves achieve it:
- **Gate**: addon-owned sections, rows, and bullets are wrapped in a
  conditional so they vanish when the addon is off.
- **Reword**: where a sentence names an addon only as an incidental pointer
  and the action itself applies regardless, the sentence is reworded to drop
  the addon name (guidance stays complete, the reference is gone).

## Technology decisions

- **Conditional template rendering: a minimal, self-contained directive
  engine — no new dependency.** The only capabilities needed are conditional
  inclusion, an optional else-branch, and nesting, over a string-keyed
  boolean map. A full template crate (handlebars/minijinja/tera) is rejected
  because:
  - Its default delimiters collide with the existing `{{CHANGE_ID}}`,
    `{{VERIFY_COMMAND}}`, `{{FEATURES_PATH}}`, `{{GIVN_VERSION}}` slots,
    forcing custom delimiters or a full syntax migration of 18 assets.
  - Loops, partials, filters, and HTML escaping are unused; Markdown output
    has no escaping needs.
  - The project currently ships plain-string template rendering
    (`render.rs`), so the marginal complexity of a ~60-line directive
    resolver is small and fully unit-testable.
  The syntax is deliberately handlebars-shaped so LLM authors editing
  templates recognise it, and conjunctions are expressed by nesting (no
  grammar extension needed):
  ```
  {{#if addons.arc42}}
  …applies only when the arc42 addon is enabled…
  {{#if addons.domain_modeling}}
  …applies only when both addons are enabled…
  {{/if}}
  {{/if}}
  ```
- **No other new technology.** No new crates, no version lookup required
  (Rust toolchain: the repo's existing pins; cucumber-rs is the existing
  runner).

## Conditional directive specification

The renderer processes templates textually:

1. **Conditional blocks**: `{{#if addons.<name>}}` … `{{/if}}`. A block is kept
   if the named addon is enabled, dropped otherwise. An `{{else}}` marker
   switches the kept branch. Blocks may nest. Directives are recognized on
   their own line and the directive lines themselves are removed from the
   output (so a fenced code block that wraps a conditioned region keeps intact
   fences).
2. **Supported addon keys**: exactly the known addon ids — `arc42`,
   `domain_modeling`, `dev_principles`, `grill_me`, `grill_with_docs`,
   `absolute_mode`, `readme`, `coverage`.
3. **Fail-closed errors** (any error aborts the whole `skills sync` with a
   non-zero exit and a message naming the template file):
   - unknown addon name in a condition (not in the known set);
   - unterminated `{{#if …}}` / stray `{{/if}}` (mismatched nesting).
4. **Variable substitution** (after conditional resolution): the existing four
   `{{VAR}}` replacements still apply.

## Architecture impact

### `src/skills/render.rs` — the renderer becomes the template engine

- Add `addon_enabled: &[(&'static str, bool)]` to the render context.
- Replace `render_skill` with `render_template(template, ctx, addons) ->
  Result<String>` implementing the directive spec; update the single caller
  (`src/skills/targets.rs`); keep a thin `render_skill` wrapper only if a
  caller needs it (prefer updating the caller).
- Unit tests: kept/dropped branch, `{{else}}`, nesting, under/over matches,
  unknown addon error, stray `{{/if}}` error, `{{VAR}}` still substituted.

### `src/skills/targets.rs` — single generation entry, config-aware

- Build the addon map from `load_effective().addons` once per sync; pass it
  into **both** skill rendering and command rendering (commands gain
  conditional support; the four variable replacements are no-ops for them).
- **Coupling**: sync `givn-domain-modeling` only when BOTH
  `addons.domain_modeling` and `addons.arc42` are enabled. Mirror in
  `managed_generated_paths` (the upgrade-migration allowlist).
- **Self-reconciliation**: after writing, remove managed generated files that
  are no longer valid for the current configuration (the domain-modeling
  skill when arc42 or domain_modeling is off). Extract the skill-file path
  computation into a shared helper reused by `addons.rs` removal. One removal
  path only — inside sync.
- Any sync error aborts with non-zero and names the offending template.

### `src/cli/addons.rs` — every toggle regenerates

- Split each of `run_enable`/`run_disable` into an inner scaffolding function
  plus a caller that syncs. A standalone `givn addons enable <name>` /
  `givn addons disable <name>` runs the scaffolding **and then
  `targets::sync`** for every addon (not only skill/command addons), so
  conditional content updates immediately when arc42, domain_modeling,
  coverage, etc. flip. `givn init` keeps its current single end-of-init sync
  by calling the non-syncing scaffolding variants for each addon — it must not
  re-sync once per addon.
- Keep the existing skill/command-file removal for skill and command addons
  (harmless; sync is idempotent after it).
- On `disable arc42`, the sync-driven reconciliation removes the generated
  `givn-domain-modeling` skill because the coupled condition no longer holds
  (no special-case removal needed in `addons.rs`).

### Template edits — complete table (universal)

| Asset | Region | Condition | Move |
|---|---|---|---|
| `assets/skills/givn-review/SKILL.md.tmpl` | “## Arc42 implementation conformance” section (currently “(conditional)”; prologue “If `addons.arc42` is disabled … / Otherwise,…” lines 125-126) | `addons.arc42` | Gate; heading loses “(conditional)”; prologue removed |
| `assets/skills/givn-review/SKILL.md.tmpl` | “## Ubiquitous language conformance” section (currently “(conditional)”; prologue 142-143) | `addons.arc42` + `addons.domain_modeling` (nested — writes into the arc42 glossary) | Gate; heading loses “(conditional)”; prologue removed |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | “ADR qualification” branch (55-60) | `addons.arc42` (ADR qualification targets the arc42 chapter-09/adr infra) | Gate |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | “Architecture documentation (arc42)” branch (61-74; prologue 61 “N/A if `addons.arc42` is not enabled. Otherwise,…” ) | `addons.arc42` | Gate; prologue removed |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | “Ubiquitous language” row (75) | `addons.arc42` + `addons.domain_modeling` (nested) | Gate |
| `assets/skills/givn-tasks/SKILL.md.tmpl` | “(and reassess arc42 chapters if structural)” parenthetical (124) | `addons.arc42` | Gate the parenthetical |
| `assets/skills/givn-archive/SKILL.md.tmpl` | README-update items 2-3 (“Coverage badges”/“Coverage section”), the whole coverage example (77-105), and the trailing coverage passage (107-110) | `addons.coverage` | Gate; generic item 1 (project name) stays |
| `assets/commands/givn-review.md` | fabrication-audit parenthetical “(reassess arc42 chapters if structural)” (120) | `addons.arc42` | Gate the parenthetical |
| `assets/commands/givn-review.md` | “### Arc42 implementation conformance” section (130-143; prologue 131-132 incl. “N/A when the arc42 addon is disabled”) | `addons.arc42` | Gate; heading loses “(conditional)”; prologue removed |
| `assets/commands/givn-design-review.md` | input bullet “If the arc42 addon is enabled …” (117-127) | `addons.arc42` | Gate the bullet |
| `assets/commands/givn-design-review.md` | “Architecture documentation (arc42)” branch (152-186; prologue 152-154 “skip this branch entirely … if `addons.arc42` is not `true`”) | `addons.arc42` | Gate; prologue removed |
| `assets/commands/givn-design-review.md` | “## Architecture documentation (arc42) check” grid + checkboxes (310-329; prologue 311 “N/A if…”) | `addons.arc42` | Gate; prologue removed |
| `assets/commands/givn-design-review.md` | Sign-off checkbox “Architecture documentation (arc42) check completed above, or marked N/A…” (330-331) | `addons.arc42` | Gate the checkbox |
| `assets/commands/givn-tasks.md` | “(reassess arc42 chapters if structural)” parenthetical (150) | `addons.arc42` | Gate the parenthetical |
| `assets/commands/grill-with-docs.md` | source sentence “an arc42 chapter,” (9) | — | Reword to “an architecture chapter,” (no addon name) |
| `assets/commands/grill-with-docs.md` | inventory bullet “`docs/arc42/` (if the arc42 addon is enabled)” (22) | `addons.arc42` | Gate the bullet; drop the inline “(if …)” |

**Kept intentionally (not violations — the addon is on whenever they are
generated/read):**
- `givn-domain-modeling` template’s “Requires the arc42 addon — enable it
  first…” line: the skill is only generated when `domain_modeling && arc42`;
  the line documents a hard prerequisite and a permanent e2e scenario asserts
  it.
- `assets/instructions/*` and runtime `givn instructions <artifact>` output:
  printed on demand, not generated files; the user boundary is generated
  skills and commands. The instructions keep their two-state guidance.
- `assets/templates/review.md` (the ejected review.md template): not a
  skill/command.

### Permanent-spec modifications

- `givn/specs/arc42-review-alignment/arc42-review-alignment.feature`,
  scenario “Generated review guidance stays aligned after skill sync”
  (lines 27-33): drop the assertion
  `the generated review guidance contains "N/A when the arc42 addon is
  disabled"` (line 33). The N/A prologue is removed from generated guidance as
  part of this change; the remaining assertions (contains “Arc42
  implementation conformance”, …) still hold with the default (arc42 enabled)
  fixture. Handled as `@givn.modified` scenarios in this change’s delta.
- No other permanent scenario or step asserts a string touched by the table
  above (verified by search: the remaining matches read raw template assets
  whose text is retained inside conditionals, or CLI stdout).

## Data model changes

None. The effective manifest already carries the addon booleans; no new
config keys.

## Step definition locations

- `tests/steps/template_conditional_rendering_steps.rs` — non-e2e steps:
  build fixtures from `crate::common::make_init_root()`, set addon state by
  editing `givn/config.yaml` **directly** (then `skills sync`) or via the real
  CLI only where the scenario drives the toggle, and read generated files:
  `.opencode/skills/givn-review/SKILL.md`,
  `.opencode/skills/givn-design-review/SKILL.md`,
  `.opencode/commands/givn-review.md`,
  `.opencode/commands/givn-design-review.md`,
  `.opencode/skills/givn-domain-modeling/SKILL.md`.
  The “no generated skill or command mentions a disabled addon” step scans
  every `.opencode/skills/**/SKILL.md` and `.opencode/commands/*.md` for a
  fixed marker list (case-insensitive `arc42`, `glossary`, `ubiquitous`,
  `domain-modeling`, plus `Coverage badge`, `coverage.measure_command`,
  `coverage.output_path`, `givn:begin:coverage`).
  Error scenarios write project-override templates under the fixture’s
  `givn/skills/…` / `givn/commands/…` and assert `skills sync` fails with the
  file named.
- `tests/steps/template_conditional_rendering_e2e_steps.rs` — the three
  `@e2e` scenarios drive the compiled CLI binary (`crate::common::run_givn`)
  as a real subprocess for `skills sync`, `addons enable arc42`,
  `addons disable domain_modeling`.
- Registered in `tests/steps/mod.rs`.

## Test runner

- **Unit/integration** (`verify.command`): `./run-tests.sh`.
- **Strict mode**: `tests/features_runner.rs` calls `.fail_on_skipped()`
  (cucumber-rs), so an undefined/pending step fails the run. New step bodies
  are implemented for real; no `unimplemented!()` stubs shipped.
- **Single-scenario run** (cucumber-rs `--name` filter, verified working):
  `env -u GIVN_SPEC_BGE -u GIVN_SPEC_RERANKER -u GIVN_SPEC_MAX_RERANK_PAIRS GIVN_FEATURES=givn/changes/template-engine-conditional-rendering/specs cargo test --test features_runner -- --name "<scenario title>"`.
- **Machine checks**: `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --check`, `cargo test --lib`.

## E2E smoke test infrastructure

- **E2E runner** (`verify.e2e_command`): `./run-tests.sh --e2e` (the
  `@e2e`-tagged scenarios run against the compiled binary via `run_givn`).
- **E2E step location**: `tests/steps/template_conditional_rendering_e2e_steps.rs`.
- **Local test infrastructure**: none beyond the compiled `givn` binary and a
  temporary project root (`make_init_root`); the CLI is the system under test.
- **E2E framework**: `cucumber-rs`, the project’s existing runner; it
  subprocess-invokes `CARGO_BIN_EXE_givn` — the real interface for a CLI.
- **E2E strict-mode proof**: same `.fail_on_skipped()` runner; an undefined
  e2e step fails with a non-zero exit.

### Interaction coverage matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| synchronize generated skills and commands through the Givn CLI | “A consumer synchronizes skills and the generated guidance matches the addon configuration” | CLI | `run_givn(env, ["skills", "sync"])` subprocess; reads generated files afterwards |
| enable an addon through the Givn CLI | “A consumer enables the Arc42 addon and the generated guidance gains the Arc42 section” | CLI | `run_givn(env, ["addons", "enable", "arc42"])` subprocess; reads generated files afterwards |
| disable an addon through the Givn CLI | “A consumer disables the domain-modeling addon and the generated domain skill is removed” | CLI | `run_givn(env, ["addons", "disable", "domain_modeling"])` subprocess; reads generated files afterwards |

## Black-Box-First justification

All scenarios go through the real CLI in a fixture project and assert on the
**generated files**, which is the observable boundary for a code generator.

- Non-e2e fixtures set addon state by editing `givn/config.yaml` directly and
  then running `skills sync`, so auto-regeneration on toggle is exercised only
  by the e2e Whens (the real feature of the enable/disable actions). The
  `disable arc42` cleanup (a variant of the disable action) and the two error
  variants are regular scenarios.
- Distinct invariants per scenario: review **command** omission (e2e sync
  asserts only the skill + coupling); design-review **skill** and **command**
  arcs (no e2e asserts them); present-state domain skill (e2e asserts only
  absence paths); the universal residual scan (no single-file e2e).
- The inclusion-side scenarios (arc42/UBL/design-review **present** when
  enabled) are GREEN-only regression guards: today’s renderer includes
  everything, so they cannot fail in RED. This is accepted and documented;
  the deletion-side scenarios drive RED.
- The two malformed-directive scenarios genuinely fail in RED (sync errors
  only after the fail-closed renderer exists).

## Local runnability & digital twins

- **Local run command**: `cargo build --release` then
  `GIVN_FEATURES=<features> ./run-tests.sh` inside the checkout. The system
  is the CLI itself; there is no server or daemon to start.
- **Isolated network**: none required — no network service.
- **Digital twins**: none — the CLI talks to no third-party service.

## Risks

- **Template drift**: a template author could wrap a section in a condition
  whose spelling diverges from the addon key. Mitigated: the directive only
  accepts the exact known-addon keys and errors otherwise; the grammar is
  unit-tested; the universal residual scan (spec scenario “No generated skill
  or command mentions a disabled addon”) machine-verifies the invariant in an
  arc42+domain_modeling+coverage-off fixture.
- **Stale generated files**: a file generated before a config change could
  survive a later sync. Mitigated: sync self-reconciles (removes generated
  files no longer valid for the current config) and every addon toggle
  re-syncs.
- **Fenced blocks**: a conditioned region inside a code fence must keep the
  fence intact. Mitigated: the renderer removes only whole directive lines and
  leaves all other lines untouched; the wrapped regions are inside single,
  pre-existing fences.
