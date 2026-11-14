# Review: enforce-ise-e2e-boundary

Review kind: independent
Author: enforce-ise-e2e-boundary implementation author (omp, db765d4c)
Reviewer context: 9111d088-7854-4cfa-8bdc-08976f308d35 (fresh read-only grilling context; produced the branch coverage, the four questions, and the blocker set)

README-IMPACT: none
PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner
VISUAL-REVIEW: PASS
ARC42 CONFORMANCE: CLEAN

## Review context and scope

This is the independent review recorded by the proposal (`Review kind:
independent`). The reviewer did not author the change and received only the
artifacts, the repository state, and the finding brief. The recorded
design-review context (`9111d088-7854-4cfa-8bdc-08976f308d35`) is the one
review context the change records; this final review continues that independent
context and is not the authoring context (`db765d4c`).

Read: `proposal.md`, `design.md`, `design-review.md`, `arc42.md`, `tasks.md`,
both delta features, both delta use-case documents, `git log --name-only
1016f3bc..HEAD`, every changed production file (`src/e2e_boundary/**`,
`src/proof.rs`, `src/cli/check.rs`, `src/cli/lint/{mod,specification}.rs`,
`src/config/lock.rs`, `src/lib.rs`), the changed step files, the delta and
permanent guidance (`assets/instructions/{specs,ise}.md`), `docs/reference.md`,
`docs/glossary.md`, the affected arc42 chapters, and ADR-070.

Ran, read-only:

- `givn status --change enforce-ise-e2e-boundary` (78/85 boxes);
- `givn check review --change enforce-ise-e2e-boundary` (gate output in
  "Verification" below);
- `givn lint --change enforce-ise-e2e-boundary` (2 `SHAPE`, 0 `LONG`, 120
  `SUBST`, exit 2 for the change-scoped `@wip` findings);
- the delta suite, `GIVN_FEATURES=givn/changes/enforce-ise-e2e-boundary/specs
  cargo test --locked --test features_runner`: 28 scenarios (28 passed), 149
  steps (149 passed), 6.90 s;
- the permanent `check-specification-health` feature in a scratch scope
  (`GIVN_FEATURES=/tmp/perm-lint`): 57 scenarios (57 passed), 345 steps (345
  passed), 8.87 s, proving the moved resolver did not change lint output;
- `cargo clippy --locked --all-targets -- -D warnings` (clean) and
  `cargo run -- instructions` (exit 0);
- `givn instructions specs` and `givn instructions ise` on this repository
  (`ise` off).

It did not edit production code, step definitions, specs, design, tasks,
glossary, coverage, or README. The adversarial probes below ran in `/tmp`
fixtures against the built binary; no reviewed file was mutated.

## Tasks preflight

`givn status --change enforce-ise-e2e-boundary` reports **78/85** boxes. Every
Setup and scenario box is checked. The 7 unchecked boxes are the Closeout
checks:

- every delta phrase is bound in a permanent step file;
- no empty or no-op stub remains;
- no delta scenario carries `@wip`;
- `givn lint --change` reports no end-to-end intent finding;
- whole-change verification (fmt, clippy, nextest, both scopes, both delta
  scopes, lint);
- `givn check tasks` exits 0 and records the receipt;
- `givn check review` passes.

The 18 scenario boxes record batch revisions and the change verification
receipt at `2d7b1362` (28 change-scope scenarios) is current per the review
gate. **Finding F0:** the Closeout commands are not run and recorded, and the
boxes are not ticked. The author must run them and tick every box before
archive. This review supplies the delta-scope sample, the permanent lint
feature run, clippy, and the run gate, but not the required `./run-tests.sh
--e2e`, `./run-tests.sh --non-e2e`, and `cargo nextest run` records.

## Implementation evidence check

### 1. `@e2e` tag integrity

No delta scenario carries `@e2e`; the design says the delta adds no end-to-end
scenario. `grep -rn '@e2e' givn/changes/enforce-ise-e2e-boundary/specs/`
returns only narrative references in the two use-case documents. No permanent
`@e2e` tag was removed: the delta's two `@givn.modified` scenarios keep their
tags, and no delta file edits a permanent feature. 0 tag removals.

### 2. Empty or trivial step bodies

