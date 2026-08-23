# Migration Bundles

Release-owned project migrations live in the binary's embedded asset tree at
`assets/migrations/`.

Each bundle is one ordered migration phase and contains:

- `proposal.md` with the user-facing problem, scope, and completion boundary;
- `design.md` with every LLM instruction required to perform that phase without
  access to the Givn source checkout.

Register the bundle in `assets/migrations/catalog.yaml` with a unique id,
strictly advancing `source_version` and `target_version`, deterministic
`order`, and the two asset paths. Do not encode a migration decision in a new
Rust conditional.

`givn upgrade` resolves the version gap through the declared bundle edges. If
multiple bundles are required, it creates one aggregate maintenance change with
`migration.yaml`, ordered phase sections, and all bundle Proposal/Design
content. The LLM must complete the phases in order. If no bundle route applies,
Givn performs only the managed config/guidance refresh and creates no migration
change.

Bundle designs must be self-contained. They may use the
`{{SOURCE_VERSION}}`, `{{TARGET_VERSION}}`, and `{{UPGRADE_COMMIT}}` placeholders
for generated metadata. The current 0.2.0-to-0.3.0 bundle additionally uses
`{{MIGRATION_GUIDE}}` to embed the public prompt pack; future bundles must carry
their own complete prompts instead of relying on that guide.
