# 6. Runtime View

Key runtime scenarios. Each shows the flow through the building blocks of
chapter 5.

## 6.1 `givn init`

```
user → cli::init
  1. detect project root (cwd)
  2. create givn/ skeleton: specs/, changes/, archive/
  3. config::render → write documented, versioned givn/config.yaml:
       - every key present-but-commented with description/default/allowed-values
       - verify.command uncommented as the sentinel "givn missing-testrunner"
       - givn_config_version set to the current binary version
  4. when config.yaml was (re)created: prompt for agent targets (multi-select
     list: opencode, .agents, .claude; default opencode) and persist the
     selection as the `skills:` block. Non-interactive (EOF) → default.
     Re-init on an existing config skips the prompt and keeps the selection.
  5. skills::render + skills::targets → write each skill template to the
     configured targets only (default: .opencode/skills/), and each command
     template to .opencode/commands/ only when opencode is selected.
     AGENTS.md is intentionally not touched (user-owned)
  6. print next steps (incl. "set verify.command to your test runner")
```

Idempotent: existing files are not overwritten unless `--force`.

## 6.2 `givn new <change-id>`

```
user → cli::new
  1. validate id:
       - non-empty
       - all chars are ASCII lowercase letters, digits, or hyphens
         (rejects uppercase, underscores, spaces, dots, non-ASCII)
        - does not start or end with a hyphen
      (note: a leading-hyphen id passed without `--` is intercepted by clap
       as an unexpected flag → exit 2, before this validator runs)
      (note: the empty string `""` is NOT intercepted by clap — it is a valid
       positional value, so `validate_id` runs and bails with
       "change id must not be empty" → exit 1, not exit 2)
  2. compute change_dir = <cwd>/givn/changes/<id>
  3. if change_dir already exists → bail "change '<id>' already exists at <abs path>"
  4. if <cwd>/givn/ does not exist → bail "givn/ not found. Run `givn init` first."
     (the "already exists" check precedes the "not found" check; in practice
      the already-exists branch is unreachable when givn/ is absent, because
      its parent cannot exist — see ch.11 TD-5)
  5. create givn/changes/<id>/
  6. config::cascade::resolve_template("proposal", "proposal.md"):
       - project override givn/artifacts/proposal/proposal.md if present
       - else embedded templates/proposal.md
       - (else embedded addons/proposal/template.md)
  7. write the resolved template verbatim to givn/changes/<id>/proposal.md
     (no placeholder substitution: the embedded template's `{{CHANGE_ID}}`
      heading is written literally — see ch.11 TD-5)
  8. print:
       Created change '<id>'
         <absolute path to proposal.md>

       Next: fill in proposal.md, then run:
         givn instructions proposal --change <id>
```

Only `proposal.md` is produced — no `specs/`, `design.md`, `tasks.md`, or
`review.md`. Exit 0 on success; any bail above exits 1 via the main error
mapper (clap argument errors exit 2 directly).

## 6.3 `givn instructions <artifact> --change <id> [--json]`

```
user/agent → cli::instructions
  1. config::manifest: load embedded default-manifest.yaml
  2. config::overlay: apply givn/config.yaml overlay
  3. resolve artifact node (id, generates, requires, ...)
  4. config::cascade: resolve `instructions` facet (project file ← embedded)
  5. emit text, or JSON {id, generates, requires, instruction} when --json
```

## 6.4 `givn lint [path]`

Static check of `.feature` files. Runs **no** tests (that is the test runner,
`verify.command` — see 6.9).

```
user/CI → cli::lint
  1. collect target .feature files (path arg or givn/specs + active change)
  2. gherkin::parse each:
       - parse error            → record (will exit 1)
       - @wip or PENDING steps  → record (will exit 2)
  3. report findings
  4. exit code: 1 if any parse error; else 2 if any wip; else 0 (clean)
```

Exit-code precedence: parse error (1) outranks wip (2) outranks clean (0).

## 6.5 `givn status --change <id>`

