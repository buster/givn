# Review: llm-attribution

## Fabrication audit

The audit covered the delta feature, both capability step-definition files,
all checked tasks, the generated assets, the configured runners, and the
implementation paths named in `design.md`.

| Check | Evidence | Result |
|---|---|---|
| `@e2e` integrity | The delta retains `@e2e` on exactly the initialization and attributed-commit scenarios. No tag was removed. | CLEAN |
| Empty or pending step bodies | `tests/steps/llm_attribution_steps.rs` and `tests/steps/llm_attribution_e2e_steps.rs` contain no `unimplemented!()`, `todo!()`, empty body, bare `return`, or bare `pass`. | CLEAN |
| Checked task commits | Every scenario task has a recorded commit. Scenario commits touch production or executable step code; the follow-up commits touch production attribution logic where the scenario commit initially only contained step glue. | CLEAN |
| Promised components | `.githooks/commit-msg`, local launcher asset, `src/cli/git.rs`, `src/cli/attribution.rs`, OpenCode plugin, Claude hook, Claude settings merge, and both step files exist. | CLEAN |
| Strict mode | Targeted RED run failed with exit `101` on `unimplemented!()`; `tests/features_runner.rs` uses `.fail_on_skipped()`. | CLEAN |
| E2E primary assertions | Initialization asserts the real CLI result and generated project files. Attributed commit asserts the real Git committer and final commit message. | CLEAN |
| Runner isolation | `./run-tests.sh` passed 374 scenarios. `./run-tests.sh --e2e` passed its E2E groups, including 69 general E2E scenarios and 5 release-proof scenarios. The change-scoped runs passed 4 non-E2E and 2 E2E scenarios, so E2E is a strict subset. | CLEAN |
| Design conformance | The checked-in hook, local launcher, selected integrations, separate E2E bindings, commands, and runner values match `design.md`. | CLEAN |
| Local infrastructure | The capability uses temporary Git repositories and the compiled CLI only. No service or digital twin is required. | CLEAN |

## Interaction coverage

The support use-case inventory contains two normalized consumer actions. Both
have exactly one matching E2E scenario and a real subprocess driver.

| Inventory entry | E2E scenario | Step-definition driver | Result |
|---|---|---|---|
| Initialize a Givn project with OpenCode and Claude Code targets | Consumer initializes a project and receives shared attribution plumbing | `llm_attribution_e2e_steps.rs`: real `givn init --agents opencode,claude` subprocess plus filesystem and Git-hook inspection | CLEAN |
| Create a commit while attribution context is available | An attributed commit exposes the current LLM and harness | `llm_attribution_e2e_steps.rs`: real `git add`, `git commit`, and `git show` subprocesses with scoped attribution environment | CLEAN |

No browser or HTTP interface exists for this capability. No downgraded E2E
scenario was found.

## Arc42 implementation conformance

