# Proposal: release-override-hardening

## Problem / Opportunity

The semantic quality gates compare every new scenario against the permanent
corpus with real models and refuse unexplained overlaps. Their verdicts are
only trustworthy if the models that produced them are the models givn ships
with. An installed build already refuses the obvious environment-carried
attempts to fake or skip the models. Three gaps remain:

1. **The refusal rule lives only in the code that reads each channel.** A
   future command that reads a new environment channel without registering it
   silently re-opens the bypass. Nothing fails when that happens — the hole is
   invisible until someone probes for it.
2. **The rule is undocumented for the audience that matters most: automated
   agents.** An agent whose configuration change has no effect will try again
   unless something tells it the attempt is pointless; and a maintainer cannot
   review whether a new channel respects the rule without reading code.
3. **The honoured exception is unrecorded.** Installed builds honour
   model-location requests (pointing at a locally provided artifact set) —
   necessary for offline installations. The trust this places in the provided
   artifacts and the fail-safe behaviour behind it exist only as
   implementation details.

## Proposed Solution

1. **The refusal rule becomes mechanically complete.** Every environment
   channel the semantic review can read is registered in one policy place, and
   the project's own suite proves that no read bypasses the registry: a new
   channel without a policy entry fails the project's tests at development
   time — the hole closes itself, not at audit time.
2. **The rule is stated where agents look.** The project documentation states
   the enforcement rule plainly: a givn build that ships with the semantic
   capability always evaluates the semantic gates with the real models;
   requests for simulated output, disabled layers, resized model work, or
   injected scores are refused and announced by name; only development builds
   honour such requests (for the offline test fixtures).
3. **The honoured exception is documented as an exception.** Model-location
   requests stay honoured in installed builds. The documentation records what
   is trusted: the artifact set's structure is validated, the model identity
   on this path is not verified, and an unusable artifact set degrades
   fail-safe to the binding embedding threshold — never to "no gate".
4. **The fail-safe is pinned.** When real model evidence is unavailable for
   any stage, the gate does not disappear: the embedding comparison at its
   binding threshold remains the verdict.

## Out of Scope

- Model-identity verification for locally provided artifact sets (decided:
  keep flexible; documented as an accepted exception).
- Refusing model-location requests in installed builds.
- Enforcement beyond the refusal policy (no sandboxing, no attestation).
- What the gates evaluate — that is `unified-semantic-rulebook`.

## Open Questions

None. Decisions taken with the product owner:

- The local model path stays flexible; the theoretical model-swap vector is
  accepted and documented, not hardened.
- Refusal completeness is enforced by the project's own suite (a registry and
  a source-derived completeness proof), not by code review.
- Development builds keep honouring every fixture request — the test
  harness's offline fixtures depend on them.
