# Guidance improvements

Status: draft for a change to the `ise` instruction, 2026-11-16. Evidence: the
Passuff trial transcripts under `/tmp/passuff-verify` and the 85-record corpus
at `/tmp/passuff-blind-3/spec-gaps.jsonl`, the Catopia `0.4.0` record at
`/home/buster/projects/givn-harness-ise/docs/experiments/ise-rebuild-0.4.0.md`,
and the S14 evidence in `givn/ideation/ise-rebuildability/plan.md`.

Each section states one proposed instruction sentence and the run or file that
grounds it. The note changes no served guidance; it is input to a later change
to `assets/instructions/ise.md`.

## 1. Declare every observed value

**Proposed instruction.** Declare every value the End-to-end (E2E) test reads:
every string, heading, label, error prefix, notification title, status format,
exit code, file format, and test seam appears in the Specification at a line a
builder can copy.

**Assertion test.** For every assertion in the acceptance suite, name the
Specification line that states it. An assertion with no line is either a
`specification defect` (the product exposes the value and the Specification
omits it) or an `E2E test defect` (the suite asserts a value the product need
not expose); classify before fixing.

**Worked example: the Passuff 8-scenario package.** The same product and the
same 8-scenario suite (60 steps) scored 3 of 8 in a blind run, then 8 of 8 twice
after amendment `dddeb14` (`plan.md`, `## S14 evidence`, Passuff results). The
suite did not change. The Specification gained the lines that state its
assertions:

| Suite assertion | Specification line after `dddeb14` |
|---|---|
| `Fresh baseline recorded for octo/repo#7` | `docs/reference.md:464`; the suite assertion is recorded at `:504` |
| `passuff: 0 unread of 0` and `passuff: 0 unread of 1` | `docs/reference.md:432`; recorded at `:504` |
| `octo/repo#7 open CI passing` and `version` 2 in the state file | `docs/reference.md:453-457` |
| `Event: comment by @reviewer` and `Pull request octo/repo#7` | `docs/reference.md:603-622` |
| `New activity on octo/repo#7` through `PASSUFF_NOTIFICATION_LOG` | `docs/reference.md:658-692` |
| `GitHub returned a malformed response` and `GitHub is unavailable` | `docs/reference.md:368-375` |
| The five GraphQL operation names the mock dispatches on | `docs/reference.md:309-324` |

The 50 `specification defect` records in the 85-record corpus name the classes
below; the notification-title row uses the first 8-scenario trial because that
gap was found before the widened corpus:

| Class | Representative evidence |
|---|---|
| Operation names | `gap-10`: query text, aliases, pagination arguments, variable shapes |
| Field paths | `gap-10`: response paths; `gap-01`: version-2 state fields |
| Test seams | `gap-67`: canberra argument form and `PASSUFF_SOUND_PLAYER` splitting |
| Headings | `gap-45`: footer hints and help overlay |
| Labels | `gap-37`: activity detail title |
| Error prefixes | `gap-25`: validation message text and prefix |
| Notification titles | `New activity on octo/repo#7` asserted by `boundary.feature`; absent from `docs/reference.md` before `dddeb14`, stated at `:680` |
| Status formats | `gap-64`: singular and plural wording for zero changed records |
| State-file shapes | `gap-01`: version-2 fields; `gap-02`: version-1 shape; `gap-03`: quarantine path |

**The test still finds one gap.** `crates/passuff-e2e/features/boundary.feature:10`
asserts the output contains `Commands:`; `docs/reference.md` states the command
table and the `--help` exit code, never the heading. A rebuild can print the
commands without that heading and fail the suite for a value the Specification
does not state.

## 2. Declare the seam

**Proposed instruction.** Declare every seam the frozen suite drives: its name,
path, format, and effect are Specification content, and an undeclared seam is
an `E2E test defect` that the suite must stop using or the Specification must
declare before the trial restarts.

