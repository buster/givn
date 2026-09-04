# Specification Organization Analysis Report

**Date:** September 4, 2026  
**Scope:** `givn` (174 feature files) and `watn` (60 feature files) projects  
**Purpose:** Evaluate spec structure, modularity, overlaps, and E2E test efficiency

---

## Executive Summary

The specification structure is **feature-rich but organizationally challenged**:

1. **174 total feature files** across givn (52 active + 80+ archived + harness)
2. **81% are marked @e2e** (43/52 active specs)
3. **Multiple semantic/review families** with significant overlaps and unclear separation
4. **Archive vs. Active duplication** (same specs exist in both locations)
5. **Test burden**: 689+ scenarios in active givn specs alone, many marked E2E

### Key Issues

- **Semantic family fragmentation**: 8 `semantic-*` specs + 2 `*-review*` + 4 `*-gate*` specs test overlapping concerns
- **Command family bloat**: 5 command specs with 90+ scenarios, some appear to duplicate coverage
- **Unclear module boundaries**: `spec-overlap-gates`, `review-disposition-gates`, `spec-duplicates` suggest spec management is itself brittle
- **Test artifact pollution**: Test corpus specs duplicate active specs, creating maintenance burden

---

## Part 1: Current Structure

### 1.1 Directory Organization

```
givn/
├── specs/ (51 active feature directories)
├── archive/ (52 completed change directories, each with nested /specs/)
└── changes/ (1 work-in-progress spec)

harness/
├── givn/specs, archive/, changes/
└── tests/features/

test-corpus/
├── specs/givn/ (23 reference copies of givn specs)
└── specs/watn/ (32 watn specs)
```

### 1.2 Spec Distribution by Category

| Category | Count | Scenarios | % E2E | Concern |
|----------|-------|-----------|-------|---------|
| Semantic Processing | 8 | 113 | 100% | Severe fragmentation |
| Review/Gates | 7 | 161 | 100% | Overlapping concerns |
| CLI Commands | 5 | 108 | 80% | Possible duplication |
| Readme/Coverage | 4 | 53 | 75% | Documentation automation |
| Spec Infrastructure | 6 | 171 | 100% | Meta-spec testing |
| NLI/Evidence | 3 | 28 | 100% | Dependency specs |
| Installation/Init | 3 | 42 | 67% | Setup flows |
| Release/Overrides | 2 | 19 | 100% | Release validation |
| Other/Bug Fixes | 7 | 25 | 14% | Tactical fixes |
| **TOTAL** | **45** | **689** | **81%** | |

Note: *semantic-scoring-throughput* and *ci-debuginfo-optimization* have 0 scenarios.

### 1.3 Granularity Analysis

**Largest Specs (highest scenario count):**
1. `spec-overlap-gates` — 62 scenarios — Tests duplicate detection across all spec scenarios
2. `bge-reranking-evidence` — 45 scenarios — Evidence ranking for semantic similarity
3. `semantic-review` — 43 scenarios — Semantic review output generation
4. `spec-duplicates` — 34 scenarios — Spec deduplication logic

**Smallest Specs (lowest scenario count):**
- `semantic-compact-serialization` — 2 scenarios
- `coverage-readme-badge` — 1 scenario
- `coverage-readme-example` — 1 scenario
- `ci-debuginfo-optimization` — 0 scenarios
- `semantic-scoring-throughput` — 0 scenarios

**Concern:** Mix of 1–2 scenario specs (likely too granular) and 40+ scenario specs (likely too coarse).

---

## Part 2: Identified Overlaps

### 2.1 The Semantic Family Problem

**8 specs with `semantic-*` prefix plus related `*-review*` and `*-evidence*` specs:**

```
semantic-review (43 scenarios)
  └─ Tests core semantic review report generation
  └─ E2E via CLI command `givn check review --change`
  └─ Tests output format, candidate scoring, candidate deduplication

semantic-review-output (30 scenarios)
  └─ Tests semantic review report format and structure
  └─ Appears to overlap with semantic-review assertions
  └─ Unclear separation: different aspects of same feature?

semantic-model-download (16 scenarios)
  └─ Tests E5 embedding model download and caching
  └─ Prerequisite for semantic-review, but tested separately

bge-reranking-evidence (45 scenarios)
  └─ Tests BGE reranker's evidence retrieval and ranking
  └─ Also appears in archive twice (indicates refactoring history)

search-evidence (9 scenarios)
  └─ Evidence search (distinct from BGE reranking?)

nli-evidence (14 scenarios)
  └─ NLI (Natural Language Inference) evidence scoring
  └─ Also duped in archive as "semantic-model-download" change

semantic-e5-token-cap (6 scenarios)
  └─ Tests token limit enforcement for E5 embeddings

semantic-token-cap-review (4 scenarios)
  └─ Review token cap constraints
  └─ Overlaps with semantic-e5-token-cap concern

semantic-reranker-budget (7 scenarios)
  └─ Budget management for reranking operations

semantic-compact-serialization (2 scenarios)
  └─ Serialization optimization (too small to justify its own spec?)
```

