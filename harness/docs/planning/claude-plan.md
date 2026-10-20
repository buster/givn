# Plan: measure the givn workflow by outcomes — paired, ablatable, re-gradable

DRAFT v3 — awaiting: event-schema report, guardrail inventory, two design
reviews. Sections marked (TBC) will be completed from them.

## Context

Three blind spots: (1) a givn change (skill text, gate, CLI behaviour) cannot be
shown to make idea→archive better or worse; (2) model drift is invisible;
(3) agent-harness differences are invisible. Underneath: no way to decide which
parts of givn are necessary.

The HN thread and the superlite post are anecdote-grade on both sides. The
superlite benchmark measures the regime where a process framework adds least —
a fully specified greenfield build, scored once. givn's claims live in two other
regimes: an underspecified idea (elicitation, domain vocabulary) and later
changes (executable specs as regression net, living spec as handoff to a fresh
session). A setup without those regimes is biased against givn; a setup with
them that shows no effect falsifies the claims. Confidence that givn adds value
is a hypothesis per component, not a premise.

Production reality (commit trailers in this repo): givn is developed mostly with
DeepSeek Flash on opencode (0.7–0.8) and omp (0.9). The evaluation should start
on that setup; a strong Claude model is the contrast arm for the HN claim, not
the baseline.

## Diagnosis: why the current harness cannot answer the questions

| # | Finding | Evidence | Consequence |
|---|---|---|---|
| 1 | No outcome oracle | 12 full-profile checks are form checks (artifacts exist, PASS markers, lint, ticks, commit format, addons on, README length) | A run that builds the wrong product passes |
| 2 | Judge = artifact counter | Every existing artifact scored 4/4 in the last campaign and in the 4-hour run (4,4,4,3,4 both times); missing = 0. Absolute 0–4 "follows givn conventions" rubric, same model as the agent, judge never sees givn's instructions | Zero discrimination between a hello-world smoke and a 58-scenario product |
| 3 | Budget decides comparisons | opencode/omp timed out at 1200 s after 5 iterations; maki finished in 455 s. Iterations mean different things per agent; N = 1 | Agent ranking = speed under a cap |
| 4 | Task contract fights givn | Smoke forbids implementation; `givn instructions` drives into it; maki obeyed givn → FAIL | Measures prompt precedence |
| 5 | Harness prompt does givn's job | `assets/ralph-givn-prompt.md` itself orders "run `givn instructions`", RED first, one commit per scenario; 66/71 sessions started with `givn instructions` — because the harness prompt said so | Rule compliance and ablations are confounded |
| 6 | Questions end the run | Prompt: write `BLOCKED.md`, output COMPLETE; `_check_blocked` exits without a report. 0 questions in the 4-hour run; design-review = 56 min of self-grilling | Elicitation/dialog value is untestable |
| 7 | One model, no Claude Code | `model.py` refuses everything but DeepSeek Flash; agents opencode/omp/maki | Axes (2) and (3) blocked |
| 8 | Trials not isolated / not durable | `~/.omp`, `~/.local` are mounted writable in the sandbox. In the 4-hour trial the agent grepped `~/.omp/agent/sessions` and got hits from another harness trial (`-tmp-givn-harness-workspaces-tmpi9gg90ts`, its DesignGrill output) and from real project sessions (`-projects-kiekste`, `-projects-givn`, `-projects-watn`), and read managed skills of other projects (`kiekste-givn-usecase-and-archive-ga…`). `~/projects` itself was correctly unreachable. 393 workspaces in `/tmp`, one real idea→archive run; resumed runs lose wall time and log | Trials copy from each other and from your real work; anything discussed in a host session can leak into a trial; no re-grading |
| 9 | Data present but unused | Record holds per-turn tokens/cost; harness reports neither | Cost axis missing |
| 10 | Bugs that corrupt manifests | dirty flag never fires (`runmeta.py:43`); ralph path never receives the template (`runner.py:105`); driver mislabelled for omp/maki; `{{context}}{{tasks}}` literal; resource limits never reported | Manifests can't be trusted for comparison |

