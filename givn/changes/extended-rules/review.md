# Review: extended-rules

Reviewer runs in this report were made against HEAD `4692851b` (working tree
clean, `git status --porcelain` empty) with the checkout's binary
(`target/debug/givn 0.8.5`; the PATH install `givn 0.8.4` was additionally
checked and produced identical findings).

## Fabrication Audit (do this first)

### 0. `@e2e` tag integrity

Delta `.feature` history scan — every removed line containing `@e2e` in the
delta's own files:

```
$ for f in $(find givn/changes/extended-rules/specs -name "*.feature"); do
    git log -p --follow -- "$f" | grep -E "^-.*@e2e"; done
9 files match; EVERY match is the paired line of an `@... @e2e @wip` →
`@... @e2e` change (only `@wip` removed). Samples:
-  @givn.added @e2e @wip   /  +  @givn.added @e2e        (serve-extended-rules, instruction-extension ×2)
-  @givn.modified @e2e @wip / + @givn.modified @e2e      (initialize-a-project, generate-ideation-guidance,
    refresh-generated-guidance, match-guidance ×3, prose-consolidation, change-release-note,
    visual-design-contract)
0 genuine `@e2e` removals inside the delta's own files.
```

Cross-tree tag comparison — permanent same-title scenario vs the delta
`@givn.modified` replacement (script `/tmp/opencode/tags.py`):

```
== (a) e2e drops: permanent @e2e, delta same-title without @e2e ==
  DROP: review-architecture-documentation.feature::Generated review guidance stays aligned after skill sync  perm=['@e2e']
== (b) e2e gains: permanent without @e2e, delta with @e2e ==
(no output)
```

The drop is **finding F2** below.

### 1. Step definition scan (empty/no-op bodies)

```
$ rg -n -U --pcre2 'fn\s+\w+\s*\([^)]*\)\s*(->[^{]+)?\{\s*\}' tests/steps/
tests/steps/llm_attribution_e2e_steps.rs:364:fn make_executable(_path: &std::path::Path) {}
$ rg -n 'unimplemented!|todo!' tests/steps/
(no matches)
$ rg -n '^\s*(pass|return\s*;)\s*$' tests/steps/
(only guard-clause `return;` statements inside multi-line helper bodies, e.g.
 tests/steps/mod.rs:315-336 — verified by inspection, not step bodies)
0 empty step bodies found across 67 step-definition files.
```

The single empty function is the pre-existing non-Unix no-op
`#[cfg(not(unix))] fn make_executable` — a platform stub in a file this
change never touched (0 commits in the change range), not a step definition.

### 2. Commit cross-check

Every `- [x] COMMIT:` bullet in `tasks.md` was cross-checked against git
(script `/tmp/opencode/full-audit.py`):

```
COMMIT bullets: 82 | with fenced identifier: 82 | without: 0
distinct ids: 82 | duplicates: [] | unresolved: []
classification: {'production': 49, 'artifact': 33}
```

49/82 commits touch production surfaces (`src/`, `assets/`, `tests/`,
`docs/`); 33/82 diffs are delta-spec/tasks/evidence-only and are all listed
below with their disposition. No task lacks a commit; no identifier is reused;
every identifier resolves to a real commit.

Exceptions (33), each verified: spec-only diffs are `@wip`-removal commits of
the project's recorded assertion-migration pattern — every one has
`ORACLE_RED`/`BASELINE_GREEN` plus a controlled-omission calibration recorded
in its task section (script `/tmp/opencode/oracle.py`: 29/29 sections contain
the rationale and the recorded hash) — or an `@e2e` transcript commit whose
bindings and GREEN run were delivered by the capability's step-file commit:

| Commit | Scenario | Pattern | Disposition |
|---|---|---|---|
| `dcb785cb` | Synchronized authoring guidance carries the classification, driver, and persona rules | assertion migration | ORACLE_RED, calibrated; served assets delivered by `7c842b31` |
| `87d1767e`, `25af5ee8`, `5ab8be8c`, `2c14c968`, `8d1e182f`, `51132a38`, `da3bd6d7`, `542c4cdc`, `a32a03ac`, `51a26d58`, `4376d379`, `6e250dde`, `48ad5465`, `d36dd1ef`, `da56b3c2`, `9befc99b`, `34723a83`, `ceac6598`, `f2ad52ff`, `f594f6ff`, `59ff6a90` | The ideation guidance/canvas/handoff scenario family (21 titles: personas binding, topic state, seed schema, term schema, canonical definition, authoring requirements, persona criteria, open-question qualifier, live-question standard, decided directions, frame confirmation, handoff seeds, canvas kinds, aggregates, event-storming review, verified names, naming question, term check, verified-name gate, persona schema, generated-file direction) | assertion migration | each ORACLE_RED/BASELINE_GREEN with calibration; `assets/instructions/ideation.md`/`explore.md` and shared step bindings delivered by `ec188787`, `ad36aa18`, `0e416600`, `d78e99f4`, `af94bf8f`, `2d389d11`, `2aa7a2c4` |
| `6ae4cdd0` | The review skill checks that the ubiquitous language is followed | assertion migration | ORACLE_RED, calibrated; served review content pre-existing |
| `b61058d3` | Lifecycle guidance describes the domain, not the givn change | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `9311f7a2` | The ubiquitous language is recorded in the project's arc42 glossary | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `b237a687` | The skill requires the arc42 glossary for domain modeling | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `9d482d40` | The domain-modeling guidance carries the term introduction test | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `9fa8c37f` | The challenge bullet hands the name choice to the user | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `d0f574cb` | The challenge bullet shows a naming question with candidates and a collision | assertion migration | ORACLE_RED/BASELINE_GREEN, calibrated |
| `b04de934` | Synchronized guidance defines the contract approval and capture mechanics | e2e transcript | real-CLI steps in `prose_consolidation_e2e_steps.rs`/`.rs`; transcript committed |
| `7a6e2e37` | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | e2e transcript | real-CLI steps; transcript committed |
| `e2415c97` | A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed | e2e transcript | real-CLI steps in `template_conditional_rendering_e2e_steps.rs`/`_steps.rs`; transcript committed |
| `f179047a` | An agent receives the extended rules through the Givn CLI | e2e transcript | real-CLI steps in `serve_extended_rules_steps.rs`/`instruction_integrity_e2e_steps.rs` (no new binding needed); transcript committed |

