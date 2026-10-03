# Review: keep-agent-prompt-open

## Fabrication Audit (do this first)

**Step definition scan** — the touched step file checked for empty/no-op
bodies (`{}`, bare `pass`, bare `return`, `todo!`, `unimplemented!`, no
assertion reference):

```
$ grep -nE "todo!|unimplemented!|^\s*\{\s*\}\s*$" tests/steps/init_agent_target_selection_steps.rs
(no output)

0 empty step bodies found. The generalized PTY helper
`run_init_in_terminal` (line 175) spawns the real `script -qec` session,
writes the key sequence, enforces the 60-second deadline, and stores the
transcript; the two steps (lines 227, 232) are thin key-sequence wrappers.
```

**Commit cross-check** — every checked-off `[x]` task has a commit:

| Scenario | Commit | Touches production code? |
|---|---|---|
| 1 The interactive prompt accepts the pre-marked OpenCode default | `5e1702b5` | yes — `src/cli/init.rs` (`DEFAULT_PROMPT_TARGET`, `defaults`) + PTY step |
| 2 The interactive prompt keeps asking until a target is marked | `ee00e05d` | yes — `src/cli/init.rs` (re-ask loop + stderr message) |

**Promised-architecture cross-check** — every component design.md said would
be created exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| `DEFAULT_PROMPT_TARGET` | yes | `src/cli/init.rs:21` |
| fresh `MultiSelect` with `defaults` inside a `loop` | yes | `src/cli/init.rs:192-212` |
| interactive error on stderr | yes | `src/cli/init.rs:209` |
| key-sequence PTY helper + two steps | yes | `tests/steps/init_agent_target_selection_steps.rs:175,227,232` |
| doc comment refreshed | yes | `src/cli/init.rs:174-183` |

**Strict-mode proof** — present in tasks.md's setup task: `.fail_on_skipped()`
at `tests/features_runner.rs:55,129`, plus the live proof from the previous
change (`… --name "An undefined step must fail the runner"` → exit 1).

**Downgraded e2e scenario scan** — the delta adds no `@e2e` scenario; its two
added scenarios are regular variants of the `initialize` action, driven
through the real CLI (the PTY is a real terminal). The existing e2e scenarios
are unchanged and keep their real-interface assertions.

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| (none added by this delta) | n/a | n/a | no |

**E2e scope check** — this delta adds no `@e2e` scenario; the interactive
action's inventory entry (`select agent targets`) still maps to the unchanged
`@e2e` scenario `Consumer selects agent targets during interactive init`.

| Capability | @e2e scenario count in delta | Distinct actions? | Over-produced? |
|---|---|---|---|
| initialize-a-project | 0 added / 0 removed | n/a | no |

**Duplicate-implementation check** — unchanged from the previous change:
`verify.e2e_command` (`./run-tests.sh --e2e`) runs `tests/features_runner.rs`
with the `@e2e` tag expression; no parallel e2e implementation exists for
this capability (tree search; `git status` shows no untracked step files).

**Local environment check** — CLI only; no server, database, or digital twin.
The interactive path is driven through the `script` (util-linux) PTY, now
documented in `AGENTS.md`.

