# Design Review: fix-opencode-v2-attribution

Reviewed and hardened: 2026-10-04; re-reviewed 2026-10-05 (flake-only Nix
boundary) and 2026-10-06 (V2 rewrite including subagents, D3).
Documentation/specification work only.

Reading note: sections up to “Re-review sign-off (2026-10-05)” are the record
of the earlier plan, in which only `server()` delegation was added to the
supplied prototype. Where they still differ from the current plan, the section
“Re-review after the V2 rewrite (2026-10-06)” supersedes them. Rows below that
the 2026-10-06 hardening corrected in place are marked “(updated 2026-10-06)”.

## Evidence boundary

The orchestrator supplied the complete outcome of an independent fresh-context,
high-reasoning grilling review and the sole operator answer. This fresh hardening
session records that outcome, reads the complete proposal, both delta features,
design, marker, twelve chapters, linked use-case/Persona context, and relevant
source, and applies only authorized factual documentation corrections. The
original independent chapter assessment below predates the marker comparison;
it is not retroactively represented as this hardening session's reading order.

No production code, plugin, test binding, runner, Nix file, configuration, task
list, or global workflow policy was edited. No compatibility, cleanup,
strictness, coverage, or terminal-evidence test was executed during grilling or
hardening. Historical source/help/network and lint findings in the design retain
their original provenance; they are not fresh scenario-completion evidence.

**Historical validation blocker — resolved by parent evidence, 2026-10-04:**
this hardening session has no shell-execution tool.
Tool-catalog discovery confirmed that limitation before file exploration.
`givn instructions` could not run as the first command, nor could the required
resolved instruction commands or fresh lint. Source instructions were read as
fallback context, not claimed as effective CLI output. The initial review
therefore recorded FAIL rather than waive validation. The parent subsequently
supplied actual resolved-instruction, fresh lint and whitespace-check results;
the Validation section records their provenance. A separate fresh quality
reviewer approved the hardening without blockers, majors or minors. The blocker
is resolved by executed parent checks, not an operator deferral or a claim that
this documentation session ran commands.

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Deferrals / Testability | Evidence for genuine characterization/reuse scenario revisions | Agreed: exact operator answer “1”, 2026-10-04. Narrow production-diff exception only; one atomic revision per scenario and all executable proofs retained. |

### Characterization/reuse scenario evidence (Q1)

- Status: closed 2026-10-04 (operator); answer supplied by the orchestrator.
- Blocks: a compliant scenario-by-scenario task plan without artificial
  production edits to preserved attribution logic.
- Context: the compatibility repair preserves the supplied plugin logic and
  changes one production adapter asset. The design plans 18 separate scenario
  commits, but characterization and regeneration scenarios may already pass
  after that asset is replaced. The completion-review policy requires each
  scenario commit to touch production source. A genuine test-only
  characterization commit does not satisfy that literal rule. This blocks a
  compliant task plan without an explicit exception or artificial production
  edits.
- Full question presented:

  > The compatibility repair preserves the supplied plugin logic and changes
  > one production adapter asset. The design plans 18 separate scenario commits,
  > but characterization and regeneration scenarios may already pass after that
  > asset is replaced. The completion-review policy requires each scenario
  > commit to touch production source. A genuine test-only characterization
  > commit does not satisfy that literal rule. This blocks a compliant task plan
  > without an explicit exception or artificial production edits.
  >
  > Which evidence rule should apply to this change?

- Options presented:
  1. **Allow a narrow exception for characterization/reuse commits —
     recommended.** Meaning: keep one atomic commit per scenario. A test-only
     commit must contain executable assertions, targeted RED/GREEN/REFACTOR
     evidence, and a reference to the production-changing adapter revision it
     verifies. Gain: preserves scenario-level evidence without altering the
     supplied logic. Cost: completion review must explicitly recognize this
     exception. Example: the scenario proving that an omitted reasoning mode
     retains high commits its bindings and execution evidence, then references
     the adapter revision; it does not rewrite the plugin again.
  2. **Allow one combined production-bearing commit for the compatibility
     scenario set.** Meaning: implement and verify scenarios individually, but
     publish the adapter and their tests together. Gain: one revision contains
     the production repair and all verification. Cost: waives
     one-commit-per-scenario publication and loses scenario-level rollback
     granularity. Example: loader compatibility, retained fallbacks, and
     regeneration checks all reference the same combined revision.
- Recommendation presented:

  > Option 1, limited to this change and genuine assertion-bearing
  > characterization/reuse commits. Reject production comment churn or repeated
  > plugin rewrites solely to satisfy the diff requirement.

- Exact user answer: **“1”** on **2026-10-04**.
- Resolution: allow the narrow characterization/reuse production-diff exception
  only for `fix-opencode-v2-attribution`. Retain one atomic `givn commit` per
  scenario, real executable assertions, targeted RED/GREEN/REFACTOR evidence,
  and the explicit production-changing adapter revision link. If retained
  behavior already passes, say so; record the genuine binding or negative-control
  failure and the assertion-bearing GREEN without inventing a product defect.
- Rejected: combined scenario publication, because the operator chose
  scenario-level atomic evidence; artificial production edits, comment churn,
  and repeated rewrites, because they violate preservation and add no evidence;
  fabricated RED, empty steps, spec-only commits, batching, removal of `@e2e`,
  skipped tests or waived strictness, because none was authorized.
- Realization: `design.md`, “Scenario revision evidence exception”. Completion
  review must recognize this explicit authorization, not silently change its
  general rule. Global policy remains untouched. This authorization is not
  GREEN and does not authorize a validation deferral.

## Resolved by codebase exploration

The original grilling findings are retained in full by branch. Source reads in
hardening corroborate the named local contracts; released-source observations
remain explicitly attributed to the supplied independent grilling record where
not independently executed here.

