# Review

Disposition table for the deterministic overlap gate of this change.

## Overlap dispositions

| Scenario A | Scenario B | Decision |
| --- | --- | --- |
| `A BGE-only recommendation flags the review without NLI participation` | `An NLI-only recommendation flags the review without BGE participation` | KEEP-AS-BOUNDARY |

The pair shares its step shape deliberately: both scenarios pin the union
verdict rule for exactly one participating stage (BGE alone, NLI alone).
They assert disjoint fixtures and environments; merging them would lose the
per-stage independence evidence.

## Split-or-keep

| Scenario | Decision |
| --- | --- |
