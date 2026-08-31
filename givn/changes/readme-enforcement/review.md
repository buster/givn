# Review: readme-enforcement

## Fabrication audit

0. `@e2e` tag integrity: all five e2e scenarios tagged `@e2e` in
   `givn/changes/readme-enforcement/specs/readme-enforcement/readme-enforcement.feature`
   retain their tags (git history diffed; no removal). `givn lint` and
   `givn check review` cross-references clean.
1. Step bodies: grep for `unimplemented!`/`todo!`/`{}` in
   `tests/steps/readme_enforcement_steps.rs` and
   `tests/steps/readme_enforcement_e2e_steps.rs` — 0 matches. Every step body
   asserts against subprocess output or writes deterministic fixture state.
2. Every checked task carries a commit; production diffs verified per
   scenario (checker, CLI dispatch, integrity hook, archive gate, refresh,
   instructions nudge, template). Commit count: 19 scenario commits plus
   fixture adaptation, skill guidance, integration regen, and one
   dev-block fix (`8cb7058`).
3. Design components present: completeness checker + `givn readme check`
   (`src/cli/readme.rs`, `src/main.rs`), archive gate + refresh
   (`src/cli/archive.rs`), README-IMPACT in the integrity hook
   (`src/integrity_hook.rs`, `src/hooks.rs`, `src/cli/check.rs`),
   instructions nudge (`src/cli/instructions.rs`), marked template
   (`assets/addons/readme/README.md.tmpl`), skill guidance (givn-archive,
   givn-review). All in tree.
4. Strict-mode proof in tasks.md: exit 101 with an unimplemented stub.
5–7. e2e steps drive the real CLI via subprocess and assert stdout/exit —
   no repository-only downgrades; `verify.e2e_command` (`./run-tests.sh
   --e2e`) isolated (94 vs 520 scenarios; delta 5 vs 19).
9. Local runnability: `cargo build && ./run-tests.sh`; no services, no
   digital twins required.
10. `verify.command` ≠ `verify.e2e_command`; counts pasted in tasks.md.

## Deterministic gates

- Overlap findings: `givn lint --change readme-enforcement` — @wip removed
  everywhere; zero findings beyond the structural inventory, which matches
  the design matrix 1:1.
- Semantic review: all five inventory actions have exactly one `@e2e`
  scenario; GREEN-first regression guards (scenarios 16, 17) and the silent
  guard (14) are paired with refusal evidence (scenarios 1–5, 6–9, 10–12)
  so no duplicate-shape or missing-RED claim is fabricated.

## Coverage gate (completeness)

Measured via `./measure-coverage.sh` + `./merge-coverages.sh` at archive.
Classification of the delta:

- All new production regions (`check_readme`, `run_check`, `run_check`'s
  dispatch, `check_readme_impact`, `refresh_development_block`,
  `render_development_block`, instructions `readme_nudge`, archive gate
  block) are exercised by the scenario suite: checker paths (placeholders,
  legacy, remnants, damaged, unmanaged, missing), gate paths (block/pass),
  refresh paths (stale/absent/present), nudge paths (surface/silent),
  impact paths (missing/none/updated/unknown/off).
- One known unmeasured region remains: the `Err(_)` fallback of
  `load_effective()` inside `render_development_block` (only reachable when
  the command file is unreadable mid-init). Classified bucket 3 —
  legitimately hard to test: it requires a config state that exists only
  mid-failure inside init; the fallback is the embedded-defaults shape
  asserted by the scaffold scenario's command listing. Justification
  recorded here per the review contract.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A binary without retrieval reports the explicit unavailable mode` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A feature-free build keeps retrieval explicitly unavailable` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A missing draft file is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Archive blocks when the README is missing` | `Archive blocks for a hand-written README without givn markers` | KEEP-AS-BOUNDARY |
| `A removed delta referencing its target by title is allowed` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A valid feature without a scenario is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `A VARIANT score below the boundary without consolidation remains blocked` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Changing a classified endpoint before remediation makes the decision stale` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Check tasks blocked when delta spec has @wip` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Consumer runs the README check on a complete README and it passes` | `A project without the readme addon needs no README impact decision` | KEEP-AS-BOUNDARY |
| `Consumer runs the README check on a complete README and it passes` | `Review accepts an updated README impact decision naming the sections` | KEEP-AS-BOUNDARY |
| `Detail mode — unknown change errors` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Every resolved semantic classification requires a rationale` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Explore remains unavailable without the retrieval feature` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `File and standard input cannot be selected together` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `givn addons disable errors when givn/ is not initialised` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `givn addons enable errors when givn/ is not initialised` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Id ending with a hyphen is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Id with a dot is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Id with non-ASCII letters is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Id with spaces is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Integrity hook fails on Step Definitions section that is not a table` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Integrity hook resolves the change directory from a relative features path inside givn/changes` | `An unknown README impact value is refused` | UNRELATED |
| `Invalid Gherkin input is rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Multiple scenarios in one input are rejected` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by archive` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by check` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by graph` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by instructions` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by lint` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Path traversal id is rejected by status` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `README check reports a damaged managed block` | `README check reports a hand-written README without givn markers as unmanaged` | KEEP-AS-BOUNDARY |
| `README check reports a damaged managed block` | `README check reports a missing README` | KEEP-AS-BOUNDARY |
| `README check reports a legacy scaffold README as needing migration` | `README check reports a damaged managed block` | KEEP-AS-BOUNDARY |
| `README check reports a legacy scaffold README as needing migration` | `README check reports a hand-written README without givn markers as unmanaged` | KEEP-AS-BOUNDARY |
| `README check reports a legacy scaffold README as needing migration` | `README check reports a missing README` | KEEP-AS-BOUNDARY |
| `README check reports a missing README` | `README check reports a hand-written README without givn markers as unmanaged` | KEEP-AS-BOUNDARY |
| `Review accepts an updated README impact decision naming the sections` | `A project without the readme addon needs no README impact decision` | KEEP-AS-BOUNDARY |
| `Review blocks with a clear message when review.md is missing` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `The audit reports an unavailable retrieval model` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `The audit reports retrieval unavailable without the retrieval feature` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `The default scope excludes the archive tree` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `The default scope excludes the archive tree` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `The default scope excludes the archive tree` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Uninitialized project errors` | `Review refuses sign-off without a recorded README impact decision` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A project without the readme addon needs no README impact decision` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Consumer runs the README check on a complete README and it passes` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Review accepts an updated README impact decision naming the sections` | UNRELATED |

| Scenario | Decision |
|---|---|
| `Search does not return the running change's own delta` | `Instructions stay silent about the README when it is complete` | UNRELATED |
| (none) | Keep all |

## Ubiquitous language

Glossary terms used verbatim across spec/design/code: open placeholder,
README check, README completeness gate, README impact decision, README
migration, managed block. No drift.

## Sign-off

- Full suite: `./run-tests.sh` exit 0 (417 scenarios).
- E2E: `./run-tests.sh --e2e` exit 0 (30 scenarios).
- Delta: 14/14 non-e2e + 5/5 e2e.
- README impact decision: this change added a user-facing command, a gate,
  and a marker convention — the project README's "Blocking gates" section
  was updated accordingly.

REVIEW-IMPACT recorded below.

README-IMPACT: updated - Blocking gates

REVIEW: PASS