```
user/agent → cli::status
  1. config::manifest + overlay → load effective manifest
  2. state: walk artifact graph against filesystem → derive status per artifact
  3. print artifact checklist (✓ Done / ~ Partial / · Pending / ⊘ Skipped)
  4. if tasks artifact is Partial, read tasks.md and show next 3 pending
     task lines (the `[ ]` descriptors)
  5. print next required artifact, optional pending artifacts
```

`givn` tracks and reports; it does not drive the per-scenario TDD loop. That loop
is run by the agent following the instructions, invoking `verify.command` for each
RED/GREEN. givn *does* run `verify.command` at gate points via hooks — see
`givn check` (6.5b) and the archive gate (6.6).

## 6.5b `givn check [<artifact>] [--change <id>]`

Runs an artifact's `post` hooks (see ch.8.12). This is how givn executes the test
runner (`verify.command`) at gate points — e.g. after implementation and review.

```
user/agent/CI → cli::check
  1. config::manifest + overlay → resolve target artifact(s):
       - `givn check tasks`  → just the tasks artifact's post hooks
       - `givn check`        → post hooks of every artifact whose output exists
  2. hooks: for each post entry (verify | lint | shell):
       - set GIVN_FEATURES (active change features in dev; givn/specs after archive)
       - execute; stream output
  3. if any hook fails (non-zero) and not allow_failure → exit non-zero (gate fails)
     else → exit 0
```

Example: after the agent finishes `tasks`, `givn check tasks` runs the configured
test suite and confirms GREEN before review. With the sentinel default runner, it
fails loudly until `verify.command` is set.

## 6.6 `givn archive --change <id>`

```
user → cli::archive
  1. config::manifest + overlay → check apply.requires satisfied
       (tasks complete AND review.md present/PASS) else refuse
  1b. GATE: run hooks of every apply.requires artifact (e.g. tasks.post + review.post)
       → typically runs verify.command (the test suite). If any required hook
         fails (non-zero, no allow_failure), refuse archive and exit non-zero.
  2. for each delta givn/changes/<id>/specs/**/*.feature:
       gherkin::delta: read @givn.delta + scenario tags
       gherkin::merge: into givn/specs/<cap>/<cap>.feature
         - @givn.added    → append (error if title exists)
         - @givn.modified → replace by title (error if missing)
         - @givn.removed  → delete by title (error if missing)
       strip ALL @givn.* tags from the permanent output
  3. write merged permanent specs (clean, runnable Gherkin)
  4. post-merge verify gate:
       run verify.command against givn/specs/ → if non-zero, rollback and bail
  5. post-merge verify-e2e gate:
       if verify.e2e_command is empty AND merged specs contain @e2e → rollback and bail
         (mirrors per-change hook behaviour: @e2e without a configured runner is an
          unenforced gap, not an optional one)
       else if e2e_command is non-empty AND merged specs contain @e2e → run it;
         if non-zero, rollback and bail
       else if no @e2e in merged specs → skip silently
  6. coverage gate (if addons.coverage): see cli::coverage_gate
  7. move givn/changes/<id>/ → givn/archive/<id>/
  8. report summary
```

If any merge precondition fails (e.g. modify a missing scenario), archive aborts
without partial writes.

## 6.7 `givn eject <artifact>[:facet] | skill:<name> | manifest`

```
user → cli::eject
  1. resolve what to eject:
       - artifact:facet  → assets/templates or instruction for that artifact
       - skill:<name>    → assets/skills/<name>
       - manifest        → assets/default-manifest.yaml
  2. config::embedded: read the default bytes
  3. write into givn/ at the conventional override path
       (refuse to overwrite an existing override unless --force)
  4. print the path now under the user's control
```

## 6.8 `givn skills sync`

```
user → cli::skills
  1. skills::render all skill templates (cascade-resolved)
  2. config::overlay %load_skills_targets → resolve configured targets
     (default: opencode)
  3. skills::targets:
       - write each skill to the configured targets only
       - write .opencode/commands/<name>.md only when opencode is selected
       - AGENTS.md is intentionally not touched (user-owned)
  (Used after a binary upgrade or manifest change to refresh generated agent files.)
```

## 6.11 `givn status [--change <id>] [--json]`

`givn status` is the manifest-aware state view. It goes beyond listing which
files exist — it computes completion per artifact, derives the next required
step, and surfaces optional artifacts still pending.

