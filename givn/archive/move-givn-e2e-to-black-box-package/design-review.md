# Design Review: move-givn-e2e-to-black-box-package

Review kind: independent
Author: move-givn-e2e-to-black-box-package hardening author (omp, deepseek-flash)
Reviewer context: the grilling context of 2026-11-16 whose five ranked questions and five blockers this authoring context applied; the same context owns the final review, so `review.md` must record the same kind and this context identifier.

The proposal records `Review kind: independent` and its reason: the change moves
the repository's acceptance authority, changes which command proves the `@e2e`
corpus, and adds a workspace manifest that outlives code replacement, so the
author of the move cannot sign off the suite that judges the move. This review
read `proposal.md`, both delta features, `usecase.md`, `design.md`, `arc42.md`,
the affected durable docs, and the cited product and test code. It did not edit
`tasks.md`, `review.md`, or `src/**`.

## Grilling questions

The reviewer produced five ranked questions with recommended answers. The
operator's delegation carried those recommendations as answers; each is recorded
below with the rejected alternatives and the line where it landed.

**Q1 (Tech choices) — does `./run-tests.sh --e2e` name the package?**
No. `names_root` (`src/e2e_boundary/mod.rs:428-441`) matches a token equal to
the root or a token containing `<root>/`; `--e2e` matches neither, so the old
design (`design.md:68` and `:131` before hardening) claimed a form the product
refuses. Resolution: the command becomes `./run-tests.sh --e2e --package e2e`.
`names_root` accepts the `--package` value because that value is a whole token
equal to the root; the rule needed no product change, and review finding F1
deleted the dead `--package` branch that had been added for it. The existing
whole-token and `<root>/` forms stay
accepted, so no consumer-visible refusal changes. `--e2e` alone names the tier,
not the package. Rejected: accepting `--e2e` as evidence (it names no package);
narrowing the whole-token form (it would refuse positional root tokens projects
use today, for no acceptance gain). Landed at `design.md:148-154`, `:228-244`,
`usecase.md:111`, `docs/reference.md:164-172`, the accepted-command Given
(`specs/verify-change/givn-workflow-enforcement.feature:7`), and the modified
outline row (`:26`).

**Q2 (Testability) — does the package read a relative `GIVN_FEATURES`?**
No. The package's working directory is `e2e/`, so `givn/specs` resolves under
`e2e/`. `givn check tasks` already passes an absolute delta path
(`src/cli/check.rs:489-497`), but `givn archive` passes `givn/specs`
(`src/cli/archive.rs:1769`) and `measure-coverage.sh` does the same.
Resolution: `run-tests.sh` canonicalizes `GIVN_FEATURES`, `GIVN_RESULT_FILE`,
and the binary directory before running the package; callers keep their
relative paths. The design's manual single-scenario command uses `$PWD`
(`design.md:440-448`). Rejected: requiring absolute paths from every caller,
which changes the product's runner contract for every project. Landed at
`design.md:235-244`, `:525-536`, `docs/reference.md:260-262`.

**Q3 (Risk) — does the plan prove the package links no product crate?**
Not by itself. The manifest reader is static and admits a dependency without a
`path` (`src/e2e_boundary/manifest.rs:45-58,86-99`), so a registry, git, or
renamed `givn` edge would pass the change check. Resolution: the package runner
reads `e2e/Cargo.lock` and exits 2 when a package named `givn`, `givn-config`,
or `givn-core` appears, on every E2E invocation; the reader's blindness is a
named limit with risk row `R-159`. Rejected: a one-time qualification script,
because a later dependency addition would never be checked. Landed at
`design.md:245-254`, `:377-380`, `:537-539`, `:597-601`,
`docs/reference.md:170-174`, `docs/arc42/11-risks-and-technical-debt.md:144`.

**Q4 (Testability) — how does coverage instrument a binary the package only
receives by path?**
If the runner rebuilds, the instrumented binary is replaced. Resolution:
`run-tests.sh --e2e` uses a preset executable `GIVN_BIN` and profile directory
instead of rebuilding, and `measure-coverage.sh` builds with
`cargo llvm-cov --no-run --no-report --locked --bin givn`, exports that path and
`LLVM_PROFILE_FILE`, and calls the same runner. Rejected: a package-side
coverage flag, because the package must stay runnable without llvm-cov. Landed
at `design.md:255-262`, `:575`, `:585-588`, `docs/reference.md:274-276`.

