# Design Review: add-ise-clean-room-commands

Review kind: independent
Author: add-ise-clean-room-commands hardening author (omp, deepseek-flash)
Reviewer context: independent grilling session whose decisions 0-12 the operator recorded on 2026-11-14; this authoring context applied them and owns the artifact edits

The proposal records `Review kind: independent` and its reason (user-facing
command pair, durable trial record with its own scope, a revision reader that
must agree on Git and Fossil, and a write path outside the checkout). The
reviewer's decision set is the grilling result; this context is the hardening
author. Every decision below is applied to the artifacts. No grilling question
is unresolved, so the review signs off.

## Decisions applied

**0. The command surface is `replace` with `--path` and `--full`.** The
subcommand is renamed from `prepare` to `replace` everywhere; `--path` replaces
the retired single-value replacement flag and repeats, accepting a tracked file
or a directory whose tracked files are the unit; `--full` names every tracked
file under every source root;
exactly one of the two is required, and neither, both, or an omitted `--into`
is a usage error (exit 2). Landed: `proposal.md:5` (release note), `:45-98`
(observable behavior), `:132-164` (D1); `specs/prepare-an-ise-test/usecase.md:31`
(main flow), `:56-84` (rules), `:86-127` (examples); `design.md:7-13`
(overview), `:192-210` (D1), `:468-515` (replace flow), `:603-620` (partition),
`:624-640` (refusals); every feature step that named a replaced path;
`docs/arc42/adr/ADR-076-clean-room-trial-record.md:48`, `:118-125`;
`docs/glossary.md:74-75`; `givn/ideation/ise-rebuildability/plan.md:322-433`,
`:565`, `:647-650`, `:684`, `:771-773`, `:800-802`, `:853-855`, `:927`, `:954`,
`:978-982`; `docs/ideation/ise-rebuildability.md:90`, `:165-227`, `:308-311`,
`:346`, `:404-408`; `docs/arc42/02-architecture-constraints.md:54-55`,
`03-context-and-scope.md:62-63`, `04-solution-strategy.md:24`,
`05-building-block-view.md:62-63`, `06-runtime-view.md:982`,
`08-crosscutting-concepts.md:586`, `1465-1520`, `README.md:50`; the seed
`givn/ideation/ise-rebuildability/changes/add-ise-clean-room-commands.md:3-10`,
`:21-30`, `:36`, `:41-61`, `:63-91`, `:162-169` (surface note: the accepted
decisions keep their substance, and a named `--path` directory is explicit
input, not inferred deletion).
`docs/reference.md` carries no `ise` command reference yet (it receives it in
the implementation batch), so no stale name existed there; the required grep
confirms it.

**1. The Fossil reader decodes `\s` and accepts 40-hex or 64-hex hashes.** A
`l` entry exists only with `allow-symlinks on`, so the fixture sets it and
asserts an executable and a symlink. Landed: `design.md:14-19` (overview),
`:247-253` (D6), `:317-323` (data model), `:688-697` (E2E infrastructure),
`:787-792` (obstacles), `:337-345` (Fossil scenario);
`docs/arc42/11-risks-and-technical-debt.md:132` (R-145 corrected; it no longer
claims a fixture that did not exist); `docs/arc42/08:1514-1520`;
`docs/arc42/02:55`.

**2. Contracts are recorded for the two read seams.** `RevisionE2eFileSet`
(`e2e_file_set`, implementation `src/e2e_boundary`, consumers `replace` and
`verify`) and `ScenarioDigest` (implementation `gherkin`, consumer `ise`) are
rows in the contract table; `src/config/declaration.rs` is no longer declared
unchanged because its normalizer becomes `pub(crate)`. Landed:
`design.md:165-175` (contracts), `:122-129` (architecture impact),
`docs/arc42/05-building-block-view.md:90` (resolver responsibility).

