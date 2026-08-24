# Review: search-evidence

## Fabrication Audit

This change adds an opt-in `givn spec search --evidence` surface that appends
BGE and NLI cross-encoder verdicts to the existing E5 search ranking without
changing the flag-off path.

**@e2e tag integrity:** The delta feature file contains one `@e2e` scenario
(`Search reports BGE and NLI evidence through the givn CLI`). No planned
`@e2e` tag was removed; the tag is present in the committed delta and was
exercised by `verify.e2e_command`.

**Step definition scan** — the two capability files were checked for empty
bodies, `unimplemented!()`, `todo!()`, bare returns, and no-op bodies:

```text
tests/steps/search_evidence_steps.rs: 0 matches
tests/steps/search_evidence_e2e_steps.rs: 0 matches
0 empty or unimplemented step bodies found across 2 files.
```

All steps drive the real compiled CLI subprocess and assert on its exit
status and stdout; the evidence fixtures are directional-score overrides fed
through environment variables.

**Commit cross-check**

| Scenario | Commit hash | Authoritative source touched? |
|---|---|---|
| Register the two step modules + strict-mode proof | `824940f` | Yes — step modules, `tests/steps/mod.rs` |
| Search without the evidence flag prints no evidence lines | `2377f08` | Yes — step glue (`local NLI evidence is enabled`) |
| Evidence search reports a BGE verdict per candidate | `8f7b961` | Yes — `src/main.rs`, `src/cli/spec.rs`, `src/spec/retrieval.rs` |
| Evidence search reports an NLI verdict per candidate | `51cd570` | Yes — `src/spec/retrieval.rs` |
| Absent BGE evidence is reported without a verdict | `73c39eb` | Yes — `src/spec/retrieval.rs` |
| Evidence search without cross-encoders reports unavailable evidence | `a4e5057` | Yes — `src/spec/retrieval.rs` |
| Candidates beyond the pair budget stay unscored | `e0324d8` | Yes — `src/spec/retrieval.rs` |
| Over-limit search pair receives BGE token-cap evidence | `35cb301` | Yes — `src/spec/retrieval.rs`, search evidence steps |
| Over-limit search pair receives NLI token-cap evidence | `2c27603` | Yes — `src/spec/retrieval.rs`, search evidence steps |
| E2E: end-to-end evidence search through the real CLI | `853341b` | Yes — `tests/steps/search_evidence_e2e_steps.rs`, arc42 durable chapters |

Every checked task maps to a real commit whose diff touches production source
or the executable test glue; no spec-only or stub-only commit was used as a
task completion.

**Promised-architecture cross-check**

| Component or source from `design.md` | Exists in tree? | Path |
|---|---|---|
| `--evidence` CLI flag on `SpecCommands::Search` | Yes | `src/main.rs` |
| `cli::spec::run_search(query, evidence)` pass-through | Yes | `src/cli/spec.rs` |
| `spec::retrieval::search(root, query, evidence)` evidence branch | Yes | `src/spec/retrieval.rs` |
| Reuse `bge_explore_run`/`nli_explore_run` | Yes | `src/spec/retrieval.rs` |
| Reuse `max_rerank_pairs()` (default 2048) | Yes | `src/spec/retrieval.rs` |
| Regular step file | Yes | `tests/steps/search_evidence_steps.rs` |
| E2E step file | Yes | `tests/steps/search_evidence_e2e_steps.rs` |
| Registered in `tests/steps/mod.rs` | Yes | `tests/steps/mod.rs` |

**Strict-mode proof:** The setup proof in `tasks.md` ran the Cucumber runner
against an `unimplemented!()` Given and recorded `1 scenario (1 failed)` with a
non-zero exit (`exit status: 101`). `GivnWorld::cucumber().fail_on_skipped()`
is active in `tests/features_runner.rs`.

**Downgraded E2E scenario scan**

| @e2e Scenario | Real-interface assertion present? | Browser driver required? | Downgraded? |
|---|---|---|---|
| Search reports BGE and NLI evidence through the givn CLI | Yes — CLI exit status, `rank 1`, `BGE_RECOMMENDED`, `NLI_RECOMMENDED` in stdout | No — CLI | No |

**E2E scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---:|---|---|
| Evidence search | 1 | Yes — the single normalized action `givn spec search --evidence "<query>"` | No |