| Branch | Finding |
|---|---|
| Scope | No proposal/spec mismatch. Both deltas extend existing `attribute-commits` and `enable-commit-attribution` under the reusable support fragment, `givn/specs/fragments/fragment.md`. Changed actions are init, skills sync, target addition, addon enable, addon disable, and managed upgrade. Reinit, no-addons init, and target exclusion are regular variants. Three modified titles already exist permanently. Done requires supported-loader compatibility **and** retained behavior; no separate generated V1 adapter, version detection, preservation feature, or generic stamper change. |
| Decision ledger | (updated 2026-10-06) Supported-generation compatibility (D1), supplied-behavior preservation (D2), the V2 rewrite (D3) and its stale-request refinement retain full provenance/options/rejections in `proposal.md`; `design.md` resolves all four in its decision ledger and states that D3 supersedes D2 for the V2 session link and default lookup. Source is a direct defect report, not an ideation seed. |
| Tech choices | `src/config/embedded.rs` embeds the asset; `src/skills/targets.rs:399–448` is the canonical selected-target integration writer and overwrites unconditionally. Init, skill sync, target addition, addon enable/disable and upgrade converge there. Official V2 plugin migration docs support default `id/server/setup` and V1 object entrypoints from 1.18.29. Grilling inspected released `@opencode/core@2.0.22` default `id/setup` or `id/effect` decoding. One local asset replacement is simpler than runtime detection or two generated files. No new production library or ADR. |
| Tech choices / source freshness | Loaded OpenCode skill. Read official V2 plugins, plugin migration and configuration docs; no V1 documentation was used to infer V2 shapes. V2 `context`, `compaction`, `generate`, shell `create.before`, and `ctx.model.default()` are valid. Context7 resolution was attempted again in hardening and returned `Invalid API key`; official V2 docs and the released-source evidence in the grilling record are the fallback. Existing release refresh/pinning remains a mandatory mechanical setup step, not an invented current version. |
| Tech choices / runtime shell | Corrected the false assumption that the built-in V2 tool executes the configured fixture shell. Grilling inspected released V2 compatibility-priority selection: fish is incompatible and Darwin falls back to `/bin/zsh`. The revised orchestration uses POSIX `sh`; the pinned Fish executable remains a test dependency only for explicitly configured fixture behavior. Validation records the actual selected executable/reason per pinned runtime/platform and requires that executable; fixed `node attribution-fixture.mjs` remains valid. No production shell logic was introduced. |
| Missing scenarios | All mandatory retained paths are represented: missing/no/identityless/failing default, absent/unknown sessions, remembered/session/default precedence, omitted-variant retention, both `id`/`modelID` spellings, and the three non-title request kinds. (updated 2026-10-06) Registration is checked against the design's hook list; prototype behavior comparison covers the V1 hooks and retained fallbacks only, because the V2 `setup()` is rewritten. Executable/loading/result/timeout failures are hard infrastructure errors, not new product fallbacks. No missing scenario or authorized scope expansion was identified; feature edits are unnecessary. |
| Testability | `tests/features_runner.rs:54–64` uses `fail_on_skipped()` and explicit failed/skipped non-zero exit; nextest uses the same strictness at `:125–133`. Design requires independent undefined/unimplemented probes under cargo, nextest and E2E, nonempty `unimplemented!` starting bindings, anchored single-title command, concrete attribution/committer assertions, and (updated 2026-10-06) 23 scenario identities. Current 38-line V1-only asset cannot pass the V2 loader matrix. These observations are static; strictness probes have not run. Characterization reuse is governed only by the operator exception above. |
| Testability / canonical gate admission | The operator clarified that Nix must appear only in `flake.nix`. Revised design inlines the pinned runtime/module derivations in `flake.nix`, exports the exact `GIVN_OPENCODE_*` paths from the dev-shell hook, and uses POSIX-shell `resolve-runtime.sh` only to validate those exported paths. `run-tests.sh`, `run-suite.sh`, and direct Cucumber test admission do not invoke Nix or Fish; without the active dev-shell environment they fail with a named missing-variable error. The wrapper retains the canonical `./run-tests.sh` and `./run-tests.sh --e2e` commands and prepares `ready_fixture`/supervisor before scenarios. Parent direct-shell validation succeeded with explicit paths from the existing Nix store; this is validator evidence, not a flake rebuild or test-suite proof. |
| Testability / binding enforcement | `src/integrity_hook.rs:526–540` skips promised-file validation without exact `## Step Definitions`. Added that heading and four rows, capability in column one and exactly one existing path in column two: `tests/steps/llm_attribution_steps.rs`, `tests/steps/llm_attribution_e2e_steps.rs`, `tests/steps/llm_attribution_installation_steps.rs`, `tests/steps/llm_attribution_installation_e2e_steps.rs`. Regular/E2E remain separate. Presence is not assertion-quality or GREEN proof. |
| E2E fidelity | (updated 2026-10-06: eight, adding the V2 subagent characterization scenario) Seven changed E2E outlines correspond to distinct capability/action boundaries; release, target and missing/obsolete Examples are variants. Real provisioned OpenCode executes its built-in shell tool against deterministic local twins. Primary assertion is the completed acknowledged CLI tool result, not model prose; regeneration also retains Givn command reports. Read `llm_attribution_e2e_steps.rs`: its Node public-hook call and mutable-root parity are explicitly replaced by the real driver, not credited as loader proof. Grilling searched for stronger drivers and found none. The separate 38-line legacy harness plugin is excluded and remains untouched. |
| Interaction coverage | (updated 2026-10-06: 39 with the subagent row) All 38 durable inventory titles match permanent or delta E2E titles and design matrix rows. Corrected the retained “Synchronized guidance explains how to resolve the deterministic gates” driver to `tests/steps/mod.rs::user_runs` (real sync) plus `prose_consolidation_steps.rs` (generated-content assertions), not instruction-reading `skill_rewrites_steps.rs`. Non-attribution rows remain outside repair scope and their historical assertion limitations are not claimed fixed. |
| Risk | Most likely failure is isolated runtime admission/cleanup, not adapter entrypoint complexity. RAII does not cover runner SIGINT/SIGTERM or `process::exit`. Design now specifies test-only `tests/opencode_supervisor.rs`, existing `libc`, `sigaction` with signal-safe notification, parent pipe lease/EOF via `poll/read`, `CommandExt::process_group`, owned fixture PGID, bounded readiness/deadlines, TERM then 3-second KILL escalation, direct-child reaping, Linux subreaper/waitpid and Darwin OS-reaping/disappearance checks. Controlled interrupt, runner-exit and timeout probes must terminate owned server/descendants/twins without touching an unrelated sentinel service. No probe ran; chapter 11 now states this boundary. |
| Domain context / use-case and Persona | Support fragment owns the capabilities; initialization and upgrade contracts have typed `Includes: fragment: support`. Historical use-case rename provenance does not change current identity. Three confirmed Personas are backed by `givn/ideation/multi-scm-support/session.md` confirmation and promotion to `givn/personas/`. Developer needs no repeated manual repair and preserved targets/work; technical role needs parseable machine/revision evidence without human-approval substitution; maintainer needs one writer, truthful best effort, immutable baseline and bounded harness. No Actor, Persona, use case, Aggregate or Domain Event was invented. |
| Domain context / use-case scope | Applied review instruction's rejection criteria to linked contracts: identifiable actors/triggers, mutating guarantees, goal-shaped capability ownership, numbered extension references and typed Includes remain valid. Found pre-existing upgrade contradiction: precondition assumed a clean-state witness that main flow obtains; `src/cli/migration.rs:351–354` selects backend and reads dirty entries. Narrowly changed `givn/specs/upgrade-a-project/usecase.md` to repository/backend information availability as precondition and fresh witness acquisition/check in main flow. Missing-backend refusal remains reachable; no product scope rewrite. |
| Visual contract | `src/integrity_hook.rs:120–125` recognizes only the exact value `terminal` or `web`; old “terminal; no new…” silently became `none`. Corrected to standalone `- Interface: terminal`, explanation separate. No introduced/redesigned screen requires rendered options, browser viewports or fictional approval. Real terminal transcripts and per-Example ordered shards remain mandatory for the seven affected E2E titles; they are not already captured. |
| Interface classification | Tested the opposite: a rendered browser screen is Web UI and requires a browser. Although OpenCode serve can host UI, this change introduces/changes no browser screen; actual interaction is CLI invoking built-in shell. HTTP is readiness/secondary diagnostics only. Therefore CLI driver and terminal evidence are correct, not an HTTP substitute for browser evidence. |
| Deferrals | No implicit runtime, network, strictness or terminal-evidence deferral. Unavailable prerequisites fail. Sole explicit operator exception is the production-diff rule for real characterization/reuse revisions, not skipping tests. Hardening's lack of shell initially blocked instruction/lint validation; actual parent command evidence on 2026-10-04 resolves that historical blocker without deferral. |
| ADR qualification | Active/archive register search and generated-agent ownership (ADR-013) / preservation-oriented upgrade (ADR-023) reads found unchanged boundaries. Both candidates fail qualification and name exactly one canonical lower-level home; summaries below. No new, amended, replaced or archived MADR required. Test-only resolver/supervisor mechanics fit the same reversible design rather than a new authority boundary. Chapter 11 contains retained consequences. |
| Architecture documentation | Original all-12 independent table found row 1 Yes versus marker No: selection row maps chapters 01 **and 10**, with quality refinement already in chapter 10. Corrected design handoff and marker to Yes without inventing chapter-01 goals. Corrected stale `.claude/settings.json` footprint to `.claude/settings.local.json`, confirmed by `src/skills/targets.rs:452` and glossary's user-owned/shared-settings distinction. Aligned chapters 05/07/08/10/11 with shared admission and bounded teardown; all twelve are substantive. Original and final assessment preserved below. |
| Ubiquitous language | (updated 2026-10-06: Issuing session added; Model identity amended) Attribution context, Model identity and Reasoning mode in `docs/arc42/12-glossary.md` already describe best-effort inference and retained variants, not verified current-session facts. Session map/lastModel are ephemeral adapter state, not Aggregates. Existing `llm_attribution_*` binding names are code symbols, not new domain nouns; no renaming. Generic stamper still never fabricates missing values. |
| Security / errors / Git | Cleared fixture credentials/config, fake keys only, owned ports and fixed safe command; no inherited provider secrets, global plugins or host service. Swallowed default lookup errors remain retained behavior (the V2 half is rewritten under D3); driver errors are hard. Supplied immutable baseline hashes remain unchanged in design; no root plugin rewrite. No issue ID, commit identifier, executed GREEN or cleanup proof was invented. Dirty user work preserved by narrow patch-only documentation edits. |

