# Tasks: guard-retained-paths-across-archive

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`), both already set in `givn/commands.yaml`. Strict mode:
`.fail_on_skipped()` on the cucumber builder in `tests/features_runner.rs`; an
undefined or skipped step fails the run. Empty-body ban: no step body is `{}`,
a bare `pass`, a bare `return`, or whitespace. Unimplemented Rust steps use
`todo!` / `unimplemented!`. This change installs no runner and changes no
command, so strictness is proven once in `## Setup`.

Outline parameters: `<entry>`, `<moved>`, and `<archived>` of the refusal
Outline are `Examples` values substituted before matching; bind each phrase
with `regex = r#"..."#`, never as a literal `<entry>` binding.

Single-scenario run command (`<feature>` is the path under the change's
`specs/`, design.md "Step Definitions"):

```sh
GIVN_FEATURES=givn/changes/guard-retained-paths-across-archive/specs \
  cargo test --locked --test features_runner -- \
  "<capability>/<feature>.feature::<title>" --exact --nocapture
```

Release note (every batch commit subject, verbatim from `proposal.md`):

```text
A project with `ise` effective can no longer archive a change while a declared retained path points inside the moving change directory: the archive names the entry, the moved path, and the archived path to declare instead, and the served `ise` guidance states that retained paths must survive archiving.
```

Batch commit rule: one atomic
`givn commit -m "<release note>"$'\n\n'"Batch: <batch-name>"$'\n'"Scenarios: <titles>" -- <paths>`
per batch with explicit paths. Every scenario of the batch records the same
revision as `COMMIT: <hash> -- Batch: <batch-name>`; the gate reads the first
`COMMIT:` line under the scenario line, so a scenario section carries exactly
one. `givn check review` passes a scenario commit only when the commit message
contains the batch name.

**Evidence fields.** Each scenario carries the RED slot (observed command and
failure, or the protected requirement plus its discrimination probe), the GREEN
slot, and the commit line.

**`@wip`.** Each batch removes `@wip` from the scenarios it starts, never in
bulk. `givn check tasks` refuses the change while any selected delta scenario
still carries `@wip`.

**Base digest recompute rule (mandatory from design.md).** The delta
`specs/archive-change/usecase.md` line 1 records
`givn:base-sha256:1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82`,
the digest of `givn/specs/archive-change/usecase.md` at authoring time. After
any rebase or any revision that replaces the permanent
`givn/specs/archive-change/usecase.md`, recompute it and update delta line 1
before archive:

```sh
sha256sum givn/specs/archive-change/usecase.md
head -1 givn/changes/guard-retained-paths-across-archive/specs/archive-change/usecase.md
```

## Batch map

| Batch | Scenario entries | Executed rows | Owning capability |
|---|---|---|---|
| `setup` | 0 | 0 | none |
| `retained-path-guard` | 5 | 6 | `require-durable-retained-paths` |
| `guidance` | 1 | 1 | `instruction-integrity` |
| `archive-success-e2e` | 1 (`@e2e`) | 1 | `require-durable-retained-paths` |
| **Total** | **7** | **8** | 6 `require-durable-retained-paths` + 1 `instruction-integrity` |

The delta holds seven scenario declarations: six in
`specs/archive-change/require-durable-retained-paths.feature` and one in
`specs/fragments/instruction-integrity.feature`. Expanding the refusal
Outline's two `Examples` rows gives eight executed scenarios. Every title
appears in exactly one batch by exact title below.

## Interaction coverage pre-flight

