# Frame: readability-experiment

Status: confirmed 2026-10-19 (user) for the goal, scope, and stakeholders.
Q1-Q8 closed.

## Problem

Givn's verification, review, and archive gates check specification conformance,
coverage, structural integrity, migration evidence (a separate
`givn check migration` check), and visual evidence. They do not check whether
the source code itself can be understood from its structure. An author can
satisfy every scenario and still leave a source unit whose public interface does
not name its responsibility, whose internal call order does not reveal its flow,
whose methods mix concerns, and whose file grows without bound. The reviewer has
no recorded, repeatable basis for rejecting that code.

The spike narrows the open question to: **do selected, agent-applied fixes based
on the proposed readability rules improve the code in the owner's judgment?**
Whether an LLM judge is reliable enough to gate future work, whether the rules
produce a low noise rate, and whether an outside reader can understand the
outline are deferred until the spike shows that the fixes themselves are worth
it.

## Vision

The experiment runs a retrospective readability review over the existing code of
the three Givn-created projects: Givn itself, `watn`, and `kiekste`. The review
produces the outline of each source unit, judges whether the unit's purpose and
call order can be understood from that outline, applies the naming,
single-responsibility, and size rules, and records every candidate finding with
the recommended action that would fix it.

The agent applies the recommended action for the selected findings in isolated
worktrees. The owner inspects the code and the diff, classifies the findings,
accepts or rejects each fix, and judges whether the code improved. That judgment
is the experiment's result.

The spike adds **no Givn behavior**. It is run by the authoring agent with the
existing tools and a written procedure. Only if the owner judges the result an
improvement does the project consider a Givn change (an opt-in capability). The
spike exists to answer that question before any product work is funded.

## Experiment definition

- **Projects under review:** `/home/buster/projects/givn`,
  `/home/buster/projects/watn`, `/home/buster/projects/kiekste`. All three are
  Rust projects created with Givn and carry `givn/` state. The trial pins the
  checkouts it reads:

  | Project | Revision | Working tree |
  |---|---|---|
  | givn | `89e5423deb94fbd97775b484cd2aec14d8ef257f` | one untracked `tools/.gitignore` |
  | watn | `9b2fc8455d9a5e145d69d9a01764ad874c6eee95` | clean |
  | kiekste | `1be22f70d2a51ec2d94e626d7c4de735866132ea` | clean |

- **Source inventory:** the review covers every production Rust source unit in
  the three projects. Q8 excluded test and tooling sources: the selected scope
  is 208 production files and 121,708 physical lines. The all-Rust columns are
  recorded for context only.

  | Project | Production files | Production lines | All Rust files | All Rust lines | Production files over 500 | All Rust files over 500 |
  |---|---:|---:|---:|---:|---:|---:|
  | givn | 68 | 28,626 | 147 | 65,288 | 20 | 30 |
  | watn | 33 | 11,865 | 69 | 32,620 | 6 | 14 |
  | kiekste | 107 | 81,217 | 258 | 187,259 | 40 | 98 |
  | **total** | **208** | **121,708** | **474** | **285,167** | **66** | **142** |

  A whole-project review is a real workload. One unit
  (`kiekste/src/workflow.rs`, 17,474 lines) is so large that its outline alone
  is a decomposition finding.

- **Rules under test (four):**
  1. **Outline comprehension** — a reader given only the unit's outline can
     state what the unit does and why its internal calls happen in that order.
  2. **Expressive names** — method names state intent, not a generic action.
  3. **Single responsibility** — each method does one thing; a method that
     mixes unrelated concerns or levels of abstraction is a finding.
  4. **Source-unit size** — a source unit stays at or below roughly 500 lines.
- **Finding ownership:** the agent produces candidate findings and recommended
  actions. The owner classifies each candidate as genuine or false positive and
  selects the three most severe findings per rule per project — up to 36
  findings across the three projects. Findings may overlap across rules, so the
  actual count can be lower.
- **Fix batch:** the agent applies the recommended action for every selected
  finding. Each distinct code change is one commit; a fix that resolves
  overlapping findings cites every rule it resolves; dependencies between fixes
  are recorded. Each commit applies cleanly to the pinned base and passes the
  project's build check; if the project's test command is available and
  reasonably fast, it passes that too.
- **Fix isolation:** each project gets its own Git worktree and a spike branch;
  the pinned main checkouts are not moved. Every fix is one commit in that
  worktree.
- **Owner judgment:** the owner inspects the worktrees and the diff, accepts or
  rejects each commit, and judges whether the code improved. The owner decides
  whether the check is promoted, kept opt-in, or retired. The spike does not
  merge its fixes into the pinned main checkouts and does not gate future
  changes on its own.
- **Output:** one report per project plus a cross-project summary. Each finding
  names the source unit, the rule, the outline evidence, the recommended action,
  the owner's classification, and the commit that applies it.
- **Scope of change:** only the selected fixes in the isolated worktrees. The
  review recommends; the agent applies only the selected recommendations; the
  pinned main checkouts and Givn's product behavior are unchanged.

## Scope boundary

In scope:

