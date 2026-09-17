# Design: finding-severity-routing-signal

## Technical Approach

Two Rust surfaces and two guidance files. Lint gains a severity
classification and rendering; route gains confidence and ownership-weighted
scoring. No new dependency, gate, or artifact.

## Severity Classification

`src/cli/lint.rs` gains:

```rust
enum Severity { Advisory, Finding, Blocking }
fn severity_for(kind: &str) -> Severity
```

One data-driven table:

| Severity | Finding kinds |
|---|---|
| `blocking` | `parse_error`, `duplicate_title`, `empty_feature`, `usecase_invalid`, `fragment_invalid`, `dangling_capability_ref`, `ambiguous_relationship_target`, `dangling_use_case_ref`, `use_case_cycle`, `extend_fragment`, `interaction_mismatch` |
| `finding` | `wip`, `e2e_intent` |
| `advisory` | `shape_match`, `subset`, `long_scenario`, `seed_status`, `persona_ref`, `singleton_usecase`, `usecase_interaction_invalid` |

The default arm is `advisory`, and a unit test enumerates every kind the
scanner emits so a new kind cannot fall through silently. The
`interaction_mismatch` prose label stays `[WARN ]` (pre-existing); only the
severity word is added.

- The exit-code contract is preserved: the existing flags (`has_parse_error`,
  `has_duplicate`, `has_empty_feature`, `has_use_case_error`, `has_wip`) keep
  driving the exit code exactly as today; the severity classification is
  descriptive and is the single registry the other changes' kinds register
  in. Advisory findings still never affect the exit code.
- Prose rendering keeps the existing short label and appends the severity:
  `[ERROR] blocking  <location>  <message>`, `[SHAPE] advisory  …`,
  `[WIP  ] finding  …`. Existing assertions (`[ERROR]`, `[WIP  ]`, `[SEED ]`,
  `[PERS ]`) keep matching.
- JSON output gains a `severity` field per finding, in both the main branch
  and the empty-scope branch (which renders seed and persona advisories). The
  findings vector is mapped to `serde_json::Value` objects with the original
  fields plus `severity`, so the overlap `Finding` struct keeps its shape.
- The e2e-intent prose line (printed directly, not through `render_prose`)
  also carries its severity word.

## Route Confidence

`src/spec/route.rs`:

```rust
enum RouteConfidence { Signal, NoSignal }
struct RouteReport { candidates, suggested_usecase, confidence, note }
```

Rules, in order:

1. No candidates → `NoSignal` ("no command, token, or lexical overlap").
2. The top two candidate scores within `0.01` → `NoSignal` ("the top
   candidates tie at X.XX").
3. Top score `>= 3.0` (the extend bar) → `Signal`.
4. Otherwise the `NEW` path: a unique top use-case affinity of at least `2.0`
   → `Signal`; a tied or zero top affinity → `NoSignal`.

`print_report`:

- Signal with an extend candidate: `recommendation: EXTEND <cap> (use case
  '<uc>', score X.XX) — confidence: signal`.
- Signal on the NEW path: `recommendation: NEW in <usecase> — confidence:
  signal`.
- No signal: `recommendation: no signal — <reason>`; the ranked candidate
  list is not printed. The trailing "this ranking is advisory only" sentence
  is removed: confidence is now explicit.

The permanent scenario "A proposal with no matching surface recommends NEW"
is modified: an empty-overlap proposal now reports no signal instead of a
NEW recommendation. The decision record stays mandatory; the proposal
guidance says how to record it without a recommendation.

## Ownership-Weighted Scoring

`CapabilityInfo` gains the owning use case's `Goal` + `Rules` text and its
`Interactions` consumer actions (both lowercased). Scoring additions:

- Proposal word found in the owning use case's Goal/Rules: `+2.0` per word,
  capped at three words.
- Proposal word found in the capability's interaction consumer actions:
  `+1.0` per word, capped at three words.
- The capability name or its owning use-case id appears as a proposal word or
  backticked token: `+2.0`. This term is explicit; the capability's scenario
  text does not include its own name.
- Ownership loading is tolerant: a missing or unparsable `usecase.md`
  contributes no ownership evidence and never errors, so the existing route
  unit tests with corpus-only fixtures keep passing.
- Scatter penalty: a candidate with no Goal/Rules ownership hit and no
  command-surface overlap has its score halved, with the evidence line
  "no goal ownership for this proposal". A goal-aligned candidate therefore
  ranks above a lexically similar but goal-mismatched one.
- The capability name plus its owning use case are already part of the
  structural-token and command evidence; the new weights sit above lexical
  overlap, which stays capped as today.

## Proposal Guidance

`assets/instructions/proposal.md`, `assets/commands/givn-propose.md`, and
`assets/templates/proposal.md` all state: when route reports signal, record
route's recommendation and the decision as today; when route reports no
signal, omit the recommendation column and record the decision with its
rationale directly. The recorded decision stays mandatory either way. The
existing addon conditional mechanism cannot express route confidence, so this
is guidance text, not a template conditional, and the template edit is
asserted by an eject-based step.

## Spec Coverage

| Delta scenario | Asserted phrases (implementation detail) |
|---|---|
| A blocking finding renders its severity | exit 1, stderr "[ERROR] blocking" |
| An advisory finding renders its severity and keeps the exit code | exit 0, stderr "[SHAPE] advisory" |
| The JSON output exposes the finding severity | exit 2, stdout `"severity": "finding"` |
| A tied route reports no signal | exit 0, stdout "no signal", no "ranked candidates" |
| A goal-aligned capability ranks above a lexically similar candidate | exit 0, stdout "recommendation: EXTEND coverage-tool" |
| A proposal with no matching surface recommends NEW (modified) | exit 0, stdout "recommendation: no signal" |
| The propose guidance carries the no-signal routing rule | "no signal", "omit the recommendation", "decision", and the ejected proposal template contains "no signal" |

Fixture score arithmetic, pinned before implementation:

- Tied corpus: two capabilities in use case `gates`, each invoking `givn lint`
  once, with the use-case goal sharing no proposal word. Each scores
  `3.0` (command) × `0.5` (scatter penalty) = `1.5`; the top-two tie is
  inside `0.01` → no signal. The affinity fallback stays at zero.
- Goal-aligned corpus: `alignment/coverage-tool` with goal "Validate
  coverage reports." and a scenario mentioning "a coverage report exists"
  scores `4.0` (two ownership words × 2.0) + `0.4` (two lexical words) =
  `4.4`; `gates/lint-tool` with goal "Check things." and scenario text
  mentioning coverage reports scores `0.4` × `0.5` = `0.2`. The aligned
  candidate wins above the `3.0` extend bar with `confidence: signal`.
- A route unit test pins the aligned ranking before the Gherkin loop runs.

## Use-Case Traceability

| Contract element | Source | Technical decision | Evidence |
|---|---|---|---|
| Lint is a static check with a stable exit-code contract | `givn/specs/verify-change/usecase.md` | Severity is descriptive; the flags drive the exit code | The three lint scenarios |
| Route narrows candidates deterministically | `givn/specs/fragments/spec-explore-change.feature` | Confidence, no-signal, and ownership weighting | The route scenarios |
| The recorded routing decision remains the declaration | `givn/specs/author-change/usecase.md` | Guidance keeps the decision mandatory | The propose-guidance scenario |

Persona impact: `agent--technical-role` (CLI output, JSON) and
`givn-maintainer--owner` (tooling). No Persona is invented or promoted;
Gherkin actors stay the user and the agent.

## Step Definitions

| Capability | Step definition file | Interface |
|---|---|---|
| lint-command | `tests/steps/mod.rs` generic steps | Real `givn lint` subprocess; no new steps |
| spec-explore-change | `tests/steps/spec_discovery_deterministic_steps.rs` | Real `givn spec route` subprocess; adds two corpus Givens |
| instruction-integrity | `tests/steps/instruction_integrity_steps.rs` | Direct asset reads; adds the routing-rule When and Thens |

## Runner and Strict Mode

- Runner: Rust `cucumber-rs` through `tests/features_runner.rs`.
- Strict mode: `.fail_on_skipped()`; proven by the existing suite.
- Not-implemented stub: `unimplemented!()`.
- Single-scenario command:

```text
GIVN_FEATURES=givn/changes/finding-severity-routing-signal/specs cargo test --test features_runner -- --name '^<scenario title>$'
```

## E2E Infrastructure and Local Runnability

- Real interface: the CLI (lint, route) driven as a real subprocess.
- Local run command: `./run-tests.sh`; e2e scope: `./run-tests.sh --e2e`.
- Digital twins: none required; no external dependency.
- Anticipated interface obstacle: none.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| lint-command: run the static lint check | A duplicate title across two permanent spec files is a hard error (existing, sibling `spec-overlap-gates`) | CLI | Real `givn lint` subprocess |
| spec-explore-change: route a proposal (existing) | Routing a proposal prints ranked candidates with evidence (existing) | CLI | Real `givn spec route --change` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | An agent requests machine-readable artifact guidance (existing) | CLI | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end,
and the new scenarios are output variants. No browser UI exists.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | cargo coverage build of the givn binary/test runner | separate non-E2E and E2E Cobertura reports | `./merge-coverages.sh` | `givn::cli::lint::severity_for` and `givn::spec::route::route` |

## Version Freshness

No version-bearing technology is introduced. The Rust toolchain, `cucumber-rs`
0.23.0 (already pinned in `Cargo.lock`), and the existing scripts are
unchanged.

## Architecture Impact

The lint output gains a severity word and a JSON field; the route report
gains confidence and ownership-weighted scoring. The exit-code contract, the
recorded-decision gate, and the artifact graph are unchanged. `givn spec
route` has no JSON flag; its only consumer is the printed report, so no JSON
compatibility claim is made.

## Risks

- Threshold tuning is delicate. The confidence rules use the existing extend
  bar (3.0) and a conservative affinity floor (2.0); the tie epsilon is
  0.01, so the six-way tie case reports no signal while a clear winner still
  reports.
- The modified permanent route scenario changes observable output for the
  empty-overlap case; the decision requirement is unaffected.
- The severity mapping is one table, so later finding kinds register in one
  place without touching rendering.
- Ownership-credit leakage: the goal-mismatched fixture must share no
  proposal word with its own Goal/Rules, and the goal-aligned fixture must
  reach the extend bar (3.0); the pinned fixture text and score arithmetic in
  tasks.md make both deterministic, and a route unit test pins the ranking
  before the acceptance loop.
