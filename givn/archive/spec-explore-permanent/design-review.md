# Design Review: spec-explore-permanent

## Review Context

The review examined `proposal.md`, the complete delta feature, `design.md`,
`arc42.md`, all twelve durable Arc42 chapters, the existing CLI dispatch,
retrieval implementation, Gherkin runner, process helpers, and retrieval
fixtures. The explicit boundary is PR 1 only: permanent-only exploration of
one complete Gherkin scenario through `--file` or `--stdin`.

## Grilling Findings and Resolutions

### 1. Scope

**Finding: resolved before sign-off.** The initial feature covered malformed
Gherkin and multiple scenarios but did not explicitly cover a valid feature with
no scenario, a missing file, or simultaneous `--file` and `--stdin`. Those are
required observable rejection paths. Three scenarios were added:

- `A valid feature without a scenario is rejected`
- `A missing draft file is rejected`
- `File and standard input cannot be selected together`

No later-layer capability was added. There is no `--change`, report,
classification, remediation, duplicate command, BGE/NLI path, coverage
function, step resolver, or automatic edit.

### 2. Technology Choices

**No issue found.** The existing Clap/Gherkin/Rust retrieval path is the
smallest compatible implementation. Reusing `SpecIndex`, its content hash,
embedding roles, and deterministic `rank_entries` avoids a second index format
or model. A separate JSON renderer is required because the existing index
refresh prints human diagnostics, and emitting those diagnostics before JSON
would violate the machine-readable contract.

### 3. Missing Scenarios

**Finding: resolved before sign-off.** The missing-input and no-scenario
boundaries are now executable. The feature also covers all requested positive,
incremental, identity, non-mutation, feature-disabled, and existing-command
compatibility cases.

Unreadable-file handling shares the file-read boundary with the missing-file
scenario. The implementation must attach the file path to the read error; a
permission-specific fixture would be platform-sensitive and add no distinct
observable contract.

### 4. Testability and RED

**No blocking issue found.** New capability bindings are placed in dedicated
step modules and initially use `unimplemented!()` for unique behavior. The
existing runner calls `.fail_on_skipped()`, so undefined/skipped steps fail.
The three compatibility scenarios intentionally reuse already-green retrieval
steps; they are regression evidence rather than new behavior and are exempt
from inventing a failing stub. Every new behavior scenario has a concrete
exit, stdout, stderr, JSON, filesystem, or index-count assertion.

### 5. E2E Fidelity

**No issue found.** The capability is CLI-only. E2E scenarios spawn the real
compiled binary, use a temporary project, pass a real file or piped stdin, and
assert on terminal output or parsed JSON. `./run-tests.sh --e2e` is a strict
`@e2e` subset of `./run-tests.sh`; it uses the deterministic stub and no network
service. The interaction matrix maps all three inventory entries to exactly one
e2e scenario each.

### 6. Same-Feature Candidate Boundary

**Finding: constrained and resolved in the fixture design.** A file query must
contain exactly one scenario, so a permanent file used directly as a query
cannot also contain a second query-file scenario. The normal exploration corpus
therefore puts three permanent scenarios in one feature file and queries an
external one-scenario draft; this makes same-feature candidates observable.
The exact-identity fixture uses a one-scenario permanent query file and places
other candidates in separate files. The implementation still filters by the
pair `(file, title)`, never by file alone. This preserves the required contract
without relaxing single-scenario validation.

### 7. Risk

**Primary risk:** path normalization or refresh logging can silently corrupt
identity filtering or JSON consumption. The design now requires one normalized
path helper, fixed JSON structs, and separate refresh metadata from rendering.
The corresponding risks are recorded as R-30 and R-31 in durable Arc42.

### 8. Arc42

**No issue found after independent re-derivation.** The twelve-row assessment
was independently checked against proposal/spec/design:

| # | Result | Evidence |
|---|---|---|
| 1 | Yes | New authoring goal and FR-26 in chapter 1. |
| 2 | No | Existing Rust/Gherkin/feature-gated/local constraints are reused. |
| 3 | Yes | New CLI input/output interface in chapter 3. |
| 4 | Yes | Embedding-only advisory strategy in chapter 4. |
| 5 | Yes | CLI/retrieval/test building blocks in chapter 5. |
| 6 | Yes | Permanent exploration flow in chapter 6. |
| 7 | No | No deployment artifact, service, or port change. |
| 8 | Yes | Validation, hash reuse, deterministic output, and read-only boundary in chapter 8. |
| 9 | Yes | ADR-043 documents the complete-scenario permanent-only decision. |
| 10 | Yes | Q-42 through Q-45 cover the new quality contract. |
| 11 | Yes | R-30 and R-31 cover path identity and JSON contamination. |
| 12 | Yes | Explore query, exact identity, and embedding score are glossary terms. |

The marker matches this table. All twelve durable chapter files exist and
contain project-specific content. New diagrams use Mermaid. Existing plain
code blocks and directory listings are descriptions, not newly introduced
architecture diagrams. ADR-043's negative consequences are reflected in R-30
and R-31 and in the implementation boundary.

## Hardening Applied

- Added three explicit invalid-input scenarios.
- Split e2e step wording from regular capability step wording so the e2e step
  module remains a separate real-CLI boundary.
- Documented the two fixture corpus shapes needed to observe same-feature
  ranking without violating single-scenario input validation.
- Documented the no-feature acceptance process and the explicit compatibility
  boundary.
- Updated durable Arc42 chapters and the change-level impact marker.
- Ran the static spec check after hardening; `@wip` is expected until the
  scenario-by-scenario implementation loop removes the tags.

## Sign-Off Checklist

- [x] Scope matches the PR 1 proposal and product boundary.
- [x] All required rejection paths have executable scenarios.
- [x] All interaction-inventory entries map to one `@e2e` scenario.
- [x] Real CLI process and strict runner mechanism are specified.
- [x] Permanent-only index scope and exact identity semantics are explicit.
- [x] No BGE, NLI, coverage, classification, review decision, or mutation is
      introduced.
- [x] Arc42 impact table, durable chapters, ADR, risks, and glossary agree.

DESIGN-REVIEW: PASS