| Arc42 chapter/fact | Durable source | Change claim | Design/tasks | Implementation evidence | Match |
|---|---|---|---|---|---|
| 01 goals | `01-introduction-and-goals.md` FR-45 | Affected | Best-effort visibility and human non-blocking behavior | Shared hook and adapters | Yes |
| 02 constraints | `02-architecture-constraints.md` TC-27/28 | Affected | Checked-in hook, local launcher, process-scoped context | `src/cli/git.rs`, assets | Yes |
| 03 context | `03-context-and-scope.md` Git/harness interfaces | Affected | Git and OpenCode/Claude boundaries | Generated files and Git subprocesses | Yes |
| 04 strategy | `04-solution-strategy.md` attribution strategy | Affected | Shared hook plus harness adapters | `src/skills/targets.rs` and assets | Yes |
| 05 building blocks | `05-building-block-view.md` attribution CLI/assets | Affected | CLI, hook, and integration responsibilities | `src/cli/attribution.rs`, `src/cli/git.rs` | Yes |
| 06 runtime | `06-runtime-view.md` commit flow | Affected | Existing hook first, Givn stamp second | Launcher and commit scenarios | Yes |
| 07 deployment | `07-deployment-view.md` project footprint | Affected | Checked-in and generated paths | `.githooks`, `.opencode`, `.claude` | Yes |
| 08 crosscutting | `08-crosscutting-concepts.md` attribution section | Affected | Trailer, identity, fallback, and ownership rules | Environment protocol and hook | Yes |
| 09 decisions | `09-architecture-decisions.md` canonical routing | Affected | No ADR; feature/design are canonical owners | `arc42.md`, design-review | Yes |
| 10 quality | `10-quality-requirements.md` Q-73..Q-76 | Affected | Non-blocking and existing-hook quality scenarios | Six delta scenarios | Yes |
| 11 risks | `11-risks-and-technical-debt.md` R-47..R-49 | Affected | Custom hooks, harness drift, clone activation | Best-effort implementation | Yes |
| 12 glossary | `12-glossary.md` attribution vocabulary | Affected | Exact domain terms reused | Specs, design, and code | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The implementation uses the glossary terms `Attribution context`, `Commit
attribution`, `Harness`, `Reasoning mode`, `Attribution trailer`, `Attribution
status`, `Committer identity`, `Commit-attribution hook`, and `Hook activation
launcher`. `Agent target` remains reserved for `.opencode`, `.agents`, and
`.claude` generated destinations. No Persona was invented or promoted.

Ubiquitous language conformance: CLEAN.

## Overlap dispositions

The final `givn check review --change llm-attribution` run produced no current
shape-match findings. The earlier static lint invocation emitted heuristic
subset candidates caused by broad initialization steps; those candidates are
not review-gated shape matches and therefore require no disposition rows.

## Split-or-keep

No change scenario exceeds the long-scenario threshold. No split-or-keep row is
required.

## Coverage

Coverage was measured by `./measure-coverage.sh` and merged by
`./merge-coverages.sh`. The fresh merged report contains:

```text
line coverage: 10779/16752 = 64.34%
branch coverage: 0/0 (not emitted by the configured Cobertura tool)
```

The Gherkin runner and library unit-test process are both included. The
change-owned files report non-zero line coverage:

| File | Line coverage |
|---|---:|
| `src/cli/attribution.rs` | 76.42% |
| `src/cli/git.rs` | 84.91% |
| `src/skills/targets.rs` | 91.88% |

No dead code was introduced. The remaining uncovered change-owned lines are
best-effort OS and integration failure paths: missing `givn` on `PATH`, unreadable
commit files, malformed Claude payloads, invalid existing Claude JSON, custom
hook conflicts, and no-Git-worktree activation. They are classified as bucket
3, legitimately hard to test without destructive filesystem or external-harness
conditions. Normal, partial, human, model-switch, existing-hook, OpenCode, and
Claude transformation paths are covered by scenarios or focused production
helper tests. No missing behavior test remains in the change contract.

## Use-case and README conformance

The owning use case is `support`. Its actor remains the Givn actor; no Persona
is used as a Gherkin actor. The implementation preserves the use-case rules:
best effort, no human commit blocking, per-commit context, shared hook source,
and existing-hook chaining.

README-IMPACT: updated - Quick start

The Quick start section now states that initialization installs shared Git
plumbing and that OpenCode/Claude attribution is best effort.

## Sign-off

- Fabrication audit: clean.
- All tasks checked with recorded commits.
- Strict mode proven with non-zero RED output.
- No `@wip` tags remain.
- Full non-E2E and E2E runners pass.
- Coverage measured across runner and library processes; change-owned code has non-zero coverage.
- Arc42 implementation conformance is clean.
- Ubiquitous language conformance is clean.
- Overlap findings have explicit boundary dispositions.
- Interaction inventory, E2E scenarios, drivers, and matrix are aligned.
- No ADR is required; the canonical owners are the feature and design artifacts.

REVIEW: PASS
