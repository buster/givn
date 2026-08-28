# Design: fix-domain-modeling-skill-trigger

## Architecture Impact

The `domain_modeling` addon is currently a pure skill toggle: when enabled it
installs `givn-domain-modeling` on the next `givn skills sync`. Two defects
prevent it from ever guiding an agent:

1. The skill's trigger text (the frontmatter `description`, which opencode's
   skill loader matches against conversation state) is framed as a lifecycle
   phase — "model a change's domain before designing" — so no conversation
   about writing specs or the design matches it.
2. The session router (`givn instructions`) never mentions the skill: it
   prints slash-commands only, and `givn-domain-modeling` is a skill, not a
   slash-command, and has no lifecycle artifact to route to.

This change fixes both without adding any enforcement. It is advisory-only:
no new artifact, no marker gate, no lint class, no archive hook. Nothing
blocks on a missing or stale domain vocabulary. The skill template becomes
trigger-by-conversation-state with an active-discipline body; the router emits
an explicit "load the skill now" line at the phases where naming happens.

Two production touchpoints, plus test-only step definitions:

1. `assets/skills/givn-domain-modeling/SKILL.md.tmpl` — replace body and
   frontmatter description.
2. `src/cli/instructions.rs` — emit `Skill to load now: \`givn-domain-modeling\``
   in the human-readable output and a `skill_to_load` field in the JSON
   output, for the specs and design phases, only when
   `manifest.addons.domain_modeling` is true.

No external service or new runtime dependency is required. The artifact graph,
manifest schema, and lifecycle order are unchanged.

## Source Asset: skill template

Replace `assets/skills/givn-domain-modeling/SKILL.md.tmpl` in full. The
template keeps the `{{CHANGE_ID}}` placeholder (rendered by
`src/skills/render.rs`). The frontmatter `description` is the single load
trigger opencode exposes; it moves from a phase label to conversation states:
naming or discussing domain concepts, before writing `.feature` specs or
`design.md`, and when spec and code terms disagree.

The body becomes an active discipline rather than a one-shot scaffold:

- Challenge generic or overloaded terms, with a concrete utterance example.
- Name aggregates with domain nouns (`ShoppingList`, not `List`).
- Probe edge-case scenarios; cross-reference with code; update `domain.md`
  inline as the vocabulary resolves.
- Document `domain.md` as a real per-change artifact with a ubiquitous-language
  table, aggregates/boundaries, invariants, and lifecycle events.
- Lifecycle events describe the *domain's own* observable states (shopping
  list example), never the givn change lifecycle
  ("created -> planned -> implemented -> archived" is removed).
- Default anti-term guidance for aggregates/entities.

The template must pass `validate_frontmatter` (`src/skills/targets.rs`):
starts with `---`, `name: givn-domain-modeling`, `description:` present.

## Router: `src/cli/instructions.rs`

Add a small pure helper:

```rust
fn domain_skill_to_load(next_artifact_id: Option<&str>, domain_modeling: bool)
    -> Option<&'static str>
```

Returns `Some("givn-domain-modeling")` only when `domain_modeling` is true and
`next_artifact_id` is `specs` or `design`; otherwise `None`. This is the only
change the two invocation paths call:

- **Generic path** (`run_generic` -> `report_change`): the manifest is already
  loaded. Compute the next required artifact id from the change state and
  print, after the existing output, a line
  `Skill to load now: \`givn-domain-modeling\`` when the helper returns a
  skill. In JSON mode, add a `skill_to_load` top-level key carrying the skill
  name, present only when applicable (key omitted otherwise, keeping the
  existing contract stable).
- **Artifact path** (`run_artifact`): the manifest is already loaded. After
  printing the resolved instruction for the requested artifact, print the same
  line when the requested artifact is `specs` or `design` and the addon is on.
  In JSON mode, add the `skill_to_load` key to the returned object.

Phases scoped to specs and design: those are exactly the two artifacts the new
skill trigger names ("before writing .feature specs or design.md"). Proposal
and tasks are deliberately not routed; the trigger description covers them.

## Step Definitions

One file per capability, matching repo convention. Both are registered under
`pub mod` in `tests/steps/mod.rs`:

| Capability | Step definition file |
|---|---|
| Domain-modeling skill is triggered and routed (regular) | tests/steps/fix_domain_modeling_skill_trigger_steps.rs |
| Domain-modeling skill is triggered and routed (e2e) | tests/steps/fix_domain_modeling_skill_trigger_e2e_steps.rs |

The regular file inspects the `assets/skills/givn-domain-modeling/SKILL.md.tmpl`
source asset for trigger-text, active-discipline, lifecycle, and anti-term
content, and drives the addon-off variant and the JSON variant through the CLI
via `crate::common::run_givn`. The e2e file creates a fixture project
(`crate::common::make_init_root`), toggles the addon with
`givn addons enable|disable domain_modeling`, creates a change awaiting specs
with `givn new <id>`, then invokes the real compiled `givn` binary as a
subprocess (`givn skills sync`, `givn instructions`, `givn instructions
specs|design --change <id>`) and asserts on stdout/JSON.

Fixture change id: `domain-skill-trigger`. Addon enable step:
`run_givn(env, &["addons", "enable", "domain_modeling"])` must succeed before
the routing assertions; the addon-off variant must disable it explicitly
(`make_init_root` enables every addon by default).

## Test Infrastructure

No external dependency. The local run command:

```sh
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner
```

Configured verification is `./run-tests.sh`; configured E2E is
`./run-tests.sh --e2e`. A single scenario is run with:

```sh
GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_RERANKER=stub cargo test --features spec-retrieval --test features_runner -- --name "<scenario title>"
```

The runner uses `fail_on_skipped()` (`tests/features_runner.rs`); RED step
bodies use `unimplemented!()`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run `givn skills sync` to regenerate skills | An agent synchronizes skills and the domain skill is routed | CLI | Real compiled Givn subprocess `givn skills sync`; read the generated `.opencode/skills/givn-domain-modeling/SKILL.md` and assert trigger/lifecycle content |
| request next-step guidance via `givn instructions` | An agent requests next-step guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess `givn instructions`; inspect stdout |
| request the specs instructions via `givn instructions specs --change <id>` | An agent requests the specs guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess; inspect stdout |
| request the design instructions via `givn instructions design --change <id>` | An agent requests the design guidance and is routed to the domain skill | CLI | Real compiled Givn subprocess; inspect stdout |

The addon-off variant and the JSON variant are variants of the specs /
next-step actions, not additional E2E actions; they are covered by regular
scenarios driving the same commands.

## Internal Primitives

No new domain or production primitives. The router uses the existing manifest
addon flag and change state. Step definitions use the existing
`crate::common` subprocess helpers.

## Verify command

Unit/integration:

```sh
./run-tests.sh
```

E2E smoke tests:

```sh
./run-tests.sh --e2e
```

Plus the repo self-dogfood: `cargo install --path . --locked --force`,
`givn skills sync`, and inspection of the generated
`.opencode/skills/givn-domain-modeling/SKILL.md`.
