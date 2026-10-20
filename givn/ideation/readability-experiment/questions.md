# Questions: readability-experiment

## Q1 — Product capability or project-local rule?

- Status: closed 2026-10-19 (user)
- Blocks: the Frame scope boundary, the stakeholders, and whether this topic
  produces change seeds at all.
- Context: Givn's review step already records verdicts that `givn check review`
  enforces, so a readability verdict could follow the visual-review pattern.
  But Givn is a single feature-free binary with no language parser and no model
  dependency, and this repository has many source files over 500 lines today.
  If the check is a Givn capability, it must work for every project and
  language the project claims to support; if it is project-local, it can be
  Rust-specific and tuned to this repository.
- Question: should the readability check be a Givn product capability, a rule
  local to this repository, or a product capability that ships but is enabled
  only here at first?
- Options:
  1. **Givn product capability** — every project using Givn can run the check
     during review.
     In practice: a Python project using Givn changes `orders/service.py`; the
     review step produces the outline, judges it, and records a verdict that
     `givn check review` enforces.
     Gain: one consistent quality rule across projects; the rule is durable and
     specified.
     Cost: Givn must define outline extraction and size counting for arbitrary
     languages or accept a procedural, self-reported outline; every project
     inherits the false-positive and threshold decisions.
  2. **Project-local rule for this repository** — the rule lives in this
     repository's `givn/config.yaml` extension and/or a local script; Givn's
     product behavior does not change.
     In practice: changes to `src/cli/archive.rs` are judged by a local Rust
     outline command; a different project using Givn sees nothing.
     Gain: no product-boundary risk; the rule can be Rust-specific and tuned to
     the existing tree.
     Cost: the rule is not specified or enforced by Givn; other projects gain
     nothing; the check must be maintained as local tooling.
  3. **Opt-in product capability, enabled here first** — the check ships in
     Givn but is off unless a project enables it.
     In practice: `givn/config.yaml` sets `readability: on`; this repository
     turns it on, other projects keep today's review unchanged.
     Gain: the implementation is shared and dogfooded before it becomes the
     default; projects can adopt at their own pace.
     Cost: a new configuration surface and two review paths to specify and
     test; the extraction and language-scope problem remains.
- Resolution: **Opt-in product capability, enabled here first**, because it lets
  this repository evaluate the idea on real changes before the check is imposed
  on every project, while still producing a durable, specified capability
  rather than ad-hoc local tooling. The topic and capability are named
  **readability-experiment**. Rejected: option 1 because it imposes an unproven
  LLM judgment and an arbitrary-language extraction burden on every project
  before evidence exists; option 2 because it cannot be dogfooded as the
  product path, would not produce a reusable capability, and the local work
  would be thrown away when the idea is adopted.

## Q2 — What evidence decides whether the readability experiment succeeded?

- Status: closed 2026-10-19 (user)
- Blocks: the Frame Vision and the experiment's exit decision; without it the
  check becomes an unbounded gate that is never evaluated.
- Context: the check is opt-in and enabled in this repository first. It
  produces two kinds of evidence: recorded verdicts and findings from real
  changes, and the maintainer's judgment of whether those findings were genuine
  and useful. The experiment needs a decision rule that says when to promote
  the check, keep it opt-in, or retire it. The choice determines how much work
  the trial adds to each change and what evidence the change seeds must record.
- Question: what evidence should decide whether the readability experiment
  succeeded, and therefore whether the check is promoted, kept opt-in, or
  retired?
