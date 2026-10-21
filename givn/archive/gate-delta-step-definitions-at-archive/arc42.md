# arc42 Documentation Update: gate-delta-step-definitions-at-archive

Every row was checked against `proposal.md`,
`specs/archive-change/require-merged-step-definitions.feature`, the updated
`specs/archive-change/usecase.md`, and `design.md`.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | No goal, stakeholder, or quality attribute is added or changed. The gate realizes the archive use case's existing success guarantee (a permanent corpus bound to a passing receipt) before the candidate is published. | None |
| 2 | 02 architecture-constraints | No | The change adds no technology, dependency, or deployment constraint. The heuristic's recognition limits are a chapter 11 risk, not a constraint. | None |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface changes: `givn archive --change <id>` keeps its shape and gains only a pre-mutation refusal on stderr. | None |
| 4 | 04 solution-strategy | No | The strategy is unchanged: forward-only archive, text-level delta merge, one published candidate verified once. The new comparison is a precondition inside that strategy. | None |
| 5 | 05 building-block-view | Yes | The `cli::archive` building block gains the step-definition preflight and its helpers; no module or component is added. | The archive row names the delta step-definition comparison and its refusal before mutation on the fresh and resume paths. `docs/arc42/05-building-block-view.md` |
| 6 | 06 runtime-view | Yes | The archive runtime gains a gate in stage 3 and repeats it on the pending/resume path before verification. | The 6.7 sequence diagram and stage 3 list the step-definition comparison and its placement before hooks and before candidate verification. `docs/arc42/06-runtime-view.md` |
| 7 | 07 deployment-view | No | Still one native binary, no new process, port, container, database, or distribution artifact. | None |
| 8 | 08 crosscutting-concepts | Yes | The Gherkin delta model must state that step definitions are never merged and how the preflight compares phrase sets. | Section 8.3 adds the durable-source rule, the one path rule, the binding-expression scanner and its tokens, the two prune sets (component names and project-relative prefixes), the unreadable-file rule, the bounded refusal with the project-relative delta path, and the repair instruction. `docs/arc42/08-crosscutting-concepts.md` |
| 9 | 09 architecture-decisions | No | The decision "step definitions are durable source, not archive-merged code" fails the lower-level-artifact and product-behaviour dimensions: it is workflow behaviour owned by the archive-change use case, its Gherkin scenarios, design.md, and chapter 8.3. It moves no component, authority, or deployment boundary, so it is routed to that canonical artifact rather than an ADR. | None |
| 10 | 10 quality-requirements | No | No new quality scenario: the archive's existing proven-publication requirement now holds one step earlier. The scenarios in the capability are functional behaviour, not new quality goals. | None |
| 11 | 11 risks-and-technical-debt | Yes | The binding scanner skips a dynamically computed phrase, compares phrases literally after whitespace normalization, and skips an unreadable or non-UTF-8 file under a step directory. | R-123 records the remaining limits and their mitigation. `docs/arc42/11-risks-and-technical-debt.md` |
| 12 | 12 glossary | Yes | "Step definition" becomes durable project vocabulary with a boundary that archive enforces, and "step phrase" names the comparison unit. | The Step definition entry states durable source, the one path rule, the archive-history role of the delta copy, and the permanent binding requirement; the Step phrase entry names the literal bound by a binding. `docs/arc42/12-glossary.md` |

## ADR qualification verdict

Candidate: "step definitions are durable project source and are never merged,
copied, or deleted by archive; archive refuses before mutation when a delta
phrase is unbound permanently."

```json
{
  "qualification": "NOT_QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "FAIL",
  "durable_consequence": "PASS",
  "lower_level_artifact": "FAIL",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "CANONICAL_ARTIFACT",
  "canonical_artifact": "givn/specs/archive-change/usecase.md + specs/archive-change/require-merged-step-definitions.feature + docs/arc42/08-crosscutting-concepts.md §8.3",
  "target_adr": null,
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "refuse with a named repair (chosen)",
      "auto-merge definitions into a permanent step file",
      "rely on post-move verification (the reproduced failure)"
    ],
    "architectural_impact": [
      "the change modifies one existing component (cli::archive) and adds no module, dependency, process, or deployment boundary",
      "archive's authority over the permanent corpus is unchanged; it now checks before it writes instead of discovering the gap after publishing"
    ],
    "durable_consequence": [
      "reversal would change the archive contract, the served instructions, and the spec; the one-change reversibility test does not rescue it, but architectural impact and lower-level artifact already fail"
    ],
    "lower_level_artifact": [
      "the behaviour is owned by the archive-change use case and executable Gherkin",
      "the recognition rules and the repair text are owned by chapter 8.3 and design.md",
      "authoring guidance is owned by assets/instructions/steps.md and archive.md"
    ],
    "existing_adr_check": [
      "chapter 09 register and adr/README.md searched; no ADR fixes step-definition merge behaviour"
    ]
  }
}
```

## Affected-chapter evidence

- **05 building-block-view.** The `cli::archive` row now includes "compare the
  delta step-definition binding phrases with the permanent test tree (the
  project root minus change, archive, history, VCS, build, and cache trees) and
  refuse before mutation when one is missing (fresh and resume paths)". The
  implementing code stays in `src/cli/archive.rs`.
- **06 runtime-view.** Stage 3 lists the step-definition comparison between
  `apply.requires` completeness and the configured hooks; the sequence diagram
  names it in the structural-gate line and repeats it on the resume branch.
- **08 crosscutting-concepts.** Section 8.3 states that archive merges feature
  files and use-case/fragment documents only, defines the one step-definition
  path rule, describes the binding-expression scanner and the supported tokens,
  lists the two prune sets, records that unreadable/non-UTF-8 files are skipped
  in both walks, and records the bounded phrase list and the repair
  instruction.
- **11 risks-and-technical-debt.** R-123 records the scanner's remaining limits
  (dynamic bindings skipped, literal phrase comparison, unreadable files
  skipped) and names the documentation and scenario coverage that keep them
  inspectable.
- **12 glossary.** The Step definition entry fixes the durable term, the one
  path rule, and the delta-step-file wording; the Step phrase entry names the
  comparison unit.
- **Index.** `docs/arc42/README.md` records the change, its canonical artifacts,
  the no-ADR disposition, and the affected chapters.

## Status

STATUS: DONE
