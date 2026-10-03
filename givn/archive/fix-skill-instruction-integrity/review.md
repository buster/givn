# Review: fix-skill-instruction-integrity

## Fabrication Audit

### Step definition scan

Two capability step files were inspected:

- `tests/steps/instruction_integrity_steps.rs`
- `tests/steps/instruction_integrity_e2e_steps.rs`

No empty, no-op, bare `return`, or bare `pass` bodies remain. The only
temporary `unimplemented!()` body was used during RED and was replaced before
the first GREEN result.

Result: `0 empty step bodies found across 2 step definition files`.

### Task commit cross-check

| Scenario | Commit | Production/source asset touched? |
|---|---|---|
| Canonical E2E guidance rejects weaker interface evidence | `0c764e3` | Yes: shipped instruction asset and step bindings. |
| Design guidance requires the complete E2E setup record | `7a5e7e7` | Yes: shipped instruction asset and step bindings. |
| Review guidance verifies semantic remediation | `5dfab14` | Yes: shipped instruction and review template assets. |
| Machine-readable instructions describe the effective artifact | `20e7782` | Yes: CLI source, Commands, and step bindings. |
| Artifact instructions reject an unsafe change context | `a7d05c4` | Yes: CLI source and step bindings. |
| Project instruction overrides survive skill synchronization | `fd1d3ec` | Yes: compiled-CLI acceptance bindings and feature contract. |
| Retrieval guidance distinguishes build capabilities | `8597d5e` | Yes: shipped instruction, skill, and Command assets. |
| An author requests the canonical interaction and E2E guidance | `6da5348` | Yes: compiled-CLI E2E bindings. |
| An author requests the technical E2E design guidance | `71f4056` | Yes: compiled-CLI E2E bindings. |
| A reviewer requests remediation-aware review guidance | `47d7f89` | Yes: compiled-CLI E2E bindings. |
| An agent requests machine-readable artifact guidance | `cf91807` | Yes: compiled-CLI E2E bindings. |

All 11 checked scenario tasks have a commit and evidence. Documentation-only
policy scenarios modify embedded production assets; they are not spec-only
commits.

### Promised components

| Component | Exists? | Evidence |
|---|---|---|
| `tests/steps/instruction_integrity_steps.rs` | Yes | Source file exists and regular scenarios pass. |
| `tests/steps/instruction_integrity_e2e_steps.rs` | Yes | Source file exists and four real-CLI E2E scenarios pass. |
| New production service/module | None promised | The design explicitly introduces no new domain primitive. |

### Strict-mode proof

The setup task records a targeted undefined-step failure with
`fail_on_skipped()` enabled. The final runner executes all 11 scenarios:
`11 scenarios`, `47 steps`, all passed.

### E2E tag integrity

The inventory contains four normalized actions and exactly four `@e2e`
scenarios. No planned `@e2e` tag was removed. Retrieval capability state and
the override scenario are regular variants of existing CLI authoring actions.

### E2E implementation audit

| E2E scenario | Primary assertion | Real interface | Driver | Downgraded? |
|---|---|---|---|---|
| An author requests the canonical interaction and E2E guidance | CLI stdout contains complete policy and retrieval-mode wording | CLI | Compiled Givn subprocess | No |
| An author requests the technical E2E design guidance | CLI stdout contains all design E2E fields | CLI | Compiled Givn subprocess | No |
| A reviewer requests remediation-aware review guidance | CLI stdout contains classification and remediation contracts | CLI | Compiled Givn subprocess | No |
| An agent requests machine-readable artifact guidance | CLI stdout parses as one JSON document with four contract fields | CLI | Compiled Givn subprocess | No |

`verify.e2e_command` is `./run-tests.sh --e2e`. It invokes the
`features_runner` binary with `--tags @e2e`; the exact E2E bindings are in
`tests/steps/instruction_integrity_e2e_steps.rs`. No parallel E2E implementation
for this capability exists.

### E2E isolation and interaction coverage

The change suite reports 11 total scenarios and 4 E2E scenarios. The repository
suite reports 282 total scenarios and 37 E2E scenarios. The E2E count is a
strict subset in both cases.

