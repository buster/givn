# Review: release-truth

README-IMPACT: none

The README's release statements (source-only install, no crates.io
publication or release binaries, `CHANGELOG.md` as the release history)
remain true; the change adds no README section.

## Fabrication Audit (do this first)

**Step definition scan** — every touched step-definition file checked for
empty/no-op bodies, `unimplemented!()`, or `todo!()`:

```
$ grep -n 'fn .*{ *}\|unimplemented!\|todo!()' tests/steps/release_steps.rs tests/steps/release_e2e_steps.rs
(no matches)
```

The two temporary `unimplemented!()` stubs existed only inside the E2E setup
task and were replaced by the dedupe scenario's real assertions in commit
`1cf43a50`; the repository-wide `grep -rn 'unimplemented!\|todo!' tests/steps/`
returns no matches.

**Commit cross-check** — every checked `[x]` task in tasks.md has a matching
commit:

| Task | Commit | Touches production? |
|---|---|---|
| Planning artifacts (proposal, design, design-review, arc42, delta, tasks) | `1eb3e2d5` | docs/arc42 chapters + change artifacts; see disposition |
| Setup: strictness proof, baseline, command forms | `760a3872` | evidence only; see disposition |
| A release section lists only the user-visible change groups | `b5f91a1c` | yes: `cliff.toml` |
| A release range without user-visible change has no release section | `4071f575` | yes: `cliff.toml` |
| A release cannot be prepared without a tag for the current version | `5ddbad58` | yes: `scripts/release-guard.sh` |
| A release section without user-visible change stops the release | `966aa7ef` | yes: `scripts/release-guard.sh` |
| E2E setup: e2e scope, skeleton, and scenario-count proof | `654f0b89` | test bindings; see disposition |
| Repeated revisions of one change yield one changelog entry | `1cf43a50` | yes: `cliff.toml` |
| Release workflow and CI | `68893341` | yes: `.github/workflows/*` |
| Durable documentation | `df2a04bd` | yes: `AGENTS.md` (chapters were in `1eb3e2d5`) |
| Review sign-off | this commit | review artifact only; see disposition |

Dispositions for the non-production commits:

- Planning artifacts: the change's own workflow requires proposal → specs →
  design → design-review → tasks before implementation; the durable chapter
  updates (`docs/arc42/07`, `11`, `12`) are part of that planning commit
  because the arc42-docs artifact precedes the task list.
- Setup and E2E setup: runner strictness, scope counts, and command-form
  proofs produce evidence, not product behavior; the E2E setup creates the
  step module the later scenario fills.
- Review sign-off: the review artifact is the gate this commit records.

**Promised-architecture cross-check** — every component design.md said would
exist:

| Component (design.md §Production edit set, §Step definitions) | Exists? | Path |
|---|---|---|
| Selection rules | yes | `cliff.toml` |
| Guard predicates (`base`, `section`) | yes | `scripts/release-guard.sh` |
| Single dispatch workflow | yes | `.github/workflows/release.yml` |
| Split workflow removed | yes | `.github/workflows/prepare-release.yml` absent |
| CI changelog-tool install | yes | `.github/workflows/ci.yml` |
| Regular step module | yes | `tests/steps/release_steps.rs` |
| `@e2e` step module | yes | `tests/steps/release_e2e_steps.rs` |
| Interaction inventory row | yes | `givn/changes/release-truth/specs/publish-a-release/usecase.md` |
| Durable chapters | yes | `docs/arc42/{07,11,12}` |
| Acceptance prerequisite | yes | `AGENTS.md` |
| No `src/**`, `Cargo.toml`, `Cargo.lock`, `givn/config.yaml` change | confirmed absent | `git diff --stat 1eb3e2d5^..HEAD` names none of them |

**Strict-mode proof** — present with output in tasks.md (setup task):

```
$ GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --non-e2e
  exit 100; "4 tests run: 0 passed, 4 failed, 1 skipped"; "Step doesn't match any function"
$ GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --e2e
  exit 100; "1 test run: 0 passed, 1 failed, 4 skipped"
$ GIVN_FEATURES=givn/changes/release-truth/specs cargo test --locked --test features_runner -- \
    --exact 'publish-a-release/describe-the-release.feature::A release section lists only the user-visible change groups'
  exit 101; "1 scenario (1 failed)"; "Step doesn't match any function"
```

E2E strictness after the skeleton existed: the stub panicked
(`not implemented: implemented by the dedupe scenario task`, exit 100) instead
of passing.

**Downgraded e2e scenario scan** — the `@e2e` scenario's Then steps assert on
the real CLI output:

| `@e2e` scenario | Real-interface assertion | Driven via a real driver? | Downgraded? |
|---|---|---|---|
| Repeated revisions of one change yield one changelog entry | yes — the release-note occurrence count and the absence of a documentation entry in the stdout of the real `git-cliff` 2.13.1 binary with the workspace `cliff.toml` | yes — `Command::new("git-cliff")` subprocess over a temp git repository (CLI is the interface; no browser involved) | no |

