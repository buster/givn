# Proposal: fix-domain-modeling-skill-trigger

## Problem / Opportunity

The optional `domain_modeling` addon exists so a change keeps one coherent
domain vocabulary from the first domain noun through specs, design, and code.
In practice the skill it installs is never triggered. The skill's trigger text
(the description an agent reads to decide whether to load it) is framed as a
lifecycle phase — "model a change's domain before designing" — so the agent has
no reason to load it while it is writing a proposal or specs, which is exactly
when naming happens. As a result the vocabulary drifts: one action is spelled
with two verbs, aggregates get framework words instead of domain nouns, and the
semantic gate later flags the symptoms (near-duplicate scenario pairs) instead
of the root cause (bad terms).

The second half of the problem is that the session entry point never mentions
the skill either. `givn instructions` promises to tell an agent which command
or skill to run next, but its output names slash-commands only. The domain
skill is not a slash-command and has no lifecycle artifact of its own, so no
phase ever points at `givn-domain-modeling`. Even a correct trigger text would
still depend on chance: nothing in the loop tells the agent, at the phases
where naming happens, to load the skill.

## Proposed Solution

The domain-modeling skill is triggered by conversation state, not by a phase
label. Its trigger text loads whenever the agent is naming or discussing
domain concepts, before writing specs or the design, or when spec and code
terms disagree. Its body becomes an active discipline (challenge generic
terms, name aggregates with domain nouns, keep the domain vocabulary current)
instead of a one-shot scaffold, and the lifecycle guidance describes the
domain's own states rather than the givn change lifecycle.

The session entry point routes the agent to the skill at the phases where
naming happens. When the addon is enabled and the next required artifact is
the specs or the design, `givn instructions` says explicitly
"load the `givn-domain-modeling` skill now", in both the human-readable and
the machine-readable output.

## Out of Scope

- No enforcement of the vocabulary: no new artifact, no marker gate, no lint
  findings, no archive hook. The skill and the router are advisory; nothing
  blocks on a missing or stale domain vocabulary.
- The anti-term list and the ubiquitous-language guidance are skill content
  only; no tooling consumes them in this change.
- Addon enablement/disabling behaviour and skill installation behaviour are
  unchanged.
- The change lifecycle order is unchanged.

## Open Questions

None. The deferred enforcement layer (gate/hook) is intentionally out of scope
and will be revisited separately once a generalised addon-hook pattern is
agreed, avoiding further per-addon special-casing in the lifecycle tooling.
