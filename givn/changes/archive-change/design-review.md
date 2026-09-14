# Design Review: archive-change

## Grilling

A fresh-context grilling subagent (read-only, full planning artifacts plus the
affected code, permanent specs, and the installed `fossil` binary) walked
every required branch and independently reproduced the Fossil mechanics. It
found no genuinely operator-facing question: every finding had an unambiguous
authoring fix derived from recorded decisions (Q3, Q5, Q13, Q14) and the seed
order. Its verified mechanics: `fossil changes --differ` includes `EXTRA`;
`fossil cat` exits 1 for an added-but-uncommitted path and 0 for a committed
one; a `before-commit` hook exits 1 with `Before-commit hook failed`;
`fossil info` prints the 40-hex `checkout:` line; `fossil open -f` plus
`addremove` + `commit` builds the fixture; `fossil diff --brief --checkin
<rev>` lists a revision's changed paths.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| B1 | Blocker | The design's flowchart put the Project clean-state witness inside Fresh only, while the code runs it before the branch; after a hook rejection the staged/recorded merge is dirty, so a literal implementation would block the rerun with "uncommitted work" and both repairable scenarios would fail. | `design.md` now states the witness ordering explicitly: skip `Witness::Project` on the unpublished-candidate republish path, keep it for fresh and published-candidate resume; `Backend::publish`'s dirty-set validation is the exactness guard. |
| B2 | Blocker | The Fossil hook-rejection scenario never reran after the hook was removed, so the change's core repair mechanism (`revision_contains` false branch plus Fossil republish) had no scenario. | The generic "A rejected candidate revision leaves the archive unproven and repairable" scenario became a `<backend>` outline over Git and Fossil, asserting the rerun proves the archive. |
| B3 | Blocker | The Git implementation of `revision_contains` was unspecified; an index-based check (`git ls-files`) would wrongly report a rejected candidate as published because the index keeps the staged merge. | `design.md` specifies committed-revision-only checks: `git cat-file -e HEAD:<path>`, `fossil cat <path>`. |
| S4 | Should-fix | The refusal scenarios asserted only that the change directory survived, so a bug that merged first and refused later could pass; the design claimed the shared `no file in the project changed` Then but the delta did not use it. | `no file in the project changed` added to the no-backend, nested-root, Fossil-EXTRA, Fossil-edited, and missing-`fossil` scenarios; the refusal Givens take the tree snapshot (`snapshot_tree` becomes `pub(crate)`). |
| S5 | Should-fix | Only the Fossil `EXTRA` dirty state was covered; a parser returning only `EXTRA` would pass every delta scenario. | New scenario "A Fossil edited file blocks the archive" covers the `EDITED` state. |
| S6 | Should-fix | The candidate Then asserted publication but not the use-case rule "the candidate revision contains exactly the merged corpus paths". | New Then `the candidate revision contains exactly the merged corpus paths` added to the candidate-publication outline; the design names the Git/Fossil path-listing helpers. |
| S7 | Should-fix | `commit_fixture` learning the backend by marker sniffing would send the `scm-backend-selection` fake `.fslckout` fixture into `fossil addremove` on a non-repository. | `TestEnv` gains a backend field set at construction; `commit_fixture` branches on it and never sniffs markers. |
| S8 | Should-fix | The step-text strategy was a summary; the renamed Thens and new Whens had no binding plan. | `design.md` enumerates the backend-parameterized Givens, the renamed Thens, the replaced bindings, and the reused refusal Thens with their files. |
| S9 | Should-fix | `fossil changes --differ` prints `RENAMED    old  ->  new`; a "rest of line is the path" parser would produce `old -> new` and fail the prefix validation. | The design records the split-on-`->` destination rule and adds a parser unit test (no archive scenario can produce a rename). |
| S10 | Should-fix | `sudo apt-get install -y fossil` on a fresh runner may have no package lists. | CI step is `apt-get update && apt-get install -y --no-install-recommends fossil`. |
| S11 | Should-fix | With the executable probe in `select`, a fossil-less machine gets a different refusal than the permanent scenario expects. | `fossil` is recorded as a hard suite prerequisite (CI installs it, `AGENTS.md` records it); the scenarios fail loudly rather than skipping. |
| S12 | Should-fix | TC-33's new text was garbled (a refusal list mixed with the selection case), and TC-32 did not say how a root inside an ancestor Fossil checkout is handled. | TC-33 rewritten as selection-then-refusals; TC-32 records the no-version-control fall-through. |
| S13 | Should-fix | The binary declares its own module tree; `src/fossil.rs` must be registered in `src/lib.rs` and `src/main.rs`. | Both registrations added to the design's Production Impact. |
| S14 | Should-fix | The design's shorthand `fossil diff --checkin --brief` omits the required revision argument, and `fossil info` exits 0 without a `checkout:` line on a fake marker. | Corrected to `fossil diff --brief --checkin <rev>`; `revision()` must error on a missing `checkout:` line. |
| N15 | Note | ADR-072's evidence overclaimed that the Gherkin specifies candidate contents; ADR-070's Git-worktree consequence became stale. | ADR-072 evidence reworded (the exact-contents assertion now exists); ADR-070's consequence generalized with an ADR-072 pointer. |
| N16 | Note | The design omitted the `adr-qualification-gate` count step binding and the permanent-spec alignment from its file lists. | Both added to Production Impact and Test Design. |