**E2e isolation check** — read literally from `givn/commands.yaml`; the change
is not archived yet, so no receipt exists. Delta-scope counts measured during
implementation:

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2e scope scenario count | E2e count strictly smaller (or all scenarios @e2e)? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | no | 3 (delta scope) | 0 (delta adds no `@e2e`) | yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `specs/initialize-project/usecase.md`: main flow 1 records the pre-marked list and re-ask; the minimal guarantee holds (nothing written before a target is marked; Ctrl-C aborts) | yes |
| Capability ownership and typed relationships preserved | `initialize-a-project` extended in place; `Includes: fragment: support`, `Extends: none` untouched | yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user` sees the list stay open and the error in its own vocabulary | yes |
| Actors remain distinct from Persona biographies | Gherkin actors stay `the user` | yes |
| Migration ledger and coverage evidence complete, if applicable | not a migration change | yes |

PERSONAS: developer--primary-user

## Deferred mandatory checks

None.

**Design.md conformance check** — implementation diffed against what design.md
named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: was design.md updated + design-review re-run? |
|---|---|---|---|---|
| Prompt default | `DEFAULT_PROMPT_TARGET = AgentTarget::Opencode`, `defaults` vector | exactly that | yes | — |
| Re-ask loop | fresh builder with `defaults` inside the loop; error to stderr | exactly that | yes | — |
| Error text | `error: no agent target selected; mark at least one agent target (.opencode, .agents, .claude) and confirm` | identical text | yes | — |
| Cancel semantics | Ctrl-C aborts; Esc ignored | unchanged dialoguer `interact()` | yes | — |
| Step layout | PTY helper takes the key sequence; two steps replace the old one | exactly that | yes | — |
| Non-terminal paths | untouched | unchanged code paths | yes | — |

**Interaction coverage verification** — inventory and matrix are unchanged by
this change; re-checked:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| select agent targets | yes | Consumer selects agent targets during interactive init | CLI subprocess with piped stdin | `run_givn_with_stdin` writes `opencode, .claude\n` | yes |
| initialize non-interactively | yes | Consumer runs init non-interactively and gets opencode skills | CLI subprocess with `--agents opencode` | `consumer_runs` with the flag | yes |
| resynchronize selected targets | yes | Consumer re-syncs skills after init and the selection is honoured | CLI subprocess `givn skills sync` | `run_givn(env, &["skills","sync"])` | yes |
| initialize a Fossil project | yes | Consumer initializes a Fossil checkout and receives backend-aware guidance | CLI subprocess in a Fossil checkout | `llm_attribution_steps.rs` init call with `--agents` | yes |
| initialize without SCM | yes | Consumer initializes a plain directory and receives a named warning | CLI subprocess in a plain directory | `init` + stdout assertions | yes |

## Visual Review

N/A — the design declares no visual interface. Opposite classification
tested: nothing here renders HTML/CSS/JS; the surface is the terminal text of
the CLI prompt, so a Web UI classification does not hold.

## Retirements

N/A — no capability retired.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| The interactive prompt opens with `.opencode` marked and re-asks until a target is marked | `docs/arc42/06-runtime-view.md` §6.1 step 2; §8.7 | affected, updated | defaults + loop decisions | scenarios 1 and 2 | `src/cli/init.rs:184-212`; PTY scenarios pass | yes |
| Non-terminal empty/unknown-only selection still refuses before the first write | §6.1, §8.7 | affected, updated | "every non-terminal path is unchanged" | out of scope, guarded by the final verification | unchanged `require_agent_target` paths; permanent refusal scenarios | yes |
| ADR-013 amendment wording | `docs/arc42/adr/ADR-013-agent-integration.md`; chapter-09 register; `adr/README.md` | affected, updated (`AMEND_ADR`) | `## Decisions` D1 | scenario commits | ADR + both register rows updated | yes |
| Q-5 agent-ergonomics precondition | `docs/arc42/10-quality-requirements.md` | affected, updated | re-ask vs refusal | scenarios 1–2 | delta scenarios | yes |
| Scripted-init risk | `docs/arc42/11-risks-and-technical-debt.md` R-72 | not affected | unchanged | — | scripted refusal untouched and still asserted by the permanent spec | yes |
| Glossary terms | `docs/arc42/12-glossary.md` | not affected | "Agent target" used | — | wording matches | yes |

- [x] Every chapter marked affected by either assessment was opened and checked
      for stale, contradictory, or placeholder content.
- [x] Every relevant fact is represented in `design.md` and mapped to tasks.
- [x] The completed implementation follows those facts; evidence names files
      and passing scenarios.
- [x] Architecture changes discovered during implementation are recorded in the
      durable chapter and the change-level assessment.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

**Result:** CLEAN

## Coverage

No archive receipt exists yet (unproven candidate); classification from
inspection of the delta and the touched code.

**Coverage result:** no archive receipt yet — classification from inspection.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/cli/init.rs` TTY branch | loop, empty-selection branch, non-empty return | no gap — both branches exercised by the two PTY scenarios (default confirmation; unmark + re-ask + second confirmation) | — |
| `DEFAULT_PROMPT_TARGET` | constant | no gap — used by the `defaults` vector, exercised by scenario 1 | — |
| No other uncovered regions identified | — | — | — |

## Scenario execution evidence

Review does not re-run the suite. The change is not archived yet; the
delta-scope runs recorded in tasks.md are the evidence: regular
`3 tests run: 3 passed`; e2e `0 tests run` (this delta adds no `@e2e`).

**Receipt status:** not archived yet — unproven candidate

## Overlap dispositions

| Scenario A | Scenario B | Disposition | Rationale |
|---|---|---|---|
| addon skills are written to the default agent target after sync | The interactive prompt accepts the pre-marked OpenCode default | boundary | Different commands and guarantees: `givn skills sync` writing addon skills to the persisted default target vs the interactive prompt accepting its pre-marked entry. Shape coincidence from the shared "files exist after the run" steps. |

Removed+added pair: the removed scenario `The interactive prompt refuses a
confirmation without a marked target` is superseded by the two added
scenarios — the interactive refusal no longer exists; the dialog re-asks
instead. The `@e2e` inventory scenario is untouched (this change adds and
removes no `@e2e` tag).

## Split-or-keep

No delta scenario exceeded the lint's long-scenario threshold; the gate
reported none.

| Scenario | Decision |
|---|---|

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified,
      all promised components exist, no downgraded e2e scenarios, no
      over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope (delta runs; the archive
      receipt will carry the merged result).
- [x] Every coverage gap is classified and resolved from inspection.
- [x] No `@wip` tags remain outside removed scenarios.
- [x] The delta spec contains no implementation-layer detail.
- [x] Canonical E2E policy applied: no new action, no new `@e2e` scenario; the
      two added scenarios are regular variants of `initialize`.
- [x] Every E2E scenario keeps a real-interface primary assertion.
- [x] `verify.e2e_command` differs from `verify.command`; delta counts show the
      e2e scope is strictly smaller.
- [x] Implementation matches design.md; no silent deviation.
- [x] Interaction coverage verified above — no mismatches.
- [x] No finding was excused with a classification outside the three buckets.

README-IMPACT: updated - Quickstart

REVIEW: PASS