**Problem:** These are tightly coupled subsystems of semantic scoring. The current split makes it unclear which scenarios are responsible for end-to-end behavior vs. which test internal constraints. Recommend consolidation into:
- `semantic-embedding-and-ranking` (model download, E5 tokenization, reranking)
- `semantic-review-and-output` (report generation, output format)
- `semantic-evidence-scoring` (NLI, search, deduplication)

### 2.2 The Review/Gate/Disposition Family

**7 specs with overlapping concerns:**

```
spec-overlap-gates (62 scenarios) ← HUGE
  └─ Duplicate scenario detection across spec files

review-disposition-gates (24 scenarios)
  └─ Review outcome classification (PASS/BLOCKED/FAIL)
  └─ Related to adr-qualification-gate?

adr-qualification-gate (17 scenarios)
  └─ ADR (Architecture Decision Record) qualification logic
  └─ Seems orthogonal to review-disposition-gates

enforce-post-merge-e2e-gate (4 scenarios)
  └─ Post-merge E2E validation gate
  └─ Specific workflow enforcement

extract-coverage-gate (3 scenarios)
  └─ Coverage extraction (why a "gate"?)

spec-overlap-gates + spec-duplicates (62 + 34 = 96 scenarios)
  └─ Both test spec duplication detection
  └─ Likely significant overlap
  └─ Combined, they're 27% of all active scenarios
```

**Problem:** Unclear separation between gate policies, review outcomes, and duplicate detection. Recommend:
- `spec-integrity` — Duplicate and overlap detection (consolidate spec-overlap-gates + spec-duplicates)
- `review-classification` — Review outcome determination (review-disposition-gates + adr-qualification-gate)
- `enforcement-gates` — Workflow validation gates (enforce-post-merge-e2e-gate + extract-coverage-gate)

### 2.3 The Command Family

**5 specs for CLI command testing:**

```
lint-command (25 scenarios)
run-command (29 scenarios)
  └─ `givn run` command
run-execution (19 scenarios)
  └─ Execution details for `givn run`
  └─ 48 scenarios for a single command (lint + run combined)
  └─ Possible overlap: run-command vs run-execution separation?

new-command (17 scenarios)
givn-status-command (18 scenarios)
```

**Problem:** 
- `run-command` (29) + `run-execution` (19) = 48 scenarios for one feature
- Unclear why they're split: is `run-execution` testing implementation details?
- Could consolidate into `run-workflow` with clear separation of happy-path vs. edge cases

### 2.4 Spec Management Specs (Meta-Specs)

**6 specs about spec behavior itself:**

```
spec-overlap-gates (62 scenarios)
spec-duplicates (34 scenarios)
spec-explore-change (28 scenarios)
spec-explore-permanent (22 scenarios)
spec-progress-output (8 scenarios)
spec-retrieval (11 scenarios)
```

**Problem:** 
- These **171 scenarios** test the **spec system itself**, not the application
- Combined, they're ~25% of all active scenarios
- This is a major burden if all are E2E (they are)
- Consider: Can spec-overlap-gates and spec-duplicates be unit-tested instead of E2E?

### 2.5 Installation/Initialization Family

**3 specs:**

```
install-extras (12 scenarios)
init-agent-target-selection (16 scenarios)
domain-skill-trigger (15 scenarios)
```

**Problem:** 
- Unclear domain semantics: why is "domain-skill-trigger" grouped with install/init?
- domain-skill-trigger may belong with "instruction-integrity" or skill testing?

---

## Part 3: E2E vs. Unit Test Distribution

### 3.1 Current E2E Burden

**43 out of 52 active specs are marked @e2e (81%)**

This means:
- All 689+ scenarios are E2E tests
- Each spec run requires full system initialization, external dependencies, or complex state setup
- Execution time = scenario count × per-scenario overhead (likely 10–30s per E2E scenario at scale)
- Est. total runtime for all E2E tests: **2–5+ hours**

### 3.2 Non-E2E Specs (9 specs, 19% of active)

