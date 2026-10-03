# Review: visual-ui-review

## Fabrication Audit (do this first)

**Step definition scan** — every touched step definition file checked for
empty/no-op bodies (`{}`, bare `pass`, bare `return`, `unimplemented!()`
left behind, no assertion reference):

```
rg -n "unimplemented!|todo!|\{\s*\}|pass$|return;" tests/steps/visual_review_gate_steps.rs tests/steps/visual_design_contract_steps.rs tests/steps/prose_consolidation_steps.rs
-> only two `panic!` matches inside existing unwrap_or_else handlers; 0 empty
   step bodies across 3 files
```

**@e2e tag integrity** — every delta `.feature` file checked. The planned
`@e2e` tags are all present: the modified prose-consolidation scenario, both
visual-design-contract `@e2e` scenarios, and both visual-review-gate `@e2e`
scenarios. No tag was removed from any scenario.

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source:

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Setup: strict-mode proof and skeletons | `33b5174d` | Test runner sources (`tests/steps/`) |
| Canonical policy requires the visual baseline | `1906f05c` | `assets/instructions/specs.md` |
| Canonical policy uses transcripts | `6287d8f4` | `assets/instructions/specs.md` |
| Design guidance names the default viewports | `99c4eeab` | `assets/instructions/design.md` |
| Design guidance keeps improvements inside the style | `05a63a24` | `assets/instructions/design.md` |
| Design-review guidance requires an approved contract | `9b13a7e4` | `assets/instructions/design-review.md` |
| The design template carries the visual contract section | `3df5b660` | `assets/templates/design.md` + step |
| A visual change without a visual verdict fails review | `90d19b9e` | `src/integrity_hook.rs` + steps |
| A change without a visual contract is exempt | `03fe25ca` | Step implementation; exemption path in `90d19b9e` |
| A change that declares no visual interface is exempt | `be04ae99` | Step implementation; `none` path in `90d19b9e` |
| A web contract without viewports fails review | `14522f52` | `src/integrity_hook.rs` + steps |
| A visual change with incomplete evidence fails review | `7a263654` | `src/integrity_hook.rs` + steps |
| An empty evidence file fails review | `6b0137bb` | `src/integrity_hook.rs` + step |
| An unverified visual verdict cannot pass review | `1e91a0b9` | `src/integrity_hook.rs` |
| A terminal change passes review with a transcript | `a7a8d06b` | Steps; terminal branch in `7a263654` |
| Review guidance treats style improvements as contract updates | `80d79f1a` | `assets/instructions/review.md` |
| Review guidance records an unverified verdict | `686c0c82` | `assets/instructions/review.md` |
| The review template carries the visual review section | `fe8d6958` | `assets/templates/review.md` |
| Synchronized review guidance carries the visual block | `3b6965c3` | `assets/skills/givn-review/SKILL.md.tmpl` |
| Canonical E2E policy is emitted by specs instructions | `64f4ad30` | `assets/instructions/specs.md` |
| Author requests the visual design contract guidance | `f305eefc` | `assets/instructions/design.md` |
| Synchronized guidance defines the contract approval | `458acb68` | `assets/skills/givn-design-review/`, `givn-steps/` |
| A visual change with complete evidence passes review | `0f6ded04` | `src/integrity_hook.rs` (verdict-state refactor; activation `03630cda`) |
| Review guidance requires the visual verdict and persona lenses | `a6c9d0f9` | `assets/instructions/review.md` |

**Promised-architecture cross-check** — every component design.md named
exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `VisualContract`, `parse_visual_contract`, `scenario_slug`, `collect_e2e_scenario_titles`, `check_visual_review`, `missing_visual_evidence` | Yes | `src/integrity_hook.rs` |
| `tests/steps/visual_review_gate_steps.rs` | Yes | fixture steps for the gate |
| `tests/steps/visual_design_contract_steps.rs` | Yes | ejected design-template step |
| Instruction, template, and skill assets | Yes | `assets/instructions/{specs,design,design-review,review}.md`, `assets/templates/{design,review}.md`, `assets/skills/{givn-design-review,givn-steps,givn-review}/` |
| Arc42 chapters and glossary | Yes | `docs/arc42/{05,06,08,09,10,11,12}` |

