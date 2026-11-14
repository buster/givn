# Use Case Definitions: ise-rebuildability

Cut from `eventstorming.md` (stable event IDs). Existing permanent use cases
keep their IDs and contracts; this topic defines their delta plus the new
leaves and the modified fragment. Levels: `!` user goal (change-sized), `+`
summary, fragment = reusable behavior with no independent user goal. Every
event with a Givn owner has exactly one; "Consumes" lists inclusion or
observation, not ownership.

All questions are closed (`Q1`-`Q16`). The cut reads these settled inputs:

- `ise` is an `Opt-in addon` that requires `architecture_principles`; enabling
  `ise` refuses by name until the prerequisite is on, and disabling the
  prerequisite refuses while `ise` is on (`Q6`).
- `givn/specification.yaml` declares five retained roles (`behavior`,
  `structure`, `contracts`, `limits`, `e2e`) as path globs and one `source:`
  list of replaceable roots; it declares no component (`Q4`).
- The declared `e2e` role is one package holding the runner and every `@e2e`
  step definition; it links no production source and fails on an unbound or
  skipped step; feature files stay shared (`Q5`).
- `givn ise prepare [--at <revision>] [--replace <path>...] --into <dir>
  [--receipt <path>] [--dry-run]` requires a target outside the checkout,
  defaults `--at` to `HEAD`, materializes the clean room, and writes the trial
  record when a receipt path is given; `givn ise verify --trial <receipt>
  --at <dir>` re-checks the retained and `e2e` digests; both are bound to
  `addons.ise` (`Q3`).
- Only the `End-to-end (E2E) test` package is frozen as acceptance authority;
  component step definitions, fakes, and harness are implementation; the
  removed component's paths are trial input; the trial report lines are
  `Replaced`, `Not replaced`, and `Outside the declaration` (2026-11-14
  constraints 1-4).
- `Change strategy` and `Spec gap` are recorded decisions (`Q16`, `Q14`).

## Summary table

| ID | Level | Status | Actors / personas | Goal (delta in italics) | Owns events | Consumes |
|---|---|---|---|---|---|---|
| `initialize-project` | ! | existing, modified | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Set up a Givn project and its agent integrations. *`ise` registers off by default, refuses enablement until `architecture_principles` is on, refuses disablement of the prerequisite while `ise` is on, and enabling it scaffolds the declaration.* | A1-A3, A5, A6 | A4, `declare-the-specification` (scaffold shape) |
| `upgrade-a-project` | ! | existing, modified | Project maintainer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Cross a supported release gap with reviewable managed revisions, preserving user-owned content and accurately reporting recovery from a committed checkpoint. *The upgrade adds a missing declaration scaffold and never rewrites a project-edited declaration.* | none (scaffold refresh, not a chain event) | `declare-the-specification` |
| `create-change` | ! | existing, modified | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role) | Create a change ready to carry from intent to executable specification. *The `@e2e` policy names the declared package boundary; the design record carries `Change strategy`; a corrective change carries the `Spec gap` outcome.* | A4, B4, D5, D6 | B1-B3, D4 |
| `obtain-effective-guidance` | ! | existing, modified | Authoring agent, Supporting: Developer (agent--technical-role, developer--primary-user, givn-maintainer--owner) | Obtain the rules governing the work at hand, extended with the project's extension. *The `ise` obligations are served only when `ise` is effective; core step guidance names each stack instruction id and its load condition and carries no stack body.* | B1-B3 | - |
| `verify-change` | ! | existing, modified | Developer, Supporting: Reviewer (developer--primary-user, givn-maintainer--owner) | Check a change for structural validity, applicable verification evidence, and completed review without treating advisory structural matches as proof or blockers. *The `@e2e` package boundary and its unbound or skipped step refusal are checked, the E2E identity is bound into the verification evidence, and the four declaration findings are reported as advisory.* | C2-C4 | C1, `declare-the-specification` |
| `declare-the-specification` | ! | new | Developer, Supporting: Authoring agent (developer--primary-user, agent--technical-role, givn-maintainer--owner) | Declare the boundary between the Specification and the Source code. | none (the declaration is a project specification, not a chain event) | - |
| `prepare-an-ise-test` | ! | new | Developer, Supporting: Independent verifier (developer--primary-user, givn-maintainer--owner) | Produce a clean-room trial whose frozen inputs a verifier can confirm. | D1 | `declare-the-specification`, D3, D4 |
| `author-change` | + | existing, unchanged | Developer, Authoring agent (developer--primary-user, agent--technical-role) | Set up a Givn project and create changes with the guidance to carry them from intent to executable specification. *No direct delta; the delta lives in its children `initialize-project` and `create-change`.* | - | - |
| `maintain-project` | + | existing, unchanged | Project maintainer (developer--primary-user, givn-maintainer--owner) | Keep a project's documentation and managed files current across releases. *No direct delta; the delta lives in `upgrade-a-project`, one of its three children (`keep-documentation-current`, `upgrade-a-project`, `publish-a-release`).* | - | - |
| `archive-change` | ! | existing, unchanged | Developer (developer--primary-user, agent--technical-role) | Archive a completed change with proven publication. *No delta: it consumes the E2E-bound receipt, and the existing freshness check refuses a receipt whose relevant inputs changed.* | C5 | C4 |
| `support` | fragment | existing, modified | included by the lifecycle use cases modified here; the two new leaves include no fragment | No independent goal. *Capability `instruction-integrity` modified: the design ledger requires the `Change strategy` entry; review guidance requires one `Spec gap` outcome before any fix and the neighboring-components question; a named stack instruction id that does not resolve is a named defect.* | none (guidance content) | B1-B3 |

