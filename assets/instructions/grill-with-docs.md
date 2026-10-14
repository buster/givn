# Instruction: grill-with-docs

Grill the plan or design against the project's own docs and specs — cite the
source for every challenge.

For every user-facing question, follow the effective policy from
`givn instructions dialog`: name artifacts in domain language, and give every
option its meaning, gain, cost, and a concrete example.

Unlike `grill-me`, this discipline **must cite a source** for each challenge: a
line in the proposal, a spec scenario, the design doc, an architecture chapter,
or an ADR. If no source backs a concern, drop it.

## Inventory the sources

Gather the authoritative material:

- `givn/changes/<id>/proposal.md`
- `givn/changes/<id>/specs/**/*.feature`
- `givn/changes/<id>/design.md`
- `givn/changes/<id>/tasks.md`
{{#if addons.arc42}}
- `docs/arc42/` (if the arc42 addon is enabled)
{{/if}}
- Any architecture decision records or the project's decisions log
- `givn/config.yaml` (verify.command, features_path, addons, extras)

## Cross-check question by question

For each plan element, ask: which source supports it, and which source
contradicts it? Pose one question, cite the source, wait. Examples:

- "tasks.md says the flow is write-config -> set_extras -> sync, but design.md
  pitfall #4 warns about --force wiping the flag. Is the ordering covered?"
- "The spec asserts `extras: true`, but config.yaml.tmpl ships it commented.
  Which step uncomments it?"

## Flag drift

Call out anywhere the plan disagrees with a cited source. Force a resolution:
update the plan, update the source, or record an explicit decision.

## Record

List each challenge with its citation, the resolution, and any open question
that needs a human decision before implementation.

## Output

A sourced list of contradictions and drift findings. Every line must name where
it came from. Unresolved drift blocks implementation.