**Duplicate-implementation check**

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? | Stronger one used instead? |
|---|---|---|---|
| Evidence search | `./run-tests.sh --e2e` invokes `tests/features_runner.rs` with `tests/steps/search_evidence_e2e_steps.rs` registered | No parallel evidence-search E2E implementation; existing E2E modules cover other capabilities | No alternative existed |

**Local environment check:** `./run-tests.sh` is the project-local command.
This capability starts no application server, database, queue, browser, or
third-party service; the temporary project is created per scenario inside the
fixture root and the CLI runs offline against stub model directories.

**E2E isolation check**

| `verify.command` value | `verify.e2e_command` value | Identical? | Full suite count | E2E-only count | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 9 non-E2E + 1 E2E change scenarios, 10 passed | 1 E2E scenario, 1 passed | Yes |

**Design.md conformance check**

| Decision | `design.md` says | Actually built | Matches? |
|---|---|---|---|
| CLI surface | `--evidence` flag on `Search` | `src/main.rs` `Search { query, #[arg(long)] evidence }` | Yes |
| Search flow | Keep `print_search_results` byte-for-byte when `evidence` false; append evidence block | `search_enabled` routes only when `evidence == true` | Yes |
| Evidence generation | Top `RERANKER_TOP_K` (5) candidates; `ExploreQuery` wrap; `bge_explore_run` / `nli_explore_run` | Implemented in `search_enabled` | Yes |
| Step files | Two new capability files, registered in `tests/steps/mod.rs` | `search_evidence_steps.rs`, `search_evidence_e2e_steps.rs`, registered | Yes |
| E2E glue | Subprocess of compiled `givn spec search --evidence "<query>"` | `common::run_givn_with_envs` spawning `CARGO_BIN_EXE_givn` | Yes |
| Reused steps | `a givn project with permanent scenarios for retrieval`, `the local spec index has been built`, `local BGE evidence is enabled`, `the second-stage pair budget is 2` | All reused | Yes |
| Runner strictness | Cucumber `fail_on_skipped()` | `tests/features_runner.rs` | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row? | Matching @e2e scenario? | Promised driving mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| search permanent scenarios with optional cross-encoder evidence through the givn CLI | Yes | Search reports BGE and NLI evidence through the givn CLI | `common::run_givn_with_env` subprocess of the real `givn` binary | `run_givn_with_envs` subprocess of `CARGO_BIN_EXE_givn` | Yes |

## Arc42 Implementation Conformance

The Arc42 addon is enabled. The change-level assessment names chapters 3, 6,
8, 11, 12 plus README and the ADR-052/ADR-058 surface enumerations. The
implementation evidence is the updated durable chapters in commit `853341b`:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Context and Scope | `03-context-and-scope.md` | Extends the search surface | CLI flag and evidence block | E2E search scenario | Flag and rendered evidence block | Yes |
| Runtime flow | `06-runtime-view.md` | New search → evidence flow | Evidence block after ranking | BGE/NLI scenario runs | `search_enabled` evidence branch | Yes |
| Crosscutting | `08-crosscutting-concepts.md` | Renders the search surface in the index/second-stage section | Evidence rendering from the same run objects | BGE/NLI scenarios | Evidence output functions | Yes |
| ADR surface enumeration | ADR-052, ADR-058 | Amend surface enumerations to include `spec search --evidence` | Flag existence in every build | E2E + regular scenarios | ADR files updated in `853341b` | Yes |
| Risks | `11-risks-and-technical-debt.md` | R-38 per-invocation second-stage model load behind the opt-in flag | XDG caches + unavailable fallback | Unavailable-evidence scenarios | Summary lines `BGE evidence: unavailable` / `NLI evidence: unavailable` | Yes |
| Glossary | `12-glossary.md` | "Search evidence" term | Evidence block definition | Scenario wording | Glossary updated in `853341b` | Yes |

- [x] Every row matches the durable chapter, the design, the task, and the implementation.
- [x] The ADR-052/ADR-058 surface enumerations were amended and committed.
- [x] The added risk (per-invocation model load) is documented and mirrored in the implementation's fallback behavior.

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured by the repository's configured `coverage.measure_command`
(`./measure-coverage.sh`) + `coverage.merge_command`
(`./merge-coverages.sh`) against the delta feature path.

## Coverage Gap Classification