**3. The record is a trust anchor only after three checks.** `verify` refuses
unless the canonicalized `written_path` equals `--at`, the record's `backend`
equals the selected backend, and the record path lies outside the trial; it
asserts `replaced_digest` is a 64-hex value. The runner command is the
verifier's responsibility, not the record's. Landed: `design.md:274-283` (D9),
`:381-398` (identity), `:458-463` (verify output), `:549-570` (verify flow),
`:165-175` (contract note); `usecase.md:80-82`; `02:55`; `06:1044-1046`;
`08:1500-1505`.

**4. The identity encodes the entry kind.** `kind` is `f`, `x`, or `l`; the
per-path digest covers path, kind, executable marker, and bytes or target; the
rollup hashes sorted `path` plus digest pairs; the feature and E2E path sets
are disjoint by construction, so no duplicate-path tie-break exists. Landed:
`design.md:257-265` (D7), `:381-398` (identity), `:366-368` (record kinds);
`usecase.md:76`; `08:1485-1491`.

**5. The missing scenario set is complete.** Prepare gains malformed
declaration, architecture-principles refusal, directory-contains-retained,
neither/both forms, omitted `--into`, existing target file, receipt inside the
checkout, receipt ancestor, symlinked target, directory expansion, `--full`,
duplicate path, glob-shaped path, version-control-metadata absence, explicit
revision, absent revision, blob read failure, and record write failure. Verify
gains architecture-principles refusal, unknown field, missing field, record
inside the trial, `written_path` mismatch, and `backend` mismatch; the ise-off
verify scenario uses a corrupted record to prove it is not read. Every report
line is asserted with a value (counts, resolved revision, 12-hex digest plus
ellipsis, written and receipt paths), and the prepare E2E asserts
`replaced_digest`, `written_path`, and `backend` in the record. Landed:
`clean-room-preparation.feature:4-13` (E2E assertions), `:27`, `:42`, `:99`,
`:107`, `:115`, `:123`, `:146`, `:161`, `:168`, `:175`, `:215`, `:224`, `:233`,
`:241`, `:268`, `:300`, `:309`, `:316`, `:324`;
`trial-input-verification.feature:55-62` (corrupted record), `:64`, `:78`, `:84`,
`:90`, `:97`, `:104`; `design.md:504-508` (re-read unit test), `:649-658`
(boundary cases).

**6. `Trial directory` is a glossary term.** The term is added as the plain
directory written outside the checkout, and the design prose uses "receipt
path" or "trial record" instead of "the receipt". Landed:
`docs/glossary.md:76`; `design.md:59-61`, `:403-426`, `:688-697`;
`usecase.md:69-74`.

**7. The prerequisite is checked, not only `ise`.** Both commands refuse when
`architecture_principles` is off, matching the use-case precondition. Landed:
`design.md:287-289` (D10), `:473-475` (flow), `:626` (refusal 2),
`usecase.md:44`; `clean-room-preparation.feature:42`;
`trial-input-verification.feature:64`; `02:55`.

**8. ADR-076 names its cheaper home and a concrete confirmation.** The ADR
records that §8.29 plus `design.md` own the current facts and cannot own the
rejected alternatives or durable consequence; `Confirmation` names the delta
E2E scope (`./run-tests.sh --e2e`, `GIVN_RESULT_SCOPE=e2e`) instead of
`cargo nextest run`; `More Information` links the delta use case, not a
permanent file that does not exist yet. Landed:
`docs/arc42/adr/ADR-076-clean-room-trial-record.md:83-91`, `:118-125`,
`:151-152`; `arc42.md:27` (cheaper home in the assessment).

**9. Section references are corrected.** The runtime flow is 6.21 and the
record section is 8.29. Landed: `arc42.md:12`, `:14`;
`docs/arc42/06-runtime-view.md:982`; `docs/arc42/08-crosscutting-concepts.md:1463`.

**10. The domain model no-op is recorded.** `docs/domain-model.md` excludes
internal Rust components, command parsing, and file formats, so the trial
record and the command are architecture, not domain structure. Landed:
`arc42.md:46-49`; `design.md:676-679`.