**Strict-mode proof** — present in tasks.md setup with non-zero exit:
```
1 feature / 1 scenario (1 failed) / 1 step (1 failed) / exit status: 1
```

**Downgraded e2e scenario scan** — every `@e2e` scenario's Then steps assert
on the real CLI interface (stdout of `givn instructions`, generated skill
files from `givn skills sync`, exit status and stderr of
`givn check review`). No scenario asserts only on a repository/database, and
no browser capability exists in this change:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| Canonical E2E policy is emitted by specs instructions | Yes — `givn instructions specs` stdout | N/A (CLI) | No |
| Author requests the visual design contract guidance | Yes — `givn instructions design` stdout | N/A (CLI) | No |
| Synchronized guidance defines the contract approval and capture mechanics | Yes — generated skill files | N/A (CLI) | No |
| A visual change with complete evidence passes review | Yes — `givn check review` exit 0 | N/A (CLI) | No |
| Review guidance requires the visual verdict and persona lenses | Yes — `givn instructions review` stdout | N/A (CLI) | No |

**E2e scope check** — one `@e2e` scenario per distinct action; the failure
variants (missing verdict, incomplete/empty evidence, exemption, unverified,
terminal) are regular scenarios:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| prose-consolidation | 1 (modified) | Yes — request canonical E2E guidance | No |
| visual-design-contract | 2 | Yes — request design guidance; read synchronized guidance | No |
| visual-review-gate | 2 | Yes — run the review check; read review guidance | No |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`, which runs `cargo nextest run --test
features_runner` with `GIVN_CUCUMBER_TAGS=@e2e`. The only `@e2e` step
implementations are the shared `tests/steps/` modules; `git status` shows no
untracked parallel e2e implementation.

| Capability | File(s) verify.e2e_command invokes | Other implementation found? (incl. untracked) | Stronger one used instead? |
|---|---|---|---|
| all three | `tests/features_runner.rs` + `tests/steps/` | No | N/A |

**Local environment check** — design.md's Local Runnability names
`./run-tests.sh`; givn is a CLI with no external dependency or digital twin:

```
./run-tests.sh -> Summary [24.374s] 415 tests run: 415 passed, 0 skipped
./run-tests.sh --e2e -> Summary [4.686s] 79 tests run: 79 passed, 336 skipped
```

**E2e isolation check** — `verify.command` and `verify.e2e_command` read
literally from `givn/commands.yaml`; they differ and the e2e scope is
strictly smaller. No archive receipt exists yet (candidate not archived), so
the counts below are the implementation runs; the archive will record the
per-scope receipt:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 415 | 79 | Yes |

**Design.md conformance check** — implementation matches the reviewed design
(integrity hook, step files, asset surfaces, CLI driver). One scenario was
added during implementation to cover the already-promised
`assets/skills/givn-review/SKILL.md.tmpl` change; design.md's Architecture
Impact already names that asset, so no design change was needed:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Gate location | `src/integrity_hook.rs`, wired into `run_integrity_hook` | `check_visual_review` called from `run_integrity_hook` | Yes | N/A |
| Step files | `visual_review_gate_steps.rs`, `visual_design_contract_steps.rs`, reused prose steps | Same | Yes | N/A |
| Contract format and paths | `## Visual Design Contract`, `evidence/visual/<slug>/<viewport>.png` | Same | Yes | N/A |
| Runner | `./run-tests.sh` / `./run-tests.sh --e2e` | Same | Yes | N/A |

