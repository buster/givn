# Design review: release-truth

## Review boundary and provenance

FreshPlanGrill performed the mandatory independent fresh-context grilling
before this hardening. Its complete 35 KB result (`agent://FreshPlanGrill`),
not a preview, is the retained handoff: two questions (Q1, Q2), findings
F1–F17, verified facts V1–V6, the independently derived 12-row arc42
assessment, and the verdict `sign_off: false` with blocking findings
F1, F2, F3, F5, F6, F9.

This hardening read that complete result, the full planning layer
(`proposal.md`, `design.md`, the delta feature, the delta use case), and the
effective design-review, specs, design, arc42-docs, and dialog instructions,
plus the named repository mechanisms: `.github/workflows/{release,
prepare-release,ci,security}.yml`, `cliff.toml`, `run-tests.sh`,
`tests/features_runner.rs`, `tests/common/mod.rs`, `src/gherkin/interaction.rs`,
`givn/commands.yaml`, and the ADR register.

Boundary: only planning artifacts, the delta spec, and the delta narrative
are changed here. No tasks, no production implementation, no test glue, no
scenario execution. A plan PASS is not delivery, behavioral GREEN, or archive
evidence.

## Grilling log index

- Fresh independent grilling source: `agent://FreshPlanGrill`, complete result.
- Questions asked of the user: **1** (Q2 below).
- Full question/answer records: the operator record below.
- Approved direction, not re-asked: adopt the watn-style release flow for
  givn without crates.io publication, as one workflow, with a changelog that
  lists each user-visible change once (user instruction, 2026-10-04).
- Codebase-resolved findings: F1–F17 and V1–V6 below.

## Operator question record

### Q2 — How the rewritten release workflow is verified

- Status: closed 2026-10-04 (user, option B)
- Blocks: design-review sign-off; whether the guard predicates become a
  checked-in script with scenarios or stay reviewed YAML.
- Context: the change's central deliverable is the single release workflow
  (base guard, empty-section guard, identity revalidation, idempotent
  publication). The delta scenarios covered only `cliff.toml`; the workflow
  itself had no executable check. `security.yml` runs actionlint and zizmor
  on workflow changes — syntax and security, not the guard logic.
- Question: how deep should the workflow's verification go?
- Options:
  1. **Diff review plus a throwaway local simulation** — the workflow stays
     YAML; the two guards run once locally against fixtures and the output is
     recorded. Gain: no new permanent artifact. Cost: wiring, permission, or
     ordering defects surface at the first real release, possibly after a
     partial publication. Example: a misspelled `needs.prepare.result`
     condition creates no release until someone repairs it manually.
  2. **Guards as a script plus one scenario each** — `base` and `section`
     predicates in `scripts/release-guard.sh`, driven from real fixture
     repositories and asserted by Gherkin. Gain: the release's only automated
     truth protection becomes RED/GREEN-proven and regression-covered.
     Cost: one script, two scenarios, step bindings, and two more task
     commits; wiring stays reviewed, not executed. Example: a fixture with
     package version 0.8.0 and no `v0.8.0` tag makes `release-guard.sh base`
     exit non-zero and name the missing tag.
  3. **All inline logic as scripts plus scenarios** — also the version
     checks, identity validation, and changelog merge. Gain: maximum
     mechanical coverage. Cost: doubles the release-tooling surface and
     re-implements GitHub-side behavior (annotated tags, reachability)
     locally. Example: an identity script that must distinguish annotated
     from lightweight tags and needs both fixtures.
- Resolution: **Chosen option 2.** The guards are the release's only
  automated truth protection, and the repository's own standard is an
  executable specification for behavior; two predicates are a proportionate
  price. Rejected option 1: it defers the one automated guard behind a manual
  note. Rejected option 3: it copies GitHub behavior locally and doubles the
  surface for little added signal. The decision's bound is recorded in
  design.md A2: the remaining job wiring stays verified by diff review,
  actionlint, and zizmor.

### Q1 — Release-note identity used for dedupe (risk; settled by the direction)

- Status: settled 2026-10-04 by the approved direction to adopt the
  watn-style selection rules; recorded as an accepted limitation, not asked.
