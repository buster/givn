# Review: maki-0-7-support

Review kind: self-review
Author: omp agent (maki-0-7-support author)
Reviewer context: omp agent (maki-0-7-support author)

## Implementation evidence check

- Empty or trivial step bodies: **0** found across the four step files
  (`agent_activity_reporting_steps.rs`, `development_record_steps.rs`,
  `initialize_a_project_steps.rs`, and the shared
  `development_timeline_support.rs` driver support). Every new step performs a
  real action or assertion.
- `@e2e` tag integrity: the delta adds no `@e2e` scenario and removes no tag.
  The permanent Maki row "Consumer installs the agent reporter and its reported
  activity lands in the record" is retained and was executed by the e2e scope
  (104 passed).
- Strict-mode proof: `GIVN_FEATURES=/tmp/strict-probe.feature cargo test
  --locked --test features_runner` exits 1 on one undefined step
  (`Step doesn't match any function`), proving `.fail_on_skipped()`.
- Commits: each checked scenario names its batch revision.

| Batch | Revision | Scenarios | Production files |
|---|---|---|---|
| intake-schema | `a0a1868c` | 6 development-record scenarios | `src/cli/history.rs` |
| reporter | `17673fcf` | 11 agent-activity-reporting scenarios | `assets/agent-integrations/maki/givn-timeline.lua` |
| maki-install | `80742b9e` | 2 initialize-a-project scenarios | `assets/agent-integrations/maki/plugin.toml`, `src/skills/targets.rs`, `setup-e2e-harness.sh` |

Every batch revision contains the scenario implementation or its regression
test; the shared revision is labelled with its batch in `tasks.md`.

Promised components: all exist — the intake additions are in
`src/cli/history.rs`; the reporter helpers/handlers are in the generated Lua
asset; the grant merge is in `src/skills/targets.rs`; the pinned install is in
`setup-e2e-harness.sh`.

Design conformance: `givn/commands.yaml` still declares
`./run-tests.sh --non-e2e` and `./run-tests.sh --e2e`; the new step files match
the design's per-capability paths; the Maki driver still runs the real binary
through `maki --trust prompt` with the real intake shim. No deviation.

### Interaction cross-reference

Inventory entries come from `givn/specs/inspect-development-history/usecase.md`
§Interactions and `givn/specs/initialize-project/usecase.md` §Interactions; the
evidence is the existing `@e2e` scenario or the delta variant.

| Use case / capability | Inventory action | Evidence | Driver |
|---|---|---|---|
| inspect-development-history / development-record | run a lifecycle command | existing `@e2e` "Consumer runs a lifecycle step and reads its recorded actions" | real `givn` CLI |
| inspect-development-history / development-record | read the development record | existing `@e2e` "Consumer reads the development record as machine-readable output"; delta intake scenarios | real `givn` CLI / `history ingest` |
| inspect-development-history / agent-activity-reporting | report agent activity from a supported harness | existing `@e2e` Maki row; delta reporter scenarios | real Maki binary + real intake |
| inspect-development-history / development-report | generate the development report for a change | existing `@e2e` | real `givn timeline` |
| inspect-development-history / development-report | open the report in a browser | existing `@e2e` | Playwright |
| inspect-development-history / development-report | read a change's linked ideation | existing `@e2e` | Playwright |
| inspect-development-history / development-report | open the report of a scope with no recorded activity | existing `@e2e` | Playwright |
| inspect-development-history / development-report | expand a workflow phase | existing `@e2e` | Playwright |
| inspect-development-history / development-report | select a timeline element | existing `@e2e` | Playwright |
| inspect-development-history / development-report | zoom the timeline | existing `@e2e` | Playwright |
| initialize-project / initialize-a-project | select agent targets in the interactive terminal | existing `@e2e` | PTY |
| initialize-project / initialize-a-project | initialize with an explicit noninteractive target selection | existing `@e2e`; delta manifest scenarios | real `givn init` |
| initialize-project / initialize-a-project | resynchronize selected targets | existing `@e2e`; delta legacy-manifest scenario | real `givn skills sync` |
| initialize-project / enable-optional-addons | enable the outline addon | existing `@e2e` | real `givn addons` |

No inventory action is unmapped and no extra `@e2e` scenario was added.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Maki host constraint 0.7.0 | 02 | affected | Verified facts | setup/batches | pinned script + reporter probes | yes |
| Reporter responsibilities | 05 | affected | Architecture Impact | reporter batch | Lua asset | yes |
| Capture order and span close | 06 | affected | Span correlation | reporter batch | reporter handlers | yes |
| Pinned test tool | 07 | affected | Technology Decisions | setup | `setup-e2e-harness.sh` | yes |
| Recorder content | 08 | affected | Data Model Changes | intake + reporter | `history.rs`, Lua | yes |
| R-90 re-verification | 11 | affected | Decisions D5 | batches | risk row updated | yes |
| New terms | 12 | affected | Decisions | reporter/install | glossary rows | yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language

The change uses the durable terms "backgrounded call", "session point entry",
"subagent lane", "reported cost", and "session read grant", all added to
`docs/glossary.md`. The structural model (`docs/domain-model.md`) is unchanged:
no entity, boundary, or relationship changed; only the record's payload shape
gained optional fields inside the existing development-record contract.

## Coverage classification

- Dead code: the old `role = "tool"` transcript lookup was deleted; no dead
  branches remain in the changed helper.
- Missing test coverage: none after the delta scenarios (19, all passing). Each
  boundary row with an artifact-visible consequence has a scenario; the
  remaining rows (missing transcript, host without UI) are covered by the
  degrade assertions in the reporter scenarios and the existing best-effort
  scenarios.
- Legitimately hard to test: none claimed. Older-Maki inertness is a manual
  verification recorded in D5/R-90, not a suite test, because the suite pins one
  binary; that is a documented evidence limit, not an untested product path.

## Personas

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role

- `developer--primary-user`: can read one subagent span instead of two, and see
  answers, lane names, and honest background outcomes. Disposition: satisfied.
- `agent--technical-role`: receives accurate spans, bounded lane synthesis, and
  no duplicate entries. Disposition: satisfied.

## Visual review

The design declares a web interface for the existing development report. The
delta adds no `@e2e` scenario and no layout, control, or style change; the only
rendered difference is a lane name derived from an existing field. The
permanent Maki e2e scenario's committed screenshot pair is unchanged and the
e2e scope (104 passed) re-rendered the report. No new visual evidence is
required for a delta with no `@e2e` scenario.

VISUAL-REVIEW: PASS

## Overlap dispositions

No overlap findings were reported for the delta scenarios.

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

## README impact decision

README-IMPACT: none

The change adds no command, flag, or configuration key; the public face already
names Maki as a supported target.

## Deferrals

None.

## Sign-off checklist

- Fabrication audit: clean.
- Every checked task has a verified commit touching production code.
- Every promised component exists.
- Every consumer uses its declared contract.
- Strict-mode proof present and passing.
- Scenario execution evidence is recorded in the verification receipt.
- Coverage classification resolved.
- Dead code deleted; missing tests added; no hard-to-test gaps claimed.
- No `@wip` tags remain; no implementation detail in the specs.
- Canonical E2E policy applied; every inventory action has one `@e2e` scenario.
- Real-interface primary assertions retained.
- The local run command starts the full stack (no server/database introduced).

REVIEW: PASS
