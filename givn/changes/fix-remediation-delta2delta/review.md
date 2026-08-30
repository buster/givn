# Review: fix-remediation-delta2delta

## Coverage

The delta spec scenario-by-scenario RED/GREEN loop implemented a delta-to-delta
remediation branch. See tasks.md for per-scenario commit evidence.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|


| `A DUPLICATE classification left unresolved keeps the first change blocked` | `A VARIANT pairing stays blocked until the ending is consolidated away` | boundary |
| `A DUPLICATE classification left unresolved keeps the first change blocked` | `A boundary pairing without rewording still fails review on the first change` | boundary |
| `Removing the duplicated endpoint afterwards clears the gate for the first change` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `Removing the duplicated endpoint afterwards clears the gate for the first change` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Removing the duplicated endpoint afterwards clears the gate for the first change` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `Once reworded apart both delta endpoints may survive the first-change gate` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Once reworded apart both delta endpoints may survive the first-change gate` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `A VARIANT pairing stays blocked until the ending is consolidated away` | `A boundary pairing without rewording still fails review on the first change` | boundary |
| `Folding the variant body into a Scenario Outline passes the first-change gate` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `A binary without retrieval reports the explicit unavailable mode` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Detail mode — unknown change errors` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Uninitialized project errors` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Integrity hook fails on Step Definitions section that is not a table` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Check tasks blocked when delta spec has @wip` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `givn addons enable errors when givn/ is not initialised` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `givn addons disable errors when givn/ is not initialised` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Id with spaces is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Id with non-ASCII letters is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Id with a dot is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Id ending with a hyphen is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Review blocks with a clear message when review.md is missing` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A feature-free build keeps retrieval explicitly unavailable` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A complete classification allows change review and survives regeneration` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `A complete classification allows change review and survives regeneration` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `A complete classification allows change review and survives regeneration` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `A complete classification allows change review and survives regeneration` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `Every resolved semantic classification requires a rationale` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `A regenerated classified report is byte-for-byte deterministic` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `Removed candidates disappear after classification` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `Removed candidates disappear after classification` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `Removed candidates disappear after classification` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Removed candidates disappear after classification` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A DUPLICATE classification left unresolved keeps the first change blocked` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A VARIANT pairing stays blocked until the ending is consolidated away` | boundary |
| `A DUPLICATE classification without removal remains blocked` | `A boundary pairing without rewording still fails review on the first change` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A DUPLICATE classification left unresolved keeps the first change blocked` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A VARIANT pairing stays blocked until the ending is consolidated away` | boundary |
| `A VARIANT classification without consolidation remains blocked` | `A boundary pairing without rewording still fails review on the first change` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Consolidating a VARIANT candidate verifies the projected remediation` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `A VARIANT score below the boundary without consolidation remains blocked` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A VALID-BOUNDARY score below the boundary without distinction remains blocked` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A DUPLICATE classification left unresolved keeps the first change blocked` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A VARIANT pairing stays blocked until the ending is consolidated away` | boundary |
| `A VALID-BOUNDARY classification above the boundary remains blocked` | `A boundary pairing without rewording still fails review on the first change` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `A VALID-BOUNDARY score exactly at the boundary remains blocked` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Changing a classified endpoint before remediation makes the decision stale` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Removing the duplicated endpoint afterwards clears the gate for the first change` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Once reworded apart both delta endpoints may survive the first-change gate` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Folding the variant body into a Scenario Outline passes the first-change gate` | boundary |
| `Real CLI review reports successful duplicate remediation` | `Real CLI review verifies a duplicate remediated within the first change` | boundary |
| `The audit reports retrieval unavailable without the retrieval feature` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `The audit reports an unavailable retrieval model` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Invalid Gherkin input is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Multiple scenarios in one input are rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A valid feature without a scenario is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `A missing draft file is rejected` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `File and standard input cannot be selected together` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Explore remains unavailable without the retrieval feature` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by status` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by archive` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by check` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by lint` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by graph` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
| `Path traversal id is rejected by instructions` | `Rewording below the boundary without a distinction keeps the gate closed` | boundary |