**Worked example: Passuff's three seams.** The blind run before `dddeb14`
scored 3 of 8; the run after it scored 8 of 8 twice. The three passes were the
scenarios that do not require a successful GraphQL response; the dispatch rule
returns `{"data": {}}` for an unnamed query (`docs/reference.md:311-316`). The
missing declarations were:

| Seam | Declaration after the amendment | What the suite does with it |
|---|---|---|
| GraphQL operation names | `docs/reference.md:309-324`: `PassuffViewer`, `PassuffProbe`, `PassuffAuthored`, `PassuffDiscovery`, `PassuffObserve` | The mock dispatches on query text; an unnamed query returns `{"data": {}}` (`docs/reference.md:311-316`) |
| GraphQL endpoint | `docs/reference.md:688-692`: each account's `graphql_url`, overridden by `PASSUFF_GRAPHQL_URL` | The package answers `POST /graphql` |
| Desktop notification delivery | `docs/reference.md:658-692`: `PASSUFF_NOTIFICATION_LOG`, append mode, one JSON line per digest, fields `title`, `body`, `urgency` | The notification scenario reads the file and asserts one line titled `New activity on octo/repo#7` |

**Classification.** Run 1 passed 8 of 8 with the frozen package visible; run 2
passed 3 of 8 blind (`plan.md`, Passuff results). A suite that needs a hook the
Specification does not declare can pass only while the builder can read the
suite. The seam is an `E2E test defect`: the test reached a surface outside the
declaration. The repair is to declare the seam, then restart the trial; it is
not to relax the assertion.

## 3. Keep E2E few and decisive

**Proposed instruction.** Select E2E scenarios by external surface and failure
mode: one scenario per external surface (command, HTTP, pty, config file, state
file, signal), one per failure mode that changes an exit code, and one per
persisted-state migration; every other rule belongs to the component tier.

**Worked result: 8 scenarios to 29.** The starter package held 8 command-level
scenarios: help, one check and status, activity and read, two pty viewports, one
notification, and two failures. The widened package holds 29 scenarios and 240
steps (`/tmp/passuff-verify/baseline.txt:274-277`) and adds the surfaces the
selection rule names:

| Surface | Scenarios added by `12577f6` |
|---|---|
| Interactive setup | Writes a configuration; leaves an existing one; rejects an invalid watch repository |
| Credentials | Environment token; token file readable only by its owner; world-readable token file refused; token for another login |
| State file | Version-1 migration; corrupt state quarantined; duplicate delivery deduplicated |
| CI and event families | Failing CI notifies and recovery notifies; own comment recorded read; bot comment ignored or recorded by setting |
| Scope and filtering | Two accounts report only the watched scope |
| pty and viewport | Terminal too short; tab and back-tab switch panes |
| Signal | Default watch command delivers one notification and exits 0 on SIGINT |
| Exit-code failures | Server error exits 1; unreachable endpoint exits 1; invalid read exits 2; unknown id exits 2 |

**The widened suite first scored 25 of 29.** The four deterministic failures
were an activity detail (`implementation defect`), the version-1 migration
(`specification defect`, `gap-02`), bot-comment storage (`specification
defect`, `gap-57`), and the watch SIGINT exit (`specification defect`, no gap).
Amendments `ce3b982` (corrupt state asserted by bytes) and `1cecd0b`
(migration row, empty-inbox wording, watch exit) preceded a re-run at 29 of 29
twice (`plan.md`, Passuff results).

The 29 E2E scenarios sit over 197 component scenarios
(`/tmp/passuff-verify-regular.log:2041-2043`). Eighteen seeded defects qualify
the widened suite, one per new surface group (`7279f87`,
`scripts/qualify/*.patch`).

## 4. Keep the tiers differentiated

**Proposed instruction.** State each tier's assertion licence: component tests
cover permutations and in-process wiring and may be rewritten with the stack;
E2E tests assert only the external surface and are frozen.