```
user/agent/command → cli::status
  1. config::manifest + overlay → load effective manifest
  2. state::topological_sort → order artifacts by requires DAG
  3. for each artifact: check generates paths against filesystem
       - tasks.md: also parse [x]/[ ] checkboxes
       - review.md: also detect REVIEW: PASS marker
     → ArtifactStatus: Done | Partial | Pending
  4. derive:
       next_required  = first non-optional artifact not yet Done
       optional_pending = optional artifacts still Pending
       all_required_done = all non-optional artifacts are Done
  5. --json: structured output for machine consumption by commands/scripts
     default: human-readable checklist with next step instruction
```

This is the mechanism commands use to discover both the workflow sequence
and the current position within it at runtime — without hardcoding artifact
names or order. A project that adds a `risk-assessment` artifact sees it in
`givn status --json` automatically; `/givn-propose` follows `next_required`
to drive its loop regardless of how the manifest is configured.

## 6.9 Running the test runner on a fresh project (sentinel)

When a project has not configured `verify.command`, the manifest's default points
at the sentinel:

```
agent/user runs the configured verify.command (= "givn missing-testrunner")
  → cli::missing_testrunner
      1. print "no test runner configured" + how to set verify.command
      2. show per-language examples (cargo/mvn/cucumber-js/behave/go test/...)
      3. exit non-zero (treated as "not GREEN" by CI and agents)
```

Note: `givn lint` (static Gherkin check) is unaffected — it still runs and
returns 0/1/2. Only the *test runner* (`verify.command`) is the sentinel until
configured. The two are complementary: `givn lint` checks the spec is well-formed;
`verify.command` runs the tests that confirm behaviour (GREEN).

## 6.9a `givn completions <shell>`

```
user → main.rs (inline handler, no separate cli module)
  1. clap_complete::generate(shell, &mut Cli::command(), "givn", stdout)
     → emits a shell-specific completion script to stdout
  2. exit 0
```

The script is printed to stdout so the user can eval it, pipe it, or redirect
it to a completions file. No filesystem writes; no project config required.
Works before `givn init` (does not need `givn/config.yaml`).

Supported shells: `bash`, `zsh`, `fish`, `elvish`, `powershell` (all
`clap_complete::Shell` variants).

Typical one-time install:
```sh
# bash
echo 'eval "$(givn completions bash)"' >> ~/.bashrc

# zsh
echo 'eval "$(givn completions zsh)"' >> ~/.zshrc

# fish (persist to the completions dir — preferred over eval)
givn completions fish > ~/.config/fish/completions/givn.fish
```

## 6.9b `givn addons list / enable / disable`

```
user → cli::addons

  list:
    1. read givn/config.yaml (line scan) → parse which addons are enabled
    2. print each known addon with [enabled] / [disabled] status + description

  enable <name>:
    1. validate name against known addon registry (currently: arc42)
    2. read givn/config.yaml; locate or create the addons: block
    3. write addons.<name>: true (line-based edit; preserves all comments
       and other user overrides)
    4. print confirmation + next-step hint
       → givn status now shows arc42-docs in the workflow (optional, after design)

  disable <name>:
    1. same as enable but writes false
    2. existing docs/arc42/ files are not deleted
```

Enabling an addon is equivalent to adding the artifact manually in the overlay —
`addons: { arc42: true }` expands to the `arc42-docs` artifact definition during
`config::overlay::apply`. The CLI command is convenience sugar; power users may
edit `givn/config.yaml` directly.

## 6.10 `givn upgrade`

```
user → cli::upgrade
  1. read givn/config.yaml: parse user overrides + givn_config_version
  2. config::render: re-render the managed (commented) documentation scaffold
       from the current binary's template
  3. merge: preserve every user-uncommented key/value verbatim; refresh only
       the managed comment regions; keep removed/renamed keys with a note
  4. set givn_config_version to the current binary version
  5. write givn/config.yaml (ordinary file write)
  6. optionally run `skills sync` (or advise it)
  7. print summary; user reviews via `git diff` and resolves conflicts in git
```

Idempotent: running `givn upgrade` twice on the same binary produces no further
diff. Never deletes user-set values.