**11. Testability names the mechanism and the stub pattern.** The dangling
"the unit test below" is gone; the traceability row names the re-read unit test
with an injectable writer, and the strict-mode section states the
not-implemented stub pattern (`Err(anyhow!("not implemented: <step text>"))`)
that keeps RED a failed step rather than a skip. Landed: `design.md:90`,
`:504-508`, `:749-754`.

**12. The visual contract separates fixed lines from additions.** The report is
ten fixed lines plus `Receipt path` when a receipt is written; `--dry-run`
adds a `Would write:` block. Landed: `design.md:59-61`, `:403-426`, `:513-515`;
`clean-room-preparation.feature:4-13`, `:283-289`; `usecase.md:72-74`.

## Grill branches

- **Scope.** The proposal contains exactly the two commands and their record.
  "Done" is observable: the report lines, the trial contents, the record
  fields, the nine refusals, the two usage errors, and the verify drift lines.
  The Out of Scope list keeps the instruction split, `ise` enablement, harness
  variants, paid campaigns, and runner execution outside the change
  (`proposal.md:109-126`). No scope mismatch remains; the proposal edit is the
  operator's decision 0, not a silent scope change.
- **Tech choices.** Rust modules with the existing crates (`serde_yaml_ng`,
  `sha2`, cucumber-rs); no new dependency. Simpler alternatives (one rollup, a
  record inside the trial, a worktree) are rejected with reasons in D2/D4 and
  the ADR. The clap argument group is the simplest enforcement of exactly one
  replacement form.
- **Missing scenarios.** The full operator set is present (decision 5). Each
  scenario asserts a concrete value: `Backend`, the resolved revision,
  `Retained`, `Kept source`, `Replaced`, `Outside the declaration`,
  `Not replaced`, 12-hex digest plus `…`, `Written path`, `Receipt path`, and
  the record's `replaced_digest`, `written_path`, and `backend`. Refusals name
  their condition and subject; usage errors assert exit 2.
- **Testability.** RED is reachable without production code: the subcommand is
  unbound and every delta step stub returns a named failure; `.fail_on_skipped()`
  turns a skip into `failed = 1`; `givn check tasks` refuses a selected delta
  scenario while `@wip` (`design.md:743-754`). The single-scenario command is
  exact (`design.md:737-741`). The re-read comparison cannot be raced through
  the CLI, so it is a unit test with an injectable writer (`design.md:504-508`).
  Every Then-step asserts an observable value, never "it ran".
- **E2E fidelity.** The spec instruction's normalized action scope, CLI
  interface, real-driver, and primary-assertion rules are applied without
  redefinition: each inventory entry has exactly one `@e2e` scenario driving
  the compiled binary as a subprocess; the primary assertions are the printed
  report and the exit code; trial and record contents are secondary evidence.
  Regular scenarios assert distinct refusals and variants, not a restatement of
  the E2E outcome. No browser driver is declared, and no HTTP path stands in
  for the CLI (`design.md:759-775`).
- **Interaction Coverage.** The use case's `## Interactions` table has two
  entries (`usecase.md:129-134`); the design matrix has the same two rows with
  a real interface and a non-empty driving mechanism (`design.md:759-764`);
  the `@e2e` titles match the table exactly. `givn lint` reports no inventory
  finding.
- **Risk.** The most likely implementation failure is fixture fidelity for the
  re-read and record-write paths and for the Fossil manifest. Mitigations: the
  injectable-writer unit test; the receipt whose parent is a regular file; the
  `allow-symlinks on` Fossil fixture; the refusal order stated once
  (`design.md:816-834`). The typo risk stays deliberate and is recorded as
  R-142.
- **Use-case and Persona context.** Identity `prepare-an-ise-test`, Level `!`,
  goal, actors, preconditions, main flow, extensions, rules, examples, minimal
  and success guarantees, and both capabilities are preserved. The typed
  relationships are `Includes: none` and `Extends: none`, consistent with the
  proposal's routing. Gherkin names the domain actors (developer, verifier),
  never a Persona biography. The three Personas are confirmed with per-capability
  impact rows (`design.md:766-778`).