- Subject: the dedupe filter compares the conventional-commit description, so
  two changes whose release-note sentences are identical collapse into one
  entry; because the filter applies inside each group, a change whose
  revisions carry both a `feat` and a `fix` subject with the same description
  lists once per group.
- Alternatives recorded: a uniqueness token (change id) in the release-note
  convention — gain: the collapse becomes impossible; cost: a guidance and
  skill change beyond this change's scope plus reformatting of future notes;
  rejected. A composite key through a git-cliff message preprocessor — gain:
  distinct sentences survive; cost: new preprocessor machinery, harder
  review; rejected.
- Disposition: keep exact-subject dedupe with `case_sensitive=true`, which
  removes the silent case-insensitive collision class found in F13. Both
  remaining collisions are convention violations (every revision of a change
  shares the release note as its subject), review-visible, and registered in
  `docs/arc42/11-risks-and-technical-debt.md`.

## Findings and hardening applied

| ID | Finding | Disposition |
|---|---|---|
| F1 | The empty-range scenario had no RED at its own commit; it shared the parser change with the group scenario, and its Given even carried the uncategorised revision both needed | Fixed: the parser change is split — S1 owns the internal-scope fix skip and the `.*` fallback; S2 owns the `^docs`/`^refactor` skip and its range holds only skipped types. Each scenario now has its own behavioral RED (design.md §Fixture and assertion design) |
| F2 | The rewritten workflow had no executable verification; the deferral was not an operator decision | Fixed by the operator decision (Q2, option B): `scripts/release-guard.sh` plus one scenario per guard; the remaining wiring's verification bound is recorded in A2 |
| F3 | The workflow's authorization surface (permissions, push credential, concurrency) was undecided | Fixed: A8 decides `RELEASE_PUSH_TOKEN` for the push and records the rejected `GITHUB_TOKEN`; D1 names the permissions blocks and the concurrency group |
| F4 | Proposal item 1's carried version checks (newer-than-manifest, tag-absent) were absent from the design | Fixed: A3 names all three checks the prepare job keeps |
| F5 | The allowlist leaked `fix(harness)` test-harness fixes into Bug Fixes, while the added `fix(e2e)` rule matched zero commits | Fixed: A1 skips the internal scopes (`harness`, `tests`, `e2e`, `ci`, `givn`, `release`, `specs`); S1's range contains a `fix(harness)` revision and asserts its absence |
| F6 | S1's Gherkin did not assert the five positive group headings its design claimed | Fixed: S1 asserts the `Breaking Changes`, `Features`, `Bug Fixes`, `Performance`, and `Reverts` groups |
| F7 | ADR rows named multiple canonical destinations and omitted the gate verdicts | Fixed: the classification table records the five gate verdicts and exactly one `CANONICAL_ARTIFACT` destination per candidate |
| F8 | Arc42 chapter 11 was missing from the durable-update set | Fixed: chapter 11 joins the update set; the dedupe limitation and the base-tag rollout dependency are register entries |
| F9 | The proposal's routing rationale ("no signal, top candidates tie") was contradicted by the actual route output | Fixed: the proposal records `EXTEND apply-a-managed-upgrade` (17.00, signal) and the deviation rationale |
| F10 | The cargo-release pin was attributed to a workflow that does not install it | Fixed: the technology table names `prepare-release.yml` as the pin source and the rewritten workflow as its new home |
| F11 | The ci.yml rationale said the delta scenarios run in CI; they run under `GIVN_FEATURES` in the gates | Fixed: A5 now scopes the install to the permanent scenarios that CI runs after archive |
| F12 | The design quoted an unquoted `!cancelled()` `if:` expression | Fixed: the expression is written with `${{ }}` |
| F13 | The dedupe wording said "byte-identical" while Tera's `unique` is case-insensitive by default | Fixed: the filter uses `case_sensitive=true`, probed 2026-10-04; the wording states exact-subject matching |
| F14 | The glossary update was not enumerated | Fixed: the design names `Changelog`, `Release section`, `Release base tag`, and `Release revision` |
| F15 | The visual contract lacked a Motion rule and named prose as its rendered reference | Fixed: a Motion bullet (not applicable) and the committed `@e2e` transcript as the rendered reference |
| F16 | The per-group mirror case (one change listed once per group) was undocumented | Fixed: recorded as an accepted limitation in A1 and chapter 11 |
| F17 | S5's fixture did not constrain the shared-note revisions to one grouped type, allowing a false GREEN | Fixed: S5 names three feature revisions of one change, and the design states the required subject shape |