Events without a single Givn owner: C1 (a durable contract was authored) is
realized by the change's authoring flow; `declare-the-specification` owns where
those contracts live, but it does not author their content. D2 (the replacement
was built) and D3 (the frozen E2E tests ran) are the rebuilding agent's and the
independent verifier's actions outside Givn; `prepare-an-ise-test` consumes D3
and D4 and records only D1 and the trial. D4 (a difference was observed) is the
verifier's observation. A4 (addon state was locked) belongs to `create-change`
through `givn new`.

## Sizing evidence for the new leaves

| Rule | `declare-the-specification` | `prepare-an-ise-test` |
|---|---|---|
| One goal, one primary actor | One verb phrase: declare the boundary. The developer is the primary actor; the authoring agent reads the declaration, and the Givn maintainer owns its schema. | One verb phrase: produce a trial. The developer is the primary actor; the independent verifier runs the frozen tests and the re-check. |
| Goal-shaped, never component-shaped | The goal names the declaration, not the file, the loader, or the schema. | The goal names the trial, not the command, the directory, or the digest. |
| Level discipline | `!` with one direct capability; no `+` is introduced. | `!` with two direct capabilities; no `+` is introduced. |
| Mutation containment | Changing: the declaration is written and validated. | Changing: the trial directory and its record are written. `trial-input-verification` observes the written trial; an observing capability under a changing goal is allowed, and no mutating capability sits under an observing goal. |
| Extension discipline | Every refusal branches from the validation step before the write and protects the minimal guarantee (the previous declaration stands unchanged). | Every refusal branches from the input or write step and protects the minimal guarantee (the checkout is unchanged and nothing is written). |
| Preconditions and flow | The scaffolded file exists before the flow; the flow writes its content and never creates the file (that is init or upgrade). | The clean checkout, the declaration, the selected backend, and the revision hold before the trigger; the flow checks them and refuses, never establishes them. |
| Implementation independence | Goal, trigger, and guarantees name the boundary and the roles, never a loader, schema language, or glob engine. | Goal, trigger, and guarantees name the trial and its frozen inputs, never a command flag, digest algorithm, or directory layout. |
| Relationship semantics | No `Includes`; no new fragment. | No `Includes`; no new fragment. It consumes the declaration and the verifier's evidence. |

