# Handoff: change-verification-receipt

Planning input for the givn agent. This is not a givn artifact. Use it to write
the specs, design and tasks.

## Evidence

Real trial, initial stage:

- `givn check tasks --change lending-register` passed at event `0:652`
  (16:29:13), but did not persist a test result.
- `givn check review --change lending-register` failed 25 times on integrity,
  task revision and README-impact issues. It never ran the test suite.
- Archive then failed 10 times on clean-tree, result-file and state issues
  before eventually succeeding.
- `regular.json` was rejected for missing `scope`, then `total`, then `passed`.
- The regular and E2E suites were green; the archive failures were not test
  failures.

## Source references

- `src/cli/check.rs:103-174` — `check tasks` runs `verify` against change specs;
  `check review` runs integrity, run declaration, overlap, retirement and
  routing only.
- `src/cli/check.rs:106-109` — comment: review does not re-run the suite.
- `src/hooks.rs` — verify hook receives `GIVN_FEATURES`, not a result file.
- `src/cli/archive.rs:258-303` — archive runs regular/e2e scopes after the
  candidate commit.
- `src/proof.rs:208-269` — `RunnerResult` schema and validation.
- `src/cli/init.rs` and `src/cli/run.rs` — scaffold and commands surface.

## Proposed design shape

### `givn test-result`

New CLI helper:

```text
givn test-result --total N --passed N --failed N --skipped N
```

Behaviour:

- require `GIVN_RESULT_FILE`;
- validate non-negative integers and count reconciliation;
- write atomically;
- write only the four counts; `scope` is optional and ignored;
- clear error if the variable is unset.

### `givn init`

Generate a sentinel `run-tests.sh` only when absent and when `verify.command` is
still the sentinel. The script must fail closed and show the `givn test-result`
example. Do not overwrite an existing runner.

Add the environment/result contract to `givn/commands.yaml` comments and to the
`givn doctor`/commands help surface.

### `givn check tasks`

- require a clean committed tree except verification/evidence paths;
- set `GIVN_FEATURES=givn/changes/<id>/specs`;
- set `GIVN_RESULT_FILE=givn/changes/<id>/verification/tasks-result.json`;
- optionally set `GIVN_SCOPE=change` but do not require it from the runner;
- run `verify.command`;
- require a valid non-empty green result;
- write `givn/changes/<id>/verification/tasks-verification.json` with
  `head`, `features_digest`, `command_digest`, result counts, status and time.

The receipt must not be committed before archive; it is declared evidence.

### `givn check review`

- validate the receipt exists, passed and is current;
- if missing/stale: tell the operator to run `givn check tasks --change <id>`;
- run no tests;
- keep integrity, overlap, retirement and routing checks.

### `givn archive`

- validate the receipt before merge;
- refuse if missing or stale;
- merge and publish the candidate;
- run the full merged suite once;
- do not rerun change specs;
- full-suite failure leaves the pending candidate and rolls forward.

## Receipt schema

```json
{
  "schema": 1,
  "change": "<id>",
  "status": "passed",
  "head": "<tested revision>",
  "features_path": "givn/changes/<id>/specs",
  "features_digest": "<sha256>",
  "command_digest": "<sha256>",
  "runner_exit": 0,
  "result": {"total": 48, "passed": 48, "failed": 0, "skipped": 0},
  "finished_at": "<ISO-8601>"
}
```

Staleness when `head`, features digest or command digest differs.

## Acceptance criteria

- Review cannot PASS without a current green change-scoped receipt.
- Archive refuses a missing/stale receipt before mutating specs.
- Archive does not rerun the change suite when the receipt is valid.
- Archive runs the full merged suite once.
- Archive retries caused by state/schema/evidence errors reuse the receipt.
- A project-specific runner can satisfy the contract with four counts and an
  exit code; `scope` is optional.
- `givn init` and `givn doctor` make the contract discoverable.
- Missing/malformed results report all missing fields in one failure.
- Existing runners that already write `GIVN_RESULT_FILE` for archive can reuse
  the same writer.

## Risks

- Requiring a clean tree may force an extra commit; document the ordering.
- Receipt path under the change directory must be an allowed evidence path and
  must not invalidate its own tree hash.
- Some runners ignore `GIVN_FEATURES`; givn cannot prove selection. Record the
  features path/digest and make the limitation explicit.
- Changing the result contract must remain backward-compatible for existing
  archive result writers.

## Suggested scenario areas

- check tasks writes a green receipt;
- check tasks fails on missing/malformed/failed/skipped counts;
- review refuses a missing or stale receipt without running tests;
- archive refuses stale receipt before merge;
- archive does not rerun change specs but runs the merged suite once;
- full-suite failure leaves pending candidate and supports rerun;
- init scaffolds a failing sentinel with result guidance;
- helper writes valid JSON atomically.
