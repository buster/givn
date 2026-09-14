# Design: author-change

## Technical Decisions

### `givn commit` — the SCM-neutral commit path

New `src/cli/commit.rs`, wired as a clap subcommand (`-m <message>`, optional
paths):

- `scm::select` refuses a project without a backend by name before writing.
- Git: stage the named paths (`stage_paths`, non-force) or every pending
  change (new `Worktree::stage_all()` = `git add -A`), then
  `commit_staged(stamped)`; the chained `commit-msg` hook may re-stamp, which
  is idempotent because stamping replaces managed trailers.
- Fossil: record the named paths (`stage_paths`, which skips paths that do
  not exist in the checkout) or record all changes (new
  `FossilCheckout::record_all_changes()` = `fossil addremove`), then
  `commit_staged(stamped)`.
- The stamped message comes from `attribution::stamped_message(message)` —
  the same trailer builder the `commit-msg` hook uses, gated on
  `GIVN_ATTRIBUTION_ACTIVE` so a commit without context stays unstamped
  (identical semantics on both backends).
- "Nothing to commit" is refused after staging, before any revision.

### Backend-aware attribution plumbing

`cli::git::install` (called by `skills sync`, `init`, and the upgrade) asks
the seam first: on Fossil it writes no hook and prints
`commit attribution: givn commit path only (raw fossil commit is not stamped)`;
Git and backend-less projects keep today's behavior (the shared hook file is
part of the scaffold and is activated only inside a Git worktree).
`FossilCheckout::stage_paths` skips managed paths that do not exist in the
checkout (a Git-only hook file has nothing to record), which keeps the
upgrade allowlist honest on Fossil.

### Init warning and report

`init::run` asks the seam once for the no-backend warning
(`warning: no Git worktree or Fossil checkout found at '<root>'` /
`warning: archive and managed upgrade are unavailable …`, exit 0 — Q7) and
selects the attribution summary line per backend.

### Generated guidance

`assets/skills/givn-implement/SKILL.md.tmpl` gains a static "Committing"
section that names `givn commit` (valid on every backend) and states the
limitation (`fossil commit` is not stamped). A static section needs no
conditional rendering (A15) and satisfies the elaboration's "the generated
skill states that".

## Production Impact

- `src/cli/commit.rs` (new), `src/main.rs` (subcommand), `src/cli/mod.rs`.
- `src/cli/attribution.rs`: public `stamped_message` (pure builder extracted
  from `stamp_message_values`).
- `src/git.rs`: `Worktree::stage_all`.
- `src/fossil.rs`: `record_all_changes`; `stage_paths` skips missing paths.
- `src/cli/git.rs`: backend-aware `install`.
- `src/cli/init.rs`: backend-aware attribution summary + the no-SCM warning.
- `assets/skills/givn-implement/SKILL.md.tmpl`: the committing note.
- `givn/specs/fragments/fragment.md`: registers
  `llm-attribution-installation` (the seed's open question — registered, not
  folded; the capability owns its own permanent feature and inventory).

User-visible surface: CLI text and generated guidance; no
`## Visual Design Contract` (exempt).

## Test Design

Delta scenarios: three `llm-attribution` wrapper scenarios, the
`llm-attribution-installation` Fossil-init `@e2e`, two
`init-agent-target-selection` `@e2e` scenarios (Fossil init guidance; no-SCM
warning), and one `prose-consolidation` guidance scenario.

| File | Binds |
|---|---|
| `tests/steps/llm_attribution_steps.rs` | The wrapper When steps (with/without attribution context) and the trailer-message Thens (backend-aware message reads). |
| `tests/steps/init_agent_target_selection_steps.rs` | The Fossil-checkout Given, the guidance/report/warning Thens, the bare-directory archive refusal. |

Fixtures: `common::make_fossil_root_bare()` (a Fossil checkout without the
`givn/` scaffold). Runner, strict mode, and the single-scenario command are
unchanged:

```sh
GIVN_FEATURES=givn/changes/author-change/specs cargo test --test features_runner -- --name "The SCM-neutral commit path attributes a Fossil check-in"
```

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| initialize a Fossil project (init) | Consumer initializes a Fossil checkout and receives backend-aware guidance | givn CLI | compiled binary runs `givn init` in a real Fossil checkout; asserts the generated guidance names `givn commit` |
| initialize without SCM | Consumer initializes a plain directory and receives a named warning | givn CLI | compiled binary in a bare directory; asserts the warning, generated skills, and the named archive refusal |
| initialize a Fossil project (installation) | Consumer initializes a Fossil project and receives the givn commit path without a hook | givn CLI | compiled binary; asserts the report line and that no `.githooks` directory exists |
| create an attributed revision | An attributed commit exposes the current LLM and harness | givn CLI | unchanged (Git hook); the wrapper scenarios are regular variants |
| other rows | unchanged | givn CLI | unchanged |

## Use-Case Traceability

| Guarantee or rule | Design decision | Capability | Evidence |
|---|---|---|---|
| `givn commit` is the SCM-neutral path (Q2) | The wrapper through the seam with the shared trailer builder | llm-attribution | The Git and Fossil wrapper scenarios |
| Fossil attribution exists only through `givn commit` | `install` writes no hook on Fossil; the guidance states it | llm-attribution-installation, prose-consolidation | The Fossil-init e2e and the guidance scenario |
| Init without an SCM warns and exits 0 (Q7) | The warning block in `init::run` | init-agent-target-selection | The plain-directory e2e |
| The installation report names what was skipped and why | Backend-aware report lines | llm-attribution-installation | The report assertions |
| Guidance names only valid commands | The static `givn commit` section in the template | prose-consolidation | The guidance scenario |
| Personas | developer--primary-user, agent--technical-role, givn-maintainer--owner | — | Review lens |

## Local Runnability

`./run-tests.sh`; external executables `git`, `fossil`, the compiled binary.
No digital twin. Obstacle fix: `givn commit` without a staged change is
refused after staging, so the scenario fixtures write a file first.