No commit is unexplained; the strict reading of this step's "spec-only diff =
fabricated task" is addressed by the recorded assertion-migration pattern
(same construction the archived precedent `make-user-questions-decision-ready`
dispositioned under "test/evidence only (see disposition)").

### 3. Promised-architecture cross-check

Every path design.md's `## Exact file changes` promises exists (script
`/tmp/opencode/promised.sh`, 66 checks including the 15 skill templates, 15
commands, the 10 new instruction assets, the step-file set, and the two arc42
chapters):

```
== existing: 66 | missing: 0
```

Note: the deterministic `check_promised_files_exist` hook skips this
verification because it looks for the heading `## Step Definitions` while
design.md carries `## Step definitions` (case difference); the check was
therefore made manually and is complete above.

### 4. Strict-mode proof

Present in `tasks.md`'s setup task (lines 230-271): the throwaway probe's
undefined step → `EXIT=101` "Step doesn't match any function"; the
`unimplemented!()` stub → `EXIT=101` panic; probes removed; 68 units
re-listed. Confirmed in code (`tests/features_runner.rs`):

```
$ sed -n '55p;64,65p;151p' tests/features_runner.rs
        .fail_on_skipped()
    if stats.failed > 0 || stats.skipped > 0 {
        std::process::exit(1);
        .fail_on_skipped()
```

### 5. Downgraded e2e scenario scan

All 12 delta `@e2e` scenarios assert on observable output of the real CLI
interface (stdout/stderr, exit codes, or files the real command wrote), never
on repository state alone. Interface is terminal (CLI text output), so no
browser driver applies.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| An agent receives the extended rules through the Givn CLI | yes — real `givn instructions proposal --change …` stdout: exit 0, built-in rules, marked `## Project extension` section | N/A (terminal) | no |
| A developer records an extension and it governs the next guidance request | yes — real CLI stdout after the fixture writes `extensions:`; built-in prefix byte-compared | N/A | no |
| An upgrade preserves the extension and the next request serves it | yes — real `givn upgrade` revision plus real `givn instructions` stdout | N/A | no |
| Consumer runs init non-interactively and gets opencode skills | yes — real `givn init --no-addons --agents opencode` report + written files + preserved AGENTS.md | N/A | no |
| A consumer synchronizes skills and every agent target receives the ideation facilitation skill | yes — real `givn skills sync` stdout + served ideation content + generated direction | N/A | no |
| Synchronized guidance explains how to resolve the deterministic gates | yes — real sync stdout + served instruction content | N/A | no |
| A consumer synchronizes skills and the generated guidance omits disabled-addon sections | yes — real sync after real addon state + served/generated content | N/A | no |
| A consumer enables the addons a generated section needs and the guidance gains it | yes — real `givn addons enable` + sync + served/generated content | N/A | no |
| A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed | yes — real addon toggle + sync + file removal + served content (Scenario Outline, 2 examples) | N/A | no |
| Generated guidance points to the canonical E2E policy | yes — real sync + served specs instruction + delivered guidance | N/A | no |
| The generated implementation guidance shares the release note | yes — real sync + served tasks instruction + AGENTS.md | N/A | no |
| Synchronized guidance defines the contract approval and capture mechanics | yes — real sync + served design-review/steps instructions | N/A | no |

Opposite classification tested and rejected: a rendered page is a Web UI;
nothing here renders HTML, CSS, or JavaScript in a browser — the product
prints text/JSON to a terminal and writes Markdown, so a browser driver would
be wrong and no HTTP/fetch substitution exists (steps spawn
`CARGO_BIN_EXE_givn`).

### 6. E2e scope check

One `@e2e` scenario per distinct happy-path action; variants/error cases stay
regular.

| Capability | @e2e scenario count (delta) | Distinct actions? | Over-produced? |
|---|---|---|---|
| serve-extended-rules | 1 | yes — request the extended instruction | no |
| instruction-extension | 2 | yes — record (row 2 of the matrix), upgrade (row 3) | no |
| initialize-a-project | 1 in delta (noninteractive init; interactive and resync are retained permanent e2e) | yes — distinct actions | no |
| generate-ideation-guidance | 1 | yes — sync delivers the ideation guidance | no |
| refresh-generated-guidance | 1 | yes — sync explains gate resolution | no |
| match-guidance-to-enabled-addons | 3 | yes — one per addon-state transition (disabled baseline, enable, disable); preserved 1:1 from the permanent e2e set | no |
| prose-consolidation | 1 | yes — sync + E2E policy | no |
| change-release-note | 1 | yes — sync + release note | no |
| visual-design-contract | 1 | yes — sync + capture mechanics | no |

