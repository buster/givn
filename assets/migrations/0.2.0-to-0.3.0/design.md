# Migration Design: 0.2.0 to 0.3.0

This bundle is the canonical release-owned migration phase for projects created
from Givn 0.2.0. The complete public prompt pack is embedded below into the
generated aggregate design by the migration engine. Future bundles must contain
their complete prompts here rather than relying on the consumer having this
source repository available.

## Required LLM Actions

1. Inventory the consumer project's config, commands, generated integrations,
   active changes, permanent specs, ejected facets, and optional Arc42/ADR
   documents before editing.
2. Confirm the machine-safe managed upgrade commit and inspect its allowlist.
3. Synchronize generated guidance without overwriting project-owned overrides.
4. Review active changes and permanent specifications against the 0.3.0
   interaction, overlap, semantic, E2E, and review contracts.
5. Reconcile Arc42 and ADR content using project evidence and the canonical
   qualification procedure; do not invent decisions or provenance.
6. Verify all applicable commands and archive the maintenance change only after
   its design-review, tasks, and review gates pass.

## Required Boundaries

- Do not automatically edit domain specifications or architecture decisions.
- Do not add a product feature spec for this maintenance phase.
- Do not remove `@e2e`, semantic evidence, or review gates to obtain a pass.
- Keep this phase ordered before the next migration bundle in an aggregate
  plan.

## Full Prompt Pack

The following marker is replaced with the complete version-specific prompt
pack when this bundle is materialized. Future bundles must carry their full
prompt content directly in this asset.

{{MIGRATION_GUIDE}}