- **Decision ledger.** `D1` to `D4` resolve in the design ledger, `D1` keeps the
  seed reference (`seed Q4`); `D5` to `D10` are local decisions that state what
  they reject. No proposal question is unresolved or unrecorded
  (`design.md:192-292`).
- **Contracts and components.** Two consumer faces exist: `cli::ise` and the
  trial component. `CleanRoomTrial` is implemented once by `src/ise`;
  `RevisionContent` by the SCM seam; `RevisionE2eFileSet` by `e2e_boundary`;
  `ScenarioDigest` by `gherkin`. Each contract names consumers, one
  implementing component, and a split trigger; no consumer imports an
  implementation. The external boundaries are the Git and Fossil CLIs (one SCM
  port with two adapters) and the filesystem; the design names both. The trial
  record carries no runner command, so no verifier mechanism leaks into the
  contract.
- **Decision timing.** No decision is carried: D1 to D10 are foundational and
  realized in code or record scope. Contracts are current requirements, not
  open questions. No speculative abstraction exists for future use cases; the
  two read seams are current consumers' needs.
- **Visual design contract.** `design.md:36-67` declares `Interface: terminal`,
  viewports not applicable, states not applicable, design system none, the
  baseline (named refusals, no internal names, exit codes, ten fixed lines plus
  the conditional receipt line and the dry-run block, record field names), and
  the two transcript-producing `@e2e` scenarios. No screen is introduced, so
  no rendered options are required and none are claimed.
- **Deferrals.** None. No `@e2e` tag was removed; the e2e infrastructure is
  buildable in this change; no mandatory check is deferred.
- **Persona disposition.** `developer--primary-user`, `agent--technical-role`,
  and `givn-maintainer--owner` are confirmed with relevance and evidence in
  the Persona impact table; no Persona is invented or silently promoted.
- **Interface classification.** The opposite classification (Web UI) does not
  hold: the change emits terminal text, a YAML record, and exit codes, renders
  no HTML/CSS/JavaScript, and exposes no browser control. The CLI classification
  is tested, not assumed.
- **ADR qualification.** The assessment records the architectural commitment,
  real alternatives, durable consequence, existing-decision search, the
  `NEW_ADR` route, and the cheaper home (§8.29 plus `design.md`) that cannot
  own the rejected alternatives. ADR-076 is one decision; the partition,
  Fossil reader, report abbreviation, and e2e resolution stay in their
  canonical artifacts.
- **Architecture documentation.** Independent 12-row re-derivation against the
  arc42-docs table: 1 No (no goal/stakeholder/quality change), 2 Yes
  (constraints), 3 Yes (CLI surface, external verifier), 4 Yes (strategy row),
  5 Yes (new blocks and read seams), 6 Yes (replace and verify flows), 7 No
  (one binary, a directory target), 8 Yes (record and identity), 9 Yes
  (ADR-076), 10 No (functional scenarios, no quality metric), 11 Yes
  (R-142 to R-146), 12 Yes (glossary terms). The assessment matches every row,
  including the corrected 6.21 and 8.29 references. All twelve chapter files
  exist with real content; no box-drawing characters or `+---`/`|---`
  pseudo-diagrams exist. Chapter 02 (`TC-46`, `TC-47`), 03 (two command rows,
  verifier entity), 04 (strategy row), 05 (`cli::ise`, `ise`, resolver), 06
  (6.21), 08 (8.29), 11 (`R-142`, `R-145`), the ADR register row, and the ADR
  file match `design.md`.
- **Ubiquitous language.** `Trial directory`, `Trial record`, and `Trial E2E
  digest` are in `docs/glossary.md`; the artifacts use them consistently;
  `E2E identity` stays scoped to the change verification receipt. No invented
  noun survives the sweep.
- **Structural model.** `docs/domain-model.md` scopes itself to domain units
  and excludes internal Rust components, command parsing, and file formats; the
  change assesses the impact as none and records it in `arc42.md:46-49` and
  `design.md:676-679`. The no-op is the maintenance result, not an omission.