0 empty, whitespace-only, `todo!()`, `unimplemented!()`, or assertion-free step
bodies found across the 92 files under `tests/steps/`. The single `todo!()` hit
(`tests/steps/outline_report_support.rs:581`) is a Rust source fixture string
passed to the outline analyzer, not a step body. The new bindings are in
`tests/steps/workflow_enforcement_steps.rs:3530-4050` and
`tests/steps/prose_consolidation_steps.rs:480-720`; two delta phrases reuse
existing bindings (`specification_health_steps.rs:288` for the dependency-edge
Then, `archive_proof_steps.rs:742` for `stderr names "..."`), so no phrase is
registered twice.

### 2a. Characterization

`CHARACTERIZATION` is not used. This is a normal behavior change.

### 2b. Discrimination

`tasks.md` records observed RED for `receipt-identity` and `receipt-freshness`,
and revert-based RED (uncommitted production reverted with `git stash`) for the
three `e2e-scope` scenarios. The five `boundary-resolution` scenarios record
"RED admitted (steps TESTS.5)" because the resolver was implemented before the
bindings; no historical RED is claimed for them. I did not mutate the reviewed
tree. I did confirm the steps are not no-ops: the probes below produced the
exact refusal lines and receipt fields the steps assert, so each step fails
when that production path is absent. The three `served-policy` scenarios and
the two `pinned-policy-replacements` scenarios assert committed prose through
the served instruction; they pass on their first run by design (the design
records this drift-protection role), and they are not RED evidence.

### 3. Promised components

| Component (design) | As-built | Check |
|---|---|---|
| `E2eBoundary` (`resolve`, `conditions`, `identity`) | `src/e2e_boundary/{mod,conditions,manifest,imports,paths,hash}.rs` | present; `E2eBoundary::resolve` at `mod.rs:34`, `conditions` at `mod.rs:296`, `identity` at `mod.rs:246` |
| `ChangeReceipt` (`write`, `read`, `freshness`) with `E2eIdentity`, `ScopeOutcome` | `src/proof.rs:542-586`, `write_change_receipt` at `proof.rs:1204`, `freshness` at `proof.rs:880` | present |
| Blocking check integration | `src/cli/check.rs:469`, `581-687` | present |
| Read-only effectiveness helper | `src/config/lock.rs:126` | present; no schema change |
| Shared resolver consumed by lint | `src/cli/lint/specification.rs:165` | present; `src/cli/lint/manifest.rs` deleted and the hash moved to `src/e2e_boundary/hash.rs` |

Consumers import the two contracts only. `cli::lint`, `cli::check`, and
`proof` import `e2e_boundary`; no `e2e_boundary` module imports `proof` or a
`cli` module, so the dependency is one-way. No dead code: clippy with
`-D warnings` is clean and the `pub(crate)` surface is consumed.

### 4. Strict-mode proof

`tasks.md` Setup records the real-runner probe: a scratch `@e2e` scenario with
one unbound step exits 1 and writes `{"failed":1,"passed":0,"scope":"e2e",
"skipped":0,"total":1}`. Strict mode is present and exercised. The delta also
asserts distinct counts for the two runtime cases (`1 failed test` and
`1 skipped test`), so one generic message cannot satisfy both.

### 5. `@e2e` Then assertions

The delta contains no `@e2e` scenario (0 of 18). The permanent `@e2e` scenarios
that share these actions are outside the delta and unchanged; no downgraded
Then assertion exists in the delta. Not applicable, no finding.

### 6. Browser-UI driving

Not a browser capability. The product is a CLI, every delta scenario drives
the built binary as a subprocess, and no step sends HTTP or calls `fetch()`.

### 7. The exact E2E implementation

`verify.e2e_command` is `./run-tests.sh --e2e`. `run-tests.sh:78` runs
`GIVN_FEATURES="$FEATURES_PATH" run_cucumber '@e2e and not @givn.removed'`,
which invokes `cargo test --locked --test features_runner -- --tags ...`; the
only E2E harness is `tests/features_runner.rs`. I searched the tree for a
second implementation of the boundary's E2E steps: the delta adds none, and
`tests/steps/{workflow_enforcement_e2e_steps,prose_consolidation_e2e_steps}.rs`
exist with their existing content. No parallel, stronger implementation was
ignored.

### 8. E2E scope and the interaction inventory

The delta adds no inventory entry. The two owning documents hold 13
(`verify-change`) and 9 (`create-change`) interaction rows, the design matrix
has the same 22 rows, and each matrix target title resolves to a permanent
scenario carrying `@e2e` (verified in the owning files; the tasks pre-flight
records the same cross-check). The delta's boundary refusals, receipt
inspections, and guidance cases are variants of those actions, so 0 new
actions and 0 excess E2E scenarios.

