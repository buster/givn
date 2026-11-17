# Design review: givn-self-rebuild-under-the-updated-rules

Review kind: independent
Author: givn-self-rebuild-under-the-updated-rules author (omp)
Reviewer context: independent-design-review-2026-11-17

Reviewed revision: HEAD `9e803807` (branch `ise/implementation`) plus the
uncommitted change directory and the three modified arc42 chapters.

| Artifact | sha256 (first 16) |
|---|---|
| `design.md` | `72dc2b2c7e33aca0` |
| `proposal.md` | `de2b7a65588afc11` |
| `arc42.md` | `10e202a1fc9559b2` |
| `specs/declare-the-specification/specification-declaration.feature` | `8b3386bbc80ea151` |
| `specs/prepare-an-ise-test/clean-room-preparation.feature` | `8573ab9f60c99342` |
| `docs/arc42/02-architecture-constraints.md` | `77256df9cec9fb30` |
| `docs/arc42/04-solution-strategy.md` | `8f9e4ef12c816975` |
| `docs/arc42/11-risks-and-technical-debt.md` | `248a34e5c6d7d962` |

## Verdict

PASS. No blocking finding remains. B1, B2, B3, and A1 through A14 are closed.
The declaration, the two delta scenarios, the trial protocol, the clean-room
scan, and the durable documentation agree with the verified tree.

## Closure

| Finding | State | Evidence |
|---|---|---|
| B1 accepted-declaration evidence | CLOSED | The scratch probe with the exact declaration exits 0; the only `[SPEC ]` line for a build input is the expected `specification_feature_unretained` for `assets/templates/spec.feature`; no `specification_path_missing`, `specification_role_duplicate`, or `specification_e2e_boundary` names a build input. `## Verification` item 2, the `declaration-lint.txt` row, and the proposal's acceptance bullet record it. |
| B2 arc42 chapters | CLOSED | `git diff` shows §2.3, the `04` `Self-rebuild` row, `R-163`, and the new `R-164`; `arc42.md` rows 2, 4, and 11 match, and `STATUS: DONE` is accurate. |
| B3 clean-room scan stores | CLOSED | The scan now reads `~/.cargo/registry/src`, `~/.cargo/registry/cache`, and `~/.cargo/git` at `maxdepth 4`; D6, the mount table, the evidence row, the boundary row, C2, and the neighbor record name the same three paths. The exact loop prints nothing on this host: all three stores exist and no `givn` or `givn-*` name is present. |
| A1 route record | CLOSED | The proposal's table now matches the current output exactly: 25.50 `check-specification-health`, 24.00 `clean-room-preparation`, 22.50 `specification-declaration`, 21.00 `apply-a-managed-upgrade`, 20.00 `spec-gap`, 19.50 `spec-overlap-gates`, 19.00 `givn-workflow-enforcement`, 19.00 `validate-change-ids`, 18.00 `trial-input-verification`, 17.00 `change-strategy`. |
| A2 guidance contract | CLOSED | The `Served change guidance` row is present for `create-change`. |
| A3 untouched inventory rows | CLOSED | The matrix names the three untouched `create-change` capabilities, covering the other seven of its nine entries. |
| A4 `[[test]]` targets | CLOSED | The builder protocol, boundary row, and builder-notes row require both minimal targets before the first build. |
| A5 attempt evidence | CLOSED | Per-attempt directories, per-attempt result and log, and a final `spec-gaps.md` that names the classified attempt and lists every attempt's counts. |
| A6 fixture e2e package | CLOSED | The declaration scenario's Given writes `tests/e2e/Cargo.toml` with a minimal `[package]`. |
| A7 scenario title | CLOSED | The title is the observable the assertions prove: "A declaration that names build inputs under the four content roles is accepted". |
| A8 `Neighbor` spelling | CLOSED | The design, the evidence paths, and the prose use `Neighbor record`; no `neighbour` remains. |
| A9 absent store output | CLOSED | The scan records `absent: <path>` for a store that does not exist. |
| A10 pin scope | CLOSED | The design, `## Version freshness`, `D1`, `R-164`, and the risks bullet limit the pin to rustup builds and the sandbox and put `nix build` out of scope. |
| A11 evidence working directory | CLOSED | Every evidence block writes under `givn/changes/<id>/evidence/ise/` and runs from the repository root, matching `./run-tests.sh` and `$PWD/givn/changes/<id>/...`. |
| A12 toolchain risk wording | CLOSED | `R-164` and the risks bullet distinguish a networked rustup host (installs on first build) from a sandbox or offline host (fails before compilation). |
| A13 preparing-attempt evidence | CLOSED | `## Evidence artifacts` says the preparing attempt owns `rebuild-report.txt`, `manifest.yaml`, and `compare-before.txt`; a rerun writes the other files and references those three. |
| A14 target-parse record | CLOSED | The builder-notes row requires the exit 101 and the two authored targets when the first build hits the missing-target error. |

## Verification

### Declaration and copy

