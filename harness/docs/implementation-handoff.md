# Implementation handoff

Date: 2026-10-19. Status: deterministic core implemented; one real
initial-stage run executed. Authorized budget: USD 5 total; observed spend:
USD 0.2141 across three attempts. Remaining: USD 4.7859.

## Real run result

The first real opencode trial ran the lending benchmark initial stage with
DeepSeek Flash maximum thinking, bubblewrap, host networking explicitly marked
exploratory, and a USD 4.90 cap.

- Stage outcome: **timed out at 2400 seconds**.
- Public probe: **passed**; `app.py` existed and the basic lending sequence
  worked.
- Independent grade: **7/7 graded initial requirements passed**, no critical
  failures.
- Follow-up: not reached; all follow-up and cross-version requirements
  ungraded.
- givn archive: **not completed**. The agent spent the stage on proposal,
  specs, design, design review, implementation and tests but did not reach
  archive inside the 40-minute bound.
- Observed cost: **USD 0.1333** for this attempt.
- Attempt 1 was interrupted at **USD 0.0349**; attempt 2 at **USD 0.0459**.

The delivered initial product passed every independent initial check. Workflow
completion failed on time, not on behavior. A longer timeout or a smaller task
is required before drawing a conclusion about givn's workflow cost.

The trial exposed and fixed two harness defects:

- OpenCode snapshots were enabled and indexed the growing `.harness` state,
  driving git to high CPU and filling the disk. Snapshots are now disabled in
  the harness config and `.harness/` is git-ignored.
- `git status` during snapshot freezing refreshed `.git/index` after checksums
  were recorded. Snapshots now read git metadata before checksums with
  `GIT_OPTIONAL_LOCKS=0` and exclude volatile `.git/index`.

The DeepSeek key was briefly present in `bwrap` argv during attempt 1. It now
moves through a mode-600 secrets file sourced inside the sandbox and is removed
after the driver exits. Rotate the key used for these attempts.

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
- OpenCode driver with `--standalone`, a private XDG config/data/cache/state
  tree, provider policy limited to `deepseek`, `snapshots: false`, absolute
  binary path, and credential injection through a mode-600 file.
- Pinned model guard, dated price table and explicit exploratory-network flag.
- Descriptive `compare` that reports per-requirement counts and marks
  comparisons inconclusive when evidence or comparability is insufficient.

## Test evidence

- `./run-tests.sh`: 66 deterministic tests, all passing.
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

1. **Host networking is not a verified egress allowlist.** Opencode runs only
   with `--allow-unverified-network` and is labelled exploratory. A controlled
   run needs an external VM/container firewall or equivalent.
2. **Budget granularity is per stage.** The ledger admits one conservative
   envelope per stage. The real run's actual cost exceeded its reservation
   (`admission_exceeded: true`); a multi-request agent needs request-level
   reservation before a hard invoice ceiling can be claimed.
3. **The opencode invocation path is exercised, but the first trial timed out
   before archive.** The driver, private config, credential injection and
   grading worked; the 40-minute stage bound was too short for the observed
   givn workflow.
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

1. Re-run the initial stage with a longer timeout, or reduce the task's workflow
   overhead, and compare the delivered behavior against the observed 7/7
   initial grade.
2. Implement and verify a request-level provider guard plus an outbound
   allowlist, then run the full two-stage workflow once under the remaining
   budget.
3. Add one harder, brownfield multi-stage benchmark using the same bundle and
   grader machinery.
4. Compare current versus candidate givn only after the execution foundation is
   proven; keep tasks frozen so saturation does not silently change the ruler.
