# Design Review: support-opencode-2

## Grilling

Fresh-context grilling subagent (`deepseek/deepseek-v4-pro#high`) read the
proposal, the delta spec, the design, the arc42 assessment, the generated
integration, the test glue, and the runner, then walked the required branches.
Two findings needed an operator decision; the operator answered both in favor
of adding the missing scenario (rejected alternative recorded below). One
critical finding was refuted by repository evidence. All other findings were
resolved by hardening the artifacts.

### Findings and hardening applied

| # | Severity | Finding | Resolution |
|---|---|---|---|
| F1 | Critical | The replayed OpenCode 2 plugin contract was claimed to contradict the published plugin documentation (named export, `shell.env`, `tool.execute.before` with `input.tool === "bash"`, no `session.hook`, no `create.before`, no `ctx.model.default`). | Refuted from repository evidence. The superseded branch drove real pinned OpenCode 2.0.22 servers (`tests/fixtures/opencode/runtime_driver.mjs`) and a contract probe recorded the exact lifecycle the generated integration implements: session events `context`/`compaction`/`generate`, tool `execute.before`/`execute.after` carrying session/message/call identifiers and `input.command`, and shell `create.before` carrying `{command, cwd, timeout, shell, env}`. The installed `opencode` v2.0.26 binary contains the `create.before` and `session.hook` strings. The published documentation the subagent fetched describes the older 1.x contract. No design change. |
| F2 | High | The proposal promises attribution stays marked unavailable when no model is observed, but no scenario exercises the integration's unavailable path; the design's traceability row wrongly claimed the fragment's existing non-e2e scenarios covered it (they exercise the commit-message hook with missing environment values, not the integration). | Operator decision (2026-10-09): add the regular scenario `OpenCode leaves attribution unavailable when no model is observed` (`@givn.added @wip`) and correct the traceability row. Rejected: recording the gap and amending the claim only. |
| F3 | High | In a single-session scenario, the issuing-session link (D2) and the rejected last-observed-model behavior produce the same environment, so a regression to the rejected behavior would still pass. | Operator decision (2026-10-09): add the regular scenario `OpenCode attributes a command to the session that issued it` (`@givn.added @wip`), where a later model observed in another session must not claim the command. Rejected: recording the cut and leaving the session-link guarantee unverified. |
| F4 | Medium | The generated-integration Given step compared the embedded asset only with the repository's dogfood copy, never with the file `givn init` wrote into the temporary project. | Design step definitions now require the omp-pattern assertion: the generated temporary-project integration equals the embedded asset. |
| F5 | Medium | The Interaction Coverage Matrix cited an inventory entry the fragment's durable inventory does not hold. | Matrix row annotated as a change-level mapping; the pre-existing fragment inventory gap is recorded and not extended by this change. |
| F6 | Low/Medium | "Issuing session" entered durable chapters 2 and 6 without a glossary entry, while the arc42 assessment claimed no new domain term. | Glossary entry added under chapter 12; arc42 row 12 changed from No to Yes; the documentation-status paragraph updated. |
| F7 | Low | The R-50 mitigation could read as resting on a simulation of unverified fidelity. | Consequence of F1: the mitigation wording now names the replayed OpenCode 2 contract. No further change. |

## Branch dispositions

- **Scope**: the delta spec matches the proposal; the two new regular scenarios
  close the only observable promises without a scenario (F2, F3). Out of Scope
  stays consistent: the dropped surface is OpenCode 1 and integration-internal
  edges, not the attribution guarantees.
- **Tech choices**: one generated integration, no version detection, no
  sibling file; the simulated inline driver is the operator-approved
  verification mechanism (D3). No hidden coupling beyond the existing
  `OpencodePluginState` world field.
- **Missing scenarios**: addressed by F2 and F3.
- **Testability**: RED is genuine — the driver imports the integration's
  default export, which the previous contract does not provide, so all three
  scenarios fail before the port and pass after.
- **E2E fidelity**: the normalized consumer action is "an OpenCode session
  supplies a model and prepares a shell"; the primary assertion reads the
  contributed environment from the generated integration; the real interface
  is the OpenCode 2 plugin contract, replayed with the payload shapes recorded
  from the pinned real runtime (F1).
- **Visual contract**: Interface: none — no screen is introduced or changed.
- **Interface classification**: the opposite classification does not hold; no
  screen is rendered in a browser.
- **Persona disposition**: `developer--primary-user` (who runs OpenCode and
  reads commit attribution); evidence recorded in the design's persona impact
  paragraph. No persona invented or promoted.
- **Deferrals**: the two added scenarios carry `@wip` until their step
  bindings exist in the implement step, when the tag is removed; the E2E
  runner is configured (`verify.e2e_command = ./run-tests.sh --e2e`). No
  undeclared deferral.
- **Interaction Coverage**: the matrix maps the change-level action to the
  modified acceptance scenario and names the two regular scenarios; the
  durable fragment inventory gap is annotated (F5).
- **Risk**: contract drift remains the top risk; the mitigation reads the
  hook payloads defensively, keeps missing metadata non-blocking, and
  replays the recorded contract in the acceptance scenario.
- **Domain context**: fragment use case `support`; capability
  `attribute-commits`; the actor is the OpenCode session, not a persona.
- **ADR qualification**: D1, D2, and D3 each fail the single-change
  reversibility falsification (single component, one change) and their
  rationale is owned by the proposal, design, and specs; no existing ADR
  covers the plugin contract generation. No ADR introduced, none amended.
- **Arc42**: all twelve chapter rows re-derived independently against the
  proposal and design; the assessment matches except row 12, which the F6
  glossary addition corrected to Yes; every Yes chapter was verified to match
  the design rather than placeholder text.
- **Ubiquitous language**: the design's terms are the glossary's terms;
  "Issuing session" is now recorded there (F6).

## Operator questions

1. Should the unavailable-model promise gain its own scenario, or be recorded
   as an accepted cut? — Answered: add the scenario (F2). Rejected: accepted
   cut with a corrected claim.
2. Should the session-link guarantee gain a discriminating scenario, or be
   recorded as an accepted cut? — Answered: add the scenario (F3). Rejected:
   accepted cut leaving D2 unverified.

DESIGN-REVIEW: PASS