**E2E scope check** — one `@e2e` scenario per distinct action, not per
variant:

| Capability | Inventory actions | `@e2e` scenarios | Distinct actions? | Over-produced? |
|---|---|---|---|---|
| describe-the-release | 1 (generate the release section for a range of changes) | 1 | yes | no |

The four regular scenarios assert distinct invariants (group selection, the
empty-range boundary, the base-tag guard, the section guard), not a second
driver of the `@e2e` action.

**Duplicate-implementation check** — the exact file(s) `verify.e2e_command`
invokes and the tree search for a parallel implementation:

| Capability | `verify.e2e_command` path | Other implementation found? | Stronger one used? |
|---|---|---|---|
| describe-the-release | `./run-tests.sh --e2e` → `cargo nextest run --test features_runner` with `GIVN_CUCUMBER_TAGS=@e2e`; bindings in `tests/steps/release_e2e_steps.rs` (plus the shared `When` in `release_steps.rs`) | no — `git status` clean, no other release binding files | N/A |

**Local environment check** — no service, port, database, or digital twin is
involved; the fixtures are tempfile repositories and a local binary. The four
scope runs are recorded under "Scenario execution evidence" below.

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`:

| `verify.command` | `verify.e2e_command` | Identical? | Delta counts | Permanent counts | E2e strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 4 regular / 1 e2e | 578 regular / 88 e2e | yes (1 < 5; 88 < 578) |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case rules realized | `publish-a-release` rules: "a release section lists only user-visible change … one entry per change, taken from the change's release note" (S1, S2, S5) and "a release starts at a version tag that matches the package version; a range without user-visible change produces no release" (S2, S3, S4) | yes |
| Capability ownership and typed relationships preserved | `describe-the-release` stays with `publish-a-release`; no `Extends` added; the fragment include is unchanged | yes |
| Confirmed Persona-relevant outcomes preserved | the maintainer reads a release section with one entry per change and a refusal that names the missing tag or the empty section | yes |
| Actors remain distinct from Persona biographies | Gherkin uses the maintainer as the domain role; `givn-maintainer--owner` is the review lens only | yes |
| Interaction inventory updated | one row: `describe-the-release` — generate the release section for a range of changes — `Repeated revisions of one change yield one changelog entry` | yes |

PERSONAS: givn-maintainer--owner

Also reviewed as a lens, recorded evidence: `external-contributor--open-source-consumer`
(release notes are the only place that persona learns what changed; the
stricter selection rules remove documentation and test-harness noise).

## Deferred mandatory checks

None deferred. The one deferral-shaped item — how the rewritten workflow's
non-guard wiring is verified — was put to the operator during design-review
and decided (option B: guards as a script with one scenario each; the
remaining YAML covered by diff review, actionlint, and zizmor).

## Design.md conformance check

| Decision (design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| D1 dispatch-only | `workflow_dispatch` with `version`/`tag`, no `push: tags`, release job runs after prepare | exactly that in `.github/workflows/release.yml` | yes |
| D2 base guard | newest tag must equal `v<current version>`, realized via `release-guard.sh base` before the bump | `scripts/release-guard.sh base "$CURRENT_VERSION"`; scenario S3 | yes |
| A1 selection rules | allowlist, internal-scope fix skip, exact-subject dedupe | `cliff.toml` parsers plus `unique(attribute="message", case_sensitive=true)`; scenarios S1, S2, S5 | yes |
| A2 guards as a script | `base` and `section` subcommands; `section` called after the merge and in the release job | implemented and called twice; scenarios S3, S4 | yes |
| A3 one job graph, carried checks, no repeated suite | `prepare` → `release`; version checks kept; release job validates and publishes | implemented; the smoke asserts all three checks and the job graph | yes |
| A4 idempotent release job | `gh release view` short-circuit | implemented | yes |
| A5 CI installs the pinned tool | cache plus conditional install in `ci.yml` | implemented | yes |
| A6 interaction inventory row | one row replacing `- none` | shipped in the delta narrative | yes |
| A7 release notes from the section | notes file extracted from `CHANGELOG.md` with a group check | implemented | yes |
| A8 push credential | `secrets.RELEASE_PUSH_TOKEN`, workflow `contents: read`, release job `contents: write`, concurrency `givn-release` | implemented | yes |

No deviation was taken silently; the only post-review artifact correction is
this review's own commit, which changes no design decision.

## Interaction coverage verification

| Inventory entry | Matrix row exists? | `@e2e` scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| describe-the-release: generate the release section for a range of changes | yes | yes — `Repeated revisions of one change yield one changelog entry` | real `git-cliff` subprocess with the workspace `cliff.toml` over a fixture repository | `Command::new("git-cliff")` in `release_steps.rs::ChangelogFixture::generate`, asserted in `release_e2e_steps.rs` | yes |

## Visual Review

Interface: terminal (no Web UI; the generator prints text and writes Markdown
— no HTML, CSS, or JavaScript is rendered).

| `@e2e` Scenario | Evidence kind | Evidence file | Present and non-empty? |
|---|---|---|---|
| Repeated revisions of one change yield one changelog entry | transcript | `givn/changes/release-truth/evidence/visual/repeated-revisions-of-one-change-yield-one-changelog-entry/transcript.txt` | yes |

| Confirmed persona | Goal | Finding | Disposition |
|---|---|---|---|
| givn-maintainer--owner | read a truthful release section | one entry for the shared note, no documentation entry, group headings name their subject, no internal identifiers | none — passes |

**Baseline (terminal):** headings identify their subject; entries are complete
sentences with a bold scope prefix; no duplicated visible content (the
transcript shows one entry for three revisions); no color, motion, or
interactive control is introduced.

VISUAL-REVIEW: PASS

## Retirements

RETIRE: none — this change retires no capability and no scenario. It removes
one workflow file, not a catalogued capability.

## Arc42 implementation conformance

| Chapter or architectural fact | Durable source | `arc42.md` claim | `design.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|
| Release pipeline facts | `docs/arc42/07-deployment-view.md` | Yes | §Status, D1, A2 | `.github/workflows/release.yml`, `scripts/release-guard.sh` | match |
| Accepted limitations | `docs/arc42/11-risks-and-technical-debt.md` | Yes | A1, §Risks | R-75, R-76, amended D-2 | match |
| Release vocabulary | `docs/arc42/12-glossary.md` | Yes | §Status | four new terms | match |
| No new ADR | `docs/arc42/09-architecture-decisions.md` | Yes | ADR classification | no ADR file added; `assets/migrations` untouched | match |
| Untouched product building blocks | `docs/arc42/05-building-block-view.md` | No | §Production edit set | `git diff` names no `src/**` file | match |

