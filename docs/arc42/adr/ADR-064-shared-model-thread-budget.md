---
status: proposed
date: 2026-08-26
decision-makers:
  - givn maintainers
consulted:
  - semantic-scoring-throughput proposal, design, and executable specs
informed:
  - maintainers operating semantic review, duplicate audit, and benchmark tooling
---

# Share one thread budget across the semantic model stages

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

### MADR checklist evidence

- [x] **Hard to reverse** - the setting becomes a public environment contract across E5, BGE, and NLI; removing or renaming it later would change the configuration surface scripts and agents rely on, and the resolver's split-on-overlap rule encodes a constraint that future concurrent staging must honour.
- [x] **Shapes quality attributes** - it sets a system-wide concurrency ceiling and floor for every model stage and directly bounds model-scoring throughput on multicore machines (the change's motivating performance attribute).
- [x] **Structural** - it fixes a single shared computing-resource contract that all three independent model features obey, replacing three dispersed hardcoded thread budgets with one externally visible boundary.
- [x] **Must be shared** - a developer or agent unfamiliar with the decision would reintroduce per-stage hardcoded budgets or oversubscribe a container's quota, violating the shared-configuration rule; supporting only, never a gate.

## Context and Problem Statement

The E5, BGE, and NLI scoring stages each carried a hardcoded thread budget (`1` for E5, `4` for BGE and NLI), so multicore machines ran model scoring with one core saturated and long wall times. Each stage resolved compute parallelism independently. The stages need one user-tunable concurrency budget.

## Decision Drivers

- one externally visible compute setting for every model stage
- sensible default on machines of any size without configuration
- invalid values rejected before any model artifact, download, or inference work
- identical evidence, ordering, accounting, and report formats regardless of the resolved budget
- a deterministic, cgroup-aware default under containers and CI

## Considered Options

- Keep hardcoded per-stage thread budgets.
- Auto-detect machine parallelism with no user override.
- One shared `GIVN_SPEC_THREADS` setting with `available_parallelism()` as the default.
- Separate per-stage settings (`GIVN_SPEC_E5_THREADS`, `GIVN_SPEC_BGE_THREADS`, `GIVN_SPEC_NLI_THREADS`).

## Decision Outcome

Chosen option: **One shared `GIVN_SPEC_THREADS` setting**, resolving to `std::thread::available_parallelism()` when unset, applied to the compute threads of E5, BGE, and NLI in the real-model paths. Malformed, zero, or overflowing values are rejected by name; the semantic-review command also validates the setting eagerly, before any index or model work. Because the stages run sequentially, the full budget is never contended; if inter-stage overlap is ever introduced, the budget must be split first, and that constraint is documented at the resolver.

This record remains **proposed**: `givn status --change semantic-scoring-throughput` reports the change between design and review, so it is not yet an accepted architecture contract.

### Consequences

- Good, because one documented setting controls all model compute, with a sensible default and a clear failure mode for invalid input.
- Bad, because a single shared budget trades per-stage tuning granularity; a user cannot limit one heavy stage without limiting the others.

### Confirmation

The retained behaviour is verified at the code level (resolver semantics,
real-model thread application) plus the full-suite and E2E smoke runs; the
change carries no scenario tests by decision, because the only observably
testable surface (the setting's action on model threads) is not reachable
through the stub backends. Final `givn status`, full tests, E2E tests, and
blocking review remain required before changing this ADR to `accepted`.

## Pros and Cons of the Options

### One shared setting with available-parallelism default

- Good, because the default needs no configuration and remains quota-correct in containers.
- Good, because one knob is easy to document, validate, and apply across all three stages.
- Bad, because it cannot express a deliberately unbalanced mix of stage budgets.

### Hardcoded per-stage budgets

- Good, because no user configuration is needed.
- Bad, because multicore machines idle and wall time scales with the fixed budget.

### Auto-detection with no override

- Good, because machines self-tune.
- Bad, because users cannot cap compute in shared or constrained environments.

### Separate per-stage settings

- Good, because each stage can be tuned independently.
- Bad, because the surface is wider, defaults can diverge, and most users need only one knob.

## More Information

This extends the computing-resource configuration surface established by [ADR-063](ADR-063-reranker-pair-budget.md), which bounds how much second-stage work is performed; this record controls how many threads that bounded work uses. It depends on [ADR-060](ADR-060-actual-tokenizer-caps.md) and [ADR-061](ADR-061-surface-specific-independent-pools.md) for the unchanged score/pool evidence. NLI batched-tensor inference is explicitly out of scope and deferred to a follow-up change so wall-time gains can be measured before new tensor code is written.