The owning use case's `## Interactions` table holds one
`require-durable-retained-paths` row, and the design's Interaction Coverage
Matrix has exactly one matching row targeting the `@e2e` scenario in the delta
feature.

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/archive-change/usecase.md` | 1 `require-durable-retained-paths` | 1 | yes, added by this delta |
| `specs/fragments/instruction-integrity.feature` | 1 (no interaction row; support fragment) | 0 | not applicable |

No gap: the one inventory entry has its matrix row and its `@e2e` scenario in
this delta; the refusal, bounded, malformed, broad-root, and `ise`-off outcomes
are variants of the same consumer action, not entries.

## Domain Constraints

Copied from `specs/archive-change/usecase.md` (preconditions, main flow 1,
extensions 1, rules, success guarantee, Personas) and design.md's Decisions
ledger (D1–D3, Q1–Q5). Each scenario's Design constraints repeat the rules its
behavior exercises.

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `require-durable-retained-paths` | Main flow 1: while `ise` is effective, archive refuses a declaration whose retained path the move would orphan, before any mutation. | `agent--technical-role`, `givn-maintainer--owner` |
| `require-durable-retained-paths` | Extension 1: a retained entry that matches moved content and neither a surviving content path nor an archived counterpart is refused, naming role, entry, moved path, and the archived path to declare. | `developer--primary-user` |
| `require-durable-retained-paths` | Rule: entry matching uses the declaration's path patterns; a glob spanning the change tree is refused like a literal, an entry that still resolves is unaffected, and the advice keeps the entry's shape (`givn/changes/**` → `givn/archive/**`). | `developer--primary-user` |
| `require-durable-retained-paths` | Rule: the guard runs on the fresh move only; success guarantee: the refusal precedes the pending receipt write and the rename, so the change tree and declaration are unchanged and no candidate exists. | `givn-maintainer--owner` |
| `require-durable-retained-paths` | Rule: the guard reads `givn/specification.yaml` through the one strict declaration loader and only while `ise` is effective; a declaration the loader refuses blocks the archive by name; `ise` off reads no declaration. | `agent--technical-role` |
| `instruction-integrity` | The served `ise` instruction states that a declared retained path must survive `givn archive`, names the archive location a change's artifacts move to, and keeps the five rebuild obligations. | `agent--technical-role` |

Carried by every batch: the guard never writes `givn/specification.yaml`; no
`source:` root is examined (D1); no pending-candidate re-check exists (D3); the
guard compares project-relative `/` paths from `Backend::content_paths` and
never follows a symlink (Q4); the refusal list is bounded at twenty entries.

## Setup

- [x] Read the owning `specs/archive-change/usecase.md` (preconditions, main
      flow 1, extensions 1, rules, success guarantee, Personas, Interactions),
      the `support` fragment, `design.md` (Guard Algorithm, Internal
      Primitives, Decisions), and `design-review.md`. Domain constraints are in
      `## Domain Constraints` above.
      ```
      read: usecase.md Preconditions, Main flow 1, Extensions 1 (refusal,
        cannot be loaded), Rules (retained-path, pattern, fresh-only, loader),
        Success guarantee, Personas, Interactions; design.md Overview,
        Use-Case Traceability, Architecture Impact, Decisions D1-D3/Q1-Q5,
        Guard Algorithm, Internal Primitives, External Boundary Cases,
        Step Definitions, Strict Mode, E2E Smoke Test Infrastructure;
        design-review.md blockers, decisions, branch walk, sign-off.

      base digest verified:
      $ sha256sum givn/specs/archive-change/usecase.md
      1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82  givn/specs/archive-change/usecase.md
      $ head -1 givn/changes/guard-retained-paths-across-archive/specs/archive-change/usecase.md
      <!-- givn:base-sha256:1d7d8d8bd7db3ed06a2afb4401b2c4a001bf2458fc55ece51b418424b5751e82 -->
      ```

- [x] Confirm the runner and both commands; change neither. cucumber 0.23.0
      from `Cargo.lock`, `.fail_on_skipped()` in `tests/features_runner.rs`,
      step files registered in `tests/steps/mod.rs`.
      ```
      $ grep -n 'fail_on_skipped' tests/features_runner.rs
      55:        .fail_on_skipped()
      157:        .fail_on_skipped()
      verify.command:     ./run-tests.sh --non-e2e
      verify.e2e_command: ./run-tests.sh --e2e
      ```

- [x] Proof-of-strictness. Write a scratch feature outside the delta with one
      unbound step, run it through the change-scoped runner, confirm non-zero
      exit, then delete it and show the delta's `@wip` scenarios are the only
      non-green selection.
      ```
      $ mkdir -p target/strict-probe && printf 'Feature: Probe\n  Scenario: Strictness probe\n    Given an unbound probe step\n' > target/strict-probe/probe.feature
      $ GIVN_FEATURES=target/strict-probe cargo test --locked --test features_runner
      Feature: Probe
        Scenario: Strictness probe
         ✘  Given an unbound probe step
            Step failed:
            Defined: target/strict-probe/probe.feature:3:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed; exit 1
      $ rm -rf target/strict-probe
      $ GIVN_FEATURES=givn/changes/guard-retained-paths-across-archive/specs \
          cargo test --locked --test features_runner -- --tags 'not @wip'
      [Summary]
      0 features
      0 scenarios
      0 steps
      exit 0
      ```

## Batch: retained-path-guard

Files: `src/cli/archive.rs` (guard and helpers), `src/e2e_boundary/paths.rs`
(the moved matcher), `src/e2e_boundary/mod.rs` (re-export),
`src/ise/partition.rs` (import), `tests/steps/require_durable_retained_paths_steps.rs`
(step definitions), `tests/steps/mod.rs` (module registration).

- [x] Scenario: Archive is blocked when a declared retained path moves with the change
  - Design constraints: The guard is
    `ensure_declared_paths_survive_archive(&project_root, &backend, &manifest, change)`,
    called in `cli::archive::run` inside `if fresh { ... }` after
    `ensure_change_receipt` and before `ensure_artifact_gates`, before any hook,
    receipt write, rename, or publication. It returns `Ok(())` unless
    `SpecificationDeclaration::is_effective(manifest)`; otherwise it loads the
    declaration through `SpecificationDeclaration::load(project_root)`
    (`Ok(None)` passes) and reads `Backend::content_paths()` once, formatting
    each path project-relative with `/` and sorting. `moved` is every path at or
    under `givn/changes/<id>/`; `surviving` is every other path; `archived` is
    `archived_counterpart` of every moved path
    (`givn/changes/<id><rest>` → `givn/archive/<id><rest>`). For each
    `(role, pattern)` of `declaration.roles()` (`source_roots()` is never read),
    compile `Glob`, collect the moved paths
    `glob_matches_path_or_parent` matches; skip the entry when there are none;
    accept it when it matches any surviving or archived path; otherwise it is an
    offender. The reported moved path is the shallowest prefix of the first
    matched moved path at or below `givn/changes/<id>` that the pattern matches.
    The advice replaces the leading `givn/changes` segments with `givn/archive`.
    Refuse with `archive blocked: a declared retained path is moved by this
    archive:`, one line per offender
    (`  - <role> '<entry>' matches '<moved>'; declare '<advice>'`), the bounded
    remainder, then `A retained role names a path that survives \`givn archive\`.`
    and the repair sentence naming `givn/specification.yaml` and the change.
  - RED: `GIVN_FEATURES=givn/changes/guard-retained-paths-across-archive/specs
    cargo test --locked --test features_runner -- --tags 'not @e2e and not
    @wip'` -> both rows fail on
    `Then the command exits non-zero`: `Step panicked. Captured output:
    expected failure, got exit Some(0)`, with the unguarded archive reporting
    `Archived 'moved-limits'` and `Change archived successfully.`
  - GREEN: same scope -> `1 feature / 6 scenarios (6 passed) / 47 steps (47
    passed)`. Both rows print the refusal naming the entry, the moved path,
    and `givn/archive/**` or the archived literal.
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

- [x] Scenario: The refusal reports at most twenty isolated retained paths
  - Design constraints: `MAX_REPORTED_RETAINED_PATHS = 20` caps the refusal
    lines; offenders are sorted by role declaration order then entry text, the
    first twenty print, and the remainder is appended as `+N more`. The fixture
    declares 21 literal `limits` entries under
    `givn/changes/many-limits/` and writes those 21 files, so every entry
    matches moved content and nothing else.
  - RED: same scope -> `Then the command exits non-zero` fails with
    `expected failure, got exit Some(0)`; the unguarded archive lists no
    retained path.
  - GREEN: same scope -> the scenario passes; stderr lists 20 `- limits '...'`
    lines and `  +1 more`, with the 21st entry absent.
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

- [x] Scenario: Archive is blocked when the declaration cannot be loaded
  - Design constraints: `SpecificationDeclaration::load` wraps a refusal as
    `archive blocked: the declared retained paths cannot be read: {error}`; the
    loader's own message carries `givn/specification.yaml:` and the reason
    (for an unknown key, `unknown key '<key>'`). The branch is defensive: the
    change receipt runs first, so the fixture refreshes the receipt after
    writing the malformed declaration.
  - RED: same scope -> `Then the command exits non-zero` fails with
    `expected failure, got exit Some(0)`; the unguarded archive never reads the
    malformed declaration.
  - GREEN: same scope -> exit non-zero, stderr
    `archive blocked: the declared retained paths cannot be read:
    givn/specification.yaml: unknown key 'unknown_key' — ...`, the change
    directory present, and no candidate.
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

- [x] Scenario: A retained root that also covers the archive location is unaffected
  - Design constraints: An entry survives iff it matches a surviving content
    path or an archived counterpart, so `givn/**` is accepted: it matches moved
    content and surviving content (for example `givn/specs/`,
    `givn/config.yaml`). A naive "refuse whenever moved content matches" is
    rejected. Existing-behavior protection with a discrimination probe; no
    production diff is manufactured for this requirement.
  - RED: existing behavior protected. Probe: with the survival check replaced
    by `let survives = false;` (any moved match refuses), the scenario fails on
    `Then the command exits 0`: `expected exit 0, got Some(1): stderr=error:
    archive blocked: a declared retained path is moved by this archive:
    - limits 'givn/**' matches 'givn/changes/broad-limits'; the archived paths
    must still match 'givn/**'` -> `1 scenario (1 failed)`. Restored.
  - GREEN: the scenario passes (exit 0, change moved) because `givn/**` also
    matches surviving content (`givn/config.yaml`, `givn/specs/`).
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

- [x] Scenario: A project with ise off archives without reading the declaration
  - Design constraints: `SpecificationDeclaration::is_effective(manifest)` is
    the early return; with `addons.ise` off the declaration file is not read, so
    a `limits` entry naming `givn/changes/<id>/design.md` archives normally.
    Existing-behavior protection with a discrimination probe; no production diff
    is manufactured for this requirement.
  - RED: existing behavior protected. Probe: with the
    `is_effective(manifest)` early return removed, the guard reads the
    declaration with `ise` off and the scenario fails on `Then the command exits
    0`: `expected exit 0, got Some(1): stderr=error: archive blocked: a declared
    retained path is moved by this archive: - limits
    'givn/changes/ise-off/design.md' matches 'givn/changes/ise-off/design.md';
    declare 'givn/archive/ise-off/design.md'` -> `1 scenario (1 failed)`.
    Restored.
  - GREEN: the scenario passes; the declaration is present but unread.
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

- [x] Unit tests for the predicate in `src/cli/archive.rs` (part of this
      batch's commit, not a scenario): `declared_retained_path_offenders` over a
      synthetic content listing covers a literal under the change tree refused
      with its archived literal advice, an entry matching only the archived
      counterpart accepted, `givn/changes/**` refused with `givn/archive/**`
      advice, `givn/**` accepted, a literal for another active change
      unaffected, an entry matching a surviving content path accepted, and the
      twenty-entry bound with `+1 more`.
  - `cargo test --locked --lib -- cli::archive::tests` -> 26 passed, 0 failed;
    the seven new tests are
    `a_literal_under_the_change_tree_is_refused_with_its_archived_path`,
    `an_entry_matching_only_the_archived_counterpart_survives`,
    `a_wildcard_spanning_the_change_tree_is_refused_with_its_archived_pattern`,
    `a_broad_root_covering_surviving_content_is_accepted`,
    `a_literal_for_another_active_change_is_unaffected`,
    `an_entry_matching_a_content_path_outside_the_move_is_accepted`, and
    `the_refusal_lists_twenty_offenders_and_counts_the_rest`.
  - [x] COMMIT: 710e0a7c7a666b61296662d248ece030203f116e -- Batch: retained-path-guard

## Batch: guidance

Files: `tests/steps/instruction_integrity_steps.rs` (step bindings), the delta
feature's `@wip` removal. The served text landed in `d160e7ed`
(`assets/instructions/ise.md`, `assets/instructions/archive.md`), so this
scenario protects existing behavior and records a discrimination probe.

- [x] Scenario: The ise instruction says declared retained paths survive archiving
  - Design constraints: The Given enables `ise` and synchronizes skills through
    `tests/common`; the When runs `givn instructions ise`; the Then asserts the
    served text names the retained-path survival rule (`must survive`)
    and the archive location (`givn/archive`). `assets/instructions/ise.md`
    states both in the Specification-completeness obligation, and the five
    rebuild obligations keep their count.
  - RED: existing behavior protected. Probe: deleting the retained-path
    sentence from `assets/instructions/ise.md` makes the scenario fail on
    `Then the served instruction says a declared retained path must survive
    `givn archive``: `the served ise instruction omits the retained-path
    survival rule` -> `1 scenario (1 failed)`. Restored.
  - GREEN: `GIVN_FEATURES=givn/changes/guard-retained-paths-across-archive/specs
    cargo test --locked --test features_runner --
    "fragments/instruction-integrity.feature::The ise instruction says declared
    retained paths survive archiving" --exact --nocapture` -> `1 scenario (1
    passed)`. The served text is hard-wrapped, so the step normalizes
    whitespace before matching.
  - [x] COMMIT: a641ed616a05dbbf697d18f7f382d6529b0674d9 -- Batch: guidance

## Batch: archive-success-e2e

Files: `tests/steps/require_durable_retained_paths_e2e_steps.rs` (step
definitions plus the transcript capture), `tests/steps/mod.rs` (module
registration), the delta feature's `@wip` removal, and the rewritten
`evidence/visual/consumer-archives-a-change-whose-declared-retained-paths-survive-the-move/transcript.txt`.

- [x] Scenario: Consumer archives a change whose declared retained paths survive the move
  - Design constraints: The `@e2e` driver is the compiled binary
    (`CARGO_BIN_EXE_givn`) run as a subprocess through `tests/common`. The Given
    builds the `ready_change` Git fixture, enables `ise`, and declares
    `limits: ["givn/archive/durable-paths/design.md"]`; the generic archive When
    step commits the fixture and refreshes the change receipt. The Then asserts
    exit 0, the moved change, and `design.md` under
    `givn/archive/durable-paths/`. The declaration names the archived location,
    so it matches no moved content and the guard never refuses. The step
    rewrites the committed transcript from the executed run (terminal
    interface, no viewport).
  - RED: existing behavior protected. Probe: adding an existence requirement
    (an entry that matches no content path is an offender) makes the guard
    refuse `givn/archive/durable-paths/design.md`; the scenario fails on `Then
    the command exits 0`: `expected exit 0, got Some(1): stderr=error: archive
    blocked: a declared retained path is moved by this archive: ...` -> `1
    scenario (1 failed)`. Restored.
  - GREEN: `GIVN_FEATURES=givn/changes/guard-retained-paths-across-archive/specs
    cargo test --locked --test features_runner --
    "archive-change/require-durable-retained-paths.feature::Consumer archives a
    change whose declared retained paths survive the move" --exact --nocapture`
    -> 6 steps passed, `1 scenario (1 passed)`; the step rewrote
    `evidence/visual/consumer-archives-a-change-whose-declared-retained-paths-survive-the-move/transcript.txt`
    from the executed run (`exit: 0`, archived `design.md`, 116 bytes).
  - [x] COMMIT: d8fbea448dfb180c5c01feef1b3cabd24673a250 -- Batch: archive-success-e2e

## Resolved during implementation

1. **Two step phrases already had bindings.** `stderr says "<text>"` is bound in
   `tests/steps/require_merged_step_definitions_steps.rs:173`, and
   `a Givn project with the <addon> addon enabled and the skills synchronized`
   is bound generically in
   `tests/steps/template_conditional_rendering_steps.rs:39`. Draft bindings in
   the new step file made cucumber report `Step match is ambiguous`; both were
   deleted and the existing definitions reused. No design statement changes:
   the design names the shared Givens, not their file.

2. **A literal cannot match only the archived counterpart.** The design's unit
   test list asks for "a literal under the change tree that survives only via
   the archived counterpart". A literal that matches a moved path cannot also
   match that path's `givn/archive/` form, so the archived-counterpart branch is
   exercised with `givn/*/moved-limits/design.md`
   (`an_entry_matching_only_the_archived_counterpart_survives`), and the
   literal case asserts the archived-literal advice
   (`a_literal_under_the_change_tree_is_refused_with_its_archived_path`). The
   D1 survival predicate is unchanged.

3. **Declared paths that match nothing are not offenders.** The guard's
   offender set is "matches moved content and nothing else", so the fixtures
   that exercise the refusal must write the files their `limits` entries name
   (the 21-path Given writes them); a declaration naming a nonexistent path
   under the change tree is skipped, exactly as D1 specifies. The success
   scenario relies on the same rule: `givn/archive/<id>/design.md` matches no
   moved content before the move and is accepted.

4. **Served guidance is hard-wrapped.** `givn instructions ise` preserves the
   asset's line breaks, so the retained-path sentence spans lines. Both
   guidance steps collapse whitespace before matching.

---

## Verification runs

Both commands come from `givn/commands.yaml` and run after the three batch
commits.

```text
$ ./run-tests.sh --non-e2e
cargo fmt --all -- --check                     -> clean
cargo clippy --locked --all-targets -- -D warnings -> clean
Summary [ 186.281s] 1054 tests run: 1054 passed, 113 skipped
exit 0