**Interaction coverage verification** — every touched inventory entry maps
to a design matrix row, an existing `@e2e` scenario, and step definitions
using the promised CLI driving mechanism. The untouched
`Generated guidance points to the canonical E2E policy` entry keeps its
permanent `@e2e` scenario and is not part of this delta:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| request canonical E2E guidance (prose-consolidation) | Yes | Canonical E2E policy is emitted by specs instructions | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| inspect generated E2E guidance (prose-consolidation) | Permanent scenario, unchanged | Generated guidance points to the canonical E2E policy (permanent) | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| request the design guidance for a visual interface | Yes | Author requests the visual design contract guidance | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| read the synchronized design and step guidance | Yes | Synchronized guidance defines the contract approval and capture mechanics | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| run the review check on a visual change | Yes | A visual change with complete evidence passes review | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |
| read the visual review guidance | Yes | Review guidance requires the visual verdict and persona lenses | CLI `givn` subprocess | `common::run_givn` subprocess | Yes |

**Coverage measurement validity** — coverage is measured once by the archive
against the committed candidate; no receipt exists yet, so this review
classifies from static inspection and the implementation evidence. The
configured `measure_command`/`merge_command` are unchanged by this change;
the archive will validate them. New production code
(`src/integrity_hook.rs`) is covered by 17 lib unit tests and the gate
scenarios. Result: no coverage measurement to reject at review time; no
runner-only or test-only claim is made here.

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `author-change` guidance stays synchronized from assets; `verify-change` still blocks an invalid change | Yes |
| Capability ownership and typed relationships preserved | `prose-consolidation` extended in `author-change`; two new capabilities owned by their declared use cases; both include `fragment: support` | Yes |
| Confirmed Persona-relevant outcomes preserved | Both use cases declare `Personas: none`; no Persona was invented or promoted | Yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay `Developer`/`Reviewer`/`Authoring agent`; Personas are review lenses only | Yes |
| Migration ledger and coverage evidence complete, if applicable | Not a migration change | N/A |

## Retirements

N/A if this change retires nothing.

N/A — no capability is retired.

## Arc42 implementation conformance

The completed implementation matches the change-level assessment. Chapter 09
records the ADR-028 amendment; chapter 11 records its consequences.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| Integrity hook owns the visual verdict and evidence check | `docs/arc42/05-building-block-view.md` | Affected | `check_visual_review` in `src/integrity_hook.rs` | Gate scenarios | `check_visual_review` wired into `run_integrity_hook` | Yes |
| `givn check review` runs the visual gate | `docs/arc42/06-runtime-view.md` | Affected | Gate inside the integrity hook | Gate scenarios | `givn check review` exits non-zero for a declared contract without verdict/evidence | Yes |
| Visual review discipline | `docs/arc42/08-crosscutting-concepts.md` | Affected | Contract format, evidence path, verdict rules | All gate scenarios | Section 8.17 matches the built format and paths | Yes |
| ADR-028 amendment | `docs/arc42/09-architecture-decisions.md` | Affected | Gate is a refinement of the mandatory-check boundary | `arc42.md` hardening | Chapter 09 paragraph added; no new ADR | Yes |
| Gate-safety quality scenarios | `docs/arc42/10-quality-requirements.md` | Affected | Q-85 to Q-87 | Gate scenarios | Q-85..Q-87 match the built behavior | Yes |
| Risks and guidance-level debt | `docs/arc42/11-risks-and-technical-debt.md` | Affected | R-56..R-59, D-18 | — | R-56..R-59 and D-18 match the built boundaries | Yes |
| Visual terms and invariants | `docs/arc42/12-glossary.md` | Affected | Terms used in code and specs | — | `VisualContract`, `VisualInterface`, `scenario_slug` reuse the glossary terms | Yes |

- [x] Every chapter marked affected by either the independent review or
      `arc42.md` was opened and checked for stale, contradictory, or placeholder
      content.