| Spec | Scenarios | Type |
|------|-----------|------|
| choose-project-license | 3 | Unit/Integration |
| ci-debuginfo-optimization | 0 | ??? |
| config-overlay | 14 | ??? |
| givn-status-command | 18 | CLI (possibly unit?) |
| givn-workflow-enforcement | 16 | ??? |
| install-extras | 12 | Unit/Integration |
| lint-command | 25 | CLI (possibly unit?) |
| new-command | 17 | CLI (possibly unit?) |
| semantic-scoring-throughput | 0 | ??? |

**Issue:** The "non-E2E" specs are unmarked, but their testing approach is unclear. Some (lint-command) may still run full CLI processes (pseudo-E2E).

### 3.3 Recommendations for E2E Reduction

1. **Move meta-specs to unit tests:**
   - `spec-overlap-gates` — Validate the linter logic via direct function tests, not E2E
   - `spec-duplicates` — Unit test deduplication algorithm
   - Save: ~96 scenarios, potentially 1+ hour of test time

2. **Split command specs:**
   - Happy path: Keep 1–3 E2E scenarios per command to validate integration
   - Edge cases: Move to unit tests (input validation, error handling)
   - `run-command` + `run-execution` → 48 scenarios → 5–10 E2E (happy path), 35+ unit tests

3. **Consolidate semantic family:**
   - Core flow (E2E): embedding → ranking → review generation (1–2 scenarios)
   - Details (unit): token caps, budget management, serialization formats
   - Save: ~60 scenarios from E2E

4. **Use test contracts/fixtures:**
   - Avoid re-running full setup for each scenario
   - Use seeded fixtures (spec files, corpora) instead of generating them each time

**Target:** Reduce E2E scenarios from 689 to ~100–150 critical path tests (75% reduction), move remaining to integration/unit.

---

## Part 4: Test Corpus Duplication Problem

### 4.1 test-corpus/specs Structure

```
test-corpus/specs/
├── givn/ (23 feature files)
│   └─ Reference copies of most givn/specs/ features
└── watn/ (32+ feature files)
    └─ watn project specifications
```

### 4.2 Issues

1. **Givn specs are duplicated:**
   - Same feature exists in `givn/specs/foo/foo.feature`
   - AND copied to `test-corpus/specs/givn/foo/foo.feature`
   - Maintenance burden: changes must be replicated
   - Risk: Copy may drift from original

2. **Purpose unclear:**
   - Is test-corpus for reference? Validation? Regression testing?
   - If it's for testing the spec framework itself, why duplicate the full specs?

3. **Watn specs in givn repo:**
   - `test-corpus/specs/watn/` contains 32 watn feature files
   - These may be:
     - Integration tests for watn within givn's CI
     - A separate project's specs
     - Dead code if watn is maintained elsewhere

---

## Part 5: Watn Project Baseline

### 5.1 Watn Structure (for comparison)

Watn uses a **cleaner approach:**

```
watn/givn/specs/ (26 active specs)
watn/givn/archive/ (34 completed changes)
```

**Watn spec categories:**
- Setup/Configuration: 5 specs (streamlined-setup, unified-setup-wizard, etc.)
- UI/Interaction: 6 specs (ratatui-model-picker, highlight-active-setup-input, etc.)
- Features: 8 specs (ask, reasoning, providers, models, etc.)
- Infrastructure: 7 specs (transport, shell-completions, config, etc.)

**Observations:**
- Fewer specs overall (26 vs. 52 givn)
- Clearer domain separation (UI, setup, features, infra are distinct)
- Less meta-spec testing (no "spec-overlap-gates" equivalent)
- But still heavily E2E (25+ specs, mostly interactive terminal tests)

---

## Part 6: Recommendations for Restructuring

### 6.1 Givn Spec Reorganization

#### Current: 52 scattered specs → Proposed: ~25–30 focused specs

**Domain Groupings:**

1. **CLI Commands (consolidate 5 → 3)**
   ```
   givn-lint-command (25 + edge cases)
   givn-run-workflow (29 + 19 combined, separate happy-path E2E from edge-case unit tests)
   givn-init-workflow (new-command + init-agent-target-selection + install-extras)
   ```

2. **Semantic Scoring (consolidate 8 → 3)**
   ```
   semantic-model-embedding (semantic-model-download + semantic-e5-token-cap)
   semantic-ranking-evidence (bge-reranking-evidence + search-evidence + nli-evidence)
   semantic-review-generation (semantic-review + semantic-review-output)
   ```