**Worked example: the Passuff boundary.** The E2E package is
`crates/passuff-e2e`. It declares no dependency on `passuff`, `passuff-config`,
or `passuff-core`; `scripts/verify.sh:47-53` fails the component scope when the
dependency graph grows such an edge. The component tier does use the product as
a library: `crates/passuff-cli/tests/steps/commands.rs:5` imports
`passuff::args::Cli`, `crates/passuff-cli/tests/steps/tui.rs:13` calls
`passuff::tui::keys::parse_keys`, and `crates/passuff-cli/tests/support/clock.rs:4`
imports `passuff_core::Clock`.

**The anti-pattern is one runner for both scopes.** Before the move,
`scripts/verify.sh:57` ran the same `features_runner` binary for the regular and
`@e2e` scopes, so the tag was the only difference (`topic-document.md:24-30`).
The suite was green because it linked the product, not because it drove it. The
repair is a separate package with its own manifest, lockfile, runner, and
`verify.e2e_command`, plus a dependency check that fails on a production edge.

## 5. Assert only what the Specification states

**Proposed instruction.** Assert in the verifier only what the Specification
states; when a check needs a value the Specification does not pin, either state
the value or relax the check, and prove the relaxed check with a seeded defect
that still fails on the real defect.

**Worked example: Catopia `E17`.** The seed scored 17 of 18 and the rebuild 18
of 18 under frozen package `1.1.0`. The only failing check was `E17`:
`cert["id"] in sweep["expired"]` required an element kind that `contract.md:113`
did not state, because the contract printed `{"ok": true, "expired": [...],
"fines": [...], "referrals": [...]}` and pinned neither the element shape nor
its type (`ise-rebuild-0.4.0.md` `## 8. Arm records`, `E17` finding). The seed
returned certificate records; the reference and the rebuild returned id
strings.

Package `1.2.0` accepts an id string or a record carrying `id`. The
discrimination table is the evidence that the relaxation is narrow:

| Tree | Package `1.1.0` | Package `1.2.0` |
|---|---|---|
| Reference, `expired` holds ids | 18/18 | 18/18 |
| `expired-records` variant, `expired` holds records | 17/18, `E17` fails | 18/18 |
| `rebuild-sweep-lists-without-expiring`, records and no status change | 17/18, `E17` fails on the element first | 17/18, `E17` fails at `status after expiry` |

The relaxed check still fails the seeded defect, so it measures expiry rather
than list element type. The recorded 18 of 18 for the `0.4.0` rebuild kept its
meaning as a match to the reference, not as contract compliance
(`ise-rebuild-0.4.0.md` `## 9. Correction`).

## 6. Write the exact observed value

**Proposed instruction.** Write the exact observed value, not its category:
literal text, position, and the condition that produces it.

**Status line.**
Bad: `passuff status reports the unread count.`
Corrected: `The last line of passuff status is "passuff: <unread> unread of <total>".`
The 8-scenario suite asserted both `passuff: 0 unread of 0` and `passuff: 0
unread of 1`; the pre-amendment `docs/reference.md` stated neither, and
`docs/reference.md:432` now states the line. `gap-23` records a related
conflict: the read output reports the changed-record count, not the remaining
unread count.

**TUI label.**
Bad: `The activity detail shows the selected event.`
Corrected: `The activity detail pane prints "Pull request <owner>/<repo>#<number>" and then "Event: <summary>", after the fixed header "Activity <repo>#<number>".`
The 8-scenario suite asserted `Event: comment by @reviewer` and `Pull request
octo/repo#7`; the pre-amendment reference stated neither, and
`docs/reference.md:603-622` now states both. `gap-37` shows why a scenario
title is not a declaration: one scenario showed `Activity repo#42` and another
`Activity api#42`, and the derivation from the pull request key was unstated.