## `initialize-project` (existing, modified)

- Goal unchanged; setup and addon configuration gain the `ise` posture.
- Capability status:
  - `enable-optional-addons` modified. `ise` joins the registry with
    `default_enabled: false`; a fresh `givn init` writes the nine explicit
    `true` keys and omits `ise:`; `givn addons list` reports `ise` disabled;
    `givn addons enable ise` refuses by name until `architecture_principles`
    is on, and `givn addons disable architecture_principles` refuses while
    `ise` is on, naming `ise` first; `--operator --reason` keeps its existing
    meaning during an active change; `TC-14`, the `Addon` and `Addon default`
    glossary rows (`docs/glossary.md:8,9`), the all-defaults rule
    (`givn/specs/initialize-project/usecase.md:57`),
    `assets/config.yaml.tmpl:135,137`, and the three all-default scenarios
    (`enable-optional-addons.feature:109,119,133`) are amended. Enabling `ise`
    scaffolds `givn/specification.yaml` from its documented shape when the file
    is absent, preserves an existing declaration, and reports the scaffold it
    wrote.
  - `initialize-a-project` unchanged. No fresh-init path makes `ise` effective:
    the registry default is `false`, no flag enables a default-off addon
    (`--no-addons` only disables every addon), and a fresh init scaffolds only
    addons whose declared default is `true` (`src/cli/init.rs:150-155`), so it
    writes no declaration. The declaration arrives through
    `givn addons enable ise` (`src/cli/addons.rs:103-115,132-176`) or a forced
    re-init that restores an explicit `ise: true` choice
    (`src/cli/init.rs:59-62,138-149`). The scaffold's content is the documented
    shape, not a project declaration.
- Boundary: setup and local installation. The declaration's content is
  `declare-the-specification`; the lock write is `create-change`.
- Stakeholders: developer--primary-user (an ordinary project stays unchanged
  until it opts in), agent--technical-role (the served guidance and the
  scaffold agree), givn-maintainer--owner (one default-posture rule and no
  hidden second write).

## `upgrade-a-project` (existing, modified)

- Goal unchanged; the upgrade becomes declaration-aware.
- Capability status: `apply-a-managed-upgrade` modified. The staged set is
  `givn/config.yaml`, `givn/commands.yaml`, the ignore files, and
  `managed_generated_paths` (`src/cli/migration.rs:620-645`), which enumerates
  generated skills, agent integrations, `AGENTS.md`, and the commit hook
  (`src/skills/targets.rs:575-619`); it contains no project scaffold, and
  `apply_managed_upgrade` writes only the config and the commands backfill
  (`src/cli/migration.rs:567-608`). The change therefore
  extends both: the upgrade writes the declaration when `ise` is effective and
  the file is absent, and the managed revision stages it. An existing
  declaration is project-owned and preserved byte-for-byte, because its roles
  and `source:` roots are project decisions; the upgrade never rewrites them.
- Boundary: crossing a release gap with reviewable revisions. Scaffolding on
  setup is `initialize-project`; the managed upgrade adds the scaffold when
  absent; the declaration's content is `declare-the-specification`.
- Stakeholders: developer--primary-user (an edited declaration survives every
  upgrade), givn-maintainer--owner (the revision stays reviewable and
  allowlist-only).

## `create-change` (existing, modified)

- Goal unchanged; the change's authoring records and guidance carry the `ise`
  obligations.
