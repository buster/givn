# Design review: ise-ship-component-tier

Review kind: self-review
Author: givn-maintainer--owner
Reviewer context: givn-maintainer--owner

## Grilling

| Branch | Question and answer | Finding |
|---|---|---|
| Scope | Does the delta contain exactly the proposal? The proposal asks for a component tier on every Ise ship, review and archive refusal without it, and the five served-guidance corrections. The delta has five `givn-workflow-enforcement` scenarios (retained-corpus pass, delta plus retained corpus, missing result, short result, review refusal) and five `serve-extended-rules` scenarios. The archive refusal is the same predicate as review and is covered by the predicate's unit tests; no separate archive fixture is written because every archive precondition outside the receipt is unrelated to this rule. | None after the review scenario was added; see Hardening. |
| Tech choices | Is parsing `design.md` the right trigger? Yes. The design decision is the only durable record of `Rebuilt code: ship`; the change lock supplies `ise` effectiveness. The reader accepts only a plain declaration line (fenced examples, comments, and prose mentions excluded), because a design that discusses this very rule would otherwise declare a ship. A structured design field would be a larger schema change for one value. | Finding: the first reader matched a prose mention and classified this `No rebuild` change as a ship. Hardened by requiring a plain line; see Hardening. |
| Missing scenarios | Are error paths present? Yes: no result file, a short result, review without a receipt, and the delta-plus-corpus composition. The zero-unit corpus keeps today's branch by D4; a scenario would assert an unchanged rule. | None. |
| Testability | Can each scenario fail in RED? Yes. Before the implementation, `check tasks` prints the not-applicable line and writes no receipt, so the pass scenario fails on the missing `component_executed=2/2` line and the receipt field; the refusal scenarios fail on exit 0. The fixture runner is a real script that reads `GIVN_RESULT_FILE` and `GIVN_RESULT_SCOPE`, and the receipt assertion reads the real receipt JSON. | None. |
| E2E fidelity | The scenarios drive the compiled binary with `--change`; no library call replaces the CLI. The fixture supplies the e2e package and runner command that `preflight_e2e_boundary` resolves, so the scenario exercises the real preflight rather than bypassing it. | None. |
| Interaction Coverage | The owning use cases keep their existing inventory rows. The new scenarios are variants of `givn check tasks` and instruction reading, not new consumer actions; no `@e2e` inventory entry is added or removed. | None. |
| Risk | Most likely failure: the component count is compared to a stale declared count and a legitimate runner is refused after a Specification edit. The digest of `givn/specs` is bound into the receipt and rechecked in `freshness`, so an edited corpus makes the receipt stale rather than silently passing. Second risk: a project runner that writes a `scope` value other than `change` now fails the component run; the run uses the documented `change` value, so the documented contract is unchanged. | None. |
| Use-case and Persona context | Use cases `verify-change`, `prepare-an-ise-test`, and `obtain-effective-guidance`; personas `developer--primary-user`, `agent--technical-role`, `givn-maintainer--owner`. The change preserves each use case's goal, guarantees, and inventory; the delta does not rewrite the narratives beyond the affected rules. No invented Persona. | None. |
| Decision ledger | D1-D8 resolve in `design.md`. D7 records the Ise rebuild decision as `No rebuild`; the change edits Source code in place and records no `Rebuilt code` line. | None. |
| Contracts and components | `ChangeVerificationEvidence` is consumer-shaped (result, subject path and digest) and implemented by `cli::check` plus `proof`. `IseShipDecision` is implemented by `ise::ship` plus `ise::contract`; `ServedInstruction` by `cli::instructions` plus the assets. No consumer imports another component's implementation. | None. |
| Decision timing | No carried question. D1-D6 are answered before tasks; none is deferred to usage. | None. |
| Ise rebuild decision | Exactly one `Decides D7` entry records `No rebuild`; no `Rebuilt code` line is present. | None. |
| Visual design contract | The change alters CLI output and served text, not a screen. No visual contract is declared; terminal assertions and the unit tests cover the output. | None. |
| Deferrals | None. The Passuff branch that exposed the gap stays a separate change; this change does not modify it. | None. |
| Persona disposition | The CLI output and instruction text are read by `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`; the evidence is the command output and the guidance scenarios. | None. |
| Interface classification | CLI. The real interface is the compiled binary invoked by the fixture; no browser driver is required. | None. |
| ADR qualification | An architectural commitment: the receipt contract now binds a component subject and every `verify` hook adopts the result contract. Real alternatives: gate only at archive, or add a separate `givn ise verify-ship` command. Durable consequence: harnesses and external projects rely on the receipt field and the hook result. Existing decisions: ADR-076 owns the Ise trial record; this change extends the same evidence family rather than reversing it. Route: `AMEND_ADR`. | None. |
| Architecture documentation | Walked the 12 chapters: 2, 5, 6, 8, 9, 11, and the glossary are affected; 1, 3, 4, 7, and 10 are unchanged. `arc42.md` records each row and the ADR route. | None. |
| Ubiquitous language | Uses existing **Execution unit**, **Verification contract**, and **Trial result**; introduces no new glossary term. The receipt field is named `component` and the refusal text reuses `component_executed`. | None. |
| Structural model | No new domain entity or boundary; `docs/domain-model.md` needs no structural change. | None. |

## Hardening

- **Added the review refusal scenario.** The first draft asserted the gate through
  `check tasks` only. `check_change_receipt` has its own not-applicable branch,
  so a separate scenario now asserts that `givn check review` refuses the ship
  without a receipt.
- **One predicate for three gates.** `change_verification_required` owns the
  "not applicable unless a component tier is required" decision; `check tasks`,
  `check review`, `archive`, and `state` call it. Unit tests assert the
  predicate for a ship, a non-ship, and a zero-unit corpus.
- **Receipt subject is explicit.** The receipt records the subject path and
  digest it actually executed instead of assuming the change delta; a
  whole-system ship records `givn/specs`. `freshness` resolves the recorded
  path, so a corpus edit makes the receipt stale.
- **The ship trigger is a plain declaration.** `givn check tasks` on this
  change first printed `component_executed=1317/1317` because the reader matched
  a prose mention of `Rebuilt code: ship` in `design.md`. The reader now
  requires the line to begin with the marker; a design that documents the rule
  no longer declares a ship, and the fixture records the plain line the
  guidance asks for.
- **Component result validation is one function.** `read_component_result`
  maps a missing result to `component_executed=0/<declared>` and a short result
  to `component_executed=<executed>/<declared>`; the same message shape the
  trial gate uses.
- **Hook result contract.** `run_verify_hook` now sets and validates a result
  file; the failure names the command and the missing result, so a `verify`
  post-hook cannot pass on exit status alone.

DESIGN-REVIEW: PASS
