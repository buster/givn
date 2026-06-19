# 6. Runtime View

Key runtime scenarios. Each shows the flow through the building blocks of
chapter 5.

## 6.1 `givn init`

```
user → cli::init
  1. detect project root (cwd)
  2. create givn/ skeleton: specs/, changes/, archive/
  3. write minimal givn/config.yaml (e.g. `verify: { command: "..." }`)
  4. skills::render + skills::targets:
       - render AGENTS.md from embedded rules template → repo root
       - render each default skill template → .agents/skills/<name>/SKILL.md
       - mirror to .claude/skills/ and .opencode/skills/
  5. print next steps
```

Idempotent: existing files are not overwritten unless `--force`.

## 6.2 `givn new <change-id>`

```
user → cli::new
  1. validate id (kebab-case), ensure givn/changes/<id> absent
  2. create givn/changes/<id>/
  3. write proposal.md stub (from cascade: project override or embedded template)
  4. print: run `givn instructions proposal --change <id> --json`
```

## 6.3 `givn instructions <artifact> --change <id> [--json]`

```
user/agent → cli::instructions
  1. config::manifest: load embedded default-manifest.yaml
  2. config::overlay: apply givn/config.yaml overlay
  3. resolve artifact node (id, generates, requires, ...)
  4. config::cascade: resolve `instructions` facet (project file ← embedded)
  5. emit text, or JSON {id, generates, requires, instruction} when --json
```

## 6.4 `givn verify [path]`

```
user/CI → cli::verify
  1. collect target .feature files (path arg or givn/specs + active change)
  2. gherkin::parse each:
       - parse error            → record (will exit 1)
       - @wip or PENDING steps  → record (will exit 2)
  3. report findings
  4. exit code: 1 if any parse error; else 2 if any wip; else 0 (GREEN)
```

Exit-code precedence: parse error (1) outranks wip (2) outranks clean (0).

## 6.5 `givn apply --change <id>`

```
user/agent → cli::apply
  1. config::manifest + overlay → read apply.requires, apply.tracks (tasks.md)
  2. state: verify required artifacts exist (e.g. tasks, review per apply.requires)
  3. parse tasks.md checkboxes: count [ ] vs [x]
  4. report progress and the next pending task group
  (Execution of tasks is performed by the developer/agent, not by givn.)
```

`givn` tracks and reports; it does not itself run the TDD loop. The loop is
driven by the agent following the instructions, invoking the manifest's
`verify.command` for RED/GREEN.

## 6.6 `givn archive --change <id>`

```
user → cli::archive
  1. config::manifest + overlay → check apply.requires satisfied
       (tasks complete AND review.md present/PASS) else refuse
  2. for each delta givn/changes/<id>/specs/**/*.feature:
       gherkin::delta: read @givn.delta + scenario tags
       gherkin::merge: into givn/specs/<cap>/<cap>.feature
         - @givn.added    → append (error if title exists)
         - @givn.modified → replace by title (error if missing)
         - @givn.removed  → delete by title (error if missing)
       strip ALL @givn.* tags from the permanent output
  3. write merged permanent specs (clean, runnable Gherkin)
  4. move givn/changes/<id>/ → givn/archive/<id>/
  5. report summary
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
  1. skills::render all skill templates (cascade-resolved) + AGENTS.md
  2. skills::targets: rewrite .agents/.claude/.opencode + root AGENTS.md
  (Used after a binary upgrade to refresh generated agent files.)
```
