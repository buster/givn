# Proposal: drop-legacy-residue

## Release note

Givn drops its stale residue — the retired history rewrite, the vestigial `extras` flag, the legacy coverage commands, and the pre-0.8 migration bundles and guides — so every documented claim and migration route matches what the tool actually does.

## Use-Case Context

- Use-case IDs: `upgrade-a-project` (migration route and retired-command
  contract), `publish-a-release` (guide prompt-sequence contract),
  `archive-change` (archive coverage gate); `rewrite-history` is retired by
  this change.
- Ideation topic: none. The retired capability's topic record
  (`local-only-integrations`) stays as written; this change does not originate
  from a seed.
- Seed path: none.
- Confirmed Personas: `developer--primary-user` — runs the retired command,
  owns the project history the removal concerns, and crosses the migration
  gap; `agent--technical-role` — reads the retired command's exit code and
  stderr and the upgrade's machine-visible effects. No Persona biography is
  copied here.

## Problem / Opportunity

The product and its front-door documentation still describe capabilities that
were removed releases ago, and the tool still carries the migration and
coverage machinery of unsupported releases:

- The README announces a semantic duplicate review gate, a retrieval build
  feature, `--no-default-features` variants, and Claude/opencode-only skill
  formats; the reference describes an `extras` system with its own install
  command; the workflow document offers an advisory row about a `givn spec
  index` hint no command prints and an archive chokepoint listing a similarity
  gate that no longer exists; the migrations README claims the install
  includes retrieval features; the served guidance assets still inventory
  `extras` as a configuration key and point reviewers and authors at
  `givn/config.yaml` for the runner commands, which live in
  `givn/commands.yaml` — and the archive and directory-resolution scenarios
  repeat that attribution in their Given steps. The reference's configuration
  section and project-layout block make the same attribution. The
  configuration scaffold shows the four project-level addon keys outside the
  `addons:` block, documents arc42 settings the binary does not implement,
  still claims the embedded manifest carries `tasks.post`/`review.post`
  defaults that no longer exist, and carries a placeholder repository URL;
  the command file's header credits an undefined "harness evaluator" instead
  of the commands that actually read it. A reader cannot tell which claims
  still hold.
- The `extras` manifest flag is dead: nothing has read it since the extras
  became individual addons. A project config may still carry `extras: true`
  and nothing says what it does.
- The coverage gate still accepts and reparses the pre-0.8 two-command
  coverage schema and merges its reports itself, duplicating what the
  project-owned merge command now does.
- `givn rewrite-history` still ships: a destructive, explicit history rewrite
  the maintainers no longer want the tool to own. Rule cleanliness is already
  a report: tracked and reachable-history occurrences are named, and nothing
  blocks.
- The migration catalog still declares bundles and guides from 0.2.0 to
  0.7.0 — releases no longer supported. A pre-0.8 project is handed a
  migration plan built from unsupported bundles instead of the managed
  refresh.

The user impact is distrust: the front door promises gates and flags that do
not exist, a consumer configuring `extras` believes it does something, a
pre-0.8 project gets a migration plan whose guides the project cannot rely
on, and the tool keeps a destructive history operation it no longer wants to
own.

## Proposed Solution

- The history rewrite leaves the product. Invoking `givn rewrite-history`
  fails because the command no longer exists; the capability is retired with
  a tombstone, and the retired use case's narrative leaves the corpus. Rule
  cleanliness stays report-only: tracked and reachable-history occurrences
  are reported, `givn check rules` exits non-zero on a violation, and Givn
  never rewrites history.
- The oldest supported migration source is 0.7.0. A 0.7.0 project receives
  the reviewable aggregate migration plan crossing the retained routes
  (0.7.0 → 0.8.0 → 0.9.0); a project whose recorded version has no declared
  route (0.2.0 … 0.6.0) receives only the managed config/guidance refresh and
  no migration change.
- The pre-0.8 bundles, their design assets, their guides, and their index
  entries are gone; the retained bundles and guides stay.
