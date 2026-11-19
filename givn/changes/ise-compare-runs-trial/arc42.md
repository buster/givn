# Arc42 impact: ise-compare-runs-trial

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The change fixes the verifier's normal command; the project goals are unchanged. |
| 2 | 02 architecture-constraints | Yes | TC-53 now records that `compare` executes the contract by default and that `--inputs-only` is the read-only path. |
| 3 | 03 context-and-scope | Yes | The verifier interaction and the two `ise` command rows now describe default execution, `--result`, and `--inputs-only`. |
| 4 | 04 solution-strategy | Yes | The clean-room strategy row now names `compare` as the normal executor and `verify-trial` as the explicit path. |
| 5 | 05 building-block-view | Yes | `cli::ise` and the `ise` block now render the executed compare path and the inputs-only line. |
| 6 | 06 runtime-view | Yes | Section 6.21's compare step now continues into the Verify trial flow; the verify step removes the previous result first. |
| 7 | 07 deployment-view | No | No deployment topology or runtime host changes. |
| 8 | 08 crosscutting-concepts | Yes | The trial-gate paragraph now states default execution, freshness, and the inputs-only path. |
| 9 | 09 architecture-decisions | Yes | ADR-076 is amended: the 2026-11-18 rejected alternative is superseded by default execution plus `--inputs-only`. |
| 10 | 10 quality-requirements | No | No new quality scenario; the command's exit semantics are already covered by the trial gate. |
| 11 | 11 risks-and-technical-debt | Yes | R-167, R-168, R-169, and R-170 name the executed trial; R-172 records that compare executes copied commands; R-173 records result-path collision safety. |
| 12 | `docs/glossary.md` | Yes | The **Trial result** entry now names both executed paths and the inputs-only form. |

## ADR qualification

- **Architectural commitment**: The verifier command boundary is a durable CLI
  contract that harnesses and external Ise consumers call.
- **Real alternatives**: keep a read-only `compare` plus a separate
  `verify-trial` (the 2026-11-18 decision) versus default execution with an
  explicit read-only `--inputs-only` path.
- **Durable consequence**: reversing the choice again changes the meaning of
  `compare`'s exit 0 and the security posture on an untrusted copy; harnesses
  and documentation build against it.
- **Existing decisions**: ADR-076 already owns the clean-room trial record and
  names the rejected alternative. Route: `AMEND_ADR`; the new amendment records
  the reversal and its security consequence.

## Status

STATUS: DONE