- Options:
  1. **Useful-findings trial** — run the check on the next 5 changes that touch
     source code; success means at least one finding is judged genuine and
     fixed, and no change is blocked by a finding the maintainer judges wrong.
     In practice: after five changes, three genuine findings were fixed and no
     change was falsely blocked; the check is kept.
     Gain: measures whether the check finds real problems; simple to record.
     Cost: "genuine" is a judgment call; a quiet period may produce no findings
     even when the check is sound.
  2. **Judge agreement** — two independent LLM judges assess the same outlines;
     success means they agree on pass/fail and on the named readability
     problem.
     In practice: both judges reject the same 600-line service and name the
     same vague method as the reason.
     Gain: measures reliability of the LLM judge.
     Cost: agreement does not prove the code improved; doubles the judgment
     cost; disagreement leaves no decision rule.
  3. **Reader comprehension test** — a fresh agent that did not write the code
     receives only the outline; success means it can state the unit's
     responsibility and the reason for the call order, and the maintainer
     agrees with that reading.
     In practice: a fresh agent reads `OrderService`'s public methods and call
     order and explains "places an order, reserves stock, then charges"; the
     maintainer agrees.
     Gain: directly tests the stated goal — understanding what the unit does
     and why from the outline.
     Cost: needs a second agent and a human check; "why" may not be answerable
     from an outline at all.
  4. **Bounded trial with all three signals** — record findings, judge
     agreement, and comprehension; the maintainer decides at the end.
     Gain: the richest evidence for the promote/keep/retire decision.
     Cost: the heaviest trial per change; risks measuring three things badly
     instead of one well.
  5. **Retrospective cross-project review** — review existing code in Givn,
     `watn`, and `kiekste`, and record every finding with the recommended
     action that would fix it.
     In practice: the three projects are reviewed at pinned revisions; the
     owner sees findings from code already shipped rather than waiting for the
     next change.
     Gain: evidence exists now; the findings describe real code, not a
     hypothetical future change.
     Cost: the review is large; it measures the code, not the cost or noise of
     running the check on every future change.
- Resolution: **Retrospective cross-project review** (option 5, introduced by
  the user in the answer), because the three projects already contain the
  evidence and a finding with a concrete fix is more useful than a pass/fail
  verdict. The review executes the proposed rule on real source units and
  records every finding with the recommended action that would fix it.
  Rejected: waiting for the next five changes because it delays evidence and may
  never reach a large unit; judge-agreement-only because it measures the judge
  rather than the code; comprehension-only because it produces no remediation.

## Q3 — What should the baseline review cover?

- Status: closed 2026-10-19 (user)
- Blocks: the review's workload, the report format, the experiment's runtime,
  and whether the findings form a complete remediation backlog.
- Context: the three target projects contain 208 Rust files and roughly 122k
  lines. Many files exceed 500 lines; `kiekste/src/workflow.rs` alone is 17,474
  lines. A flat review of every file produces a complete baseline but an
  enormous report; a sample is cheaper but cannot claim completeness. The
  eventual rule applies to a change's touched units, so the baseline is already
  a superset of that behavior.
- Question: which source units should the baseline review cover, and in what
  order?
- Options:
  1. **All Rust source units in all three projects** — every file, including
     tests and tooling, is judged; findings are prioritized after the fact.
     In practice: 208 files receive a verdict; `kiekste/src/workflow.rs`
     produces one decomposition finding and the report ranks it first.
     Gain: a complete baseline and a full remediation backlog.
     Cost: the largest review; a 17k-line unit may yield an outline too large
     for a single judgement; the report needs its own prioritization scheme.
  2. **Largest units first, then expand** — review the top 10 units by size in
     each project, then review further units only if the rule proves useful.
     In practice: givn's `timeline.rs`, watn's `setup.rs`, and kiekste's
     `workflow.rs` are reviewed first; smaller units wait.
     Gain: finds the worst offenses quickly and keeps the report usable.
     Cost: not a complete baseline; a small but unreadable unit can be missed.
  3. **Stratified sample per project** — one large, one median, and one small
     unit per project; expand only if the rule proves useful.
     In practice: three units per project are judged; the report compares how
     the rule behaves across sizes.
     Gain: tests the rule across the size range cheaply.
     Cost: findings are illustrative, not a remediation backlog for the
     codebase.
  4. **Only units touched by the next change in each project** — no
     retrospective baseline; the rule runs where it would run in normal work.
     In practice: a future change touching `src/cli/lint.rs` is reviewed; the
     17k-line `workflow.rs` is reviewed only when someone changes it.
     Gain: exactly matches the eventual gate and avoids a firehose of legacy
     findings.
     Cost: no evidence now; contradicts the user's request to review existing
     code.