## Open questions

None. All operator questions are answered; both proposal decisions remain
binding. The historical instruction/lint execution blocker is resolved by the
parent's actual results below; no execution blocker or authorized deferral
remains at this planning-stage sign-off.

## Use-Case And Persona Context

| Check | Finding | Disposition |
|---|---|---|
| Contract/capability ownership | Initializing a project (`initialize-project`) and upgrading a project (`upgrade-a-project`) include the reusable support fragment; two EXTEND routes retain reusable attribution ownership. | Confirmed existing contracts, no new use case or duplication. |
| Typed relationships | Both consumers include `fragment: support`; no conditional Extends relationship is invented. | Preserved. |
| Use-case scope | Upgrade previously assumed a witness acquired by the flow. | Factual precondition/main-flow correction applied; guarantees/refusal scope preserved. |
| Actor/Persona distinction | Developer, supporting Authoring agent, Project maintainer, and OpenCode acting for developer are interaction roles; confirmed Personas are stakeholder lenses. | Preserved; no auto-promotion. |
| Persona confirmation | Session confirmation/promotion on 2026-09-14 and project biographies resolve all three references. | Existing confirmation sufficient; no new operator question required. |

### Persona disposition

| Persona | Relevance | Evidence / disposition |
|---|---|---|
| Primary user, developer/maintainer (`developer--primary-user`) | Routine init/sync/upgrade should not require manual plugin repair or lose selected targets/work; on V2 each agent's commits, subagents included, should carry that agent's model. | Proposal promise, six-action generation matrix, reinit/no-addons/no-target edges and clean upgrade fixture; (updated 2026-10-06) gains subagent-correct labels through the issuing-session link and its regular/E2E scenarios. Relevant; planned assertions preserve outcome, not yet runtime-proved. |
| Technical role, authoring agent/automation (`agent--technical-role`) | Needs exact executable commands, JSON tool results and one revision per scenario; cannot supply human judgment. | Shared admission, fail-closed result scope, explicit binding table, anchored command and authorized characterization evidence rule. Relevant; unchanged by the 2026-10-06 rewrite. Neither operator authorization nor prose substitutes for execution. |
| Givn maintainer/owner (`givn-maintainer--owner`) | Must keep one adapter/writer and a truthful compatibility scope with bounded test processes. | (rewritten 2026-10-06) V1 semantics preserved; V2 session link plus retained fallbacks; limits documented (chapter 11 R-73–R-81, design “Retained risks and limitations”). Recent-V1/V2 matrix, non-ADR qualification of all three candidates. Relevant; no broader production seam or vocabulary beyond the issuing session. |

