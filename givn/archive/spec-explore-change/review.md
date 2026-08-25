# Review: spec-explore-change

## Scope

This review covers PR 2: change-aware advisory exploration for complete file
and standard-input scenario queries. It adds active `Added` and `Modified`
delta candidates, excludes `Removed` scenarios and self-identities, labels
candidate ownership, preserves a permanent-only persisted index, and validates
the requested change before refreshing that index.

It does not add semantic classification, remediation verification, archive
projection, BGE/NLI evidence, coverage gating, review reports, or automatic
feature-file edits.

## Fabrication Audit

### E2E tag integrity

The delta feature contains exactly two `@e2e` scenarios:

- `Change-aware file exploration finds permanent and active change candidates`
- `Change-aware JSON exploration labels candidate sources`

Both retain their `@e2e` tags in the final feature. No tag was removed. The
remaining 14 scenarios are regular scenarios, including the change-aware stdin
parity case and all validation/error cases.

### Step-body audit

Files scanned:

- `tests/steps/spec_explore_change_steps.rs`
- `tests/steps/spec_explore_change_e2e_steps.rs`

Result: `0` empty bodies, `0` `unimplemented!()` bodies, `0` `todo!()` bodies,
and no bare `return`/`pass` bodies. Assertions use captured CLI output,
filesystem snapshots, JSON values, or real process status. The final runner
uses `.fail_on_skipped()` in `tests/features_runner.rs`.

The setup task records the strict RED proof: the targeted runner exited 101
with `1 scenario (1 failed)`, `1 step (1 failed)`, and a matched step panic
with `not implemented`.

### Commit and implementation audit

All 74 task checkboxes are complete. Every scenario task records a commit hash.
The scenario commits are:

| Scenario group | Commit |
|---|---|
| Removed delta filtering | `0d36bd7` |
| Modified source identity | `eb3831b` |
| Permanent-index persistence boundary | `df1b43f` |
| Added source identity | `af62538` |
| Modified projected-target identity | `5de7ff0` |
| Relative/absolute identity normalization | `2fd7acf` |
| Change-aware stdin parity | `58043f3` |
| Permanent text metadata | `7386892` |
| Permanent JSON metadata | `ec9c082` |
| Unsafe change id | `58a56c2` |
| Missing change | `bcff09f` |
| Missing specs directory | `fa4e4a5` |
| Malformed delta | `7e007bd` |
| File exploration E2E | `2cc2d73` |
| JSON exploration E2E | `b70e495` |
| Rule-nested delta collection | `df91758` |

The production implementation is present in `src/main.rs`, `src/cli/spec.rs`,
and `src/spec/retrieval.rs`. The first capability commit introduced the
change-aware CLI/retrieval path; subsequent scenario commits that only extend
fixtures and assertions are genuine reuse cases against that already-built
production path, not empty stubs or spec-only commits. The E2E commits drive
the compiled binary and the Rule-nested commit exercises the existing delta
collector's `Rule:` branch through the CLI.

All design-promised files exist:

- `src/main.rs`
- `src/cli/spec.rs`
- `src/spec/retrieval.rs`
- `tests/steps/mod.rs`
- `tests/steps/spec_explore_change_steps.rs`
- `tests/steps/spec_explore_change_e2e_steps.rs`
- `givn/changes/spec-explore-change/specs/spec-explore-change/spec-explore-change.feature`

### E2E fidelity and command isolation

The real interface is the CLI. Both E2E step paths use
`common::run_givn_with_envs`, which starts the compiled `givn` process in an
isolated temporary project. Their primary assertions are on captured terminal
stdout and exit status. The JSON scenario parses the returned stdout document;
it does not call retrieval functions directly.

The configured commands were read literally from `givn/commands.yaml`:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

The full runner passed `223 scenarios (223 passed)`, `1254 steps`; the E2E
runner passed `34 scenarios (34 passed)`, `203 steps`. The E2E command is a
strictly smaller `@e2e` scope and is not identical to the full command.

The tree contains only the two intended capability step files. No second or
stronger E2E implementation exists elsewhere. The local run command is
`./run-tests.sh`; this CLI-only capability has no server, database, external
service, browser, or digital twin requirement.

### Interaction coverage cross-reference

| Inventory entry | Matrix row | E2E scenario | Driver and primary assertion | Result |
|---|---|---|---|---|
| invoke `givn spec explore --change <id> --file <draft.feature>` and read source-labeled text results | same text entry in `design.md` | `Change-aware file exploration finds permanent and active change candidates` | `std::process::Command` via `run_givn_with_envs`, asserting CLI exit status and ranked stdout titles, scopes, change id, and source paths | PASS |
| invoke `givn spec explore --change <id> --file <draft.feature> --format json` and consume source metadata | same JSON entry in `design.md` | `Change-aware JSON exploration labels candidate sources` | `std::process::Command` via `run_givn_with_envs`, parsing one CLI stdout JSON document and asserting result metadata | PASS |

Every inventory entry has exactly one matching matrix row and one matching
`@e2e` scenario. Standard-input support is intentionally regular coverage and
has no extra inventory row.

## Arc42 Implementation Conformance