- Resolution: **All Rust source units in all three projects**, because a
  complete baseline is the point of the experiment: it must show whether the
  rule finds real problems across the whole body of Givn-created code, not only
  in a selected sample. Findings are prioritized after the review by severity
  and unit size, so the report stays actionable. Rejected: largest-first
  because it cannot claim completeness and may miss a small unreadable unit;
  stratified sample because it produces illustrations rather than a
  remediation backlog; next-change-only because it produces no evidence now and
  contradicts the request to review existing code.
  Post-resolution note: the inventory shown when this was answered was
  production-only (208 files, ~122k lines). The literal option includes tests
  and tooling, which measured 474 files and ~285k lines; Q8 records the exact
  scope.

## Q4 — What decision rule ends the experiment?

- Status: closed 2026-10-19 (user)
- Blocks: the Frame's exit condition and the Handoff's promote/keep/retire
  decision. Without a rule the review produces evidence but no verdict on the
  rule itself.
- Context: the baseline review will produce findings and recommended actions
  across three projects. The experiment can end in three ways: promote the
  check (make it a default or a stronger gate), keep it opt-in, or retire it.
  The decision needs evidence that the rule is useful and not merely noisy. The
  review already promises a recommended action for every finding; the open
  question is what level of accepted, actionable findings justifies which
  outcome.
- Question: what evidence from the baseline review should decide whether the
  readability check is promoted, kept opt-in, or retired?
- Options:
  1. **Owner acceptance per project** — the experiment succeeds if the review
     produces at least one finding per project that the owner judges genuine
     and worth fixing, with an actionable recommendation.
     In practice: `watn/src/setup.rs` yields "extract provider setup from CLI
     parsing"; the owner accepts it, so the check is kept.
     Gain: directly tests usefulness and actionability.
     Cost: almost any large codebase yields some finding, so the bar may be too
     low; acceptance is subjective.
  2. **Severity threshold with a false-positive limit** — the experiment
     succeeds if at least five high-severity findings are confirmed across the
     three projects and no more than one finding is judged a false positive.
     In practice: ten confirmed high-severity findings and zero false positives
     lead to promotion; four findings and three false positives lead to
     retirement.
     Gain: measurable, prioritizable, and falsifiable.
     Cost: the severity scale and the numbers are arbitrary; one 17k-line unit
     can dominate the count.
  3. **Cross-project consistency** — the experiment succeeds if the same class
     of problem appears in all three projects and the owner judges the
     recommended actions useful.
     In practice: all three projects have vague public method names, and every
     proposed rename is accepted.
     Gain: shows the rule generalizes across Givn-created code.
     Cost: does not measure the cost or noise of running the check per change.
  4. **Owner decision after reading the report** — no numeric threshold; the
     owner reads the three reports and decides.
     In practice: after reading the reports, the owner keeps the check and
     schedules the top ten fixes.
     Gain: matches the real decision and is simple to run.
     Cost: not repeatable or falsifiable; the experiment cannot fail, so it is
     not really an experiment.
  5. **Owner-judged fix spike** — review the code, apply the recommended fixes
     to the selected findings in an isolated diff, and let the owner inspect the
     code and judge whether it improved.
     In practice: the agent produces the findings and the fixes; the owner
     reads the worktree diff and says "this is better" or "this is not".
     Gain: the judgment rests on concrete code changes rather than abstract
     findings.
     Cost: the owner must review the diff; a single judgment does not prove
     judge reliability or a low false-positive rate.
- Resolution: **Owner-judged fix spike** (option 5, introduced by the user in
  the answer). The experiment runs the baseline review, the agent applies the
  recommended fixes to the selected findings in an isolated diff, and the owner
  inspects the code and judges whether it improved. The owner's judgment is the
  decision rule, grounded in concrete code changes rather than abstract
  findings. Chosen because readability is a human judgment and the diff is the
  evidence the owner can actually assess. Rejected: a numeric severity
  threshold because no threshold substitutes for reading the result; owner
  acceptance per project because it judges findings, not the resulting code;
  cross-project consistency because it does not show whether the fixes improve
  the code.

