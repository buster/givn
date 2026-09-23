# Ideation session: local-only-integrations

## Phase

Handoff complete 2026-09-23.

## Progress

- 2026-09-23: Frame research complete. Read `givn status`, `givn spec tree`,
  README orientation, arc42 chapters 2/3/8/10/11/12, the initialize-project
  and upgrade-a-project specs, the repository-ownership and attribution
  fragments, `givn/config.yaml`, and the implementation seams
  (`src/skills/targets.rs`, `src/cli/init.rs`, `src/cli/git.rs`,
  `src/cli/upgrade.rs`, `src/cli/migration.rs`, `src/scm.rs`). Checked the
  working tree's own tracked generated files (`.agents/skills/gh-stack` is
  non-Givn content in a generated directory).
- 2026-09-23: Frame written to `frame.md`; unknowns parked in `questions.md`
  (Q1-Q14).
- 2026-09-23: User clarified the driving motivation: Givn stays closed source
  while it is developed; projects created with it are published as open
  source; published repositories must carry no Givn rules, and must build and
  test with no Givn installed; maintenance keeps using Givn locally. Frame
  amended: publishing intent in Problem, publishability in Vision and Scope,
  external contributor added as a stakeholder, gaps G10 (no publishability
  check)   added, terms recorded in `domain-terms.md`, questions Q15 (runnable/
  testable acceptance example) and Q16 (givn/ visibility) parked.
- 2026-09-23: Mixed-ownership fork answered in direction: the tracked
  `AGENTS.md` references a separate local file carrying Givn's rules; the
  tracked file stays checkable. Feasibility verified in the opencode rules
  documentation (imports are not automatic; a read instruction is the
  documented pattern) and the Claude Code settings documentation
  (`.claude/settings.local.json` is project-local, kept out of git, and
  outranks the shared settings file). Recorded in `questions.md` Q3.
- 2026-09-23: User asked whether opencode can include files alongside
  `AGENTS.md`. Verified: opencode combines `opencode.json` `instructions`
  entries with `AGENTS.md`; `AGENTS.md` does not parse imports. Probe in
  `/tmp/opencode/oc-instructions-probe`: a project `opencode.json` with an
  `instructions` entry pointing at a missing file loads without error
  (`opencode debug config` exits 0 and keeps the entry). Parked as Q17.
- 2026-09-23: User asked whether opencode has an `opencode.local.json`-style
  local config. Probed the installed version: root `opencode.local.json` is
  NOT read; `.opencode/opencode.json` IS read as project config and its
  `instructions` are honoured, so it can serve as an untracked local config
  when the project does not already own that file. Also fetched the Claude
  Code memory documentation: `CLAUDE.local.md` is the local instruction file
  and suppresses project `AGENTS.md` reading unless Project instructions is
  `claude-md-and-agents-md`; `@path` imports in AGENTS.md are expanded for
  Claude. Parked as Q18.
- 2026-09-23: User decided the `AGENTS.md` complexity away: keep the fenced
  block, never auto-commit the file, user owns tracking/merging; the strict
  local-only and clean-history rules cover every other generated rule.
  Q3 confirmed; Q17/Q18 dropped; Q19 parked (Claude hook belongs in
  `.claude/settings.local.json`); Q5 and Q6 moved to proposed. Frame carries
  the confirmed boundary.
- 2026-09-23: Frame confirmed by the user ("Lgtm"). Persona consolidation
  proposed: reuse the three project personas (`developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`) with topic stances, add
  one new topic persona for the external contributor (open-source consumer).
  The collaborator/fresh-clone user, the repository reviewer, and the
  non-Givn rule author fold into those as stances unless the user wants a
  separate segment.
- 2026-09-23: Persona set confirmed by the user. Four topic files written
  under `personas/`: three reuse records with topic stances and one new
  external-contributor persona. Frame boundary quality review run by a
  fresh-context subagent; all eight findings applied: spec citation fixed
  (`enable-commit-attribution`), gap G3 restated around
  `Backend::revision_contains`, gap G4 narrowed to the Claude hook, the
  rejected local-rules-file term marked, the repository self-hosting claim
  marked pending Q9, the reviewer stance added to the primary-user record,
  the Claude hook phrased as candidate pending Q19, and the technical role
  added to the frame stakeholder table.