The scratch probe (`git archive HEAD`, the exact `### Exact declaration`, the
pin, `givn lint`) exits 0. It prints 52 `[SPEC ]` advisories: the one expected
`specification_feature_unretained` for `assets/templates/spec.feature` and 51
for the undeclared `test-corpus/**` fixtures, which are outside the
declaration. No other kind names a build input. The copy scenario's declared
set yields the asserted `Specification files: 7`.

### Counts

Behavior 145, structure 29, contracts 11, limits 2, Specification files 188,
source 187, e2e 100, feature paths with a selected `@e2e` scenario 56, `@e2e`
coverage entries 115, `end_to_end_files` 156, outside the spec copy 2509. Every
declared entry resolves; all 95 `assets/**` files are claimed and none is
unclaimed.

### Clean-room scan and mounts

All three scan stores exist: `~/.cargo/registry/src` (66,970 entries),
`~/.cargo/registry/cache` (2,048), and `~/.cargo/git` (27, including `db/` and
`checkouts/`). The exact scan loop prints nothing and exits 0. The sandbox
mount table binds exactly those Cargo stores (`~/.cargo/registry` and
`~/.cargo/git`), so the scan covers every Cargo path the builder can read.
`~/.cargo/registry/git` is correctly described as absent and not a store.

### Test-target rule

`Cargo.toml` declares `[[test]] name = "features_runner"` and
`[[test]] name = "ready_fixture"`. A scratch manifest with a `[[test]]` and no
test file fails `cargo build --release --offline` at manifest parse with exit
101, confirming the documented failure. The rule cannot hide a Specification
gap: both target files are under `tests/**` (the `source` role), the builder
authoring them is inside the replaced unit, the frozen acceptance is the
checkout's `e2e` package through `GIVN_BIN`, and the rule covers only the
manifest-parse failure, not a missing declared build input (still a setup error
reported by `compare`) or a scenario failure (still a gap-triage difference).
The builder notes now record the exit 101 and the two targets if it occurs.

### Chapters

`git diff` shows the §2.3 constraint rewrite, the `04` `Self-rebuild` row,
`R-163`, and `R-164`. `arc42.md` rows 2, 4, and 11 match the chapter content;
`STATUS: DONE` is accurate. The twelve chapter files exist with real content
and no ASCII art; chapter 12 is the glossary link. The independent 12-row walk
still yields 02, 04, and 11 as the only affected chapters.

### Change lint

`givn lint --change givn-self-rebuild-under-the-updated-rules` exits 2 with the
two `@wip` findings for the added scenarios and their `[SUBST]` advisories.
That is the expected pre-task state; the tasks untag the scenarios.

### Toolchain

`rustup toolchain list` holds `1.97.1`; `cargo +1.97.1 check --locked` exits 0;
the clippy and rustfmt components are installed. The pin's scope matches
`R-164`.

## Branch walk

| Branch | Result |
|---|---|
| Scope | The declaration, the two scenarios, and the trial cover the proposal's observables; the expected template advisory is recorded. No issue. |
| Tech choices | The declaration, pin, and existing commands are the simplest fit; the template stays under `structure` for the digest reason. No issue. |
| Missing scenarios | No proposed observable lacks a scenario or a trial record. No issue. |
| Testability | Strict mode is named; both bindings are planned non-empty; the delta batch selects the two added scenarios; no RED is claimed. No issue. |
| E2E fidelity | No `@e2e` tag moves; the frozen package runs through `GIVN_BIN`. No issue. |
| Interaction coverage | Five touched rows plus the three untouched `create-change` capabilities. No issue. |
| Risk | The test-target trap is documented and sound; the scan covers every mounted Cargo store; a new `specification defect` is carried by C3. No issue. |
| Use-case and Persona | Use-case IDs, capability ownership, and the three confirmed Personas match the owning documents. No issue. |
| Decision ledger | D1 and D2 resolve; D3 to D9 and C1 to C3 are recorded; every carried decision has default, trigger, switch cost, and owner. No issue. |
| Contracts and components | Four contracts, four components; the change adds none. No issue. |
| Decision timing | D1/D2 were carried then decided; no scenario depends on a carried answer; no speculative abstraction. No issue. |
| change strategy | `Decides D3` records `whole-product rebuild`, `Dry-Run Ise`, the consumers, the persisted state, and the rebuild boundary. No issue. |
| Visual design contract | `Interface: none`, viewports not applicable, three confirmed Personas with evidence. No issue. |
| Deferrals | None. No issue. |
| Persona disposition | Each Persona has impact and evidence rows. No issue. |
| Interface classification | CLI; the opposite browser classification does not hold. No issue. |
| ADR qualification | No `NEW_ADR`; chapter 02 is the cheaper canonical home. No issue. |
| Architecture documentation | The affected set is right and the chapters match `arc42.md`. No issue. |
| Ubiquitous language | `Neighbor record` and the glossary terms are consistent. No issue. |
| Structural model | No structural boundary changed; `docs/domain-model.md` stays unchanged. No issue. |

DESIGN-REVIEW: PASS
