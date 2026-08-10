# arc42 Documentation Update: coverage-script-workflow

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The project goals are unchanged. | |
| 2 | 02 architecture-constraints | No | No new runtime or deployment constraint is introduced. | |
| 3 | 03 context-and-scope | No | The archive gate remains inside givn's existing boundary. | |
| 4 | 04 solution-strategy | Yes | Coverage publication changes from an internal merge to project-owned scripts. | Commands stay simple and visible in README and `givn/commands.yaml`. |
| 5 | 05 building-block-view | Yes | `coverage_gate` and `Coverage` change responsibilities. | Measurement and merge command execution, report validation, and README rendering are explicit. |
| 6 | 06 runtime-view | Yes | Archive now has a measure-then-merge sequence and transactional report rollback. | The configured merged report becomes the README source. |
| 7 | 07 deployment-view | No | No deployment topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | Configuration, coverage validation, managed README blocks, and rollback change. | New script contract and line/branch badge ownership are documented. |
| 9 | 09 architecture-decisions | Yes | The merge-authority and README ownership decisions are new. | Add decisions for project-owned scripts and authoritative merged output. |
| 10 | 10 quality-requirements | No | No new quality target or percentage threshold is introduced. | |
| 11 | 11 risks-and-technical-debt | Yes | Stale output, mismatched reports, and unmeasured branches need explicit handling. | Add risks and mitigations for script/report drift and zero branch denominators. |
| 12 | 12 glossary | Yes | The terms source report, merged report, measurement command, and merge command gain precise meaning. | Add the coverage workflow vocabulary. |

## Notes

The change keeps legacy two-command coverage configuration readable during
migration, but all new projects use `measure_command` and `merge_command`. The
merged report emitted by the project script is authoritative; givn does not
recompute it for the new path.

---

## Status

STATUS: DONE
