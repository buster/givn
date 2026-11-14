# Persona: Primary user (developer / maintainer)

## Stable core

- Role: The human who uses givn in their own projects. Two merged segments —
  the developer who builds features and the maintainer who runs upgrades and
  crosses release gaps — share the authority boundary of owning the project
  and its repository and the vocabulary of day-to-day version control.
- Authority boundary: Owns the project, its repository, and its history; may
  decide what to archive, upgrade, and migrate; is never forced to adopt
  tooling or vocabulary beyond the active SCM.
- Evaluation capability: Judges whether the workflow fits the project, whether
  an upgrade preserved their work, and whether the history stays readable;
  does not read givn's internals.
- Own vocabulary: "my project", "commit"/"check in", "upgrade", "migration",
  "clean", "history", "timeline"; Git users say "branch", "commit",
  "worktree"; Fossil users say "wiki", "tickets", "checkout". The
  commit/check-in collision is their normal speech; givn must not force
  either word.
- Success criterion: A Git project's behavior is unchanged, and a Fossil
  project completes the same lifecycle with the same guarantees, using the
  same commands.

## Running record

- 2026-09-14 — Decision: givn natively supports Git and Fossil; a Git project
  uses Git, otherwise Fossil. (User)
- 2026-09-14 — Need: a named refusal when neither SCM is present. (Frame)
- 2026-09-14 — Demand: attribution should work in Fossil projects if
  feasible. (User; answered by `givn commit`.)
- 2026-09-14 — Need: no Git-only claim in generated guidance for a Fossil
  project. (Frame)
- 2026-11-13 — Need: `ise` stays opt-in, so an ordinary feature change pays
  no new ceremony. (Frame, vision)
- 2026-11-13 — Need: a replacement keeps persisted user data and state across
  versions, not just compilable code. (Frame, Ise consequence "Data, state, and
  contracts outlive the code")
- 2026-11-13 — Need: `@e2e` suites strong enough to rebuild a module or the
  whole product without reading the old source. (Frame, gaps G4/G7)

## Topic stances

- 2026-09-14 — multi-scm-support: one mental model across both SCMs; the
  difference is handled inside givn, not pushed onto the user.
- 2026-09-14 — kiekste hosting: out of scope; the hosting and work-item
  features are kiekste's domain and never a givn specification obligation.
- 2026-09-23 — local-only-integrations: publishes projects created with Givn
  as open source while Givn stays closed during development; keeps every
  Givn rule local; owns the `AGENTS.md` tracking decision and merges its
  fence by hand; owns their own non-Givn rule files.
- 2026-09-23 — local-only-integrations: expects one Givn command on a fresh
  clone or second machine to restore the local workflow.
- 2026-09-23 — local-only-integrations (reviewer interest): untracking and
  ignoring must be explicit and reviewable, never a silent mutation.
- 2026-10-13 — rules-through-instructions: demands extensions in
  project-owned configuration, added to the guidance, without owning whole
  files.
- 2026-10-13 — rules-through-instructions: demands that `givn skills sync`
  and `givn upgrade` never conflict with extensions.
- 2026-10-13 — rules-through-instructions: expects the workflow of a fresh
  clone or second machine to stay recoverable as under local-only.
- 2026-11-13 — ise-rebuildability: a replacement that loses user data, or
  needs a manual state migration step the specification never named, is not a
  successful replacement.
- 2026-11-13 — ise-rebuildability: a clean-room rebuild and a single-unit
  replacement must be runnable from the same commands on Git and Fossil; the
  SCM difference stays inside givn. (open: Q3)
- 2026-11-13 — ise-rebuildability: wants the ordinary feature workflow
  unchanged until `givn addons enable ise`, and no gate that fires on projects
  that never opted in.
- 2026-11-14 — ise-rebuildability: `Q3` closed with `givn ise prepare` and
  `givn ise verify`, bound to `addons.ise`; the Git and Fossil commands are
  the same two commands.