**Q5 (Missing scenarios) — is the rewrite set complete?**
No. The design stated "nine step files" and "four helper steps". Measurement on
2026-11-16 (bind every `@e2e` step phrase to its `#[given|when|then]`
attribute, then read each bound body for `givn::` or a module-scope
`use givn::…`) gives **23 step definitions across 18 scenarios in 11 files**,
plus three `tests/steps/mod.rs` helper sites, four product-calling support
modules, and three copied `tests/common/` helpers. Resolution: the inventory
lists every binding with its rewrite. Rejected: keeping a count without the
list, and keeping the "four" figure. The grill's 22 counted the two adjacent
`readme_enforcement_steps.rs` definitions (`:462` and `:485`, the cited
`:465-490` range) as one entry; the measured 23 is the recorded figure, so the
design states no undercount. Landed at `design.md:34-48`, `:292-362`.

## Branch walk

| Branch | Finding | Disposition |
|---|---|---|
| Scope | The delta contains exactly the proposal's package acceptance and the typed runner-naming rule; no capability is invented. `givn/specification.yaml` is written by the change, as `D2` decides. | No issue. |
| Tech choices | cucumber-rs 0.23, tokio, serde_json, tempfile, `gherkin` 0.16, `serde_yaml_ng` 0.10, Playwright 1.64.1 (read from `tests/e2e/package.json`), Node from `PATH`, `/bin/sh`. Each version comes from the repository lockfile or is left unpinned. | No issue; the two added rows name the parser and YAML crates the rewrites need (`design.md:182-183`). |
| Missing scenarios | The refusal of a tier flag was absent from the delta; the accepted command form was not pinned. The modified outline adds `./run-tests.sh --e2e` and the added scenario pins the typed command. | Fixed (`feature:6-26`). |
| Testability | The new Given is unbound, so every delta scenario fails RED before implementation; the single-scenario command is exact; `.fail_on_skipped()` plus the zero/failed/skipped exit rules are named (`design.md:473-484`). | No issue. |
| E2E fidelity | The delta adds no `@e2e` scenario and removes no tag; the moved corpus keeps its titles and tags. The browser tier keeps its real driver. | No issue. |
| Interaction Coverage | The delta adds no inventory entry. The design's matrix covers all 14 entries of the owning `usecase.md` `## Interactions` table one-to-one. | No issue. |
| Risk | Registry/git edge admitted by the static reader; uninstrumented `GIVN_BIN`; mirrored slug and narratives; second lockfile; archived evidence rewrite; qualification cost. | Each has a named mitigation and a risk row (`R-155` to `R-159`). |
| Use-case and Persona context | `verify-change` owns both capabilities; `adr-qualification-gate` is extended by count only. Confirmed Personas `agent--technical-role` and `givn-maintainer--owner`; no invented Persona and no biography in Gherkin. | No issue. |
| Decision ledger | `D1` is carried with default, trigger, switch cost, and owner; `D2` and `D3` are decided with their realized artifacts. The four hardening decisions are recorded beside them. | No issue. |
| Contracts and components | One contract, and the table now names one implementing component (`e2e/tests/e2e_runner.rs`); the project scripts are consumers. The contract uses the consumer's variable names. | Fixed (`design.md:153`). |
| Decision timing | No foundational question is carried; no scenario depends on `D1`; the contract is current, not deferred. | No issue. |
| Visual design contract | Present with `Interface: terminal`, viewports and states marked not applicable, and the terminal baseline stated. | No issue. |
| Deferrals | None requested and none recorded. | No issue. |
| Persona disposition | Two confirmed Personas with per-persona impact rows; the change visibly changes CLI text and exit codes, so a confirmed Persona is required and present. | No issue. |
| Interface classification | Terminal is correct: the change alters CLI text, exit codes, and the receipt. The opposite (Web UI) does not hold: no rendered surface changes, and the browser is an existing test asset whose screenshots the move does not alter. | No issue. |
| ADR qualification | One qualified candidate, `NEW_ADR` -> ADR-079, with the cheaper home named (`arc42.md:29`); rejected candidates name their owners. Chapter 11 carries the consequences. | Fixed (cheaper home added). |
| Architecture documentation | Independent 12-row derivation below matches `arc42.md`; all 12 chapter files exist with content; no box-drawing or pseudo-diagram artifacts; ADR-079 is in the register and chapter 11 carries its consequences. | No issue. |
| Ubiquitous language | `E2E package` is recorded in `docs/glossary.md` and names the new unit; `Source code`, `E2E identity`, and `Frozen package` keep their recorded meanings; `component tier` is defined as the anti-term inside the `E2E package` entry and is used with that meaning. No competing noun is introduced. | No issue. |
| Structural model | `docs/domain-model.md` already excludes the `e2e` package from the behavior aggregate; the change moves a repository test asset and alters no domain boundary or relationship, so no model edit is required and none is claimed. | No issue. |