### 7. Duplicate-implementation check

`verify.e2e_command` read literally from `givn/commands.yaml`:

```
$ grep -n "command:" givn/commands.yaml
3:  command: "./run-tests.sh"
4:  e2e_command: "./run-tests.sh --e2e"
```

`./run-tests.sh --e2e` runs `GIVN_CUCUMBER_TAGS='@e2e' cargo nextest run
--test features_runner` (tag filter in the runner itself, `run-tests.sh`
lines 76-78). Tree search (`git status` clean; `find tests`):

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| serve-extended-rules | `tests/steps/serve_extended_rules_steps.rs` (shared real-CLI steps) + pre-existing `instruction_integrity_e2e_steps.rs` Given; `serve_extended_rules_e2e_steps.rs` is a registered doc-only module (0 bindings, see F4) | no — no parallel/untracked binding exists; `tests/features_runner.rs` is the only runner | N/A |
| instruction-extension | `instruction_extension_e2e_steps.rs` (+ `run_upgrade_steps.rs`, shared served step) | no | N/A |
| all other delta e2e | existing per-capability `*_e2e_steps.rs` / `*_steps.rs` files named in design.md | no | N/A |

### 8. Local environment check

design.md §Local runnability & digital twins declares no service, no
docker-compose, no isolated network, no digital twin — a CLI plus the
compiled binary and temporary project directories. Confirmed:

```
$ cargo run --locked --quiet -- instructions        → guidance printed        EXIT=0
$ GIVN_FEATURES=givn/changes/extended-rules/specs ./run-tests.sh
  Summary [5.914s] 79 tests run: 79 passed, 0 skipped                        EXIT=0
```

### 9. E2e isolation check

Values read literally from `givn/commands.yaml` (lines 3-4) — not identical.
No archive receipt exists yet for this change (unproven candidate), so the
measured delta scopes are given; the archive receipt is the full-suite
authority and will record the merged per-scope counts.

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | measured delta: 79 nextest units / 67 `not @e2e` / 80 cucumber scenarios (one Outline has two examples); receipt pending | measured delta: 12; receipt pending | yes — 12 < 67 < 79; the "all scenarios @e2e" exception does not apply |

### 10. Design.md conformance check

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| `verify.e2e_command` | `./run-tests.sh --e2e` | same (commands.yaml line 4) | yes | N/A |
| Runner / framework | nextest via `./run-tests.sh`; cucumber 0.23.0, gherkin 0.16 per Cargo.lock; strict `.fail_on_skipped()` | same (verified in source/Cargo.lock) | yes | N/A |
| Production seams | `render_conditionals` exposed in `src/skills/render.rs`; render+append in `src/cli/instructions.rs`; new `src/config/extensions.rs` (`load` line 18, `append` line 70); `src/config/overlay.rs` lenient entry list (line 385); `src/cli/project_hint.rs` marker extraction | all present at the named paths (instructions.rs lines 51-52/92-93 render then append, line 121 load; project_hint.rs consts lines 14-15) | yes | N/A |
| Asset rewrite set | 15 `assets/skills/*/SKILL.md.tmpl`, 15 `assets/commands/*.md`, new `assets/instructions/{archive,explore,ideation,steps,reverse-engineer,characterize,domain-modeling,dev-principles,grill-me,grill-with-docs}.md`, `review/design-review/tasks`, `tasks-reverse`, `agents-project-hint.md`, `config.yaml.tmpl` | all 66 promised paths exist; delta scenarios verify the served content and the no-copy bound | yes | N/A |
| Step-definition file layout | `serve-extended-rules` → `serve_extended_rules_steps.rs` **and** `serve_extended_rules_e2e_steps.rs` ("The serving `@e2e` steps (real CLI)"); `instruction-extension` → both extension files | extension files match; `serve_extended_rules_e2e_steps.rs` is a 5-line doc-only skeleton with 0 bindings — the serving e2e scenario binds through the shared real-CLI steps in `serve_extended_rules_steps.rs` + pre-existing steps; driving mechanism is unchanged real CLI, so no downgrade, but the design cell is stale | no | **F4** — design.md not updated |
| `## Internal Primitives` | `extensions::{load, append}`, `render_conditionals`, `resident_rules_section(asset: &str)`, overlay entry-list wrapper | first three exist (`load`, `append`, `render_conditionals` exact); project_hint's private fn is `resident_rules(tasks: &str)`, not `resident_rules_section(asset: &str)` | no (name/signature) | **F4** — design.md not updated |
| arc42 chapters 08/10 | wording per `arc42.md` | present (08 §8.3a/8.3b/8.7; 10 Q-97/§10.3) | yes | N/A |

### 11. Interaction coverage verification