## Q5 — How large should the fix batch be?

- Status: closed 2026-10-19 (user)
- Blocks: the spike's workload, the diff size, and the owner's review time.
- Context: the review covers 208 files and will produce more findings than the
  owner can inspect in one sitting. The batch must show whether the rule
  improves real code without burying the signal.
- Question: how many findings should the agent fix in the spike?
- Options:
  1. **One most-severe finding per project** — three fixes total, one commit
     each.
     In practice: Givn's `timeline.rs` gets one decomposition commit, watn's
     `setup.rs` one, kiekste's `workflow.rs` one.
     Gain: small, focused diff; every project represented.
     Cost: the rule's range is not tested; a project with two severe findings
     shows only one.
  2. **Three most-severe findings per project** — up to nine fixes, one commit
     each.
     In practice: three ordered commits per project; the owner reviews commit
     by commit.
     Gain: broader evidence and a realistic fix workload; still reviewable.
     Cost: more work; some fixes may be cosmetic; the diff is larger.
  3. **All high-severity findings** — every high-severity finding across the
     three projects.
     In practice: kiekste may produce dozens of commits.
     Gain: complete remediation of the worst findings.
     Cost: potentially unreviewable; the spike may take too long and bury the
     signal.
  4. **One finding per rule across the projects** — one naming, one
     single-responsibility, one size, one comprehension finding.
     In practice: four commits, each chosen where the rule fails most clearly.
     Gain: tests every rule.
     Cost: not the most severe; the selection is rule-driven rather than
     risk-driven.
  5. **Three most severe findings per rule per project** — up to 36 findings
     across the three projects, with overlaps reducing the count.
     In practice: each of the four rules contributes up to three fixes in each
     project; one commit per distinct fix.
     Gain: tests every rule across every project with a bounded batch.
     Cost: up to 36 fixes to apply and inspect; overlapping findings need
     shared commits.
- Resolution: **Three most severe findings per rule per project** (option 5,
  introduced by the user in the answer). The four rules — outline
  comprehension, expressive names, single responsibility, and source-unit size
  — each contribute up to three findings per project, so the batch is at most 36
  findings across the three projects. Findings may overlap across rules, so the
  actual count can be lower. The agent applies the recommended action for every
  selected finding, one commit per distinct fix. Chosen because it tests every
  rule across every project with a bounded, reviewable batch and still
  prioritizes severity. Rejected: one per project because it does not exercise
  the rule range; three per project because it does not cover all four rules;
  all high-severity findings because the diff becomes unreviewable.

## Q7 — Does the spike require a Givn change?

- Status: closed 2026-10-19 (user)
- Blocks: whether this topic produces a Givn change seed now, or only an
  experiment procedure and a decision record.
- Context: Q1 chose an opt-in product capability as the eventual direction. But
  the owner wants to know whether the rule works before investing in product
  work, and the spike can be run by an agent with existing tools.
- Question: should the spike be implemented as a Givn change, or run manually
  without changing Givn?
- Options:
  1. **Run the spike manually, no Givn change** — the authoring agent follows a
     written procedure and applies the rules by hand.
     In practice: the agent reads the outlines, writes the findings report, and
     applies the fixes in worktrees; nothing in `givn/` changes.
     Gain: the idea is tested before any product investment; nothing to
     implement, specify, or maintain.
     Cost: the procedure is not enforced by the tool; the next run depends on
     the agent following the written steps.
  2. **Build a minimal Givn addon first** — add the opt-in capability, then run
     the spike through it.
     In practice: `givn addons enable readability_experiment` changes the served
     review instruction before the first finding exists.
     Gain: the spike exercises the real product path.
     Cost: product work before the idea is validated; the experiment is coupled
     to an unproven capability.