- The changelog gets exactly one edit: the released 0.3.0 entry keeps the
  migration-playbook string (a pinned scenario asserts
  `0.2.0-to-0.3.0` appears in the changelog) and the dead link target that
  pointed at the deleted guide becomes plain text. The release-record
  principle is otherwise untouched.
- A configuration that still carries the legacy coverage commands
  (`non_e2e_command`/`e2e_command`) is refused when it loads, and the error
  names the unknown field: the coverage block is strict, so the removed keys
  cannot silently linger, and the documented replacement is to declare
  `coverage.measure_command`/`merge_command`. A configuration that still
  carries the retired `extras:` key keeps working — the key has no effect and
  the overlay contract tolerates unknown keys, so the removal does not break
  projects for a line Givn itself once wrote.
- Invoking the retired command is pinned by one end-to-end scenario: the real
  CLI exits non-zero and says `rewrite-history` is not an available command.
- Every stale documentation claim is corrected where it lives: the README,
  the reference, the workflow document, the migrations README, the
  documentation index, `AGENTS.md`, the CI prerequisite lists, the served
  guidance assets (the grilling instruction's source inventory uses only
  current keys, and the review, specs, and tasks guidance names
  `givn/commands.yaml` as the runner commands' home), the executable spec's
  Given steps (the archive and directory-resolution scenarios name
  `givn/commands.yaml` where they exercise the e2e runner), the reference's
  configuration section and layout block (each value is listed under the file
  that owns it), the configuration scaffold (the addon keys sit under
  `addons:`, the arc42 section states the fixed `docs/arc42/` path, the
  mandatory-check contract replaces the stale post-hook defaults, and the
  header names the real repository), the command file's header (it names the
  commands that actually read it), and the arc42 chapters that described the
  removed paths.

## Capability Routing

`givn spec route --change drop-legacy-residue` reports signal: it recommends
`EXTEND declare-the-product-start` (`operate-product`, 29.50) and ranks
`apply-a-managed-upgrade` second (25.50). Both match on generic command
vocabulary the proposal necessarily mentions (`givn check`, `givn readme`,
`givn upgrade`, `givn spec`) rather than on this change's behaviour. The
decisions below deviate where they must; this table is the declaration.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `declare-the-product-start` | `EXTEND declare-the-product-start` (29.50, signal) | not extended | The change starts no product, changes no run declaration, and touches no `operate-product` behaviour; the match is generic command vocabulary. |
| `history-rewrite` | — (not ranked) | `RETIRE history-rewrite` | The capability's goal in the `rewrite-history` use case — remove generated rules from the project's reachable history — leaves the product; a `@givn.retired` tombstone records it and the retired use case's narrative is removed with it. |
| `apply-a-managed-upgrade` | `EXTEND apply-a-managed-upgrade` (25.50) | `EXTEND apply-a-managed-upgrade` | Owns migration planning in `upgrade-a-project`, the oldest-route boundary, and the "the system names any command a release removed" contract; the migration fixtures re-point to the oldest supported source (0.7.0), and the retired-command evidence follows the existing retired-engine-command precedent in the same capability. |
| `describe-the-release` | — (not ranked) | `EXTEND describe-the-release` | Owns the guide-as-executable-prompt-sequence rule in `publish-a-release`; the scenario pinned to the deleted 0.2.0-to-0.3.0 guide is re-pointed to the oldest retained guide. |
| `extract-coverage-gate` | — (not ranked) | `EXTEND extract-coverage-gate` | Owns the archive coverage gate in `archive-change`; the two scenarios that exercised the removed built-in legacy merge are removed with the branch, while the project-owned measurement/merge path stays. |
| `enforce-post-merge-e2e-gate` | ranked `enforce-post-merge-e2e-gate` (18.00) | `EXTEND enforce-post-merge-e2e-gate` | Owns the archive-time e2e gate in `archive-change`; three scenarios re-point their Given text to `givn/commands.yaml` with identical observable behaviour. |
| `grouped-merge-for-new-capabilities` | — (not ranked) | `EXTEND grouped-merge-for-new-capabilities` | Owns the grouped-merge routing gate in `archive-change`; three `@e2e` scenarios re-point their Given text with identical observable behaviour. |
| `resolve-change-directories` | ranked `resolve-change-directories` (22.50) | `EXTEND resolve-change-directories` | Owns change-directory resolution for the review gates in `verify-change`; the e2e-resolution scenario re-points its Given text with identical observable behaviour. |
| `customize-the-lifecycle-graph` | — (not ranked) | `NEW in fragments` | The capability lives in the reusable support fragment (`givn/specs/fragments/fragment.md`); fragment deltas record `NEW in fragments` because the routing check resolves only use-case-owned capabilities. Its extras-overlay scenario is removed with the flag. |
| (documentation, served-guidance, and scaffold-text corrections) | — | no capability | The corrected claims are repository text, served-guidance prose, and configuration-scaffold text; the contracts they describe (where the runner commands live, what the grilling instruction inventories, how addons nest) are already pinned by existing scenarios and the manifest schema, and no scenario delta is added, so no new capability is introduced and none is deleted beyond the retirement above. |

