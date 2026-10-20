# givn-harness

Evaluation harness for improving givn's workflow, guardrails and rules using
evidence rather than artifact checkboxes.

It has two deliberately separate modes:

| Mode | Input | Output | What it establishes |
|---|---|---|---|
| **Retrospective analysis** | Existing `events.jsonl` files, optionally a metadata sidecar | `analysis.md` and `analysis.json` | Observed counts, available durations, usage, cost, errors and evidence-linked hypotheses. It never executes repository code or calls a model. |
| **Controlled evaluation** | A pinned benchmark, givn binary, driver, model, environment and policy | A durable trial bundle with frozen stage snapshots, independent grades and a comparison | Whether a delivered product still satisfies independent behavioral checks, and where a trial stopped. |

Neither mode produces a single "quality score". Outcome, workflow completion
and measurement validity stay separate.

## Quick start: offline analysis

<!-- givn:begin:run -->
```sh
uv sync
uv run givn-harness analyze /path/to/givn/history/events.jsonl \
  --output analysis/current
```
<!-- givn:end:run -->

Read `analysis/current/analysis.md` first. It reports data-quality gaps before
interpretations, then time, usage, cost, failures, archive friction, questions,
context pressure and evidence-linked findings.

Exported reports are redacted by default: question text, command labels, error
text and agent identities are replaced with length markers while event ids and
structural counts remain. Use `--no-redact` for a local detailed report, and
review it before sharing.

Useful filters:

```sh
uv run givn-harness analyze events.jsonl --output analysis/session \
  --session 01a148a7-1dd7-7000-8b9a-651b2cabbdd0 \
  --since 2026-10-17T09:00:00Z
```

The source record is never modified. A byte boundary and SHA-256 are captured
on read; bytes appended after that boundary are reported separately. Malformed
lines, unsupported schemas, duplicate ids and ambiguous usage are counted, not
silently dropped.

## What the analyzer refuses to claim

- `at` is an **end** timestamp. An interval is `[at - duration_ms, at]`.
- Nested commands, tools, turns and subagents overlap. The analyzer reports
  the union of intervals and per-kind sums separately. Per-kind sums are never
  added into wall time.
- Inline turn usage and a linked usage event are one request. Inline usage wins;
  the duplicate is reported and not counted.
- Cumulative cost rollups are not summed. Missing cost is not zero. Observed
  cost is producer-reported, not an invoice.
- Recorder intake (`history ingest`, attribution) is excluded from command
  friction totals.
- Phase markers mean an artifact was observed Done, not when work started.
- A question may have wait time but no recorded answer. Silence is not approval.
- A record alone cannot prove product correctness, an independent false-green
  rate, or whether a review prevented a defect. Those stay unmeasured.

## Controlled evaluation

### 1. Verify the benchmark

```sh
uv run givn-harness benchmark verify benchmarks/lending \
  --output /tmp/lending-verification
```

This runs the trusted reference, the public scaffold, and every seeded defect.
The gate passes only when:

- the reference passes **all** required checks;
- the empty scaffold fails substantive checks;
- each seeded defect trips its named checks.

### 2. Run without spending money

A deterministic script driver exercises the whole runner, snapshot and grading
pipeline:

```sh
uv run givn-harness run \
  --benchmark benchmarks/lending \
  --output runs/ \
  --condition fake \
  --driver script \
  --driver-command "python3 tests/agents/copy_app.py" \
  --driver-env HARNESS_APP_SOURCE=$PWD/benchmarks/lending/private/reference/app.py \
  --sandbox required \
  --network none \
  --no-analyze

uv run givn-harness grade runs/<bundle> --benchmark benchmarks/lending
```

The script driver copies the reference application into the frozen workspace.
No model is contacted. The same command with `--sandbox none` is useful for
debugging but not labelled a graded execution.

### 3. Real model runs

Paid execution is blocked unless the run states its evidence class explicitly.