## Blockers and gaps

1. **`--full` retains role precedence.** `--full` removes every tracked file
   under the source roots that no retained role matches. A retained file under
   a source root stays in the trial and is counted in `Not replaced`, so the
   report's `0` is asserted with a declaration whose roles sit outside the
   source roots. Landed: `design.md:200-205`, `:603-608`;
   `clean-room-preparation.feature:224-231`.
2. **The re-read mismatch is not a CLI fixture.** A deterministic fixture
   cannot corrupt a file between the write and the re-read, so the comparison
   is a unit test with an injectable writer. Landed: `design.md:90`, `:504-508`.
3. **The record-write failure is provoked by a non-directory parent.** The
   receipt path's parent is a regular file, so `create_new` fails after the
   trial exists; the refusal names the record and removes the trial. Landed:
   `design.md:509-511`; `clean-room-preparation.feature:324-331`.
4. **Verify cannot recompute the replaced digest.** The removed files are not
   in the trial and the checkout may have changed, so verify asserts the
   recorded 64-hex value and re-checks the retained and `e2e` identities.
   Landed: `design.md:274-283`, `:381-398`, `:458-463`.
5. **Fossil `l` entries need repository support.** The fixture sets
   `allow-symlinks on` and asserts both an executable and a symlink; without
   the setting Fossil records no `l` entry. Landed: `design.md:247-253`,
   `:317-323`, `:688-697`; `clean-room-preparation.feature:333-345`.
6. **A glob-shaped `--path` value is not a refusal.** It resolves to no tracked
   file and removes nothing, preserving the settled nine refusals; the scenario
   asserts `Replaced: 0` and the trial keeps the files. Landed:
   `design.md:199-205`, `:600-608`; `clean-room-preparation.feature:241-248`;
   R-142 at `docs/arc42/11-risks-and-technical-debt.md:129`.

No blocker is unresolved.

## Verification

- `givn lint --change add-ise-clean-room-commands` exits 2 with 58 findings:
  42 `@wip` on the prepare scenarios and 16 `@wip` on the verify scenarios,
  one per added scenario, expected before implementation. 100 `SUBST`
  advisories pair unrelated scenarios that share a step count; the scenario
  claims, drivers, and assertions differ, so no merge is warranted.
- `givn status --change add-ise-clean-room-commands` reports
  `Stage: design-review-done` and `Next required: tasks`.
- The stale-name grep over `givn/changes/add-ise-clean-room-commands/`,
  `docs/reference.md`, `givn/ideation/ise-rebuildability/plan.md`, and
  `docs/ideation/ise-rebuildability.md`, for the old subcommand name and the
  retired replacement flag, returns no line.
- Feature counts: 42 scenarios in `clean-room-preparation.feature` (23 before,
  19 added) and 16 in `trial-input-verification.feature` (10 before, 6 added);
  58 delta scenarios.
- `docs/reference.md` has no `ise` command table yet; the design records its
  update in the implementation batch (`arc42.md:40-44`), so no stale name is
  present.

## Post-review clarifications

Three clarifications are applied after the review sign-off; none reopens a
decision:

1. Retained precedence is stated where the reader needs it. A tracked path that
   matches both a retained role and a `source:` root stays in the trial under
   `--full` as well as `--path`, and `Replaced` counts only non-retained files.
   Landed: `design.md` D1, the Use-Case Traceability retained row, the
   `## Partition Rules` placement-boundary table, and
   `specs/prepare-an-ise-test/usecase.md` rules.
2. `RevisionContent` has no consumers today. The contract row and its note now
   name only this change's `ise` reader; `cli::archive` and `cli::migration` do
   not call `revision_files` or `revision_blobs` outside `src/scm.rs`.
3. The re-read mismatch is unit-only. `design.md` Replace Flow step 10 and the
   `partition-and-write` batch record that the injectable-writer unit test in
   `src/ise/mod.rs` proves it and no delta scenario covers it.

DESIGN-REVIEW: PASS