The inventories live in the use-case documents (this change carries no
separate inventory comments in `.feature` files): four
`specs/*/usecase.md` `## Interactions` tables + `specs/fragments/fragment.md`
= 13 entries, matched against design.md's Interaction Coverage Matrix and the
built step definitions. All delta and permanent e2e scenarios named below
executed green at HEAD (no failures involve them).

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| obtain-effective-guidance / serve-extended-rules: request effective instructions by id (artifact, policy, JSON, change-context variants) | yes | An agent receives the extended rules through the Givn CLI (delta) | real CLI `givn instructions proposal --change …` subprocess | `serve_extended_rules_steps.rs::request_instruction` → `common::run_givn` | yes |
| instruction-extension: record an extension for an instruction | yes | A developer records an extension and it governs the next guidance request (delta) | fixture writes `extensions:`; real `givn instructions` subprocess | `instruction_extension_e2e_steps.rs::developer_records_proposal_rule` (write + run) | yes |
| instruction-extension: upgrade a project that holds an extension | yes | An upgrade preserves the extension and the next request serves it (delta) | real `givn upgrade` then `givn instructions` subprocesses; inspect revision + served content | `consumer_upgrades_the_project` (`run_givn(["upgrade"])`) + `run_upgrade_steps.rs` + served-content assertion | yes |
| initialize-a-project: select agent targets in the interactive terminal | yes | Consumer selects agent targets during interactive init (permanent, green) | real `givn init` under a PTY (`script -qec`); arrow/space/enter keys | `initialize_a_project_e2e_steps.rs::consumer_selects_targets_in_terminal` → `common::run_init_in_terminal` → `script -qec "<bin> init"` | yes |
| initialize-a-project: initialize with explicit noninteractive selection | yes | Consumer runs init non-interactively and gets opencode skills (delta) | real `givn init --no-addons --agents opencode` subprocess | same (delta e2e ran green) | yes |
| initialize-a-project: resynchronize selected targets | yes | Consumer re-syncs skills after init and the selection is honoured (permanent, green) | real `givn skills sync` subprocess; report + persisted targets + preserved text | `run_givn(["skills","sync"])` + file assertions | yes |
| apply-a-managed-upgrade: plan a release migration | yes | A clean 0.2.0 project receives a reviewable aggregate migration plan (permanent Outline, green) | real `givn upgrade` on a committed 0.2.0 fixture | `run_upgrade_steps.rs` / `scm_backend_selection_e2e_steps.rs` real subprocess | yes |
| apply-a-managed-upgrade: run a managed upgrade | yes | A clean project without migration requirements receives one managed upgrade commit (permanent, green) | real `givn upgrade` subprocess | same | yes |
| apply-a-managed-upgrade: upgrade a Fossil project | yes | A clean Fossil project receives the same managed upgrade revision shape (permanent, green) | real `givn upgrade` in a Fossil checkout | same | yes |
| apply-a-managed-upgrade: invoke a retired engine command | yes | A consumer invoking a retired engine command is told it does not exist (permanent, green) | real subprocess; non-zero named refusal | same | yes |
| apply-a-managed-upgrade: archive a maintenance migration | yes | A completed maintenance migration can be archived without a product specification (permanent, green) | real `givn archive` subprocess | same | yes |
| instruction-integrity (support fragment): resolve an instruction and expose the machine contract | yes | An agent requests machine-readable artifact guidance (permanent, green) | real `givn instructions specs --change … --json` subprocess | `instruction_integrity_e2e_steps.rs` real subprocess | yes |
| instruction-integrity (support fragment): request normal session-start instructions | yes | Author starts a session with the universal dialog obligation (permanent, green) | real `givn instructions` subprocess without an id | `instruction_integrity_e2e_steps.rs` real subprocess | yes |

### Fabrication-audit findings (instruction steps 0-11)

- **F1 — `givn check review` fails on the revision-identifier recording
  format.** `givn check review --change extended-rules` exits 1 with 65
  `[integrity]` findings: each repeated `- [x] COMMIT:` plan sentence is read
  as a duplicate revision identifier, because the unique identifier is
  recorded in the fenced block below the bullet. The check extracts the
  identifier from the bullet itself (`src/integrity_hook.rs:459-470`,
  split on `--`). The underlying contract holds (82/82 identifiers unique and
  resolvable, section 2); the representation does not satisfy the gate.
  Evidence: `target/debug/givn check review --change extended-rules` → 65
  findings, `EXIT=1`; PATH `givn 0.8.4` identical. Diagnostic: in a scratch
  copy with the COMMIT bullets mechanically re-recorded as
  `- [x] COMMIT: <id> -- <text>` plus this review.md, the same check reports
  `✓ integrity passed / ✓ run declaration passed / ✓ overlap dispositions
  passed` and then F5's routing findings. Fix required: re-record the
  identifiers inline in `tasks.md` (or change the check) — outside this
  review's permitted edits.
- **F2 — planned `@e2e` coverage removed on a delta replacement (step 0).**
  The permanent `givn/specs/fragments/review-architecture-documentation.feature`
  scenario "Generated review guidance stays aligned after skill sync" is
  `@e2e`; its same-title delta replacement
  (`givn/changes/extended-rules/specs/fragments/review-architecture-documentation.feature`,
  introduced by repair commit `9ca8c848`) carries no `@e2e` tag, so the merge
  drops the interaction from the e2e scope and no transcript is required.
  The replacement still drives the real CLI, so the driver is not weaker —
  but tag removal is a fabrication-audit finding; the delta inventory even
  records it as 0 e2e. Fix: restore `@e2e` on the delta scenario and commit
  its transcript at the design's evidence path (or update design.md and
  re-run design-review) — a spec/evidence change, outside this review's
  permitted edits.
