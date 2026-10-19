# Real model runs

Two evidence classes are distinct. Do not compare them as if they were the
same.

| Class | Egress | Manifest | Use |
|---|---|---|---|
| Controlled | Enforced outside the harness, for example a VM/container firewall allowing only `api.deepseek.com:443` | `execution_class: controlled` | Comparisons and givn change decisions |
| Exploratory | Host networking; no verified allowlist | `execution_class: exploratory`, `network_allowlist: unverified` | Loop validation, integration debugging, hypotheses |

## Preflight

1. Pin the givn binary and record its hash:

   ```sh
   /path/to/pinned/givn --version
   sha256sum /path/to/pinned/givn
   ```

2. Put the DeepSeek key in a private file outside the repository:

   ```sh
   umask 077
   printf '%s' "$DEEPSEEK_API_KEY" > /tmp/opencode/deepseek-key
   ```

   The harness injects it as `DEEPSEEK_API_KEY` inside the sandbox. It is not
   written to the manifest, journal or prompt.

3. Verify the price table is dated and not a placeholder:

   ```sh
   cat config/prices.toml
   ```

4. Decide the budget. The authorized first-loop cap is USD 5. The ledger
   reserves before dispatch and keeps uncertain charges after a crash. It is
   not a provider-side invoice ceiling.

## Exploratory run

```sh
uv run givn-harness run \
  --benchmark benchmarks/lending \
  --givn-binary /path/to/pinned/givn \
  --output runs/exploratory \
  --condition exploratory \
  --repeats 1 \
  --driver opencode \
  --network host \
  --allow-unverified-network \
  --sandbox required \
  --budget-usd 5 \
  --price-table config/prices.toml \
  --deepseek-api-key-file /tmp/opencode/deepseek-key \
  --max-output-tokens 8192
```

For a first integration check, stop after the initial stage:

```sh
  --stop-after initial
```

The follow-up is recorded as `not_reached`; the initial stage is still
snapshotted, analyzed and gradable.

## Controlled run

Run the same command inside a VM or container whose outbound firewall is
outside the agent's control and permits only the provider endpoint. Keep
`--allow-unverified-network` because the harness itself still shares the VM's
host network; the VM supplies the boundary. Record the firewall configuration
and VM image next to the bundle.

## OpenCode isolation

The harness creates a private XDG tree per trial:

```text
<workspace>/.harness/opencode/config/opencode/opencode.json
<workspace>/.harness/opencode/data
<workspace>/.harness/opencode/cache
<workspace>/.harness/opencode/state
```

The generated config allows only the `deepseek` provider and pins
`deepseek/deepseek-flash#max`. It also sets `"snapshots": false`. OpenCode's
internal git snapshot service is not needed for evaluation, and with the
private data directory inside the workspace it can recursively index its own
growing state, consuming high CPU and disk. The workspace `.gitignore` contains
`.harness/` as a second guard. Do not re-enable OpenCode snapshots in an
evaluation workspace.

The host OpenCode config, plugins, cache, sessions and credential database are
not mounted. Only the OpenCode runtime at `~/.opencode` is mounted read-only.

The driver always uses `--standalone`, so it does not attach to the shared
background server.

## After the run

```sh
uv run givn-harness grade runs/exploratory/<bundle> --benchmark benchmarks/lending
```

Read:

```text
<bundle>/manifest.json
<bundle>/spend.json
<bundle>/journal.jsonl
<bundle>/stages/initial/snapshot.json
<bundle>/stages/initial/stage.json
<bundle>/stages/initial/events.jsonl
<bundle>/stages/initial/analysis.md
<bundle>/grades/<grade-id>/grade.md
```

A completed run is not automatically a successful delivery. Check the
requirement-level grade, stage completion and any `ungraded` entries
separately.