## Out of Scope

- The README legacy-scaffold placeholder matcher in `givn readme check` — it
  is current readme-enforcement behaviour, kept.
- The omp agent target and its attribution integration — kept.
- Historical records: `givn/archive/**`, `docs/archive/**`, and the ideation
  topic records (`givn/ideation/**`, `docs/ideation/**`) that name the
  retired command or the removed bundles; they stay as written.
- The retained migration machinery: the catalog loader, route resolution,
  managed preparation, aggregate plan generation, and the 0.7.0/0.8.0
  bundles.
- The repository's own `merge-coverages.sh` union arithmetic — untouched;
  only the tool's built-in legacy merge goes.
- New migration bundles, new addons, or a registry of supported versions.

## Open Questions

### D1 — Retired `extras:` key in a consumer configuration: refuse or ignore?

- Question: When a project's `givn/config.yaml` or `givn/commands.yaml` still
  carries the removed `extras:` key, should loading the configuration fail
  with a named error, or ignore the key?
- Options:
  1. **Refuse with a named error** — every command that loads the
     configuration fails until the line is deleted. Gain: the dead key cannot
     linger unnoticed. Cost: a breaking change for projects Givn itself once
     wrote the key into; the failure also blocks `givn upgrade`, so the
     repair is a manual edit. In practice: an old project carrying
     `extras: true` fails with `failed to parse givn/config.yaml: unknown
     field 'extras'` on every command, including the upgrade that would
     otherwise refresh it.
  2. **Ignore the key** — the overlay keeps accepting unknown keys; the key
     has no effect. Gain: no consumer breaks; the removal changes no
     observable behaviour because nothing read the flag anyway. Cost: a dead
     line can remain in a configuration indefinitely. In practice: the same
     project runs unchanged, and the next `givn upgrade` preserves the line
     as a user-owned value.
- Disposition: answered — option 2. The overlay contract is lenient by design
  (only the `coverage:` block is strict), the flag has had no reader since the
  extras became addons, and a rejection would hard-fail every command for a
  key Givn itself once wrote. Realized in `design.md`.

### D2 — Retired legacy coverage keys: what does a project still carrying them observe?

- Question: After `coverage.non_e2e_command`/`coverage.e2e_command` are
  removed, what happens when a project still declares them?
- Options:
  1. **Refuse on load with the unknown-field error** — automatic because the
     coverage block is strict. Gain: the migration window closes visibly and
     the removal cannot be mistaken for a working configuration. Cost: every
     command fails until the keys are replaced, and `givn upgrade` cannot
     rewrite them because the strict parse refuses the file first. In
     practice: `failed to parse givn/config.yaml: coverage: unknown field
     'non_e2e_command'`.
  2. **Keep accepting the keys silently** — keep the legacy fields as inert.
     Gain: no consumer breaks. Cost: the legacy path is not actually removed,
     the residue stays, and the migration window never closes.
