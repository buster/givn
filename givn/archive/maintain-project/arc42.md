# arc42 Documentation Update: maintain-project

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality-attribute statement changes; the archiving-developer expectation already covers both backends. | — |
| 2 | 02 architecture-constraints | Yes | TC-21 scoped the managed upgrade to Git. | TC-21 now states both backends with the same guarantees; TC-22 and TC-33 already hold backend-neutrally. |
| 3 | 03 context-and-scope | Yes | The Fossil marker entity and the in-scope list still said the upgrade was Git-only. | The Fossil CLI entity and the in-scope bullet now cover the managed upgrade. |
| 4 | 04 solution-strategy | Yes | The SCM backend strategy row said the managed upgrade consumes Git only. | The strategy row now names both backends for the upgrade; ADR-071 amendment noted. |
| 5 | 05 building-block-view | Yes | `cli::migration` and `fossil` rows encoded the temporary refusal. | Both rows describe the implemented Fossil staging/commit operations. |
| 6 | 06 runtime-view | Yes | 6.13 recorded the Fossil refusal. | 6.13 describes the backend preflight, allowlist staging (`git add [-f]` / `fossil add [--force]`), and seam commits. |
| 7 | 07 deployment-view | No | The backend-executable prerequisite (7.3) already covers both backends. | — |
| 8 | 08 crosscutting-concepts | Yes | 8.18 said the upgrade refuses Fossil. | 8.18 now records the Fossil allowlist staging and the shared consumption. |
| 9 | 09 architecture-decisions | Yes | The upgrade-on-both-backends decision refines ADR-071. | Chapter 09 prose records the amendment; `adr/README.md` gains the relationship row; no new record (A12). |
| 10 | 10 quality-requirements | No | Q-48/Q-49/Q-50/Q-57 already describe backend-neutral guarantees; their scenarios gain Fossil evidence without wording changes. | — |
| 11 | 11 risks-and-technical-debt | Yes | D-20 (Fossil upgrade not implemented) is repaid. | The D-20 row is removed. |
| 12 | 12 glossary | Yes | The closing note said the upgrade was still refused. | The note now says the archive and the upgrade run on Fossil; attribution stays a later seed. |

## Notes

The embedded 0.2.0-to-0.3.0 migration guide and bundle assets keep their Git
wording; the proposal defers catalog content changes, so a Fossil plan quotes
the same guide (A13 in the topic assumptions).

```text
ADR action: AMEND_ADR-071
ADR target: ADR-071
```

## Status

STATUS: DONE