- Capability status:
  - `prose-consolidation` modified. The canonical `@e2e` policy states that
    the declared `e2e` package holds the runner and every `@e2e` step
    definition, links no production source, and fails on an unbound or skipped
    step; feature files stay shared. The two `assets/instructions/specs.md`
    sentences (`:135-140`, `:164-169`), the `prose-consolidation` scenarios at
    `:124-135`, and the `create-change` rules change together (`Q5`
    consequences).
  - `change-strategy` new. The design's `Decides D<n>` entry records one of
    `incremental modification`, `Constant Ise`, or a whole-product rebuild,
    with the `Ise Test` mode (`Dry-Run Ise` or `Full replacement`) beside it
    when the change rebuilds. The entry names the affected consumers, the
    persisted state, and the rebuild boundary in prose; no new mandatory
    design field is added. Prose keeps `change strategy` qualified because
    "strategy" is generic.
  - `spec-gap` new. A `Spec gap` record holds exactly one of `E2E test defect`,
    `implementation defect`, `specification defect`, or `acceptable
    undocumented change` before any fix. A corrective proposal names the
    record and its outcome: an `implementation defect` returns to the builder
    inside the Ise Test; a `specification defect` or `E2E test defect` opens a
    corrective change that edits the corpus or the E2E suite; an `acceptable
    undocumented change` is recorded in the corpus. The guidance carries the
    limitation that only `specification defect` is a gap in the Specification.
- Boundary: creating a change and the records its authoring carries. The trial
  that observes a difference is `prepare-an-ise-test`; the `@e2e` boundary gate
  is `verify-change`.
- Stakeholders: developer--primary-user (a proportionate strategy and one
  classification before any fix), agent--technical-role (deterministic record
  shapes and guidance), givn-maintainer--owner (no new mandatory design field).

## `obtain-effective-guidance` (existing, modified)

- Goal unchanged; the instruction set gains the `ise` instruction and the
  stack-instruction split.
- Capability status: `serve-extended-rules` modified.
  - `givn instructions ise` serves the obligations only when `ise` is
    effective, which requires `architecture_principles`: Specification
    completeness, the two-layer test boundary, persisted-state continuity, the
    `Change strategy` choice, and the four-way `Spec gap` classification.
  - Core step instructions (`specs`, `steps`, `design`, `review`) carry the
    universal rules and name each stack instruction id plus the condition for
    loading it; the stack instruction (`steps-rust`, or the existing
    `dialog-examples` precedent) is served on request by id and is never
    included in the core instruction's served text.
  - The text and JSON paths carry the same content; an unknown id still
    refuses with the known list.
- Boundary: serving the effective content for one instruction id. Instruction
  resolution and reference integrity stay with the `instruction-integrity`
  fragment; the content assets stay with the canonical instruction files.
- Stakeholders: agent--technical-role (loads only the mechanics the change
  needs), developer--primary-user (manual reading), givn-maintainer--owner
  (instruction budget).

## `verify-change` (existing, modified)

- Goal unchanged; the check consumes the declaration and enforces the `@e2e`
  package boundary.
- Capability status:
  - `givn-workflow-enforcement` modified. With `ise` effective,
    `givn check tasks` requires the declared `e2e` package to exist, to link
    no production source, and to be the target `verify.e2e_command` runs; an
    unbound or skipped step fails. The change verification receipt binds the
    `e2e` package digests, the E2E runner command digest, and the running Givn
    version, so a weakened black-box suite changes the receipt.
  - `check-specification-health` modified. `givn lint` reports the four
    project-level declaration findings: a declared path is missing; a path is
    declared in two roles; a `.feature` file sits outside the `behavior` globs;
    the declared `e2e` package is missing or has a dependency edge into
    `source`. The findings are advisory: they do not change the lint exit
    code, unlike `givn check review`, which fails on every finding.
- Boundary: checking a change. It does not prepare the trial
  (`prepare-an-ise-test`), declare the roles (`declare-the-specification`), or
  publish (`archive-change`).
- Stakeholders: developer--primary-user (deterministic refusals before a
  rebuild), agent--technical-role (machine-readable findings), givn-maintainer--owner
  (the receipt binds what ran).

## `declare-the-specification` (new)