## Architecture documentation (arc42) check

The independent original audit below is the fresh grilling review's assessment
formed **before** accepting the marker. Hardening preserves its original
mismatch and records final alignment after correction. A Yes applies to the
selection row, not an instruction to rewrite every mapped chapter.

| # | Chapter | Independent Yes/No | Marker Yes/No, original → final | Match?, original → final | Content matches design |
|---|---|---|---|---|---|
| 1 | Introduction/goals; selection row maps chapters 01 and 10; the 2026-10-05 operator constraint also affects rows 4, 5, 7, 8, 10 and 11 | Yes | No → Yes | **No → Yes** | Compatibility/truthfulness quality refinement is in chapter 10. Chapter 01 is substantive; unchanged goals/stakeholders need no edit. |
| 2 | Architecture constraints | Yes | Yes → Yes | Yes → Yes | TC-29 recent-V1/V2 window and best-effort boundary; substantive. |
| 3 | Context and scope | Yes | Yes → Yes | Yes → Yes | V2 External System interface and unverified fallback; no production provider twin. |
| 4 | Solution strategy | No | No → No | Yes → Yes | Embedded adapter/one binary/selected-target strategy unchanged; substantive. |
| 5 | Building block view | Yes | Yes → Yes | Yes → Yes | One dual-entrypoint adapter and existing writer; test-only acceptance blocks use a shell validator, permanent-only setup wrapper, post-TDD four-root wrapper and supervisor. |
| 6 | Runtime view | Yes | Yes → Yes | Yes → Yes | Session/last/default fallback and late shell-ID order; regeneration preconditions retained, substantive. |
| 7 | Deployment view | Yes | Yes → Yes | Yes → Yes | One canonical plugin; runtime closure is supplied by the flake development shell; test-only process supervisor remains outside deployed services. |
| 8 | Cross-cutting concepts | Yes | Yes → Yes | Yes → Yes | Attribution environment/generation plus shell-only validation of flake-exported paths and fail-closed missing-runtime behavior. |
| 9 | Architecture decisions | No | No → No | Yes → Yes | No qualified new ADR; existing active/archive register and ownership/upgrade boundaries retained, substantive. |
| 10 | Quality requirements | Yes | Yes → Yes | Yes → Yes | Q-78/Q-93–Q-95 plus canonical-shell validation and unexecuted runtime/signal/timeout prerequisites. |
| 11 | Risks and technical debt | Yes | Yes → Yes | Yes → Yes | R-50/R-73–R-76 include best-effort risks, flake-exported runtime validation and signal-aware cleanup rather than RAII alone. |
| 12 | Glossary | Yes | Yes → Yes | Yes → Yes | Narrow Attribution context/Model identity/Reasoning mode definitions, best-effort limit; no new domain Aggregate or Domain Event. |

### Chapter integrity and decision consequences

- [x] All 12 rows independently assessed in the supplied fresh grilling record
  before marker comparison; complete original/final trace retained.
- [x] No final Yes/No mismatch or omitted assessment remains after correction.
- [x] All 12 chapter files exist and contain substantive project content;
  none is scaffold-identical. Full chapter reads corroborate the original audit.
- [x] No Unicode box-drawing or non-table ASCII pseudo-diagram exists in these
  twelve chapters. Hardening grep found no box-drawing matches; full reads
  distinguish Markdown tables/plain path lists from diagrams. Existing chapter
  diagrams are Mermaid; no new diagram was needed.
- [x] No claimed updated chapter is placeholder-only. Placeholder-related
  vocabulary found by grep describes actual Givn placeholder contracts/debt,
  not unfilled architecture sections.
- [x] Every ADR candidate has a qualification summary and one route-specific
  canonical destination. Must-be-shared is supporting only. No qualifying
  candidate lacks a chapter-09 MADR entry; there are no new qualifying candidates.
- [x] Retained compatibility/fallback and test-lifecycle consequences are in
  chapter 11. No new MADR “Bad, because…” consequence is missing.

### ADR qualification summaries

| Candidate | Mandatory gates and verdict | Route / exact canonical destination | Cheaper home and falsification |
|---|---|---|---|
| Shared dual-generation entrypoint and recent-V1/V2 support realization | Alternatives PASS; architectural impact PASS (external protocol, ownership unchanged); durable consequence FAIL (one reversible asset, no migration/authority boundary); lower-level artifact FAIL (complete rationale fits design); existing-ADR check PASS; must-be-shared SUPPORTING. **NOT_QUALIFIED**. | CANONICAL_ARTIFACT: `givn/changes/fix-opencode-v2-attribution/design.md` | Same design is the cheaper home. Single-change reversibility applies; no multi-change constraint applies; support promise is observable product behavior. No new/amended/replaced ADR. |
| Retain supplied best-effort model and reasoning fallbacks | Alternatives PASS (preserve versus current-session-only unavailable); architectural impact FAIL; durable consequence FAIL; lower-level artifact FAIL; existing-ADR check PASS; must-be-shared SUPPORTING. **NOT_QUALIFIED**. | CANONICAL_ARTIFACT: `givn/changes/fix-opencode-v2-attribution/specs/fragments/attribute-commits.feature` | Same feature is the cheaper home. No moved component, authority, deployment or persisted-data invariant. All three falsifications apply; glossary/risks clarify rather than duplicate ADR rationale. |
| V2 rewrite: link each shell command to its issuing session; drop stale same-session requests (D3 and refinement, added 2026-10-06) | Alternatives PASS (three unlinked fallbacks; three stale-request strategies); architectural impact FAIL (internal state of one adapter asset); durable consequence FAIL (in-memory, reversible in one change); lower-level artifact FAIL; existing-ADR check PASS (registers re-searched 2026-10-06); must-be-shared SUPPORTING. **NOT_QUALIFIED**. | CANONICAL_ARTIFACT: `givn/changes/fix-opencode-v2-attribution/design.md` | Cheaper home: the D3 entry in `design.md` plus `attribute-commits.feature`. All three falsifications apply. No MADR. |

