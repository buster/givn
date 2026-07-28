# arc42 Documentation Assessment: bubblewrap-sandbox

| # | Row description | Affected? | Chapter | Reason / Summary |
|---|---|---|---|---|
| 1 | Goals, stakeholders, quality attributes | No | 01, 10 | Same goal (confined agent execution). No new stakeholders, no new quality requirements. |
| 2 | Constraints | Yes | 02 | New technical constraint: bubblewrap >= 0.11.0 must be installed; user namespaces must be enabled. Added constraint entries and conventions. |
| 3 | External systems, interfaces, context | Yes | 03 | bubblewrap is a new external dependency. Added business context, technical context diagram with bwrap as interface, system boundary description. |
| 4 | Solution strategy | Yes | 04 | Sandboxing strategy changed from sandlock to bubblewrap. Added key decisions, technology choices, quality goal approaches. |
| 5 | Building blocks | Yes | 05 | sandbox.py and runner.py modified; new `check_bubblewrap` and `build_bwrap_cmd` functions. Added level 1 system overview diagram and level 2 block details. |
| 6 | Runtime view | Yes | 06 | Agent execution flow changed (bwrap wraps the agent command). Added sequence diagram for happy path, steps, unsandboxed fallback scenario. |
| 7 | Deployment | Yes | 07 | bwrap must be installed on deployment host. Added infrastructure topology diagram, host and Python runtime detail tables. |
| 8 | Cross-cutting concepts | Yes | 08 | Process sandboxing is a cross-cutting concern. Added sandbox mechanism description, availability check, configuration, error handling, observability. |
| 9 | Architecture decisions | Yes | 09 | ADR-0001: Use bubblewrap for process sandboxing (MADR format). Compared bubblewrap, firejail, and keeping sandlock. |
| 10 | Quality requirements | No | 10 | No new quality requirements introduced by this change. |
| 11 | Risks and technical debt | Yes | 11 | 7 new risks (user namespace disabled, bwrap missing, ro-bind path absent, agent writes outside workspace, disk quota, port conflicts, bwrap API change). 5 technical debt items (env var name, static whitelist, resource limits, heuristic error detection, no learn-then-confine). |
| 12 | Glossary | Yes | 12 | New terms: bubblewrap, bind mount, user namespace, sandlock, COW, systemd-run, ralph, opencode, givn, termination, workspace. |

## Status

STATUS: DONE