Arc42 is enabled. The change-level marker ends with `STATUS: DONE`. The durable
chapters and change marker agree with the completed implementation:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| User-facing current-change exploration goal | `docs/arc42/01-introduction-and-goals.md` | Chapter 1 affected; FR-27 and advisory goal updated | Optional `--change` CLI boundary | Change-aware file/JSON scenarios | `src/main.rs`, `src/cli/spec.rs` | Yes |
| Optional CLI context and scope | `docs/arc42/03-context-and-scope.md` | Chapter 3 affected; current-change source-labeled output | CLI-only boundary and out-of-scope review layers | E2E text and JSON tasks | `spec explore --change` dispatch | Yes |
| Request-local retrieval strategy | `docs/arc42/04-solution-strategy.md` | Chapter 4 affected; transient delta vectors | Candidate model and ranking sections | Persistence and active-delta tasks | `ExploreCandidate` and transient embedding path | Yes |
| Retrieval building blocks | `docs/arc42/05-building-block-view.md` | Chapter 5 affected; collection and path helper | Architecture impact table | Setup and scenario step files | `src/spec/retrieval.rs`, two step modules | Yes |
| Validation, collection, filtering, ranking flow | `docs/arc42/06-runtime-view.md` | Chapter 6 affected; request-local sequence | Mermaid runtime flow | Identity, error, and E2E tasks | `explore_enabled`, delta collection, renderer | Yes |
| Path identity and additive output metadata | `docs/arc42/08-crosscutting-concepts.md` | Chapter 8 affected; canonical path and output contract | Output and identity sections | Path, text, JSON, and parity tasks | `normalize_project_file_identity`, result fields | Yes |
| Request-local corpus and path decisions | `docs/arc42/09-architecture-decisions.md` | ADR-044 and ADR-045 added | Same decisions and alternatives | Scenario evidence and commit hashes | `src/spec/retrieval.rs` implementation | Yes |
| Quality scenarios | `docs/arc42/10-quality-requirements.md` | Q-46 through Q-51 added | Acceptance scenarios and runner commands | 16 regular plus 2 E2E scenarios | Change runner `16/106`, E2E `2/16` | Yes |
| Persistence and identity risks | `docs/arc42/11-risks-and-technical-debt.md` | R-30, R-32, R-33 documented | Pre-refresh validation and source contract | Index snapshots and metadata assertions | Byte-for-byte index test and rejection tests | Yes |
| Current-change vocabulary | `docs/arc42/12-glossary.md` | Current-change and identity terms added | Domain terms used consistently | Scenario language uses the terms | CLI output and feature titles | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Verification

### Executable specifications

| Command | Result |
|---|---|
| `givn lint --change spec-explore-change` | PASS: `1 file(s) checked - clean` |
| Change-specific full runner | PASS: `16 scenarios, 106 steps` |
| Change-specific E2E runner | PASS: `2 scenarios, 16 steps` |
| `./run-tests.sh` | PASS: `223 scenarios, 1254 steps` |
| `./run-tests.sh --e2e` | PASS: `34 scenarios, 203 steps` |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --features spec-retrieval` | PASS |
| `cargo check --no-default-features --locked` | PASS |
| `cargo test --features spec-retrieval --lib` | PASS: `144 tests` |
| `cargo test --no-default-features --lib` | PASS: `142 tests` |
| `cargo clippy --workspace --all-targets --features spec-retrieval -- -D warnings` | PASS |

### Coverage measurement

Coverage was measured with `./measure-coverage.sh`, which runs the real
`features_runner` binary through `cargo llvm-cov` separately for non-E2E and
E2E scopes. The two reports were merged with `./merge-coverages.sh` into a
fresh Cobertura report.

Raw merged report values:

```text
lines-covered=2308
lines-valid=10618
line-rate=0.21736673573177623 (21.7367%)
branches-covered=0
branches-valid=0
branch-rate=0.0
```

Separate raw reports were non-E2E `1185/5309` lines (`0.22320587681295911`)
and E2E `1123/5309` lines (`0.21152759465059334`). The configured LLVM-Cov
reports emitted zero valid branch counters (`0/0`), so no branch denominator
or branch regions were available to classify; this is recorded as a tool
measurement boundary, not as a claim of 100% branch coverage.

The changed production lines are covered:

| File | Changed lines | Changed lines with zero hits |
|---|---:|---:|
| `src/main.rs` | 10 | 0 |
| `src/cli/spec.rs` | 13 | 0 |
| `src/spec/retrieval.rs` | 187 | 0 |

The Rule-nested scenario was added after the first measurement specifically to
cover the previously unexecuted nested-delta collector branch. The final
measurement has zero uncovered changed production lines. Uncovered lines in
unchanged retrieval/review and unrelated workflow paths are outside this
change's implementation surface; no new PR-2 production gap remains.

### Mutation and feature-file integrity

Exploration is non-mutating. The persistence scenario compares the generated
permanent index byte-for-byte before and after a change-aware request, and the
JSON E2E scenario asserts that `index.entries` remains the permanent count
while a current-change candidate is present. Existing permanent-feature
snapshot coverage remains green. The branch diff contains no modifications
under `givn/specs/`.

## Review Outcome

Fabrication audit: PASS.

All planned PR-2 behaviors are executable, the real CLI boundary is exercised,
the permanent index remains permanent-only, all changed production lines are
covered, and Arc42 documentation conforms to the implementation.

REVIEW: PASS