Existing-check evidence: searched active/archived `docs/arc42/adr/` for OpenCode,
plugin, attribution and agent integration; read both registers and generated-agent
ownership (ADR-013) / preservation-oriented upgrade (ADR-023). Boundaries remain
unchanged. Canonical qualification guidance was read from the embedded Arc42
instruction as fallback, not copied here as a second procedure.

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `givn/changes/fix-opencode-v2-attribution/design.md` | Shared runtime resolver and canonical runner/cargo/nextest/coverage admission; concrete signal-aware supervisor and lease/group teardown/probes; actual tool-selected shell caveat; exact Step Definitions heading/table; operator evidence exception; corrected retained sync driver; exact terminal line and opposite-classification explanation; row-1 quality impact; fallback ADR summary. |
| `givn/changes/fix-opencode-v2-attribution/design.md` final clarity note | Added the prepared supervisor artifact and `GIVN_OPENCODE_SUPERVISOR` prerequisite beside the single-scenario command; runtime admission does not build it during a scenario. No approach change. |
| `givn/changes/fix-opencode-v2-attribution/arc42.md` | Row 1 No→Yes with both mapped chapters and original/final trace; deployment/risk summaries aligned; separate best-effort non-ADR candidate destination. Existing documentation-only completion marker retained, not treated as execution proof. |
| `docs/arc42/05-building-block-view.md` | Planned resolver/supervisor acceptance-block responsibilities, test-only existing-libc boundary. |
| `docs/arc42/07-deployment-view.md` | Corrected `.claude/settings.local.json` footprint; shared canonical-gate admission and owned-process supervision noted as pending tests. |
| `docs/arc42/08-crosscutting-concepts.md` | Shared resolver for canonical commands, feature/result/instrumentation preservation, infrastructure failure versus optional product fallback. |
| `docs/arc42/10-quality-requirements.md` | Pending actual-shell admission and controlled signal/exit/timeout evidence across canonical runners/coverage. No execution success claimed. |
| `docs/arc42/11-risks-and-technical-debt.md` | R-76 includes admission/cleanup risk and explicit signal-aware supervisor, escalation/reaping and user-service protection; proofs remain pending. |
| `givn/specs/upgrade-a-project/usecase.md` | Narrow pre-existing contradiction fix: repository/backend information availability precondition; fresh clean-state witness selected/read/checked in main flow. Refusal and guarantees unchanged. |
| `givn/changes/fix-opencode-v2-attribution/design-review.md` | Complete question, exploration, Persona, all-12 original/final architecture, hardening and validation record. |

Proposal and both complete delta features were read and preserved. No missing
scenario was found; all `@e2e` and `@wip` tags remain unchanged. Root README,
support-fragment inventory, other dirty architecture chapters, user prototypes,
legacy harness plugin, scripts/config/Nix and production source were not edited.
No tasks, generated integration, commit, installation or next-stage artifact was
created. Prototype hashes in design remain the supplied immutable values; this
hardening session did not execute filesystem hashing. The parent's separate
fresh reviewer confirmed both prototype hashes without changing the files.

## Re-review after operator constraint (2026-10-05)

The prior reviewed Nix/Fish re-entry design is superseded and not approved. The
operator required that Nix occur only in `flake.nix`. The revised design and
implementation now place runtime derivations inline in that file; shell scripts
consume exported paths and never invoke Nix or Fish. This is a changed
technology/setup decision, so the previous PASS was replaced with FAIL before
this re-review.

Re-review findings resolved by codebase changes:

- Updated `design.md` file layout, runtime provisioning, local run command,
  canonical runner validation, direct-runner admission, risk text, and PTY
  driver selection to match the flake-only boundary and Darwin/Linux `script`
  syntax.
- Inlined the former `runtimes.nix` expressions in `flake.nix`; removed the
  obsolete Fish resolver/wrapper files and added `resolve-runtime.sh` and
  `run-suite.sh`. No Nix invocation remains in runner/setup scripts.
- Updated Arc42 chapters 05, 07, 08, 10, 11 and the change assessment to describe
  flake-exported runtime paths and shell-only validation. Chapters 02/03/06/12
  remain compatible; 09 remains unaffected.
- Retained strictness proofs and runtime evidence as historical only; they do
  not prove the revised setup green. Setup remains unchecked, and full gates,
  V2 plugin activation, OS isolation and supervisor lifecycle matrix remain
  mandatory.

No missing behavior scenario was identified. The change remains CLI/terminal,
not a browser interface: the product interaction is an OpenCode CLI invocation
and its completed shell-tool result; no HTML/CSS/JS browser screen is introduced.
The existing user-visible Personas remain the developer, technical role and
Givn maintainer; no new actor or Persona is created. No ADR qualifies: the
flake-only setup decision is a reversible local development/test mechanism
owned by this design and `flake.nix`, not a durable component/authority boundary.

The revised plan is internally aligned after the Arc42 reassessment. Parent validation now confirms the source boundary, shell syntax, and lock-based validator against explicit paths already present in the Nix store. The configured verification suites, V2 activation, isolation, and full supervisor matrix remain incomplete; these are setup implementation work, not grounds to claim implementation GREEN. Re-review sign-off applies to the design decision only.

## Validation

This documentation session could not execute shell commands and initially
recorded that limitation as blocking. The following actual command evidence was
executed by the **parent**, supplied to this session on **2026-10-04**, in
`/Users/daniel.kressner/dev/PRIVATE/givn`. It resolves the historical blocker;
none of these results is represented as this subagent's execution.

| Parent command | Actual result / provenance |
|---|---|
| `givn instructions` | Exit 0, first parent-session action; selected `fix-opencode-v2-attribution` and design-review. |
| `givn status --change fix-opencode-v2-attribution --json` | Reported design and arc42-docs done, design-review pending, tasks pending. Historical state before final sign-off, not a claim of final status. |
| `givn instructions design-review --change fix-opencode-v2-attribution --json` | Exit 0; generates design-review.md, requires design/arc42-docs; full resolved policy read. |
| `givn instructions dialog` | Exit 0; full resolved dialog policy read. |
| `givn instructions arc42-docs --change fix-opencode-v2-attribution` | Exit 0 after hardening; full resolved output read. |
| `givn instructions design --change fix-opencode-v2-attribution` | Exit 0 after hardening; full resolved output read. |
| `givn instructions specs --change fix-opencode-v2-attribution` | Exit 0 after hardening; full resolved output read. |
| `givn instructions review` | Exit 0 after hardening; full resolved output read. |
| `givn lint --change fix-opencode-v2-attribution` | **Exit 2** after hardening; summary `givn lint: 2 file(s) checked, 18 finding(s)`. WIP findings/advisory overlaps, no parse failure. Allowed exit 0-or-2 condition satisfied. |
| `git diff --check` | **Exit 0**, no output. |

