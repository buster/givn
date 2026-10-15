# Design Review: drop-legacy-residue

Hardening phase record. The grilling phase walked every mandatory branch;
three operator questions were asked and answered; the findings were repaired
in `design.md`, the delta specs, the proposal, and the arc42 chapters. This
record carries the operator decisions, the repairs, the branch ledger, and
the verdicts.

## Operator decisions

### Q1 — Visual evidence scope

- Question: The terminal contract's expected evidence set named only the two
  new `@e2e` scenarios, but the review gate's `check_visual_evidence`
  collects every delta `@e2e` scenario (minus `@givn.removed`) and demands
  one non-empty transcript per scenario at
  `evidence/visual/<scenario-slug>/transcript.txt`. The change's delta also
  re-points four existing `@e2e` scenarios (the blocked-runner scenario in
  `enforce-post-merge-e2e-gate` and the three `grouped-merge-for-new-capabilities`
  scenarios). What is the expected evidence set?
- Options:
  - A (accepted): **six transcripts** — the two new scenarios plus the four
    re-pointed ones. Gain: the declared contract matches what the integrity
    gate actually checks at review; each re-pointed scenario keeps its
    corrected Given text and its evidence is a real archive/CLI run. Cost:
    four additional transcripts to capture at implement. Example:
    `evidence/visual/consumer-archives-a-new-capability-and-the-merged-spec-lands-in-its-declared-use-case/transcript.txt`
    records the re-pointed scenario's `givn archive` run with the corrected
    `givn/commands.yaml` Given.
  - B (rejected): withdraw the Given-text corrections so the four scenarios
    stay byte-identical and need no new evidence. Cost: the stale
    `givn/config.yaml` claims — the exact residue this change removes — stay
    in the executable spec.
  - C (rejected): strip the `@e2e` tags from the four re-pointed scenarios.
    Prohibited by the design-review rules: an `@e2e` scenario whose evidence
    is not captured is an unenforced gap, and tag removal only hides it.
    `@e2e` tags are never removed.
- Outcome: option A. The terminal contract's expected evidence set is six
  transcripts: the two new end-to-end scenarios plus the four re-pointed
  ones ("Consumer archives a delta that adds e2e scenarios and is blocked
  without a configured runner", "Consumer archives a new capability and the
  merged spec lands in its declared use case", "Archive is blocked when a
  new capability has no declared routing decision", "Nothing is merged when
  one delta file of a change has no routing decision").

### Q2 — Changelog dead link

- Question: The released `## [0.3.0]` changelog entry links
  `docs/migrations/0.2.0-to-0.3.0.md`, which this change deletes, and a
  permanent scenario ("The changelog describes the complete 0.3.0 release")
  asserts the `0.2.0-to-0.3.0` string stays in the changelog. What does the
  changelog edit look like?
- Options:
  - A (rejected): leave the dead link and record a rationale for it. Cost: a
    broken front-door reference stays broken, and the change's own goal is
    that every documented claim matches what the tool does.
  - B (accepted): **one edit** — keep the `0.2.0-to-0.3.0` string (the
    scenario's step "it contains the 0.2.0 to 0.3.0 migration entry point"
    asserts it) and remove or redirect the dead link target to
    `docs/migrations/README.md` or plain text. Gain: no broken link, the
    asserting scenario stays green, the release-record principle is
    otherwise untouched. Cost: `CHANGELOG.md` enters scope for one line.
    Example: "Added the LLM migration playbook at
    `docs/migrations/0.2.0-to-0.3.0.md`."
  - C (rejected): remove the changelog/migration-entry scenario. Cost:
    unpins the release-record contract the use case owns.
- Outcome: option B. The string becomes plain text (no redirect needed; the
  bullet is a release record, not a guide). The later prose code block that
  names the path is already plain text and is untouched. No other changelog
  line changes.

### Q3 — Migration floor glossary term

- Question: The durable chapters will name "the oldest declared source
  (0.7.0)" as the migration floor. The glossary's **Migration catalog** entry
  ("Embedded release policy at `assets/migrations/catalog.yaml` that declares
  source/target bundle edges and their order") already defines the
  declaration site, but there is no dedicated floor row. Is "the oldest
  declared source" an undefined standalone term?
- Options:
  - A (rejected): add a new glossary row for the migration floor. Cost:
    duplicates the catalog's declared-edge concept and gives one property a
    second durable home.
  - B (accepted): **no new row** — the floor is the first edge the migration
    catalog declares, covered by the existing **Migration catalog** entry.
    Record the decision, and phrase the durable chapters so the floor ties to
    the catalog concept instead of presenting an undefined standalone term.
- Outcome: option B. The chapters now say "the oldest source the migration
  catalog declares (0.7.0)" (chapters 3, 4, 7, 10, 11, and the arc42
  README); the out-of-scope row says "the oldest migration route the catalog
  declares". No glossary row is added.

## Finding repairs

- **F2 — Interface line formatting (blocker).** `design.md`'s Visual Design
  Contract read `- Interface: terminal. The change's user-visible surface
  is…`, which `src/integrity_hook.rs::parse_visual_contract` resolves to
  `VisualInterface::None` (the trimmed value is matched against exactly
  `web`/`terminal`). The line is now `- Interface: terminal` on its own
  line, with the justification moved to a `- Interface rationale:` bullet.
  Verified against the parser source: the section now parses as
  `VisualInterface::Terminal`, so the review gate will enforce the
  transcripts, the persona disposition, and the `VISUAL-REVIEW` verdict.
- **F1 + Q1 — Six-transcript evidence set.** The contract's "Expected
  evidence set" names all six transcripts (the two new `apply-a-managed-upgrade`
  scenarios and the four re-pointed archive scenarios) with their slugs and
  evidence paths `evidence/visual/<slug>/transcript.txt`; the capture
  procedure is unchanged. No other section named a transcript count.