### 9. Explicit scope selection

The check passes `GIVN_RESULT_SCOPE=change` for the regular run
(`check.rs:502-507`) and `GIVN_RESULT_SCOPE=e2e` for the E2E run
(`check.rs:626-631`); `run-tests.sh:20-40` honors it, and the receipt records
`result.scope: "change"` plus the `e2e_scope` outcome. The receipt read is
fail-closed on failed, skipped, zero, malformed, unreconciled, and wrong-scope
results.

### 10. Design against the as-built tree

| Design statement | As built | Verdict |
|---|---|---|
| Receipt fields `e2e_identity` and `e2e_scope` are optional and not serialized when absent (`design.md:288`) | `proof.rs:578-584` with `#[serde(default, skip_serializing_if = "Option::is_none")]`; field order otherwise unchanged | matches |
| `not_applicable` only without a delta `@e2e`, green scope otherwise (`design.md:211`, `:430`) | `check.rs:523-538`; `proof.rs:980-1010` | matches |
| The runner rule is textual and proves naming, not execution (`design.md:214`) | `e2e_boundary/mod.rs:369-381`; refusal text names the command and root | matches |
| The boundary preflight runs before the no-behavior-change return (`design.md:399-403`) | `check.rs:459-469` returns on `not_applicable` before the preflight | **deviates (B2)** |
| The dependency graph covers direct, transitive, and workspace-inherited edges (`design.md:358`, `docs/reference.md:146-160`) | `manifest.rs:76` reads only the three top-level tables; target-specific tables are ignored | **deviates (B1)** |
| Step Definitions: every delta phrase is bound in `workflow_enforcement_steps.rs` / `prose_consolidation_steps.rs` (`design.md` Step Definitions) | two phrases reuse permanent bindings in `specification_health_steps.rs` and `archive_proof_steps.rs` | wording drift only (O3), no behavior gap |

### 11. Interaction coverage cross-reference

Every inventory entry has a design matrix row, an existing `@e2e` scenario, and
a real CLI driving mechanism (a `givn` subprocess). The full mapping is the 22
rows of `design.md`'s Interaction Coverage Matrix; the tasks pre-flight table
reproduces it, and I re-verified that all 22 titles exist and carry `@e2e` in
`givn/specs/**`. No gap, no extra row, no changed driver.

## Arc42 implementation conformance

I re-derived the affected chapters from the implementation before reading
`arc42.md`: 02 (constraints), 04 (strategy), 05 (building blocks), 06 (runtime),
08 (cross-cutting), 09 (decisions), 11 (risks), and 12 (glossary link) are
affected; 01, 03, 07, and 10 are not.

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Receipt binds the E2E identity and scope | `docs/arc42/02-architecture-constraints.md:53` (TC-39) | Yes | `design.md:76`, `:280-310` | batch `receipt-identity`, `e2e-scope` | `proof.rs:542-586`, `check.rs:523-573` | yes |
| Blocking boundary and advisory counterpart | `02:54` (TC-45) | Yes | `design.md:165-250` | batch `boundary-resolution`, `runner-target` | `check.rs:650-687`, `lint/specification.rs:165-177` | yes |
| Change-verification strategy row | `docs/arc42/04-solution-strategy.md:23` | Yes | `design.md:20-30` | all batches | `check.rs:457-573` | yes |
| Addons strategy row | `04:30` | Yes | `design.md:14`, `:170-190` | `config/lock.rs:126` | `ise_effective_for_change` | yes |
| `e2e_boundary` building block | `docs/arc42/05-building-block-view.md:83` | Yes | `design.md:96-150` | batch `boundary-resolution` | `src/e2e_boundary/**` | yes |
| Check flow: preflight, regular scope, E2E scope, receipt | `docs/arc42/06-runtime-view.md:263-280` | Yes | `design.md:350-390` | batches `boundary-resolution` to `e2e-scope` | `check.rs:469-573`; B2 is the no-behavior exception | B2 |
| Review reads the receipt and runs no suite | `06:328-331` | Yes | `design.md:418-443` | batch `receipt-freshness` | `check.rs:692-744` | yes |
| Receipt concept includes the E2E identity | `docs/arc42/08-crosscutting-concepts.md:666-670` | Yes | `design.md:280-310` | `receipt-identity`, `receipt-freshness` | `proof.rs:542-586`, `:960-1085` | yes |
| ADR-070 amended under `change-proof/v1` | `docs/arc42/09-architecture-decisions.md:25`; `adr/ADR-070-forward-only-archive-proof.md:318-379`; `adr/README.md:71` | Yes | `design.md:165-250` | `receipt-identity` | `proof.rs:556-584`, `:1215-1245` | yes |
| R-138 runner-target limit | `docs/arc42/11-risks-and-technical-debt.md:125` | Yes | `design.md:214`, `:678` | `runner-target` | `e2e_boundary/mod.rs:369-381` refusal names the root | yes |
| R-139 import-reader depth | `11:126` | Yes | `design.md:696` | `boundary-resolution` | `imports.rs:1-9`, refusal names the target | yes |
| R-140 in-flight rerun, R-141 broad entry | `11:127-128` | Yes | `design.md:690-706` | `receipt-freshness` | `proof.rs:972`, `:1011-1023` | yes |
| Chapter 12 stays a link | `docs/arc42/12-glossary.md` | Yes | n/a | n/a | link to `docs/glossary.md`, no table | yes |