- Resolution: **Run the spike manually, with no Givn change.** The agent follows
  a written procedure in this topic and applies the rules by hand; the opt-in
  product capability remains a later decision, taken only if the spike works.
  Chosen because the owner wants to validate the idea before investing in Givn
  development. Rejected: the minimal addon because it spends product effort
  before the idea has evidence and makes the experiment depend on an unproven
  product path.

## Q6 — How should the spike isolate and present the fix diff?

- Status: closed 2026-10-19 (user)
- Blocks: how the agent applies fixes without disturbing the main checkouts,
  and how the owner inspects the result.
- Context: the spike touches three separate repositories at pinned revisions.
  The owner wants to look at the code and judge each fix, so the diff must be
  inspectable per finding and must not mix the three projects. The main
  checkouts are clean except for one untracked file in the Givn checkout.
- Question: how should the spike isolate the fixes and present the diff to the
  owner?
- Options:
  1. **One branch per project, one commit per finding** — apply the fixes on a
     dedicated branch in each existing checkout.
     In practice: the owner runs `git log --oneline main..readability-spike` and
     `git show <commit>` in each project; each commit is one finding.
     Gain: natural Git review with no extra tooling; commit-by-commit
     acceptance.
     Cost: each main checkout gains a branch and must stay clean while the
     spike runs; switching branches to inspect is required.
  2. **One worktree per project, one commit per finding** — create a separate
     Git worktree for each spike branch and leave the main checkouts untouched.
     In practice: the owner opens the spike worktree in an editor and runs
     `git diff main...readability-spike`; the main checkout never moves.
     Gain: safe, parallel, and easy to open in an editor; the main checkouts
     are untouched.
     Cost: three worktrees to manage and clean up; extra disk space.
  3. **One patch series per project** — apply the fixes to temporary copies and
     export `git format-patch` files plus an index.
     In practice: the owner reads the patch files or applies them to a scratch
     branch; the experiment carries portable evidence.
     Gain: self-contained, archivable review artifacts; easy to share.
     Cost: patches can drift from the branches; inspecting code in context
     requires applying them.
  4. **One combined markdown report with before/after diffs** — a single
     document containing every diff.
     In practice: the owner reads one report; no branch or worktree is needed.
     Gain: one artifact and one reading pass.
     Cost: loses the surrounding code and the ability to run the project; the
     owner cannot judge the code in context.
- Resolution: **One worktree per project, one commit per finding.** Each
  projected pinned revision gets a dedicated Git worktree on a spike branch, so
  the main checkouts never move and the owner can open the code in an editor,
  run the project, and review a normal Git diff per finding. Chosen because the
  owner explicitly wants to read the code in context. Rejected: branches in the
  main checkouts because the spike would move the pinned revisions; patch
  series because applying patches adds a step and loses live context; combined
  markdown because the owner cannot inspect the code in context.

## Q8 — Does the baseline review include test and tooling sources?

- Status: closed 2026-10-19 (user)
- Blocks: the exact review scope, the report size, and whether test code is
  judged by the same readability rules as production code.
- Context: Q3 chose "all Rust source units", and the option text said
  "including tests and tooling". The inventory shown when the choice was made
  was production-only: 208 files and 121,708 lines. The literal all-Rust scope
  is 474 files and 285,167 lines, because the three projects also carry 266 test
  files (163,459 lines). Some of those test files are themselves very large:
  `givn/tests/steps/development_report_steps.rs` is 4,529 lines,
  `watn/tests/steps/interactive_shell_shortcut_steps.rs` is 4,295, and
  `kiekste/tests/steps/port_actions.rs` is 7,203. Test step definitions,
  fixtures, and tooling may need different readability expectations from shipped
  source.
- Question: should the baseline review include test and tooling sources, or
  only production sources?