Parent's full fresh lint output:
`/Users/daniel.kressner/.local/share/opencode/shell/e98ffcd377525f953f1169c57a10e379f8a6c7d9/sh_1079410e2001tzrSB6xPWQ0dlU.out`.
The earlier design author's lint exit 2 retains its separate historical
provenance; the parent result above is the fresh hardening validation.

Separate fresh quality review, session `ses_ef86b9d55ffeSsDeiR8VlMmRHk`:
**APPROVED**, with no blockers, majors or minors. The reviewer executed
`givn instructions` first, made no edits and ran no tests. It confirmed
canonical-instruction consistency, coherent resolver/supervisor design,
headings/paths, all-12 architecture trace, use-case contract, all 18 scenario
identities, all 38 matrix entries, and both prototype hashes. Its sole clarity
suggestion was applied: the single-scenario command now names the prepared
artifact and `GIVN_OPENCODE_SUPERVISOR` prerequisite. Runtime validation does not
build the supervisor. No implementation or scope choice changed.

Read-back/grep checks validate text only: exact binding heading/paths, standalone
terminal value, original/final all-12 table, corrected settings path, planned
rather than executed proof wording, and no chapter box-drawing characters. These
are not tests of parser enforcement, runtime compatibility or signal cleanup.

All real-loader compatibility, prototype behavior, regeneration, strictness,
admission/isolation, interruption/timeout cleanup, instrumentation and terminal
evidence remain mandatory implementation/setup prerequisites before any scenario
completion. The characterization exception cannot fabricate RED or GREEN and
cannot replace any of those proofs.

## Sign-off

- [x] Every grilling branch walked, including Scope, Tech choices, Missing
  scenarios, Testability, Risk, E2E fidelity, Interaction coverage, Visual
  contract, opposite Interface classification, Persona disposition, Domain
  context/use-case rejection criteria, Decision ledger, Deferrals, ADR
  qualification, all-12 Arc42, Ubiquitous language and Security/errors/Git.
- [x] Supported-generation compatibility (D1) and supplied-behavior preservation
  (D2) resolve in the design; no seed-sourced question or binding choice reopened.
- [x] All operator questions resolved; sole exact answer/date/options recorded.
- [x] Design reflects the factual corrections and authorized exception.
- [x] Missing scenarios handled: N/A, none identified; no feature/tag changes.
- [x] Architecture documentation checked; original mismatch retained and final
  marker/design/chapter alignment corrected; no missing qualified MADR.
- [x] Required resolved CLI instructions executed/read by the parent; actual
  provenance recorded above, not attributed to this shell-less session.
- [x] Fresh parent `givn lint --change fix-opencode-v2-attribution` exits 2,
  satisfying the allowed 0-or-2 condition without parse failure.
- [x] Parent `git diff --check` exits 0; separate fresh quality reviewer approved
  hardening with no blockers, majors or minors.

**Planning-stage sign-off:** all questions and factual design blockers are
resolved, hardening is applied, mandatory CLI validation has actual parent
evidence, and the independent quality review approved the original planning hardening. The revised flake-only boundary was independently rechecked against the current design and affected chapters in this session; no separate reviewer tool was available for a new delegation. The initial
shell-execution limitation remains recorded as history, not an outstanding
blocker or a waived check. Real-runtime compatibility, strictness, isolation,
cleanup, coverage and terminal proofs remain **mandatory and UNEXECUTED** before
scenario completion. This PASS is design-review evidence only, not implemented
GREEN, a completion review, or archive proof. No tasks were authored here.

Historical marker of this 2026-10-04/05 section: FAIL (superseded by the
re-review sign-offs below).

## Re-review sign-off (2026-10-05)

The operator's constraint is binding: `flake.nix` is the only place that may
contain Nix expressions or invoke Nix. The current setup draft conforms: pinned
OpenCode/Node/module derivations and exports are in `flake.nix`; test validators
and wrappers use POSIX shell, consume exported paths, and never invoke Nix or
Fish. The Nix-store paths were already present before this edit; using them to
validate the new shell validator did not invoke Nix. Missing variables produce
a direct setup error.

Independent Arc42 reassessment found chapters 04, 05, 07, 08, 10 and 11 affected
by the test-runtime ownership change. Durable chapters and the change-level
twelve-row table have been updated. All other chapter classifications remain
consistent with the earlier review. No new domain scenario or ADR follows from
this test-infrastructure constraint.

The previous Fish-based PASS is historical and superseded. The current design
review passes for the revised flake-only Nix boundary. This does not complete
the setup task or prove the full suites, real V2 plugin activation, network
isolation, or complete supervisor lifecycle matrix.

Historical marker (2026-10-05): PASS for the flake-only boundary. Superseded by
the V2 rewrite (D3) on 2026-10-06; see the re-review below.

## Re-review after the V2 rewrite (2026-10-06)

The user asked for a rewrite of the OpenCode plugin for V2, including
subagents; the decision is recorded as D3 in `proposal.md` and `design.md`.
The production asset `assets/agent-integrations/opencode/givn-attribution.js`
is final for this plan and was read in full; this hardening did not edit it,
the user-owned `.opencode/plugins/*.js`, or `tasks.md`. A fresh-context grill
found about thirty passages still describing the earlier plan (prototype kept
byte-for-byte except `server()`). This section records the grilling outcome,
the user decisions, and the hardening applied.

Hardening ran `givn instructions design-review --change fix-opencode-v2-attribution`
and `givn instructions arc42-docs --change fix-opencode-v2-attribution` first
(both resolved and read; the first invocation also loaded the domain-modeling
pointer). Released 2.0.22 `@opencode/core` and `@opencode/client` tarballs in
the session scratchpad were read for the subagent observation point.

