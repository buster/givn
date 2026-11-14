# Arc42 Documentation Update: add-ise-addon-and-specification

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | Section 1.1 states the addon default rule and now names `ise` as the one opt-in addon. | Addon posture sentence rewritten; chapter 10 unchanged because the change adds functional behavior, not a quality scenario. |
| 2 | 02 architecture-constraints | Yes | `TC-14` asserted uniform default-on addons. | `TC-14` rewritten to per-addon declared defaults and the opt-in exception. |
| 3 | 03 context-and-scope | Yes | A new project input crosses the technical boundary. | The interface table gains the read-only `givn/specification.yaml` input. |
| 4 | 04 solution-strategy | Yes | The strategy table claimed every addon is default-on. | Addons row added (opt-in posture, prerequisite, unchanged non-opted projects); Arc42 row corrected. |
| 5 | 05 building-block-view | Yes | A new component and changed consumer contracts. | `config::addons` (registry) and `config::declaration` added; `cli::init`, `cli::instructions`, `cli::lint`, `cli::addons`, `cli::migration`, and `config::manifest` updated. |
| 6 | 06 runtime-view | Yes | Three runtime flows change. | 6.4 records the declaration health pass; 6.12 records opt-in enable/disable and the scaffold; 6.13 records the upgrade scaffold and staged allowlist. |
| 7 | 07 deployment-view | No | No packaging, container, runtime, or topology change; the product stays a single binary. | |
| 8 | 08 crosscutting-concepts | Yes | A third configuration authority and a new exit-code condition. | 8.2 adds the read-only declaration authority; 8.4 adds the file to the layout; 8.8 adds the declaration error; 8.11 rewrites addon defaults. |
| 9 | 09 architecture-decisions | Yes | The configuration-authority decision is amended. | ADR-010 amended; register and `adr/README.md` rows name the amendment. |
| 10 | 10 quality-requirements | No | The four findings are functional behavior asserted by Gherkin; no new quality attribute scene or metric is introduced. | |
| 11 | 11 risks-and-technical-debt | Yes | The Cargo-only edge finding is a deliberate, deferred limit. | `D-28` records the limit and its repayment condition. |
| 12 | 12 glossary | Yes | Eight terms are confirmed and several existing rows need disambiguation. | The rows land in the implementation batches whose code makes each statement true; chapter 12 stays a link and needs no edit. |

## ADR qualification

```text
Architectural commitment: the declared Specification is a durable project file format and compatibility boundary read by every later ise capability.
Real alternatives: a specification: block in givn/config.yaml (rejected: the lenient overlay hides a misspelled block) and convention (rejected: cannot express the source: partition).
Durable consequence: reversal migrates every project's declaration and every reader; a misspelled key is refused, not ignored.
Existing-decision search: active and archived ADR registers checked; ADR-010 owns configuration authority and the commands.yaml split.
ADR action: AMEND_ADR
ADR target: ADR-010
```

Rejected candidates: the opt-in addon posture (reversal is a default flip with
no file migration and no broken interface, so it fails the durable-consequence
question); the `ise` prerequisite relation (a two-line registry rule whose
reversal breaks no consumer contract); the advisory findings (observable
behavior owned by the `check-specification-health` capability and its Gherkin).

## Notes

No documentation gap was found that this change leaves open: every affected
chapter named above is updated in this change, and the glossary rows,
`docs/domain-model.md` corpus clarification, and `docs/reference.md` updates
land in this change before tasks.

STATUS: DONE