- Level: `!`.
- Primary actor: Developer. Supporting: Authoring agent (reads the declaration
  before authoring).
- Goal: declare the boundary between the Specification and the Source code.
- Trigger: the developer changes a declared role or a replaceable source root,
  or a durable contract moves to another path.
- Preconditions: the project is initialized; `ise` is effective; the
  declaration file exists from setup or upgrade; the project is the developer's
  to configure.
- Main flow:
  1. The developer maps each retained role to its paths and lists the
     replaceable source roots; the system reads the proposed declaration.
  2. The system validates the proposal and refuses an unknown key or role, a
     duplicate role, and a path outside the project.
  3. The system writes the declaration without exposing a partial role set and
     makes the resolved roles and roots available to the flows that read them:
     the boundary check and the trial preparation.
- Extensions:
  - 2: an unknown key or role → the refusal names it and writes nothing;
    protects the minimal guarantee.
  - 2: a role is declared twice → the refusal names the role and writes
    nothing; protects the minimal guarantee.
  - 2: a path falls outside the project → the refusal names the path and
    writes nothing; protects the minimal guarantee.
- Rules:
  - Exactly five retained roles: `behavior`, `structure`, `contracts`,
    `limits`, `e2e`.
  - Exactly one `source:` list of replaceable roots, each bounded to the
    project.
  - A declared path missing from the tree is not a write refusal;
    `verify-change` reports it as an advisory finding.
  - No `components:` key and no per-component file list: the replaced
    component's paths are trial input, passed to `prepare-an-ise-test`, not a
    permanent project declaration.
  - A project-edited declaration is project-owned; setup and upgrade never
    rewrite its content.
- Minimal guarantee: a refused declaration leaves the previous file
  byte-for-byte unchanged and exposes no partial role set.
- Success guarantee: the declaration names exactly the five retained roles and
  one `source:` list of replaceable roots, every path bounded to the project,
  and the resolved roles and roots are available to the flows that read them.
- Capabilities: `specification-declaration`.
- Interactions: none yet; Use Case Elaboration supplies them.
- Includes: none.
- Extends: none.
- Boundary: the project's declaration of its retained Specification and
  replaceable Source code; the retained contracts belong to the change's
  authoring flow, and the trial belongs to `prepare-an-ise-test`.
- Out of scope: authoring the contracts the roles retain (the change's
  authoring flow); materializing a trial (`prepare-an-ise-test`); reporting
  declaration findings (`verify-change`); scaffolding the file
  (`initialize-project`, `upgrade-a-project`).
- Diagram: none yet.
- Stakeholders: developer--primary-user (the declaration states the rebuild
  boundary), agent--technical-role (a strict, discoverable input), and
  givn-maintainer--owner (one schema and no component list).

## `prepare-an-ise-test` (new)

- Level: `!`.
- Primary actor: Developer (the consumer-project owner; the Givn maintainer
  when Givn rebuilds itself). Supporting: Independent verifier (runs the frozen
  E2E tests and the re-check).
- Goal: produce a clean-room trial whose frozen inputs a verifier can confirm.
- Trigger: the developer starts an Ise Test.
- Preconditions: `ise` is effective; the declaration exists; the checkout is
  clean; one backend is selected; a revision is available (defaulting to
  `HEAD`); the component's paths fall inside the declared source roots.