- Options:
  1. **Production sources only** — 208 files, 121,708 lines; anything under a
     `tests/` directory is excluded.
     In practice: `kiekste/src/workflow.rs` is judged; `kiekste/tests/steps/*.rs`
     is not.
     Gain: focuses the review on shipped code and keeps the workload smaller.
     Cost: test-code readability is not evaluated; the largest test helpers
     remain unreviewed.
  2. **All Rust sources including tests and tooling** — 474 files, 285,167
     lines; the literal Q3 option.
     In practice: Givn's 4,529-line `development_report_steps.rs` is judged with
     the same rules as `src/`.
     Gain: complete baseline across every Rust file.
     Cost: nearly doubles the review; test and tooling code may need different
     rules, so findings may be noisy.
  3. **Production plus test step definitions** — production sources plus the
     `tests/steps` files where scenario behavior lives.
     In practice: step definitions are judged; fixtures and support files are
     not.
     Gain: covers the test code most likely to encode behavior.
     Cost: an arbitrary boundary; the review is still large.
  4. **Production plus tooling and test support, excluding step definitions** —
     reusable helpers are judged, repetitive step definitions are not.
     In practice: `kiekste/tools/` and `testsupport/` are judged;
     `tests/steps/port_actions.rs` is not.
     Gain: focuses on reusable code rather than repetitive glue.
     Cost: long step definitions remain unreviewed.
- Resolution: **Production sources only** (option 1). The review covers 208
  production files and 121,708 physical lines; anything under a `tests/`
  directory is excluded. Chosen because the question is whether the rules
  improve shipped code, and the test and tooling inventory would nearly double
  the review while likely needing different readability expectations. Rejected:
  all Rust sources because the test and tooling load is disproportionate and
  noisy; production plus step definitions and production plus tooling because
  both keep an arbitrary boundary and still expand the review.

## Q9 — Facade exception or true type split for `Engine`?

- Status: open
- Blocks: the revision of `reports/engine-decomposition-plan.md`; the plan's
  acceptance criteria; whether the 500-line type rule can be met at all.
- Context: the plan keeps all 22 public `Engine` fields and 144 public methods
  to preserve the public API, tests, and `Clone` seams. The fresh-context review
  shows the facade alone cannot be <=500 type code lines, and the target
  services are stateless borrowed views over the same fields. The type therefore
  remains the owner; the plan is a facade extraction, not a type split. The
  options differ in whether the public field/method surface may change.
- Question: should the Engine decomposition preserve the public surface as an
  explicit facade exception, perform a true type split that breaks direct field
  mutation, or take an incremental compatibility path?
- Options:
  1. **Facade exception (compatibility-first).** Keep the 22 public fields and
     144 public methods; extract method bodies into borrowed services; record
     `Engine` as an explicit type-size exception.
     In practice: `Engine` remains 144 delegating methods and >500 code lines;
     `PreviewService`, `RunLoop`, etc. are stateless views; `workflow.rs` shrinks
     but the type does not.
     Gain: no public API or test break; lowest migration risk.
     Cost: the god type remains; the 500-line type rule is not met; the finding
     is closed by exception, not by refactoring.
  2. **True type split (break public field mutation).** Privatize the `Engine`
     fields, move state into state-owning services, reduce `Engine` to a small
     facade with accessors, and update tests/embedders.
     In practice: `PreviewService` owns the preview fields; `Engine` no longer
     has 22 public fields; callers use methods; `Engine` can fall under 500 code
     lines.
     Gain: a real type split; services own state and lifecycle; the rule can be
     met.
     Cost: breaking API change for tests and embedders; `Clone`/`Send`/`Sync`
     and public-field mutation seams must be redesigned; higher migration risk.
  3. **Incremental true split with a compatibility layer.** Keep the public
     fields for now, add state-owning services behind them, deprecate direct
     mutation in phases, and shrink the facade over several releases.
     In practice: `Engine` keeps fields and delegates; new services own derived
     state; later steps make fields private and remove the delegates.
     Gain: lower risk than option 2; a path to the type split.
     Cost: transitional duplication; the type remains >500 lines for several
     releases; the plan must define when the facade exception ends.
  4. **Stop at leaf services.** Extract only the low-risk leaves (support,
     reporting, events) and accept `Engine` as a documented exception.
     In practice: a few small services leave; `Engine` remains the god type.
     Gain: small, safe improvement.
     Cost: the rule is not met; the finding stays open; the main problem remains.