## Blockers and their closure

1. **Runner naming.** Closed by Q1. `design.md:148-154`, `:228-244`,
   `:84`; `docs/reference.md:164-172`; `givn/commands.yaml:4`. The rule needed
   no product change: the whole-token form already accepts the `--package`
   value equal to the declared root. Review finding F1 closed a dead
   `--package` branch and its three unit tests; `design.md`, this record, and
   `tasks.md` batch 2 state the command satisfies the rule as a token.
2. **ADR register count.** `docs/arc42/adr/README.md` carries 31 accepted rows
   and the ADR-079 row. The permanent scenario and its binding agreed on 30 and
   went red (`left: 31, right: 30`). Reconciled: permanent
   `givn/specs/fragments/adr-qualification-gate.feature:123` asserts 31;
   `tests/steps/adr_qualification_gate_steps.rs:881-912` asserts 31 and requires
   the ADR-078 and ADR-079 rows to be accepted; the delta `@givn.modified`
   scenario asserts the same
   (`specs/fragments/adr-qualification-gate.feature:10`). Both scopes green
   before the move: the permanent fragment scope passed 18/18 scenarios, the
   delta `@givn.modified` scenario passed 1/1, and the non-e2e suite passed
   1081/1081 (the 114 skipped are the excluded `@e2e` scenarios). After the move
   the same tag run splits: 17 scenarios in the component tier
   (`@adr-qualification-gate and not @e2e`) and 1 in the package
   (`@e2e and @adr-qualification-gate`), measured in `tasks.md` batch 6.
3. **Migration inventory.** Closed by Q5; the design states 23, the measured
   count, and lists every binding.
4. **Relative `GIVN_FEATURES`.** Closed by Q2. `design.md:440-448` passes an
   absolute path; `src/cli/archive.rs:1769` keeps `givn/specs` because the
   runner canonicalizes it; no product change.
5. **No-product-dependency claim.** Closed by Q3. `design.md:245-254`,
   `:377-380`; the limit is in `docs/reference.md:170-174`; risk row
   `R-159`.

## Independent Arc42 derivation

| # | Chapter | My read | `arc42.md` | Agreement |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No: no goal, stakeholder, or quality attribute changes | No | Yes |
| 2 | 02 architecture-constraints | Yes: TC-52 records the package, typed command, and lockfile guard | Yes | Yes |
| 3 | 03 context-and-scope | No: the product's external interface is unchanged | No | Yes |
| 4 | 04 solution-strategy | Yes: two test tiers and a named acceptance package | Yes | Yes |
| 5 | 05 building-block-view | Yes: the `e2e` package and the renamed component tier | Yes | Yes |
| 6 | 06 runtime-view | Yes: the acceptance flow runs two packages | Yes | Yes |
| 7 | 07 deployment-view | Yes: browser assets move; the package is absent from the installed product | Yes | Yes |
| 8 | 08 crosscutting-concepts | Yes: 8.31 records the package shape and runner contract | Yes | Yes |
| 9 | 09 architecture-decisions | Yes: ADR-079, register count 31 | Yes | Yes |
| 10 | 10 quality-requirements | No: no new quality scenario or metric | No | Yes |
| 11 | 11 risks-and-technical-debt | Yes: `R-86` path, `R-155` to `R-159` | Yes | Yes |
| 12 | 12 glossary | Yes: `E2E package` in `docs/glossary.md` | Yes | Yes |