- 2026-09-23: Event Storming draft written to `eventstorming.md`: chain
  A1-F5 covering init, local life, upgrade reconciliation, publication,
  fresh clone, and refusal/repair, plus a backend divergence table. Awaiting
  user confirmation.
- 2026-09-23: Term "de-version" rejected by the user and replaced with the
  local-only/versioned pair; the local-only transition is recorded with
  verified Git (`rm --cached`) and Fossil (`rm`/`forget`) mechanics. Event
  Storming confirmed by the user.
- 2026-09-23: Event Storming boundary quality review run (fresh context).
  Applied: ignore-rule partner for the transition (C6) and ignore changes in
  the managed revision (C8); explicit versioned-policy branch
  (policy-divergence table); `givn commit` stage-all exclusion for
  `AGENTS.md` and strict rules (B3, verified in `src/cli/commit.rs`);
  policy-change trigger (C0); `commit-msg` dispatcher and Claude hook home
  in A5; published-clone check and test evidence (D2, E2); `AGENTS.md`
  fence overwrite behavior (C9, F6); Fossil mechanics pinned to
  `fossil rm --soft`/`forget`; term refinements for strict rules,
  managed generated paths, and ignore set. Parked: history reachability
  boundary (Q2), drift detection (Q14).
- 2026-09-23: `givn commit` decision confirmed by the user: under local-only
  the stage-all path excludes `AGENTS.md` and every strict rule; the
  developer stages `AGENTS.md` with raw SCM when wanted.
- 2026-09-23: Use Case Definition written to `use-case-definitions.md` and
  boundary quality review run (fresh context). Applied: `author-change` and
  `maintain-project` restored to `+` summaries, `create-change` added,
  event ownership made single-owner (A1 and D2 belong to their fragments),
  E2/E3 declared external evidence, F1-F6 declared per-flow variants, D3
  declared a cross-cutting preservation rule, Q1/Q14 claims gated, the
  shared cleanliness report justified as a fragment with a promotion path,
  and persona IDs added to every definition.
- 2026-09-23: Vocabulary corrected by the user: the noun is `rules`, not
  integrations; all topic files swept (`Givn rule`, `strict rules`, `Rules
  policy`, fragment `rules-policy`). Setting shape confirmed: flat scalar
  `rules: local-only` (default) or `rules: versioned` in `givn/config.yaml`.
- 2026-09-23: `rules-policy` fragment elaborated to
  `use-cases/rules-policy.md`; two proposed resolutions await confirmation
  (absent key resolves to local-only for existing projects too; unrecognized
  value refused by name).
- 2026-09-23: Both resolved: an absent key resolves to local-only for new and
  existing projects, and an unrecognized value warns and continues as
  local-only. `initialize-project` elaborated to
  `use-cases/initialize-project.md`; ignore placement confirmed by the user
  as both destinations — a fenced tracked block plus a local exclude. A
  second Givn-authored project artifact is sanctioned: the ignore block,
  with surrounding user content untouched.
- 2026-09-23: User decision: `givn init` also reconciles tracked strict
  rules. The behavior is extracted into the new `local-only-transition`
  fragment (included by `initialize-project` and `upgrade-a-project`):
  file-precise removals from the current revision, files preserved, staged
  for the developer at init and folded into the managed revision at upgrade.
  Event Storming C4/C5 and the use-case definitions updated.
- 2026-09-23: History guarantee confirmed by the user: report occurrences in
  reachable history and never block; a separate, opt-in rewrite command
  (new use case `rewrite-history`, events G1-G4) performs the destructive
  rewrite, never automatically. Unreachable objects, remotes, and forks stay
  outside the guarantee. Mechanics parked as Q20; Fossil support may be a
  named refusal.