- [x] Every Arc42 fact or architecture decision relevant to this change is
      represented in `design.md` and mapped to one or more tasks in `tasks.md`.
- [x] The completed implementation follows those facts and decisions; evidence
      names the built file, command, test, or observable result.
- [x] Architecture changes discovered in the completed implementation are
      recorded in the durable Arc42 chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The durable glossary holds every new term (Visual interface, Visual Design
Contract, Design system, Visual evidence, Visual baseline, Visual verdict,
Style tile) and the visual-review invariants. The spec, design, and code
reuse them: the gate parses `## Visual Design Contract` with an `Interface:`
line, the verdict markers are exactly `VISUAL-REVIEW: PASS` and
`VISUAL-REVIEW: UNVERIFIED`, and the evidence path is
`evidence/visual/<scenario-slug>/...`. No inconsistent reuse found.

## Deterministic gate dispositions

`givn check review --change visual-ui-review` reports 189 deterministic
shape-match findings between this delta and the permanent corpus. Every
finding is dispositioned below.

Classification rule: `variant` when the other scenario also exercises the
`givn check review` gate with a different rule or outcome; `boundary` when
the pair shares only the generic Given/When/Then shape across different
actions or distinct exemption boundaries. No pair is a `duplicate`: each
scenario asserts a distinct contract rule, verdict, or evidence boundary.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| A change without a visual contract is exempt from the visual gate | A change that declares no visual interface is exempt | boundary |
| A change without a visual contract is exempt from the visual gate | A terminal change passes review with a transcript | boundary |
| A disabled addon needs no impact decision | A change that declares no visual interface is exempt | variant |
| A disabled addon needs no impact decision | A change without a visual contract is exempt from the visual gate | variant |
| A disabled addon needs no impact decision | A terminal change passes review with a transcript | variant |
| A disabled addon needs no impact decision | A visual change with complete evidence passes review | variant |
| A malformed permanent feature reports its path and fails the audit | A visual change without a visual verdict fails review | boundary |
| A malformed permanent feature reports its path and fails the audit | A web contract without viewports fails review | boundary |
| A malformed permanent feature reports its path and fails the audit | An empty evidence file fails review | boundary |
| A malformed permanent feature reports its path and fails the audit | An unverified visual verdict cannot pass review | boundary |
| A modified delta matching its merge target in the same capability is allowed | A change that declares no visual interface is exempt | boundary |
| A modified delta matching its merge target in the same capability is allowed | A change without a visual contract is exempt from the visual gate | boundary |
| A modified delta matching its merge target in the same capability is allowed | A terminal change passes review with a transcript | boundary |
| A modified delta matching its merge target in the same capability is allowed | A visual change with complete evidence passes review | boundary |
| A recorded impact decision passes the review | A change that declares no visual interface is exempt | variant |
| A recorded impact decision passes the review | A change without a visual contract is exempt from the visual gate | variant |
| A recorded impact decision passes the review | A terminal change passes review with a transcript | variant |
| A recorded impact decision passes the review | A visual change with complete evidence passes review | variant |
| A removed delta referencing its target by title is allowed | A change that declares no visual interface is exempt | boundary |
| A removed delta referencing its target by title is allowed | A change without a visual contract is exempt from the visual gate | boundary |
| A removed delta referencing its target by title is allowed | A terminal change passes review with a transcript | boundary |
| A removed delta referencing its target by title is allowed | A visual change with complete evidence passes review | boundary |
| A terminal change passes review with a transcript | A change that declares no visual interface is exempt | boundary |
| A visual change with complete evidence passes review | A change that declares no visual interface is exempt | boundary |
| A visual change with complete evidence passes review | A change without a visual contract is exempt from the visual gate | boundary |
| A visual change with complete evidence passes review | A terminal change passes review with a transcript | boundary |
| A visual change without a visual verdict fails review | A web contract without viewports fails review | boundary |
| A visual change without a visual verdict fails review | An empty evidence file fails review | boundary |
| A visual change without a visual verdict fails review | An unverified visual verdict cannot pass review | boundary |
| A web contract without viewports fails review | An empty evidence file fails review | boundary |
| An unverified visual verdict cannot pass review | A web contract without viewports fails review | boundary |
| An unverified visual verdict cannot pass review | An empty evidence file fails review | boundary |
| Archive blocked when delta spec has non-removed @wip scenario | A visual change without a visual verdict fails review | variant |
| Archive blocked when delta spec has non-removed @wip scenario | A web contract without viewports fails review | variant |
| Archive blocked when delta spec has non-removed @wip scenario | An empty evidence file fails review | variant |
| Archive blocked when delta spec has non-removed @wip scenario | An unverified visual verdict cannot pass review | variant |
| Canonical policy uses transcripts for terminal interfaces | Design guidance keeps improvements inside the style | boundary |
| Canonical policy uses transcripts for terminal interfaces | Review guidance records an unverified verdict without vision | boundary |
| Check tasks blocked when delta spec has @wip | A visual change without a visual verdict fails review | variant |
| Check tasks blocked when delta spec has @wip | A web contract without viewports fails review | variant |
| Check tasks blocked when delta spec has @wip | An empty evidence file fails review | variant |
| Check tasks blocked when delta spec has @wip | An unverified visual verdict cannot pass review | variant |
| Design guidance keeps improvements inside the style | Review guidance records an unverified verdict without vision | boundary |
| Design guidance names the default viewports | Design-review guidance requires an approved contract | boundary |
| Design guidance names the default viewports | Review guidance requires the visual verdict and persona lenses | boundary |
| Design-review guidance requires an approved contract | Review guidance requires the visual verdict and persona lenses | boundary |
| Detail mode — unknown change errors | A visual change without a visual verdict fails review | boundary |
| Detail mode — unknown change errors | A web contract without viewports fails review | boundary |
| Detail mode — unknown change errors | An empty evidence file fails review | boundary |
| Detail mode — unknown change errors | An unverified visual verdict cannot pass review | boundary |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | A visual change without a visual verdict fails review | boundary |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | A web contract without viewports fails review | boundary |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | An empty evidence file fails review | boundary |
| Id beginning with a hyphen passed after the argument separator is rejected by validation | An unverified visual verdict cannot pass review | boundary |
| Id ending with a hyphen is rejected | A visual change without a visual verdict fails review | boundary |
| Id ending with a hyphen is rejected | A web contract without viewports fails review | boundary |
| Id ending with a hyphen is rejected | An empty evidence file fails review | boundary |
| Id ending with a hyphen is rejected | An unverified visual verdict cannot pass review | boundary |
| Id with a dot is rejected | A visual change without a visual verdict fails review | boundary |
| Id with a dot is rejected | A web contract without viewports fails review | boundary |
| Id with a dot is rejected | An empty evidence file fails review | boundary |
| Id with a dot is rejected | An unverified visual verdict cannot pass review | boundary |
| Id with non-ASCII letters is rejected | A visual change without a visual verdict fails review | boundary |
| Id with non-ASCII letters is rejected | A web contract without viewports fails review | boundary |
| Id with non-ASCII letters is rejected | An empty evidence file fails review | boundary |
| Id with non-ASCII letters is rejected | An unverified visual verdict cannot pass review | boundary |
| Id with spaces is rejected | A visual change without a visual verdict fails review | boundary |
| Id with spaces is rejected | A web contract without viewports fails review | boundary |
| Id with spaces is rejected | An empty evidence file fails review | boundary |
| Id with spaces is rejected | An unverified visual verdict cannot pass review | boundary |
| Integrity hook fails on Step Definitions section that is not a table | A visual change without a visual verdict fails review | variant |
| Integrity hook fails on Step Definitions section that is not a table | A web contract without viewports fails review | variant |
| Integrity hook fails on Step Definitions section that is not a table | An empty evidence file fails review | variant |
| Integrity hook fails on Step Definitions section that is not a table | An unverified visual verdict cannot pass review | variant |
| Integrity hook fails on empty step-definition file | A visual change with incomplete evidence fails review | variant |
| Integrity hook fails on missing step-definition file | A visual change with incomplete evidence fails review | variant |
| Integrity hook fails on placeholder commit hash | A visual change with incomplete evidence fails review | variant |
| Integrity hook fails on unedited arc42 chapter | A visual change with incomplete evidence fails review | variant |
| Integrity hook passes on edited arc42 chapter | A change that declares no visual interface is exempt | variant |
| Integrity hook passes on edited arc42 chapter | A change without a visual contract is exempt from the visual gate | variant |
| Integrity hook passes on edited arc42 chapter | A terminal change passes review with a transcript | variant |
| Integrity hook passes on edited arc42 chapter | A visual change with complete evidence passes review | variant |
| Integrity hook passes when arc42 is disabled | A change that declares no visual interface is exempt | variant |
| Integrity hook passes when arc42 is disabled | A change without a visual contract is exempt from the visual gate | variant |
| Integrity hook passes when arc42 is disabled | A terminal change passes review with a transcript | variant |
| Integrity hook passes when arc42 is disabled | A visual change with complete evidence passes review | variant |
| Lost E2E evidence blocks migration coverage | A visual change without a visual verdict fails review | variant |
| Lost E2E evidence blocks migration coverage | A web contract without viewports fails review | variant |
| Lost E2E evidence blocks migration coverage | An empty evidence file fails review | variant |
| Lost E2E evidence blocks migration coverage | An unverified visual verdict cannot pass review | variant |
| Lower branch coverage blocks migration coverage | A visual change without a visual verdict fails review | variant |
| Lower branch coverage blocks migration coverage | A web contract without viewports fails review | variant |
| Lower branch coverage blocks migration coverage | An empty evidence file fails review | variant |
| Lower branch coverage blocks migration coverage | An unverified visual verdict cannot pass review | variant |
| Lower interaction coverage blocks migration coverage | A visual change without a visual verdict fails review | variant |
| Lower interaction coverage blocks migration coverage | A web contract without viewports fails review | variant |
| Lower interaction coverage blocks migration coverage | An empty evidence file fails review | variant |
| Lower interaction coverage blocks migration coverage | An unverified visual verdict cannot pass review | variant |
| Lower line coverage blocks migration coverage | A visual change without a visual verdict fails review | variant |
| Lower line coverage blocks migration coverage | A web contract without viewports fails review | variant |
| Lower line coverage blocks migration coverage | An empty evidence file fails review | variant |
| Lower line coverage blocks migration coverage | An unverified visual verdict cannot pass review | variant |
| Missing impact decision refuses the review | A visual change without a visual verdict fails review | variant |
| Missing impact decision refuses the review | A web contract without viewports fails review | variant |
| Missing impact decision refuses the review | An empty evidence file fails review | variant |
| Missing impact decision refuses the review | An unverified visual verdict cannot pass review | variant |
| Path traversal id is rejected by archive | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by archive | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by archive | An empty evidence file fails review | boundary |
| Path traversal id is rejected by archive | An unverified visual verdict cannot pass review | boundary |
| Path traversal id is rejected by check | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by check | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by check | An empty evidence file fails review | boundary |
| Path traversal id is rejected by check | An unverified visual verdict cannot pass review | boundary |
| Path traversal id is rejected by graph | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by graph | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by graph | An empty evidence file fails review | boundary |
| Path traversal id is rejected by graph | An unverified visual verdict cannot pass review | boundary |
| Path traversal id is rejected by instructions | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by instructions | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by instructions | An empty evidence file fails review | boundary |
| Path traversal id is rejected by instructions | An unverified visual verdict cannot pass review | boundary |
| Path traversal id is rejected by lint | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by lint | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by lint | An empty evidence file fails review | boundary |
| Path traversal id is rejected by lint | An unverified visual verdict cannot pass review | boundary |
| Path traversal id is rejected by status | A visual change without a visual verdict fails review | boundary |
| Path traversal id is rejected by status | A web contract without viewports fails review | boundary |
| Path traversal id is rejected by status | An empty evidence file fails review | boundary |
| Path traversal id is rejected by status | An unverified visual verdict cannot pass review | boundary |
| Review blocks when a delta scenario shape-matches a permanent scenario without disposition | A visual change with incomplete evidence fails review | variant |
| Review blocks when a long scenario in the delta has no split-or-keep entry | A visual change with incomplete evidence fails review | variant |
| Review blocks with a clear message when review.md is missing | A visual change without a visual verdict fails review | variant |
| Review blocks with a clear message when review.md is missing | A web contract without viewports fails review | variant |
| Review blocks with a clear message when review.md is missing | An empty evidence file fails review | variant |
| Review blocks with a clear message when review.md is missing | An unverified visual verdict cannot pass review | variant |
| Review instructions require deterministic gate dispositions | Review guidance treats style improvements as contract updates | boundary |
| Review passes when a long scenario has a split-or-keep entry | A change that declares no visual interface is exempt | variant |
| Review passes when a long scenario has a split-or-keep entry | A change without a visual contract is exempt from the visual gate | variant |
| Review passes when a long scenario has a split-or-keep entry | A terminal change passes review with a transcript | variant |
| Review passes when a long scenario has a split-or-keep entry | A visual change with complete evidence passes review | variant |
| Review passes when a shape match exists only between permanent scenarios | A change that declares no visual interface is exempt | variant |
| Review passes when a shape match exists only between permanent scenarios | A change without a visual contract is exempt from the visual gate | variant |
| Review passes when a shape match exists only between permanent scenarios | A terminal change passes review with a transcript | variant |
| Review passes when a shape match exists only between permanent scenarios | A visual change with complete evidence passes review | variant |
| Routing requires exactly one input | A visual change without a visual verdict fails review | boundary |
| Routing requires exactly one input | A web contract without viewports fails review | boundary |
| Routing requires exactly one input | An empty evidence file fails review | boundary |
| Routing requires exactly one input | An unverified visual verdict cannot pass review | boundary |
| The check passes on a complete README | A change that declares no visual interface is exempt | variant |
| The check passes on a complete README | A change without a visual contract is exempt from the visual gate | variant |
| The check passes on a complete README | A terminal change passes review with a transcript | variant |
| The check passes on a complete README | A visual change with complete evidence passes review | variant |
| The default scope excludes the archive tree | A change that declares no visual interface is exempt | boundary |
| The default scope excludes the archive tree | A change without a visual contract is exempt from the visual gate | boundary |
| The default scope excludes the archive tree | A terminal change passes review with a transcript | boundary |
| The default scope excludes the archive tree | A visual change with complete evidence passes review | boundary |
| Uninitialized project errors | A visual change without a visual verdict fails review | boundary |
| Uninitialized project errors | A web contract without viewports fails review | boundary |
| Uninitialized project errors | An empty evidence file fails review | boundary |
| Uninitialized project errors | An unverified visual verdict cannot pass review | boundary |
| Updated impact decision passes the review | A change that declares no visual interface is exempt | variant |
| Updated impact decision passes the review | A change without a visual contract is exempt from the visual gate | variant |
| Updated impact decision passes the review | A terminal change passes review with a transcript | variant |
| Updated impact decision passes the review | A visual change with complete evidence passes review | variant |
| Valid kebab-case change id is accepted by every command that accepts --change | A change that declares no visual interface is exempt | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A change without a visual contract is exempt from the visual gate | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A terminal change passes review with a transcript | boundary |
| Valid kebab-case change id is accepted by every command that accepts --change | A visual change with complete evidence passes review | boundary |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A change that declares no visual interface is exempt | variant |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A change without a visual contract is exempt from the visual gate | variant |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A terminal change passes review with a transcript | variant |
| Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | A visual change with complete evidence passes review | variant |
| givn addons disable errors when givn/ is not initialised | A visual change without a visual verdict fails review | boundary |
| givn addons disable errors when givn/ is not initialised | A web contract without viewports fails review | boundary |
| givn addons disable errors when givn/ is not initialised | An empty evidence file fails review | boundary |
| givn addons disable errors when givn/ is not initialised | An unverified visual verdict cannot pass review | boundary |
| givn addons disable when already disabled is idempotent | A change that declares no visual interface is exempt | boundary |
| givn addons disable when already disabled is idempotent | A change without a visual contract is exempt from the visual gate | boundary |
| givn addons disable when already disabled is idempotent | A terminal change passes review with a transcript | boundary |
| givn addons disable when already disabled is idempotent | A visual change with complete evidence passes review | boundary |
| givn addons enable errors when givn/ is not initialised | A visual change without a visual verdict fails review | boundary |
| givn addons enable errors when givn/ is not initialised | A web contract without viewports fails review | boundary |
| givn addons enable errors when givn/ is not initialised | An empty evidence file fails review | boundary |
| givn addons enable errors when givn/ is not initialised | An unverified visual verdict cannot pass review | boundary |
| givn check with known artifact proceeds to run hooks | A change that declares no visual interface is exempt | boundary |
| givn check with known artifact proceeds to run hooks | A change without a visual contract is exempt from the visual gate | boundary |
| givn check with known artifact proceeds to run hooks | A terminal change passes review with a transcript | boundary |
| givn check with known artifact proceeds to run hooks | A visual change with complete evidence passes review | boundary |

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold.

