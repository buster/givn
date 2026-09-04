# Plan: retire the semantic gate, group specs by use case, fix routing

Read this file once, then work from `TASKS.md`. `TASKS.md` is the executable
checklist; this file is why each task exists. If a task in `TASKS.md` seems
to contradict this file, this file wins — stop and ask rather than guessing.

## 1. Why this change

A prior report (`SPEC_ORGANIZATION_REPORT.md`, repo root) proposed slashing
E2E scenario counts, based on a claim that "81% of scenarios are E2E." That
claim is wrong — it counted *files containing at least one* `@e2e` scenario
(44/53 files), not e2e scenarios. The real number is **116 @e2e out of 700
scenarios = 16.6%**. Do not use that report as a source of requirements.

The real problems, measured directly against `givn/specs/` and the archived
changes under `givn/archive/`:

1. **~14 capabilities (~2,000 of 6,037 lines) specify a semantic-similarity
   engine (E5/BGE/NLI embeddings) that is being turned off.** There is no
   way to remove a capability today — only scenarios can be removed
   (`DeltaOp::Removed`), never a whole capability. Two capabilities already
   exist as empty 1-line corpses proving this
   (`ci-debuginfo-optimization`, `semantic-scoring-throughput`).
2. **Archive re-executes the entire permanent spec corpus in two places**
   (`src/cli/archive.rs` post-merge verify/verify-e2e, and
   `src/cli/coverage_gate.rs`'s measure command), both with
   `GIVN_FEATURES=givn/specs`, both rolling back on failure. If the semantic
   capabilities' step definitions are compiled out before the specs
   describing them are deleted, **every future `givn archive` breaks
   permanently.** Retirement must happen before the Cargo features are
   dropped.
3. **~9-12 of 53 capability directories are actually change names**
   (`release-0-3-0`, `fix-uppercase-checkbox-parsing`, …). There is no step
   in the workflow that decides whether new spec content belongs in an
   existing capability or a new one — the agent just creates one directory
   per change.
4. **The step-phrasing vocabulary is heavily distorted, and it is
   self-inflicted.** Measured on authored deltas, split at the date the
   semantic similarity gate landed (2026-08-16): phrasing reuse dropped from
   **4.46× before the gate to 1.70× after it** (median per-change: 2.55× →
   1.24×) even on changes with nothing to do with the semantic engine. The
   gate scored specs on *not resembling each other*, so specs stopped
   resembling anything. This is the strongest reason to remove it — not
   speed, not resource cost.
5. **The existing E2E-per-interaction rule is correct but unenforceable.**
   `assets/instructions/specs.md` already says "one `@e2e` scenario per
   distinct user action, not per file." It cannot be enforced because the
   inventory that defines "distinct user action" lives in a `#` comment
   block at the top of the delta `.feature` file, and
   `src/gherkin/merge.rs` discards feature-level comments when merging into
   the permanent spec. Of 65 archived deltas carrying that inventory
   comment, **1** survives into `givn/specs/`.

## 2. What we are doing about each

| problem | fix | phase |
|---|---|---|
| no capability retirement | new `@givn.retired` delta op that deletes a capability directory, with rollback | 0 |
| archive can brick when features are dropped | retire dead capabilities *before* dropping the Cargo features | 1 |
| change-named capabilities | `givn spec route` narrows to `EXTEND`/`NEW` candidates at propose time; recorded in `proposal.md`; checked (not re-derived) at review time | 4 |
| phrasing distortion | fold remediation into the group.md authoring pass in Phase 2, target the *measured* pre-gate baseline (4.46×/2.55×), report only, never gate | 2.8 |
| unenforceable e2e rule | move the interaction inventory into `group.md` (which is never merged, so it survives), gate `@e2e` count against it | 3 |
| dead corpus / no groups | `givn/specs/<group>/<capability>.feature`, one `group.md` per group, derived from real command-surface clustering | 2 |
| discovery tools disappear with embeddings | deterministic `spec tree` / `spec find` / `spec route`, no embeddings, no score | 4 |
| pre-existing gate bugs found during this work | fixed as Phase 5, independent of the above | 5 |

## 3. Explicitly rejected approaches (do not re-propose these)

- **"Exactly one `@e2e` scenario per capability file."** Rejected: 9 of 53
  files legitimately have zero `@e2e` today (e.g. `lint-command`, 25
  scenarios) — this rule would force manufacturing e2e coverage that isn't
  warranted. It also breaks capabilities where one interaction fans out
  across several CLI entry points (`semantic-model-download`'s 6 e2e cover
  5 distinct commands). The correct unit is the *interaction*, not the
  file — see Phase 3.
- **A registered/controlled step-phrasing vocabulary.** Would be the most
  direct fix for the reuse-ratio problem, but the problem is iatrogenic
  (caused by the gate we're removing). Building permanent machinery to
  counteract a pressure that no longer exists is over-engineering. Instead:
  one remediation pass per group (Phase 2.8), measured against history,
  reported not gated.
- **A blocking threshold on phrasing similarity/reuse.** This is a
  similarity score with different math. Section 4 of this file is the
  demonstration of what happens when spec text is optimized against a
  similarity score. Report the ratio in `spec tree`; never gate on it.
- **A size band on scenario count per file (e.g. <3 or >40 triggers a
  finding).** Contradicts the e2e rule on real files: `coverage-readme-example`
  and `readme-coverage-badge` each have exactly 1 scenario, which is the
  e2e one. A size gate would force deleting real coverage to comply.
- **A fresh-subagent "qualification" reviewer that re-scores candidates and
  requires `DUPLICATE`/`VARIANT`/`VALID-BOUNDARY` classification+rationale.**
  This is the shape of the semantic-review gate we are removing. Its track
  record: 28 of 53 archived changes even produced the artifact, 18 of those
  are header-only, exactly 1 was ever `BLOCKED` — while it was simultaneously
  degrading the corpus (point 4 above). The replacement in Phase 4 is a
  **decision-consistency check**: compare a recorded structural decision
  (`EXTEND <cap>` / `NEW in <group>`) against what was actually built. This
  is a discrete filesystem fact, not a score, so there is no gradient to
  optimize against.

## 4. The measurement behind point 4 (for anyone who doubts it)

Authored deltas in `givn/archive/*/specs/**/*.feature`, step lines outside
docstrings, quoted spans/digits/whitespace normalized, split at the
semantic-review gate's landing commit (`semantic-review`, 2026-08-16):

| cohort | changes | steps | distinct phrasings | pooled reuse | median reuse per change |
|---|---|---|---|---|---|
| before the gate | 23 | 1,187 | 266 | 4.46× | 2.55× |
| after the gate, unrelated topics (e.g. `run-command`, `readme-enforcement`) | 12 | 805 | 473 | 1.70× | 1.24× |
| after the gate, semantic-engine topics | 16 | 2,032 | 1,017 | 2.00× | 1.72× |

The "semantic specs are just naturally more varied" explanation is ruled
out: the *unrelated*-topic cohort is the worst one. All 8 of the most
divergent individual changes post-date the gate.

Retiring the ~14 dead capabilities (Phase 1) does **not** fix this — it
removes 39% of all steps but only moves pooled reuse from 2.24× to 2.37×,
because the damage is spread across the whole post-gate corpus, not
concentrated in the semantic specs. Phase 2.8 is required.

## 5. The use-case grouping signal (for Phase 2)

Command-surface clustering of the current 53 capabilities (which `givn`
subcommand their scenario steps invoke) produces clean groups without any
human taxonomy work: `givn spec` ← 7 capabilities (mostly 100% surface
purity), `givn check` ← 5, `givn archive` ← 4, `givn status` ← 4,
`givn lint` ← 3, `givn new` ← 2, `givn upgrade` ← 2. The 10 capabilities
that invoke **no** command are `«include»` fragments, not use cases
(`config-overlay`, `instruction-integrity`, `template-conditional-rendering`,
`adr-qualification-gate`, `spec-explore-change`, plus 5 dying semantic
capabilities).

This signal correctly places two change-named capabilities:
`fix-uppercase-checkbox-parsing` → `givn status` (67% of its command
surface), `release-0-3-0` → `givn upgrade` (100%).

**Known false positive, keep this in mind while implementing `spec regroup`:**
a naive extractor also matched the prose phrase "Given a givn project
with…" as if it were a `givn project` command invocation, producing a
fake cluster of 8. Only match `givn <subcommand>` tokens that appear after
a step keyword (`Given`/`When`/`Then`/`And`) **and** match a real
subcommand from `SpecCommands`/`Commands` in `src/main.rs` — never a bare
substring match. This is why `spec regroup` proposes and a human confirms;
it must never auto-apply its own clustering.

## 6. Routing: what "deterministic" actually means here

Leave-one-out testing (each of 50 archived proposals routed against only
the capabilities that existed before it, with the capability actually built
excluded from candidates) showed the deterministic router:

- **Gets it right** when there's a command-surface signal:
  `semantic-e5-token-cap` → correctly suggests `EXTEND semantic-token-cap-review`.
  `release-0-3-0` → correctly suggests `run-command` group via `cmd:givn upgrade`.
- **Ties** when several existing capabilities share a surface
  (`spec-progress-output`: three candidates score within 0.26 of each
  other, all sharing `givn spec`).
- **Misses entirely** when the change is internal logic with no command
  surface (`fix-remediation-delta2delta`).
- **Gets pulled toward large capabilities** — a 29-scenario capability
  scores higher than it should just by having more surface to match against.

**Conclusion baked into the design:** `givn spec route` is a narrowing
tool, not a decision-maker. It emits ranked candidates with evidence; a
human/agent reads them and writes the decision. The gate at review time
(Phase 4) never recomputes or trusts route's ranking — it only checks that
the *recorded* decision matches what was *built*. This is why the gate
survives being wrong at proposal time: routing quality is advisory,
decision-consistency is enforced.

## 7. File index (where each phase's work lives)

- `src/gherkin/delta.rs` — `DeltaOp`, `capability_tag`, `is_delta_feature` (Phase 0)
- `src/cli/archive.rs` — merge/retire dispatch, rollback, post-merge verify (Phase 0, 1)
- `src/cli/coverage_gate.rs` — second full-corpus re-execution point (Phase 0, 1)
- `Cargo.toml` — feature flags (Phase 1)
- `assets/instructions/review.md`, `assets/templates/review.md` — semantic-review sections to delete (Phase 1); disposition headings to fix (Phase 5)
- `src/overlap/analysis.rs` — capability/group derivation, `duplicate_title` exemption, `scan_tree` (Phase 2, 5)
- `src/overlap/disposition.rs` — disposition table parser, `decision` field (Phase 5)
- `src/overlap/fingerprint.rs` — `mask()`, `tty` substring bug (Phase 5)
- `src/cli/check.rs` — retrieval gate dispatch, overlap gate call site (Phase 0, 4, 5)
- `src/cli/lint.rs` — overlap scan scoping (Phase 5)
- `src/config/overlay.rs` — `OverlayDoc`, artifact removal (Phase 5, referenced not changed by 2)
- `src/gherkin/merge.rs` — header synthesis that destroys the inventory comment (Phase 3, informational — the fix is to stop relying on the comment, not to change merge.rs)
- `src/main.rs`, `src/cli/spec.rs` — CLI wiring for new `givn spec` subcommands (Phase 4)
- `src/spec/retrieval.rs` — `collect_duplicate_scenarios`, the part of `duplicates` that survives (Phase 4)
- `assets/templates/proposal.md`, `assets/commands/givn-propose.md`, `assets/commands/givn-spec.md` — routing table home and instruction rewrite (Phase 4)
- `assets/instructions/specs.md` — canonical e2e policy, already correct, not rewritten, only re-homed (Phase 3)

## 8. Order of operations (do not reorder)

Phase 0 must land first (retirement primitive + disabled-feature-path fix)
or Phase 1 bricks archive. Phase 1 must land before Phase 2 touches
`analysis.rs` capability derivation, since Phase 1 shrinks the corpus you'd
otherwise have to migrate. Phase 2 must land before Phase 3, since Phase 3
puts the interaction inventory inside `group.md`, which doesn't exist until
Phase 2. Phase 5 fixes are independent and can land at any time, but land
them *before* Phase 2.8 remediation, since 5.1/5.2 change what
`shape_match` even detects.

**Phase 1 splits into two parts with a hard dependency on Phase 4, found
while enumerating the exact retirement list — do not skip this.** The 15
capabilities being retired (`TASKS.md` Phase 1 has the exact list) do not
include `spec-duplicates`, `spec-explore-change`, `spec-explore-permanent`,
or `spec-progress-output` — those describe *other* `givn spec` subcommands
(`duplicates`, `explore`, progress reporting) that Phase 4 rebuilds
deterministically rather than deletes. But today, every one of those
subcommands is fully gated behind `#[cfg(feature = "spec-retrieval")]` with
a `bail!` fallback (`src/spec/retrieval.rs`). **If Cargo.toml's default
features are dropped (Phase 1.2) before Phase 4's deterministic
replacements exist, those four capabilities' own specs start failing
`verify` for a reason unrelated to their actual (retained) scope.**

So: Phase 1.1 (retire the 15 dead capabilities) and Phase 1.3 (drop the
`semantic-review.md` sections) can land immediately. **Phase 1.2 (drop
`spec-retrieval`/`spec-similarity-gate`/`cross-encoder-reranker`/
`nli-evidence` from `Cargo.toml`'s `default` list) must wait until Phase 4
has shipped `spec tree`, `spec find`, `spec route`, and the deterministic
half of `spec duplicates`, and the four surviving `spec-*` capabilities'
specs have been rewritten to describe only that deterministic behavior.**
Track this as a named blocker, not an implicit ordering assumption.
