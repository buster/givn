# arc42 Documentation Update: strengthen-ise-specification

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality attribute is added; the change renames a public surface and states its evidence format. | |
| 2 | 02 architecture-constraints | Yes | TC-46, TC-47, TC-49, and TC-51 name the retired commands, the record scope, and the report lines. | The four constraints name `rebuild-all`, `rebuild-component`, and `compare`, the `ise-manifest/v1` manifest, and the new report. |
| 3 | 03 context-and-scope | Yes | The external-interface table and the in-scope list name the retired commands and the `ise-trial/v1` record. | The verifier row and the three command rows name `rebuild-all`, `rebuild-component`, `compare`, and the `ise-manifest/v1` manifest. |
| 4 | 04 solution-strategy | Yes | The clean-room row names the retired commands, `--full`, `--path`, and the record fields. | The row states the two rebuild commands, the spec copy, the manifest, and the comparison. |
| 5 | 05 building-block-view | Yes | The `cli::ise` and `ise` building blocks are described there with the retired commands and record fields. | Both rows name the three commands, the spec copy partition, and the `ise-manifest/v1` manifest. |
| 6 | 06 runtime-view | Yes | §6.21 narrates the prepare and re-check steps with the retired flags and record. | §6.21 narrates the rebuild steps, the manifest write, and the comparison steps under the new commands. |
| 7 | 07 deployment-view | No | No deployment, packaging, or runtime-environment change. | |
| 8 | 08 crosscutting-concepts | Yes | §8.29 owns the clean-room evidence, and the checksum form, the enumeration, and the compatibility rule are stated there. | §8.29 names the three commands, the `ise-manifest/v1` schema, the canonical checksum and scenario form, the revision-versus-checkout enumeration, and the four spec copy rule sentences. |
| 9 | 09 architecture-decisions | Yes | ADR-076 owns the clean-room evidence; the rename and the scope cut refine that decision. | ADR-076 is amended with the command names, the manifest scope, the byte form, the compatibility consequence, and the second-implementation confirmation; the chapter-09 register and the ADR README record the amendment. |
| 10 | 10 quality-requirements | Yes | A second implementation reproducing the manifest's checksums byte for byte is a new quality boundary. | Added Q-127. |
| 11 | 11 risks-and-technical-debt | Yes | The `Rule:`-nested selection limit is the existing D-9; the pinned fixture, the command rename, and the unverified kept Source code are new downsides. | Extended D-9 with the scenario-checksum and runner-alignment limit; added R-160 for the pinned-fixture re-pin cost, R-161 for the retired command names, and R-162 for kept Source code that comparison does not verify. |
| 12 | 12 glossary | Yes | Four trial terms retire, two terms are added, and one existing row gains a contrast. | `Trial record`, `Trial directory`, `Trial E2E digest`, and `Frozen package` are retired; `Spec copy` and `Checksum` are added; `Manifest` gains a contrast naming the spec copy's ephemeral file. |

## ADR qualification

```text
Architectural commitment: yes - the clean-room commands are the product's public
  surface for an Ise Test, and the manifest is a durable compatibility boundary
  read by the verifier. Renaming the commands and cutting the manifest scope fixes
  what every script, every second implementation, and every reader must use.
Real alternatives: keep `replace`/`verify` with the old flags versus three named
  commands versus a hidden alias (proposal D5); read the retired record shape
  versus refuse it by name (proposal D4); print the manifest versus write it only
  with `--manifest` (proposal D1).
Durable consequence: a script or habit that calls the retired names fails with
  exit 2; a manifest written before this change is refused by name and must be
  re-prepared; a second implementation that follows the rule must produce
  byte-identical checksums or fail the golden scenario.
Existing-decision search: ADR-076 owns the clean-room record's shape, placement,
  and scope rule; ADR-078 owns the End-to-end (E2E) package exclusion; ADR-070
  owns the change verification receipt (`change-proof/v1`); ADR-071 owns the SCM
  read. No active or archived record owns the command names or the byte form.
ADR action: AMEND_ADR-076
ADR target: docs/arc42/adr/ADR-076-clean-room-trial-record.md
```

## Notes

The independent check is the load-bearing evidence. The production binary wrote
the manifest for the pinned fixture; a separate Python implementation of the
stated rule read the same fixture bytes and feature text and produced every
value, including a richer feature with a background, a docstring, a table, an
outline, and Examples. The design records both computations, and both fixtures
are pinned by a regular golden scenario. Design-review repeats the independent
computation from the stated rule alone.

`docs/reference.md` is the `contracts` role and
`docs/arc42/02-architecture-constraints.md` is the `limits` role. Editing them
changes the next spec copy's checksums for those paths and its
`specification_checksum`; this change prepares no spec copy. Archive merges the
delta use case and features into the `behavior` role, so the next copy's
checksums change after archive.

`docs/arc42/08-crosscutting-concepts.md`, `docs/arc42/09-architecture-decisions.md`,
`docs/arc42/10-quality-requirements.md`, `docs/arc42/11-risks-and-technical-debt.md`,
and `docs/arc42/adr/**` are outside the declaration (`limits` names only
`docs/arc42/02-architecture-constraints.md`), so this change does not alter the
copy content through them. The retired command names survive only in the ADR's
historical note and in the change artifacts that record the rename.

## Status

STATUS: DONE