## README impact decision (readme addon enabled)

README-IMPACT: updated - Description

The change adds a consumer-observable gate to the workflow. The Description
section's "Blocking gates" bullet now names the visual review gate and the
verdict/evidence requirement.

## Coverage

Coverage is measured once by the archive against the committed candidate and
recorded in the verification receipt. No receipt exists yet for this change.

**Coverage result:** not yet measured — unproven candidate; the archive
receipt will carry the merged line/branch result.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/integrity_hook.rs` (new parser, verdict, evidence check) | new | Covered by 17 lib unit tests and 13 gate scenarios | None needed |
| `tests/steps/visual_review_gate_steps.rs` | test surface | Covered by the scenarios that use it | None needed |
| (no uncovered production region identified) | — | — | — |

No dead code was introduced. No missing test coverage was found. No
hard-to-test region was found.

## Scenario execution evidence

The change is not archived yet, so no verification receipt exists.

```
unproven candidate — no receipt yet. Implementation runs:
delta suite 23/23 passed; permanent suite 415/415; permanent e2e 79/79.
```

**Receipt status:** unproven

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no e2e
      scenario driven via HTTP/fetch() in place of real browser interaction,
      no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the full stack, including digital twins,
      cleanly.
- [x] The exact file(s) `verify.e2e_command` invokes were identified, and
      the tree (including untracked files) was searched for a parallel e2e
      implementation before accepting a weaker one.
- [x] `verify.e2e_command` is not identical to `verify.command`; scenario
      counts prove real isolation.
- [x] Implementation (commands, file layout, framework/driver) matches
      design.md as reviewed.
- [x] Interaction coverage verified: every spec inventory entry maps to a
      design matrix row, an existing `@e2e` scenario, and step definitions
      using the promised driving mechanism — no mismatches.
- [x] Design.md's `## Internal Primitives` section accurately lists every
      internal data structure and abstraction this change introduces.
- [x] arc42 slug-mapping verified.
- [x] No finding anywhere in this report is excused with a classification
      outside the three buckets.

**REVIEW: PASS**