- Main flow:
  1. The developer selects the revision, or accepts `HEAD`, and the component's
     paths; the system reads the declaration and the tracked files at the
     revision.
  2. The system partitions the tracked paths into retained, kept source,
     replaced paths, and files outside the declaration.
  3. The system refuses by name before any write on a missing declaration, an
     ineffective `ise`, a dirty checkout, a missing, mixed, or nested backend,
     a missing retained path, a replaced path outside every source root, a
     replaced path matched by a retained role, a target inside the checkout or
     already existing, and a repository link or nested repository in the copy
     set.
  4. The system writes the retained files and kept source into a plain
     directory outside the checkout, preserving relative paths, the executable
     bit, and symlink targets, with no version-control metadata and no ignored
     or untracked files.
  5. The system re-reads each written file, compares it to the revision blob,
     records a tamper-evident identity for the retained, `e2e`, and
     replaced-paths sets, reports the counts, and prints the trial record with
     the revision, the paths named with `--replace`, the counts, the recorded
     identities, the backend, and the Givn version; a receipt path writes that
     record to the named file.
  6. The independent verifier runs the project's frozen E2E target against the
     replacement; the system re-checks the retained and `e2e` identities on
     request and reports drift per path.
- Extensions:
  - 3: any stated refusal → the command exits non-zero, names the condition,
    and writes nothing; protects the minimal guarantee.
  - 5: a written file differs from its revision blob → preparation fails and
    the trial record is not written; protects the success guarantee.
  - 6: a recorded identity no longer matches → verification fails and names
    the drifting paths; protects the success guarantee.
- Rules:
  - The report lines are `Backend`, `Revision`, `Retained`, `Kept source`,
    `Replaced`, `Outside the declaration`, `Not replaced`, `Retained digest`,
    `E2E digest`, `Written path`, and, when a receipt path was given,
    `Receipt path`.
  - A trial is a plain directory outside the checkout, never a branch or a
    worktree, because a branch carries history and a Fossil checkout carries
    the repository database.
  - The checkout is never written; the replaced component's Source code and
    history are absent from the trial.
  - Only the `End-to-end (E2E) test` package is frozen; component step
    definitions, fakes, and harness are not frozen and are not diff-counted.
  - An existing scenario is frozen only after it was observed failing against
    the pre-change implementation for the intended reason.
- Minimal guarantee: a refusal leaves the checkout byte-for-byte unchanged and
  writes no trial directory or receipt.
- Success guarantee: a trial directory outside the checkout holds the declared
  Specification and kept source with the component's paths removed, and, when
  a receipt path was given, the trial record's retained and `e2e` evidence
  re-checks against it.
- Capabilities: `clean-room-preparation`, `trial-input-verification`.
- Interactions: none yet; Use Case Elaboration supplies them.
- Includes: none.
- Extends: none.
- Boundary: the clean-room trial directory and its record; the checkout, the
  declaration, and the corrective change stay outside it.
- Out of scope: declaring the roles (`declare-the-specification`); building
  the replacement (the rebuilding agent); running the frozen E2E tests (the
  independent verifier); classifying a difference (`create-change`,
  `spec-gap`); opening the corrective change (`create-change`).
- Diagram: none yet.
- Stakeholders: developer--primary-user (a trial that is cheap to prepare and
  honest about what it holds), givn-maintainer--owner (a bare-clone verifier
  runs with Givn absent from PATH and complete inputs), agent--technical-role
  (deterministic refusals and a machine-readable record).

## `archive-change` (existing, unchanged)

- No contract delta. The change verification receipt it proves now carries the
  E2E identity; archive's existing freshness check refuses a receipt whose
  relevant inputs changed. The frozen E2E test files stay byte-identical in the
  published corpus.
- Listed to confirm the interaction is examined, not to change it.

## `author-change` and `maintain-project` (summaries, unchanged)

- Both are `+` summaries whose children carry this topic's delta;
  `author-change` covers `initialize-project` and `create-change`,
  `maintain-project` covers `keep-documentation-current`, `upgrade-a-project`,
  and `publish-a-release`. Their coverage rules and contracts are untouched.

## Other existing use cases (unchanged)

`inspect-change-state`, `inspect-development-history`, `discover-specifications`,
`keep-documentation-current`, `publish-a-release`, `assess-code-structure`,
`operate-product`, `extend-an-instruction`, and `make-usecases-canonical` were
examined and keep their contracts. The outline parsers under `src/outline/` are
not repurposed for the `@e2e` boundary; the boundary is the package's
dependency graph and the runner's unbound or skipped step failure (`Q5`).

