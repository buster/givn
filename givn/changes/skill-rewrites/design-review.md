# Design Review: skill-rewrites

## Scope

**PASS.** The proposal is limited to gate-aware choreography, Black-Box-First
guidance, and deterministic pre-retrieval behavior. The spec has one inventory
action with one E2E scenario and three regular instruction-output scenarios.
The gates and retrieval implementation remain out of scope.

## Technical Choices

**PASS.** Embedded asset edits and existing cascade/skills-sync behavior are the
smallest fit. Reusing the prior CLI guidance assertions and adding one small
capability-specific step file avoids renderer or binary changes.

## Missing Scenarios

**PASS.** The scenarios cover generated review gate choreography, design/step
Black-Box-First guidance, review instruction output, and spec guidance that
defers retrieval. The E2E scenario asserts the real `skills sync` CLI output
before inspecting generated guidance.

## Testability

**PASS.** Strict mode is already configured and will be proven in tasks. All
steps use existing or real subprocess bindings; undefined steps fail before
GREEN. The feature asserts concrete strings for every required policy.

## Risk

**PASS.** The likely failure is guidance drift across generated targets. The
E2E scenario enables all three targets, checks generated skills and commands,
and rejects retrieval instructions before their planned change.

## Arc42 Impact

**PASS.** The independent twelve-row assessment agrees with arc42.md: this is
an asset-prose rewrite on top of the canonical-source architecture documented
by Change 3. No new runtime component, interface, deployment, decision, risk,
or glossary term is introduced. All twelve durable chapter files remain
substantive and existing diagram policy is unchanged.

## Hardening

- Review choreography now requires Overlap dispositions, split-or-keep, and
  removed+added supersession explanations.
- Design and step choreography now asks **which case this test covers that the
  E2E does not**.
- Spec guidance explicitly uses deterministic gates and defers retrieval.
- Generated guidance and instruction outputs are covered by the feature.
- Rechecked the Step Definitions table after the integrity hook required an
  exact path cell; reuse documentation now sits outside the table.

DESIGN-REVIEW: PASS