| Location | Lines | Classification | Action taken |
|---|---:|---|---|
| `src/spec/retrieval.rs` evidence-only branches not executed by the search-evidence scenarios that are shared with other changes | (see merged report) | Legitimately hard to test in this change | The existing permanent suite and the BGE/NLI token-cap/review scenarios own those paths; adding unrelated scenarios would violate the change boundary. |
| Branch metric (Cobertura emits no branch nodes) | 0/0 | Legitimately hard to classify in this format | Same as the repository baseline; no branch denominator exists. |

The change-scoped measurement report numbers are taken from the report emitted
by the archive gate's coverage run with the search-evidence delta path; see
`coverage/cobertura-coverage.xml` and the per-run reports.

## Overlap Dispositions

The deterministic gate reports three shape matches among the added scenarios:

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Evidence search reports a BGE verdict for every listed candidate` | `Evidence search reports an NLI verdict for every candidate` | keep-both |
| `Candidates beyond the evidence pair budget stay unscored` | `An over-limit search pair receives BGE token-cap evidence` | keep-both |
| `Candidates beyond the evidence pair budget stay unscored` | `An over-limit search pair receives NLI token-cap evidence` | keep-both |
| `An over-limit search pair receives BGE token-cap evidence` | `An over-limit search pair receives NLI token-cap evidence` | keep-both |
| `The net-delta receipt counts added, modified, and removed scenarios` | `Evidence search reports a BGE verdict for every listed candidate` | keep-both |
| `The net-delta receipt counts added, modified, and removed scenarios` | `Evidence search reports an NLI verdict for every candidate` | keep-both |

The first pair covers the two independent evidence layers (BGE verdicts vs
NLI verdicts); each requires its own model-specific fixture and assertion
path, and each runs under a different feature tag. The second, third, and
fourth pairs cover distinct evidence states: an unscored budget tail
(`BGE_UNSCORED`/`NLI_UNSCORED`), a BGE token-cap state (`BGE_TOKEN_CAP`), and
an NLI token-cap state (`NLI_TOKEN_CAP`). All three states are observable
outputs of the same search action but cannot be merged — each asserts a
different per-candidate admission state and a different user-facing
remediation. Removing either would leave a distinct evidence state untested.

The final two pairs match the permanent net-delta receipt scenario against the
two verdict scenarios: the receipt scenario asserts JSON accounting of
added/modified/removed scenarios via the review CLI, while the verbatim
scenarios assert per-candidate evidence verdicts on the search CLI. The shape
overlap comes from the shared `command exits 0` + `stdout contains` skeleton;
both are distinct user actions with different assertions and are kept as-is.

No long scenarios exceed the gate threshold, and no removed+added pairs exist
(9 added, 0 removed).

## Semantic Review and Remediation

The delta has no semantic-review candidates (the change adds a CLI surface over
existing retrieval; no scenario pair produced a semantic candidate). No
semantic report was silently omitted; the blocking review gate runs clean with
zero candidates.

## Test Suite Result

```text
GIVN_FEATURES=givn/changes/search-evidence/specs cargo test --test features_runner
10 scenarios passed (incl. 1 E2E); 61 steps passed

./run-tests.sh --non-e2e
296 scenarios passed

./run-tests.sh --e2e
53 scenarios passed

cargo clippy --workspace --all-targets -- -D warnings
passed

cargo fmt --all -- --check
passed

givn lint --change search-evidence
clean
```

**Result:** PASS

**Coverage measured:** YES

## Sign-off

- [x] Fabrication audit clean: no empty step bodies, all task commits and
      authoritative source paths checked, no downgraded or over-produced E2E
      scenarios, no parallel implementation, no design deviation.
- [x] All delta scenarios pass through the configured Cucumber runner.
- [x] Coverage measured change-scope and classified into the permitted
      buckets.
- [x] No `@wip` tags remain in the search-evidence delta.
- [x] The delta contains observable behavior only; implementation detail
      lives in `design.md`.
- [x] Local runnability requires no external service; runs offline behind
      stub env vars.
- [x] The exact E2E command and capability-specific implementation were
      read; no parallel implementation exists.
- [x] `verify.e2e_command` is distinct from `verify.command` and its count is
      strictly smaller.
- [x] Implementation matches design.md's named commands, files, driver, and
      flags; no silent deviation.
- [x] Interaction coverage matrix verified; the single inventory entry maps
      to one real-interface `@e2e` scenario driving the compiled binary.
- [x] No finding was excused with a classification outside the three buckets.

REVIEW: PASS