- The manual spike: the written procedure, the four rules, the review of the
  selected Rust source scope, the fix batch, the isolated worktrees, and the
  owner's judgment.
- The finding and recommended-action record, including prioritization and the
  owner's classification.
- The experiment protocol: evidence, trial boundary, and the
  promote/keep/retire decision that follows the owner's judgment.
- The treatment of existing oversized units within the selected scope.

Out of scope:

- Any Givn change: no addon, command, configuration key, instruction change,
  manifest change, or review gate. The opt-in product capability remains a
  later decision, taken only if the spike works.
- Making the check a default for every Givn project.
- Gating future changes before the experiment produces evidence.
- Merging the spike's fixes into the pinned main checkouts without the owner's
  judgment.
- Measuring LLM judge reliability, the false-positive rate across all findings,
  or outside-reader comprehension; these are deferred until the fixes show
  enough value to justify measuring them.
- Non-Rust languages; the spike is Rust-only.

## Research protocol

### What exists

- Lifecycle: proposal → spec → design → design-review → tasks → implement →
  review → archive. Review is an agent-run completeness and coverage review;
  `givn check review` enforces integrity, the run declaration, overlap
  dispositions, and the visual-review gate through markers in `review.md`.
  Migration evidence is a separate `givn check migration` check.
- Review instruction (`assets/instructions/review.md`): fabrication audit,
  coverage classification, arc42 conformance, ubiquitous-language conformance,
  and the visual review gate. No source-code readability criteria.
- Dev principles (`assets/instructions/dev-principles.md`): correctness first,
  "optimise for the reader six months out", boring when called for, remove
  obsolete code. No measurable outline, naming, responsibility, or size rule.
- Opt-in mechanism: addons are declared in `addons:` in `givn/config.yaml`,
  registered in `KNOWN_ADDONS` (`src/cli/addons.rs`), and condition instruction
  blocks through `{{#if addons.<id>}}` in `assets/instructions/*.md`. Today
  `givn init` enables every known addon by default (opt-out with `--no-addons`),
  so a default-off addon would be a new behavior. Project instruction
  extensions (`extensions:` in `givn/config.yaml`) are the existing non-addon
  way to add review rules without changing the binary.
- Permanent specs: the `verify-change` use case owns structural validity,
  completeness, overlap, review dispositions, and the visual-review gate. No
  capability owns source-code readability.
- Architecture: single feature-free Rust binary (ADR-069); the semantic and
  embedding review machinery was removed, so there is no in-binary model
  dependency. Agent harnesses already provide the LLM that runs the lifecycle.
- Target projects: measured on 2026-10-19 with `find ... -name '*.rs'` excluding
  `.git`, `target`, and `node_modules`; line counts are physical lines from
  `wc -l`. Production means outside a `tests/` directory; the Givn figure
  includes `harness/src` only in the all-Rust scope. The full inventory is in
  the Experiment definition.

### Gaps

- G1: Outline definition exists in `domain-terms.md`; the procedure that
  produces an outline without method bodies is open (elaboration).
- G2: No recorded readability verdict or gate; out of scope for the spike.
- G3: Expressive-naming and single-responsibility rules are not yet precise
  enough to apply consistently (elaboration).
- G4: No size counting rule (physical vs logical lines, comments, blank lines,
  tests, generated code) (elaboration).
- G5: No policy for existing oversized units beyond "split the worst first";
  Rust-only and production-source-only (Q8) (elaboration).
- G6: The spike tests owner-judged improvement of selected fixes. Judge
  reliability, noise rate, and outside-reader comprehension are deferred.
- G7: No default-off addon path; deferred with the Givn change.
- G8: Experiment protocol partly resolved by Q1-Q7; the judgment record and
  the promote/keep/retire record are open (elaboration).
- G9: No report format with findings and recommended actions (elaboration).
- G10: Scale resolved by Q3 and Q8: production sources only, with the three
  most severe findings per rule per project.
- G11: No severity scale for ranking findings within a rule (elaboration).
- G12: Diff isolation closed by Q6: worktree per project, one commit per
  finding.
- G13: The four rules have no precise definitions, thresholds, or exceptions
  (elaboration).
- G14: No owner-judgment record or post-judgment action (elaboration).
- G15: No written spike procedure an agent can execute without a Givn change
  (elaboration).

## Stakeholders

| Stakeholder type | Interest |
|---|---|
| Developer (owner of the three projects, outside-reader lens) | Wants to see whether the rules find real readability problems in code they already ship; wants each finding to come with a concrete fix; inspects the diff as a reader who did not write the code, classifies findings, decides which fixes to keep, and decides whether to adopt the check. |
| Reviewer / authoring agent | Produces the outlines and candidate findings, applies the selected fixes, and records the evidence; does not make the final readability judgment and must not fabricate understanding or recommend generic refactorings. |
| Givn maintainer | Must keep the single-binary, credential-free boundary; must not add an LLM dependency or a language parser without a justified scope; no product change happens before the spike's result. |

The external contributor / outside reader is not a separate persona in this
topic: the owner carries that lens when judging the diff (user decision
2026-10-19).
