# 12. Glossary

| Term | Definition |
|---|---|
| **Addon** | Embedded behavior toggled through `addons.<name>`. The embedded manifest starts disabled, but current `givn init` enables all known addons unless `--no-addons` is supplied. |
| **Addon-aware generation** | Generation that includes only the template sections whose declared addon condition matches the current effective addon set; an addon that is off leaves no trace in the generated skill or slash command. |
| **AGENTS.md ownership** | Givn owns only the content between its own named fenced markers. All other `AGENTS.md` content is user-owned and preserved. |
| **Agent target** | One generated guidance destination: `opencode`, `agents`, or `claude`, represented by `.opencode/`, `.agents/`, or `.claude/`. |
| **Attribution context** | The runtime facts available while one commit is created: LLM, reasoning mode, harness, and Givn version. Missing facts remain unknown rather than inferred. |
| **Model identity** | The provider and model pair selected by the harness for the current attribution context. Anti-terms: model name, model label. |
| **Commit attribution** | The best-effort LLM and harness identification visible through a revision's committer identity and message trailers; on Git through the hook, on Fossil through `givn commit`. |
| **Harness** | The agent runtime that executes project commands, such as OpenCode or Claude Code. Anti-term: agent target. |
| **Reasoning mode** | The harness-reported effort or thinking variant active when a commit is created. |
| **Attribution trailer** | The managed `Co-Authored-By` message trailer containing the commit-attribution context; on Git it is a commit message, on Fossil a check-in comment. |
| **Managed co-author attribution** | The Givn-owned `Co-Authored-By` trailer formatted as `<model>/<reasoning>@<harness> with Givn <version>`; ordinary human co-author trailers remain user-owned. |
| **Attribution status** | The message trailer value `complete` or `partial`, indicating whether the active attribution context supplied every expected field. |
| **Committer identity** | The Git name and email stored as the commit creator; an agent integration may provide the name while Git supplies the configured email fallback. Fossil records its own user and has no equivalent override. |
| **Commit-attribution hook** | The checked-in Git `commit-msg` hook that appends best-effort attribution trailers without blocking a commit. Anti-term: post-artifact hook. |
| **Hook activation launcher** | The machine-local Git hook that preserves an existing `commit-msg` hook and delegates to the checked-in commit-attribution hook. |
| **Installation report** | The `givn init` or `givn upgrade` output that names installed or refreshed shared plumbing and selected agent integrations. |
| **Attribution retrofit** | Refreshing commit-attribution files in an existing project during `givn upgrade`, including the files needed to share the behavior. |
| **arc42-docs** | The effective lifecycle artifact added when the Arc42 addon is enabled. Its marker is `arc42.md`; durable chapters are scaffolded at fixed `docs/arc42/`. |
| **Artifact** | A lifecycle document or generated path described by the effective manifest. It may have `requires`, `optional`, and configurable `post` hooks. |
| **Capability** | A named behavior unit, represented by one `.feature` file under a use case. A capability is not itself a use case; it is owned by one. |
| **Cascade** | Project-overrides-first lookup for an individual template, instruction, skill, or command asset, with embedded content as fallback. |
| **Conditional section** | A marked region of a template retained only when its declared addon condition matches the current effective addon set; it is dropped from the generated file otherwise. |
| **Change** | A unit of work under `givn/changes/<id>/` that, once archived, moves to `givn/archive/<id>/`. |
| **Archive candidate** | A change whose accepted material has been merged into the permanent corpus at its archive location and published as a revision, but which has not yet passed archive verification. Anti-terms: pending change, merged change. |
| **Proven archive** | An archive candidate whose full suite and coverage passed for its tested revision, recorded by a verification receipt. Only a proven archive is complete and safe to publish. Anti-term: finished archive. |
| **Legacy archive** | An archive directory produced before the receipt contract and therefore without a verification receipt. It is historical and is never re-published. Anti-term: unproven archive. |
| **Verification receipt** | The durable machine-readable record binding a proven archive to the revision that was tested, the per-scope scenario counts, the coverage outputs, and the timing. Anti-terms: verification manifest, evidence cache. |
| **Verification scope** | One runner execution's declared boundary: the regular scenarios or the end-to-end scenarios. Each scope reports its own scenario counts. |
| **Runner result** | The machine-readable scenario counts a verification scope's runner writes for Givn; a missing or non-reconciling result blocks proof. Anti-term: test summary. |
| **Forward repair** | Repairing a failed archive in place and rerunning the archive against the new published state; the failed candidate is kept, never silently discarded. Anti-terms: rollback, restore. |
| **Delta** | An active Gherkin feature marked `@givn.delta` that describes Added, Modified, or Removed scenarios. |
| **Disposition** | A review entry for a deterministic overlap finding involving a change delta. |
| **Fingerprint** | A deterministic masked assertion sequence plus best-effort facts: exit code, `contains` polarity, table values, `[DONE]`, and TTY mention. |
| **Generated skill** | A skill file written from a skill template into an agent target directory when `givn skills sync` runs. |
| **Net-delta receipt** | The count of Added, Modified, and Removed scenarios printed by the overlap review gate. |
| **Eject** | Materialization of an embedded default into a project-owned `givn/` override. |
| **Facet** | An independently resolvable asset unit such as a template or instruction. |
| **Feature file** | A Gherkin `.feature` file. In givn it is the executable behavior specification and, after archive, the permanent record. |
| **Manifest** | YAML data describing artifacts, dependencies, archive requirements, verify settings, coverage settings, and addons. |
| **Manifest overlay** | Project YAML applied to the embedded manifest. `givn/config.yaml` is applied first; `givn/commands.yaml` is applied later. |
| **Permanent spec** | A tag-stripped `.feature` under `givn/specs/<usecase-id>/`, used as the durable behavior source. |
| **Use case** | A domain goal or summary of domain goals represented by `usecase.md`, with actors, guarantees, rules, capabilities, interactions, and typed relationships. Anti-terms: group, cluster, folder, bucket. |
| **`«include»` relationship** | A directed dependency from a use case to another use case or typed fragment whose behavior it always needs; recorded under `usecase.md`'s `## Includes`. Anti-terms: depends on, uses (prose). |
| **`«extend»` relationship** | A conditional or error-path behavior contribution from one use case to a base use case; recorded under `usecase.md`'s `## Extends`. Extensions cannot target fragments. Anti-term: optional feature. |
| **Fragment** | Reusable behavior with no independent user goal, represented under `givn/specs/fragments/` and included by use cases. Fragments are not use cases. |
| **Hook (`post`)** | A configurable post-artifact command. It may name `verify`, `verify-e2e`, `lint`, `integrity`, or a shell command. Custom shell hooks may allow failure; built-in names cannot weaken mandatory semantics. |
| **Mandatory check** | Binary-owned behavior run by explicit `givn check tasks` or `givn check review`, independent of an artifact's `post` list. |
| **Template** | A source asset (embedded default or project override) rendered into a generated skill or slash command. Skill templates carry the `.tmpl` marker; conditional sections are resolved against the current addon configuration at generation time. |
| **Test runner** | The project command in `verify.command` that executes feature scenarios. In this checkout it is supplied by `givn/commands.yaml`. |
| **`verify.command`** | The effective project test-runner command. The embedded fallback is `givn missing-testrunner`; initialization writes `./run-tests.sh` in the project command template. |
| **`verify.e2e_command`** | The command used for E2E scenarios. The current project command file uses `./run-tests.sh --e2e`. |
| **Lint (`givn lint`)** | Static feature parsing and overlap checking. It runs no project tests; exit precedence is parse `1`, duplicate title `3`, WIP `2`, clean `0`. |
| **Coverage gate** | Optional archive-level validation of project-owned measurement and merge commands and Cobertura output. It is operational/test governance, not an active ADR. |
| **Managed block** | A README region Givn owns and rewrites wholesale between its own fenced markers (coverage badges, coverage summary, development commands). User content outside managed blocks is never touched. |
| **Open placeholder** | A machine-recognizable request-for-content left in a README section by scaffolding (`givn:todo` marker text). A README without open placeholders is complete. Anti-term: "todo comment". |
| **README check** | `givn readme check` — deterministic report of every open placeholder by section plus damage to the development managed block; exits non-zero while any remains. |
| **README completeness gate** | Archive-level blocking validation that the README contains no open placeholders and its managed blocks are intact, enabled by the readme addon. It mirrors the coverage gate. |
| **README impact decision** | The machine-checkable review line recording that the README needed no update (`none`) or was updated, naming the sections. Required for review sign-off when the readme addon is enabled. Anti-term: "docs impact". |
| **README migration** | One-time agent-executed rewrite of a legacy-scaffolded or hand-written README into the marked form while preserving all real content; accepted only when the README check passes. Anti-term: "rescaffold". |
| **Permanent duplicate audit** | `givn spec duplicates`, a complete advisory scan of current `givn/specs/` excluding active changes and archive contents. |
| **Exact duplicate** | Two permanent scenarios with the same canonical behavior body, including relevant backgrounds, steps, tables, docstrings, and examples, independent of title/path. |
| **Structural overlap** | Equal masked assertion shape or a strict assertion-subset relation with Fingerprint facts. Given/When action structure is not part of the relation. |
| **Scenario identity** | A project-relative forward-slash feature file plus scenario title. |
| **Project-relative path identity** | The normalized project-relative forward-slash path form used when comparing permanent and delta scenarios. |
| **Arc42 implementation conformance** | Human evidence review of durable Arc42 chapters and change assessment against design, tasks, and implementation, supplemented by affected-file integrity checks. |
| **ADR** | Architecture Decision Record. Active records live in `adr/`; historical records live in `adr/archive/`. |
| **Current architecture contract** | Only an ADR with `status: accepted` and no supersession relationship. Proposed and archived records do not bind the current system. |
| **ADR candidate** | A proposed choice that is evaluated for architectural significance before an ADR file is created. |
| **Qualification verdict** | The structured authoring result containing the five mandatory gate outcomes, supporting `must_be_shared` evidence, route, and route-specific target. It is not a runtime result. |
| **Mandatory qualification dimension** | One of `alternatives`, `architectural_impact`, `durable_consequence`, `lower_level_artifact`, or `existing_adr_check`; all five must pass. |
| **Canonical artifact** | The one authoritative location for a piece of information: Gherkin spec, design/code, Arc42 chapter, process guidance, or project documentation. |
| **Supporting-only evidence** | Evidence that informs a qualification decision but cannot make it pass; `Must be shared` has this role. |
| **ADR amendment** | A change to the rationale or scope of an existing decision that does not introduce an independent boundary; routed as `AMEND_ADR`. |
| **ADR supersession** | A qualified replacement of an existing decision that keeps the old record as explicit architectural history; routed as `SUPERSEDE_ADR`. |
| **ADR routing** | The single outcome `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, or `CANONICAL_ARTIFACT` selected after qualification and duplicate checks. |
| **Single-source architecture documentation** | The rule that current facts, behavior, implementation, process, and decision rationale each have one authoritative home rather than redundant copies. |
| **Slash command** | A command file written from a command template into `.opencode/commands/` when `givn skills sync` runs. |
| **Managed upgrade path** | The machine-safe portion of a version migration: config scaffold and generated Givn integrations refreshed under an explicit SCM allowlist. |
| **Maintenance change** | A visible Givn change for project-wide tooling or migration work that may intentionally skip a product feature specification. |
| **Migration plan** | The proposal and detailed design generated for an upgrade so an LLM can derive project-specific tasks without fabricated completion evidence. |
| **Upgrade revision** | The first automatic revision of a managed upgrade, containing only the managed scaffold and generated integration changes. Former name: Upgrade Commit A. |
| **Plan revision** | The second automatic revision of a migration-bearing upgrade, containing the maintenance plan and the recorded identifier of the upgrade revision. Former name: Migration Commit B. |
| **Migration catalog** | Embedded release policy at `assets/migrations/catalog.yaml` that declares source/target bundle edges and their order. |
| **Migration bundle** | A release-owned Proposal/Design asset pair describing one project migration phase. |
| **Migration evidence** | The before/after snapshots, human-readable ledger, and comparison receipts proving that a semantic corpus migration preserved behavior, E2E evidence, interaction coverage, and source coverage. Anti-term: rename proof. |
| **Migration coverage receipt** | The machine-readable comparison written by `givn check migration`, including behavior, E2E, interaction, line, and branch outcomes. Anti-term: test summary. |
| **Aggregate migration change** | One visible maintenance change containing all bundle phases required to cross a version gap, with ordered metadata in `migration.yaml`. |
| **Managed upgrade** | A migration-free upgrade that synchronizes Givn-owned files and creates one allowlist-only commit when needed. |
| **Run declaration** | The author-owned `run:` block in `givn/commands.yaml`: product type, start command, exactly one server port source, readiness path, and readiness timeout. One replace-whole overlay unit, like `verify` and `coverage`. Anti-terms: "start config", "launch config". |
| **Product type** | The declared kind of the built product: `server`, `cli`, or `library`. Drives which run-declaration fields apply and how the README block renders. Anti-term: "product kind". |
| **Start command** | The declared command that starts the built product. Anti-terms: "entrypoint", "launch command". |
| **Sentinel start command** | `givn missing-run-command` — the scaffold placeholder meaning "not configured yet"; a hidden subcommand that prints guidance and exits 1 when executed. Anti-term: "default command". |
| **Run finding** | One labeled deterministic validation result for the run declaration: `UNCF`, `MISS`, `FORB`, or `NOOP`, from rules R1–R10. Anti-term: "warning". |
| **Run gate** | The blocking check of the run declaration in `givn check review` and `givn archive`, independent of the readme addon. Anti-terms: "start gate", "run-block gate". |
| **Run managed block** | The README region between `<!-- givn:begin:run -->` and `<!-- givn:end:run -->` rendered from the run declaration. Anti-terms: "quick start section", "run todo". |
| **Port hand-off** | Declaring `port_env` so the caller passes the port to the product through that environment variable; the declared alternative is a fixed `port`. Anti-terms: "port injection", "port mapping". |
| **Ephemeral port** | A kernel-assigned free port obtained by binding `127.0.0.1:0`, reading the port, and dropping the listener before spawn. Anti-term: "dynamic port". |
| **Readiness path** | The declared HTTP path on the started server that answers 2xx once it is ready to serve. Anti-terms: "health endpoint", "health URL". |
| **Readiness probe** | givn's blocking HTTP/1.0 poll against the readiness path: 2xx accepts, everything else retries until `ready_timeout_s`. Anti-terms: "health check", "ping". |
| **Readiness timeout** | The declared `ready_timeout_s` seconds the readiness probe may take before giving up; elapsed without a 2xx response, `givn run` exits 2. Anti-term: "grace period". |
| **Ready line** | The one stdout contract line `givn run: ready at http://127.0.0.1:<port><ready_path>`, emitted once after the first 2xx response, with a stable greppable prefix. Anti-terms: "banner", "status output". |
| **Start plan** | The resolved, printable form of the run declaration produced by `givn run --print`: one sh-eval-able line (`PORT=<port> <command>` with a port hand-off, else `<command>`). Anti-term: "run config". |
| **Backfill scaffold** | The managed-upgrade append of the run-declaration scaffold into an existing `givn/commands.yaml` when the `run:` key is absent; idempotent. Anti-term: "migration". |
| **Executor** | The givn subsystem behind `givn run` that resolves the start plan, spawns the product, probes readiness, forwards exit codes, and forwards signals on unix. Anti-terms: "supervisor", "daemon". |
| **Ideation** | The facilitation skill that works a vague idea into change-ready use cases through phases, stakeholder personas, and rubric reviews. Anti-terms: brainstorm, discovery, workshop. |
| **Ideation topic** | One ideation effort whose state persists under `givn/ideation/<topic-id>/` across sessions until handoff. Anti-term: ideation session (a topic spans sessions). |
| **Frame** | The ideation phase that inventories existing project knowledge or establishes problem, vision, and stakeholders for a new domain. |
| **Event Storming** | The ideation phase that models the domain as a chain of past-tense domain events with actors, commands, and policies. Anti-term: Storm (rejected as ambiguous). |
| **Use Case Definition** | The ideation phase that defines the use cases from the event storming protocol: name, goal level, boundary, and affected stakeholder types. Anti-terms: slicing, cutting. |
| **Use Case Elaboration** | The ideation phase that elaborates each defined use case into acceptance rules, concrete examples, and the use-case contract. Anti-terms: mapping, specification (reserved for the feature file). |
| **Persona** | A documented embodiment of one persona segment, kept as a living record of decisions, needs, and open demands, that reviews every artifact affecting it. Anti-terms: user profile, role, actor. |
| **Stakeholder type** | A class of interested parties identified during Frame; one type may carry several personas. Anti-term: stakeholder group. |
| **Persona segment** | One distinct incarnation of a stakeholder type with interests of its own (avid reader vs. collector). Anti-terms: variant, flavor. |
| **Domain terms** | The per-topic candidate file `givn/ideation/<topic-id>/domain-terms.md` holding domain terms as they crystallize; candidates are merged into the durable term list at handoff by user decision. Anti-term: glossary candidates (the always-on guidance may not name the addon-owned artifact). |
| **Persona promotion** | The user-approved copy of a validated persona into the project-level persona collection at handoff; repeated promotion of the same segment merges. Anti-term: export. |
| **Change seed** | The per-use-case proposal seed written into the topic state at handoff; the user starts the normal proposal flow from it, handoff never starts a change. Anti-terms: stub, pre-proposal. |
| **Quality review** | A fresh-context, rubric-based advisory review at an ideation phase boundary. Anti-term: gate (binary-owned checks keep that word). |
| **Open question** | A recorded unknown parked in the topic state instead of being debated. Anti-term: blocker. |
| **Handoff** | The ideation step that publishes the topic document under the project docs, proposes glossary and persona merges, and writes change seeds. |
| **Visual interface** | A consumer interface whose evidence is what the consumer sees (web UI) or reads (terminal); API-only and library interfaces are not visual. |
| **Web UI classification** | The fail-closed rule that a capability whose interface renders HTML, CSS, or JavaScript in a browser is a Web UI — even without controls or server state; a browser driver is then mandatory and an HTTP client is not a valid driver. Anti-term: static document. |
| **Visual Design Contract** | The style-level contract a design records when a change introduces or visibly changes a screen: tokens, component states, viewports, and the durable design system it follows. Anti-terms: mockup, pixel spec. |
| **Design system** | The durable project style source under `docs/design/` that visual contracts reference and improve. Anti-term: style guide (reserved for prose advice). |
| **Visual evidence** | The committed final-state screenshots per end-to-end scenario and viewport — or captured transcripts for a terminal interface — that review inspects. Anti-term: screenshot dump. |
| **UI state evidence** | The committed `<viewport>--<state>.png` screenshots for each UI state a contract declares (empty, loading, error), in addition to the final-state screenshot. Anti-term: state dump. |
| **Visual baseline** | The mechanical checks asserted at every visual interaction: labeled controls, no internal identifiers in visible text, a real page title and heading, no duplicated visible content, a designed error state, the configured language, and reduced-motion. Anti-term: UI checklist. |
| **Visual verdict** | The review's explicit machine-checkable outcome for a visual change: `VISUAL-REVIEW: PASS` or `VISUAL-REVIEW: UNVERIFIED`. Anti-term: visual review note. |
| **Persona disposition** | The review's record, as a `PERSONAS:` line, of the confirmed Persona that reviewed the rendered evidence; a declared visual interface requires at least one. Anti-term: persona note. |
| **Deferral decision** | The operator's recorded approval (`OPERATOR-DECISION:`) that allows a mandatory check to be deferred when a project constraint conflicts with it; a `DEFERRAL:` without it fails review. Anti-term: waiver. |
| **Style tile** | One rendered, concrete design option proposed to the user when no approved design exists. Anti-term: mockup (rejected as pixel-precise). |
| **SCM backend** | The concrete version-control system givn talks to in a project: Git or Fossil. Selected per project, never per command. Anti-terms: provider, driver. |
| **Revision** | One immutable recorded state of the project in the active SCM backend: a Git commit or a Fossil check-in. |
| **Checkout** | The working directory givn operates in, as seen by the active SCM backend: a Git worktree or a Fossil checkout. |
| **Clean state** | The state in which the active backend reports no changes outside the resolved evidence paths; Git untracked files and Fossil `EXTRA` files both count as dirty. |
| **Candidate revision** | The revision that publishes the merged archive before verification; recorded unbound in the pending receipt. |
| **Tested revision** | The revision against which every verification scope passed; recorded in the proven receipt. |
| **Revision binding** | The receipt property that the tested revision is the published candidate revision. |
| **Marker** | The project-root entry that identifies a version-control system: a `.git` entry (file or directory) for Git, or `.fslckout`/`_FOSSIL_` for Fossil. |
| **Named refusal** | A command's failure that names the reason, the project root, and the missing prerequisite, exits non-zero, and mutates nothing. Anti-term: generic error. |
| **Version-controlled project** | A project whose root carries a recognized backend marker and whose executable is available; a project without one is refused by name. |
| **Witness-bounded publication** | The publication contract: resolve the publication paths, validate the exact dirty set against them, and record only that set. Git stages the paths; Fossil records all changes and commits the witnessed dirty set. |
| **Unpublished candidate** | A pending candidate whose publication a backend hook rejected: the receipt exists on disk but the current revision does not contain it, so a rerun publishes the on-disk merged state again. |