ARC42 CONFORMANCE: CLEAN

The chapter files exist and carry decision-specific content at the cited lines;
no box-drawing or pseudo-diagram characters were found in the changed
chapters. The one non-matching row is the implementation deviation B2, which is
recorded as a finding rather than silently accepted.

## Ubiquitous language conformance

The glossary already holds the terms this change uses: `End-to-end (E2E) test`
(`docs/glossary.md:72`), `E2E identity` (`:73`), `Change verification receipt`
(`:50`), `Verification scope` (`:54`), `Tested content` (`:53`), and
`Source code` (entry above the E2E group). The delta specs, design, code
symbols (`E2eBoundary`, `E2eIdentity`, `ChangeReceipt`, `ScopeOutcome`), and
served guidance use those terms; the rejected "unit" noun from the seed is
absent, and the design-review records why `package` was chosen. No unrecorded
term is introduced and no old term is left with a second definition.

`docs/domain-model.md` is unchanged (`git diff 1016f3bc..HEAD -- 
docs/domain-model.md` is empty). The change adds an evidence field and an
internal resolver; it adds no bounded context, aggregate, or domain
relationship, so the structural impact is none and the model correctly needs no
edit.

## Coverage

No measured coverage evidence exists yet for this change: `verification/`
holds only the runner result and the change receipt, and the archive receipt
will carry the Cobertura report. The classification below is from inspection;
the archive measurement will confirm or reopen it.

Per-scenario classification (all 18 executed green in the delta scope):

| # | Delta scenario | Exercised production path | Classification |
|---|---|---|---|
| 1 | declared `e2e` role resolves no readable package | `E2eBoundary::resolve`, `MissingPackage`, `MalformedManifest` | covered |
| 2 | package reaches a source package (3 rows) | `dependency_edges`, BFS, `DependencyEdge` | covered; target-table input missing (B1) |
| 3 | `e2e` path under a source root | `PackageUnderSource` | covered |
| 4 | package contains a source root | `SourceInsidePackage` | covered |
| 5 | path resolves no package and imports source | `import_edges`, `ImportEdge` | covered for `mod`/`use`; other syntax branches untested (F1) |
| 6 | runner command names no root (3 rows) | `names_root`, `RunnerNamesNoPackage` | covered |
| 7 | unbound E2E step | `run_change_e2e_scope`, result read, count wording | covered |
| 8 | skipped E2E step | same path, skipped count | covered |
| 9 | receipt binds two packages, runner, version | `E2eBoundary::identity`, `E2eIdentity`, `ScopeOutcome::passed` | covered |
| 10 | delta without `@e2e` records not applicable | delta re-read, `ScopeOutcome::not_applicable` | covered |
| 11 | identity no longer matches (5 rows) | `e2e_stale_reasons` (5 variants) | covered |
| 12 | scope no longer matches (2 rows) | delta-aware scope rules | covered |
| 13 | `ise` off keeps today's receipt | `preflight_e2e_boundary` `None`, `skip_serializing_if` | covered |
| 14-15 | two modified pinned policy scenarios | served `specs` render | covered (drift protection) |
| 16-17 | declared package policy; served `ise` guidance | served `specs`/`ise` render | covered (drift protection) |
| 18 | `ise` off allowance | `{{else}}` render | covered |