| Inventory action | Design matrix row | Matching E2E scenario | Driving mechanism |
|---|---|---|---|
| request canonical interaction and E2E authoring guidance, including retrieval capability state | Yes | An author requests the canonical interaction and E2E guidance | Compiled CLI subprocess |
| request the technical design guidance for E2E setup | Yes | An author requests the technical E2E design guidance | Compiled CLI subprocess |
| complete a semantic review with class-specific remediation evidence | Yes | A reviewer requests remediation-aware review guidance | Compiled CLI subprocess |
| request machine-readable artifact guidance | Yes | An agent requests machine-readable artifact guidance | Compiled CLI subprocess and JSON parser |

No inventory entry is unmapped and no E2E scenario is over-produced.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Canonical complete E2E policy | ch.01 FR-30, ch.08.3a | Chapter 1 Yes; chapter 8 Yes | Source policy and derived-layer separation | Canonical guidance scenario | `assets/instructions/specs.md`, `0c764e3` | Yes |
| Instruction JSON contract | ch.03 interface, ch.06.3, ch.08.3b | Chapters 3, 5, 6, 10 Yes | `{id, generates, requires, instruction}` | JSON regular/E2E scenarios | `src/cli/instructions.rs`, `20e7782`, `cf91807` | Yes |
| Semantic remediation evidence | ch.08.5e, ch.10 Q-67-Q-70 | Chapters 8, 10, 11 Yes | Class contracts and receipt | Review guidance scenario | `assets/instructions/review.md`, `assets/templates/review.md`, `5dfab14` | Yes |
| Retrieval feature modes | ch.03, ch.11 R-47 | Chapters 3, 8, 11 Yes | Feature-aware guidance | Retrieval scenario | `assets/instructions/specs.md`, `8597d5e` | Yes |
| Override and generated snapshot behavior | ch.03, ch.08, ch.09 ADR-048 | Chapters 3, 8, 9, 10 Yes | Cascade and sync behavior | Override scenario | `src/config/cascade.rs`, `fd1d3ec`, `givn skills sync` | Yes |
| No deployment change | ch.07 | Chapter 7 No | No deployment decision | No task | No deployment files changed | Yes |

`ARC42 CONFORMANCE: CLEAN`

## Coverage

Coverage was measured with the configured project scripts after the full
non-E2E and E2E suites:

```text
./measure-coverage.sh: pass
./merge-coverages.sh: pass
Merged lines: 7518/11904 (63.2%)
Merged branches: 0/0 (not measured by the runner)
Non-E2E lines: 4213/5952 (70.8%)
E2E lines: 3305/5952 (55.5%)
```

The changed CLI instruction paths and all changed policy assets have targeted
regular or E2E scenarios. No new dead production code was introduced. No
change-specific uncovered behavior remains unclassified; the merged report
contains existing repository-wide uncovered regions outside this capability,
while the changed paths are exercised by the scenarios above.

## Verification

- `givn lint --change fix-skill-instruction-integrity`: clean.
- Change non-E2E suite: `7 scenarios`, `34 steps`, all passed.
- Change E2E suite: `4 scenarios`, `13 steps`, all passed.
- Repository non-E2E suite: `245 scenarios`, `1530 steps`, all passed.
- Repository E2E suite: `37 scenarios`, `224 steps`, all passed.
- Repository full suite: `282 scenarios`, `1754 steps`, all passed.
- `cargo fmt --all -- --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- Retrieval-enabled binary installed and all configured skill targets synced.
- Root `.opencode`, `.agents`, `.claude`, and harness mirrors contain no stale
  pre-retrieval prohibition or obsolete instruction metadata keys.

## Sign-off

- [x] Fabrication audit clean.
- [x] All checked tasks have evidence and commits.
- [x] Strict-mode proof present.
- [x] Both configured runners pass.
- [x] Coverage measured and recorded.
- [x] Arc42 conformance clean.
- [x] Canonical E2E policy, design fields, retrieval modes, remediation
      contracts, override semantics, and JSON contract are verified.

REVIEW: PASS
