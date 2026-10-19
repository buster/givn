# Implementation handoff

Date: 2026-10-19. Status: deterministic core implemented; one complete real
two-stage trial executed. Authorized budget: USD 5 total; observed spend:
USD 1.2421 across four attempts. Remaining: USD 3.7579.

## Real run result

The real trial used the lending benchmark, DeepSeek Flash maximum thinking,
bubblewrap, exploratory host networking, and a fresh session for the follow-up.

- Initial stage: **completed**, archive receipt `lending-register`.
- Follow-up stage: **completed**, archive receipt `reference-only-books`.
- Public probes: **passed**.
- Independent grade: **13 passed, 0 failed, 0 ungraded**.
  All seven initial, five follow-up and the cross-version requirement passed.
- Initial duration: 2801 s; 460 requests; USD 0.6302.
- Follow-up duration: 1724 s; 295 requests; USD 0.3978.
- Trial cost: **USD 1.0280**.
- Initial archive attempts: 12, failures 10. Follow-up archive attempts: 2,
  failure 1.
- Initial `check review` failures: 25. Follow-up `check review` failures: 6.
- OpenCode exited 1 after the initial delivery summary but had archived the
  change. The harness now records driver exit separately from workflow
  completion (probe + archive + protocol).

Attempt costs:

```text
attempt 1: 0.0349
attempt 2: 0.0459
attempt 3: 0.1333
attempt 4: 1.0280
total:     1.2421
remaining from 5 USD: 3.7579
```

Archive and review friction is documented in
`docs/findings/2026-10-19-run4-archive-friction.md`. The dominant issues are a
non-transactional archive, an incrementally discovered runner-result schema,
late task-integrity failures, visual review applied to a CLI product, and archive
gates mutating the tested tree.

The trial exposed and fixed three harness defects:

- OpenCode snapshots were enabled and indexed the growing `.harness` state,
  driving git to high CPU and filling the disk. Snapshots are now disabled in
  the harness config and `.harness/` is git-ignored.
- `git status` during snapshot freezing refreshed `.git/index` after checksums
  were recorded. Snapshots now read git metadata before checksums with
  `GIT_OPTIONAL_LOCKS=0` and exclude volatile `.git/index`.
- Stage analysis treated the cumulative givn history as stage-local. Each stage
  now records a byte offset before the driver and analyzes only bytes appended
  during that stage.

The DeepSeek key was briefly present in `bwrap` argv during attempt 1. It now
moves through a mode-600 secrets file sourced inside the sandbox and is removed
after the driver exits. The machine is private and the owner does not require
rotation; the argv/log exposure fix remains in place.

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

- `./run-tests.sh`: 69 deterministic tests, all passing.
- Archive/review friction report:
  `docs/findings/2026-10-19-run4-archive-friction.md`; the bundle keeps the
  evidence-linked original.
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
3. **The opencode invocation path is exercised.** Attempt 4 completed the
   initial workflow and archived the change, but OpenCode exited with code 1
   after its final summary. The harness now records that exit separately from
   workflow completion.
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

1. Implement one archive-friction candidate, starting with transactional
   archive, and compare current-versus-candidate givn on the same benchmark.
2. Add a request-level provider guard and verified egress boundary; current
   host-network trials remain exploratory.
3. Add one harder, brownfield multi-stage benchmark using the same bundle and
   grader machinery.
4. Keep tasks frozen so saturation does not silently change the ruler.