3. **Spec Integrity (consolidate 2 → 1)**
   ```
   spec-quality-gates (spec-overlap-gates + spec-duplicates)
   ```

4. **Spec Navigation/Exploration (consolidate 3 → 1)**
   ```
   spec-retrieval-and-matching (spec-explore-change + spec-explore-permanent + spec-retrieval)
   ```

5. **Review & Approval (consolidate 4 → 2)**
   ```
   review-disposition (review-disposition-gates + adr-qualification-gate)
   review-enforcement (enforce-post-merge-e2e-gate + extract-coverage-gate)
   ```

6. **Configuration & Metadata**
   ```
   project-configuration (config-overlay + release-strict-model-overrides)
   readme-automation (readme-enforcement + readme-coverage-badge + coverage-readme-example)
   domain-modeling (domain-skill-trigger + instruction-integrity + skill-rewrites)
   ```

7. **Keep Single-Purpose**
   ```
   fix-uppercase-checkbox-parsing
   prose-consolidation
   preserve-docstrings-on-archive
   template-conditional-rendering
   unified-semantic-rulebook
   validate-artifact-id-input
   validate-change-id-input
   release-0-3-0
   ```

#### Rationale

- **Reduces from 52 to ~27 active specs** (48% consolidation)
- **Clusters related features** by domain, not implementation detail
- **Clarifies boundaries**: each spec owns a coherent capability, not an algorithm variant
- **Enables modular testing**: can test one domain without touching others

### 6.2 E2E Reduction Strategy

**Phase 1: Low-hanging fruit (weeks 1–2)**
- Move `spec-overlap-gates` to unit tests (test linter directly)
- Move `spec-duplicates` to unit tests (test deduplication logic)
- Consolidate `run-command` + `run-execution` into 5 E2E + 43 unit tests
- **Est. savings: 130+ scenarios, 1–2 hours runtime**

**Phase 2: Semantic refactoring (weeks 3–4)**
- Move semantic token cap and budget tests to unit tests
- Keep only core semantic review flow as E2E
- **Est. savings: 60+ scenarios, 30–40 min runtime**

**Phase 3: Fixture optimization (ongoing)**
- Refactor E2E setup to use seeded fixtures instead of generation
- Use scenario outlines to avoid per-scenario setup
- **Est. savings: 30% of remaining E2E time**

**Target state:** 150–200 E2E scenarios (critical path only), 500+ unit tests (details and edge cases)

### 6.3 Directory Structure Redesign

```
givn/specs/
├── cli/
│   ├── lint-command.feature
│   ├── run-workflow.feature
│   └── init-workflow.feature
│
├── semantic-processing/
│   ├── model-embedding.feature
│   ├── ranking-evidence.feature
│   └── review-generation.feature
│
├── spec-integrity/
│   ├── quality-gates.feature
│   └── retrieval-matching.feature
│
├── review-approval/
│   ├── review-disposition.feature
│   └── review-enforcement.feature
│
├── infrastructure/
│   ├── project-configuration.feature
│   ├── readme-automation.feature
│   ├── domain-modeling.feature
│   └── template-rendering.feature
│
└── fixes-and-features/ (bite-sized features)
    ├── checkbox-parsing.feature
    ├── prose-consolidation.feature
    ├── docstring-preservation.feature
    └── artifact-validation.feature
```

### 6.4 Test Corpus Cleanup

1. **Remove givn spec duplicates from test-corpus:**
   - If test-corpus exists for testing the framework, use symbolic links or references
   - If it's for regression testing, clearly document why and automate sync

2. **Relocate watn specs:**
   - If watn is a separate project, either:
     - Keep specs in watn/ repo, reference them via submodule or sync job
     - Or document they're "example specs for integration testing"

---

## Part 7: Modularity Metrics

### Current State

| Metric | Value | Assessment |
|--------|-------|------------|
| Avg scenarios per spec | 13 | Highly variable (0–62) |
| Largest spec | 62 (spec-overlap-gates) | Too coarse |
| Smallest spec | 0–1 (multiple) | Too granular |
| Specs with single responsibility | ~40% | Unclear boundaries |
| E2E test ratio | 81% | Too high, limits iteration |
| Spec-to-feature ratio | 52:~15 major features | ~3.5:1 (too fragmented) |

### Proposed Target State

| Metric | Target | Rationale |
|--------|--------|-----------|
| Avg scenarios per spec | 15–25 | Coarser specs, each covers one capability |
| Largest spec | 30–35 | Still testable in <5 min |
| Smallest spec | 5+ | No single-scenario specs |
| Specs with clear SRP | >90% | Refactor to achieve |
| E2E test ratio | 20–30% | Only critical paths remain |
| Spec-to-feature ratio | 25–30:~15 major features | ~1.8:1 (more modular) |