## What the single full run suggests (omp + Flash, N = 1 → hypotheses only)

| Window (commit-derived) | Wall | Cost | Turns | Note |
|---|---|---|---|---|
| proposal → design | 13 min | $0.13 | 105 | |
| design-review | 56 min | $0.90 (30 %) | 622 | 6 self-grilling subagents |
| tasks | 6 min | $0.07 | 42 | |
| implementation | ~150 min | $1.62 (55 %) | 2,573 | 57 scenarios, median 2.7 min |
| review + archive | 18 min | $0.25 | 226 | 22 of 23 `givn archive` calls failed |
| total | 4 h 13 min | $2.96 | 3,568 | 274 M cache-read, 4.3 M in, 2.5 M out |

- Same token profile on Claude: ≈ $88 (Sonnet 5.5) / ≈ $122 (Opus 5.5) per run
  (cache writes excluded; a different model changes the profile).
- Smoke (planning-only) runs: $0.06–0.18 on Flash.
- Archive friction (reasons only in tool output; givn's own command entries carry
  only `exit_code`): uncommitted `AGENTS.md` generated by givn itself; missing
  runner-result contract; corrupt state (`changes/` and `archive/` both present);
  coverage command failure; evidence tree changed; recovery via `rm -rf`.
- 60 of 125 commits are "record evidence" bookkeeping (≈ 12 min total).
- givn's own config asserts "NEVER remove design-review … it defers the same
  problems to implementation". Testable; the costliest planning component.

## Verified defect in the development report (independent of the harness)

`tasks` is `Done` only when every checkbox is ticked (`src/state.rs:206-219`);
the transition writer emits a `phase` entry only for `Done` artifacts
(`src/history.rs:404-444`); the report lets `tasks.md` open `implement`
(`src/cli/timeline.rs:1535-1550`, `stage_opened_by`). Result in `tmpoqt4z103`:
the report shows `tasks` = 16:59→19:41 (2 h 42 min, containing the whole
implementation) and `implement` = 19:41→19:47 (6 min). The same per-artifact
semantics mismatch applies to `design-review.md`/`review.md`/`arc42.md`
(stamped at PASS, not appearance). Every change's report is affected.

## Guardrail map: what givn enforces vs what it only asks

The rules an agent sees live in served instructions (`assets/instructions/*.md`
via `givn instructions <id>`, cascade `src/config/cascade.rs:55-83`); the 15
`SKILL.md` files are 10–16-line pointers. One full change with all addons loads
≈22,700 distinct words (≈35–40 k tokens, estimate) plus repeated loads in
subagents; the dialog policy alone is 2,691 words.

| Class | Members | Evaluation consequence |
|---|---|---|
| Code, always on | archive pipeline: run declaration, merge, regular+e2e runs with runner result file, clean tree, tested revision; coverage gate (no threshold) | Measure friction (failed attempts, time lost) and what they catch |
| Code, available but optional | integrity hook, overlap dispositions, routing consistency, retirement disclosure — only via `givn check review --change`; no forward instruction tells the agent to run it (`check.rs:142-171`) | Measure how often they run and what they catch; wire into archive or drop |
| Prompt only | dialog policy, tick tasks immediately, RED first, strict-mode proof, never-empty step bodies (no binary check), subagent orchestration, fresh-context design review, one commit per scenario | Rule-compliance matrix + text ablation; candidates for code or deletion |
| Form signals (gameable) | `REVIEW: PASS` is a substring check (`VISUAL-REVIEW: PASS` satisfies it); checkbox substrings anywhere; `COMMIT:` ids never looked up in git; editing `design.md` does not withdraw design-review; `.givn-skip` can skip `review`; agent can write `givn/artifacts/**` overrides and `commands.yaml` | Never grade on them; add tamper checks per trial |

