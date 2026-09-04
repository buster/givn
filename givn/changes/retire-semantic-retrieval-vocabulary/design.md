# Design: retire-semantic-retrieval-vocabulary

## Overview

No new architecture. Two permanent scenarios pin text from the deleted
`## Semantic review and retrieval vocabulary` specs-instruction section;
both are modified to assert the surviving (or newly-correct) content.

## Technology decisions

None. Two `@givn.modified` scenarios in two capabilities. One new step
definition (`the CLI output no longer advertises retrieval commands`)
asserting the retrieval commands are gone from the specs instruction.

## Step Definitions

| Cap | File |
|---|---|
| instruction-integrity | `tests/steps/instruction_integrity_e2e_steps.rs` |
| skill-rewrites | `tests/steps/mod.rs` |

## Architecture Impact

None. Guidance content only.