- **F3 — one permanent `@e2e` scenario is left failing with no delta
  replacement.** Measured at HEAD: permanent full scope `606 run: 562 passed,
  44 failed`; non-`@e2e` `516 run: 479 passed, 37 failed` (all 37 carry a
  same-title delta `@givn.modified`; claim verified); `@e2e` scope `90 run:
  83 passed, 7 failed`. Six of the seven e2e failures are reconciled by
  delta replacements (five `@e2e`, one via F2); the seventh —
  `givn/specs/fragments/guide-domain-modeling.feature::An agent synchronizes
  skills and the domain skill is routed` — has no same-title delta scenario,
  so the merged permanent suite stays red after archive. Failure:
  `the generated skill instructs maintaining the project's arc42 glossary`
  (the content moved to the served instruction). Fix: add the same-title
  `@givn.modified @e2e` delta scenario re-pointing the assertions to the
  served instruction and commit its transcript — a spec/evidence change,
  outside this review's permitted edits. Classified bucket 2 (missing test
  coverage) below.
- **F4 — design.md cells stale vs as-built (step 10).** (a)
  `tests/steps/serve_extended_rules_e2e_steps.rs` is described as "The serving
  `@e2e` steps (real CLI)" but contains 0 bindings (doc-only skeleton); the
  serving e2e scenario's steps are shared in
  `serve_extended_rules_steps.rs` + pre-existing files. (b) §Internal
  Primitives names `resident_rules_section(asset: &str) -> Result<&str>`, but
  the as-built private fn is `resident_rules(tasks: &str) -> Result<&str>`.
  Both are silent deviations from reviewed design text; the fix is an
  explicit design.md update plus a design-review re-run (or conforming the
  code), outside this review's permitted edits.
