# Arc42 Impact Assessment: run-command

Assessment of every chapter against the change's
[proposal](proposal.md), [specs](specs/run-command.feature) +
[run-execution specs](specs/run-execution.feature), and
[design](design.md). Affected chapters were updated surgically; a new ADR was
created only after passing the qualification gate.

| # | Chapter(s) | Affected | Reason / what changed |
|---|---|---|---|
| 1 | 01 introduction-and-goals, 10 quality-requirements | Yes | FR-41–FR-44 added (run declaration, gates/backfill, `givn run` executor, no-derivation boundary); automation callers added as stakeholders; quality scenarios Q-69–Q-72 added. |
| 2 | 02 architecture-constraints | Yes | TC-25 (unix-only signal forwarding, direct `libc` dependency) and TC-26 (no derivation of run content from repository content) added; gate lists updated for the run-declaration check and its archive position. |
| 3 | 03 context-and-scope | Yes | Started product and automation callers (kiekste) added as external entities; `givn/commands.yaml` and `givn run` interface contracts added; in-scope/out-of-scope updated (no guessing, no daemon, no health monitoring). |
| 4 | 04 solution-strategy | Yes | "Product start" strategy row added (declaration + execution, ADR-067); mandatory review-check list extended with the run-declaration check. |
| 5 | 05 building-block-view | Yes | New building blocks `cli::run_block`, `cli::run`, `cli::missing_run_command`; `config::manifest` responsibility extended; `cli::archive`, `cli::readme`, `cli::upgrade`, `cli::migration` rows updated; crate-structure diagram gains the run node; `ready_fixture` documented as a test building block. |
| 6 | 06 runtime-view | Yes | New §6.15 `givn run` with a Mermaid sequence (preflight → port resolution → spawn → probe → ready line → signals/exit codes); §6.13 records the idempotent backfill in the managed upgrade; §6.14 records the ready-fixture role. |
| 7 | 07 deployment-view | No | Deployment is unchanged: same single binary, same install path. `libc` is already compiled in every build configuration (lockfile-verified), so the direct dependency adds no deployment artifact. |
| 8 | 08 crosscutting-concepts | Yes | New §8.16: run-declaration data placement and replace-whole merge, validation rules R1–R10 table with skip logic, gate positions, README managed run block, `givn run` machine contract (ready line, exit codes, probe, signals). |
| 9 | 09 architecture-decisions, adr/ | Yes | ADR-067 created (`proposed`, owned by this change) after the qualification gate; ADR-028 amended in prose (run-declaration gate joins the mandatory-check boundary); register and index updated. |
| 10 | 10 quality-requirements | Yes | Q-69–Q-72 (ready-line contract, gate determinism, backfill idempotency, ephemeral-port hand-off). |
| 11 | 11 risks-and-technical-debt | Yes | R-41 (ephemeral-port TOCTOU), R-42 (unix-only signal forwarding), R-43 (declaration spoofing left deliberately uncountered inside givn), R-44 (compound-command grandchildren survive forwarding), R-45 (the `givn run` machine contract becomes a public compatibility obligation). |
| 12 | 12 glossary | Yes | Run-declaration vocabulary recorded (run declaration, product type, start command, sentinel start command, run finding, run gate, run managed block, port hand-off, ephemeral port, readiness path/probe/timeout, ready line, start plan, backfill scaffold, executor) plus the run-declaration invariants. |

## ADR qualification verdicts

| Candidate decision | Alternatives | Architectural impact | Durable consequence | Lower-level artifact? | Existing-ADR check | Verdict |
|---|---|---|---|---|---|---|
| One tool declares and executes the product start | declaration-only (superseded source-doc position); separate companion tool | new CLI surface + process-supervision subsystem + machine contract | givn's public contract grows to include start execution; callers build on it | no — the boundary is not expressible as a scenario | no existing ADR covers product execution (ADR-015 is verification, ADR-028 is gates) | **NEW_ADR → ADR-067 (proposed)** |
| `run:` block in `givn/commands.yaml`, not `config.yaml` | `config.yaml` | file charter only | applies the existing commands-file charter | yes — follows the existing single-source-of-commands boundary (ADR-015 analog) | no conflict | CANONICAL_ARTIFACT → design.md + §8.16 |
| Archive run gate independent of the readme addon | enforce only via README placeholder | gate pipeline order | projects without the readme addon are still blocked | partially — but it extends ADR-028's existing mandatory-gate boundary rather than creating one | ADR-028 owns the boundary | AMEND_ADR → recorded as an ADR-028 amendment in ch.09 |
| Ephemeral port policy | always require explicit port; retry allocation | executor implementation | low | yes — design/mechanics | no | CANONICAL_ARTIFACT → design.md §7.1 + ch.11 R-41 |
| Signal forwarding inside givn (libc, unix-only) | no forwarding (kill-only); external wrapper | executor implementation | platform constraint only | yes — design/mechanics | no | CANONICAL_ARTIFACT → design.md §7.4 + TC-25 + ch.11 R-42 |

## Status

STATUS: DONE
