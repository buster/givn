# Design Review: retire-semantic-retrieval-vocabulary

## Coverage

| Question | Answer | Where |
|---|---|---|
| New components? | None | design.md §Overview |
| Step definitions | One new assertion step; skill-rewrites uses existing generic steps | design.md §Step Definitions |
| Why modify instead of remove? | Both scenarios still guard real invariants (E2E policy presence; no retrieval advertising); only the pinned phrase was deleted | proposal.md |

DESIGN-REVIEW: PASS