Gaps, by bucket:

- **Bucket 2 (missing test coverage):** target-specific Cargo dependency tables
  (B1), the import-reader syntax branches `from ... import`, quoted
  `require`/`import`, `self`/`super`, and the escape-above-root guard (F1), and
  the empty `e2e: []` refusal guard (F2). The first is also a missing behavior;
  the last three function correctly in manual probes but have no committed
  test.
- **Bucket 1 (dead code):** none found; clippy with `-D warnings` is clean and
  the resolver surface is consumed by lint, check, and proof.
- **Bucket 3 (legitimately hard to test):** no bucket-3 claim is used for the
  gaps above. The SCM error branches of `identity()` are fail-closed through
  `?` and are exercised by the existing SCM failure tests, not counted as gaps.

## Visual review

`design.md` declares `Interface: terminal`, no viewports, no states, and a
product with no rendered surface. The delta adds no `@e2e` scenario, so the
transcript rule collects nothing and no evidence file is expected. The terminal
baseline is the served refusal text and the receipt JSON, both asserted by the
regular delta scenarios, and the baseline holds: every refusal names its
condition and subject, no Rust identifier appears in user-facing output, and
the receipt uses the domain field names. `VISUAL-REVIEW: PASS`.

## Persona disposition

| Persona | Disposition | Evidence |
|---|---|---|
| `developer--primary-user` | confirmed; reads the refusals and the receipt | proposal `Confirmed Personas`; design Persona impact table; boundary and receipt scenarios |
| `agent--technical-role` | confirmed; reads the served policy and stale reasons | same; guidance and freshness scenarios |
| `givn-maintainer--owner` | confirmed; owns the declaration and runner command | same; runner-target and identity scenarios |

No Persona is invented. Gherkin names `the developer` and `the author`, which
are interaction roles, never Personas. A user-visible terminal surface
therefore has confirmed Personas.

## README impact

The decision is `README-IMPACT: none`. The README's only statement about this
surface is the generic gate bullet (`README.md:27`): `givn check tasks`, `givn check review`,
and `givn archive` verify receipts and sign-off before a change lands. That
bullet remains true. The README documents neither the `ise` addon nor the
change-verification receipt fields, and the detailed conditions, receipt
fields, and limits are documented in `docs/reference.md:146-160` and `:277-310`,
which this change updated. No named README section is falsified.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `An ise-effective change receipt whose E2E identity no longer matches the project is stale for review` | `An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review` | boundary |
| `Canonical policy accepts the library public API as its real interface` | `Canonical policy keeps the in-process allowance while ise is off` | variant |

Rationale: the two stale Outlines share a shape but assert different rules
(identity-value comparison versus delta-aware scope applicability), so merging
them would lose a distinct failure. The permanent library scenario and the
delta ise-off scenario are the two render branches of the same action and are
each other's negative control, so neither supersedes the other. Net delta: 17
added, 2 modified, 0 removed. The two `@givn.modified` scenarios supersede the
permanent titles they replace; no removed+added pair exists.

## Split-or-keep

No delta scenario exceeds the length limit; `givn lint --change` reports 0
`LONG` findings. No split-or-keep row is required.

## Deferrals

None. No mandatory check is deferred and no operator decision is recorded.

## Findings

### B1 (blocker): a target-specific Cargo dependency edge into `source` is accepted

`src/e2e_boundary/manifest.rs:76` reads only the top-level `dependencies`,
`dev-dependencies`, and `build-dependencies` tables. Cargo also links through
`[target.'cfg(...)'.dependencies]` (and the dev/build variants); the resolver
ignores those tables, so a resolved `e2e` package that links production source
on a platform passes the blocking boundary.

Reproduction (fixture in `/tmp/ise-exp/b`, the same shape as
`e2e_boundary_support::ise_change_ready_for_verification`, with
`source: ["src/**"]`):

```text
tests/e2e/Cargo.toml:
  [target.'cfg(unix)'.dependencies]
  product = { path = "../../src" }

$ givn check tasks --change ise-boundary
  ✓ change verification receipt: 1 scenario(s), 0 failed, 0 skipped at e7ab4e09...
  ✓ E2E verification: not_applicable
  ✓ verify passed
  EXIT=0
```

The control with the same edge under top-level `[dependencies]` is refused:

```text
error: the declared e2e boundary cannot be accepted:
  - tests/e2e: e2e package 'e2e-tests' has a dependency edge into source (root 'src/**') through package 'product'
  EXIT=1
```

Variant: with `source: ["src/**/*.rs"]` the direct edge into the `src` package
is also accepted (exit 0), because the edge check compares the package
directory against the glob, not the package tree. The proposal says the check
refuses when a package "reaches a `source` root through a direct, transitive,
or workspace-inherited dependency edge"
(`proposal.md`, Proposed Solution); `docs/reference.md:146-160` says the same
without a table qualification; `specs/verify-change/usecase.md:116` states the
rule. Neither R-138 nor R-139 discloses this input. Fix: read the
`target.<cfg>.` tables (and the tree-vs-directory source match), add a delta
example row for a target-specific edge, or record the limit with an operator
decision. The release-note claim "links production source" is false until then.

### B2 (blocker): the boundary preflight is skipped for a no-behavior-change change

`design.md:399-403` states that the boundary preflight runs before the
no-behavior-change return, "so an `ise`-effective change always needs a
readable declaration and a non-empty resolved set". The implementation returns
first: `src/cli/check.rs:459-469` evaluates
`change_verification_not_applicable` and returns at `:461` before
`preflight_e2e_boundary` at `:469`. `check_change_receipt` (`check.rs:696-707`)
has the same early return, so review accepts the change too.

Reproduction (fixture `/tmp/ise-exp/a`: an `ise: true` change lock, a proposal
with `Behavior change: none`, `.givn-skip` containing `specs`, and no
`givn/specification.yaml` at all):

```text
$ givn check tasks --change ise-none
  changed-scenario verification: not applicable (Behavior change: none; specs explicitly skipped)
  ✓ verify passed
  EXIT=0
```

By the design, this tree must be refused because the declaration is absent and
the resolved set is empty. Any `ise`-effective change can therefore skip the
new acceptance boundary by declaring no behavior change, and no delta scenario
covers the combination. Fix: move the preflight before the not-applicable
return (capturing the boundary when the change is otherwise not applicable),
or amend `design.md`, `proposal.md`, the use-case rule, and
`docs/reference.md` to state the exemption and re-run design-review; add a
scenario for the ise-effective no-behavior change either way.

### F0 (task record): 7 Closeout boxes unchecked

Recorded in "Tasks preflight". The author must run and record the whole-change
verification, `givn check tasks`, and `givn check review`, and tick the boxes,
before archive.

### F1 (bucket 2): untested import-reader branches

`src/e2e_boundary/imports.rs` recognizes five reference forms. The delta
scenario and the four module tests cover `mod` and `use`; these branches have
no committed test: `from ... import` (`imports.rs:62-68`), quoted
`require`/`import` (`:70-74`), `self`/`super` (`:141-155`), and the
escape-above-root guard (`manifest.rs:176-186`). Manual probes refuse the first
three:

```text
from src.helpers import helper          -> error: ... imports the source path 'src' (root 'src/**')
const x = require("../../src/helpers")  -> error: ... imports the source path 'src/helpers'
import { helper } from "../../src/..."  -> error: ... imports the source path 'src/helpers'
```

Add unit tests for the three syntax forms and the admitted `#[path = ...] mod`
form (`imports.rs` already has a test module; the admitted case is the R-139
limit probe I ran: `#[path] mod helpers` is admitted).

### F2 (bucket 2): the empty-role refusal guard has no test

`src/cli/check.rs:670-675` refuses an `ise`-effective change whose declaration
has `e2e: []` with `the 'e2e' role lists no path; the change check requires a
resolved E2E package`. Probe: it refuses (exit 1) with that exact text. No
committed scenario or unit test exercises the empty role list (the delta's
`matches no path` row uses a non-empty entry that matches nothing). Add a row
or a unit test.

### F3 (non-blocking, route to the next change): a shared permanent step was loosened

`tests/steps/archive_proof_steps.rs:742` changed the shared
`stderr names "..."` step from `stderr.contains(needle)` to "the needle's words
appear in order on one line, case-insensitively". 25 permanent assertions use
that step. The loosening lets a message with an inserted or separated word
satisfy a needle (for example a negated sentence), and it was introduced to
serve the delta's `E2E package digest changed` reason, whose actual line
inserts the package path. The permanent readings I sampled still pass and
their needles are mostly single paths, so this is not a functional blocker,
but a permanent assertion was weakened without a design decision. Recommended:
restore contiguous matching in the shared step and bind a delta-specific step
for the identity reason, or narrow the matcher to allow only the package-name
insertion.