**Controlled run.** Use a verified egress boundary outside the harness, for
example a VM or container firewall that permits only `api.deepseek.com:443`.
Supply the pinned binary, budget, dated price table and key file:

```sh
uv run givn-harness run \
  --benchmark benchmarks/lending \
  --givn-binary /path/to/pinned/givn \
  --output runs/current \
  --condition current \
  --repeats 1 \
  --driver opencode \
  --network host \
  --allow-unverified-network \
  --sandbox required \
  --budget-usd 5 \
  --price-table config/prices.toml \
  --deepseek-api-key-file /path/to/key
```

`--allow-unverified-network` is required because the harness cannot prove the
egress boundary. Inside a VM with an enforced firewall, the same flag records
the host-network mechanism while the VM supplies the actual boundary.

**Exploratory run.** On a normal workstation, the same command with
`--allow-unverified-network` is explicitly exploratory. The manifest records
`execution_class: exploratory` and `network_allowlist: unverified`. Exclude it
from controlled comparisons.

The opencode driver creates a private XDG tree per trial: private config, data,
cache and state. It mounts only the OpenCode runtime under `~/.opencode`; the
host config, plugins, cache and credentials database are not mounted. A
generated project config allows only the `deepseek` provider and pins
`deepseek/deepseek-flash#max`. The DeepSeek credential is injected through
`DEEPSEEK_API_KEY` from `--deepseek-api-key-file`; the key is not written to the
manifest or logs.

For a first real check, stop after the initial stage:

```sh
  --stop-after initial
```

The follow-up is then recorded as `not_reached`, and the initial snapshot can
still be graded.

`config/prices.toml` contains dated rates from the OpenCode model catalog.
`config/prices.example.toml` is a placeholder template; zero or placeholder
rates are rejected. Re-check prices before a new campaign.

See `docs/real-run.md` for the preflight checklist, controlled-versus-
exploratory setup, key handling and post-run inspection.

### 4. Grade and compare

```sh
uv run givn-harness grade runs/current/<bundle> --benchmark benchmarks/lending
uv run givn-harness compare \
  --baseline runs/current \
  --candidate runs/candidate \
  --output comparison/
```

`grade` reads frozen snapshots. It copies each snapshot to a temporary
directory, starts the submission through its documented black-box entry point,
and never imports submission code. The authoritative snapshot is checksummed
and remains unchanged. Regrading appends a new grade instead of overwriting an
old one.

`compare` reports individual trials, stage completion, per-requirement pass /
fail / ungraded counts, cost completeness and driver time. Fewer than two
trials per condition, differing execution conditions, missing grades or
ungraded requirements make the result **inconclusive**. No single score is
computed.

## Trial bundle

```text
runs/<bundle>/
  manifest.json              # versions, hashes, policy, stage list, status
  journal.jsonl              # append-only execution and budget events
  spend.json                 # reservations and committed charges (if budgeted)
  stages/initial/
    request.md               # exact stage request
    request.json             # task hash, timeout, required gates
    prompt.md                # exact driver prompt
    agent.stdout.log
    agent.stderr.log
    driver.json
    probe.json
    stage.json               # completion, probe, archive, analysis summary
    snapshot/                # authoritative frozen working tree
    snapshot.json            # per-file checksums, git state, givn artifacts
    events.jsonl             # raw record copy, when present
    analysis.md / .json      # offline analysis of that record
    probe...                 # stage-specific public check output
  stages/followup/           # fresh session, same repository
  grades/<grade-id>/
    result.json              # independent requirement-level evidence
    grader.log
    grade.md
```

Manifest identity includes the harness revision/dirty state, givn binary
checksum and version, benchmark and grader hashes, model/variant, driver,
sandbox and network policy, budget, repeats and stage boundaries. Git history
alone is not enough: uncommitted and untracked files are checksummed in the
snapshot too.

## Benchmark package