## Fragment `support`: capability `instruction-integrity` (existing, modified)

- No independent user goal. The `support` fragment
  (`givn/specs/fragments/fragment.md:1`) owns 21 capabilities, including
  `instruction-integrity`
  (`givn/specs/fragments/instruction-integrity.feature`). The lifecycle use
  cases modified here include it; `assess-code-structure`,
  `extend-an-instruction`, `inspect-development-history`, and
  `make-usecases-canonical` declare no Includes, and the two new leaves include
  no fragment.
- Capability: `instruction-integrity` modified.
- Behavior to elaborate:
  - The design and design-review guidance require the `Change strategy` entry,
    its three values, and the `Ise Test` mode beside it when the change
    rebuilds; an unresolved or unrecorded decision stays a design-review
    finding.
  - The review guidance requires exactly one `Spec gap` outcome before any fix
    and asks the neighboring-components question ("name the neighboring
    components changed outside the replaced component and say why").
  - Each core step instruction names its stack instruction ids and their load
    conditions; a named id that does not resolve is a named defect, not a
    silent omission.
  - The `ise` instruction text carries the obligations the `ise` addon
    promises; it is served only when `ise` is effective.
- Boundary: instruction content, reference integrity, and the design ledger's
  shape. Serving is `obtain-effective-guidance`; the record shapes are
  `create-change`; the declaration schema is `declare-the-specification`.
- Stakeholders: agent--technical-role (guidance it can satisfy), givn-maintainer--owner
  (one canonical owner per rule).

## Rejected candidates

| Candidate | Level proposed | Why rejected |
|---|---|---|
| `enable-the-ise-addon` | ! | Enabling an addon is `initialize-project`/`enable-optional-addons`; a separate leaf would split one configuration goal across two use cases. |
| `serve-the-stack-instruction` | ! | Serving by id is `obtain-effective-guidance`/`serve-extended-rules`; the split is served content plus the existing id convention (`Q15`), not a new actor goal. |
| `check-the-e2e-boundary` | ! | The actor goal, checking a change, is `verify-change`; the boundary is a capability (`givn-workflow-enforcement`), not a use case. |
| `report-the-declaration-warnings` | ! | The four findings are advisory output of `givn lint`, owned by `verify-change`; no actor goal is "be warned". |
| `verify-an-ise-test` as a separate leaf | ! | `givn ise verify` re-checks the trial `givn ise prepare` wrote and has no success outcome independent of it; it is the `trial-input-verification` capability. An independent verifier's separate goal, running the frozen E2E tests, is an external action, not a Givn use case. |
| `classify-a-spec-gap` | ! | The classification is a change-level decision the corrective proposal carries; `create-change`/`spec-gap` owns its shape, and a new leaf would split the authoring flow. |
| `record-the-change-strategy` | ! | The design ledger already records decisions; `create-change`/`change-strategy` owns the entry shape and `instruction-integrity` owns the guidance. |
| `scaffold-the-specification-declaration` | ! | A step inside `initialize-project` and `upgrade-a-project`; the scaffold has no independent goal. |
| `run-the-frozen-e2e-tests` | ! | The independent verifier runs the project's target outside Givn; `prepare-an-ise-test` consumes the evidence and re-checks the frozen inputs. |
| `promote-ise-to-default` | ! | Out of scope: `ise` stays an `Opt-in addon` until empirical Catopia and Passuff evidence justifies promotion (Frame exclusion). |
| `full-rebuild` or `Strangler Ise` as use cases | ! | Values of `Change strategy` and future patterns, not Givn goals in this topic. |
| A `specification-declaration` fragment | fragment | The declaration's loader is a single shared read of one file by three use cases; the consumers list it in `Consumes`, and a fragment would duplicate the leaf's capability without adding reusable behavior. |