- Disposition: answered — option 1. Removing the fields closes the
  unsupported pre-0.8 coverage route; the error names the unknown field and
  the documented repair is to declare `coverage.measure_command` and
  `coverage.merge_command`. Realized in `design.md`; the consequence is
  recorded as a risk in chapter 11.

### D3 — Evidence for the retired command: a scenario or the tombstone alone?

- Question: Does the retirement get a user-visible scenario asserting the
  command is gone, and which capability owns it?
- Options:
  1. **One `@e2e` scenario under `apply-a-managed-upgrade`** — the capability
     already owns "the system names any command a release removed" and holds
     the retired `givn spec index` precedent. Gain: the refusal is pinned at
     the real CLI boundary with exit code and stderr. Cost: one more `@e2e`
     scenario and one interaction row. In practice: a later change that
     accidentally re-adds the subcommand fails the suite.
  2. **The tombstone alone** — gain: leanest. Cost: no executable evidence
     that the command actually disappears; only the retirement record remains.
- Disposition: answered — option 1. Realized in the `apply-a-managed-upgrade`
  delta and `design.md`.

### D4 — Oldest supported migration source

- Question: After the pre-0.8 bundles are deleted, what is the oldest
  supported migration source, and what do older projects receive?
- Options:
  1. **0.7.0** — the oldest retained bundle and guide. Gain: the supported
     boundary is the oldest retained route; 0.2.0 … 0.6.0 projects receive
     the managed refresh only, never a plan built from unsupported bundles.
     Cost: pre-0.8 projects must cross with manual work or their own copies of
     the deleted guides. In practice: a clean 0.7.0 project receives the
     reviewable aggregate plan; a 0.2.0 project receives the managed refresh
     and no migration change.
  2. **Keep the pre-0.8 chain** — delete only the guides. Gain: maximum
     continuity. Cost: keeps exactly the bundles the user chose to remove.
- Disposition: answered — option 1 (the user's reviewed scope). Realized in
  the upgrade scenarios and in the catalog test in `design.md`.

### D5 — The retired use case's narrative

- Question: After `history-rewrite` retires, what happens to
  `givn/specs/rewrite-history/usecase.md`, whose only capability it was?
- Options:
  1. **Delete it with the retirement** — the directory disappears and the
     `.retired.yaml` tombstone is the durable record. Gain: no use case
     remains whose goal the product no longer serves. Cost: the use case's
     prose history lives only in `givn/archive/` and the tombstone. In
     practice: `givn/specs/rewrite-history/` is gone after archive.
  2. **Keep the narrative as historical** — gain: the prose stays in place.
     Cost: the corpus keeps a leaf use case with zero capabilities, and every
     inventory surface must tolerate the orphan.
- Disposition: answered — option 1: the tombstone is the durable record, the
  goal leaves the product with its only capability, and the archived corpus
  shows no surviving directory for a capability retired out of its own
  directory. Realized in `design.md` and the task list.

### D6 — Re-point or remove the pinned guide scenario

- Question: "The migration guide is an executable prompt sequence" is pinned
  to the deleted 0.2.0-to-0.3.0 guide. Re-point it or remove it?
- Options:
  1. **Re-point it to the oldest retained guide** (`docs/migrations/
     0.7.0-to-0.8.0.md`). The scenario title carries no version, so it stays
     and its steps become the retained guide's prompt contract. Gain: the
     release use case's rule stays executable on a guide that ships. Cost:
     the assertions change to what the retained guide actually says. In
     practice: ordered prompts, inspect-before-editing, and corpus repairs
     kept out of the maintenance change.
  2. **Remove the scenario** — gain: no scenario churn. Cost: the
     `publish-a-release` rule "the migration guide is an executable prompt
     sequence that forbids automatic changes to domain specifications and
     architecture decisions" becomes unverified.
- Disposition: answered — option 1. Realized in the `describe-the-release`
  delta and its steps.