- 2026-09-23: Rewrite mechanism confirmed: Git uses `git-filter-repo` and
  refuses by name when absent; no fallback. Fossil refuses by name. The
  `rewrite-history` use case is elaborated at `use-cases/rewrite-history.md`;
  confirmation, preflight, and remote rules are proposed there.
- 2026-09-23: Rewrite confirmation confirmed: `--yes` proceeds, a terminal
  prompts after showing paths and revision count, piped input without
  `--yes` refuses. Recorded in `use-cases/rewrite-history.md`.
- 2026-09-23: Cleanliness report surface confirmed: init and upgrade include
  the report in their output; a dedicated on-demand check serves publication
  and CI. `upgrade-a-project`, `publication-cleanliness`, and
  `attribute-commits` elaborated to `use-cases/`. Parked: dedicated-check
  name (Q21).
- 2026-09-23: Dedicated-check exit contract confirmed: non-zero on any
  violation; init and upgrade report and exit 0.
- 2026-09-23: All use-case files written; elaboration boundary quality review
  launched (fresh context).
- 2026-09-23: Elaboration boundary review completed; all 15 findings applied:
  warn-not-refuse diagram, initialize-project includes the cleanliness
  report, minimal-guarantee staging wording, capability name
  `policy-resolution`, `AGENTS.md` never staged under either policy
  (eventstorming, use-case definitions, and upgrade file aligned), transition
  idempotence gated on Q14, upgrade trigger gated, publication-cleanliness
  definition updated, OOS exit-contract note removed, terms flipped to
  confirmed (Q1, Q2) and ignore set gated on Q19, rules-policy success
  guarantee scoped to reporting commands, "persist" reworded, frame question
  range and Next steps refreshed.
- 2026-09-23: Change cut revised after the user challenged the five seeds:
  two changes by deliverable guarantee — `local-only-rules` (setting through
  upgrade reconciliation as one unit) and `rewrite-history`. The four
  obsolete seeds were removed.

## Next steps

1. Run `/givn-propose` per seed, in the suggested order: `local-only-rules`,
   then `rewrite-history`.
2. Handoff never starts a change; the user starts each proposal when ready.

## Decisions already taken by the user

- `AGENTS.md` keeps Givn's fenced block and is the single exception to the
  strict rule: Givn never stages or commits it automatically; if the user
  checks it in that is accepted, and merging/diffing it is the user's
  responsibility. This exception does not extend to any other Givn rule.
- Givn is closed source during development; projects created with it are
  published as open source. Generated Givn rules must stay local.
- A published project must be runnable and testable with no Givn installed.
- Maintenance and further development run through Givn locally (it
  understands the config, the changes, and the markers).
- A Givn project should be able to keep generated Givn files out of the
  repository; the `givn/` directory itself stays as-is.
- The desired default for the setting is "do not check in generated Givn
  files".
- Installation and upgrade are expected to make the repository clean of the
  generated Givn rules when the setting says local-only; `AGENTS.md` is the
  exception the user granted on 2026-09-23.

## Completion (2026-09-23)

- Handoff records confirmed by the user: use-case-to-permanent mapping,
  persona promotion choices, and the change seeds.
- Change cut revised per the user: the original five per-use-case seeds
  collapsed to two deliverable-guarantee changes — `local-only-rules` (the
  core that makes `rules: local-only` true end to end) and `rewrite-history`
  (the destructive, tool-gated part). The five-seed split was rejected
  because the intermediate states ship a setting that upgrade does not honor.
- Topic document published at `docs/ideation/local-only-integrations.md`.
- New terms merged into `docs/arc42/12-glossary.md`: Givn rule, Rules policy,
  Strict rule, Local-only rule, Versioned rule, Local-only transition, Ignore
  block, Publishable project, Clean history, History rewrite.
- Persona promoted: `givn/personas/external-contributor--open-source-consumer.md`.
  Topic stances appended to `developer--primary-user`,
  `agent--technical-role`, and `givn-maintainer--owner`.
- Two change seeds written under `changes/` with the suggested order.