- [x] Every chapter the independent derivation marked affected was updated.
- [x] No chapter claims a fact the implementation contradicts.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Coverage result:** no measured receipt exists yet; the archive measures it
against the committed candidate and records it in
`givn/archive/release-truth/verification.json`.

```
unproven candidate — no receipt yet. Static classification below; the archive
receipt will carry the measured line/branch percentages.
```

## Coverage gap classification

| Location | Kind | Classification | Action taken |
|---|---|---|---|
| `cliff.toml` | configuration | not executable code | asserted through the real generator by S1, S2, S5 |
| `scripts/release-guard.sh` | shell | covered | S3 and S4 drive both subcommands through `sh`; the successful and failing paths are asserted |
| `.github/workflows/*.yml` | CI configuration | not locally executable | structural smoke (31 shell run blocks syntax-checked, trigger/job/guard/idempotence assertions, no crates.io step) plus diff review; the verification bound is the recorded operator decision |
| `tests/steps/release_steps.rs`, `release_e2e_steps.rs` | test code | not production code | exercised by the four scope runs below |
| `docs/arc42/*`, `AGENTS.md` | documentation | not executable code | reviewed against the implementation in the design-conformance and arc42 tables |

## Scenario execution evidence

```
Delta scopes (same runner the archive invokes):
  GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh
    5 scenarios, 5 passed, 0 failed                                EXIT=0
  GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --non-e2e
    4 scenarios, 4 passed, 0 failed                                EXIT=0
  GIVN_FEATURES=givn/changes/release-truth/specs ./run-tests.sh --e2e
    1 scenario, 1 passed, 0 failed                                 EXIT=0
Permanent corpus:
  ./run-tests.sh            578 tests run: 578 passed, 0 skipped    EXIT=0
  ./run-tests.sh --e2e       88 tests run:  88 passed, 490 skipped  EXIT=0
```

**Receipt status:** unproven (candidate not yet archived)

## Overlap dispositions

The overlap gate's scan reports no shape match involving this delta, so its
required table is empty:

| Scenario A | Scenario B | Disposition |
|---|---|---|

`givn lint --change release-truth` is clean (1 file checked, 0 findings); no
advisory enumeration is required.

## Split-or-keep

The overlap gate reports no long-scenario finding for this delta, so no row is
required:

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, every checked task has a
      commit, every promised component exists, no downgraded e2e scenario, no
      over-produced e2e scope, e2e scope strictly smaller).
- [x] Strict-mode proof recorded in tasks.md.
- [x] Coverage: every changed region classified and resolved.
- [x] Design conformance verified decision by decision.
- [x] Use-case, persona, interaction, visual, and arc42 conformance verified.
- [x] No deferral without the operator's recorded decision.

REVIEW: PASS
