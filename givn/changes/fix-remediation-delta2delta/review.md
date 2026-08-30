# Review: fix-remediation-delta2delta

## Coverage

The delta spec scenario-by-scenario RED/GREEN loop implemented a delta-to-delta
remediation branch. See tasks.md for per-scenario commit evidence.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A DUPLICATE delta-to-delta classification without removal remains blocked` | `A VARIANT delta-to-delta classification without consolidation remains blocked` | boundary |
| `A DUPLICATE delta-to-delta classification without removal remains blocked` | `A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked` | boundary |
| `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A VARIANT delta-to-delta classification without consolidation remains blocked` | `A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked` | boundary |
| `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A binary without retrieval reports the explicit unavailable mode` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Detail mode — unknown change errors` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Uninitialized project errors` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Check tasks blocked when delta spec has @wip` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Id with spaces is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Id with non-ASCII letters is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Id with a dot is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Id ending with a hyphen is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Review blocks with a clear message when review.md is missing` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A feature-free build keeps retrieval explicitly unavailable` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A complete classification allows change review and survives regeneration` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `A complete classification allows change review and survives regeneration` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `A complete classification allows change review and survives regeneration` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `A complete classification allows change review and survives regeneration` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `Every resolved semantic classification requires a rationale` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `Removed candidates disappear after classification` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `Removed candidates disappear after classification` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `Removed candidates disappear after classification` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `Removed candidates disappear after classification` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A DUPLICATE delta-to-delta classification without removal remains blocked` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A VARIANT delta-to-delta classification without consolidation remains blocked` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A DUPLICATE delta-to-delta classification without removal remains blocked` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A VARIANT delta-to-delta classification without consolidation remains blocked` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A VARIANT score below the boundary without consolidation remains blocked` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A DUPLICATE delta-to-delta classification without removal remains blocked` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A VARIANT delta-to-delta classification without consolidation remains blocked` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Changing a classified endpoint before remediation makes the decision stale` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Removing the redundant delta-to-delta scenario verifies duplicate remediation` | boundary |
| `Real CLI review reports successful duplicate remediation` | `A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Real CLI review verifies a delta-to-delta duplicate remediation` | boundary |
| `The audit reports retrieval unavailable without the retrieval feature` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `The audit reports an unavailable retrieval model` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Invalid Gherkin input is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Multiple scenarios in one input are rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A valid feature without a scenario is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `A missing draft file is rejected` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `File and standard input cannot be selected together` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Explore remains unavailable without the retrieval feature` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by status` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by archive` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by check` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by lint` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by graph` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
| `Path traversal id is rejected by instructions` | `A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked` | boundary |