### Minor observations (no action requested)

- `RunnerResult::read_counts` (`src/proof.rs:511-524`) changed its shared
  failure wording from `declares N failed scenario(s)` to `reports N failed
  test(s)`. This alters the archive refusal text, is not named in `design.md`
  or `docs/reference.md`, and no permanent test pins the old wording. The
  delta's count assertions need the new wording. Record it in the next design
  revision or the reference.
- `ScopeOutcome::not_applicable()` serializes the zero count fields
  (`total`, `passed`, `failed`, `skipped`, `seconds`) as well as `status`
  (`proof.rs:408-413`, `:381-394`). The tasks' wording `{ "status":
  "not_applicable" }` is a shorthand, and the shape is pre-existing, so no
  action is needed.
- `E2eBoundary::identity` iterates `backend.content_paths()` without an
  explicit sort (`mod.rs:246-262`). Git and Fossil both return sorted entries,
  so the digest is stable today; an explicit sort would make it independent of
  the backend.
- `names_root` (`mod.rs:369-381`) accepts a token containing `<root>/`
  anywhere, so `mytests/e2e/` names `tests/e2e`. R-138 discloses the class
  (naming is not execution); this is a concrete instance to keep in mind when a
  target marker replaces the textual rule.

## Verification

`givn check review --change enforce-ise-e2e-boundary` ran before and after the
review commit. It passes the lint, overlap-disposition, capability-ownership,
scenario-commit, and change-receipt checks; the integrity hook fails only on
the recorded verdict, as the blockers require. Before the commit the receipt
line reads `change verification receipt passed (28 scenario(s) at
2d7b1362...)`; after it, the receipt is reused across the documentation-only
review revision:

```text
  ✓ scenario commits passed (18 scenario(s))
  ✓ change verification receipt reused across documentation-only revision ... (tested at 2d7b1362..., 28 scenario(s))
  ✓ scenario commits passed (18 scenario(s))
error: review checks failed:
  integrity checks failed:
  review.md actual verdict is FAIL
