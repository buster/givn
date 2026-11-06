# arc42 Documentation Update: enforce-spec-interaction-integrity

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The existing "Reliable evidence" quality goal already covers the guarantee; no new goal, stakeholder class, or success guarantee is introduced. | |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint changes; the new gate is workflow behavior recorded in chapters 8 and 10. | |
| 3 | 03 context-and-scope | No | No external system, interface, or user-facing surface is added; the existing CLI gains a refusal. | |
| 4 | 04 solution-strategy | Yes | The strategy now projects the accepted corpus before mutation and shares one merge-plan implementation between publication and projection. | 4.2 adds the projected-corpus gate and the `spec::merge_plan`/`spec::projection` boundary. |
| 5 | 05 building-block-view | Yes | Two production modules are added and the lint/archive/interaction responsibilities change. | Add `spec::merge_plan` and `spec::projection`; update `cli::lint`, `cli::archive`, and `gherkin::interaction`. |
| 6 | 06 runtime-view | Yes | Lint, review, and archive gain a projected-corpus flow and archive gains a merged-candidate recheck. | 6.4 and 6.7 describe the projection, the pre-mutation refusal, and the post-merge check. |
| 7 | 07 deployment-view | No | The deployment is still one native binary; no artifact, channel, or topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | The projected interaction gate is a new cross-cutting validation concept, and the lint exit-code contract is corrected. | Add 8.3c; update 8.8 exit codes for use-case and interaction findings. |
| 9 | 09 architecture-decisions | Yes | The pre-acceptance gate and the component split are architecturally significant. | Add ADR-075 and the active-register row. |
| 10 | 10 quality-requirements | Yes | The lint precedence scenario is corrected and a new specification-integrity scenario is added. | Correct Q-8; add Q-125. |
| 11 | 11 risks-and-technical-debt | Yes | Pre-existing drift, undeclared features, feature directories with no document, and projection divergence become migration and implementation risks. | Add R-135, R-136, and R-137. |
| 12 | 12 glossary | Yes | Three interaction-inventory terms become part of the durable vocabulary. | Add "Interaction inventory", "Projected interaction inventory", and "Undeclared capability". |

## Notes

The change adds no external service, dependency, database, or deployment
surface. It repairs the permanent corpus in the same change: the delta narrative
adds the 19 missing rows for the 10 fragment capabilities and refreshes the
base hash to the current permanent `fragment.md`, and the `initialize-project`
delta adds the missing `@e2e` tag. Review and archive evaluate the projected
corpus before mutation; a merged candidate is rechecked on the actual permanent
corpus before verification. The projection deliberately does not subtract
pre-existing findings, so a downstream project with drift must repair it before
its next review or archive.

`docs/domain-model.md` is unchanged: the change adds no bounded context,
aggregate, or relationship. The new components are internal and the model
explicitly excludes internal Rust components.

The reverse check requires every permanent feature file to be declared in its
use-case or fragment capability list, and a feature directory with no use-case
or fragment document is reported rather than silently skipped. The current
corpus has no undeclared feature after the fragment repair. The shared
archive/review fixtures are repaired in the change's test plan so their
inventories match their actual `@e2e` titles; the repository-ownership fixture
gains its missing document. That is test infrastructure, not a durable
architecture change.

```text
ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-075-projected-interaction-integrity.md

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: FAIL
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null
evidence:
  alternatives:
    - "project the merged corpus before mutation vs. check only after the archive merge vs. CI-only lint; the pre-mutation projection gives the developer the failure while the change is active and preserves the archive's block-before-mutation rule"
  architectural_impact:
    - "adds `spec::projection` as a consumer-facing component and moves merge planning out of `cli::archive` into `spec::merge_plan`, so publication and projection share one implementation and dependency direction points inward"
  durable_consequence:
    - "reversal changes the review and archive gate contract and the component boundary; future gates and consumers use the same projected inventory, and the new Gherkin scenarios bind the pre-mutation refusal and the merged-candidate recheck"
  lower_level_artifact:
    - "the projection algorithm, in-memory corpus mechanics, the shared merge_feature_text entry point, and the exact finding text are owned by design.md, spec::corpus, spec::merge_plan, spec::projection, and the delta features; the durable boundary between acceptance and publication is not owned by a lower-level artifact"
  existing_adr_check:
    - "searched the active register and docs/arc42/adr/archive/ for interaction, mandatory-check, and archive-publication records; the archived semantic-engine records are disjoint from this boundary, and ADR-019 delegates the current lint contract to givn/specs/verify-change/; ADR-011 owns the feature-is-spec contract, ADR-028 the mandatory checks, and ADR-070/ADR-072 the archive proof and publication boundaries; none owns a projected pre-acceptance consistency gate"
```

No new version number is introduced; the change uses the existing pinned
dependencies in `Cargo.lock`.

---

## Status

STATUS: DONE
