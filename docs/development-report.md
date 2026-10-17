# Development record and report

Givn keeps a local journal of the work it observes and turns it into one
self-contained HTML report. This guide covers the record settings, the report
screens, and analytics over the raw JSONL store.

The record is **local runtime state**, written to `givn/history/events.jsonl`
and ignored by both backends. It never enters repository history. The report is
one HTML file with no server, no network access, and no third-party JavaScript.

## Screens

The report of a change reads top to bottom: the scope and current state, six
KPI tiles, the workflow phase stack, then the statistics. Opening a phase
reveals its vertical time axis, one lane per recorded session side by side, and
the selected lane's log. Every phase has a Fit / 1 h / 15 m zoom. The linked
ideation group, when the change's proposal or seed names an existing topic,
sits above the change workflow with its own ideation phases and totals.

<p align="center">
  <img src="development-report-desktop.png" width="900" alt="Desktop development report: current state, KPI tiles, and the nine workflow phases with their durations, lanes, and statistics.">
</p>

<p align="center"><em>Desktop: current state, KPI tiles, the workflow stack, and each phase's lanes.</em></p>

<p align="center">
  <img src="development-report-mobile.png" width="340" alt="Mobile development report: current state, two-column KPI tiles, and the workflow phase stack.">
</p>

<p align="center"><em>Mobile: one column, two-column KPIs, full-width phase cards, and lanes scrolling horizontally inside a card.</em></p>

The screenshots use a demonstration change with seeded record data; the layout
and interactions are the product's own.

## Generate and read the report

```sh
givn timeline --change <id>            # one change
givn timeline --topic <id>             # one ideation topic
givn timeline                          # project report
givn timeline --change <id> --out path/to/report.html
```

The command writes `givn/history/reports/<scope>.html` by default, prints the
path, opens nothing, and records nothing. The file is self-contained: open it
directly in a browser; there is no port and no network request. A report of an
archived change renders without a live state; a linked topic with no recorded
entries states that instead of inventing phases.

The current state in the header is the change's derived state (stage label,
current workflow phase, next required artifact, task progress) at render time.
The topic report uses the ideation workflow (`frame`, `personas`,
`event-storming`, `use-case-definition`, `use-case-elaboration`, `handoff`).

## Settings

The capture control lives in `givn/config.yaml`:

```yaml
record:
  mode: on        # on (default) | off | full
```

| Mode | What is recorded | Notes |
|---|---|---|
| `on` (default) | Bounded metadata and the full question text: commands, phases, hooks, steps, revisions, turns, tool calls, subagent spans, waits, usage, cost, context, and reported metadata. Labels ≤ 120 chars, text metadata ≤ 240, error text ≤ 1,000, question text ≤ 4,000. No LLM-facing content. | The default when the key is absent or unreadable; an invalid value falls back to `on` with one warning. |
| `off` | Nothing new: no entry, no pending state, and no record-side ignore write. | Absolute: `givn history ingest --with-full-content` is ignored with one warning and exit 0. |
| `full` | `on` plus complete LLM-facing content: tool input/output, assistant/thinking text, user prompts, compaction summaries, plan content, subagent transcript excerpts. | Capped at 64 KiB per field and 256 KiB per entry; the store keeps every byte. Content is sensitive: enable it deliberately. |

Lowering the mode keeps entries already captured; it never purges. `givn
upgrade` preserves the key. The intake enforces the mode and every bound, so a
stale reporter cannot turn content capture on by sending content.

Inspect the resolved setting:

```sh
givn history settings          # record mode: on (source: default)
givn history settings --json   # {"mode":"on","source":"default"}
```

`source` is `config` when `record:` names a valid mode and `default` otherwise.
The query is a read view: it appends no record entry. Reporters ask the same
question once per session and treat a failed query as `on`.

The report command's own settings are its flags: `--change`, `--topic`, and
`--out` (default `givn/history/reports/<scope>.html`). Reading the report and
the record (`givn history`, `givn history --json`) never records.

The record contains, per line: `schema`, `id`, `at` (UTC), `scope`
(`change|ideation|project` + id), `source` (`givn|agent`), `kind`, `name`,
`label`, `duration_ms` (may be absent), `outcome`, `session` (nullable),
`parent`, and `data` (kind-specific facts such as `usage`, `cost`,
`context_size`, `tool_call_id`, `error`, `agent/subagent`, `question`, and
`content` only in `full`).

## Analytics on the JSONL store

The store is one JSON object per line at `givn/history/events.jsonl`. It is
append-only and grows as commands run; treat every output below as a snapshot.
**Never commit the store.**

Before summing, note four traps:

- `duration_ms` is measured; absent is not zero.
- Durations overlap: a turn overlaps its tool calls and subagent spans. Sum one
  kind, or use a phase's wall time — never all kinds together.