Fossil-specific clauses in this glossary (Fossil `EXTRA` dirty semantics,
attribution through `givn commit`, the Fossil backend itself) describe the
agreed target contract. The archive runs on Fossil; the managed upgrade is
still refused by name, and D-20 in chapter 11 tracks that remainder.

### Visual-review invariants

- A change with a visual contract cannot pass review without a visual
  verdict (enforcement site: `check review`).
- Every end-to-end scenario has one committed final-state screenshot per
  contract viewport; missing evidence blocks review (enforcement site:
  `check review` evidence completeness).
- Every declared UI state has committed per-viewport evidence (enforcement
  site: `check review` state-evidence check).
- A declared visual interface cannot pass review without a `PERSONAS:`
  disposition naming a confirmed Persona (enforcement site: `check review`
  persona check).
- A recorded deferral cannot pass review without an operator decision
  (enforcement site: `check review` deferral pair).
- A browser-rendered capability is classified as a Web UI and driven by a
  browser, never by an HTTP client (enforcement site: review and
  design-review guidance plus the fabrication audit).
- An unverified verdict never passes without human sign-off (enforcement
  site: `check review` marker validation).
- A style improvement that stays within the contract updates the contract;
  it is never a review failure (enforcement site: review guidance and
  fabrication audit).

### Run-declaration invariants

- A change cannot be reviewed as complete or archived while the run
  declaration has findings (enforcement site: `check review` mandatory check
  and the archive run gate).
- The sentinel start command is acceptable only for `product_type: library`
  (enforcement site: validation rule R1).
- A server declares exactly one port source — fixed `port` or `port_env`
  hand-off, never both, never neither (enforcement site: validation rules
  R4/R5; executor preflight for `--port`).
- givn never derives the product type or start command from repository
  content (enforcement site: product boundary — no derivation code exists).

### Archive-proof invariants

- The permanent corpus is written only by a proven archive; an unproven
  candidate is never reported as proven (enforcement site: receipt status and
  the finalization guard in `cli::archive`).
- A verification scope result must declare its scope and reconcile
  `passed + failed + skipped == total`, with at least one scenario and zero
  failures or skips (enforcement site: `proof::RunnerResult::read`).
- Finalization changes only the tested revision's declared evidence paths
  (enforcement site: `proof::is_evidence_path`).
- A resumed candidate does not repeat the merge (enforcement site: pending
  receipt mode in `cli::archive`).