- **F5 — routing consistency fails (surfaces once F1 is repaired).** In the
  scratch diagnostic (F1's format repaired, this review.md present) the gate
  continues after the overlap gate and reports:

  ```
  error: capability routing decisions do not match the built delta:
    delta capability 'guide-domain-modeling' is not declared in the Capability Routing table
    decision NEW in extend-an-instruction: usecase.md failed to parse: section '## Rules' must contain one rule per bullet
    delta capability 'review-architecture-documentation' is not declared in the Capability Routing table
    decision NEW in obtain-effective-guidance: usecase.md failed to parse: section '## Preconditions' must contain one precondition per bullet
  ```

  Causes: (a) the two repair-added capabilities (`guide-domain-modeling`,
  `review-architecture-documentation`) have delta scenarios but no row in
  `proposal.md`'s Capability Routing table; (b) the check's list parser
  (`src/gherkin/usecase_md.rs:322`) requires one bullet per line, and
  `specs/obtain-effective-guidance/usecase.md` `## Preconditions` and
  `specs/extend-an-instruction/usecase.md` `## Rules` carry wrapped bullets.
  Fix: add the two routing rows and un-wrap the parse-failing bullets —
  artifact edits outside this review's permitted edits.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `specs/*/usecase.md` guarantees are realized by the served-and-extended path (`serve-extended-rules`, `instruction-extension`), the direction-only assets and reconciled block (`initialize-project`), the unchanged allowlist (`upgrade-a-project`), and the re-pointed assertion migrations; delta scenarios for each rule are green | yes |
| Capability ownership and typed relationships preserved | capability tables unchanged: `obtain-effective-guidance` → serve-extended-rules; `extend-an-instruction` → instruction-extension; fragment `support` keeps the touched capabilities; no `Extends` added | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` keeps extension ownership across sync/upgrade and its target choices; `agent--technical-role` reaches deterministic served content on both paths; `givn-maintainer--owner` keeps one canonical asset per rule (no-copy step + served-content scenarios) | yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay `the user`/`the agent`/`the developer`/`the consumer`; Personas appear only as review lenses | yes |
| Migration ledger and coverage evidence complete, if applicable | no migration ledger applies (assets, CLI serve path, acceptance suite); coverage receipt arrives with the archive | yes |

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Deferred mandatory checks

None deferred. Strictness proofs, scenario executions, the 12 E2E
transcripts, and the Persona/visual judgement were performed or recorded, not
waived. No `DEFERRAL:` line is present.

## README impact

README-IMPACT: updated - What it is

Verified: commit `4692851b` added the "Rules are served, not copied" bullet to
the README's "What it is" section (served-rules model and the `extensions:`
map); `givn readme check` → `README check: OK`.

## Visual Review

Interface: terminal (design.md `## Visual Design Contract`, `Interface:
terminal`). Opposite classification tested: a rendered page is a Web UI and
does not hold — nothing renders HTML, CSS, or JavaScript in a browser; the
evidence is CLI text/JSON output and written Markdown. Evidence kind: one
committed final transcript per delta `@e2e` scenario (12/12 present,
non-empty, each records scenario title, exact argv, cwd disposition, exit
code, and raw stdout/stderr).

| @e2e Scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| An agent receives the extended rules through the Givn CLI | terminal | `evidence/visual/an-agent-receives-the-extended-rules-through-the-givn-cli/transcript.txt` (4700 B) | yes |
| A developer records an extension and it governs the next guidance request | terminal | `…/a-developer-records-an-extension-and-it-governs-the-next-guidance-request/transcript.txt` (5626 B) | yes |
| An upgrade preserves the extension and the next request serves it | terminal | `…/an-upgrade-preserves-the-extension-and-the-next-request-serves-it/transcript.txt` (12517 B) | yes |
| Consumer runs init non-interactively and gets opencode skills | terminal | `…/consumer-runs-init-non-interactively-and-gets-opencode-skills/transcript.txt` (5435 B) | yes |
| A consumer synchronizes skills and every agent target receives the ideation facilitation skill | terminal | `…/a-consumer-synchronizes-skills-and-every-agent-target-receives-the-ideation-facilitation-skill/transcript.txt` (4243 B) | yes |
| Synchronized guidance explains how to resolve the deterministic gates | terminal | `…/synchronized-guidance-explains-how-to-resolve-the-deterministic-gates/transcript.txt` (4562 B) | yes |
| A consumer synchronizes skills and the generated guidance omits disabled-addon sections | terminal | `…/a-consumer-synchronizes-skills-and-the-generated-guidance-omits-disabled-addon-sections/transcript.txt` (19359 B) | yes |
| A consumer enables the addons a generated section needs and the guidance gains it | terminal | `…/a-consumer-enables-the-addons-a-generated-section-needs-and-the-guidance-gains-it/transcript.txt` (31698 B) | yes |
| A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed | terminal | `…/a-consumer-disables-an-addon-the-domain-modeling-skill-needs-and-the-generated-domain-skill-is-removed/transcript.txt` (40084 B) | yes |
| Generated guidance points to the canonical E2E policy | terminal | `…/generated-guidance-points-to-the-canonical-e2e-policy/transcript.txt` (5178 B) | yes |
| The generated implementation guidance shares the release note | terminal | `…/the-generated-implementation-guidance-shares-the-release-note/transcript.txt` (6838 B) | yes |
| Synchronized guidance defines the contract approval and capture mechanics | terminal | `…/synchronized-guidance-defines-the-contract-approval-and-capture-mechanics/transcript.txt` (5389 B) | yes |

| confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | keep rules that survive sync/upgrade; keep targets and human text | fresh-context lens: PASS (12/12 transcripts show extension recording/serving, upgrade survival, init block rendering, target re-sync); 4 non-blocking observations recorded by that run, none naming a contract rule or a failed goal | non-blocking; no change required |
| agent--technical-role | deterministic, complete content on both paths; invokable directions | fresh-context lens: PASS (12/12; determinism, direction invokability, and exit codes shown; `--json` parity and refusal paths are graded by non-e2e scenarios under the canonical variant rule) | non-blocking; no change required |
| givn-maintainer--owner | one canonical asset per rule; honest evidence | fresh-context lens: PASS (12/12; the earlier withheld findings are repaired — six missing assets created, eleven files given directions, template copies removed, tasks-reverse aligned; the no-copy bound is the design's representative-marker list) | non-blocking; no change required |

**Baseline (terminal):** the transcripts show labeled command output, real
exit codes, no internal-identifier-only labels, no duplicated visible
content, the configured language, and designed refusal messages (unknown ids,
missing change context); motion/focus/viewport rules are nonapparent to
static terminal text by construction. The baseline holds for the applicable
text output.

VISUAL-REVIEW: PASS

## Retirements

N/A — this change retires nothing; the delta carries 0 `@givn.removed`
records and no `@givn.retired` tag.

## Arc42 implementation conformance

Independently re-derived from the completed change; the durable chapters were
read at HEAD. Slug mapping verified (every chapter named below maps to a real
`docs/arc42/<slug>.md` file; the deterministic hook also passed in the
scratch diagnostic: `✓ integrity passed`).

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 05 building-block-view: `config::extensions` added; `cli::instructions` renders+appends; `cli::skills` refreshes resident rules | `docs/arc42/05-building-block-view.md` (extensions row, skills row) | Yes — 5.3 gains these | §The serving path, §The project-start block, §Exact file changes | serve-extended-rules / initialize-a-project sections | `src/config/extensions.rs:18,70`; `src/cli/instructions.rs:51-52,92-93,121`; `src/cli/project_hint.rs:21,40` | match |
| 06 runtime-view: the request gains conditional rendering + extension append; step/reverse ids resolve via the non-artifact fallback | `docs/arc42/06-runtime-view.md` (serve path) | Yes — 6.3 records it | §The serving path | serve-extended-rules sections | `instructions.rs` render-then-append lines above | match |
| 08 §8.3a: generated files carry a direction; extended rule = built-in + extension; never overridden | `docs/arc42/08-crosscutting-concepts.md` §8.3a | Yes | §Generated guidance carries a direction; D2/D3 | prose-consolidation/instruction-integrity/refresh tasks | direction-only assets; served-content scenarios; no-copy step | match |
| 08 §8.3b: JSON `instruction` carries the rendered-and-extended text | §8.3b | Yes | §The serving path | serve-extended-rules JSON scenario | `instructions.rs` JSON branch; green JSON parity scenario | match |
| 08 §8.7: direction-only form, id-resolution two-class rule, reconciliation, `extensions:` warn-and-drop, replacement interplay, resident rules | §8.7 | Yes | §The serving path; two-class integrity decision | instruction-extension + initialize-project + instruction-integrity tasks | `config/extensions.rs`; `resolve_instruction` unchanged; block scenarios | match |
| 10 Q-97 + §10.3: id/context resolution clause and its verification signal | `docs/arc42/10-quality-requirements.md` | Yes | two-class integrity decision | direction-context scenario | green "Every generated direction is invokable in its documented context" | match |
| 11 R-79 (textual contradiction tolerated, visible) and R-80 (committed direction-only file without Givn) | `docs/arc42/11-risks-and-technical-debt.md` | Yes | D2/R-79–R-80 coverage | contradiction + replacement + upgrade scenarios | green delta scenarios | match |
| 12 glossary `Extended rule` predates the change | `docs/arc42/12-glossary.md` | No (with reconciliation note) | §Arc42 impact | n/a | term row present; change coins no new term | match |

- [x] Every chapter marked affected by either assessment was opened and checked.
- [x] Every Arc42 fact relevant to this change is represented in `design.md` and mapped to tasks.
- [x] The completed implementation follows those facts; evidence names files, commands, and results.
- [x] Architecture changes discovered in the completed implementation are recorded in the durable chapters.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** no measured receipt exists yet for this change; the
archive measures it against the committed candidate and records it in
`givn/archive/<id>/verification.json` and the README.

```
unproven candidate — no receipt yet. Coverage is classified from static
inspection below; the archive receipt will carry the measured line/branch
percentages. No instrumented pass was run in review.
```

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/config/extensions.rs` (`load`, `append`, warn-and-drop branches) | new module | covered by inspection | exercised by the instruction-extension scenarios (record/change/remove, unknown id, malformed entry, empty list, whitespace-only, repeated key last-wins, policy+step ids, upgrade survival) — all green |
| `src/skills/render.rs::render_conditionals` | exposed fn | covered by inspection | exercised by addon-condition and served-content scenarios (`template_conditional_rendering_steps.rs`, `serve_extended_rules_steps.rs`) |
| `src/cli/instructions.rs` render+append (text and JSON paths) | amended | covered by inspection | exercised by the serve-extended-rules feature and every served-instruction assertion |
| `src/cli/project_hint.rs` marker extraction + `{{RESIDENT_RULES}}` | amended | covered by inspection | exercised by "The project-start block renders its resident rules from the tasks instruction" and the init e2e block assertions |
| Permanent `@e2e` scenario "An agent synchronizes skills and the domain skill is routed" | n/a — missing delta re-point | **2. missing test coverage** | F3 — add the same-title `@givn.modified @e2e` delta scenario re-pointing the assertions to the served instruction (blocked outside this review's permitted edits) |
| No dead code, no hard-to-test regions, no redundant unit tests | — | — | nothing deleted or added by review; the delta removes no unit test |

## Scenario execution evidence

```
unproven candidate — no receipt yet (the archive runs and records the merged
full suite; this review does not run the full-suite authority).

Measured at HEAD 4692851b:
  delta scope, this runner:
    GIVN_FEATURES=givn/changes/extended-rules/specs cargo nextest run --locked --test features_runner --no-fail-fast
      Summary [5.809s] 79 tests run: 79 passed, 0 skipped                     EXIT=0
    GIVN_FEATURES=givn/changes/extended-rules/specs ./run-tests.sh
      Summary [5.914s] 79 tests run: 79 passed, 0 skipped                     EXIT=0
    GIVN_FEATURES=givn/changes/extended-rules/specs ./run-tests.sh --e2e
      Summary [1.221s] 12 tests run: 12 passed, 67 skipped                    EXIT=0
    givn check tasks --change extended-rules
      14 features / 80 scenarios (80 passed) / 512 steps (512 passed)         EXIT=0
      (the single-process count includes the two examples of the addon Outline)
  delta unit counts:  GIVN_CUCUMBER_TAGS='' → 79; '@e2e' → 12; 'not @e2e' → 67

  permanent pre-archive state (the delta is not yet merged; failures are
  reconciled by same-title delta modifications at archive):
    full scope:     606 run: 562 passed, 44 failed, 0 skipped
    non-@e2e scope: 516 run: 479 passed, 37 failed (all 37 superseded by
                    same-title delta @givn.modified scenarios — verified)
    @e2e scope:      90 run:  83 passed,  7 failed
                    (5 superseded by delta @e2e modifications; 1 superseded
                    by a non-@e2e modification — F2; 1 not superseded — F3)
```

**Receipt status:** unproven — candidate not yet archived.

## Overlap dispositions

For every shape match involving this delta (`givn lint --change
extended-rules` reports 24 `[SHAPE]` findings; the review gate lists the same
24 pairs). All are same-action invariants of the served ideation instruction
(distinct asserted rule content), so each is `variant`; none merges, none is
a boundary-free duplicate:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The ideation guidance binds personas to the artifacts that affect them | The ideation guidance asks the naming question as soon as a name fails | variant |
| The ideation guidance binds personas to the artifacts that affect them | The ideation command guardrails carry the verified-name rule | variant |
| The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes | The ideation guidance keeps aggregates to declared consistency boundaries | variant |
| The ideation guidance carries the seed schema and its status values | Handoff cuts change seeds by deliverable guarantee | variant |
| The ideation guidance carries the seed schema and its status values | The ideation guidance requires a canvas with distinguishable element kinds | variant |
| The ideation guidance carries the seed schema and its status values | The ideation guidance asks the term check before a phase closes | variant |
| The ideation guidance carries the seed schema and its status values | The ideation guidance carries the persona schema | variant |
| The ideation guidance carries the term schema and the collision procedure | The ideation guidance carries the canonical definition and the eight scoping rules | variant |
| The ideation guidance carries the authoring requirements and the handoff mapping | The ideation guidance stops questions at decided directions | variant |
| The ideation guidance carries the authoring requirements and the handoff mapping | The event storming review checks canvas coverage, legality, agreement, and rendering | variant |
| The ideation guidance carries the open-question qualifier rule | The ideation guidance carries the live-question standard | variant |
| The ideation guidance carries the open-question qualifier rule | The ideation command guardrails require the canvas and legal aggregates | variant |
| The ideation guidance carries the live-question standard | The ideation command guardrails require the canvas and legal aggregates | variant |
| The ideation guidance stops questions at decided directions | The event storming review checks canvas coverage, legality, agreement, and rendering | variant |
| The frame is confirmed once and later phase boundaries report | The ideation command guardrails carry the same conduct rules | variant |
| The frame is confirmed once and later phase boundaries report | The verified-name gate holds without the addon | variant |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance requires a canvas with distinguishable element kinds | variant |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance asks the term check before a phase closes | variant |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance carries the persona schema | variant |
| The ideation command guardrails carry the same conduct rules | The verified-name gate holds without the addon | variant |
| The ideation guidance requires a canvas with distinguishable element kinds | The ideation guidance asks the term check before a phase closes | variant |
| The ideation guidance requires a canvas with distinguishable element kinds | The ideation guidance carries the persona schema | variant |
| The ideation guidance asks the naming question as soon as a name fails | The ideation command guardrails carry the verified-name rule | variant |
| The ideation guidance asks the term check before a phase closes | The ideation guidance carries the persona schema | variant |

The `givn lint` run additionally reports 495 `[SUBST]` subset advisories
between permanent gate scenarios and delta scenarios (shared generic steps
only — exit codes, file existence, sync output); the review gate does not
require rows for them, and they are advisory only (`givn lint: 14 file(s)
checked — clean`, exit 0).

## Split-or-keep

The overlap gate reports no long-scenario findings for this delta (`givn lint`
count 0), so no split-or-keep row is required.

| Scenario | Decision |
|---|---|

## Sign-off

- [ ] Fabrication audit: NOT clean — F2 (removed `@e2e` coverage on a delta
      replacement), F3 (a permanent `@e2e` scenario failing with no delta
      replacement), F4 (stale design.md cells) remain unresolved; 0 empty
      step bodies; all commits verified (82/82 identifiers unique and
      resolvable; 49 production + 33 dispositioned spec/evidence commits);
      all promised components exist; no downgraded e2e scenario; no
      HTTP/fetch substitution; no over-produced e2e scope.
- [x] Scenario execution evidence recorded per scope (pre-archive runs above;
      the archive receipt will carry the authoritative measured result).
- [x] Every coverage gap is classified and resolved (or justified) from
      static inspection; no measured receipt exists yet (F3 = bucket 2).
- [x] No `@wip` tags remain in the delta spec (`givn check tasks` passed;
      80/80 scenarios green).
- [x] The delta spec contains no implementation-layer detail beyond the
      deliberate CLI/command names the interaction policy requires.
- [x] Local run command starts the declared product cleanly; no service stack
      is involved (`cargo run -- instructions` exit 0; delta suite exit 0).
- [x] The exact file(s) `verify.e2e_command` invokes were identified and read;
      the tree (including untracked files; `git status` clean) was searched
      for a second/parallel e2e implementation before accepting it.
- [x] `verify.e2e_command` is not identical to `verify.command`; measured
      counts prove real isolation (12 e2e < 67 non-e2e < 79 full); the
      archive receipt will confirm the merged scopes.
- [ ] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed — F4: two design.md cells (`serve_extended_rules_e2e_steps.rs`
      contents; `resident_rules_section` name) are stale vs as-built; the fix
      needs a design.md update + design-review re-run, not a silent shortcut.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] design.md's `## Internal Primitives` section was reviewed against the
      tree: `extensions::{load, append}` and `render_conditionals` match; the
      `resident_rules_section` naming difference is recorded under F4.
- [x] arc42 slug-mapping verified: every arc42 chapter slug in arc42.md maps
      to a real `docs/arc42/<slug>.md` file; the hook passed the scratch
      integrity run.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets (no "acceptable for now," "future scope,"
      "sufficient for current scope," or similar invented exception).

Open findings block sign-off: F1 (the `givn check review` gate exits 1 on the
`tasks.md` revision-identifier recording format), F2, F3, F4, and F5 (the
routing gate's findings, latent behind F1). Each requires a change outside
this review's permitted edits (`tasks.md`, `proposal.md`, `specs/`,
`design.md`/design-review, or evidence). Remediation: re-record the COMMIT
identifiers inline (F1); restore `@e2e` and commit the transcript (F2); add
the same-title `@givn.modified @e2e` delta scenario for "An agent synchronizes
skills and the domain skill is routed" and commit its transcript (F3); update
the two design.md cells and re-run design-review (F4); add the two Capability
Routing rows and un-wrap the failing usecase bullets (F5). After the repairs,
re-run the review gate; no mandatory check was deferred and no operator
decision is required for any of these repairs.

REVIEW: FAIL