- The recorder records its own intake invocations (`history ingest`,
  `history harness-hook`). Exclude them for development analytics; the report
  does.
- Token fields are per turn. `usage.input`/`output` are per-turn counts while
  `usage.total_tokens`/`context_size` repeat the prompt context each turn; the
  report's token tile is a cumulative total across turns, so it is not the same
  as unique tokens. Pick the field that answers the question.

### jq

```sh
# entries and measured seconds per kind
jq -s 'group_by(.kind)
  | map({kind:.[0].kind, n:length, s:((map(.duration_ms // 0)|add)/1000|floor)})
  | sort_by(-.n)' givn/history/events.jsonl

# longest measured steps, recorder excluded
jq -s 'map(select(.duration_ms != null
    and ((.kind=="command" and ((.label//"")|test("^history (ingest|harness-hook)\\b"))) | not)))
  | sort_by(-.duration_ms) | .[:8]
  | .[] | [(.duration_ms/1000), .kind, (.name//""), ((.label//"")[0:55]), .scope.id]
  | @tsv' -r givn/history/events.jsonl

# where command time goes
jq -s 'map(select(.kind=="command"
    and ((.label//"")|test("^history (ingest|harness-hook)\\b")|not)))
  | group_by(.name)
  | map({name:.[0].name, n:length, s:((map(.duration_ms//0)|add)/1000|floor)})
  | sort_by(-.n) | .[:6]' givn/history/events.jsonl

# per-turn token work and cost (usage can be the string "unavailable")
jq -s 'map(select(.kind=="turn")) | {
  turns: length,
  input:  [.[]|(.data.usage|if type=="object" then (.input // 0) else 0 end)]|add,
  output: [.[]|(.data.usage|if type=="object" then (.output // 0) else 0 end)]|add,
  reasoning: [.[]|(.data.usage|if type=="object" then (.reasoning // 0) else 0 end)]|add,
  cost: [.[]|(.data.cost|if type=="object" then (.total // 0) else (. // 0) end)]|add
}' givn/history/events.jsonl
```

Example snapshot (8,917 entries at the time of writing; the store grows as commands run):

```text
kind       n     s
command    2421  512
tool       2296  8855
turn       2074  17454
usage      2074  0
subagent   25    8358
phase      16    11
step       10    409
question   1     137

963.4  turn      turn 11
963.2  subagent  HybridBundleO
961.0  tool      wait
812.1  subagent  HybridDesignReview
737.3  turn      turn 21
737.3  subagent  HybridBundleQ

commit        164  15.9 s
attribution    97   1.1 s
status         31   0.9 s

turns 2075 · input 5,013,065 · output 1,440,883 · reasoning 684,872 · cost $2.56
```

### SQLite

Import once. The `\x01` separator cannot occur in JSON, so labels containing
`|` survive the one-column import:

```sh
rm -f /tmp/givn.db
sqlite3 /tmp/givn.db 'CREATE TABLE raw(line TEXT);'
printf '.separator \001\n.import givn/history/events.jsonl raw\n' | sqlite3 /tmp/givn.db
```

```sh
sqlite3 /tmp/givn.db <<'SQL'
CREATE VIEW ev AS
SELECT json_extract(line,'$.id')          AS id,
       json_extract(line,'$.at')          AS at,
       json_extract(line,'$.scope.kind')  AS scope_kind,
       json_extract(line,'$.scope.id')    AS scope_id,
       json_extract(line,'$.kind')        AS kind,
       json_extract(line,'$.name')        AS name,
       json_extract(line,'$.label')       AS label,
       json_extract(line,'$.duration_ms') AS duration_ms,
       json_extract(line,'$.outcome')     AS outcome,
       json_extract(line,'$.session')     AS session,
       json_extract(line,'$.data')        AS data
FROM raw WHERE json_valid(line);
SQL
```

```sql
SELECT kind, count(*) n, round(sum(coalesce(duration_ms,0))/1000.0,1) s
FROM ev GROUP BY kind ORDER BY n DESC;

SELECT round(duration_ms/1000.0,1) s, kind, name, substr(label,1,55) label
FROM ev
WHERE duration_ms IS NOT NULL
  AND NOT (kind='command'
           AND (label LIKE 'history ingest%' OR label LIKE 'history harness-hook%'))
ORDER BY duration_ms DESC LIMIT 8;

SELECT name, count(*) n, round(sum(coalesce(duration_ms,0))/1000.0,1) s
FROM ev
WHERE kind='command'
  AND NOT (label LIKE 'history ingest%' OR label LIKE 'history harness-hook%')
GROUP BY name ORDER BY n DESC LIMIT 6;

SELECT count(*) waits, round(sum(coalesce(duration_ms,0))/1000.0,1) s
FROM ev WHERE kind='question';

SELECT count(*) turns,
       sum(CAST(json_extract(data,'$.usage.input')     AS INTEGER)) input,
       sum(CAST(json_extract(data,'$.usage.output')    AS INTEGER)) output,
       sum(CAST(json_extract(data,'$.usage.reasoning') AS INTEGER)) reasoning
FROM ev WHERE kind='turn' AND json_type(data,'$.usage')='object';
```