Chapter files: all 12 exist under `docs/arc42/` with content. The box-drawing
scan over `docs/arc42/*.md` returned no match. ADR-079 is a MADR file with
accepted frontmatter, present in both `docs/arc42/adr/README.md` and chapter 09;
its "Bad, because" consequences are `R-157` (evidence rewrite) and `R-159`
(reader blindness), both actionable.

## Hardening edits

| File | Edit |
|---|---|
| `design.md` | 34-48 measured inventory; 143-147 command; 153 single implementing component; 155-159 product-code statement; 182-183 parser and YAML dependencies; 222-262 four decisions; 292-362 full rewrite inventory; 377-386 boundary cases; 410-412 step phrases; 425-448 runner commands; 525-539 smoke-test infrastructure; 575 coverage row; 585-601 risks. |
| `specs/verify-change/givn-workflow-enforcement.feature` | 6-26: typed command in the Given, modified runner-naming outline with the `--e2e` row. |
| `specs/verify-change/usecase.md` | 31-32 typed rule; 79 canonical refusal; 111 rule; 151-152 examples. |
| `specs/fragments/adr-qualification-gate.feature` | Unchanged at 31; verified against the permanent scenario. |
| `givn/specs/fragments/adr-qualification-gate.feature` | 123 count 31. |
| `tests/steps/adr_qualification_gate_steps.rs` | 881-912 count and rows. |
| `givn/commands.yaml` | 4 typed command. |
| `docs/reference.md` | 164-177 runner-target rule and static-reader limit; 247 and 260-276 acceptance package and coverage; 672, 684 script blocks. |
| `docs/arc42/02-architecture-constraints.md` | 60 TC-52. |
| `docs/arc42/06-runtime-view.md` | 684-689 acceptance flow. |
| `docs/arc42/08-crosscutting-concepts.md` | 1639-1648 package and preflight. |
| `docs/arc42/11-risks-and-technical-debt.md` | 141 `R-156`; 144 `R-159`. |
| `docs/arc42/adr/ADR-079-standalone-acceptance-package.md` | 94-110 consequences and confirmation. |
| `arc42.md` | 17 risk range; 29 cheaper home. |

## Verification

```text
$ givn lint --change move-givn-e2e-to-black-box-package
givn lint: 2 file(s) checked, 3 finding(s)   # three @wip scenarios, contract permits exit 2

$ GIVN_FEATURES=givn/specs/fragments cargo test --locked --test features_runner -- --tags "@adr-qualification-gate"
18 scenarios (18 passed); 98 steps (98 passed)
  including 'The migration preserves independent architecture history across three proposed decisions'
  # before the move; after it the tag run splits 17 component (91 steps) + 1 package (7 steps)

$ GIVN_FEATURES=givn/changes/move-givn-e2e-to-black-box-package/specs cargo test --locked --test features_runner -- --tags "@adr-qualification-gate"
1 scenario (1 passed); 11 steps (11 passed)   # the delta @givn.modified scenario

$ ./run-tests.sh --non-e2e
Summary [ 207.481s] 1081 tests run: 1081 passed, 114 skipped; exit 0
  # the 114 skipped are the @e2e scenarios the non-e2e scope excludes

$ grep -rn 'four bindings\|nine step files\|108\|--e2e` alone' givn/changes/move-givn-e2e-to-black-box-package/design.md
givn/changes/move-givn-e2e-to-black-box-package/design.md:313:| `development_report_e2e_steps.rs:1082` | ...
  # the only match is a line number in the binding table; no undercount and no old command form
```

The three `@wip` scenarios are the added acceptance scenario and the two
`@givn.modified` scenarios; their step definitions land in the implementation
tasks, so lint exit 2 is the expected state before tasks exist.

Every grilling question is resolved, every blocker is closed, and no branch
carries an open finding. The plan is ready for `tasks.md`.

DESIGN-REVIEW: PASS