**Notification title.**
Bad: `A desktop notification is delivered for new activity.`
Corrected: `With PASSUFF_NOTIFICATION_LOG set, one digest appends one JSON line whose title is "New activity on <owner>/<repo>#<number>" when the pass has one pull request and one activity.`
The 8-scenario suite asserted exactly one log line titled `New activity on
octo/repo#7`; the pre-amendment reference named neither the variable nor the
title, and `docs/reference.md:658-692` now states the seam, the JSON fields,
and the three digest titles.

**Full example.** A declaration, a trial record, one `Spec gap`, and the
report lines, all from the same 29-scenario trial:

```yaml
# givn/specification.yaml (Passuff, ise/trial)
behavior:
  - givn/specs/**
structure:
  - docs/glossary.md
  - docs/domain-model.md
contracts:
  - docs/reference.md
  - README.md
  - docs/arc42/03-context-and-scope.md
limits:
  - docs/arc42/02-architecture-constraints.md
e2e:
  - crates/passuff-e2e
source:
  - crates/**
  - scripts/**
```

```yaml
# /tmp/passuff-trial-4.yaml, first lines and counts
schema: 1
content_scope: ise-trial/v1
backend: git
revision: 1cecd0b6a8a4390916a7e2db235865a0584d0d12
named_paths: []
replaced_paths:
  - crates/passuff-cli/Cargo.toml
  # 60 replaced paths follow
counts:
  retained: 20
  kept_source: 0
  replaced: 60
  outside_declaration: 39
  not_replaced: 0
excluded_roles:
  - e2e
```

```json
{"schema":"ise-spec-gap/v1","id":"gap-57","artifact":"crates/passuff-core/src/cycle.rs","difference":"A bot comment with bot comments disabled produces no notification, but whether it is stored and with which read flag is not stated.","outcome":"specification defect","inferred":"Store it unread and omit the notification.","source":"givn/specs/watch-pull-requests/usecase.md R12","scenario":"A bot comment is handled according to the bot setting"}
```

```text
1 feature
29 scenarios (29 passed)
240 steps (240 passed)
givn result: total 29, passed 29, failed 0, skipped 0
```

## 7. Triage a blocked trial

**Proposed instruction.** Triage a blocked trial by its recorded symptom: map
the symptom to one action and one artifact, and classify the difference before
changing the candidate.

| Symptom | First check | Action | Evidence |
|---|---|---|---|
| The trial will not seed because a retained path is missing | The declaration names a path under `givn/changes/<id>/` that the archive moved | Declare `givn/archive/<id>/...` or another durable path; `givn archive` now refuses a declaration that the move would break | `guard-retained-paths-across-archive`; `plan.md` section 12 |
| The frozen package builds in the checkout but not in the trial | `crates/passuff-e2e/Cargo.toml` inherits the repository workspace or lockfile | Give the package `[workspace]`, literal versions, and a committed `Cargo.lock` | gh-notify `15c5443` |
| A blind builder passes every scenario while the Specification is thin | The trial contains the `e2e` role and the builder can read the suite | Exclude the frozen package from the trial; the verifier reads it from the checkout | `ADR-078`; `/tmp/passuff-trial-4.yaml` `excluded_roles: [e2e]` |
| A gap record has `difference: none` | No trial or verifier difference was observed, often because no trial ran | Reject the record; when no difference was observed, write no gap | `SG-001`, `/tmp/givn-harness-workspaces/stage-run-catopia-v2-ise-followup/spec-gaps.jsonl`; `plan.md` section 12 |
| The suite is green but thin | 29 scenarios against 197 component scenarios; 76 of 85 gap records unexercised | Add one scenario per external surface and failure mode, and a seeded defect per new surface; report the unexercised gaps as a limit | `plan.md` `## S14 evidence`, open limits; `12577f6`; `7279f87` |

## 8. Report coverage per tier

