# Proposal: reverse-engineer-new-command

## Capability

This change characterizes the **existing** `new` command of the `givn` CLI —
the capability that scaffolds a fresh change directory.

The black-box boundary for this capability is: **the CLI surface
`givn new <id>`**, observed through its exit code, stdout, and filesystem
effects (the created directory tree and the contents of every file written
inside it). Nothing inside the process — no functions, modules, config
resolution mechanics, or template engine internals — is in scope as an
observation target; they are only reachable through that CLI surface.

## Current Behaviour

Today, invoking `givn new <id>` from a project root:

- Accepts an `<id>` and validates it before touching the filesystem.
- A valid id is non-empty, consists only of lowercase ASCII letters, digits,
  and hyphens, and does not begin or end with a hyphen. Single-word ids and
  ids containing digits are accepted.
- When the id is rejected, the command prints a single error line to stderr
  and exits non-zero; no files are written.
- When the id is accepted **and** the project has been initialised (a `givn/`
  directory is present), the command creates `givn/changes/<id>/` and writes
  exactly one file inside it: `proposal.md`.
- The written `proposal.md` is resolved from a template cascade: a project
  override file is used if present, otherwise a built-in default template.
- The built-in default template contains a literal `{{CHANGE_ID}}` placeholder
  in its heading, and this placeholder is written verbatim into the created
  file — the user receives a file that still says `{{CHANGE_ID}}` rather than
  the id they supplied. (This is a suspected bug — see Out of Scope.)
- On success the command prints a confirmation line naming the id, the
  absolute path of the created `proposal.md`, and a short "next steps" hint,
  then exits zero.
- Only `proposal.md` is created — no `specs/`, `design.md`, `tasks.md`, or
  `review.md` are produced by this command.
- If the project has not been initialised (`givn/` absent), the command
  refuses with a "not found, run init first" message and exits non-zero.
- If a change with the given id already exists, the command refuses with an
  "already exists" message naming the absolute path and exits non-zero.

All behaviour described above is **provisional** — inferred from probing the
real binary. It will be validated scenario-by-scenario during characterization.
Where a spec assertion is proven wrong against the live code, the spec is
corrected to match AS-IS behaviour; production code is never changed.

## Out of Scope

Suspected bugs and code smells discovered during discovery. These are flagged
as follow-up `@givn.modified` / `@givn.removed` candidates for a future
prescriptive change. They are **not** addressed here — this change only
captures AS-IS behaviour.

- **BUG-1 — `{{CHANGE_ID}}` placeholder not substituted.** The built-in
  proposal template's heading reads `# Proposal: {{CHANGE_ID}}`, but the
  command writes the resolved template verbatim with no substitution. The
  user gets a file containing the literal text `{{CHANGE_ID}}` instead of
  their change id. Candidate: future `@givn.modified` delta to substitute
  the id (or remove the placeholder from the template).
- **SMELL-1 — Validation/initialisation check ordering.** The "change
  already exists" check is performed before the "givn/ not found" check.
  In practice the "already exists" branch is unreachable when `givn/` is
  absent (the change directory cannot exist without its `givn/` parent), so
  no wrong error is produced today — but the ordering is brittle and
  non-obvious. Candidate: future `@givn.modified` delta to reorder.
- **SMELL-2 — Start-hyphen ids rejected by argument parsing, not by
  validation.** A natural invocation `givn new -foo` is intercepted by the
  argument parser (exit code 2, "unexpected argument") before the id
  validator runs. The dedicated "must not start or end with a hyphen"
  message is only reachable by passing the id after `--`. This split
  behaviour is captured as AS-IS; a future `@givn.modified` delta could
  make the two paths consistent.
- **SMELL-3 — Project template overrides require manual directory setup.**
  Initialising a project does not create `givn/artifacts/`, so users who
  want a custom proposal template must create that directory tree by hand.
  Out of scope; documented as AS-IS.
- **No test runner is configured.** `verify.command` is still the sentinel
  `givn missing-testrunner`. This change configures `cargo test` as the
  runner and establishes the characterization harness; it does not add a
  production feature.
- **Generic clap behaviours are out of scope.** Invoking `givn new` with no
  argument (exit 2), with extra positional arguments (exit 2), or with
  `--help` (exit 0) are generic clap behaviours not specific to the `new`
  command's own validation logic. They are intentionally not characterised
  here.
- **Composition of invalid id + uninitialised project is out of scope.**
  `validate_id` runs before the `givn/`-not-found check, so `givn new BadId`
  in an uninitialised project yields the kebab-case error rather than the
  "givn/ not found" error. The two independent scenarios (invalid id;
  uninitialised project) already characterise the individual behaviours, and
  the composition adds no new guarantee; it is out of scope for this change.

## Open Questions

- Should the characterization harness bind Gherkin steps directly to Rust
  (e.g. via the `cucumber` crate), or exercise the CLI as a subprocess with
  plain Rust integration tests? The design chooses subprocess integration
  tests for minimum footprint and zero new runtime dependencies; adopting a
  Gherkin-to-Rust binding is left as a future decision. This does not block
  characterization — `givn lint` checks the `.feature` syntax, `cargo test`
  confirms GREEN, and `givn apply` tracks per-scenario progress via
  `tasks.md` checkboxes.
- Are there filesystem permission or read-only-directory failure modes worth
  characterizing? Not probed here; left as a possible future scenario if it
  surfaces.