- Resolution: pending.

## Q10 — How should the gate be validated or adopted next?

- Status: closed 2026-10-19 (user)
- Blocks: whether this topic produces a Givn change seed now, a dry-run plan, or
  stops at the proposal.
- Context: the gate proposal and the four-case replay are complete. The gate
  uses code lines, evaluates types across files and implementors, classifies the
  unit, and requires a plan or exception at creation and on change. The replay
  shows all four cases would have fired at or near the 500-line crossing, but
  the sample is four historical cases in three young projects. The gate is not
  yet a Givn behavior.
- Question: what should happen next — a dry-run on a real change, a wider
  historical replay, a Givn change now, or no adoption?
- Options:
  1. **Dry-run on the next real change.** Apply the gate manually to the next
     change in one of the three projects and record findings, false positives,
     and owner decisions.
     In practice: the next kiekste change is measured with the tool; the review
     records the classification and the required plan/exception.
     Gain: real validation with low cost; no product commitment.
     Cost: waits for a real change; one change is a small sample.
  2. **Replay more historical changes.** Run the gate on more historical
     commits across the three projects and tune the thresholds.
     In practice: 10-20 created or grown units are measured and classified.
     Gain: more data now; no waiting.
     Cost: analysis time; historical commits may not represent future changes.
  3. **Draft the Givn change now.** Turn the gate into an opt-in Givn capability
     with the AST tool and the review instruction.
     In practice: an opt-in addon/check generates metrics and requires a
     plan/exception at review; the tool is integrated with Givn.
     Gain: moves toward prevention in every future change.
     Cost: product work before the gate is validated on real changes; language
     scope and tool integration must be designed.
  4. **Stop at the proposal.** Keep the gate as a topic artifact and do not
     adopt it.
     Gain: no product commitment.
     Cost: the massive-file problem remains; the experiment has no enforcement.
- Resolution: **Replay more historical changes** (option 2). The four-case
  replay was extended to 13 more units across givn, watn, and kiekste. Rejected:
  dry-run first because the sample was too small; Givn change now because the
  thresholds were not tuned; stop because prevention is the goal. Results are in
  `reports/gate-replay-wide.md`; the tuned gate is in the same report.


## Q11 — After the tuned replay, what is the next step?

- Status: open
- Blocks: whether the gate is tested on a real change, turned into a Givn
  change, or parked.
- Context: the 13-case replay and the tuned gate are complete. The tuned gate
  uses a 500 code-line type gate with LCOM4/TCC classification, a 500-999 module
  watch band, a 1,000 module plan, an interface rule, a data-only exemption, and
  a method-length rule. The gate is not yet a Givn behavior. The next step
  decides whether we validate it on a real change before productizing.
- Question: what should happen next — dry-run on the next real change, draft
  the Givn change, replay a final set, or stop?
- Options:
  1. **Dry-run on the next real change.** Apply the tuned gate manually to the
     next change in one project; record findings, false positives, and owner
     decisions.
     In practice: the next kiekste change is measured; the review records the
     watch/plan/exception and the outline verdict.
     Gain: real validation of the tuned thresholds at low cost.
     Cost: waits for a real change; one change is a small sample.
  2. **Draft the Givn change now.** Turn the tuned gate into an opt-in Givn
     capability with the AST tool and the review instruction.
     In practice: an opt-in addon/check generates metrics and requires a
     plan/exception at review.
     Gain: prevention in every future change.
     Cost: product work before real-change validation; language scope and tool
     integration must be designed.
  3. **Replay a final set.** Run the tuned gate on 5-10 more units, especially
     interfaces and cohesive test doubles, to confirm the exemptions.
     Gain: more confidence in the classification.
     Cost: more analysis time; the main pattern is already clear.
  4. **Stop at the tuned proposal.** Keep the gate as a topic artifact.
     Gain: no product commitment.
     Cost: the massive-file problem remains.
- Resolution: pending.