Verified facts (no action): V1 no durable document names `prepare-release.yml`
or `release.yml`; V2 the interaction gate genuinely forces the use-case
inventory row, and the planned row satisfies it; V3 the single-scenario
commands work in both runner modes (the nextest form is now recorded); V4 the
four-regular/one-`@e2e` split matches the canonical E2E scope policy; V5 the
delta narrative's base hash is current; V6 the fixture assumptions about
git-cliff 2.13.1, its stderr logging, and the bundled Tera filter hold.

## Derived arc42 chapter assessment (independent)

| Chapter | Affected | Reason |
|---|---|---|
| 01 introduction-and-goals | No | No product goal, stakeholder, or quality attribute changes |
| 02 architecture-constraints | No | No constraint on the product or binary; the release channel is a deployment fact |
| 03 context-and-scope | No | No product boundary or external entity changes |
| 04 solution-strategy | No | Configuration and workflow rules, not an architectural direction |
| 05 building-block-view | No | `src/**` is untouched |
| 06 runtime-view | No | No product runtime flow changes |
| 07 deployment-view | Yes | 7.5 currently describes an intended pipeline (release binaries, Homebrew, shell installer, cargo publish); the change records the actual single dispatch workflow, GitHub releases as the only channel, and the selection rules |
| 08 crosscutting-concepts | No | No product-level crosscutting concept changes |
| 09 architecture-decisions | No | No candidate qualifies; all route to canonical artifacts |
| 10 quality-requirements | No | No new product-level quality scenario |
| 11 risks-and-technical-debt | Yes | Durable accepted limitations: exact-subject dedupe can merge two changes, per-group application can list one change twice, and the base guard refuses releases until `v0.8.0` exists |
| 12 glossary | Yes | New durable vocabulary for the release pipeline |

The design's durable-update set (07, 11, 12) matches this independent
derivation.

## ADR qualification verdicts

| Candidate | alternatives | architectural_impact | durable_consequence | lower_level_artifact | existing_adr_check | Routing | Destination |
|---|---|---|---|---|---|---|---|
| A1 selection rules | PASS | FAIL | FAIL | PASS | PASS | `CANONICAL_ARTIFACT` | `cliff.toml` |
| D1 dispatch-only release | PASS | FAIL | PASS | PASS | PASS | `CANONICAL_ARTIFACT` | `docs/arc42/07-deployment-view.md` |
| D2 base-tag guard | PASS | FAIL | PASS | PASS | PASS | `CANONICAL_ARTIFACT` | `givn/specs/publish-a-release/usecase.md` |
| A2 guard script | PASS | FAIL | FAIL | PASS | PASS | `CANONICAL_ARTIFACT` | `scripts/release-guard.sh` |

No `NEW_ADR`; no `AMEND_ADR` or `SUPERSEDE_ADR` target exists in the register.

## Deferrals

None. The single deferral-shaped item — workflow verification depth — was put
to the operator and decided (Q2, option B); the residual wiring verification
bound is a recorded decision, not a silent deferral.

## Sign-off

- [x] Every required branch walked: scope, tech choices, missing scenarios,
      testability, risk, use-case and Persona context, decision ledger,
      visual contract, interface classification, ADR qualification, arc42
      documentation, deferrals.
- [x] Every proposal question (D1, D2) resolves in design.md's `## Decisions`
      ledger with a realization site.
- [x] Every blocking grilling finding is fixed; every verified fact is
      retained.
- [x] No fabricated RED: all five scenarios carry a RED recipe, including the
      two guard scenarios' absent-behavior record.
- [x] Interface classification tested against the opposite (no browser
      renderer; no HTML/CSS/JS) and confirmed.
- [x] Persona coverage: `givn-maintainer--owner`, already confirmed by the
      owning use case; no invented persona, no persona used as an actor.

DESIGN-REVIEW: PASS
