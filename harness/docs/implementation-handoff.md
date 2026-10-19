# Implementation handoff

Date: 2026-10-19. Status: deterministic core implemented; no paid call made.
Authorized budget: USD 5 total; spent: USD 0.00. Remaining: USD 0.00 allocated
for this session (no request was dispatched).

## Delivered

- Streaming `events.jsonl` reader with byte-boundary hashing, tolerant schema
  handling, namespaced ids, diagnostics and filters.
- Shared accounting pipeline: interval union, per-kind sums, usage authority,
  cost normalization, recorder-plumbing exclusion, failure/recovery
  classification, archive friction, questions, context pressure, subagents,
  phase markers and evidence-linked findings.
- `givn-harness analyze` with Markdown and JSON outputs and optional read-only
  repository context.
- Two-stage lending benchmark with public contract, public probes, private
  reference, hidden black-box grader, five seeded defects and a verification
  gate.
- Durable bundles: manifest, append-only journal, frozen snapshots with
  checksums, raw record copies, stage analysis, independent grade history.
- Staged runner: fresh driver process per stage, stage-specific probes, archive
  completion that counts only a new archive entry, follow-up not-reached
  recording, failure preservation and resume of a completed prefix.
- Bubblewrap policy with private HOME/`tmp`, minimal `/etc`, network namespace
  denial, opt-in credential mounts and a probe that proves workspace, host and
  network boundaries. Graded execution has no unsandboxed fallback.
- Conservative spend ledger with reservations, dispatch marking, uncertain
  charges, crash-resume behavior and admission refusal.
- Pinned model guard, price-table template and explicit paid-run blockers.
- Descriptive `compare` that reports per-requirement counts and marks
  comparisons inconclusive when evidence or comparability is insufficient.

## Test evidence

- `./run-tests.sh`: 57 deterministic tests, all passing.
- `givn-harness benchmark verify benchmarks/lending`: reference passes all 13
  requirements; scaffold fails substantive requirements; all five seeded
  defects are caught.
- `givn-harness sandbox-probe --network none`: workspace-writable,
  host-state-hidden and network-denied checks pass.
- Full script-driver two-stage run under bubblewrap: both stages complete and
  the saved bundle grades 13/13.
- Real record analysis: 13,795 events from the parent repository's record; two
  diagnostics (duplicate inline/linked usage and a usage parent that is not a
  turn) and eight evidence-linked findings. Observed cost 4.148 USD, marked
  partial because two turns lack cost.

## Known blockers and limits

1. **Paid opencode execution is blocked by design.** Host networking is not a
   verified egress allowlist, and the runner cannot intercept every native
   request. No paid request may start until a verified outbound route and
   request-level model/cost guard exist. The USD 5 authorization remains
   unspent.
2. **Budget granularity is per stage.** The ledger admits one conservative
   envelope per stage. A multi-request agent needs request-level reservation
   before a hard invoice ceiling can be claimed.
3. **The opencode invocation path is not exercised end to end.** It is
   implemented with a fresh session, private server, JSON format and pinned
   model, but blocked before dispatch by the policy above.
4. **Complex-task support is manifest-level.** The runner and grader support
   any number of ordered stages; only the lending task is packaged and
   verified. A brownfield task needs its own public contract, reference,
   hidden grader and defects.
5. **No LLM judge.** Artifact quality is not scored. Independent behavioral
   tests are the primary quality evidence.
6. **Subtree synchronization is now applied to the real parent.** The standalone
   repository is authoritative; parent join commit `937494bf` mirrors its
   tracked tree under `harness/`. The pre-join tree is tagged
   `harness-pre-subtree` and backed up locally. Subsequent changes must be
   committed here first and synchronized down. See `docs/subtree-sync.md`.

## Next increments

1. Implement and verify a request-level provider guard plus an outbound
   allowlist, then run the small workflow once under the USD 5 cap.
2. Add one harder, brownfield multi-stage benchmark using the same bundle and
   grader machinery.
3. Compare current versus candidate givn only after the execution foundation is
   proven; keep tasks frozen so saturation does not silently change the ruler.