```text
benchmarks/lending/
  benchmark.toml             # public stages, gate checks, verification
  public/                    # copied into the agent workspace
    contract.md              # every graded obligation
    task-initial.md
    task-followup.md
    probe.py                 # public narrow smoke check
    probe_followup.py
    scaffold/                # empty starting point
  private/                   # never copied into the agent workspace
    grader/                  # black-box behavioral tests
    reference/app.py         # trusted reference implementation
    defects/*.patch          # seeded defects for the verification gate
```

The first benchmark is a small lending register delivered in two stages:

1. members, books, borrowing, five-loan limit, exclusivity, returns,
   persistence and rejection consistency;
2. reference-only books, existing-loan returns, flag persistence, and
   cross-version state preservation.

The follow-up starts in a **fresh agent session** with the repository produced
by the first stage. It is delivered only after the initial stage's workflow and
public entry point complete. Hidden test success is never the continuation
signal.

## Isolation

The default sandbox uses bubblewrap with:

- all namespaces unshared (including network) unless host networking is
  explicitly requested and recorded as unverified;
- a private HOME and private `/tmp`;
- a minimal `/etc` allowlist;
- the trial workspace as the only writable project mount;
- credentials only through explicit read-only opt-in paths.

`sandbox-probe` proves workspace writability, host sentinel denial and network
denial. If the probe fails, a graded run stops. There is no unsandboxed graded
fallback.

Grading keeps the trusted grader and private benchmark material outside the
submission's readable namespace. The grader launches only the frozen snapshot
and a fresh data directory in bubblewrap, with no network and no host home or
benchmark mount. The authoritative snapshot is copied first, then checksummed
before grading; it is never modified by submission code.

## Improving givn with the harness

Use the same loop for a rule, guardrail or workflow change:

1. **Observe.** Run `analyze` on real records and read the cited evidence, not
   just a finding title.
2. **Form one narrow hypothesis.** Example: repeated archive rejection is
   associated with generated files dirtying the tree.
3. **Change one thing.** Build a candidate givn binary; keep the benchmark,
   model, driver, sandbox and resource policy fixed.
4. **Run both conditions.** Use `--repeats` for each condition; start
   sequentially so accounting and resource contention stay simple.
5. **Grade frozen snapshots.** A faster candidate that breaks a requirement is
   not an unqualified improvement.
6. **Compare.** Retain the change only when independent behavioral evidence and
   resource measurements both support it. Report inconclusive when they do not.

Compliance without a prompt is not evidence that deleting the prompt improves
outcomes. Artifact presence, PASS markers and self-reported green tests are not
primary quality evidence.

## Development

<!-- givn:begin:development -->
```sh
./run-tests.sh                      # deterministic, no provider, no spend
uv run python -m unittest tests.test_runner -v
uv run givn-harness benchmark verify benchmarks/lending
```
<!-- givn:end:development -->

The default suite never contacts a provider. Sandbox integration tests are
skipped when bubblewrap cannot establish its namespaces.

## Repository relationship

The standalone repository is authoritative. `givn/harness/` in the parent Rust
repository is a synchronized downstream copy at parent join commit `937494bf`,
not a second editing surface. The pre-join tree remains at tag
`harness-pre-subtree`. See `docs/subtree-sync.md` for the exact local remote
commands and validation.

## Known limits

- Host networking is not a verified egress allowlist. The harness requires
  `--allow-unverified-network` and labels those trials exploratory. Controlled
  evidence needs an external VM/container firewall or equivalent boundary.
- Budget admission is proven at the ledger level. The runner reserves one
  conservative envelope per stage; a multi-request agent needs request-level
  interception before a hard invoice ceiling can be claimed.
- The lending benchmark is intentionally small. Saturation is not evidence that
  givn cannot improve; add harder tasks without changing the existing one.
- The grader measures documented behavior, not architecture, maintainability or
  end-user visual quality.