$ ./run-tests.sh --e2e
Summary [  39.557s] 113 tests run: 113 passed, 1054 skipped
exit 0
```

The e2e run rewrote six committed screenshots under
`givn/archive/development-timeline/evidence/` (the agent-activity scenarios
recapture their archived evidence). They were restored with
`git checkout -- givn/archive/development-timeline/evidence`, so the change's
diff contains no rewritten evidence.

## Review gate

- [x] Final review gate: `givn check review --change
      guard-retained-paths-across-archive` exits 0 at the review PASS revision
      `faf2cf06`, with the receipt reused across the documentation-only
      revision. Shape and subset matches are advisory; their dispositions are
      recorded in `review.md`.
      ```
      $ givn check review --change guard-retained-paths-across-archive
        ✓ integrity checks passed
        ✓ run declaration passed
        givn lint: 2 file(s) checked — clean
        net delta: 7 added, 0 modified, 0 removed
        ✓ overlap dispositions passed
        ✓ scenario commits passed (7 scenario(s))
        ✓ change verification receipt reused across documentation-only revision
          faf2cf06099cc696dc2c4ee9153c4054f7221655 (tested at 9be16da3f534be0e
          04736a83273622cff7d8e30f, 7 scenario(s))
      exit 0
      ```

---

## Done when

All boxes are checked, every scenario records its batch commit, both
`verify.command` and `verify.e2e_command` exit 0 on the full suite, and
`givn check tasks --change guard-retained-paths-across-archive` exits 0.