### Grilling log (2026-10-06)

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Missing scenarios / Risk | How does the V2 link forget a shell request that never reaches shell creation? | User: “Drop older same-session entries”. Two regular scenarios added. |
| 2 | Testability | The V2 subagent E2E scenario can pass before the change. Keep it? | User: “Keep it as written”. Recorded as characterization; observation point specified. |
| 3 | Testability / Scope | The configured-default scenario expects reasoning `low`, but real V2 `Model.Info` has no variant. | User: “Keep reasoning low”. Retained, documented spec/reality mismatch. |
| 4 | Scope | The release note still promised “without changing the supplied attribution behavior”. | User-supplied release note applied to `proposal.md`. |
| 5 | Scope / Architecture documentation | Stale “prototype except `server()`” passages, scenario/E2E counts, diagrams, matrix, invariant table, ADR evidence, old PASS marker. | Code-resolved; edits listed below. |
| 6 | Ubiquitous language | No term for the session that issued a command; Model identity order was stale. | Code-resolved: Issuing session added; Model identity amended. |
| 7 | Interaction coverage | The subagent E2E action had no fragment inventory row. | Code-resolved: row added (39 rows). |
| 8 | Decision ledger | D1–D3 and the refinement must resolve; D2 supersession must be explicit. | Code-resolved: ledger table in `design.md`. |
| 9 | ADR qualification | D3 had no verdict. | Code-resolved: `NOT_QUALIFIED`, `CANONICAL_ARTIFACT`. |
| 10 | Architecture documentation | Chapters 05, 06, 08, 10, 11, 12 described the old plan. | Code-resolved: updated; 02, 03, 04, 07, 09 checked unchanged. |
| 11 | Persona disposition | Maintainer disposition cited prototype logic and whole-source comparison. | Code-resolved: dispositions rewritten above. |
| 12 | Git hygiene | Junk files in revision `fe9cf077`. | Not handled here at the orchestrator's direction; the orchestrator owns it as a separate action. Not a design question. |
| 13 | Closure | Is the plan internally consistent after hardening? | Resolved by the closure state below. |

### Stale pending shell requests (2026-10-06 Q1)

- Question: “How should the plugin forget a pending shell request that never
  reached shell creation?” Context: a request vetoed by another plugin or
  aborted fires `execute.before` but neither `create.before` nor
  `execute.after` (released source: `execute.after` fires only when the tool
  ran, with status completed or error). Example: session A's `git commit` is
  aborted; session B's later `git commit` would be labeled with A's model.
- Options:
  1. **Expire by age.** Drop a pending request after a fixed time. Gain: also
     clears quiet sessions. Cost: a slow permission prompt expires a
     legitimate request, whose command falls back to the last observed model.
  2. **Drop older same-session entries (recommended).** On each shell-tool
     `execute.before`, drop that session's pending entries from an older
     message. Gain: no timer. Cost: a quiet session's stale entry stays until
     cleanup.
  3. **Accept.** Keep stale entries until unload. Gain: simplest. Cost: an
     aborted request can label any later identical command.
- User answer: **“Drop older same-session entries”**.
- Realization: entries removed in `execute.after` by `(sessionID, id)`;
  command read from the stored event at claim time; `pendingShells` lives in
  the `setup()` closure (per plugin instance). D3 refinement added to
  `proposal.md` and `design.md` with all options. Regular scenarios added with
  `@givn.added @wip`: “A blocked shell request does not label a later identical
  command from another session” and “A request from an older message of the
  same session is dropped”. Residual risk R-77 (quiet sessions). The asset
  triggers the drop only on shell-tool requests; a session that afterwards uses
  only other tools keeps its stale entry. This is stated in design and is
  covered by R-77.

### Subagent E2E scenario (2026-10-06 Q2)

- Question: the E2E “OpenCode V2 attributes a subagent's command to the
  subagent's model” passes with the prior plugin, because the child's own
  context event is the last model observed before the child's shell. Keep it,
  or change it?
- Options: keep it as written (chosen); the alternative was to change the
  scenario so the real-loader run discriminates. The alternative's exact
  wording was not relayed to this hardening session and is not reconstructed.
- User answer: **“Keep it as written”**.
- Realization in `design.md` (“Subagent E2E scenario: what it proves”): the
  E2E is characterization of real-loader subagent behavior and can pass before
  the change; the discriminating behavior (a main command after the child's
  context keeps the main model) is proven by the regular scenarios “A
  subagent's shell command is attributed to the subagent's own model” and
  “Identical shell commands from two OpenCode V2 sessions are attributed in the
  order issued”. Observation point justified from released 2.0.22 source: the
  child's shell result is in the child session, not the parent's NDJSON; the
  driver takes the child `sessionID` from the `subagent` tool result metadata,
  reads `GET /api/session/<child>/message` on the owned server, and parses the
  child's shell result, whose `OPENCODE_SESSION_ID` (set by the shell tool after
  the attribution hook) must be the child's. Fixture config gains the
  `attribution-subagent` agent (`mode: "subagent"`,
  `model: "givn-fixture/model-b#low"`) and the `subagent` permission; the
  provider twin gains a finite state returning a `subagent` tool call with
  `{agent, description, prompt}`.

### Configured-default reasoning (2026-10-06 Q3)

- Question: real V2 `Model.Info` from `ctx.model.default()` carries no
  variant, so against real V2 the default yields reasoning `unavailable`, but
  “OpenCode V2 uses its configured default when no model has been remembered”
  expects `low`. Change the scenario?
- Options: keep reasoning `low` (chosen); the alternative was to expect
  `unavailable`. The alternative's exact wording was not relayed to this
  session.
- User answer: **“Keep reasoning low”**.
- Realization: scenario text unchanged. Recorded as a retained known
  spec/reality mismatch in `design.md` (“Runtime semantics”, invariant table,
  risk 2), the glossary (Reasoning mode, Model identity), and R-74. The
  regular scenario characterizes the adapter's handling of a payload that
  carries a variant. The adapter reads only `response.data`.

### Release note (2026-10-06 Q4)

- Question: the release note promised compatibility “without changing the
  supplied attribution behavior”, which D3 contradicts.