Improvement candidates already visible (to be ranked by measured cost, not
fixed blindly): undocumented `$GIVN_RESULT_FILE` runner contract (the 4-hour run
found it by a `wip: probe runner result contract` commit); givn-generated
`AGENTS.md` dirtying the tree and blocking archive; partial archive leaving
`changes/` and `archive/` both present; templates that integrity parses
(`- [x] COMMIT:`, `## Step Definitions`) but never serves; `tasks.md` bloat from
pasted runner output (976 KB in `development-timeline`); the report phase
defect; 8 doc/code disagreements (e.g. `docs/workflow.md:152` claims
`check review` runs the tests).

## Method (draft; TBC with design reviews)

**Trial** = (task, arm, repeat). **Arm** = givn binary/ref + givn overlay
(addons, artifact graph, instruction overrides/extensions), harness (agent CLI +
version), model + effort, driver regime. Every trial writes a bundle; grading
runs offline on bundles with versioned graders, so a better grader re-scores old
trials.

**Neutral driver** (removes confounds 4–6). One prompt template for every arm,
with no givn rules in it: project task, current change request, "work
autonomously; to ask the product owner, write `QUESTIONS.md` and end the
session; when finished, write `DONE`". An arm's guidance comes only from what
the arm installs (givn's AGENTS.md block, skills, served instructions). Stop on
`DONE` or budget (wall clock, $, sessions). Questions → simulated product owner
answers into `ANSWERS.md` → next session.

**Four experiment types on one substrate**

| Question | Vary | Hold fixed | Cadence |
|---|---|---|---|
| Did this givn change regress/improve? | givn ref | model, harness, tasks | paired + concurrent, per release candidate |
| Model drift / which model? | model | pinned givn release (canary), harness | scheduled |
| Which harness? | harness | model where shareable, givn | campaign |
| Which component is necessary? | overlay (ablation) | model, harness | campaign; re-run when the canary shifts |

Base and candidate always run in the same time window so provider-side model
changes cancel; old numbers alone are never the comparison.

**Graders, priority order**
1. Hidden acceptance tests on a pinned observable surface, unreachable from the
   sandbox; partial credit + critical subset; oracle check (reference ≥ 95 %)
   and null check (empty scaffold ≤ 5 %) before any campaign.
2. Elicitation: hidden-brief rules learnable only by asking; their tests are
   part of (1).
3. Evolution: regression = hidden test passing after change k−1, failing after
   k (superseded tests excluded).
4. Executable-spec strength: own-suite green vs hidden red (false green);
   mutation score of the own suite on a pinned stack (later).
5. Process from record + git: per-phase wall/cost (git-derived windows), gate
   first-pass rate, failed-command reasons, retries, loops, rework, questions,
   rule compliance, tamper checks; capture coverage is a validity precondition.
6. Cost: $, tokens, wall, sessions; cost per accepted requirement.
7. LLM judge only for what 1–6 cannot see (vocabulary consistency, design
   fidelity, question quality): binary checklist or randomized pairwise,
   different model family from the agent, "unknown" escape, calibrated on
   seeded-defect artifacts and known negatives.
Infra failures (provider errors, sandbox crash, served-model mismatch) go to an
error sidecar, never into scores; budget exhaustion is a scored outcome.

**Arms for "what is necessary"**
- **A0 no givn**: same neutral driver, AGENTS.md with only stack/run facts.
- **A1 spec core**: `givn init --no-addons`; overlay removes `design-review`,
  makes `review` optional and drops it from `apply.requires`; instruction
  overrides (`givn/artifacts/<id>/instructions.md`) remove text that insists on
  the removed steps. Keeps proposal → Gherkin specs → design (runner) → tasks →
  RED/GREEN per scenario → archive into living specs.
- **A2 givn as shipped** (all addons).
- **A2−X leave-one-out**, first X = design-review (30 % of the 4-hour run's
  cost; givn's own config claims it is indispensable), then arc42, review,
  domain_modeling, dialog policy, coverage gate.
A1 vs A0 tests the user's claim (executable domain spec system); A2 vs A1 tests
the ceremony the HN thread attacks; running A0/A2 on Flash and on a strong
Claude model tests the model × givn interaction.

**Rule-compliance matrix**: detector per prompt-only rule from record + git
(checkbox delta per scenario commit; scenario commits vs scenarios; failing test
run before production edit; first tool call per session; empty step bodies by
static scan). Followed without the text → delete the text. Never followed →
enforce in code or drop. Followed only with the text → keep.

**Friction mining**: failed givn commands, their first error line (from tool
output), attempts until success, minutes and $ until success, recovery actions
(`rm -rf`, `git reset`, `.givn-skip`, edits under `givn/artifacts/`). Clustered
by reason, ranked by cost lost, tracked per givn version.

**Statistics**: pilot (5 repeats of one arm) → variance; repeats from the
minimum detectable effect, stated in every report (pass-rate noise floor ≈
1/√(cases·repeats), worse when tests within a trial correlate); paired
bootstrap CIs over tasks × repeats; pass^k for consistency; regression gate =
non-inferiority margin on the primary outcome + cost guardrail, otherwise
"inconclusive — add repeats".

**Speed tiers**: T1 phase replay from frozen checkpoints (minutes, cents, for
skill-text edits); T2 small end-to-end tasks (≤ 30–60 min, every release
candidate); T3 evolution + large task (hours, nightly/weekly, ablations).
Trials run in parallel; Flash screens, Claude confirms.

## The development record as process substrate

Envelope per line (`src/history.rs:33-69`): `schema, id, at` (END time, UTC ms),
`scope{kind,id}, source, kind, name, label, duration_ms?, outcome`
(ok|failed|skipped|unknown)`, parent, session, data`. Produced kinds: `command`,
`step`, `phase` (artifact transition | `mandatory checks` gate | ideation
marker), `turn`, `usage`, `tool`, `subagent`, `question`. Documented but never
produced: `hook`, `revision`, `session`.

Gives per trial: per-turn tokens/cost/context/model/stop reason; tool calls with
duration, outcome, error, and full input/output in `full` mode; subagent spans;
givn commands with exit code; gate runs; verification steps; questions with
full text and wait time.

Traps the metric extractor must handle:
- ~90 % of `command` entries are the recorder's own plumbing (`history ingest`,
  `history harness-hook`, `attribution …`) — exclude.
- `at` is the end time; durations of different kinds overlap — sum one kind.
- Phase transitions mean "first givn command that saw the artifact Done" (defect
  above) → derive phase windows from git (first commit touching each artifact).
- Harness name is not stored (intake drops the payload's `reporter`); model only
  on turns with usage → take both from the trial manifest.
- Claude: turns have no duration and `cost` is a session rollup (the report's
  cost sum over-counts) → take cost from `claude -p --output-format stream-json`.
- omp subagents spanning two flushes produce two entries → dedupe.
- Failing `givn lint`, `readme check`, `run` exit via `process::exit` and leave
  no entry; failed commands carry only `exit_code` → failure reasons come from
  tool output (full mode) until fixed.
- Answers to questions are not recorded → the simulated product owner's log is
  the source.
- Per-turn `usage.total_tokens` re-counts context; opencode records no
  cache reads → compare harnesses on cost, not token sums.

Small givn fixes worth separate changes (not prerequisites for the harness):
G1 transitions at artifact appearance (fixes every report's phase attribution);
G2 always record failed commands with their first error line; G3 store the
harness name on agent entries; G4 Claude per-turn cost delta and turn duration;
G5 revision id on `commit` entries.

## Task suite (draft; TBC with validity review)

Benchmark task = `harness/benchmarks/<task>/`:
`task.md` (agent-visible idea + the minimal pinned observable surface),
`brief.md` (hidden product-owner knowledge), `changes/CR-2.md…` (change
requests delivered one per fresh session), `hidden/` (acceptance tests per
stage, critical tags, superseded-test list), `reference/` (solution for the
oracle check), `task.toml` (stack pin, build/start contract, budgets).

- **S1 small, specified** — pinned CLI contract, domain with invariants (e.g.
  library loans: loan limit, due dates, renewals); CR-2 adds a conflicting rule
  (a hold blocks renewal), CR-3 is additive (late fees).
- **S2 small, underspecified** — same size; `task.md` gives the idea and the CLI
  surface only; the rules live in `brief.md` and are learnable only by asking.
- **E evolution, shopping-list domain** — sourced from `../shoppinglist-070`
  (27 permanent features, 299 scenarios, 42 archived changes): v1 household +
  lists + items + check-off; CR-2 quantities; CR-3 gift items hidden from their
  recipient (`gift-lists` + `protect-surprise-gifts`, a cross-cutting rule).
  Hidden tests are a curated port of those scenarios onto the pinned surface;
  the real project is the reference solution.
- **L large validity** — the existing shopping-list website task (its design
  zip pins labels: `pin-input`, `Liste öffnen`, `item-name`, `+ Hinzufügen`,
  `Im Einkaufswagen`), graded by browser tests on those labels plus design
  fidelity; excluded from the regression tier.
- Later: replayed real changes from givn (130 archived), watn (54), kiekste
  (129) with fail-to-pass / pass-to-pass tests from the real change.

Own-suite metrics come free from the archive receipt
(`givn/archive/<id>/verification.json`: scopes with total/passed, line
coverage) → false-green rate = hidden failures while the own suite is green.

## Decision: where the harness work is specified and archived

- Question (2026-10-19): `harness/` cannot archive (givn refuses nested roots,
  `src/scm.rs:113-121`); where do the increments live?
- Chosen (user): a separate repository, with `harness/` kept in the givn repo as
  a git subtree of it.
- Rejected: folding the evaluation into the root givn project (every root
  archive would need Python/behave next to cargo); teaching givn nested roots
  first (product change that delays the evaluation by at least one change).
- Flow: givn workflow runs only in the separate repo (`~/projects/givn-harness`,
  project root = worktree top level); the givn repo pulls it into `harness/`
  with `git subtree pull --prefix=harness` after each archived harness change.

## Increments (draft; TBC with implementation review)

Each increment is one givn change in `~/projects/givn-harness`, synced into
`givn/harness/` by `git subtree pull` after archive. Hidden material (hidden
tests, briefs, reference solutions) lives outside the subtree in a separate
local repo (`~/projects/givn-benchmarks`), never mounted into the sandbox and
never mirrored into givn (private today; agents have web search if it ever
goes public).

- **H-1 Repo setup** (no givn change; maintenance): `git subtree split
  --prefix=harness`, create the repo, `givn upgrade` 0.2.0 → 0.9.2 through its
  maintenance flow, reconcile `bubblewrap-sandbox` (changes/ copy has newer
  `sandbox.feature`, archive/ copy has `review.md`), archive or close
  `givn-test-harness`, `git remote add` + first `git subtree pull` in givn.
- **H0 Trustworthy trials** — fixes that would otherwise corrupt every later
  number: dirty flag (`runmeta.py:43`), driver label, literal `{{context}}`,
  resource limits in report, resume keeps wall time/log/iterations, per-trial
  agent-state dirs instead of shared `~/.omp`/`~/.local`, graded trials refuse
  to run unsandboxed, infra-error sidecar, `check-step-bodies.sh` subshell bug,
  mislabelled checks ("archive succeeded" on a failed receipt).
- **H1 Trial bundles + offline grading + cost** — bundle (manifest, events,
  `git bundle --all`, logs, transcripts, `metrics.json`, versioned grades);
  `givn-harness grade`; metric extractor implementing the record traps above
  (git-derived phase windows, plumbing excluded, failed-command reasons,
  dedup, own-suite totals from `verification.json`, tamper checks).
- **H2 Hidden acceptance** — benchmark task format, hidden-test runner in its own
  sandbox against the produced workspace, partial credit + critical subset,
  oracle/null checks; task S1 v1 + CR-2 with reference solution.
- **H3 Arms + neutral driver + product owner** — arm overlays (addons, artifact
  overlay, instruction overrides, no-givn workspace), one neutral template,
  `QUESTIONS.md`/`ANSWERS.md` protocol with a fixed product-owner model and
  leakage audit, arm-aware evaluator; task S2.
- **H4 Campaigns + paired comparison** — campaign spec, parallel interleaved
  execution, per-arm budgets, `compare` with paired bootstrap CI, MDE, pass^k,
  Pareto table; cross-trial tables (JSON → DuckDB/Parquet).
- → **First campaign.**
- Later, ordered by what the first campaign shows: H5 evolution runner + task E;
  H6 Claude Code driver + declared model matrix (cost from stream-json); H7
  calibrated judge; H8 phase checkpoints/replay (T1); H9 mutation grader;
  givn-side G1–G5.

## First campaign (draft): spec core vs ceremony vs nothing, on the production setup

- Setup: omp + DeepSeek Flash (`--thinking max`) — the setup givn is actually
  built with (commit trailers) and the richest capture.
- Tasks: S1 (specified) and S2 (underspecified, product owner), each v1 + CR-2.
- Arms A0 / A1 / A2, 5 repeats → 30 trials, interleaved, ~6 concurrent.
- Estimated cost $25–40, wall ~6–8 h (Flash smoke runs cost $0.06–0.18; the
  4-hour product run $2.96). Estimate, to be replaced by measured numbers.
- Primary outcome: critical hidden-test pass rate after CR-2 and regressions
  introduced by CR-2. Secondary: elicited rules (S2), cost, wall, false green,
  gate friction, rule compliance.
- Pre-registered reading:
  - A1 − A0 ≥ +10 pp with the 90 % CI above 0 → the spec core earns its keep on
    Flash; CI spanning 0 while A1 costs more → necessity not shown on these
    tasks.
  - A2 − A1 < +5 pp while cost +30 % or more → ceremony is a shrink/drop
    candidate; next campaign = leave-one-out on design-review.
  - A2 ≈ A0 on S1 but A2 > A0 on S2 → givn's value is elicitation, not
    execution discipline.
- By-product: per-arm/task variance → sizes the regression gate (T2: A2 only,
  base vs candidate, 2–3 small tasks, ~$10, ~2–3 h) and the model canary.
- Detectable effect with 5 repeats: roughly 10–15 pp on pass rate; the pilot's
  measured variance replaces this guess.

## Verification (draft)

- Every increment: behave scenarios with fake agents and a fake product owner
  (deterministic scripts writing known outputs); `./run-tests.sh` green;
  `scripts/check-step-bodies.sh` fixed so it can fail.
- Every benchmark task: oracle (reference ≥ 95 % hidden pass) and null (empty
  scaffold ≤ 5 %) before use; hidden tests and briefs reviewed by the user.
- "Mechanism wired": a sabotaged arm (instructed to skip tests) must score
  visibly worse on false green and hidden pass.
- One real Flash trial per arm on S1 before the campaign: bundle complete,
  grades present, cost extracted, product-owner log present.
- Re-grading the pilot bundles reproduces deterministic grades exactly.
- Statistics code tested on synthetic data with known effects (CI coverage).
- givn repo after each `git subtree pull`: root `./run-tests.sh` still green.

## Approval scope (draft)

Approving this plan authorizes: creating `~/projects/givn-harness` locally from
`git subtree split --prefix=harness` and repairing its givn state; increments
up to the first campaign as givn changes in that repo; authoring S1/S2 with
hidden tests and briefs (for user review before use); running the oracle/null
checks and the first campaign on omp + Flash (~$25–40).

Not authorized: creating or pushing to any remote; Claude-model campaigns
(separate budget decision); the givn-side fixes G1–G5 (separate givn changes);
edits to givn skills/instructions based on results (each its own change with
evidence).