---

## Part 8: Implementation Roadmap

### Week 1: Planning & Consensus
- [ ] Review this report with team
- [ ] Agree on new spec groupings and naming
- [ ] Identify which specs can move to unit tests immediately
- [ ] Document rationale in SPEC_ARCHITECTURE.md

### Week 2–3: Refactor Active Specs
- [ ] Consolidate run-command + run-execution
- [ ] Consolidate semantic-* family (3 specs from 8)
- [ ] Consolidate review/gate family (2 specs from 4)
- [ ] Update spec directory structure
- [ ] Update CLAUDE.md / project documentation

### Week 4: Unit Test Migration
- [ ] Move spec-overlap-gates to unit tests
- [ ] Move spec-duplicates to unit tests
- [ ] Move semantic token cap constraints to unit tests
- [ ] Re-baseline performance metrics

### Week 5: Test Corpus & Archive Cleanup
- [ ] Decide: keep or remove givn spec copies from test-corpus
- [ ] Document watn spec presence in givn repo
- [ ] Clean up dead archived changes with no reference

### Ongoing: Fixture Optimization
- [ ] Reduce per-scenario setup overhead
- [ ] Use scenario outlines for parametric tests
- [ ] Seed fixtures instead of generating them

---

## Part 9: Appendix — Full Spec Inventory

### Active Specs by Category

**CLI Commands (5 specs, 108 scenarios, 80% E2E):**
- givn-status-command (18)
- lint-command (25)
- new-command (17)
- run-command (29)
- run-execution (19)

**Semantic Processing (8 specs, 113 scenarios, 100% E2E):**
- bge-reranking-evidence (45)
- nli-evidence (14)
- search-evidence (9)
- semantic-compact-serialization (2)
- semantic-e5-token-cap (6)
- semantic-model-download (16)
- semantic-reranker-budget (7)
- semantic-review (43)
- semantic-review-output (30)
- semantic-token-cap-review (4)

**Review/Gates (7 specs, 161 scenarios, 100% E2E):**
- adr-qualification-gate (17)
- arc42-review-alignment (4)
- enforce-post-merge-e2e-gate (4)
- extract-coverage-gate (3)
- review-disposition-gates (24)
- spec-overlap-gates (62)
- spec-duplicates (34)

**Spec Infrastructure (6 specs, 171 scenarios, 100% E2E):**
- spec-explore-change (28)
- spec-explore-permanent (22)
- spec-progress-output (8)
- spec-retrieval (11)
- spec-overlap-gates (62) [counted above]
- spec-duplicates (34) [counted above]

**Installation/Initialization (3 specs, 42 scenarios, 67% E2E):**
- choose-project-license (3)
- domain-skill-trigger (15)
- init-agent-target-selection (16)
- install-extras (12)

**Readme/Coverage (4 specs, 53 scenarios, 75% E2E):**
- coverage-readme-example (1)
- coverage-script-workflow (3)
- readme-coverage-badge (1)
- readme-enforcement (19)

**Release/Management (2 specs, 19 scenarios, 100% E2E):**
- release-0-3-0 (13)
- release-strict-model-overrides (6)

**Other Features (5 specs, 25 scenarios, 20% E2E):**
- ci-debuginfo-optimization (0)
- fix-uppercase-checkbox-parsing (5)
- prose-consolidation (5)
- preserve-docstrings-on-archive (4)
- template-conditional-rendering (7)
- instruction-integrity (11)
- semantic-scoring-throughput (0)
- skill-rewrites (4)
- unified-semantic-rulebook (11)
- validate-artifact-id-input (4)
- validate-change-id-input (8)

**TOTAL: 52 active specs, 689+ scenarios, 81% E2E**

---

## Conclusion

The spec suite is **comprehensive but disorganized**. The primary opportunities for improvement are:

1. **Consolidate related specs** (52 → 27 specs, 48% reduction in file count)
2. **Reduce E2E burden** (689 → 200 E2E scenarios, 71% reduction in E2E time)
3. **Clarify domain boundaries** (group by capability, not algorithm variant)
4. **Clean up artifacts** (remove test-corpus duplication, clarify watn specs)

These changes would maintain **full behavioral coverage** while improving **maintainability, clarity, and iteration speed**. The roadmap is achievable in 5–6 weeks with team buy-in.