- User answer (chosen text): “Givn initialization and regeneration provide
  commit attribution compatible with recent OpenCode V1 and OpenCode V2, and on
  V2 attribute each agent's shell commands, including subagents', to that
  agent's model.” Applied to `proposal.md`.

### Code-resolved findings (2026-10-06 Q5–Q11, Q13)

- Q5: `design.md` scope, decisions, responsibility table, runtime semantics,
  sequence diagram (`execute.before` → `create.before` → `execute.after`),
  twin states, fixture config, prototype-oracle scope (V1 and retained
  fallbacks only; no text equality), counts, invariant table (subagent link,
  identical commands, blocked request, older-message drop), traceability,
  coverage matrix (39 rows, new E2E title), visual evidence (eight titles),
  risks, arc42 handoff, glossary/cross-cutting text, ADR evidence. Old PASS and
  FAIL markers in this file turned into historical notes.
- Counts, recounted from both delta files: 23 scenario identities (20 added,
  3 modified); 8 E2E (26 executions) and 15 regular (31 executions); 57
  executions. `attribute-commits.feature` 15 identities,
  `enable-commit-attribution.feature` 8.
- Q6: glossary adds Issuing session (anti-terms: linked shell, current
  session); Model identity is the issuing session's remembered model (V1 shell
  hook; V2 tool-hook link), else last observed, else V2 configured default (no
  reasoning mode).
- Q7: `givn/specs/fragments/fragment.md` row “have a subagent execute an
  attributed shell command through OpenCode” → “OpenCode V2 attributes a
  subagent's command to the subagent's model”; design matrix updated.
- Q8: `design.md` decision ledger resolves D1, D2 (superseded in part by D3),
  D3, and the D3 refinement; none carried, none seed-sourced.
- Q9: D3 verdict `NOT_QUALIFIED` (alternatives PASS; architectural impact
  FAIL; durable consequence FAIL; lower-level artifact FAIL; existing-ADR check
  PASS); cheaper home: D3 entry in `design.md` plus `attribute-commits.feature`.
  Recorded in `design.md`, `arc42.md`, and the ADR summary table above.
- Q10: chapters 05, 06, 08, 10 (Q-94 scoped, Q-96 added, mapping row), 11
  (R-73 rewritten for unlinked commands; R-74, R-75 adjusted; R-77–R-81
  added), 12 updated; architecture index note updated; Mermaid only.
- Q11: Persona dispositions rewritten in the table above.
- Q13: closure state below.

### Use-case and Persona context (2026-10-06)

The support fragment still owns `attribute-commits`; the subagent behavior is
an extension of that capability, not a new use case or actor goal. Actors
remain Developer, Authoring agent, Project maintainer, and OpenCode acting for
the developer; subagents are OpenCode runtime sessions, not Gherkin actors or
Personas. No Persona was invented or promoted. Developer: gains
subagent-correct labels. Agent: unchanged. Maintainer: V1 semantics preserved;
V2 session link plus retained fallbacks; limits documented.

### Independent arc42 assessment (2026-10-06)

Formed from proposal, both delta files, and design before re-reading
`arc42.md`.

| # | Chapter | Independent | Marker | Match | Content check |
|---|---|---|---|---|---|
| 1 | 01/10 | Yes (quality via ch 10) | Yes | Yes | Q-96 issuing-session quality scenario added in ch 10; ch 01 unchanged. |
| 2 | 02 | Yes (earlier) | Yes | Yes | TC-29 support window and best-effort fallback still true; no D3 edit needed. |
| 3 | 03 | Yes (earlier) | Yes | Yes | OpenCode boundary row still true; no D3 edit needed. |
| 4 | 04 | Yes (2026-10-05 flake boundary) | Yes | Yes | Unaffected by D3. |
| 5 | 05 | Yes | Yes | Yes | Rewritten V2 `setup()` described; “only the delegation” removed. |
| 6 | 06 | Yes | Yes | Yes | V2 Mermaid sequence with tool hooks and drop; V1 unchanged. |
| 7 | 07 | Yes (earlier) | Yes | Yes | One canonical file; unaffected by D3. |
| 8 | 08 | Yes | Yes | Yes | Issuing-session link first; fallbacks for unlinked commands. |
| 9 | 09 | No | No | Yes | Three candidates `NOT_QUALIFIED`; no MADR. |
| 10 | 10 | Yes | Yes | Yes | Q-94 scoped to V1/retained fallbacks; Q-96; 23-identity note; 39 interactions. |
| 11 | 11 | Yes | Yes | Yes | R-73 unlinked only; R-77–R-81 new. |
| 12 | 12 | Yes | Yes | Yes | Issuing session; Model identity; Reasoning mode mismatch. |

All twelve chapter files exist with substantive content. A scan of
`docs/arc42/*.md` for box-drawing characters returned no match; new and changed
diagrams are Mermaid. No qualified ADR exists, so no chapter-09 MADR is
missing; retained consequences are in chapter 11. `arc42.md` keeps
`STATUS: DONE`, now consistent with the chapters.

### Validation (2026-10-06, executed by this hardening session)

| Command | Result |
|---|---|
| `givn instructions design-review --change fix-opencode-v2-attribution` | Resolved output read. |
| `givn instructions arc42-docs --change fix-opencode-v2-attribution` | Exit 0; resolved output read. |
| `givn lint --change fix-opencode-v2-attribution` | Exit 2: `2 file(s) checked, 23 finding(s)`; 23 WIP findings plus advisory overlaps; no parse error. |
| `git diff --check` | Exit 0, no output. |

### Known handoff item

`tasks.md` still describes the earlier plan (for example “adding only V2
`server()` delegation” and seven E2E / eleven regular identities). It was not
edited, as instructed: the task list is regenerated in the next step and must
cover all 23 scenario identities.

### Sign-off (2026-10-06)

- [x] All 2026-10-06 grilling questions answered by the user (Q1–Q4) or
  resolved from the codebase (Q5–Q11, Q13); Q12 is owned by the orchestrator as
  a separate Git-hygiene action, not a design question.
- [x] Decision ledger D1–D3 and the D3 refinement resolved in `design.md`.
- [x] Missing scenarios added with `@givn.added @wip`; no `@e2e` tag removed.
- [x] Architecture documentation independently reassessed; no mismatch.
- [x] Lint exit 2 (allowed); `git diff --check` exit 0.

DESIGN-REVIEW: PASS
