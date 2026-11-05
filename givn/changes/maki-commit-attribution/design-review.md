Review kind: self-review
Author: maki agent (maki-commit-attribution author)
Reviewer context: maki agent (maki-commit-attribution author)

# Design Review: maki-commit-attribution

## Grilling log

| # | Branch | Question | Outcome |
|---|---|---|---|
| 1 | Scope | Do the delta specs cover exactly the proposal: generated Maki attribution, existing-manifest grant, edited-manifest warning, installation/upgrade reports, and the commit result? | closed — every proposed observable behavior has a scenario; no extra capability was added. |
| 2 | Tech choices | Is a separate Lua module plus a second fence simpler than folding attribution into the timeline reporter? | closed — the separate module keeps the reporter's bounded intake independent from a command-rewrite hook, and either fence can be removed without editing the other. The existing environment protocol is reused, so no Rust attribution change is needed. |
| 3 | Missing scenarios | The proposal says a missing model *or reasoning mode* stays unavailable, but the partial-context scenario only covers both missing. Is the model-known/reasoning-missing boundary absent? | closed — added `Maki marks a missing reasoning mode as unavailable` to the delta. |
| 4 | Testability | Can every scenario fail in RED, and are the Then steps concrete? | closed — the driver returns the actual prepared command and the e2e asserts the real Git committer and message. The design names `.fail_on_skipped()` and the single-scenario command. |
| 5 | Testability | Does the plan prove the layer is registered, rather than only that `prepare()` produces a string? | closed — the driver now reads `maki.api.get_slots()` and fails unless `tool.bash.input` lists the project `init.lua` filler. |
| 6 | E2E fidelity | Is the Maki commit scenario a real-interface test or a repository-only assertion? | closed — it drives the real Maki 0.7.0 binary to prepare the command, runs it through a real shell with the installed Git hook, and asserts the commit metadata produced by Git. |
| 7 | Interaction coverage | Does the design carry the requiring Interaction Coverage Matrix for every touched inventory entry? | closed — added the matrix for `initialize-a-project`, `attribute-commits`, `enable-commit-attribution`, and the included `enable-optional-addons` entry. The pre-existing `initialize-a-project` corpus mismatch is recorded as context, not introduced here. |
| 8 | Risk | What is the most likely implementation failure? | closed — the Maki driver may fail to load the module or register the slot on a changed host surface. The driver asserts both, the module no-ops when `set_slot` is absent, and the E2E fails rather than silently passing. |
| 9 | Use-case and Persona context | Is `initialize-project` the right leaf owner, are the fragment capabilities included, and are the Personas confirmed rather than invented? | closed — `initialize-project` includes `fragment: support`; the change confirms `developer--primary-user` and `agent--technical-role`; no Persona is used as an actor. |
| 10 | Decision ledger | Does every proposal question resolve? | closed — D1 is answered in the design ledger as the focused session's model, with the subagent case named out of scope. No decision is carried. |
| 11 | Contracts and components | Does each contract have exactly one implementing component, and are adapters named separately? | closed — the initialization contract is implemented by `src/skills/targets.rs`; the CLI summary is its terminal adapter. The attribution contract is implemented by `src/cli/attribution.rs`; the Lua asset is the Maki adapter. |
| 12 | Decision timing | Is any foundational question carried? | closed — the model source is decided now because a scenario depends on it; the subagent question is explicitly out of scope, not carried. |
| 13 | Visual design contract | Does the change introduce or visibly change a screen? | closed — `Interface: none`; the change adds generated files and one CLI report line, with no rendered screen. |
| 14 | Deferrals | Is any mandatory check deferred? | closed — none. |
| 15 | Persona disposition | Which Personas are affected, and what evidence covers them? | closed — `developer--primary-user` reads the report and owns the manifest; `agent--technical-role` issues the attributed commit. The external contributor and maintainer lenses are not confirmed for this change. |
| 16 | Interface classification | Does the opposite classification hold? | closed — no HTML, CSS, or JavaScript is rendered; the browser classification does not apply. The CLI report is covered by the existing terminal transcript. |
| 17 | ADR qualification | Is the recorded `AMEND_ADR` for ADR-013 correct, and is the amendment applied? | closed — ADR-013 owns the generated-destination boundary; its Consequences sentence now names the Maki attribution integration, and chapter 09 records the amendment. |
| 18 | Architecture documentation | Do an independent 12-row derivation and the chapter files agree with `arc42.md`? | closed — rows 2, 3, 4, 5, 6, 8, 9, 10, 11, and 12 are affected and were updated; rows 1 and 7 are unaffected. All 12 chapter files exist with substantive content and no ASCII-art diagrams. |
| 19 | Ubiquitous language | Are the design's terms in `docs/glossary.md`? | closed — added `Focused session` and `Maki attribution grant`; the `Harness` definition now names Maki. |
| 20 | Structural model | Did the change assess its structural impact? | closed — no bounded context, aggregate, external system, or relationship changes; the design records the no-op and `docs/domain-model.md` stays unchanged deliberately. |

## Hardening applied

- Added `Maki marks a missing reasoning mode as unavailable` to the
  `attribute-commits` delta.
- Added the Interaction Coverage Matrix to `design.md`.
- Added the `Focused session` and `Maki attribution grant` glossary terms and
  named Maki in the `Harness` definition.
- Added the structural-model no-op to `design.md`.
- Added the `maki.api.get_slots()` registration proof to the E2E design.
- Corrected the Contracts and Components table so each contract names one
  implementing component and its adapters separately.

## Residuals

- The pre-existing `initialize-project` interaction inventory declares
  `Consumer runs init non-interactively and gets opencode skills` but the
  permanent feature does not carry that exact e2e title. The mismatch is not
  introduced by this change and is recorded in the matrix as context. No new
  e2e tag was removed.
- Maki's focused-session model is used for a subagent's commit; the proposal
  and design name that limitation as out of scope. The e2e covers the
  top-level command path.

DESIGN-REVIEW: PASS
