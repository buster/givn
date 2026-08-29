# Proposal: release-strict-model-overrides

## Problem / Opportunity

The semantic review gates — E5 similarity, BGE reranker evidence, and NLI
entailment evidence — are the project's quality guardrails. During givn's own
development the gates are exercised with simulated model output so the fixture
suite stays fast, offline, and deterministic. That simulation is intended only
for the development build, but nothing today stops an installed givn from
honouring the same environment-driven request.

Consequences:

- A user, an automation script, or a lingering shell variable can make an
  installed givn silently substitute simulated scores for real model evidence,
  turning a blocking review gate into a rubber stamp. The entire point of the
  installed tool is that the gates are real.
- givn's own dogfooding of the installed binary can accidentally run with
  simulated evidence too, so the project validates itself against fake model
  output instead of the real models it ships.

User expectation: the givn a person installs always evaluates every semantic
gate with the real models. Simulation is a development-harness capability and
must never be reachable from an installed givn.

## Proposed Solution

- A givn installed for normal use always runs the real E5, BGE, and NLI models
  for every semantic gate, and never substitutes simulated output — even when
  the environment requests it.
- A givn built for development still honours requests for simulated model
  output, so the fast, offline fixture suite keeps working unchanged.
- When an installed givn is asked to use a simulation it will not honour, it
  announces that the request is ignored for installed builds, so the behaviour
  is never silently different from what the request implied.
- Requests that still use the real models — pointing at a local or offline
  artifact set or a custom model cache — remain available in an installed givn,
  so air-gapped and mirrored deployments keep working.

## Out of Scope

- Which models the real gates use, and their thresholds and evidence rules.
- The development build's behaviour: simulation requests keep working exactly as
  before; no fixture or test-suite change is made.
- Command-line flags and project configuration: only environment-driven model
  requests change.
- Download-hosting and cache-root variables consumed by the underlying model
  libraries.

## Open Questions

None. The scope of which requests are blocked, the announcement wording, and the
proof that a release build really ignores them are settled in the design.
