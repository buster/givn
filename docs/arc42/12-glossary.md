# 12. Glossary

| Term | Definition |
|---|---|
| **Addon** | Embedded behavior toggled through `addons.<name>`. The embedded manifest starts disabled, but current `givn init` enables every default-on registered addon unless `--no-addons` is supplied; a default-off addon such as `outline` requires `givn addons enable <name>`. |
| **Addon-aware generation** | Generation that includes only the template sections whose declared addon condition matches the current effective addon set; an addon that is off leaves no trace in the generated skill or slash command. |
| **AGENTS.md ownership** | Givn owns only the content between its own named fenced markers. All other `AGENTS.md` content is user-owned and preserved. |
| **Agent target** | One generated guidance destination: `opencode`, `agents`, `claude`, or `omp`, represented by `.opencode/`, `.agents/`, `.claude/`, or `.omp/`. |
| **Attribution context** | The runtime facts available while one commit is created: LLM, reasoning mode, harness, and Givn version. Missing facts remain unknown rather than inferred. |
| **Model identity** | The provider and model pair selected by the harness for the current attribution context. Anti-terms: model name, model label. |
| **Commit attribution** | The best-effort LLM and harness identification visible through a revision's committer identity and message trailers; on Git through the hook, on Fossil through `givn commit`. |
| **Harness** | The agent runtime that executes project commands, such as OpenCode, Claude Code, or omp. Anti-term: agent target. |
| **Reasoning mode** | The harness-reported effort or thinking variant active when a commit is created. |
| **Issuing session** | The agent-runtime session whose shell request created a command; the command is attributed to that session's remembered model, not to the last model observed anywhere in the run. |
| **Attribution trailer** | The managed `Co-Authored-By` message trailer containing the commit-attribution context; on Git it is a commit message, on Fossil a check-in comment. |
| **Managed co-author attribution** | The Givn-owned `Co-Authored-By` trailer formatted as `<model>/<reasoning>@<harness> with Givn <version>`; ordinary human co-author trailers remain user-owned. |
| **Committer identity** | The Git name and email stored as the commit creator; an agent integration may provide the name while Git supplies the configured email fallback. Fossil records its own user and has no equivalent override. |
| **Commit-attribution hook** | The checked-in Git `commit-msg` hook that appends best-effort attribution trailers without blocking a commit. Anti-term: post-artifact hook. |
| **Hook activation launcher** | The machine-local Git hook that preserves an existing `commit-msg` hook and delegates to the checked-in commit-attribution hook. |
| **Installation report** | The `givn init` or `givn upgrade` output that names installed or refreshed shared plumbing and selected agent integrations. |
| **Attribution retrofit** | Refreshing commit-attribution files in an existing project during `givn upgrade`, including the files needed to share the behavior. |
| **arc42-docs** | The effective lifecycle artifact added when the Arc42 addon is enabled. Its marker is `arc42.md`; durable chapters are scaffolded at fixed `docs/arc42/`. |
| **Artifact** | A lifecycle document or generated path described by the effective manifest. It may have `requires`, `optional`, and configurable `post` hooks. |
| **Capability** | A named behavior unit, represented by one `.feature` file under a use case. A capability is not itself a use case; it is owned by one. |
| **Cascade** | Project-overrides-first lookup for an individual template, instruction, skill, or command asset, with embedded content as fallback. |
| **Dialog policy** | The effective canonical instruction (`dialog`) governing user-facing questions and decision records; entry points reference it instead of copying the full rules. The pending `make-user-questions-decision-ready` contract makes that obligation explicit across startup, all bundled available guidance, and fresh delegated contexts, with proportional treatment for project-familiar readers. It is authoring guidance, not runtime interception. |
| **Question record** | The full entry retained for an actual decision: title, status, what it blocks, context, the standalone question, each genuine option's meaning, concrete example, gain and cost, and a resolution naming the chosen and rejected alternatives with reasons. A short information request explains its use and immediate effect without inventing alternatives or tradeoffs. |
| **Conditional section** | A marked region of a template retained only when its declared addon condition matches the current effective addon set; it is dropped from the generated file otherwise. |
| **Change** | A unit of work under `givn/changes/<id>/` that, once archived, moves to `givn/archive/<id>/`. |
| **Archive candidate** | A change that has moved to its archive location and is being made ready for proof: its accepted material is merged from that location into the permanent corpus and published as a revision, but it has not yet passed archive verification. The moved directory is an archive candidate from the transition onward, including the interval before its delta merge completes (`merge_completed: false`). Anti-terms: pending change, merged change. |
| **Interrupted archive transition** | An archive attempt that stopped after the change left its active location but before the archive was proven. It leaves either the active change or a pending archive candidate whose receipt records whether the delta merge completed; both copies exist only while a legacy or externally restored state is being reconciled, and only a candidate whose active artifacts match byte-identically is recoverable. Anti-terms: partial archive, half-archive. |
| **Merge progress** | The pending receipt's record of merge recovery: `merge_completed` says whether every delta file finished, `merged_deltas` lists the completed relative delta paths, and `merge_in_progress` names the one file that may have been written before its completion was recorded. A resume skips completed files and retries only the in-progress file with the idempotent rule. Anti-terms: merge marker, merge flag. |
| **Proven archive** | An archive candidate whose full suite and coverage passed for its tested revision, recorded by a verification receipt. Only a proven archive is complete and safe to publish. Anti-term: finished archive. |
| **Legacy archive** | An archive directory produced before the receipt contract and therefore without a verification receipt. It is historical and is never re-published. Anti-term: unproven archive. |
| **Verification receipt** | The durable machine-readable record binding a proven archive to the revision that was tested, the per-scope scenario counts, the coverage outputs, and the timing. Anti-terms: verification manifest, evidence cache. |
| **Change verification receipt** | The durable record `givn check tasks` writes for one change: the tested revision, the selected delta-feature path and digest, the verify-command digest, the tested-content digest with the `change-proof/v1` scope, the green runner counts, and the finish time. It is fresh while the tested content, features, and command match: `Current` means the tested revision also equals the current revision; `DocumentationOnly` means the tested content is unchanged under a different available revision. Anti-terms: review proof, test cache. |
| **Documentation-only revision** | A committed revision accepted by the receipt as documentation-only: the tested-content digest is unchanged and the tested revision is available (Git ancestry from `HEAD`; Fossil revision existence). In the ordinary case its changes stay inside the closed documentation/evidence allowlist — the active change's top-level Markdown artifacts, its evidence/verification/results paths, its use-case and fragment narratives, `givn/personas/**`, `givn/archive/**`, `givn/history/**`, generated semantic caches, `docs/arc42/**`, and `README.md` — but the check proves content equality, not the intervening diff. It is not a synonym for any Markdown change: instruction assets, configuration, features, source, and tests are tested content. Anti-term: docs commit. |
| **Tested content** | Every tracked or untracked non-ignored path outside the documentation/evidence allowlist, hashed with its executable marker and file bytes into the change verification receipt's tested-content digest; gitlinks use their recorded object id and symlinks their target bytes. A change to any tested content makes the receipt stale. |
| **Runner result** | The four integer counts a project runner writes to `GIVN_RESULT_FILE`: total, passed, failed, skipped. `scope` is optional; Givn labels the invocation. Anti-term: test summary. |
| **Verification scope** | One runner execution's declared boundary: the regular scenarios or the end-to-end scenarios. Each scope reports its own scenario counts. |
| **Runner result** | The machine-readable scenario counts a verification scope's runner writes for Givn; a missing or non-reconciling result blocks proof. Anti-term: test summary. |
| **Forward repair** | Repairing a failed archive in place and rerunning the archive against the new published state; the failed candidate is kept, never silently discarded. Anti-terms: rollback, restore. |
| **Delta** | An active Gherkin feature marked `@givn.delta` that describes Added, Modified, or Removed scenarios. |
| **Release note** | The one complete, user-facing sentence a change's proposal carries naming what the change does; every revision the change records shares it as the message subject, with the scenario title in the message body. A changelog line derived from revision subjects is a consequence, not the release note. Anti-terms: change summary, changelog entry. |
| **Changelog** | The durable `CHANGELOG.md` whose release sections list each release's user-visible changes, one entry per change, taken from the change's release note. Anti-terms: release history, commit log. |
| **Release section** | One `## [x.y.z] - date` block of the changelog, generated from the revisions since the last version tag and carrying only user-visible groups; a range without user-visible change has no release section and stops the release. Anti-terms: release block, release notes file. |
| **Release base tag** | The annotated version tag that matches the version the package manifest declares current; the generated release range starts there. Anti-term: last tag. |
| **Release revision** | The repository revision the release preparation records, carrying the version bump and the release section; the annotated version tag points at it. Anti-term: version commit. |
| **Disposition** | A review entry for a deterministic overlap finding involving a change delta. |
| **Root diagnostic** | A review finding about the identity or Level of a use-case document: the title line is not exactly `# Use case: <kebab-case-id>`, or `## Level` is not exactly `!` or `+`. A root diagnostic makes the document unusable for routing and is reported before any dependent finding. Anti-term: primary error. |
| **Dependent diagnostic** | A review finding whose truth depends on a valid root: a missing required heading, an empty required section, the leaf-use-case capability requirement, or a capability-routing complaint that needs a readable use-case document. These findings are withheld while the title or Level is invalid and are re-derived once the root is corrected. Anti-term: cascading error. |
| **Title line** | The first `# Use case: <kebab-case-id>` heading (or the corresponding fragment heading) that supplies the document identity. A case-mismatched line whose remainder is already a valid identifier is recoverable; a human-readable remainder is a root diagnostic and is never accepted as the identifier. |
| **Section schema** | The fixed set of required `##` headings for a use-case document, and the recognised headings for a fragment. Missing required headings are dependent diagnostics; genuinely unknown headings are independent findings and are still reported. |
| **Fingerprint** | A deterministic masked assertion sequence plus best-effort facts: exit code, `contains` polarity, table values, `[DONE]`, and TTY mention. |
| **Consumer project** | A project that uses Givn to manage its own lifecycle, as opposed to the Givn repository that builds the tool. Generated guidance and printed instructions are written for it. Anti-term: downstream project. |
| **Generated guidance** | The skills, slash commands, and instruction text Givn writes into or prints for a consumer project's authoring agents. It is the rendered output, not the templates and canonical assets in the Givn repository. |
| **Generated skill** | A skill file written from a skill template into an agent target directory when `givn skills sync` runs. |
| **Extended rule** | The rule a project's agents follow for one instruction: Givn's built-in, technology-agnostic rules plus the project's extension. Contrast: not a Givn rule (the generated file, or the built-in rule content alone); the extended rule is what an agent follows once the project's extension is added. |
| **Givn rule** | A file Givn generates into a project: a skill, slash command, plugin, hook, or attribution plumbing. Preferred noun for what earlier text called generated Givn integrations. Different sense: the built-in rule content an instruction carries is not a Givn rule file; see Extended rule. |
| **Rules policy** | The `rules:` scalar in `givn/config.yaml`: `local-only` (default; rules stay out of the repository) or `versioned` (rules are repository content). An absent key resolves to `local-only` for new and existing projects; an unrecognized value warns and continues as `local-only`. |
| **Strict rule** | Every Givn rule except the `AGENTS.md` fence: the set covered by the local-only policy, both ignore destinations, and the clean-history guarantee. The shared `.claude/settings.json` is user-owned and stays untouched; the Claude hook entry lives in `.claude/settings.local.json`, which is a strict rule. |
| **Local-only rule** | A Givn rule that exists only on the developer's machine: never tracked, never staged. |
| **Versioned rule** | A Givn rule that is repository content, staged and committed like source. |
| **Local-only transition** | Making a tracked strict rule local-only: the file stays on disk and the path leaves the current revision (Git `git rm --cached`; Fossil `fossil rm --soft` or `forget`). |
| **Ignore block** | Givn's owned entry region in an ignore destination. Two forms exist: the fenced `# givn:begin:rules` region inside the tracked ignore file (`.gitignore`, `.fossil-settings/ignore-glob`), whose entries are project content and whose surrounding user content is never touched; and the development record's block, which lives either in a backend-local, never-tracked destination (`.git/info/exclude` for a Git worktree, or the repository-level `ignore-glob` setting for a Fossil checkout that carries no managed file) or, when a Fossil checkout already has `.fossil-settings/ignore-glob`, as a deduplicated entry (`givn/history/*`) inside that file's managed entry list. Surrounding user content is never touched in either form. Anti-terms: ignore file (a file is only one of the destinations), fence markers on the Fossil record entries (that list carries none). |
| **Publishable project** | A project whose repository and reachable history carry no strict rule, and whose build, run, and test paths work with no Givn installed. |
| **Clean history** | No reachable revision contains a strict rule. Occurrences are reported and never block; Givn itself never rewrites history. Unreachable objects, remotes, and forks are outside the guarantee. |
| **Net-delta receipt** | The count of Added, Modified, and Removed scenarios printed by the overlap review gate. |
| **Eject** | Materialization of an embedded default into a project-owned `givn/` override. |
| **Facet** | An independently resolvable asset unit such as a template or instruction. |
| **Feature file** | A Gherkin `.feature` file. In givn it is the executable behavior specification and, after archive, the permanent record. |
| **Manifest** | YAML data describing artifacts, dependencies, archive requirements, verify settings, coverage settings, and addons. |
| **Manifest overlay** | Project YAML applied to the embedded manifest. `givn/config.yaml` is applied first; `givn/commands.yaml` is applied later. |
| **Permanent spec** | A tag-stripped `.feature` under `givn/specs/<usecase-id>/`, used as the durable behavior source. |
| **Use case** | A contract between one primary actor and the system: the actor's single goal, stated as observable outcome, with the success and minimal guarantees the system makes while pursuing it. Represented by `usecase.md`, with actors, guarantees, rules, capabilities, interactions, and typed relationships. Anti-terms: group, cluster, folder, bucket. |
| **Primary actor** | The actor whose single goal a use case contracts; named first in `## Actors`, with additional actors marked `Supporting:`. Anti-term: main actor. |
| **Summary use case** | A `+` use case that owns zero direct capabilities and only child leaves via `Includes`; its coverage is the union of its children. A leaf whose flow names three or more distinct actor goals must be promoted to it and split. |
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
| **Finding severity** | The `advisory`, `finding`, or `blocking` classification of a lint finding from one mapping table; it is descriptive and the existing exit-code contract still drives the exit code. |
| **Route confidence** | The explicit `signal` or `no signal` verdict of `givn spec route`; no signal is reported on a top-two tie or below-threshold overlap, and the ranked list is then omitted. |
| **Falsification tests** | Three fail-fast checks applied after the five qualification dimensions: single-change reversibility, no multi-change constraint, and the product-behavior exclusion. They classify by kind, not count. |
| **Open-question qualifier** | The `(open: Q<N>)` marker on a use-case rule or guarantee whose prerequisite is unresolved; it stays until the question closes and is removed in the same edit. |
| **Decision ledger** | The `Decides D<n>: <decision> — <where it is realized>` record in a design that maps every proposal question ID to its resolution or a named owner. |
| **Cheaper-home challenge** | The design-review requirement that every proposed `NEW_ADR` names the specific canonical artifact that would own the rationale instead; a missing challenge is a finding. |
| **Qualification verdict** | The structured authoring result containing the five mandatory gate outcomes, supporting `must_be_shared` evidence, route, and route-specific target. It is not a runtime result. |
| **Mandatory qualification dimension** | One of `alternatives`, `architectural_impact`, `durable_consequence`, `lower_level_artifact`, or `existing_adr_check`; all five must pass. |
| **Canonical artifact** | The one authoritative location for a piece of information: Gherkin spec, design/code, Arc42 chapter, process guidance, or project documentation. |
| **Supporting-only evidence** | Evidence that informs a qualification decision but cannot make it pass; `Must be shared` has this role. |
| **ADR amendment** | A change to the rationale or scope of an existing decision that does not introduce an independent boundary; routed as `AMEND_ADR`. |
| **ADR supersession** | A qualified replacement of an existing decision that keeps the old record as explicit architectural history; routed as `SUPERSEDE_ADR`. |
| **ADR routing** | The single outcome `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, or `CANONICAL_ARTIFACT` selected after qualification and duplicate checks. |
| **Single-source architecture documentation** | The rule that current facts, behavior, implementation, process, and decision rationale each have one authoritative home rather than redundant copies. |
| **Slash command** | A command file written from a command template into the `commands/` directory of a command-owning agent target (`.opencode/commands/` or `.omp/commands/`) when `givn skills sync` runs. |
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
| **Event Storming** | The ideation phase that models the domain as a chain of past-tense domain events with actors, commands, aggregates, policies, and read models, drawn as a rendered mermaid canvas whose aggregates are declared consistency boundaries (never records, specifications, configurations, or services). Anti-term: Storm (rejected as ambiguous). |
| **Use Case Definition** | The ideation phase that defines the use cases from the event storming protocol: name, goal level, boundary, and affected stakeholder types. Anti-terms: slicing, cutting. |
| **Use Case Elaboration** | The ideation phase that elaborates each defined use case into acceptance rules, concrete examples, and the use-case contract. Anti-terms: mapping, specification (reserved for the feature file). |
| **Persona** | A documented embodiment of one persona segment, kept as a living record of decisions, needs, and open demands, that reviews every artifact affecting it. Anti-terms: user profile, role, actor. |
| **Stakeholder type** | A class of interested parties identified during Frame; one type may carry several personas. Anti-term: stakeholder group. |
| **Persona segment** | One distinct incarnation of a stakeholder type. A new segment exists only when it differs from an existing one in at least one of: authority boundary, evaluation capability, or own vocabulary. Topic-specific demands live in a dated topic-stances appendix, never in the stable core. Anti-terms: variant, flavor. |
| **Domain terms** | The per-topic candidate file `givn/ideation/<topic-id>/domain-terms.md` holding domain terms as they crystallize; candidates are merged into the durable term list at handoff by user decision. Anti-term: glossary candidates (the always-on guidance may not name the addon-owned artifact). |
| **Persona promotion** | The user-approved append of a validated persona's running-record entries into the project-level persona collection at handoff, with a confirmed merged diff; the topic file stays as historical record and the biography lives in one place. Anti-term: export. |
| **Change seed** | The per-change proposal seed written into the topic state at handoff, cut by deliverable guarantee: one seed may cover several use cases, one use case may yield several seeds, and no seed creates a state where a confirmed rule is untrue. The user starts the normal proposal flow from it; handoff never starts a change. Anti-terms: stub, pre-proposal. |
| **Seed status** | The first line after a change seed's title: `ready`, `proposed by <change-id>`, `implemented in <change-id>`, `superseded by <reference>`, or `withdrawn`. Proposal planning and archiving stamp it; only the status line changes on supersession. |
| **Term record** | The `domain-terms.md` entry for one domain term: status (`candidate`, `confirmed`, `rejected`), one-sentence definition, contrast, first use, and the recorded collision check. A `confirmed` term has passed the term introduction test and carries the user's verified name — asked immediately when the name fails, otherwise at the phase's term check. |
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
| **Development record** | The append-only, scope-tagged local journal of what Givn and the coding agent did during a project's development: one entry per command, phase, hook, verification step, revision, or reported agent activity, each with a measured duration and an outcome. Anti-terms: log file (a plain log has no scope, outcome, or parent), event store (reserved sense: a system of record the workflow depends on — this record is never a gate). |
| **Record entry** | One line of the development record: an id, a timestamp, exactly one scope, a kind, a bounded label, an optional measured duration, an outcome, and an optional parent entry. Anti-term: event (reserved for Event Storming domain events and harness hook events). |
| **Development scope** | The single owner an entry belongs to: one change, one ideation topic, or the project. Resolution order is change, then topic, then project. |
| **Intake** | `givn history ingest` — the one contract through which a harness integration hands reported agent activity to Givn; it validates each event, stores the accepted ones, names a rejected one without failing the caller, and is the only writer for reported activity (ADR-073). Anti-term: ingest endpoint (there is no network endpoint). |
| **Agent reporter** | The generated integration for one supported harness (omp, opencode, Claude Code, or Maki) that observes that harness's own activity and delivers it through the intake: it keeps its own bounded buffer, is installed beside that harness's other integration, and never writes the record itself. The neutral `agents` target is a skills mirror that no harness executes and so has no reporter. Anti-terms: agent monitor, telemetry agent. |
| **Folder trust** | Maki's own gate on automatic shared project configuration: in a folder it has not been trusted for, Maki loads nothing from `.maki/` and prints a skip warning, so the Maki reporter is inactive there until the developer trusts the project. It is the harness's decision, not Givn's, and Givn reports no error for it. |
| **Pending state** | The small per-session file a process-per-hook harness needs so start timestamps survive between its separate hook processes: opaque harness call ids mapped to a start time, an entry kind, and the bounded label — starts, kind, and label only, with no mode cache (Claude resolves the mode in-process) and no payload content. It is written atomically, pruned by age and at session end, and is never written under `off`. Anti-term: checkpoint. |
| **Metadata bound** | The documented limit on a non-content metadata value: 120 characters for a label, 240 for text metadata (title, name, notification), 1 000 for error text, and 4 000 for question text (the one content value always captured) — always applied by the intake, never gated. Anti-term: truncation policy (there is one bound table, not one per reporter). |
| **Full content** | The complete LLM-facing content of an activity — tool input and output, assistant and thinking text, user prompt, compaction summary, plan content, draft text, subagent transcript excerpt — stored inline in the record only in the `full` mode (or when the intake is called with `--with-full-content` while the mode is not `off`), capped at 64 KiB per field and 256 KiB per entry. Question text is not part of it: it is captured in `on` and `full` alike. Anti-terms: raw log, transcript dump. |
| **Record mode** | The project's three-state capture control, `record.mode: on \| off \| full` in `givn/config.yaml`: `off` is absolute (no append, no pending state, no ignore write, and `--with-full-content` is ignored with one warning and exit 0), `on` (the default, and the fallback for an absent or invalid value) records metadata and full question text, `full` adds complete LLM-facing content. Lowering the mode retains already-captured entries; it never purges. The intake enforces it; reporters learn it from `givn history settings` (source `config` or `default`) and treat a failed query as `on`. Anti-terms: switch (there are three states, not two), log level. |
| **Bounded label** | The single ≤ 120-character subject label an entry keeps in place of a complete subject; it is the label bound, not the record's full privacy boundary — question text is always captured in full (capped at 4 000 characters) and complete LLM-facing content is stored in `full`. Anti-term: redaction (the label is not a general content filter). |
| **Record gap** | A recorded shortfall the report shows instead of hiding: events the reporter dropped, a flush that failed, or a store that could not be written. |
| **Development report** | The self-contained interactive file `givn timeline` writes for one development scope: a top-to-bottom workflow of phases with the derived current state (for a topic the ideation workflow; for a change the change workflow and, when linked, an optional ideation group above it), each phase expandable into a vertical lane timeline with its lane log, plus ranked long steps, question-wait totals, repeated-step groups, subagent fan-out, and token totals. The lane log carries the captured metadata facts and one bounded content region; the report embeds full content up to an 8 MiB total budget and marks the rest `content omitted; see the record`, while the store stays complete. Every number and lane counts the report scope's own entries, and a linked ideation group counts its topic's entries: the recorder's own command entries (the intake's flush and the process-per-hook mapper's invocation) are omitted before any derivation, while the record and its read views keep them. Anti-terms: dashboard (one of the rejected style tiles), log viewer. |
| **Band** | One row of the *project* report: the spans of one scope window — an archived change, a change or topic in flight, or the "outside a scope" row that carries the project's own entries. A change or ideation report replaces the band row with the workflow stack. |
| **Workflow phase** | One step of the change workflow in the report's top-to-bottom stack (`propose` through `archive`): status done, active, queued, or skipped; wall/active/waiting time and tool/subagent counts from the record; expanding it opens its vertical lane timeline. The current phase is the workflow phase mapped from the change state's next required artifact (a `partial` tasks artifact means `implement`; every required artifact done means `archive`), or the last stage the record names when no live change state exists. |
| **Lane** | One recorded session's column inside an expanded phase: the main agent lane or a session's lane, named by a subagent span only when the span's `data.subagent` equals the lane's session id, otherwise by the session title, else by the session's short id. A lane groups only entries recorded under that session; a lane that merely contains a subagent span keeps its own name, and the report never assigns a tool call to a subagent the record does not name. |
| **Lane log** | The selected lane's entries below the phase's vertical axis: tool calls, turns, revisions, questions, and the subagent span, with their facts, duration, outcome, and captured content, bounded by the entry caps and the 8 MiB report budget. |
| **Ideation phase** | One step of the ideation workflow in the report (`frame`, `personas`, `event-storming`, `use-case-definition`, `use-case-elaboration`, `handoff`): a phase starts at its first observed topic marker (the first persona file, `eventstorming.md`, `use-case-definitions.md`, the first elaborated use case, the first seed or the published topic document); `frame` is the opening stage. Anti-term: change phase (the two workflows never mix). |
| **Linked ideation** | The optional labeled group above a change report's workflow, rendered only when the change's proposal or seed path names an existing `givn/ideation/<topic>`; it keeps its own ideation phases, lanes, log, and totals, and never becomes the change's current phase. A linked topic with no recorded entries says so instead of showing invented phases. Anti-term: merged timeline (the change's totals are not silently combined with the topic's). |
| **Change seed** | `givn/ideation/<topic>/changes/<change-id>.md` — the handoff artifact naming one future change: its use cases, personas, suggested start order, accepted decisions, and open questions; its status line is stamped `proposed by <change-id>` and later `implemented in <change-id>`, making it the durable backlink from a change to its topic. |
| **Repeated-step group** | A phase's entries of one kind and one normalized label, grouped when at least five members exist; the report names the count and the combined measured duration, and the group never hides an entry from the ranking. Anti-term: loop (kept as the visible card heading, not a distinct term). |
| **Current state** | The change state the report embeds at generation: the stage label, the current workflow phase, the next required artifact, and task progress where the state has them; absent for a topic, an archived change, and the project report, which have no live change state. Anti-term: live status (the value is a snapshot, not polled). |
| **Question wait** | The span between an agent asking the developer a question and the answer resuming its work; it is recorded as its own entry so waiting is distinguishable from working. |
| **Aggregate marker** | One report tick that stands for a group of entries too short to draw individually at the current phase zoom, naming how many entries it covers and their combined duration; the sub-pixel threshold applies at every zoom, so it resolves into individual ticks only where the current zoom permits. Anti-term: cluster (reserved for use-case grouping). |
| **Subagent span** | The recorded start and end of one delegated subagent execution, nested under the turn that spawned it, so fan-out into many small subagents is visible. |
| **Outline experiment** | The default-off `outline` addon that enables the outline-first report and the change-relative outline review gate. It is an experiment, not a product-wide gate: a project that has not opted in reviews exactly as before. Anti-term: code-quality score. |
| **Outline report** | The deterministic human or JSON snapshot of production source units: each unit's method outline first, then metrics and one classification as hints. It emits no action or recommendation, stores no baseline or history, and uses schema version 3. Anti-term: outline baseline. |
| **Python analyzer** | The second `SourceAnalyzer` adapter: it starts `python3` with a bundled standard-library `ast` extractor only when the scope contains at least one `.py` file, parses text without importing or executing project code, and returns the same neutral model as the Rust adapter. Anti-term: Python runtime dependency (a Rust-only project never starts it). |
| **Metric hint** | An analyzed unit whose classification is not `within-limits`. The human report labels it `HINT` and keeps the classification set: `god-type`, `interface`, `multi-type-module`, `module`, `data-only`, `cohesive-large-type`, or `method-length`. A hint never blocks. Anti-term: recommendation. |
| **Gate metric hint** | A touched unit with a metric hint at the change base or at the committed head, printed as `HINT` by the gate. It informs the reviewer and never creates, satisfies, or blocks an outline verdict. Anti-term: gate finding (nothing metric gates). |
| **Outline review gate** | The change-relative enforcement step inside `givn check review --change <id>` that prints the outline of every touched unit with methods and fails while any outline lacks one fresh-context `PASS` with a responsibility and call-order explanation, or carries a `FAIL`. It is not a command, a project-wide scan, or an archive gate. Anti-terms: outline score, baseline check. |
| **Change base** | The revision the outline review compares against: the first parent of the first revision reachable from `HEAD` that touched `givn/changes/<change-id>/`. A merge uses its first parent, a root commit yields an empty base tree, a rebase recomputes the first touch, a shallow clone fails closed, and no reachable touch is an error, never a fallback to `HEAD`. Anti-term: merge-base (a different, rejected base). |
| **Touched unit** | A source unit a change reaches: the unit that declares in a changed production file, every unit whose files include that path, the file's module, and every implementor of a touched trait. Anti-term: changed file (a file is not the unit). |
| **Hint** | The umbrella term for a report metric hint and a gate metric hint. A hint carries the unit id, classification, and base/head evidence; it never blocks. Anti-term: violation. |
| **Outline verdict** | One `OUTLINE: PASS <unit> | <responsibility; call order>` or `OUTLINE: FAIL <unit> | <reason>` per printed outline. A missing verdict, an empty PASS explanation, a FAIL, or conflicting verdicts block. There is no justification path. Anti-term: suppression. |
| **Reviewer identity** | The `OUTLINE: AUTHOR <id>` and `OUTLINE: REVIEWER <id>` lines required when any outline verdict exists; the reviewer must differ from the author. Anti-term: self-review. |
| **Code ownership** | The project rule that everyone working on the project owns all of its code: seeing technical debt and leaving it because "it was not me" is not acceptable. An outline verdict cannot use ownership or scope as a reason. Anti-term: file ownership. |
| **Source unit** | The unit the outline report and gate measure: one type aggregated across its declaration and impls in other files, or one module file. Anti-term: file finding (a file can hold several units). |
| **Production source** | A Rust or Python source item in the production roots that is not a test or tooling source: `tests/`, `benches/`, `examples/`, `target/`, Python cache and virtualenv directories, Python test modules (`test_*.py`, `*_test.py`), and `#[cfg(test)]` items are excluded. Anti-term: all source (test and tooling sources are out of scope). |
| **Outline classification** | The one label every analyzed unit receives: `god-type`, `interface`, `multi-type-module`, `module`, `data-only`, `cohesive-large-type`, `method-length`, or `within-limits`. Anti-term: score. |
| **Readable size** | The 500-code-line threshold for a type and the lower module boundary: a unit at exactly 500 code lines is within limits, and one at 501 is a finding. The old watch band is not a threshold term; the only thresholds are the readable size and the plan limit. Anti-term: soft limit. |
| **Plan limit** | The 1,000-code-line upper module boundary inherited from the report's earlier vocabulary: a module above it is a `module` finding, and a module at exactly 1,000 is also a `module` finding because the module classification starts at 500. It names a threshold, never an action. Anti-term: exception limit. |
| **Inspect band** | The 500–1,000 code-line module range in the report's historical vocabulary: a module there is `multi-type-module` or `module`, and a type there is `cohesive-large-type`. The term survives because the report's removed and added scenario titles name it. Anti-term: watch band (the earlier action label). |
| **Review head** | The committed `HEAD` revision the outline review gate exports and analyzes. The working tree is never the head; a dirty production path fails before analysis. Anti-term: working tree. |
| **Outline** | The report's structured unit summary: type, field, method, and trait-method signatures with visibility, order, and call order, but no bodies, comments, or call arguments. |
| **LCOM4** | The number of connected components in the graph of a type's production methods and fields, with method–field and same-file method–method edges. A type with no methods scores 0; one method scores 1. |
| **TCC/LCC** | Tight and loose class cohesion over the methods that access at least one field: directly connected method pairs (TCC) and transitively connected pairs (LCC), stored as integer permille and absent when no method pairs exist. |

Fossil-specific clauses in this glossary (Fossil `EXTRA` dirty semantics,
attribution through `givn commit`, the Fossil backend itself) describe the
implemented contract: the archive, the managed upgrade, and the SCM-neutral
commit path all run on Git and Fossil.

### Visual-review invariants

- A change with a visual contract cannot pass review without a visual
  verdict (enforcement site: `check review`).
- Every Web UI end-to-end scenario has one committed final-state screenshot
  per contract viewport; missing evidence blocks review (enforcement site:
  `check review` evidence completeness). A terminal end-to-end scenario commits
  a captured final transcript instead, with the applicable text baseline;
  a planning reference is not that consumer execution evidence.
- Every declared Web UI state has committed per-viewport evidence (enforcement
  site: `check review` state-evidence check); terminal contracts name the
  covered textual states and preserve their transcripts.
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
- A resumed candidate repeats the delta merge only when the receipt records that
  the merge did not complete (enforcement site: `merge_completed` in
  `cli::archive`).

*[Architecture documentation index](README.md)*