**Proposed instruction.** Report coverage per tier: the E2E scope reports its
own coverage and the component tier reports the rest, a high component
coverage with a thin E2E suite is the intended shape, and the E2E suite does
not grow to chase a coverage number.

**Worked example: Passuff's two reports.** `givn/commands.yaml:49-55` declares
separate Cobertura paths, and `measure-coverage.sh:17-18` writes them:

| Tier | Command | Scenario total | Coverage report |
|---|---|---|---|
| Component | `./scripts/verify.sh` | 197 of 197 | `coverage/non-e2e-cobertura.xml` |
| E2E | `./scripts/verify.sh --e2e` | 29 of 29, 240 of 240 steps | `coverage/e2e-cobertura.xml` |

The intended shape has the component tier carrying the permutations: the plan
set 119 regular plus 11 `@e2e` scenarios and 87.33 line / 79.85 branch coverage
as the Catopia threshold (`plan.md` section `S7`). The recorded `0.3.0` arms did
not measure coverage, so the threshold states the shape, not a measured result
(`plan.md` section 12). A 29-scenario E2E suite is the acceptance boundary, not
the product's coverage instrument.

## 9. Declare roots, not files

**Proposed instruction.** Declare roots, not files: `givn/specification.yaml`
names path roots for the five retained roles and the replaceable `source` roots,
and names no individual file, no change artifact, and no component.

**Worked example: the Passuff declaration.**

| Rule | Declaration |
|---|---|
| Roots, not individual files | `behavior: givn/specs/**`; `source: crates/**, scripts/**` |
| No change artifacts | no `givn/changes/**` or `givn/archive/**` entry |
| Component setup under `source` | `crates/**` covers `crates/passuff-cli/tests` and its support modules |
| Frozen package under `e2e` | `e2e: crates/passuff-e2e`, with its own `Cargo.toml` and `Cargo.lock` |
| No component list | no `components:` key |
| Declaration needs no entry | the trial record lists `givn/specification.yaml` under `retained_files` (`/tmp/passuff-trial-4.yaml:85`); `assets/instructions/ise.md` makes it an implicit retained entry, not a sixth role |

## 10. Name the anti-patterns

**Proposed instruction.** Name the anti-pattern at review: a suite the builder
can read, an assertion whose value the Specification does not state, a grader
that asserts more than the contract, a corpus line cap, and a contract that
points at tests instead of stating behaviour are review findings, not style
preferences.

| Anti-pattern | Evidence | Repair |
|---|---|---|
| Tests visible to the builder | Run 1 passed 8 of 8 with the frozen package visible; run 2 passed 3 of 8 blind; `ADR-078` | Exclude the `e2e` role from the trial; the verifier reads the package from the checkout |
| Undeclared values derived from the tests | 50 `specification defect` records in 85 (`/tmp/passuff-blind-3/spec-gaps.jsonl`), including `gap-37` (two scenario titles) and `gap-57` (bot comment storage) | Apply the assertion test; declare the literal value and its condition |
| A grader that asserts more than the contract | `E17` required an element kind `contract.md:113` did not state; the seed scored 17 of 18 | State the element kind or relax the check, with a seeded defect proving it still fails |
| A corpus line cap | The split moved four core instructions from 1,630 to 1,544 lines; the numeric cap was withdrawn on 2026-11-15 (`plan.md`, instruction growth) | Remove the cap; a prose sentence that states no rule is the defect |
| A contract that points at tests instead of stating behaviour | `docs/reference.md:720-727` names `crates/passuff-e2e/features/boundary.feature` and the harness files as the source of asserted strings and exit codes | State the string and the condition in the contract; a test file name gives a builder nothing |

The 85-record corpus fixes the first anti-pattern's cost: 50 specification
defects, 24 acceptable undocumented changes, and 11 implementation defects. A
green run against 29 scenarios qualifies the boundary, not full-spec
conformance; 76 of the 85 gaps remain unexercised (`plan.md`, open limits).