```

Exit code 1. All other review checks pass; no fabrication-audit, component,
contract, strict-mode, coverage-classification, overlap, split, persona,
visual, or README check failed.

Delta scope: 28 scenarios (28 passed), 149 steps (149 passed). Permanent
`check-specification-health`: 57 scenarios (57 passed), 345 steps (345
passed). Clippy clean. Run gate: `cargo run -- instructions` exit 0.

## Sign-off checklist

- Fabrication audit: clean for the steps' production dependency; 0 empty step
  bodies; the served-policy scenarios are disclosed drift protection.
- Every checked task has a verified commit touching production code (the review
  gate reports 18 scenarios with revisions).
- Every promised component exists: `E2eBoundary`, `ChangeReceipt`, the check
  integration, the lock helper, and the lint consumer.
- Every consumer uses only its declared contract; the dependency is one-way.
- Strict-mode proof present and passing.
- Scenario execution evidence: the change receipt records 28 change-scope
  scenarios; the E2E scope is `not_applicable` because the delta has no `@e2e`
  scenario. The archive receipt will carry the measured coverage.
- Coverage classification uses inspection (no measured report yet); every gap
  is in bucket 1, 2, or 3.
- Dead code deleted (none found); missing tests recorded (F1, F2); no
  hard-to-test bucket-3 claim used for the gaps.
- No redundant unit test was added for behavior already covered by a scenario.
- No `@wip` tags remain in the delta and no implementation-layer detail is in
  the spec.
- Canonical E2E policy: the delta adds no `@e2e` scenario and no normalized
  action; every permanent action has one `@e2e` scenario.
- Browser/HTTP shortcuts: not applicable (CLI).
- The run gate starts the declared CLI cleanly.
- The exact `verify.e2e_command` files were identified and read; no parallel
  implementation exists.
- The E2E scope is selected explicitly; the receipt records the regular scope
  and the delta-aware E2E outcome.
- Implementation matches design except B2 (order) and the target-table gap B1;
  both are recorded as blockers, not silently kept.
- Interaction coverage verified: 22 inventory entries, 22 matrix rows, 22
  permanent `@e2e` titles.
- No finding is excused with an invented fourth bucket.

Two blockers (B1, B2) and three required coverage/test records (F0 to F2)
remain. The verdict is FAIL until they are resolved and this review is
re-run.

## Finding closure

Each finding's fix, its test, and the revision that carries it. The verdict
line is the reviewer's and is unchanged by this section.

| Finding | Fix | Test | Commit |
|---|---|---|---|
| B1 | `manifest::dependency_tables` reads the three top-level tables and every `[target.<cfg>.*]` table; `E2eBoundary::resolve` compares a dependency target through `source_root_reached_by`, which refuses a directory that is a `source:` root, lies inside the root's literal anchor (`Glob::anchor`), or contains a path the pattern names. `docs/reference.md` states both rules. | `a_target_specific_dependency_table_is_read`, `a_target_specific_dependency_into_source_is_refused`, `a_glob_shaped_source_root_reaches_the_directory_it_anchors_in`, `a_dependency_that_contains_a_source_root_is_refused`, `a_dependency_outside_every_source_root_is_admitted`, `the_anchor_is_the_literal_directory_a_pattern_starts_in`; delta Examples row `target-specific` in `An ise-effective change whose e2e package reaches a source package fails the tasks check`. Reverting both production reads fails the four resolver tests (30 -> 26 passed) and the row's fixture is refused only with the target-table read. | `a072aa990667e86a1af8b9932e486afffc083c7c`; row and doc sentence in `3b2c293d81b97048488ceeb71d12c702f2dd01df` |
| B2 | `run_change_verification` calls `preflight_e2e_boundary` before `change_verification_not_applicable`; the not-applicable branch of `check_change_receipt` calls it too, because that branch writes no receipt to age. | Delta scenario `An ise-effective change with no behavior change still needs a readable e2e declaration`, driving both `givn check tasks` and `givn check review`. Reverting the order fails the first Then with `expected failure, got exit Some(0)` and the not-applicable line. | `3b2c293d81b97048488ceeb71d12c702f2dd01df` |
| F0 | Every Closeout command run and recorded with its output in `tasks.md`; the receipt is refreshed after the code fixes. | The recorded runs in `tasks.md` `## Closeout`. | evidence recorded at `7788a0610ef29ffa5709dcaa7750222f63ba25f7`; closeout runs and the refreshed receipt at `384f0e617e3c55bd19eef155c6e8d714a24b0f54` |
| F1 | No production change: the reader already handled the branches. | `a_python_from_import_names_its_module_and_is_refused`, `a_quoted_require_or_import_path_is_refused`, `a_self_import_resolves_from_the_file_directory`, `a_super_import_walks_up_from_the_file_directory`, `an_attributed_module_declaration_is_admitted` (the admitted `#[path]` limit, R-139), and `a_path_dependency_may_not_escape_the_project_root` for the escape guard. | `a072aa990667e86a1af8b9932e486afffc083c7c` |
| F2 | The first refusal Outline gained the `lists no path` row; the Then asserts the empty-resolution sentence when the declaration has no entry. | `An ise-effective change whose declared e2e role resolves no readable package fails the tasks check`, row `lists no path` (`e2e: []`), asserting `the 'e2e' role lists no path`. | `3b2c293d81b97048488ceeb71d12c702f2dd01df` |
| F3 | `stderr names "..."` is a contiguous substring check again; the delta binds its own `stderr reports the stale reason "..."` step and the two Outlines use it. | The permanent `archive-change` features: 6 features, 111 scenarios (111 passed), 767 steps (767 passed). | restore `0415ffa3af450013eb8eb0b1b4968ef3d18ce0f2`; delta step and phrase `3b2c293d81b97048488ceeb71d12c702f2dd01df` |
| O1 (minor, from the review's observations) | Not actioned, as the review requested no action. The delta's two count scenarios pin `reports N failed test(s)` / `reports N skipped test(s)`; `docs/reference.md` still does not name that wording, so it stays an open documentation nit for the next change. | The two count scenarios | n/a |
| O2, O3, O4 (minor, from the review's observations) | Not actioned: no behavior claim in the reviewed reports depends on the unsorted `content_paths` order, the `not_applicable` payload shape, or the `names_root` token rule. | n/a | n/a |

The fixes were verified in this order: unit tests, the single delta scenarios,
the whole delta scope (31 scenarios, 31 passed, 166 steps), the permanent
`archive-change` run, then the Closeout suite recorded in `tasks.md`.

REVIEW: FAIL