- **F3 + F4 — Retirement sequence and corpus cleanliness (blocker).** The
  claim "no gate reads the permanent corpus before the archive merge" was
  false: default-scope `givn lint` enumerates every directory under
  `givn/specs/` (except `fragments`) and exits 3 while
  `rewrite-history/usecase.md` exists — the dangling
  `Includes: fragment: rules-policy` reference (`dangling_use_case_ref`; the
  fragment's id is `support`) and the `history-rewrite` interaction
  inventory's missing-@e2e mismatch (`interaction_mismatch`) — and would keep
  exiting 3 for any interval where the directory exists without a narrative
  (`usecase_invalid`) or without the feature (`dangling_capability_ref`).
  Repaired: the archive retirement is extended — after
  `process_delta_file` (`src/cli/archive.rs`, the `Some(usecase_dir)`
  retirement arm) removes the retiring capability's `.feature`, when the
  use-case directory is not `fragments` and holds no remaining `.feature`
  files, it removes `usecase.md` (when present) and the directory; `io::Error`s
  propagate with `.with_context()`, leaving the recorded tombstone in place
  (repair-forward). The narrative is no longer deleted during implementation;
  the retirement removes feature, narrative, and directory at the merge, and
  `givn lint` default scope returns to 0 only when the directory is gone.
  The design's exact-file-changes and spec-corpus tables, D5, the plain-
  deletion decision, the retirement mechanics section, the delta map, the
  fixtures table, and the use-case traceability all carry the new mechanism.
- **Retirement scenario owner.** The minimal `@givn.added @wip` scenario
  "Retiring a use case's last capability removes the use-case directory and
  its narrative" is added to the
  `grouped-merge-for-new-capabilities` delta. Owner choice:
  `grouped-merge-for-new-capabilities` owns the archive merge step's
  "retirement and grouping rules" (`archive-change` main flow 2); its
  archived proposal scopes it as the archive merge into the permanent
  corpus, the retirement path shares `process_delta_file` with the grouping
  path, and the retirement unit tests live beside the merge loop in
  `src/cli/archive.rs`. The scenario is deliberately not `@e2e`: it pins a
  merge rule through the real archive subprocess (established practice in
  `archive-proof` and `preserve-scenario-docstrings`), the capability's
  `@e2e` evidence is unchanged, no interaction-inventory row is needed, and
  the terminal evidence set stays at the six transcripts of Q1. The
  retirement behavior is designed and pinned by the `@wip` scenario, not
  implemented in this pass.
- **F5 — Guide assertion wording.** The design no longer promises an
  "edits no corpus artifact" assertion that the guide does not carry. The
  re-pointed `describe-the-release` steps assert the guide's actual phrases:
  "The maintenance change is the **inventory and plan**, not the repair
  site." (line 42), "Do not edit the corpus in this change." (Prompt 1),
  "Confirm this change edited no corpus artifact except the term
  collision-check cells" (Prompt 6), the sentence "The LLM must inspect the
  project before editing it" (line 9), and ordered headings of the exact
  shape `### Prompt N — <Title>` (em dash) ending in
  `### Prompt 6 — Validate and archive`.
- **Q2 — Changelog scope.** `CHANGELOG.md` left the proposal's Out of Scope
  and entered the scope text (the one-link edit above); the design's edit
  map carries the exact change. This is the only proposal edit.
- **F14 — Orphaned step.** The `history-rewrite` row of the fixtures table
  now lists the deletion of `the reachable history contains "…"` in
  `tests/steps/local_only_rules_steps.rs` beside `manifest_extras_enabled`,
  plus its equally orphaned sibling `the reachable history does not contain
  "…"` — verified against the active corpus: only the retired capability's
  scenarios use either step; the remaining `local_only_rules_steps.rs` glue
  serves the report-only `rules-policy` / `publication-cleanliness`
  scenarios.
- **Q3 — Glossary decision.** Recorded in `design.md`; no glossary row is
  added, and the durable chapters' phrasing ties the floor to the migration
  catalog (edits applied).
- **F21 — Runner caveat.** Recorded in the runner section: the
  single-scenario debug command runs plain `cargo test` (libtest, `--name`
  filter) while the gate runs `./run-tests.sh` under cargo-nextest
  (`GIVN_CUCUMBER_TAGS`). The task list must not treat a green
  single-scenario libtest run as the nextest gate result.

## Branch ledger (compact)

Walked in the grilling phase, no issue found:

- Tech choices — the stack is unchanged repository tooling; no simpler
  alternative satisfies the scenarios.
- Missing scenarios — no observable behaviour implied by the proposal is
  absent; the added retirement scenario covers the one gap the repair
  introduced.
- Testability triple — strict mode (`.fail_on_skipped()`), the
  not-implemented stub pattern, and the single-scenario run command are all
  named concretely.
- E2E fidelity — every delta `@e2e` scenario drives the real CLI boundary;
  no redefinition of the normalized action scope, interface type, real
  driver, or primary-assertion rules.
- Interaction coverage — every inventory entry has a matrix row and a
  matching `@e2e` scenario; the added retirement scenario deliberately
  creates no inventory entry.
- Risk mitigations — recorded in chapter 11 (R-81, R-82) and the retirement
  mechanics.
- Use-case and Persona context — use-case identity, contracts, capability
  ownership, typed relationships, and Persona coverage confirmed; no
  invented Personas, no Persona biographies used as Gherkin actors.
- Decision ledger — every proposal `D<n>` resolves in the design's
  `## Decisions` ledger; D2 keeps its seed provenance.
- Deferrals — none requested, none recorded.
- Persona disposition — recorded below.
- Interface classification — the opposite (browser) classification was
  tested and does not hold: nothing renders HTML, CSS, or JavaScript.
- ADR qualification — the canonical procedure ran via
  `givn instructions arc42-docs --change drop-legacy-residue`; verdict
  below.
- Arc42 re-derivation — all 12 chapter-selection rows were derived
  independently from proposal/specs/design before reading `arc42.md` and
  agree with it.
- Merge-bookkeeping checks — all 18 delta operations and the narrative
  base SHA-256 were re-checked against the delta files and the projected
  merge; no discrepancy.

## Deferrals

None. Every mandatory check was completed in the grilling phase.

## Persona disposition and visual contract status

- Confirmed Personas: `developer--primary-user` (runs the retired command,
  owns the history the removal concerns, crosses the migration gap),
  `agent--technical-role` (reads the retired command's exit code and stderr
  and the upgrade's machine-visible effects), `givn-maintainer--owner` (owns
  the catalog and the release routes). All three resolve to files under
  `givn/personas/`; a user-visible surface requires at least one confirmed
  Persona, and the change has three.
- Visual contract: `- Interface: terminal`, parsed as
  `VisualInterface::Terminal` (verified against `parse_visual_contract`).
  Viewports, design system, and motion are n/a for CLI text; the evidence
  kind is captured final transcripts, six in total per Q1. The review stage
  will require `VISUAL-REVIEW: PASS` and the `PERSONAS:` disposition line in
  `review.md` — a declared visual interface cannot pass review without them.

## ADR verdict

`CANONICAL_ARTIFACT` — `NOT_QUALIFIED`. The change removes a subcommand, a
dead manifest field, two legacy config fields, and five embedded bundles;
no component, authority, dependency, or deployment boundary moves, reversal
is one change's code and text sweep, and the rationale for report-only
cleanliness is cross-cutting documentation owned by chapter 8 section 8.7
plus the fragment specs, while the route floor is release policy owned by
the migration catalog and the use-case specs. `existing_adr_check` searched
the register and archive: ADR-023 (versioned-upgrade boundary) and ADR-071
(SCM seam) are unchanged by this removal; no amendment or supersession
applies. The same verdict covers the catalog-floor and configuration-key
candidates. No ADR file is created.

## Architecture documentation (arc42) verdict

The independent 12-row re-derivation agrees with `arc42.md` on every row
(rows 1–12 all marked per the assessment; no row where the reviewer says Yes
and the marker says No or omits the row). Every chapter marked Yes exists,
carries real content beyond the scaffold, and matches `design.md`; the
box-drawing/diagram scan found no ASCII-art violations in any chapter file.
Chapter 09 carries no new MADR, consistent with the `NOT_QUALIFIED` verdict.
The Q3 phrasing tie was applied in chapters 3, 4, 7, 10, 11, and the arc42
README; no glossary row was added (the **Migration catalog** entry is the
durable home of the floor).

## Changes made during hardening

| Artifact | Change summary |
|---|---|
| `design.md` | Interface line split (F2); six-transcript evidence set (F1+Q1); retirement mechanics rewritten with the last-capability directory removal, the lint-exit-3 intervals, and the removed false claim (F3+F4); D5 and the plain-deletion decision reworded; Q2 and Q3 decision entries; `src/cli/archive.rs` row in the production table; `CHANGELOG.md` row in the docs table; spec-corpus rows reworded; delta map and fixtures table rows for the added retirement scenario and the orphaned reachable-history steps (F14); guide assertion phrases corrected (F5); runner caveat (F21) and the retirement/guide sequencing note; architecture-impact sentence; interaction-matrix clarification. |
| `specs/archive-change/grouped-merge-for-new-capabilities.feature` | Added the `@givn.added @wip` scenario "Retiring a use case's last capability removes the use-case directory and its narrative". |
| `proposal.md` | `CHANGELOG.md` moved out of Out of Scope; the one-link edit added to the scope text (Q2). |
| `docs/arc42/03-context-and-scope.md` | Floor phrasing tied to the migration catalog (two occurrences incl. the out-of-scope row). |
| `docs/arc42/04-solution-strategy.md` | Floor phrasing tied to the migration catalog. |
| `docs/arc42/07-deployment-view.md` | Floor phrasing tied to the migration catalog. |
| `docs/arc42/10-quality-requirements.md` | Q-48 floor phrasing tied to the migration catalog. |
| `docs/arc42/11-risks-and-technical-debt.md` | Floor phrasing tied to the migration catalog. |
| `docs/arc42/README.md` | Floor phrasing tied to the migration catalog. |

## Sign-off

- [x] Every branch of the design tree has been walked in the grilling phase.
- [x] All three operator questions are answered and recorded with their
      rejected alternatives.
- [x] All hardening edits are applied; the terminal contract parses as
      `Terminal`, and the false corpus-gate claim is corrected with the
      exact retirement mechanism.
- [x] `@e2e` tags: none removed, none stripped.
- [x] `tasks.md` was not written (next artifact).
- [x] No implementation (`src/`, `tests/`, assets) was performed in this
      pass; the archive-retirement behavior is designed and pinned by a
      `@wip` scenario.
- [x] `givn lint --change drop-legacy-residue` parses and exits 0 or 2
      (2 expected while `@wip` scenarios are pending).

DESIGN-REVIEW: PASS