The view yields the same snapshot as the jq block; for the same data it prints
`command 2421/512`, `tool 2296/8855`, `turn 2074/17454`, one wait of `137.6 s`,
and `2075` turns with `5,013,065` input and `1,440,883` output tokens. Re-import into a fresh database, or `DELETE FROM raw;` and import
again, to refresh a snapshot.

### DuckDB

DuckDB reads the JSONL directly; the 2.0 alpha exposes `read_json_auto` with
`format='newline_delimited'` and infers `scope` as a struct and `usage` as JSON.
`at` is a reserved word in DuckDB 2.0, so quote it as `"at"`. Add `-no-agent`
in scripts to suppress the alpha's agent-mode banner.

```sh
# install (2.0 alpha)
curl https://install.duckdb.org | DUCKDB_VERSION=alpha bash

# direct query, no import
duckdb -c "SELECT count(*) FROM read_json_auto('givn/history/events.jsonl', format='newline_delimited');"

# or persist for repeated analysis
duckdb /tmp/givn-history.duckdb -c "
  CREATE OR REPLACE TABLE ev AS
  SELECT * FROM read_json_auto('givn/history/events.jsonl', format='newline_delimited');"
```

```sql
SELECT kind, count(*) n, round(sum(coalesce(duration_ms,0))/1000.0,1) s
FROM ev GROUP BY kind ORDER BY n DESC;

SELECT "at", round(duration_ms/1000.0,1) s, kind, name, left(label,55) label
FROM ev
WHERE duration_ms IS NOT NULL
  AND NOT (kind='command'
           AND (label LIKE 'history ingest%' OR label LIKE 'history harness-hook%'))
ORDER BY duration_ms DESC LIMIT 8;

SELECT sum(TRY_CAST(data.usage->>'input'     AS BIGINT)) input,
       sum(TRY_CAST(data.usage->>'output'    AS BIGINT)) output,
       sum(TRY_CAST(data.usage->>'reasoning' AS BIGINT)) reasoning
FROM ev WHERE kind='turn' AND json_type(data.usage)='OBJECT';

SELECT scope.id, label, "at",
       date_diff('second', lag("at") OVER (PARTITION BY scope.id ORDER BY "at"), "at") AS s_since_prev
FROM ev WHERE kind='phase' ORDER BY scope.id, "at";

SELECT kind, left(label,42) label, count(*) n
FROM ev
WHERE kind IN ('tool','command') AND label IS NOT NULL
  AND NOT (kind='command' AND (label LIKE 'history ingest%' OR label LIKE 'history harness-hook%'))
GROUP BY kind,label HAVING n >= 5 ORDER BY n DESC LIMIT 6;
```

The same snapshot returns the same kind table and token totals; the phase query
shows the transition instants and the gap to the previous transition:

```text
development-timeline  proposal.md      2026-10-17 11:59:46.56+02  NULL
development-timeline  design-review.md 2026-10-17 11:59:46.56+02  0
development-timeline  specs            2026-10-17 11:59:46.56+02  0
development-timeline  tasks.md         2026-10-17 12:42:33.994+02  2567
development-timeline  review           2026-10-17 14:12:49.985+02  5416

command  attribution stamp --message-file .git/COMMIT_…  97
tool     givn/changes/development-timeline/tasks.md      90
tool     src/cli/timeline.rs                             72
tool     export GIVN_ATTRIBUTION_ACTIVE='1' …            63
tool     givn/changes/development-timeline/design.md     60
tool     cd /home/buster/projects/givn && GIVN_FEATURES…  55
```

Choose by task: `jq` for ad-hoc questions over the whole file; SQLite for a
persistent local database and joins; DuckDB for direct JSONL analytics, larger
records, window functions, and Parquet export (`COPY ev TO 'ev.parquet'`).
While a change is in flight its record is inside the same file; filter on
`scope.kind`/`scope.id`, and exclude the recorder's own commands for work
analytics.

## See also

- [Architecture, chapter 8.25](arc42/08-crosscutting-concepts.md) — the record's ownership, privacy bound, and report packaging.
- [Design system](design/design-system.md) — the report's tokens, primitives, breakpoints, and reference views.
- [CLI and configuration reference](reference.md) — command flags and every configuration key.