## Branch dispositions

- **Scope** — the spec matches the proposal after the S5/S6 additions; the
  Fossil managed upgrade stays out of scope with a designed named refusal and
  an arc42 handoff (D-20).
- **Tech choices** — shelling out to `fossil` mirrors `git.rs`; no simpler
  alternative preserves candidate exactness (path-scoped commits cannot
  express the moved change directory).
- **Missing scenarios** — none after B2/S5/S6; the already-proven, legacy, and
  corrupt refusals and the per-scope invocation counts are backend-independent
  and remain covered by the permanent scenarios on Git.
- **Testability** — every delta scenario can fail in RED; B1/B3 remove the two
  implementation traps that would have made GREEN impossible to reach
  honestly; parser-only paths get unit tests.
- **Risk** — the most likely failure is the witness ordering (B1); the
  repairable outline is the regression guard on both backends.
- **Use-case and Persona context** — `archive-change` (existing, modified,
  `archive-proof`), includes `verify-change` and the support fragment; the
  three confirmed Personas are review lenses, never Gherkin actors.
- **Visual design contract** — not applicable: CLI text only, no rendered
  screen; the design records the exemption and the opposite classification was
  tested (nothing renders HTML/CSS/JS).
- **Interface classification** — CLI; the E2E outline drives the compiled
  binary, so no browser driver applies.
- **Persona disposition** — developer--primary-user (repairable failures),
  agent--technical-role (deterministic commands), givn-maintainer--owner (one
  seam, truthful docs). No deferrals.
- **Interaction coverage** — the three archive-proof inventory rows match
  their `@e2e` titles verbatim; the outline counts once per the parser test.
- **ADR qualification** — ADR-072 passes all five mandatory dimensions with a
  completed existing-register check; the resume rule, the temporary
  managed-upgrade refusal, and the receipt field rename are correctly not
  ADRs (Gherkin/design/code own them).
- **Architecture documentation** — the independent 12-row walk matched
  `arc42.md` (all Yes); every touched chapter was opened and matches the
  design; ADR-072 is registered with 28 accepted records; no ASCII art.
- **Ubiquitous language** — revision, checkout, clean state, candidate/tested
  revision, witness-bounded publication, and unpublished candidate are in the
  glossary and used consistently; old on-disk receipts stay readable because
  no reader uses the renamed fields.

## Hardening verification

`givn lint --change archive-change` exits 2 with 12 `@wip` findings and no
parse, duplicate, or use-case findings. The delta now carries 17 scenarios
(4 modified, 9 added, 4 removed) across the two capability files.

DESIGN-REVIEW: PASS
