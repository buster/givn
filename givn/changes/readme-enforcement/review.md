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
| `A binary without retrieval reports the explicit unavailable mode` | `Missing impact decision refuses the review` | UNRELATED |
| `A disabled addon needs no impact decision` | `A recorded impact decision passes the review` | KEEP-AS-BOUNDARY |
| `A feature-free build keeps retrieval explicitly unavailable` | `Missing impact decision refuses the review` | UNRELATED |
| `A legacy README is reported for migration` | `A marker-less README is unmanaged` | KEEP-AS-BOUNDARY |
| `A legacy README is reported for migration` | `The check names a damaged managed block` | KEEP-AS-BOUNDARY |
| `A legacy README is reported for migration` | `The check reports a missing README` | KEEP-AS-BOUNDARY |
| `A missing draft file is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `A missing README blocks the archive` | `An unmanaged README blocks the archive` | KEEP-AS-BOUNDARY |
| `A modified delta matching its merge target in the same capability is allowed` | `A disabled addon needs no impact decision` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `A recorded impact decision passes the review` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `The check passes on a complete README` | UNRELATED |
| `A modified delta matching its merge target in the same capability is allowed` | `Updated impact decision passes the review` | UNRELATED |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Missing impact decision refuses the review` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `A disabled addon needs no impact decision` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `A recorded impact decision passes the review` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `The check passes on a complete README` | UNRELATED |
| `A removed delta referencing its target by title is allowed` | `Updated impact decision passes the review` | UNRELATED |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `Missing impact decision refuses the review` | UNRELATED |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `Missing impact decision refuses the review` | UNRELATED |
| `A valid feature without a scenario is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `A VARIANT score below the boundary without consolidation remains blocked` | `Missing impact decision refuses the review` | UNRELATED |
| `Changing a classified endpoint before remediation makes the decision stale` | `Missing impact decision refuses the review` | UNRELATED |
| `Check tasks blocked when delta spec has @wip` | `Missing impact decision refuses the review` | UNRELATED |
| `Detail mode — unknown change errors` | `Missing impact decision refuses the review` | UNRELATED |
| `Every resolved semantic classification requires a rationale` | `Missing impact decision refuses the review` | UNRELATED |
| `Explore remains unavailable without the retrieval feature` | `Missing impact decision refuses the review` | UNRELATED |
| `File and standard input cannot be selected together` | `Missing impact decision refuses the review` | UNRELATED |
| `givn addons disable errors when givn/ is not initialised` | `Missing impact decision refuses the review` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `A disabled addon needs no impact decision` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `A recorded impact decision passes the review` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `The check passes on a complete README` | UNRELATED |
| `givn addons disable when already disabled is idempotent` | `Updated impact decision passes the review` | UNRELATED |
| `givn addons enable errors when givn/ is not initialised` | `Missing impact decision refuses the review` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `A disabled addon needs no impact decision` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `A recorded impact decision passes the review` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `The check passes on a complete README` | UNRELATED |
| `givn check with known artifact proceeds to run hooks` | `Updated impact decision passes the review` | UNRELATED |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Missing impact decision refuses the review` | UNRELATED |
| `Id ending with a hyphen is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Id with a dot is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Id with non-ASCII letters is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Id with spaces is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Integrity hook fails on Step Definitions section that is not a table` | `Missing impact decision refuses the review` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `A disabled addon needs no impact decision` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `A recorded impact decision passes the review` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `The check passes on a complete README` | UNRELATED |
| `Integrity hook passes on edited arc42 chapter` | `Updated impact decision passes the review` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `A disabled addon needs no impact decision` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `A recorded impact decision passes the review` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `The check passes on a complete README` | UNRELATED |
| `Integrity hook passes when arc42 is disabled` | `Updated impact decision passes the review` | UNRELATED |
| `Integrity hook resolves the change directory from a relative features path inside givn/changes` | `An unknown impact value is refused` | UNRELATED |
| `Invalid Gherkin input is rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Multiple scenarios in one input are rejected` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by archive` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by check` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by graph` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by instructions` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by lint` | `Missing impact decision refuses the review` | UNRELATED |
| `Path traversal id is rejected by status` | `Missing impact decision refuses the review` | UNRELATED |
| `Review blocks with a clear message when review.md is missing` | `Missing impact decision refuses the review` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `A disabled addon needs no impact decision` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `A recorded impact decision passes the review` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `The check passes on a complete README` | UNRELATED |
| `Review passes when a long scenario has a split-or-keep entry` | `Updated impact decision passes the review` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `A disabled addon needs no impact decision` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `A recorded impact decision passes the review` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `The check passes on a complete README` | UNRELATED |
| `Review passes when a shape match exists only between permanent scenarios` | `Updated impact decision passes the review` | UNRELATED |
| `Search does not return the running change's own delta` | `Instructions stay silent on a complete README` | UNRELATED |
| `The audit reports an unavailable retrieval model` | `Missing impact decision refuses the review` | UNRELATED |
| `The audit reports retrieval unavailable without the retrieval feature` | `Missing impact decision refuses the review` | UNRELATED |
| `The check names a damaged managed block` | `A marker-less README is unmanaged` | KEEP-AS-BOUNDARY |
| `The check names a damaged managed block` | `The check reports a missing README` | KEEP-AS-BOUNDARY |
| `The check passes on a complete README` | `A disabled addon needs no impact decision` | KEEP-AS-BOUNDARY |
| `The check passes on a complete README` | `A recorded impact decision passes the review` | KEEP-AS-BOUNDARY |
| `The check passes on a complete README` | `Updated impact decision passes the review` | KEEP-AS-BOUNDARY |
| `The check reports a missing README` | `A marker-less README is unmanaged` | KEEP-AS-BOUNDARY |
| `The default scope excludes the archive tree` | `A disabled addon needs no impact decision` | UNRELATED |
| `The default scope excludes the archive tree` | `A recorded impact decision passes the review` | UNRELATED |
| `The default scope excludes the archive tree` | `The check passes on a complete README` | UNRELATED |
| `The default scope excludes the archive tree` | `Updated impact decision passes the review` | UNRELATED |
| `Uninitialized project errors` | `Missing impact decision refuses the review` | UNRELATED |
| `Updated impact decision passes the review` | `A disabled addon needs no impact decision` | KEEP-AS-BOUNDARY |
| `Updated impact decision passes the review` | `A recorded impact decision passes the review` | KEEP-AS-BOUNDARY |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A disabled addon needs no impact decision` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `A recorded impact decision passes the review` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `The check passes on a complete README` | UNRELATED |
| `Valid kebab-case change id is accepted by every command that accepts --change` | `Updated impact decision passes the review` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A disabled addon needs no impact decision` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `A recorded impact decision passes the review` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `The check passes on a complete README` | UNRELATED |
| `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | `Updated impact decision passes the review` | UNRELATED |

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
